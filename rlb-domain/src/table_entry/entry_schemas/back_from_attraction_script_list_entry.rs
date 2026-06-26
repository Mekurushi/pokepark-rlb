use crate::rlb_file::StringId;
use crate::table_entry::layouts::{ScriptListEntry, SCRIPT_LIST_FIELDS};
use crate::table_entry::{FieldDescriptor, TableEntry};
use crate::value::Value;
use rlb_error::Result;
use rlb_format::RelocationTable;

#[derive(Debug, Clone)]
pub struct BackFromAttractionScriptList(pub ScriptListEntry);

impl TableEntry for BackFromAttractionScriptList {
    fn type_name() -> &'static str {
        "BackFromAttractionScriptList"
    }

    fn fields(&self) -> &[FieldDescriptor] {
        SCRIPT_LIST_FIELDS
    }

    fn is_terminator(&self) -> bool {
        ScriptListEntry::is_terminator(&self.0)
    }

    fn get(&self, field: &str) -> Option<Value> {
        ScriptListEntry::get(&self.0, field)
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        ScriptListEntry::set(&mut self.0, field, value)
    }
    fn size() -> usize {
        0x44
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
        ScriptListEntry::read(data, base_offset, resolve_string, relocation_table).map(Self)
    }
}
