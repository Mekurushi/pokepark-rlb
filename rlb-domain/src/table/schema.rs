use crate::FieldDescriptor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SchemaId {
    AttractionRanking,
    DispositionDataHeader,
    PlayerDispositionData,
    PokemonDispositionData,
    ItemDispositionData,
    ItemKindTotalNumData,
    FlagTable,
    ScriptList,
    FsbFileList,
    WanderingData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowBoundary {
    Fixed {
        rows: usize,
    },
    Terminated,
    CountedBy {
        schema: SchemaId,
        field: &'static str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowLayout {
    pub boundary: RowBoundary,
    pub max_rows: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaDescriptor {
    pub id: SchemaId,
    pub description: &'static str,
    pub fields: &'static [FieldDescriptor],
    pub rows: RowLayout,
}
