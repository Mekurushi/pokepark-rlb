use crate::schema::{PointerTableEntry, ScriptListTableEntry};
use crate::table::table_view::TableView;

pub enum TableKind {
    ScriptList(TableView<ScriptListTableEntry>),
    PointerTable(TableView<PointerTableEntry>),
}
