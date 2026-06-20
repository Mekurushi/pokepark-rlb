use std::io::Cursor;

use binrw::{BinRead, BinWrite};

use super::header::{HEADER_SIZE, Header};
use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntrySlot {
    Named { address: u32, name_offset: u32 },
    Unknown { address: u32, raw_offset: u32 },
}

impl EntrySlot {
    pub fn address(&self) -> u32 {
        match *self {
            EntrySlot::Named { address, .. } => address,
            EntrySlot::Unknown { address, .. } => address,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RawFile {
    pub header: Header,
    pub data: Vec<u8>, // contains also the strings used by pointer
    pub relocations: Vec<u32>,
    pub entries: Vec<EntrySlot>,
    pub table_labels: Vec<u8>,
}

impl RawFile {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let declared_size = read_u32(bytes, 0)?;
        if declared_size as usize != bytes.len() {
            return Err(Error::InvalidFileSize {
                expected: declared_size,
                actual: bytes.len() as u64,
            });
        }

        let mut cursor = Cursor::new(bytes);
        let header = Header::read(&mut cursor)?;

        let data = read_slice(
            bytes,
            header.data_offset(),
            header.data_size,
            "DATA segment",
        )?
        .to_vec();

        let mut relocations = Vec::with_capacity(header.num_relocs as usize);
        let mut offset = header.reloc_offset();
        for _ in 0..header.num_relocs {
            relocations.push(read_u32(bytes, offset)?);
            offset += 4;
        }

        let table_labels_len = bytes.len() as u32 - header.table_labels_offset();
        let table_labels =
            read_slice(bytes, header.table_labels_offset(), table_labels_len, "table labels pool")?
                .to_vec();

        let total_entries = header.num_entries + header.num_other_entries;
        let mut entries = Vec::with_capacity(total_entries as usize);
        let mut offset = header.entries_offset();
        for _ in 0..header.num_entries {
            let address = read_u32(bytes, offset)?;
            let name_offset = read_u32(bytes, offset + 4)?;
            entries.push(EntrySlot::Named {
                address,
                name_offset,
            });
            offset += 8;
        }
        for _ in 0..header.num_other_entries {
            let address = read_u32(bytes, offset)?;
            let raw_offset = read_u32(bytes, offset + 4)?;
            entries.push(EntrySlot::Unknown {
                address,
                raw_offset,
            });
            offset += 8;
        }

        Ok(RawFile {
            header,
            data,
            relocations,
            entries,
            table_labels,
        })
    }

    pub fn resolve_name(&self, name_offset: u32) -> Result<String> {
        let tail = self
            .table_labels
            .get(name_offset as usize..)
            .ok_or(Error::UnexpectedEof {
                context: "table labels tail",
            })?;
        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
        String::from_utf8(tail[..end].to_vec()).map_err(|_| Error::UnexpectedEof {
            context: "non-ASCII table labels entry",
        })
    }

    pub fn write(&self) -> Result<Vec<u8>> {
        let (named, unknown): (Vec<&EntrySlot>, Vec<&EntrySlot>) = self
            .entries
            .iter()
            .partition(|e| matches!(e, EntrySlot::Named { .. }));

        let reloc_offset = HEADER_SIZE + self.data.len() as u32;
        let entries_offset = reloc_offset + self.relocations.len() as u32 * 4;
        let table_labels_offset = entries_offset + self.entries.len() as u32 * 8;
        let file_size = table_labels_offset + self.table_labels.len() as u32;

        let header = Header {
            file_size,
            data_size: self.data.len() as u32,
            num_relocs: self.relocations.len() as u32,
            num_entries: named.len() as u32,
            num_other_entries: unknown.len() as u32,
            reserved: self.header.reserved,
        };

        let mut out = Vec::with_capacity(file_size as usize);
        {
            let mut cursor = Cursor::new(&mut out);
            header.write(&mut cursor)?;
        }
        out.extend_from_slice(&self.data);
        for reloc in &self.relocations {
            out.extend_from_slice(&reloc.to_be_bytes());
        }
        for entry in named.into_iter().chain(unknown) {
            let (address, second) = match *entry {
                EntrySlot::Named {
                    address,
                    name_offset,
                } => (address, name_offset),
                EntrySlot::Unknown {
                    address,
                    raw_offset,
                } => (address, raw_offset),
            };
            out.extend_from_slice(&address.to_be_bytes());
            out.extend_from_slice(&second.to_be_bytes());
        }
        out.extend_from_slice(&self.table_labels);

        debug_assert_eq!(
            out.len(),
            file_size as usize,
            "computed file_size must match the bytes actually written"
        );
        Ok(out)
    }
}

fn read_u32(bytes: &[u8], offset: u32) -> Result<u32> {
    let offset = offset as usize;
    let chunk = bytes.get(offset..offset + 4).ok_or(Error::UnexpectedEof {
        context: "u32 field",
    })?;
    Ok(u32::from_be_bytes(
        chunk.try_into().expect("slice is exactly 4 bytes"),
    ))
}

fn read_slice<'a>(
    bytes: &'a [u8],
    offset: u32,
    len: u32,
    context: &'static str,
) -> Result<&'a [u8]> {
    bytes
        .get(offset as usize..(offset + len) as usize)
        .ok_or(Error::UnexpectedEof { context })
}
