use encoding_rs::SHIFT_JIS;
use rlb_error::{Error, Result};

use crate::rlb_file::StringId;
use crate::string_pool::SerializedStringPoolContext;
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
        Ok(Value::String(Option::from(resolve_string(raw)?)))
    } else {
        Ok(Value::String(None)) // TODO: differentiate between None string and Integer based on field
    }
}
pub(crate) fn write_value(
    value: Value,
    field_offset: usize,
    base_offset: usize,
    out: &mut Vec<u8>,
    strings: &SerializedStringPoolContext<StringId>,
    relocations: &mut Vec<u32>,
) -> Result<()> {
    match value {
        Value::Integer(v) => {
            out.extend_from_slice(&v.to_be_bytes());
        }
        Value::String(string_id) => match string_id {
            None => {
                out.extend_from_slice(&0u32.to_be_bytes());
            }
            Some(id) => {
                let string_offset = strings.offset_of(id).ok_or_else(|| {
                    Error::Validation(format!(
                        "string ID {string_id:?} not found in serialized string pool"
                    ))
                })?;
                out.extend_from_slice(
                    &(checked_u32(string_offset, "converting string offset to u32")?).to_be_bytes(),
                );
                relocations.push(checked_u32(
                    base_offset + field_offset,
                    "calculating field offset for relocation table",
                )?);
            }
        },
    }
    Ok(())
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

pub fn require_int(field: &str, value: Value) -> rlb_error::Result<u32> {
    match value {
        Value::Integer(v) => Ok(v),
        Value::String(_) => Err(Error::Validation(format!(
            "field '{field}' expects an integer value, not a pointer"
        ))),
    }
}
