use crate::rlb_file::StringId;
use slotmap::SlotMap;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct StringPool {
    map: SlotMap<StringId, String>,
    lookup: HashMap<String, StringId>,
}
impl StringPool {
    pub fn new() -> StringPool {
        Self {
            map: SlotMap::with_key(),
            lookup: HashMap::new(),
        }
    }
    pub fn intern(&mut self, string: String) -> StringId {
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
