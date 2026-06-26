use crate::table::Table;
use crate::util::resolve_string_from_raw_data;
use rlb_error::Result;
use rlb_format::{Header, RawFile, RelocationTable, TableRecord};
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
    header: Header,
    strings: SlotMap<StringId, String>,
    tables: SlotMap<TableId, Table>,
    relocation_table: RelocationTable,
    toc: Vec<TocSlot>,
    labels: SlotMap<LabelId, String>,
}

impl RLBFile {
    pub fn from_raw(raw: RawFile) -> Result<Self> {
        let RawFile {
            header,
            data,
            relocation_table,
            records,
            table_labels,
        } = raw;
        let mut toc: Vec<TocSlot> = Vec::with_capacity(records.len());
        let mut strings: SlotMap<StringId, String> = SlotMap::with_key();
        let mut tables: SlotMap<TableId, Table> = SlotMap::with_key();
        let mut labels = SlotMap::with_key();
        

        for record in records {
            match record {
                TableRecord::Named {
                    address,
                    name_offset,
                }
                | TableRecord::Unknown {
                    address,
                    raw_offset: name_offset,
                } => {
                    let name = resolve_string_from_raw_data(&table_labels, name_offset as usize)?;
                    let mut resolve_string = |offset: u32| -> Result<StringId> {
                        let s = resolve_string_from_raw_data(&data, offset as usize)?;
                        Ok(strings.insert(s))
                    };
                    let mut is_relocated =
                        |offset: u32| -> bool { relocation_table.is_relocated(offset) };

                    let table = Table::resolve(
                        &name,
                        &data,
                        address as usize,
                        &mut resolve_string,
                        &mut is_relocated,
                    )?;

                    let table_id = tables.insert(table);
                    let label_id = labels.insert(name);

                    toc.push(TocSlot {
                        table: table_id,
                        label: Some(label_id),
                    });
                }
            }
        }

        Ok(Self {
            header,
            strings,
            tables,
            relocation_table,
            toc,
            labels,
        })
    }

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Self::from_raw(RawFile::parse(bytes)?)
    }
}
