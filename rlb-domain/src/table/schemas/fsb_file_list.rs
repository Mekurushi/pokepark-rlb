use crate::string_pool::StringPool;
use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::field::{FieldConstraint, FieldKind};
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct FsbFileListTable {
    entries: Vec<FsbFileListData>,
    terminator: FsbFileListData,
}

impl FsbFileListTable {
    const ENTRY_SIZE: usize = FsbFileListData::SIZE;

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
            let candidate = FsbFileListData::read(&mut de)?;

            if candidate.script_name == Value::String(None) {
                return Ok(Self {
                    entries,
                    terminator: candidate,
                });
            }

            entries.push(candidate);
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
            entry.write(&mut serializer)?;
            serializer.finish(out, Self::ENTRY_SIZE)?;
        }

        let mut serializer = EntrySerializer::new(
            base_offset + self.entries.len() * Self::ENTRY_SIZE,
            strings,
            relocations,
        );
        self.terminator.write(&mut serializer)?;
        serializer.finish(out, Self::ENTRY_SIZE)
    }

    pub(crate) fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        for entry in &self.entries {
            entry.visit_strings(visit)?;
        }
        self.terminator.visit_strings(visit)
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

impl FsbFileListData {
    pub(crate) const SIZE: usize = 0x4;
    pub(crate) const FIELDS: &'static [FieldDescriptor] = FSB_FILE_LIST_FIELDS;

    pub(crate) fn get(&self, field: &str) -> Option<Value> {
        match field {
            "script_name" => Some(self.script_name.clone()),
            _ => None,
        }
    }

    pub(crate) fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "script_name" => {
                self.script_name = value;
            }
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }

    pub(crate) fn read<R, E>(de: &mut EntryDeserializer<'_, R, E>) -> Result<Self>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
        let script = de.read_string_pointer()?;

        Ok(Self {
            script_name: script,
        })
    }
    pub(crate) fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        ser.write_string_pointer(&self.script_name)?;
        Ok(())
    }

    pub(crate) fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        if let Value::String(Some(value)) = &self.script_name {
            visit(value)?;
        }
        Ok(())
    }
}

pub const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
    description: "",
    kind: FieldKind::String,
    constraint: FieldConstraint::None,
}];
