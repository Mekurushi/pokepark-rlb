use crate::relocation::RelocationTable;
use crate::string_pool::StringPool;
use crate::table::Table;
use crate::util::resolve_string_from_raw_data;
use rlb_error::Result;
use rlb_format::{RawFile, TableRecord};
use slotmap::SlotMap;

slotmap::new_key_type! {
    pub struct TableId;
    pub struct LabelId;
    pub struct StringId;
}

#[derive(Debug, Clone)]
pub struct TocSlot {
    pub table: TableId,
    pub label: Option<LabelId>,
}

#[derive(Debug, Clone)]
pub struct RLBFile {
    string_pool: StringPool,
    tables: SlotMap<TableId, Table>,
    relocation_table: RelocationTable,
    toc: Vec<TocSlot>,
    other_toc: Vec<TocSlot>,
    labels: SlotMap<LabelId, String>,
}

impl RLBFile {
    pub fn from_raw(raw: RawFile) -> Result<Self> {
        let RawFile {
            header: _header,
            data,
            relocation_table,
            records,
            other_records,
            table_labels,
        } = raw;
        let mut toc: Vec<TocSlot> = Vec::with_capacity(records.len());
        let mut other_toc: Vec<TocSlot> = Vec::with_capacity(other_records.len());
        let mut string_pool = StringPool::new();
        let mut tables: SlotMap<TableId, Table> = SlotMap::with_key();
        let mut labels = SlotMap::with_key();
        let relocations = RelocationTable::from_raw(relocation_table);
        //TODO: sort by address
        build_records(
            records,
            &*data,
            &*table_labels,
            &mut string_pool,
            &mut tables,
            &mut labels,
            &mut toc,
            &relocations,
        )?;
        build_records(
            other_records,
            &data,
            &table_labels,
            &mut string_pool,
            &mut tables,
            &mut labels,
            &mut other_toc,
            &relocations,
        )?;

        Ok(Self {
            string_pool,
            tables,
            relocation_table: relocations,
            toc,
            other_toc,
            labels,
        })
    }

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Self::from_raw(RawFile::parse(bytes)?)
    }
}

// TODO: temporary solution until better way to handle building is known
fn build_records(
    records: Vec<TableRecord>,
    data: &[u8],
    table_labels: &[u8],
    strings: &mut StringPool,
    tables: &mut SlotMap<TableId, Table>,
    labels: &mut SlotMap<LabelId, String>,
    tocs: &mut Vec<TocSlot>,
    relocations: &RelocationTable,
) -> Result<()> {
    for record in records {
        let name = resolve_string_from_raw_data(table_labels, record.label_offset as usize)?;
        let mut resolve_string = |offset: u32| -> Result<StringId> {
            //TODO: better string interning
            let s = resolve_string_from_raw_data(data, offset as usize)?;
            Ok(strings.intern(s))
        };
        let mut is_relocated = |offset: u32| -> bool { relocations.is_relocated(offset) };

        let table = Table::resolve(
            &name,
            data,
            record.address as usize,
            &mut resolve_string,
            &mut is_relocated,
        )?;

        let table_id = tables.insert(table);
        let label_id = labels.insert(name);

        tocs.push(TocSlot {
            table: table_id,
            label: Some(label_id),
        });
    }
    Ok(())
}
