use binrw::{BinRead, BinWrite};

use crate::error::{Error, Result};
use crate::schema::TableEntry;

#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(big)]
pub struct PointerTableEntry {
    pub ptr: u32,
}

impl TableEntry for PointerTableEntry {
    const SIZE: usize = 0x4;

    const KNOWN_TABLES: &'static [&'static str] = &["FsbFileListData"];

    fn read(bytes: &[u8]) -> Result<Self> {
        let mut cursor = std::io::Cursor::new(bytes);
        Ok(<PointerTableEntry as BinRead>::read(&mut cursor)?)
    }

    fn write_into(&self, out: &mut [u8]) -> Result<()> {
        let mut cursor = std::io::Cursor::new(out);
        self.write(&mut cursor)?;
        Ok(())
    }

    fn is_terminator(&self) -> bool {
        self.ptr == 0x0
    }

    fn set_field(&mut self, field: &str, value: i32) -> Result<()> {
        match field {
            "ptr" => self.ptr = value as u32,
            other => {
                return Err(Error::TypeMismatch {
                    table: "ScriptListTableEntry".to_string(),
                    index: 0,
                    field: other.to_string(),
                });
            }
        }
        Ok(())
    }
}
