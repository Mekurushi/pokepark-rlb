use crate::Value;
use crate::table::serialization::{RelocatableTable, StringFixup};
use crate::util::checked_u32;
use rlb_error::{Error, Result};

#[derive(Debug)]
pub(crate) struct EntrySerializer<'a> {
    buffer: Vec<u8>,
    string_fixups: Vec<StringFixup<'a>>,
}

impl<'a> EntrySerializer<'a> {
    pub(crate) fn new() -> Self {
        Self {
            buffer: Vec::new(),
            string_fixups: Vec::new(),
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

    pub(crate) fn write_string_pointer(&mut self, value: &'a Value) -> Result<()> {
        match value {
            Value::String(string) => match string {
                None => {
                    self.buffer.extend_from_slice(&0u32.to_be_bytes());
                    Ok(())
                }
                Some(value) => {
                    self.string_fixups.push(StringFixup {
                        offset: self.buffer.len(),
                        value,
                    });
                    self.buffer.extend_from_slice(&0u32.to_be_bytes());
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

    pub(crate) fn finish(self, table: &mut RelocatableTable<'a>, expected: usize) -> Result<()> {
        if self.buffer.len() != expected {
            return Err(Error::Validation(format!(
                "entry serialized {} bytes, expected {expected}",
                self.buffer.len()
            )));
        }
        let entry_offset = table.data.len();
        table.data.extend_from_slice(&self.buffer);
        table
            .string_fixups
            .extend(self.string_fixups.into_iter().map(|fixup| StringFixup {
                offset: entry_offset + fixup.offset,
                value: fixup.value,
            }));
        Ok(())
    }
}

pub(crate) struct EntryDeserializer<'a, R, E>
where
    R: FnMut(u32) -> Result<String>,
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
    R: FnMut(u32) -> Result<String>,
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
