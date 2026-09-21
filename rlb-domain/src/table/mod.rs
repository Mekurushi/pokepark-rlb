use crate::entry_schemas::fsb_file_list::FsbFileListTable;
use crate::entry_schemas::script_list::ScriptListTable;
use crate::entry_schemas::wandering_data::WanderingDataTable;
use crate::string_pool::StringPool;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Debug, Clone)]
enum TableKind {
    ScriptList(ScriptListTable),
    FsbFileList(FsbFileListTable),
    WanderingData(WanderingDataTable),
}

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
        let kind = match name {
            "BackFromAttractionScriptList"
            | "ReplaceScriptList"
            | "CheckObjectScriptList"
            | "EnterZoneScriptList"
            | "HitDashScriptList"
            | "HitThunderboltScriptList"
            | "TimeOutScriptList"
            | "TouchAreaScriptList" => TableKind::ScriptList(ScriptListTable::parse(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?),
            "FsbFileListData" => TableKind::FsbFileList(FsbFileListTable::parse(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?),
            "WanderingDataTable" => TableKind::WanderingData(WanderingDataTable::parse(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?),
            _ => {
                return Err(Error::UnknownTableSchema {
                    name: name.to_owned(),
                });
            }
        };

        Ok(Self { kind })
    }

    pub(crate) fn serialize_into(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &StringPool,
        relocations: &mut Vec<u32>,
    ) -> Result<()> {
        match &self.kind {
            TableKind::ScriptList(table) => table.serialize(out, base_offset, strings, relocations),
            TableKind::FsbFileList(table) => {
                table.serialize(out, base_offset, strings, relocations)
            }
            TableKind::WanderingData(table) => {
                table.serialize(out, base_offset, strings, relocations)
            }
        }
    }

    pub(crate) fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        match &self.kind {
            TableKind::ScriptList(table) => table.visit_strings(visit),
            TableKind::FsbFileList(table) => table.visit_strings(visit),
            TableKind::WanderingData(table) => table.visit_strings(visit),
        }
    }

    pub(crate) fn entry_count(&self) -> usize {
        match &self.kind {
            TableKind::ScriptList(table) => table.entry_count(),
            TableKind::FsbFileList(table) => table.entry_count(),
            TableKind::WanderingData(table) => table.entry_count(),
        }
    }

    pub(crate) fn field_descriptors(&self) -> &'static [FieldDescriptor] {
        match &self.kind {
            TableKind::ScriptList(table) => table.fields(),
            TableKind::FsbFileList(table) => table.fields(),
            TableKind::WanderingData(table) => table.fields(),
        }
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        match &self.kind {
            TableKind::ScriptList(table) => table.get_field(index, field),
            TableKind::FsbFileList(table) => table.get_field(index, field),
            TableKind::WanderingData(table) => table.get_field(index, field),
        }
    }

    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        match &mut self.kind {
            TableKind::ScriptList(table) => table.set_field(index, field, value),
            TableKind::FsbFileList(table) => table.set_field(index, field, value),
            TableKind::WanderingData(table) => table.set_field(index, field, value),
        }
    }
}
