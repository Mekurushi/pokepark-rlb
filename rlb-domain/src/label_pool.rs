use crate::util::checked_u32;
use rlb_error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LabelOffset(pub u32);

#[derive(Debug, Clone)]
pub struct LabelPool {
    bytes: Vec<u8>,
}

impl LabelPool {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn resolve(&self, offset: LabelOffset) -> Result<&str> {
        let tail = self
            .bytes
            .get(offset.0 as usize..)
            .ok_or(Error::OffsetOutOfBounds {
                context: "label pool",
                offset: offset.0,
                length: checked_u32(self.bytes.len(), "label_pool")?,
            })?;
        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
        std::str::from_utf8(&tail[..end]).map_err(|e| Error::InvalidUtf8 {
            context: "label pool",
            offset: offset.0,
            source: e,
        })
    }
}
