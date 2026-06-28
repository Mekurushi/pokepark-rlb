use rlb_error::{Error, Result};

pub(crate) fn checked_u32(value: usize, context: &'static str) -> Result<u32> {
    u32::try_from(value).map_err(|_e| Error::ValueTooLarge { context, value })
}