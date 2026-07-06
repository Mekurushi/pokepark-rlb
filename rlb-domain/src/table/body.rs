use crate::rlb_file::StringId;
use crate::string_pool::SerializedStringPoolContext;
use crate::{FieldDescriptor, Value};
use rlb_error::Result;

pub(crate) trait TableBody: Sized + std::fmt::Debug + Clone {
    fn discover<R, E>(
        data: &[u8],
        root_address: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool;

    fn serialize(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &SerializedStringPoolContext<StringId>,
        relocations: &mut Vec<u32>,
    ) -> Result<()>;

    fn fields(&self) -> &'static [FieldDescriptor];

    fn entry_count(&self) -> usize;

    fn get_field(&self, index: usize, field: &str) -> Option<Value>;

    fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()>;
}
