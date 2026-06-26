use binrw::helpers::until_eof;
use std::io::Cursor;

use binrw::{BinRead, BinWrite};

use crate::header::{ENTRY_SLOT_SIZE, HEADER_SIZE, Header, RELOCATION_ENTRY_SIZE};
use crate::relocation::RelocationTable;
use rlb_error::{Error, Result};

// --- Domain ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableRecord {
    Named { address: u32, name_offset: u32 },
    Unknown { address: u32, raw_offset: u32 },
}

impl TableRecord {
    pub fn address(&self) -> u32 {
        match self {
            TableRecord::Named { address, .. } | TableRecord::Unknown { address, .. } => *address,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawFile {
    pub header: Header,
    pub data: Vec<u8>,
    pub relocation_table: RelocationTable,
    pub records: Vec<TableRecord>,
    pub table_labels: Vec<u8>,
}

// --- Layout ---
#[derive(Debug, Clone, BinRead, BinWrite)]
#[brw(big)]
struct NamedSlotRaw {
    address: u32,
    name_offset: u32,
}

#[derive(Debug, Clone, BinRead, BinWrite)]
#[brw(big)]
struct UnknownSlotRaw {
    address: u32,
    raw_offset: u32,
}

#[derive(BinRead)]
#[br(big)]
struct RawFileLayout {
    header: Header,
    #[br(count = header.data_size)]
    data: Vec<u8>,
    #[br(count = header.num_relocs)]
    relocations: Vec<u32>,
    #[br(count = header.num_entries)]
    named: Vec<NamedSlotRaw>,
    #[br(count = header.num_other_entries)]
    unknown: Vec<UnknownSlotRaw>,
    #[br(parse_with = until_eof)]
    table_labels: Vec<u8>,
}
#[derive(BinWrite)]
#[bw(big)]
struct RawFileLayoutRef<'a> {
    header: Header,
    data: &'a [u8],
    relocations: &'a [u32],
    named: Vec<NamedSlotRaw>,
    unknown: Vec<UnknownSlotRaw>,
    table_labels: &'a [u8],
}

impl RawFile {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);

        let declared_size = u32::read_be(&mut cursor).map_err(|_| Error::UnexpectedEof {
            context: "file_size",
        })?;
        if declared_size as usize != bytes.len() {
            return Err(Error::InvalidFileSize {
                expected: declared_size,
                actual: bytes.len() as u64,
            });
        }
        cursor.set_position(0);

        let layout = RawFileLayout::read(&mut cursor).map_err(|_| Error::UnexpectedEof {
            context: "RLB file layout",
        })?;

        let entries = layout
            .named
            .into_iter()
            .map(|slot| TableRecord::Named {
                address: slot.address,
                name_offset: slot.name_offset,
            })
            .chain(layout.unknown.into_iter().map(|slot| TableRecord::Unknown {
                address: slot.address,
                raw_offset: slot.raw_offset,
            }))
            .collect();

        Ok(RawFile {
            header: layout.header,
            data: layout.data,
            relocation_table: RelocationTable::from_raw(layout.relocations),
            records: entries,
            table_labels: layout.table_labels,
        })
    }

    pub fn write(&self) -> Result<Vec<u8>> {
        let (named, unknown): (Vec<&TableRecord>, Vec<&TableRecord>) = self
            .records
            .iter()
            .partition(|e| matches!(e, TableRecord::Named { .. }));

        let reloc_offset = HEADER_SIZE as usize + self.data.len();
        let entries_offset =
            reloc_offset + self.relocation_table.len() * RELOCATION_ENTRY_SIZE as usize;
        let table_labels_offset = entries_offset + self.records.len() * ENTRY_SLOT_SIZE as usize;
        let file_size = table_labels_offset + self.table_labels.len();

        let header = Header {
            file_size: checked_u32(file_size, "file_size")?,
            data_size: checked_u32(self.data.len(), "data_size")?,
            num_relocs: checked_u32(self.relocation_table.len(), "num_relocs")?,
            num_entries: checked_u32(named.len(), "num_entries")?,
            num_other_entries: checked_u32(unknown.len(), "num_other_entries")?,
            reserved: self.header.reserved,
        };
        let expected_file_size = header.file_size;

        let named = named
            .into_iter()
            .map(|slot| match *slot {
                TableRecord::Named {
                    address,
                    name_offset,
                } => NamedSlotRaw {
                    address,
                    name_offset,
                },
                TableRecord::Unknown { .. } => {
                    unreachable!("partitioned as Named above")
                }
            })
            .collect();
        let unknown = unknown
            .into_iter()
            .map(|slot| match *slot {
                TableRecord::Unknown {
                    address,
                    raw_offset,
                } => UnknownSlotRaw {
                    address,
                    raw_offset,
                },
                TableRecord::Named { .. } => {
                    unreachable!("partitioned as Unknown above")
                }
            })
            .collect();

        let layout = RawFileLayoutRef {
            header,
            data: &self.data,
            relocations: self.relocation_table.as_slice(),
            named,
            unknown,
            table_labels: &self.table_labels,
        };

        let mut out = Vec::with_capacity(file_size);
        {
            let mut cursor = Cursor::new(&mut out);
            layout
                .write(&mut cursor)
                .map_err(|_| Error::SerializationMismatch {
                    expected: expected_file_size,
                    actual: 0,
                })?;
        }

        if out.len() != file_size {
            return Err(Error::SerializationMismatch {
                expected: expected_file_size,
                actual: out.len(),
            });
        }
        Ok(out)
    }
}
fn checked_u32(value: usize, context: &'static str) -> Result<u32> {
    u32::try_from(value).map_err(|_e| Error::ValueTooLarge { context, value })
}
