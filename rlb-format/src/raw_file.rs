use binrw::helpers::until_eof;
use std::io::Cursor;

use binrw::{BinRead, BinWrite};

use crate::header::{Header};
use rlb_error::{Error, Result};

#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
pub struct TableRecord {
    pub address: u32,
    pub label_offset: u32,
}

#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(big)]
#[bw(big)]
pub struct RawFile {
    header: Header,
    #[br(count = header.data_size())]
    data: Vec<u8>,
    #[br(count = header.num_relocs())]
    relocation_table: Vec<u32>,
    #[br(count = header.num_entries())]
    records: Vec<TableRecord>,
    #[br(count = header.num_other_entries())]
    other_records: Vec<TableRecord>,
    #[br(parse_with = until_eof)]
    table_labels: Vec<u8>,
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

        let records = layout.records.into_iter().collect();
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

    pub fn new(data: Vec<u8>, relocation_table: Vec<u32>, records: Vec<TableRecord>, other_records: Vec<TableRecord>, table_labels: Vec<u8>) -> Result<Self> {
        let header = Header::from_data(&data,&relocation_table,&records,&other_records,&table_labels)?;
        Ok(Self{
            header,
            data,
            relocation_table,
            records,
            other_records,
            table_labels,
        })
    }

    pub fn serialize_custom(&self) -> Result<Vec<u8>> {
        let expected_file_size = self.header.file_size();
        let mut out = Vec::with_capacity(expected_file_size as usize);
        {
            let mut cursor = Cursor::new(&mut out);

            self.write(&mut cursor)
                .map_err(|_| Error::SerializationMismatch {
                    expected: expected_file_size,
                    actual: 0,
                })?;
        }
        Ok(out)
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn relocation_table(&self) -> &Vec<u32> {
        &self.relocation_table
    }

    pub fn records(&self) -> &Vec<TableRecord> {
        &self.records
    }

    pub fn other_records(&self) -> &Vec<TableRecord> {
        &self.other_records
    }

    pub fn table_labels(&self) -> &Vec<u8> {
        &self.table_labels
    }
}
