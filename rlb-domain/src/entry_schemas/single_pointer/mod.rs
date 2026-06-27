use crate::TableEntry;
use crate::rlb_file::StringId;
use crate::util::value_at;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct SinglePointerEntry {
    pub script_name: Value,
}

impl TableEntry for SinglePointerEntry {
    const SIZE: usize = 0x4;
    fn fields(&self) -> &[FieldDescriptor] {
        FSB_FILE_LIST_FIELDS
    }
    fn is_terminator(&self) -> bool {
        self.script_name == Value::Integer(0)
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
}

pub const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
}];
