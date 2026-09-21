use crate::TableEntry;
use crate::entry_schemas::codec::{EntryDeserializer, EntrySerializer};
use crate::entry_schemas::{FieldConstraint, FieldKind, Terminator};
use crate::string_pool::StringPool;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct FsbFileListTable {
    entries: Vec<FsbFileListData>,
    terminator: FsbFileListData,
}

impl FsbFileListTable {
    const ENTRY_SIZE: usize = <FsbFileListData as TableEntry>::SIZE;

    pub(crate) fn parse<R, E>(
        data: &[u8],
        root_address: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
        let mut entries = Vec::new();
        let mut offset = root_address;

        loop {
            let bytes =
                data.get(offset..offset + Self::ENTRY_SIZE)
                    .ok_or(Error::UnexpectedEof {
                        context: "parsing FsbFileList record",
                    })?;
            let mut de = EntryDeserializer::new(bytes, offset, resolve_string, is_relocated);

            if let Some(terminator) = <FsbFileListData as Terminator>::recognize(&mut de)? {
                return Ok(Self {
                    entries,
                    terminator,
                });
            }

            let mut de = EntryDeserializer::new(bytes, offset, resolve_string, is_relocated);
            entries.push(FsbFileListData::read(&mut de)?);
            offset += Self::ENTRY_SIZE;
        }
    }

    pub(crate) fn serialize(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &StringPool,
        relocations: &mut Vec<u32>,
    ) -> Result<()> {
        for (index, entry) in self.entries.iter().enumerate() {
            let mut serializer =
                EntrySerializer::new(base_offset + index * Self::ENTRY_SIZE, strings, relocations);
            <FsbFileListData as TableEntry>::write(entry, &mut serializer)?;
            serializer.finish(out, Self::ENTRY_SIZE)?;
        }

        let mut serializer = EntrySerializer::new(
            base_offset + self.entries.len() * Self::ENTRY_SIZE,
            strings,
            relocations,
        );
        <FsbFileListData as Terminator>::write(&self.terminator, &mut serializer)?;
        serializer.finish(out, <FsbFileListData as Terminator>::SIZE)
    }

    pub(crate) fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        for entry in &self.entries {
            <FsbFileListData as TableEntry>::visit_strings(entry, visit)?;
        }
        <FsbFileListData as Terminator>::visit_strings(&self.terminator, visit)
    }

    pub(crate) fn fields(&self) -> &'static [FieldDescriptor] {
        FsbFileListData::FIELDS
    }

    pub(crate) fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        self.entries.get(index)?.get(field)
    }

    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        let entry = self
            .entries
            .get_mut(index)
            .ok_or_else(|| Error::Validation(format!("entry index {index} out of bounds")))?;
        entry.set(field, value)
    }
}

#[derive(Clone, Debug)]
pub struct FsbFileListData {
    pub script_name: Value,
}

impl TableEntry for FsbFileListData {
    const SIZE: usize = 0x4;
    const FIELDS: &'static [FieldDescriptor] = FSB_FILE_LIST_FIELDS;
    fn get(&self, field: &str) -> Option<Value> {
        match field {
            "script_name" => Some(self.script_name.clone()),
            _ => None,
        }
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "script_name" => {
                self.script_name = value;
            }
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }

    fn read<R, E>(de: &mut EntryDeserializer<'_, R, E>) -> Result<Self>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
        let script = de.read_string_pointer()?;

        Ok(Self {
            script_name: script,
        })
    }
    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        ser.write_string_pointer(&self.script_name)?;
        Ok(())
    }

    fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        if let Value::String(Some(value)) = &self.script_name {
            visit(value)?;
        }
        Ok(())
    }
}

impl Terminator for FsbFileListData {
    const SIZE: usize = <Self as TableEntry>::SIZE;

    fn recognize<R, E>(de: &mut EntryDeserializer<'_, R, E>) -> Result<Option<Self>>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
        let candidate = <Self as TableEntry>::read(de)?;
        let is_terminator = candidate.script_name == Value::String(None);

        Ok(is_terminator.then_some(candidate))
    }

    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        <Self as TableEntry>::write(self, ser)
    }

    fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        <Self as TableEntry>::visit_strings(self, visit)
    }
}

pub const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
    description: "",
    kind: FieldKind::String,
    constraint: FieldConstraint::None,
}];
