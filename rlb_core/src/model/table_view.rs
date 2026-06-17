use crate::error::{Error, Result};
use crate::format::RawFile;
use crate::schema::{ENTRY_SIZE, ScriptListTableEntry};

#[derive(Debug, Clone)]
pub struct TableView {
    pub name: String,
    pub root_address: u32,
    pub entries: Vec<ScriptListTableEntry>,
    pub terminator: ScriptListTableEntry,
}

impl TableView {
    pub(crate) fn discover(raw: &RawFile, name: String, root_address: u32) -> Result<Self> {
        let mut entries = Vec::new();
        let mut offset = root_address as usize;

        loop {
            let record_bytes = raw.data.get(offset..offset + ENTRY_SIZE).ok_or_else(|| {
                Error::MissingTerminator {
                    table: name.clone(),
                    root_address,
                }
            })?;
            let record = ScriptListTableEntry::read_be(record_bytes)?;

            if record.is_terminator() {
                return Ok(TableView {
                    name,
                    root_address,
                    entries,
                    terminator: record,
                });
            }

            entries.push(record);
            offset += ENTRY_SIZE;
        }
    }
    pub fn record_count(&self) -> usize {
        self.entries.len() + 1
    }

    pub(crate) fn write_into(&self, data: &mut [u8]) -> Result<()> {
        let mut offset = self.root_address as usize;
        for entry in self.entries.iter().chain(std::iter::once(&self.terminator)) {
            let slice = data
                .get_mut(offset..offset + ENTRY_SIZE)
                .ok_or(Error::UnexpectedEof {
                    context: "writing table entry",
                })?;
            entry.write_be_into(slice)?;
            offset += ENTRY_SIZE;
        }
        Ok(())
    }
}
