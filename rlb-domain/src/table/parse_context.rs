use crate::relocation::RelocationTable;
use crate::util::resolve_string_from_raw_data;
use rlb_error::{Error, Result};

pub(crate) struct ParseContext<'a> {
    data: &'a [u8],
    relocations: &'a RelocationTable,
}

impl<'a> ParseContext<'a> {
    pub(crate) fn new(data: &'a [u8], relocations: &'a RelocationTable) -> Self {
        Self { data, relocations }
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

    pub(crate) fn is_relocated(&self, offset: u32) -> bool {
        self.relocations.is_relocated(offset)
    }
}
