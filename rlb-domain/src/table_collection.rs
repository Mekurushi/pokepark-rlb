use crate::rlb_file::TableId;
use crate::string_pool::StringPool;
use crate::table::Table;
use rlb_error::Result;
use slotmap::SlotMap;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TableCollection {
    map: SlotMap<TableId, Table>,
}

#[derive(Debug)]
pub(crate) struct SerializedTableContext {
    data: Vec<u8>,
    id_to_offset: HashMap<TableId, usize>,
    relocations: Vec<u32>,
}

impl SerializedTableContext {
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub(crate) fn offset_of(&self, id: TableId) -> Option<usize> {
        self.id_to_offset.get(&id).copied()
    }

    pub fn relocations(&self) -> &Vec<u32> {
        &self.relocations
    }
}

impl TableCollection {
    pub fn new() -> TableCollection {
        Self {
            map: SlotMap::with_key(),
        }
    }

    pub fn insert(&mut self, table: Table) -> TableId {
        self.map.insert(table)
    }

    pub(crate) fn collect_strings(&self, strings: &mut StringPool) -> Result<()> {
        for (_, table) in &self.map {
            table.visit_strings(&mut |value| strings.intern(value).map(|_| ()))?;
        }
        Ok(())
    }

    pub fn serialize(&self, strings: &StringPool) -> Result<SerializedTableContext> {
        let mut data: Vec<u8> = Vec::new();
        let mut id_to_offset: HashMap<TableId, usize> = HashMap::with_capacity(self.map.len());
        let mut relocation_offsets: Vec<u32> = Vec::new();

        for (id, table) in &self.map {
            // strings and tables are in the same data blob
            let table_offset = data.len() + strings.data().len();
            id_to_offset.insert(id, table_offset);
            table.serialize_into(&mut data, table_offset, strings, &mut relocation_offsets)?;
        }

        Ok(SerializedTableContext {
            data,
            id_to_offset,
            relocations: relocation_offsets,
        })
    }

    pub(crate) fn get(&self, id: TableId) -> Option<&Table> {
        self.map.get(id)
    }
    pub(crate) fn get_mut(&mut self, id: TableId) -> Option<&mut Table> {
        self.map.get_mut(id)
    }
}
