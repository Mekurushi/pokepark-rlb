pub mod script_list;
pub mod single_pointer;

use crate::Value;
use crate::rlb_file::StringId;
use rlb_error::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    pub name: &'static str,
}

pub trait TableEntry: Sized + std::fmt::Debug {
    fn type_name() -> &'static str;

    fn fields(&self) -> &[FieldDescriptor];
    fn is_terminator(&self) -> bool;

    fn get(&self, field: &str) -> Option<Value>;

    fn set(&mut self, field: &str, value: Value) -> Result<()>;

    fn size() -> usize;
    fn read<R, E>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool;
}
