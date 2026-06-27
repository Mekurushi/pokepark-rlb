use binrw::helpers::until_eof;
use std::io::Cursor;

use binrw::{BinRead, BinWrite};

use crate::header::{ENTRY_SLOT_SIZE, HEADER_SIZE, Header, RELOCATION_ENTRY_SIZE};
use rlb_error::{Error, Result};

#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
pub struct TableRecord {
    pub address: u32,
    pub label_offset: u32,
}

#[derive(Debug, Clone, BinRead,BinWrite)]
#[br(big)]
#[bw(big)]
pub struct RawFile {
    pub header: Header,
    #[br(count = header.data_size)]
    pub data: Vec<u8>,
    #[br(count = header.num_relocs)]
    pub relocation_table: Vec<u32>,
    #[br(count = header.num_entries)]
    pub records: Vec<TableRecord>,
    #[br(count = header.num_other_entries)]
    pub other_records: Vec<TableRecord>,
    #[br(parse_with = until_eof)]
    pub table_labels: Vec<u8>,
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

        let layout = RawFile::read(&mut cursor).map_err(|_| Error::UnexpectedEof {
            context: "RLB file layout",
        })?;

        let records = layout
            .records
            .into_iter().collect();
        let other_records = layout.other_records.into_iter().collect();


        Ok(RawFile {
            header: layout.header,
            data: layout.data,
            relocation_table: layout.relocation_table,
            records,
            other_records,
            table_labels: layout.table_labels,
        })
    }

    pub fn serialize_custom(&self) -> Result<Vec<u8>> {


        let reloc_offset = HEADER_SIZE as usize + self.data.len();
        let entries_offset =
            reloc_offset + self.relocation_table.len() * RELOCATION_ENTRY_SIZE as usize;
        let table_labels_offset = entries_offset + (self.records.len() + self.other_records.len()) * ENTRY_SLOT_SIZE as usize;
        let file_size = table_labels_offset + self.table_labels.len();

        let header = Header {
            file_size: checked_u32(file_size, "file_size")?,
            data_size: checked_u32(self.data.len(), "data_size")?,
            num_relocs: checked_u32(self.relocation_table.len(), "num_relocs")?,
            num_entries: checked_u32(self.records.len(), "num_entries")?,
            num_other_entries: checked_u32(self.other_records.len(), "num_other_entries")?,
            reserved: self.header.reserved,
        };
        let expected_file_size = header.file_size;
        let mut out = Vec::with_capacity(file_size);
        {
            let mut cursor = Cursor::new(&mut out);

            self.write(&mut cursor)
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
