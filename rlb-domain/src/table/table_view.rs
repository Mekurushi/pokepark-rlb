use crate::entry_schemas::{EntryDeserializer, EntrySerializer};
use crate::rlb_file::StringId;
use crate::string_pool::SerializedStringPoolContext;
use crate::TableEntry;
use rlb_error::{Error, Result};

#[derive(Debug, Clone)]
pub struct TableView<T> {
    pub entries: Vec<T>,
    pub terminator: T,
}
impl<T: TableEntry> TableView<T> {
    pub fn discover<R, E>(
        data: &[u8],
        root_address: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool,
    {
        let mut entries = Vec::new();
        let mut offset = root_address;

        loop {
            let record_bytes = data
                .get(offset..offset + T::SIZE)
                .ok_or(Error::UnexpectedEof {
                    context: "parsing table record",
                })?;
            let mut de = EntryDeserializer::new(record_bytes, offset, resolve_string, is_relocated);
            let record = T::read(&mut de)?;
            if record.is_terminator() {
                return Ok(TableView {
                    entries,
                    terminator: record,
                });
            }

            entries.push(record);
            offset += T::SIZE;
        }
    }

    pub(crate) fn serialize(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &SerializedStringPoolContext<StringId>,
        relocations: &mut Vec<u32>,
    ) -> Result<()> {
        for (i, entry) in self.entries.iter().enumerate() {
            let entry_offset = base_offset + i * T::SIZE;
            let mut ser = EntrySerializer::new(entry_offset, strings, relocations);
            entry.write(&mut ser)?;
            ser.finish(out, T::SIZE)?;
        }

        let terminator_offset = base_offset + self.entries.len() * T::SIZE;
        let mut ser = EntrySerializer::new(terminator_offset, strings, relocations);
        self.terminator.write(&mut ser)?;
        ser.finish(out, T::SIZE)?;

        Ok(())
    }
}
