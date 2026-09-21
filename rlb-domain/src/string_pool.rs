use encoding_rs::SHIFT_JIS;
use rlb_error::{Error, Result};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) struct StringPool {
    data: Vec<u8>,
    offsets: HashMap<String, usize>,
}

impl StringPool {
    pub(crate) fn new() -> Self {
        Self {
            data: Vec::new(),
            offsets: HashMap::new(),
        }
    }

    pub(crate) fn intern(&mut self, value: &str) -> Result<usize> {
        if let Some(offset) = self.offset_of(value) {
            return Ok(offset);
        }

        let offset = self.data.len();
        let (encoded, _, had_errors) = SHIFT_JIS.encode(value);
        if had_errors {
            return Err(Error::InvalidUtf8 {
                context: "serializing string pool (Shift-JIS encode failed)",
                offset,
                source: None,
            });
        }

        self.data.extend_from_slice(&encoded);
        self.data.push(0);
        self.offsets.insert(value.to_owned(), offset);
        Ok(offset)
    }

    pub(crate) fn offset_of(&self, value: &str) -> Option<usize> {
        self.offsets.get(value).copied()
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }
}
