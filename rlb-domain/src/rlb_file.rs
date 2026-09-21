use crate::relocation::RelocationTable;
use crate::string_pool::StringPool;
use crate::table::{ParseContext, Table, TableCollection, TableId};
use crate::util::{checked_u32, resolve_string_from_raw_data};
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};
use rlb_format::{RawFile, TableRecord};

#[derive(Debug, Clone)]
struct TocSlot {
    table: TableId,
    label: String,
}

#[derive(Debug, Clone)]
pub struct RLBFile {
    table_collection: TableCollection,
    toc: Vec<TocSlot>,
    other_toc: Vec<TocSlot>,
}

#[derive(Debug)]
pub struct TableView<'a> {
    pub id: TableId,
    pub label: &'a str,
    pub fields: &'static [FieldDescriptor],
    pub entry_count: usize,
}

impl RLBFile {
    pub fn from_raw(raw: &RawFile) -> Result<Self> {
        let data = raw.data();
        let relocation_table = raw.relocation_table();
        let records: &Vec<TableRecord> = raw.records();
        let other_records = raw.other_records();
        let table_labels = raw.table_labels();

        let mut table_collection: TableCollection = TableCollection::new();
        let relocations = RelocationTable::from_raw(relocation_table);
        let parse_context = ParseContext::new(data, &relocations);

        let mut pending_toc = build_pending_tables(records, table_labels)?;
        let mut pending_other_toc = build_pending_tables(other_records, table_labels)?;
        parse_tables(
            &mut pending_toc,
            &mut pending_other_toc,
            &mut table_collection,
            &parse_context,
        )?;
        let toc = finish_toc(pending_toc)?;
        let other_toc = finish_toc(pending_other_toc)?;

        Ok(Self {
            table_collection,
            toc,
            other_toc,
        })
    }

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Self::from_raw(&RawFile::parse(bytes)?)
    }

    fn to_raw(&self) -> Result<RawFile> {
        let tables = self.table_collection.serialize()?;
        let mut labels = StringPool::new();
        let mut make_records = |toc: &Vec<TocSlot>| -> Result<Vec<TableRecord>> {
            toc.iter()
                .map(|record| {
                    Ok(TableRecord {
                        address: checked_u32(
                            tables
                                .offset_of(record.table)
                                .ok_or(Error::Validation(format!(
                                    "table ID {:?} not found in serialized table pool",
                                    record.table
                                )))?,
                            "converting table offset",
                        )?,
                        label_offset: checked_u32(
                            labels.intern(&record.label)?,
                            "converting label offset",
                        )?,
                    })
                })
                .collect()
        };

        let records = make_records(&self.toc)?;
        let other_records = make_records(&self.other_toc)?;

        RawFile::new(
            tables.data().to_vec(),
            tables.relocations().clone(),
            records,
            other_records,
            labels.data().to_vec(),
        )
    }

    pub fn write(&self) -> Result<Vec<u8>> {
        self.to_raw()?.serialize_custom()
    }

    pub fn tables(&self) -> Result<Vec<TableView<'_>>> {
        self.toc
            .iter()
            .chain(self.other_toc.iter())
            .map(|slot| {
                let table = self.table_collection.get(slot.table).ok_or_else(|| {
                    Error::Validation(format!(
                        "table of contents references missing table {:?}",
                        slot.table
                    ))
                })?;
                Ok(TableView {
                    id: slot.table,
                    label: &slot.label,
                    fields: table.field_descriptors(),
                    entry_count: table.entry_count(),
                })
            })
            .collect()
    }

    pub fn get_field(&self, table_id: TableId, entry_index: usize, field: &str) -> Option<Value> {
        self.table_collection
            .get(table_id)?
            .get_field(entry_index, field)
    }
    pub fn set_field(
        &mut self,
        table_id: TableId,
        entry_index: usize,
        field: &str,
        value: Value,
    ) -> Result<()> {
        //TODO: Validation
        let table = self
            .table_collection
            .get_mut(table_id)
            .ok_or_else(|| Error::Validation(format!("unknown TableId {table_id:?}")))?;

        table.set_field(entry_index, field, value)
    }
}

#[derive(Debug)]
struct PendingTable {
    label: String,
    root_address: usize,
    parsed_table: Option<TableId>,
}

fn build_pending_tables(records: &[TableRecord], table_labels: &[u8]) -> Result<Vec<PendingTable>> {
    let mut pending = Vec::with_capacity(records.len());

    for record in records {
        let label = resolve_string_from_raw_data(table_labels, record.label_offset as usize)?;
        pending.push(PendingTable {
            label,
            root_address: record.address as usize,
            parsed_table: None,
        });
    }

    Ok(pending)
}

fn parse_tables(
    pending_toc: &mut [PendingTable],
    pending_other_toc: &mut [PendingTable],
    tables: &mut TableCollection,
    context: &ParseContext<'_>,
) -> Result<()> {
    let mut parse_order: Vec<&mut PendingTable> = pending_toc
        .iter_mut()
        .chain(pending_other_toc.iter_mut())
        .collect();
    parse_order.sort_by_key(|pending| pending.root_address);

    for pending in parse_order {
        let table = Table::parse(&pending.label, context, pending.root_address)?;
        pending.parsed_table = Some(tables.insert(table));
    }

    Ok(())
}

fn finish_toc(pending: Vec<PendingTable>) -> Result<Vec<TocSlot>> {
    pending
        .into_iter()
        .map(|pending| {
            Ok(TocSlot {
                table: pending.parsed_table.ok_or_else(|| {
                    Error::Validation(format!("table {:?} was not parsed", pending.label))
                })?,
                label: pending.label,
            })
        })
        .collect()
}
