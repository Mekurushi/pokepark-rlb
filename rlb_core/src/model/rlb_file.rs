use std::path::Path;

use super::reference::Reference;
use super::table_view::TableView;
use crate::error::{Error, Result};
use crate::format::{EntrySlot, RawFile};
use crate::schema::{ENTRY_SIZE, TABLE_NAMES};

pub struct RlbFile {
    raw: RawFile,
    tables: Vec<TableView>,
}

impl RlbFile {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let raw = RawFile::parse(bytes)?;
        let tables = discover_tables(&raw)?;
        Ok(Self { raw, tables })
    }

    pub fn tables(&self) -> impl Iterator<Item = &TableView> {
        self.tables.iter()
    }

    pub fn table(&self, name: &str) -> Result<&TableView> {
        self.tables
            .iter()
            .find(|t| t.name == name)
            .ok_or_else(|| Error::TableNotFound(name.to_string()))
    }

    pub fn table_mut(&mut self, name: &str) -> Result<&mut TableView> {
        self.tables
            .iter_mut()
            .find(|t| t.name == name)
            .ok_or_else(|| Error::TableNotFound(name.to_string()))
    }

    pub fn resolve(&self, reference: Reference) -> Option<(&TableView, usize)> {
        self.tables.iter().find_map(|view| {
            let start = view.root_address as u64;
            let end = start + view.record_count() as u64 * ENTRY_SIZE as u64;
            let addr = reference.0 as u64;
            (addr >= start && addr < end)
                .then(|| (view, ((addr - start) / ENTRY_SIZE as u64) as usize))
        })
    }

    pub fn set_entry_field(
        &mut self,
        table: &str,
        index: usize,
        field: &str,
        value: i32,
    ) -> Result<()> {
        let view = self.table_mut(table)?;
        let entry = view
            .entries
            .get_mut(index)
            .ok_or_else(|| Error::IndexOutOfRange {
                table: table.to_string(),
                index,
            })?;
        entry.set_i32_field(field, value)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut raw = self.raw.clone();
        for table in &self.tables {
            table.write_into(&mut raw.data)?;
        }
        raw.write()
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let bytes = self.to_bytes()?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
}

fn discover_tables(raw: &RawFile) -> Result<Vec<TableView>> {
    let mut tables = Vec::new();
    for entry in &raw.entries {
        if let EntrySlot::Named {
            address,
            name_offset,
        } = entry
        {
            let name = raw.resolve_name(*name_offset)?;
            if TABLE_NAMES.contains(&name.as_str()) {
                tables.push(TableView::discover(raw, name, *address)?);
            }
        }
    }
    Ok(tables)
}
