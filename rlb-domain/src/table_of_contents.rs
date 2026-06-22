use rlb_error::Result;
use std::collections::HashMap;

use rlb_format::EntrySlot;

use crate::label_pool::{LabelOffset, LabelPool};

#[derive(Debug, Clone)]
pub struct TableOfContents {
    entries: Vec<EntrySlot>,
    by_name: HashMap<String, usize>,
}

impl TableOfContents {
    pub fn new(entries: Vec<EntrySlot>, labels: &LabelPool) -> Result<Self> {
        let mut by_name = HashMap::with_capacity(entries.len());
        for (index, entry) in entries.iter().enumerate() {
            if let EntrySlot::Named { name_offset, .. } = entry {
                let name = labels.resolve(LabelOffset(*name_offset))?;
                by_name.insert(name.to_string(), index);
            }
        }
        Ok(Self { entries, by_name })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &EntrySlot> {
        self.entries.iter()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&EntrySlot> {
        self.by_name.get(name).map(|&i| &self.entries[i])
    }
}
