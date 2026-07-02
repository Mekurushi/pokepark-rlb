mod codec;
pub mod fsb_file_list;
pub mod script_list;
pub mod wandering_data;

pub(crate) use crate::entry_schemas::codec::{EntryDeserializer, EntrySerializer};
use crate::rlb_file::StringId;
use crate::Value;
use rlb_error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    Integer,
    String,
    Boolean,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldConstraint {
    None,
    IntegerRange { min: u32, max: u32 },
    TableIndex { table: &'static str },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    pub name: &'static str,
    pub description: &'static str,
    pub kind: FieldKind,
    pub constraint: FieldConstraint,
}

pub trait TableEntry: Sized + std::fmt::Debug {
    const SIZE: usize;

    const FIELDS: &'static [FieldDescriptor];
    fn is_terminator(&self) -> bool;

    fn get(&self, field: &str) -> Option<Value>;

    fn set(&mut self, field: &str, value: Value) -> Result<()>;

    fn read<R, E>(de: &mut EntryDeserializer<'_, R, E>) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool;

    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()>;
}
