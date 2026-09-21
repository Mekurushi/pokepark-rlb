use crate::entry_schemas::fsb_file_list::FsbFileListTable;
use crate::entry_schemas::script_list::ScriptListTable;
use crate::entry_schemas::wandering_data::WanderingDataTable;
use crate::string_pool::StringPool;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Debug, Clone)]
pub(crate) enum TableKind {
    ScriptList(ScriptListTable),
    FsbFileList(FsbFileListTable),
    WanderingData(WanderingDataTable),
}

impl TableKind {
    pub(crate) fn parse<R, E>(
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
        match name {
            "BackFromAttractionScriptList"
            | "ReplaceScriptList"
            | "CheckObjectScriptList"
            | "EnterZoneScriptList"
            | "HitDashScriptList"
            | "HitThunderboltScriptList"
            | "TimeOutScriptList"
            | "TouchAreaScriptList" => Ok(Self::ScriptList(ScriptListTable::parse(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)),
            "FsbFileListData" => Ok(Self::FsbFileList(FsbFileListTable::parse(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)),
            "WanderingDataTable" => Ok(Self::WanderingData(WanderingDataTable::parse(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)),
            _ => Err(Error::UnknownTableSchema {
                name: name.to_owned(),
            }),
        }
    }

    pub(crate) fn serialize(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &StringPool,
        relocations: &mut Vec<u32>,
    ) -> Result<()> {
        match self {
            Self::ScriptList(table) => table.serialize(out, base_offset, strings, relocations),
            Self::FsbFileList(table) => table.serialize(out, base_offset, strings, relocations),
            Self::WanderingData(table) => table.serialize(out, base_offset, strings, relocations),
        }
    }

    pub(crate) fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        match self {
            Self::ScriptList(table) => table.visit_strings(visit),
            Self::FsbFileList(table) => table.visit_strings(visit),
            Self::WanderingData(table) => table.visit_strings(visit),
        }
    }

    pub(crate) fn entry_count(&self) -> usize {
        match self {
            Self::ScriptList(table) => table.entry_count(),
            Self::FsbFileList(table) => table.entry_count(),
            Self::WanderingData(table) => table.entry_count(),
        }
    }

    pub(crate) fn field_descriptors(&self) -> &'static [FieldDescriptor] {
        match self {
            Self::ScriptList(table) => table.fields(),
            Self::FsbFileList(table) => table.fields(),
            Self::WanderingData(table) => table.fields(),
        }
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        match self {
            Self::ScriptList(table) => table.get_field(index, field),
            Self::FsbFileList(table) => table.get_field(index, field),
            Self::WanderingData(table) => table.get_field(index, field),
        }
    }

    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        match self {
            Self::ScriptList(table) => table.set_field(index, field, value),
            Self::FsbFileList(table) => table.set_field(index, field, value),
            Self::WanderingData(table) => table.set_field(index, field, value),
        }
    }
}
