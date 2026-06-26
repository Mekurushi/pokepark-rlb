use crate::entry_schemas::single_pointer::{FSB_FILE_LIST_FIELDS, SinglePointerEntry};
use crate::rlb_file::StringId;
use crate::value::Value;
use crate::{FieldDescriptor, TableEntry};
use rlb_error::Result;

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
        SinglePointerEntry::size()
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
        SinglePointerEntry::read(data, base_offset, resolve_string, is_relocated).map(Self)
    }
}
