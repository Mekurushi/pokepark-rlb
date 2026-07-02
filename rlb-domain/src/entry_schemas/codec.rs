use crate::rlb_file::StringId;
use crate::string_pool::SerializedStringPoolContext;
use crate::util::checked_u32;
use crate::Value;
use rlb_error::{Error, Result};

#[derive(Debug)]
pub(crate) struct EntrySerializer<'a> {
    buffer: Vec<u8>,
    base_offset: usize,
    strings: &'a SerializedStringPoolContext<StringId>,
    relocations: &'a mut Vec<u32>,
}

impl<'a> EntrySerializer<'a> {
    pub(crate) fn new(
        base_offset: usize,
        strings: &'a SerializedStringPoolContext<StringId>,
        relocations: &'a mut Vec<u32>,
    ) -> Self {
        Self {
            buffer: Vec::new(),
            base_offset,
            strings,
            relocations,
        }
    }

    pub(crate) fn write_u8(&mut self, v: u8) {
        self.buffer.push(v);
    }

    pub(crate) fn write_u16(&mut self, v: u16) {
        self.buffer.extend_from_slice(&v.to_be_bytes());
    }

    pub(crate) fn write_u32(&mut self, v: u32) {
        self.buffer.extend_from_slice(&v.to_be_bytes());
    }

    pub(crate) fn write_pad(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub(crate) fn write_string_pointer(&mut self, value: Value) -> Result<()> {
        match value {
            Value::String(string_id) => match string_id {
                None => {
                    self.buffer.extend_from_slice(&0u32.to_be_bytes());
                    Ok(())
                }
                Some(id) => {
                    let string_offset = self.strings.offset_of(id).ok_or_else(|| {
                        Error::Validation(format!(
                            "string ID {string_id:?} not found in serialized string pool"
                        ))
                    })?;
                    self.relocations.push(checked_u32(
                        self.base_offset + self.buffer.len(),
                        "calculating field offset for relocation table",
                    )?);
                    self.buffer.extend_from_slice(
                        &(checked_u32(string_offset, "converting string offset to u32")?)
                            .to_be_bytes(),
                    );

                    Ok(())
                }
            },
            Value::Integer(_) => Err(Error::Validation(
                "string_pointer field received an Integer value".into(),
            )),
            Value::Boolean(_) => Err(Error::Validation(
                "string_pointer field received an Boolean value".into(),
            )),
        }
    }

    pub(crate) fn finish(self, out: &mut Vec<u8>, expected: usize) -> Result<()> {
        if self.buffer.len() != expected {
            return Err(Error::Validation(format!(
                "entry serialized {} bytes, expected {expected}",
                self.buffer.len()
            )));
        }
        out.extend_from_slice(&self.buffer);
        Ok(())
    }
}

pub(crate) struct EntryDeserializer<'a, R, E>
where
    R: FnMut(u32) -> Result<StringId>,
    E: FnMut(u32) -> bool,
{
    data: &'a [u8],
    cursor: usize,
    base_offset: usize,
    resolve_string: &'a mut R,
    is_relocated: &'a mut E,
}

impl<'a, R, E> EntryDeserializer<'a, R, E>
where
    R: FnMut(u32) -> Result<StringId>,
    E: FnMut(u32) -> bool,
{
    pub(crate) fn new(
        data: &'a [u8],
        base_offset: usize,
        resolve_string: &'a mut R,
        is_relocated: &'a mut E,
    ) -> Self {
        Self {
            data,
            cursor: 0,
            base_offset,
            resolve_string,
            is_relocated,
        }
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8> {
        let v = self
            .data
            .get(self.cursor)
            .copied()
            .ok_or(Error::UnexpectedEof {
                context: "entry field (u8)",
            })?;
        self.cursor += 1;
        Ok(v)
    }

    pub(crate) fn read_u16(&mut self) -> Result<u16> {
        let b = self
            .data
            .get(self.cursor..self.cursor + 2)
            .and_then(|s| s.try_into().ok())
            .ok_or(Error::UnexpectedEof {
                context: "entry field (u16)",
            })?;
        self.cursor += 2;
        Ok(u16::from_be_bytes(b))
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32> {
        let b = self
            .data
            .get(self.cursor..self.cursor + 4)
            .and_then(|s| s.try_into().ok())
            .ok_or(Error::UnexpectedEof {
                context: "entry field (u32)",
            })?;
        self.cursor += 4;
        Ok(u32::from_be_bytes(b))
    }

    pub(crate) fn read_pad<const N: usize>(&mut self) -> Result<[u8; N]> {
        let b = self
            .data
            .get(self.cursor..self.cursor + N)
            .and_then(|s| s.try_into().ok())
            .ok_or(Error::UnexpectedEof {
                context: "entry pad bytes",
            })?;
        self.cursor += N;
        Ok(b)
    }

    pub(crate) fn read_string_pointer(&mut self) -> Result<Value> {
        let abs = checked_u32(self.base_offset + self.cursor, "field abs offset")?;
        let raw = self.read_u32()?;

        if (self.is_relocated)(abs) {
            Ok(Value::String(Option::from((self.resolve_string)(raw)?)))
        } else {
            Ok(Value::String(None))
        }
    }
}
