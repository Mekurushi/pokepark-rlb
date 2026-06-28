use crate::rlb_file::TableId;
use crate::table::Table;
use slotmap::SlotMap;

#[derive(Clone, Debug)]
pub struct TableCollection {
    map: SlotMap<TableId, Table>,
}
impl TableCollection {
    pub fn new() -> TableCollection {
        Self {
            map: SlotMap::with_key(),
        }
    }

    pub fn insert(&mut self, table: Table) -> TableId {
        self.map.insert(table)
    }
}
