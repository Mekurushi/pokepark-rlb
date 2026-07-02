use crate::entry_schemas::codec::{EntryDeserializer, EntrySerializer};
use crate::rlb_file::StringId;
use crate::TableEntry;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct FsbFileListData {
    pub script_name: Value,
}

impl TableEntry for FsbFileListData {
    const SIZE: usize = 0x4;
    fn fields(&self) -> &[FieldDescriptor] {
        FSB_FILE_LIST_FIELDS
    }
    fn is_terminator(&self) -> bool {
        self.script_name == Value::String(None)
    }
    fn get(&self, field: &str) -> Option<Value> {
        match field {
            "script_name" => Some(self.script_name),
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
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool,
    {
        let script = de.read_string_pointer()?;

        Ok(Self {
            script_name: script,
        })
    }
    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        ser.write_string_pointer(self.script_name)?;
        Ok(())
    }
}

pub const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
}];
