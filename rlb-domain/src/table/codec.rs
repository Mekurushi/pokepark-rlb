use crate::Value;
use crate::table::ParseContext;
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

pub(crate) struct EntryDeserializer<'context, 'data> {
    context: &'context ParseContext<'data>,
    cursor: usize,
    base_offset: usize,
}

impl<'context, 'data> EntryDeserializer<'context, 'data> {
    pub(crate) fn new(context: &'context ParseContext<'data>, base_offset: usize) -> Self {
        Self {
            context,
            cursor: 0,
            base_offset,
        }
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8> {
        let v = self.context.bytes_at(self.absolute_offset(), 1)?[0];
        self.cursor += 1;
        Ok(v)
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32> {
        let mut b = [0; 4];
        b.copy_from_slice(self.context.bytes_at(self.absolute_offset(), 4)?);
        self.cursor += 4;
        Ok(u32::from_be_bytes(b))
    }

    pub(crate) fn read_pad<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut b = [0; N];
        b.copy_from_slice(self.context.bytes_at(self.absolute_offset(), N)?);
        self.cursor += N;
        Ok(b)
    }

    pub(crate) fn read_string_pointer(&mut self) -> Result<Value> {
        let abs = checked_u32(self.absolute_offset(), "field abs offset")?;
        let raw = self.read_u32()?;

        if self.context.is_relocated(abs) {
            Ok(Value::String(Some(self.context.resolve_string(raw)?)))
        } else {
            Ok(Value::String(None))
        }
    }

    fn absolute_offset(&self) -> usize {
        self.base_offset + self.cursor
    }
}
