use crate::error::Result;
use crate::format::RawFile;
use crate::schema::{ScriptListTableEntry, TableEntry};
use crate::table::kind::TableKind;
use crate::table::model::Table;
use crate::table::table_view::TableView;

pub struct Schema {
    pub matches: fn(&str) -> bool,
    pub discover: fn(&RawFile, String, u32) -> Result<Table>,
}

pub fn script_list_matches(name: &str) -> bool {
    ScriptListTableEntry::KNOWN_TABLES.contains(&name)
}

pub fn script_list_discover(raw: &RawFile, name: String, addr: u32) -> Result<Table> {
    Ok(Table::new(
        name.clone(),
        addr,
        TableKind::ScriptList(TableView::<ScriptListTableEntry>::discover(
            raw, name, addr,
        )?),
    ))
}
pub static SCHEMAS: &[Schema] = &[Schema {
    matches: script_list_matches,
    discover: script_list_discover,
}];
