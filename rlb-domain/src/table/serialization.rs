use crate::table::TableId;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct StringFixup<'a> {
    pub(crate) offset: usize,
    pub(crate) value: &'a str,
}

#[derive(Debug, Default)]
pub(crate) struct RelocatableTable<'a> {
    pub(crate) data: Vec<u8>,
    pub(crate) string_fixups: Vec<StringFixup<'a>>,
}

#[derive(Debug)]
pub(crate) struct SerializedTableCollection {
    data: Vec<u8>,
    id_to_offset: HashMap<TableId, usize>,
    relocations: Vec<u32>,
}

impl SerializedTableCollection {
    pub(crate) fn new(
        data: Vec<u8>,
        id_to_offset: HashMap<TableId, usize>,
        relocations: Vec<u32>,
    ) -> Self {
        Self {
            data,
            id_to_offset,
            relocations,
        }
    }

    pub(crate) fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub(crate) fn offset_of(&self, id: TableId) -> Option<usize> {
        self.id_to_offset.get(&id).copied()
    }

    pub(crate) fn relocations(&self) -> &Vec<u32> {
        &self.relocations
    }
}
