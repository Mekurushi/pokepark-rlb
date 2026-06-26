use crate::rlb_file::StringId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Integer(u32),
    Pointer(StringId),
}
