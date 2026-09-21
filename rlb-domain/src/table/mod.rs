mod codec;
mod collection;
mod field;
mod parse_context;
mod schemas;
mod serialization;

use crate::Value;
use crate::table::schemas::attraction_ranking::AttractionRankingTable;
use crate::table::schemas::disposition_data_header::DispositionDataHeaderTable;
use crate::table::schemas::flag_table::FlagTable;
use crate::table::schemas::fsb_file_list::FsbFileListTable;
use crate::table::schemas::item_disposition_data::ItemDispositionDataTable;
use crate::table::schemas::item_kind_total_num_data::ItemKindTotalNumDataTable;
use crate::table::schemas::player_disposition_data::PlayerDispositionDataTable;
use crate::table::schemas::pokemon_disposition_data::PokemonDispositionDataTable;
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
    AttractionRanking(AttractionRankingTable),
    DispositionDataHeader(DispositionDataHeaderTable),
    PlayerDispositionData(PlayerDispositionDataTable),
    PokemonDispositionData(PokemonDispositionDataTable),
    ItemDispositionData(ItemDispositionDataTable),
    ItemKindTotalNumData(ItemKindTotalNumDataTable),
    Flag(FlagTable),
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
            "AttractionRanking" => {
                TableKind::AttractionRanking(AttractionRankingTable::parse(context, offset)?)
            }
            "dispositionDataHeader" => TableKind::DispositionDataHeader(
                DispositionDataHeaderTable::parse(context, offset)?,
            ),
            "playerDispositionData" => TableKind::PlayerDispositionData(
                PlayerDispositionDataTable::parse(context, offset)?,
            ),
            "pokemonDispositionData" => TableKind::PokemonDispositionData(
                PokemonDispositionDataTable::parse(context, offset)?,
            ),
            "itemDispositionData" => {
                TableKind::ItemDispositionData(ItemDispositionDataTable::parse(context, offset)?)
            }
            "itemKindTotalNumData" => {
                TableKind::ItemKindTotalNumData(ItemKindTotalNumDataTable::parse(context, offset)?)
            }
            "FlagTable" => TableKind::Flag(FlagTable::parse(context, offset)?),
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
            TableKind::AttractionRanking(table) => table.serialize(),
            TableKind::DispositionDataHeader(table) => table.serialize(),
            TableKind::PlayerDispositionData(table) => table.serialize(),
            TableKind::PokemonDispositionData(table) => table.serialize(),
            TableKind::ItemDispositionData(table) => table.serialize(),
            TableKind::ItemKindTotalNumData(table) => table.serialize(),
            TableKind::Flag(table) => table.serialize(),
            TableKind::ScriptList(table) => table.serialize(),
            TableKind::FsbFileList(table) => table.serialize(),
            TableKind::WanderingData(table) => table.serialize(),
        }
    }

    pub(crate) fn entry_count(&self) -> usize {
        match &self.kind {
            TableKind::AttractionRanking(table) => table.entry_count(),
            TableKind::DispositionDataHeader(table) => table.entry_count(),
            TableKind::PlayerDispositionData(table) => table.entry_count(),
            TableKind::PokemonDispositionData(table) => table.entry_count(),
            TableKind::ItemDispositionData(table) => table.entry_count(),
            TableKind::ItemKindTotalNumData(table) => table.entry_count(),
            TableKind::Flag(table) => table.entry_count(),
            TableKind::ScriptList(table) => table.entry_count(),
            TableKind::FsbFileList(table) => table.entry_count(),
            TableKind::WanderingData(table) => table.entry_count(),
        }
    }

    pub(crate) fn field_descriptors(&self) -> &'static [FieldDescriptor] {
        match &self.kind {
            TableKind::AttractionRanking(table) => table.fields(),
            TableKind::DispositionDataHeader(table) => table.fields(),
            TableKind::PlayerDispositionData(table) => table.fields(),
            TableKind::PokemonDispositionData(table) => table.fields(),
            TableKind::ItemDispositionData(table) => table.fields(),
            TableKind::ItemKindTotalNumData(table) => table.fields(),
            TableKind::Flag(table) => table.fields(),
            TableKind::ScriptList(table) => table.fields(),
            TableKind::FsbFileList(table) => table.fields(),
            TableKind::WanderingData(table) => table.fields(),
        }
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        match &self.kind {
            TableKind::AttractionRanking(table) => table.get_field(index, field),
            TableKind::DispositionDataHeader(table) => table.get_field(index, field),
            TableKind::PlayerDispositionData(table) => table.get_field(index, field),
            TableKind::PokemonDispositionData(table) => table.get_field(index, field),
            TableKind::ItemDispositionData(table) => table.get_field(index, field),
            TableKind::ItemKindTotalNumData(table) => table.get_field(index, field),
            TableKind::Flag(table) => table.get_field(index, field),
            TableKind::ScriptList(table) => table.get_field(index, field),
            TableKind::FsbFileList(table) => table.get_field(index, field),
            TableKind::WanderingData(table) => table.get_field(index, field),
        }
    }

    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        match &mut self.kind {
            TableKind::AttractionRanking(table) => table.set_field(index, field, value),
            TableKind::DispositionDataHeader(table) => table.set_field(index, field, value),
            TableKind::PlayerDispositionData(table) => table.set_field(index, field, value),
            TableKind::PokemonDispositionData(table) => table.set_field(index, field, value),
            TableKind::ItemDispositionData(table) => table.set_field(index, field, value),
            TableKind::ItemKindTotalNumData(table) => table.set_field(index, field, value),
            TableKind::Flag(table) => table.set_field(index, field, value),
            TableKind::ScriptList(table) => table.set_field(index, field, value),
            TableKind::FsbFileList(table) => table.set_field(index, field, value),
            TableKind::WanderingData(table) => table.set_field(index, field, value),
        }
    }
}
