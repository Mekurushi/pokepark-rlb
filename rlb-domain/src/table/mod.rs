pub mod registry;
use crate::rlb_file::StringId;
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
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool,
    {
        Ok(Self {
            kind: TableKind::discover(name, data, offset, resolve_string, is_relocated)?,
        })
    }
}
