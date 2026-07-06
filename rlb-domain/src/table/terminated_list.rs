use crate::entry_schemas::{EntryDeserializer, EntrySerializer, TableEntry, Terminator};
use crate::rlb_file::StringId;
use crate::string_pool::SerializedStringPoolContext;
use crate::table::body::TableBody;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Debug, Clone)]
pub(crate) struct TerminatedList<Entry, Term> {
    entries: Vec<Entry>,
    terminator: Term,
}

impl<Entry, Term> TableBody for TerminatedList<Entry, Term>
where
    Entry: TableEntry + std::clone::Clone,
    Term: Terminator,
{
    fn discover<R, E>(
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
            if let Some(term_bytes) = data.get(offset..offset + Term::SIZE) {
                let mut de =
                    EntryDeserializer::new(term_bytes, offset, resolve_string, is_relocated);
                if let Some(terminator) = Term::recognize(&mut de)? {
                    return Ok(Self {
                        entries,
                        terminator,
                    });
                }
            }

            let entry_bytes =
                data.get(offset..offset + Entry::SIZE)
                    .ok_or(Error::UnexpectedEof {
                        context: "parsing table record",
                    })?;
            let mut de = EntryDeserializer::new(entry_bytes, offset, resolve_string, is_relocated);
            entries.push(Entry::read(&mut de)?);
            offset += Entry::SIZE;
        }
    }

    fn serialize(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &SerializedStringPoolContext<StringId>,
        relocations: &mut Vec<u32>,
    ) -> Result<()> {
        for (i, entry) in self.entries.iter().enumerate() {
            let entry_offset = base_offset + i * Entry::SIZE;
            let mut ser = EntrySerializer::new(entry_offset, strings, relocations);
            entry.write(&mut ser)?;
            ser.finish(out, Entry::SIZE)?;
        }

        let terminator_offset = base_offset + self.entries.len() * Entry::SIZE;
        let mut ser = EntrySerializer::new(terminator_offset, strings, relocations);
        self.terminator.write(&mut ser)?;
        ser.finish(out, Term::SIZE)?;

        Ok(())
    }

    fn fields(&self) -> &'static [FieldDescriptor] {
        Entry::FIELDS
    }

    fn entry_count(&self) -> usize {
        self.entries.len()
    }

    fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        self.entries.get(index)?.get(field)
    }

    fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        let entry = self
            .entries
            .get_mut(index)
            .ok_or_else(|| Error::Validation(format!("entry index {index} out of bounds")))?;
        entry.set(field, value)
    }
}
