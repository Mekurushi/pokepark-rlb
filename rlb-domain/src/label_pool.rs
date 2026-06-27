use slotmap::SlotMap;
use std::collections::HashMap;
use crate::rlb_file::LabelId;

#[derive(Clone, Debug)]
pub struct LabelPool {
    map: SlotMap<LabelId, String>,
    lookup: HashMap<String, LabelId>,
}
impl LabelPool {
    pub fn new() -> LabelPool {
        Self {
            map: SlotMap::with_key(),
            lookup: HashMap::new(),
        }
    }
    pub fn intern(&mut self, string: String) -> LabelId {
        let existing_id = self.lookup.get(&string);
        if let Some(id) = existing_id {
            *id
        } else {
            let id = self.map.insert(string.clone());
            self.lookup.insert(string, id);
            id
        }
    }
}
