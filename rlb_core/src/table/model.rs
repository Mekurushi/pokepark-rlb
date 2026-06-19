use crate::Error;
use crate::error::Result;
use crate::schema::{PointerTableEntry, ScriptListTableEntry, TableEntry};
use crate::table::kind::TableKind;
use std::iter::Enumerate;

pub enum EntryRef<'a> {
    Script(&'a ScriptListTableEntry),
    Pointer(&'a PointerTableEntry),
}

pub enum EntryIter<'a> {
    Script(Enumerate<std::slice::Iter<'a, ScriptListTableEntry>>),
    Pointer(Enumerate<std::slice::Iter<'a, PointerTableEntry>>),
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = (usize, EntryRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EntryIter::Script(iter) => iter.next().map(|(i, e)| (i, EntryRef::Script(e))),
            EntryIter::Pointer(iter) => iter.next().map(|(i, e)| (i, EntryRef::Pointer(e))),
        }
    }
}

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
            TableKind::PointerTable(t) => t.write_into(data),
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
            TableKind::PointerTable(t) => {
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
            TableKind::PointerTable(t) => t.entries.len(),
        }
    }

    fn _is_empty(&self) -> bool {
        match &self.inner {
            TableKind::ScriptList(t) => t.entries.is_empty(),
            TableKind::PointerTable(t) => t.entries.is_empty(),
        }
    }

    pub fn get(&self, index: usize) -> Result<EntryRef<'_>> {
        match &self.inner {
            TableKind::ScriptList(t) => {
                Ok(EntryRef::Script(t.entries.get(index).ok_or_else(|| {
                    Error::IndexOutOfRange {
                        table: self.name.clone(),
                        index,
                    }
                })?))
            }
            TableKind::PointerTable(t) => Ok(EntryRef::Pointer(t.entries.get(index).ok_or_else(
                || Error::IndexOutOfRange {
                    table: self.name.clone(),
                    index,
                },
            )?)),
        }
    }

    pub fn iter_entries(&self) -> EntryIter<'_> {
        match &self.inner {
            TableKind::ScriptList(t) => EntryIter::Script(t.entries.iter().enumerate()),
            TableKind::PointerTable(t) => EntryIter::Pointer(t.entries.iter().enumerate()),
        }
    }
}
