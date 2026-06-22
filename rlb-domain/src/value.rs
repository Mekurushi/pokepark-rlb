#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataOffset(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Integer(u32),
    Pointer(Pointer),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer {
    pub pointer_value: DataOffset,
}
