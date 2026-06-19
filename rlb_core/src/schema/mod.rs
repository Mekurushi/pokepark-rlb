use crate::error::Result;

mod pointer_table_entry;
mod script_list_table_entry;

pub use pointer_table_entry::PointerTableEntry;

pub use script_list_table_entry::ScriptListTableEntry;

pub trait TableEntry: Sized {
    const SIZE: usize;
    const KNOWN_TABLES: &'static [&'static str];

    fn read(bytes: &[u8]) -> Result<Self>;
    fn write_into(&self, out: &mut [u8]) -> Result<()>;

    fn is_terminator(&self) -> bool;

    fn set_field(&mut self, field: &str, value: i32) -> Result<()>;
}
