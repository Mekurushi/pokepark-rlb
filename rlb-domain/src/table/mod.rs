mod codec;
mod collection;
mod field;
mod parse_context;
mod schemas;
mod serialization;

use crate::Value;
use crate::table::schemas::fsb_file_list::FsbFileListTable;
use crate::table::schemas::script_list::ScriptListTable;
use crate::table::schemas::wandering_data::WanderingDataTable;
use crate::table::serialization::RelocatableTable;
use rlb_error::{Error, Result};

pub(crate) use collection::TableCollection;
pub use field::{FieldConstraint, FieldDescriptor, FieldKind};
pub(crate) use parse_context::ParseContext;

slotmap::new_key_type! {
    pub struct TableId;
}

#[derive(Debug, Clone)]
enum TableKind {
    ScriptList(ScriptListTable),
    FsbFileList(FsbFileListTable),
    WanderingData(WanderingDataTable),
}

#[derive(Debug, Clone)]
pub(crate) struct Table {
    kind: TableKind,
}

impl Table {
    pub(crate) fn parse(name: &str, context: &ParseContext<'_>, offset: usize) -> Result<Self> {
        let kind = match name {
            "BackFromAttractionScriptList"
            | "ReplaceScriptList"
            | "CheckObjectScriptList"
            | "EnterZoneScriptList"
            | "HitDashScriptList"
            | "HitThunderboltScriptList"
            | "TimeOutScriptList"
            | "TouchAreaScriptList" => {
                TableKind::ScriptList(ScriptListTable::parse(context, offset)?)
            }
            "FsbFileListData" => TableKind::FsbFileList(FsbFileListTable::parse(context, offset)?),
            "WanderingDataTable" => {
                TableKind::WanderingData(WanderingDataTable::parse(context, offset)?)
            }
            _ => {
                return Err(Error::UnknownTableSchema {
                    name: name.to_owned(),
                });
            }
        };

        Ok(Self { kind })
    }

    pub(crate) fn serialize(&self) -> Result<RelocatableTable<'_>> {
        match &self.kind {
            TableKind::ScriptList(table) => table.serialize(),
            TableKind::FsbFileList(table) => table.serialize(),
            TableKind::WanderingData(table) => table.serialize(),
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
