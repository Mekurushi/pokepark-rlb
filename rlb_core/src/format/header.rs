use binrw::{BinRead, BinWrite};

pub const HEADER_SIZE: u32 = 0x20;

#[derive(Debug, Clone, PartialEq, Eq, BinRead, BinWrite)]
#[brw(big)]
pub struct Header {
    pub file_size: u32,
    pub data_size: u32,
    pub num_relocs: u32,
    pub num_entries: u32,
    pub num_other_entries: u32,
    pub reserved: [u8; 12],
}

impl Header {
    pub fn data_offset(&self) -> u32 {
        HEADER_SIZE
    }

    pub fn reloc_offset(&self) -> u32 {
        self.data_offset() + self.data_size
    }

    pub fn entries_offset(&self) -> u32 {
        self.reloc_offset() + self.num_relocs * 4
    }

    pub fn table_labels_offset(&self) -> u32 {
        self.entries_offset() + (self.num_entries + self.num_other_entries) * 8
    }
}
