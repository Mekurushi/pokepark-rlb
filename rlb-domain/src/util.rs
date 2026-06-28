use encoding_rs::SHIFT_JIS;
use rlb_error::{Error, Result};

use crate::rlb_file::StringId;
use crate::Value;

pub(crate) fn checked_u32(value: usize, context: &'static str) -> Result<u32> {
    u32::try_from(value).map_err(|_e| Error::ValueTooLarge { context, value })
}

pub(crate) fn resolve_string_from_raw_data(data: &[u8], index: usize) -> Result<String> {
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

pub(crate) fn value_at<R, E>(
    data: &[u8],
    field_offset: usize,
    base_offset: usize,
    resolve_string: &mut R,
    is_relocated: &mut E,
) -> Result<Value>
where
    R: FnMut(u32) -> Result<StringId>,
    E: FnMut(u32) -> bool,
{
    let raw = read_u32(data, field_offset)?;
    let abs_offset = checked_u32(base_offset + field_offset, "calculating field offset")?;
    if is_relocated(abs_offset) {
        Ok(Value::Pointer(resolve_string(raw)?))
    } else {
        Ok(Value::Integer(raw))
    }
}

pub(crate) fn read_u32(data: &[u8], offset: usize) -> Result<u32> {
    data.get(offset..offset + 4)
        .and_then(|b| b.try_into().ok())
        .map(u32::from_be_bytes)
        .ok_or(Error::OffsetOutOfBounds {
            context: "entry field",
            offset,
            length: data.len(),
        })
}

pub(crate) fn read_u8(data: &[u8], offset: usize) -> Result<u8> {
    data.get(offset).copied().ok_or(Error::OffsetOutOfBounds {
        context: "entry field",
        offset,
        length: data.len(),
    })
}

pub(crate) fn read_bytes<const N: usize>(data: &[u8], offset: usize) -> Result<[u8; N]> {
    data.get(offset..offset + N)
        .and_then(|b| b.try_into().ok())
        .ok_or(Error::OffsetOutOfBounds {
            context: "entry field",
            offset,
            length: data.len(),
        })
}
