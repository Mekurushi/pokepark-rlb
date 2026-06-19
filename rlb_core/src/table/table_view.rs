use crate::error::{Error, Result};
use crate::format::RawFile;
use crate::schema::TableEntry;

#[derive(Debug, Clone)]
pub struct TableView<T> {
    pub name: String,
    pub root_address: u32,
    pub entries: Vec<T>,
    pub terminator: T,
}

impl<T: TableEntry> TableView<T> {
    pub(crate) fn discover(raw: &RawFile, name: String, root_address: u32) -> Result<Self> {
        let mut entries = Vec::new();
        let mut offset = root_address as usize;

        loop {
            let record_bytes =
                raw.data
                    .get(offset..offset + T::SIZE)
                    .ok_or_else(|| Error::MissingTerminator {
                        table: name.clone(),
                        root_address,
                    })?;
            let record = T::read(record_bytes)?;
            if record.is_terminator() && !raw.relocations.contains(&(offset as u32)) {
                return Ok(TableView {
                    name,
                    root_address,
                    entries,
                    terminator: record,
                });
            }

            entries.push(record);
            offset += T::SIZE;
        }
    }

    pub(crate) fn write_into(&self, data: &mut [u8]) -> Result<()> {
        let mut offset = self.root_address as usize;
        for entry in self.entries.iter().chain(std::iter::once(&self.terminator)) {
            let slice = data
                .get_mut(offset..offset + T::SIZE)
                .ok_or(Error::UnexpectedEof {
                    context: "writing table entry",
                })?;
            entry.write_into(slice)?;
            offset += T::SIZE;
        }
        Ok(())
    }
}
