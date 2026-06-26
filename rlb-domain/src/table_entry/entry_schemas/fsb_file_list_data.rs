use crate::rlb_file::StringId;
use crate::table_entry::layouts::{SinglePointerEntry, FSB_FILE_LIST_FIELDS};
use crate::table_entry::{FieldDescriptor, TableEntry};
use crate::value::Value;
use rlb_error::Result;
use rlb_format::RelocationTable;
#[derive(Debug, Clone)]
pub struct FsbFileListDataEntry(pub SinglePointerEntry);
impl TableEntry for FsbFileListDataEntry {
    fn type_name() -> &'static str {
        "FsbFileListData"
    }

    fn fields(&self) -> &[FieldDescriptor] {
        FSB_FILE_LIST_FIELDS
    }

    fn is_terminator(&self) -> bool {
        SinglePointerEntry::is_terminator(&self.0)
    }

    fn get(&self, field: &str) -> Option<Value> {
        SinglePointerEntry::get(&self.0, field)
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        SinglePointerEntry::set(&mut self.0, field, value)
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
