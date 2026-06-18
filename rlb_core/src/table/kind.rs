use crate::schema::ScriptListTableEntry;
use crate::table::table_view::TableView;

pub enum TableKind {
    ScriptList(TableView<ScriptListTableEntry>),
}
