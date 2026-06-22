use crate::table_entry::layouts::{BackFromAttractionScriptList, SCRIPT_LIST_FIELDS};
use crate::table_entry::{FieldDescriptor, TableEntry};
use crate::value::Value;
use rlb_error::Result;

impl TableEntry for BackFromAttractionScriptList {
    fn type_name(&self) -> &'static str {
        "BackFromAttractionScriptList"
    }

    fn fields(&self) -> &[FieldDescriptor] {
        SCRIPT_LIST_FIELDS
    }

    fn get(&self, _field: &str) -> Option<Value> {
        todo!("not implemented");
    }

    fn set(&mut self, _field: &str, _value: Value) -> Result<()> {
        todo!("not implemented");
    }
}
