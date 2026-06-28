use encoding_rs::SHIFT_JIS;
use rlb_error::{Error, Result};
use slotmap::{Key, SlotMap};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct StringPool<K: Key> {
    map: SlotMap<K, String>,
    lookup: HashMap<String, K>,
}
#[derive(Debug)]
pub struct SerializedStringPoolContext<K: Key> {
    data: Vec<u8>,
    id_to_offset: HashMap<K, usize>,
}
impl<K: Key> SerializedStringPoolContext<K> {
    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn offset_of(&self, id: K) -> Option<usize> {
        self.id_to_offset.get(&id).copied()
    }
}

impl<K: Key> StringPool<K> {
    pub fn new() -> StringPool<K> {
        Self {
            map: SlotMap::with_key(),
            lookup: HashMap::new(),
        }
    }
    pub fn intern(&mut self, string: String) -> K {
        let existing_id = self.lookup.get(&string);
        if let Some(id) = existing_id {
            *id
        } else {
            let id = self.map.insert(string.clone());
            self.lookup.insert(string, id);
            id
        }
    }
    pub fn serialize(&self) -> Result<SerializedStringPoolContext<K>> {
        let mut string_data: Vec<u8> = Vec::new();
        let mut offsets: HashMap<K, usize> = HashMap::new();

        for (id, s) in &self.map {
            let offset = string_data.len();

            let (encoded, _, had_errors) = SHIFT_JIS.encode(s);

            if had_errors {
                return Err(Error::InvalidUtf8 {
                    context: "string/label pool (Shift-JIS encode failed)",
                    offset,
                    source: None,
                });
            }

            string_data.extend_from_slice(&encoded);
            //TODO: original files have a unknown alignment, check out if there is a system behind
            string_data.push(0);

            offsets.insert(id, offset);
        }
        Ok(SerializedStringPoolContext {
            data: string_data,
            id_to_offset: offsets,
        })
    }
}
