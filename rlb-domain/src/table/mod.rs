mod registry;

use crate::string_pool::StringPool;
use crate::table::registry::TableKind;
use crate::{FieldDescriptor, Value};
use rlb_error::Result;

#[derive(Debug, Clone)]
pub struct Table {
    kind: TableKind,
}

impl Table {
    pub fn parse<R, E>(
        name: &str,
        data: &[u8],
        offset: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
        Ok(Self {
            kind: TableKind::parse(name, data, offset, resolve_string, is_relocated)?,
        })
    }

    pub(crate) fn serialize_into(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &StringPool,
        relocations: &mut Vec<u32>,
    ) -> Result<()> {
        self.kind.serialize(out, base_offset, strings, relocations)
    }

    pub(crate) fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        self.kind.visit_strings(visit)
    }

    pub(crate) fn entry_count(&self) -> usize {
        self.kind.entry_count()
    }

    pub(crate) fn field_descriptors(&self) -> &'static [FieldDescriptor] {
        self.kind.field_descriptors()
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        self.kind.get_field(index, field)
    }

    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        self.kind.set_field(index, field, value)
    }
}
