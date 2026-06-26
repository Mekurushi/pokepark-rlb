use crate::TableEntry;
use crate::rlb_file::StringId;
use crate::util::value_at;
use crate::{FieldDescriptor, Value, impl_table_entry_wrapper};
use rlb_error::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct SinglePointerEntry {
    pub script_name: Value,
}

impl SinglePointerEntry {
    pub fn read<R, E>(
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
    pub fn is_terminator(&self) -> bool {
        self.script_name == Value::Integer(0)
    }
    pub fn get(&self, field: &str) -> Option<Value> {
        match field {
            "script_name" => Some(self.script_name),
            _ => None,
        }
    }

    pub fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "script_name" => {
                self.script_name = value;
            }
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }

    pub fn size() -> usize {
        0x4
    }
}

pub const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
}];

impl_table_entry_wrapper! {
    struct FsbFileListDataEntry(SinglePointerEntry);

    type_name = "FsbFileListData";
    fields = FSB_FILE_LIST_FIELDS;
}
