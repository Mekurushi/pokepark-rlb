use std::path::Path;

use crate::error::{Error, Result};
use crate::format::{EntrySlot, RawFile};
use crate::table::model::Table;
use crate::table::schema::SCHEMAS;

pub struct RlbFile {
    raw: RawFile,
    tables: Vec<Table>,
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

    pub fn tables(&self) -> impl Iterator<Item = &Table> {
        self.tables.iter()
    }
    pub fn table(&self, name: &str) -> Result<&Table> {
        self.tables
            .iter()
            .find(|t| t.name() == name)
            .ok_or_else(|| Error::TableNotFound(name.to_string()))
    }

    pub fn table_mut(&mut self, name: &str) -> Result<&mut Table> {
        self.tables
            .iter_mut()
            .find(|t| t.name() == name)
            .ok_or_else(|| Error::TableNotFound(name.to_string()))
    }

    pub fn set_entry_field(
        &mut self,
        table: &str,
        index: usize,
        field: &str,
        value: i32,
    ) -> Result<()> {
        let table = self.table_mut(table)?;
        table.set_entry_field(index, field, value)
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

fn discover_tables(raw: &RawFile) -> Result<Vec<Table>> {
    let mut tables = Vec::new();
    for entry in &raw.entries {
        if let EntrySlot::Named {
            address,
            name_offset,
        } = entry
        {
            let name = raw.resolve_name(*name_offset)?;
            for schema in SCHEMAS {
                if (schema.matches)(&name) {
                    tables.push((schema.discover)(raw, name.clone(), *address)?);
                    break;
                }
            }
        }
    }
    Ok(tables)
}
