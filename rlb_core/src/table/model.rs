use crate::Error;
use crate::error::Result;
use crate::schema::{ScriptListTableEntry, TableEntry};
use crate::table::kind::TableKind;
use std::iter::Enumerate;

pub struct Table {
    name: String,
    _root_address: u32,
    inner: TableKind,
}

impl Table {
    pub fn new(name: String, root_address: u32, inner: TableKind) -> Self {
        Self {
            name,
            _root_address: root_address,
            inner,
        }
    }

    pub fn write_into(&self, data: &mut [u8]) -> Result<()> {
        match &self.inner {
            TableKind::ScriptList(t) => t.write_into(data),
        }
    }

    pub fn set_entry_field(&mut self, index: usize, field: &str, value: i32) -> Result<()> {
        match &mut self.inner {
            TableKind::ScriptList(t) => {
                let entry = t
                    .entries
                    .get_mut(index)
                    .ok_or_else(|| Error::IndexOutOfRange {
                        table: self.name.clone(),
                        index,
                    })?;
                entry.set_field(field, value)
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        match &self.inner {
            TableKind::ScriptList(t) => t.entries.len(),
        }
    }

    fn _is_empty(&self) -> bool {
        match &self.inner {
            TableKind::ScriptList(t) => t.entries.is_empty(),
        }
    }

    pub fn iter_entries(&self) -> Enumerate<std::slice::Iter<'_, ScriptListTableEntry>> {
        match &self.inner {
            TableKind::ScriptList(t) => t.entries.iter().enumerate(),
        }
    }
}
