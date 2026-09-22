use crate::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Row {
    //TODO: typed or constant for field names; at least stable reference
    fields: HashMap<String, Value>,
}

impl Row {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with(mut self, field: impl Into<String>, value: Value) -> Self {
        self.insert(field, value);
        self
    }

    pub fn insert(&mut self, field: impl Into<String>, value: Value) -> Option<Value> {
        self.fields.insert(field.into(), value)
    }

    #[must_use]
    pub fn get(&self, field: &str) -> Option<&Value> {
        self.fields.get(field)
    }
}
