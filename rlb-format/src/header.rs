use crate::TableRecord;
use crate::util::checked_u32;
use binrw::{BinRead, BinWrite};
use rlb_error::Result;

pub const HEADER_SIZE: u32 = 0x20;

pub const RELOCATION_ENTRY_SIZE: u32 = 4;
pub const ENTRY_SLOT_SIZE: u32 = 8;

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[brw(big)]
pub struct Header {
    file_size: u32,
    data_size: u32,
    num_relocs: u32,
    num_entries: u32,
    num_other_entries: u32,
    reserved: [u8; 12],
}

impl Header {
    pub fn data_offset(&self) -> u32 {
        HEADER_SIZE
    }

    pub fn reloc_offset(&self) -> u32 {
        self.data_offset() + self.data_size
    }

    pub fn entries_offset(&self) -> u32 {
        self.reloc_offset() + self.num_relocs * RELOCATION_ENTRY_SIZE
    }

    pub fn table_labels_offset(&self) -> u32 {
        self.entries_offset() + (self.num_entries + self.num_other_entries) * ENTRY_SLOT_SIZE
    }
    pub fn from_data(
        data: &[u8],
        relocation_table: &[u32],
        records: &[TableRecord],
        other_records: &[TableRecord],
        table_labels: &[u8],
    ) -> Result<Self> {
        let data_size = checked_u32(data.len(), "data size")?;
        let num_relocs = checked_u32(relocation_table.len(), "relocation count")?;
        let num_entries = checked_u32(records.len(), "record count")?;
        let num_other_entries = checked_u32(other_records.len(), "other record count")?;
        let labels_size = checked_u32(table_labels.len(), "table labels size")?;

        let file_size = HEADER_SIZE
            + data_size
            + num_relocs * RELOCATION_ENTRY_SIZE
            + (num_entries + num_other_entries) * ENTRY_SLOT_SIZE
            + labels_size;

        Ok(Self {
            file_size,
            data_size,
            num_relocs,
            num_entries,
            num_other_entries,
            reserved: [0; 12],
        })
    }

    pub fn file_size(&self) -> u32 {
        self.file_size
    }

    pub fn data_size(&self) -> u32 {
        self.data_size
    }

    pub fn num_relocs(&self) -> u32 {
        self.num_relocs
    }

    pub fn num_entries(&self) -> u32 {
        self.num_entries
    }

    pub fn num_other_entries(&self) -> u32 {
        self.num_other_entries
    }

    pub fn reserved(&self) -> [u8; 12] {
        self.reserved
    }
}
