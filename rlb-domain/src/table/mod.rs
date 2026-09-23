mod codec;
mod collection;
mod field;
mod parse_context;
mod schema;
mod schemas;
mod serialization;

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
use crate::{Row, Value};
use rlb_error::{Error, Result};

pub(crate) use collection::TableCollection;
pub use field::{FieldConstraint, FieldDescriptor, FieldKind, FloatKind, IntegerKind};
pub(crate) use parse_context::ParseContext;
pub use schema::{RowBoundary, RowLayout, SchemaDescriptor, SchemaId};

slotmap::new_key_type! {
    pub struct TableId;
}

//TODO: check schema-driven design if it would allow to collapse and hold Vec of Rows here in a
// generic Table
#[derive(Debug, Clone)]
pub(crate) enum Table {
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

impl Table {
    pub(crate) fn create(name: &str, rows: &[Row]) -> Result<Self> {
        let table = match name {
            "AttractionRanking" => Self::AttractionRanking(AttractionRankingTable::create(rows)?),
            "dispositionDataHeader" => {
                Self::DispositionDataHeader(DispositionDataHeaderTable::create(rows)?)
            }
            "playerDispositionData" => {
                Self::PlayerDispositionData(PlayerDispositionDataTable::create(rows)?)
            }
            "pokemonDispositionData" => {
                Self::PokemonDispositionData(PokemonDispositionDataTable::create(rows)?)
            }
            "itemDispositionData" => {
                Self::ItemDispositionData(ItemDispositionDataTable::create(rows)?)
            }
            "itemKindTotalNumData" => {
                Self::ItemKindTotalNumData(ItemKindTotalNumDataTable::create(rows)?)
            }
            "FlagTable" => Self::Flag(FlagTable::create(rows)?),
            "BackFromAttractionScriptList"
            | "ReplaceScriptList"
            | "CheckObjectScriptList"
            | "EnterZoneScriptList"
            | "HitDashScriptList"
            | "HitThunderboltScriptList"
            | "TimeOutScriptList"
            | "TouchAreaScriptList" => Self::ScriptList(ScriptListTable::create(rows)?),
            "FsbFileListData" => Self::FsbFileList(FsbFileListTable::create(rows)?),
            "WanderingDataTable" => Self::WanderingData(WanderingDataTable::create(rows)?),
            _ => return Err(Error::UnknownTableSchema { name: name.into() }),
        };
        Ok(table)
    }

    pub(crate) fn parse(name: &str, context: &ParseContext<'_>, offset: usize) -> Result<Self> {
        let table = match name {
            "AttractionRanking" => {
                Self::AttractionRanking(AttractionRankingTable::parse(context, offset)?)
            }
            "dispositionDataHeader" => {
                Self::DispositionDataHeader(DispositionDataHeaderTable::parse(context, offset)?)
            }
            "playerDispositionData" => {
                Self::PlayerDispositionData(PlayerDispositionDataTable::parse(context, offset)?)
            }
            "pokemonDispositionData" => {
                Self::PokemonDispositionData(PokemonDispositionDataTable::parse(context, offset)?)
            }
            "itemDispositionData" => {
                Self::ItemDispositionData(ItemDispositionDataTable::parse(context, offset)?)
            }
            "itemKindTotalNumData" => {
                Self::ItemKindTotalNumData(ItemKindTotalNumDataTable::parse(context, offset)?)
            }
            "FlagTable" => Self::Flag(FlagTable::parse(context, offset)?),
            "BackFromAttractionScriptList"
            | "ReplaceScriptList"
            | "CheckObjectScriptList"
            | "EnterZoneScriptList"
            | "HitDashScriptList"
            | "HitThunderboltScriptList"
            | "TimeOutScriptList"
            | "TouchAreaScriptList" => Self::ScriptList(ScriptListTable::parse(context, offset)?),
            "FsbFileListData" => Self::FsbFileList(FsbFileListTable::parse(context, offset)?),
            "WanderingDataTable" => {
                Self::WanderingData(WanderingDataTable::parse(context, offset)?)
            }
            _ => {
                return Err(Error::UnknownTableSchema {
                    name: name.to_owned(),
                });
            }
        };

        Ok(table)
    }

    pub(crate) fn serialize(&self) -> Result<RelocatableTable<'_>> {
        match self {
            Self::AttractionRanking(table) => table.serialize(),
            Self::DispositionDataHeader(table) => table.serialize(),
            Self::PlayerDispositionData(table) => table.serialize(),
            Self::PokemonDispositionData(table) => table.serialize(),
            Self::ItemDispositionData(table) => table.serialize(),
            Self::ItemKindTotalNumData(table) => table.serialize(),
            Self::Flag(table) => table.serialize(),
            Self::ScriptList(table) => table.serialize(),
            Self::FsbFileList(table) => table.serialize(),
            Self::WanderingData(table) => table.serialize(),
        }
    }

    pub(crate) fn entry_count(&self) -> usize {
        match self {
            Self::AttractionRanking(table) => table.entry_count(),
            Self::DispositionDataHeader(table) => table.entry_count(),
            Self::PlayerDispositionData(table) => table.entry_count(),
            Self::PokemonDispositionData(table) => table.entry_count(),
            Self::ItemDispositionData(table) => table.entry_count(),
            Self::ItemKindTotalNumData(table) => table.entry_count(),
            Self::Flag(table) => table.entry_count(),
            Self::ScriptList(table) => table.entry_count(),
            Self::FsbFileList(table) => table.entry_count(),
            Self::WanderingData(table) => table.entry_count(),
        }
    }

    pub(crate) fn field_descriptors(&self) -> &'static [FieldDescriptor] {
        match self {
            Self::AttractionRanking(table) => table.fields(),
            Self::DispositionDataHeader(table) => table.fields(),
            Self::PlayerDispositionData(table) => table.fields(),
            Self::PokemonDispositionData(table) => table.fields(),
            Self::ItemDispositionData(table) => table.fields(),
            Self::ItemKindTotalNumData(table) => table.fields(),
            Self::Flag(table) => table.fields(),
            Self::ScriptList(table) => table.fields(),
            Self::FsbFileList(table) => table.fields(),
            Self::WanderingData(table) => table.fields(),
        }
    }

    pub(crate) fn schema(&self) -> &'static SchemaDescriptor {
        match self {
            Self::AttractionRanking(_) => &AttractionRankingTable::SCHEMA,
            Self::DispositionDataHeader(_) => &DispositionDataHeaderTable::SCHEMA,
            Self::PlayerDispositionData(_) => &PlayerDispositionDataTable::SCHEMA,
            Self::PokemonDispositionData(_) => &PokemonDispositionDataTable::SCHEMA,
            Self::ItemDispositionData(_) => &ItemDispositionDataTable::SCHEMA,
            Self::ItemKindTotalNumData(_) => &ItemKindTotalNumDataTable::SCHEMA,
            Self::Flag(_) => &FlagTable::SCHEMA,
            Self::ScriptList(_) => &ScriptListTable::SCHEMA,
            Self::FsbFileList(_) => &FsbFileListTable::SCHEMA,
            Self::WanderingData(_) => &WanderingDataTable::SCHEMA,
        }
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        match self {
            Self::AttractionRanking(table) => table.get_field(index, field),
            Self::DispositionDataHeader(table) => table.get_field(index, field),
            Self::PlayerDispositionData(table) => table.get_field(index, field),
            Self::PokemonDispositionData(table) => table.get_field(index, field),
            Self::ItemDispositionData(table) => table.get_field(index, field),
            Self::ItemKindTotalNumData(table) => table.get_field(index, field),
            Self::Flag(table) => table.get_field(index, field),
            Self::ScriptList(table) => table.get_field(index, field),
            Self::FsbFileList(table) => table.get_field(index, field),
            Self::WanderingData(table) => table.get_field(index, field),
        }
    }

    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        match self {
            Self::AttractionRanking(table) => table.set_field(index, field, value),
            Self::DispositionDataHeader(table) => table.set_field(index, field, value),
            Self::PlayerDispositionData(table) => table.set_field(index, field, value),
            Self::PokemonDispositionData(table) => table.set_field(index, field, value),
            Self::ItemDispositionData(table) => table.set_field(index, field, value),
            Self::ItemKindTotalNumData(table) => table.set_field(index, field, value),
            Self::Flag(table) => table.set_field(index, field, value),
            Self::ScriptList(table) => table.set_field(index, field, value),
            Self::FsbFileList(table) => table.set_field(index, field, value),
            Self::WanderingData(table) => table.set_field(index, field, value),
        }
    }

    pub(crate) fn append_row(&mut self, row: &Row) -> Result<usize> {
        match self {
            Self::AttractionRanking(table) => table.append_row(row),
            Self::PlayerDispositionData(table) => table.append_row(row),
            Self::PokemonDispositionData(table) => table.append_row(row),
            Self::ItemDispositionData(table) => table.append_row(row),
            Self::ItemKindTotalNumData(table) => table.append_row(row),
            Self::Flag(table) => table.append_row(row),
            Self::ScriptList(table) => table.append_row(row),
            Self::FsbFileList(table) => table.append_row(row),
            Self::WanderingData(table) => table.append_row(row),
            _ => Err(Error::Validation(format!(
                "appending rows to {:?} is not supported",
                self.schema().id
            ))),
        }
    }

    pub(crate) fn remove_row(&mut self, index: usize) -> Result<()> {
        match self {
            Self::AttractionRanking(table) => table.remove_row(index),
            Self::PlayerDispositionData(table) => table.remove_row(index),
            Self::PokemonDispositionData(table) => table.remove_row(index),
            Self::ItemDispositionData(table) => table.remove_row(index),
            Self::ItemKindTotalNumData(table) => table.remove_row(index),
            Self::Flag(table) => table.remove_row(index),
            Self::ScriptList(table) => table.remove_row(index),
            Self::FsbFileList(table) => table.remove_row(index),
            Self::WanderingData(table) => table.remove_row(index),
            _ => Err(Error::Validation(format!(
                "removing rows from {:?} is not supported",
                self.schema().id
            ))),
        }
    }
}
