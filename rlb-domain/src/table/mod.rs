pub mod body;
pub mod registry;
pub mod terminated_list;

use crate::string_pool::StringPool;
use crate::table::registry::TableKind;
use rlb_error::Result;

#[derive(Debug, Clone)]
pub struct Table {
    pub kind: TableKind,
}

impl Table {
    pub fn resolve<R, E>(
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
            kind: TableKind::discover(name, data, offset, resolve_string, is_relocated)?,
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
}
