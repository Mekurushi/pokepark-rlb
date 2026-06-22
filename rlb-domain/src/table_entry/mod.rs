use crate::value::Value;
mod back_from_attraction_script_list_entry;
mod fsb_file_list_data;
mod layouts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    pub name: &'static str,
}

pub trait TableEntry: std::fmt::Debug {
    fn type_name(&self) -> &'static str;

    fn fields(&self) -> &[FieldDescriptor];

    fn get(&self, field: &str) -> Option<Value>;

    fn set(&mut self, field: &str, value: Value) -> rlb_error::Result<()>;
}
