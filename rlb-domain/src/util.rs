use encoding_rs::SHIFT_JIS;
use rlb_error::Error;
pub(crate) fn checked_u32(value: usize, context: &'static str) -> rlb_error::Result<u32> {
    u32::try_from(value).map_err(|_e| Error::ValueTooLarge { context, value })
}

pub(crate) fn resolve_string_from_raw_data(data: &[u8], index: usize) -> rlb_error::Result<String> {
    let tail = data.get(index..).ok_or(Error::OffsetOutOfBounds {
        context: "string/label pool",
        offset: index,
        length: data.len(),
    })?;
    let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
    let (cow, _, had_errors) = SHIFT_JIS.decode(&tail[..end]);

    if had_errors {
        return Err(Error::InvalidUtf8 {
            context: "string/label pool (Shift-JIS decode failed)",
            offset: index,
            source: None,
        });
    }

    Ok(cow.into_owned())
}
