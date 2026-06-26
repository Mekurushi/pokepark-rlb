use crate::rlb_file::StringId;
use crate::TableEntry;
use rlb_error::{Error, Result};
use rlb_format::RelocationTable;

#[derive(Debug, Clone)]
pub struct TableView<T> {
    pub entries: Vec<T>,
    pub terminator: T,
}
impl<T: TableEntry> TableView<T> {
    pub fn discover<R>(
        data: &[u8],
        root_address: usize,
        resolve_string: &mut R,
        relocations: &RelocationTable,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
    {
        let mut entries = Vec::new();
        let mut offset = root_address;

        loop {
            let record_bytes = data
                .get(offset..offset + T::size())
                .ok_or(Error::UnexpectedEof { context: "parsing table record" })?;

            let record = T::read(record_bytes, offset, resolve_string, relocations)?;
            if record.is_terminator() {
                return Ok(TableView {
                    entries,
                    terminator: record,
                });
            }

            entries.push(record);
            offset += T::size();
        }
    }
}
