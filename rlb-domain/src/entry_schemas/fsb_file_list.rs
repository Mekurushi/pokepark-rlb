use crate::rlb_file::StringId;
use crate::string_pool::SerializedStringPoolContext;
use crate::util::{value_at, write_value};
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

    fn read<R, E>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool,
    {
        Ok(Self {
            script_name: value_at(data, 0x00, base_offset, resolve_string, is_relocated)?,
        })
    }
    fn write(
        &self,
        base_offset: usize,
        strings: &SerializedStringPoolContext<StringId>,
        relocations: &mut Vec<u32>,
    ) -> Result<Vec<u8>> {
        let mut entry: Vec<u8> = Vec::with_capacity(Self::SIZE);

        write_value(
            self.script_name,
            0x00,
            base_offset,
            &mut entry,
            strings,
            relocations,
        )?;
        if entry.len() != Self::SIZE {
            return Err(Error::SerializationMismatch {
                expected: Self::SIZE as u32,
                actual: entry.len(),
            });
        }
        Ok(entry)
    }
}

pub const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
}];
