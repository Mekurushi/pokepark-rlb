use rlb_error::Result;
use rlb_format::{Header, RawFile, RelocationTable};

use crate::label_pool::LabelPool;
use crate::table_of_contents::TableOfContents;

#[derive(Debug, Clone)]
pub struct RLBFile {
    #[allow(dead_code)]
    header: Header,
    // TODO: data segment, stringpool, tables
    #[allow(dead_code)]
    relocation_table: RelocationTable,
    pub toc: TableOfContents,
    #[allow(dead_code)]
    labels: LabelPool,
}

impl RLBFile {
    pub fn from_raw(raw: RawFile) -> Result<Self> {
        let RawFile {
            header,
            data: _data,
            relocation_table,
            entries,
            table_labels,
        } = raw;

        let labels = LabelPool::new(table_labels);
        let toc = TableOfContents::new(entries, &labels)?;

        Ok(Self {
            header,
            relocation_table,
            toc,
            labels,
        })
    }

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Self::from_raw(RawFile::parse(bytes)?)
    }
}
