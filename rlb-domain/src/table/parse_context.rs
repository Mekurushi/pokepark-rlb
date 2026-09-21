use crate::relocation::RelocationTable;
use crate::util::resolve_string_from_raw_data;
use rlb_error::{Error, Result};
use std::collections::HashMap;

pub(crate) struct ParseContext<'a> {
    data: &'a [u8],
    relocations: &'a RelocationTable,
    table_offsets: HashMap<String, usize>,
}

impl<'a> ParseContext<'a> {
    pub(crate) fn new<'name>(
        data: &'a [u8],
        relocations: &'a RelocationTable,
        tables: impl IntoIterator<Item = (&'name str, usize)>,
    ) -> Self {
        let mut table_offsets = HashMap::new();
        for (label, offset) in tables {
            table_offsets.entry(label.to_owned()).or_insert(offset);
        }

        Self {
            data,
            relocations,
            table_offsets,
        }
    }

    pub(crate) fn bytes_at(&self, offset: usize, length: usize) -> Result<&'a [u8]> {
        let end = offset.checked_add(length).ok_or(Error::OffsetOutOfBounds {
            context: "table data",
            offset,
            length: self.data.len(),
        })?;

        self.data.get(offset..end).ok_or(Error::OffsetOutOfBounds {
            context: "table data",
            offset,
            length: self.data.len(),
        })
    }

    pub(crate) fn resolve_string(&self, offset: u32) -> Result<String> {
        resolve_string_from_raw_data(self.data, offset as usize)
    }

    pub(crate) fn read_u8(&self, offset: usize) -> Result<u8> {
        Ok(self.bytes_at(offset, 1)?[0])
    }

    pub(crate) fn is_relocated(&self, offset: u32) -> bool {
        self.relocations.is_relocated(offset)
    }

    pub(crate) fn table_offset(&self, label: &str) -> Result<usize> {
        self.table_offsets
            .get(label)
            .copied()
            .ok_or_else(|| Error::Validation(format!("RLB table entry {label:?} was not found")))
    }
}
