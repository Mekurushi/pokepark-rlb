use crate::rlb_file::StringId;
use crate::table_entry::layouts::{FsbFileListDataEntry, SinglePointerEntry, FSB_FILE_LIST_FIELDS};
use crate::table_entry::{FieldDescriptor, TableEntry};
use crate::value::Value;
use rlb_error::{Error, Result};
use rlb_format::RelocationTable;

impl TableEntry for FsbFileListDataEntry {
    fn type_name() -> &'static str {
        "FsbFileListData"
    }

    fn fields(&self) -> &[FieldDescriptor] {
        FSB_FILE_LIST_FIELDS
    }

    fn is_terminator(&self) -> bool {
        self.0.script_name == Value::Integer(0)
    }

    fn get(&self, field: &str) -> Option<Value> {
        match field {
            "script_name" => Some(self.0.script_name),
            _ => None,
        }
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "script_name" => {
                self.0.script_name = value;
            }
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
    fn size() -> usize {
        0x4
    }

    fn read<R>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        relocation_table: &RelocationTable,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
    {
        SinglePointerEntry::read(data, base_offset, resolve_string, relocation_table).map(Self)
    }
}
