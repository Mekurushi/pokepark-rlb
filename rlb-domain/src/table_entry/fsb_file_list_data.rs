use crate::table_entry::layouts::{FSB_FILE_LIST_FIELDS, FsbFileListDataEntry};
use crate::table_entry::{FieldDescriptor, TableEntry};
use crate::value::Value;
use rlb_error::Result;

impl TableEntry for FsbFileListDataEntry {
    fn type_name(&self) -> &'static str {
        "FsbFileListData"
    }

    fn fields(&self) -> &[FieldDescriptor] {
        FSB_FILE_LIST_FIELDS
    }

    fn get(&self, _field: &str) -> Option<Value> {
        todo!("not implemented");
    }

    fn set(&mut self, _field: &str, _value: Value) -> Result<()> {
        todo!("not implemented");
    }
}
