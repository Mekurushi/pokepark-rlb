use crate::string_pool::StringPool;
use crate::table::serialization::SerializedTableCollection;
use crate::table::{Table, TableId};
use crate::util::checked_u32;
use rlb_error::{Error, Result};
use slotmap::SlotMap;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) struct TableCollection {
    map: SlotMap<TableId, Table>,
}

impl TableCollection {
    pub(crate) fn new() -> TableCollection {
        Self {
            map: SlotMap::with_key(),
        }
    }

    pub(crate) fn insert(&mut self, table: Table) -> TableId {
        self.map.insert(table)
    }

    pub(crate) fn serialize(&self) -> Result<SerializedTableCollection> {
        let mut serialized_tables = Vec::with_capacity(self.map.len());
        for (id, table) in &self.map {
            serialized_tables.push((id, table.serialize()?));
        }

        let mut strings = StringPool::new();
        for (_, table) in &serialized_tables {
            for fixup in &table.string_fixups {
                strings.intern(fixup.value)?;
            }
        }

        let mut data = strings.data().to_vec();
        let mut id_to_offset: HashMap<TableId, usize> = HashMap::with_capacity(self.map.len());
        let mut relocation_offsets: Vec<u32> = Vec::new();

        for (id, mut table) in serialized_tables {
            let table_offset = data.len();
            id_to_offset.insert(id, table_offset);

            for fixup in table.string_fixups {
                let string_offset = strings.offset_of(fixup.value).ok_or_else(|| {
                    Error::Validation(format!(
                        "string {:?} is missing from the serialized string pool",
                        fixup.value
                    ))
                })?;
                let pointer = checked_u32(string_offset, "converting string pool offset")?;
                let end = fixup.offset + size_of::<u32>();
                table.data[fixup.offset..end].copy_from_slice(&pointer.to_be_bytes());
                relocation_offsets.push(checked_u32(
                    table_offset + fixup.offset,
                    "converting string pointer relocation offset",
                )?);
            }

            data.extend_from_slice(&table.data);
        }

        Ok(SerializedTableCollection::new(
            data,
            id_to_offset,
            relocation_offsets,
        ))
    }

    pub(crate) fn get(&self, id: TableId) -> Option<&Table> {
        self.map.get(id)
    }
    pub(crate) fn get_mut(&mut self, id: TableId) -> Option<&mut Table> {
        self.map.get_mut(id)
    }
}
