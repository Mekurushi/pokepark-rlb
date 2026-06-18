use binrw::{BinRead, BinWrite};

use crate::error::{Error, Result};
use crate::schema::TableEntry;

#[derive(Debug, Clone, PartialEq, BinRead, BinWrite)]
#[brw(big)]
pub struct ScriptListTableEntry {
    pub name_ptr: u32,
    pub object_id: i32,
    pub minimum_chapter: i32,
    pub medium_chapter: i32,
    pub maximum_chapter: i32,
    pub flagname_ptr: u32,
    pub flag_value_condition: i32,
    pub target_script: u8,
    pub pad_0x1d: [u8; 3],
    pub unknown: i32,
    pub entrypoint_ptr: u32,
    pub zone_id: i32,
    pub area_id: i32,
    pub position_id: i32,
    pub pad_0x34: i32,
    pub after_script_entrypoint_ptr: u32,
    pub animation_ptr: u32,
    pub flagname2_ptr: u32,
}

impl TableEntry for ScriptListTableEntry {
    const SIZE: usize = 0x44;

    const KNOWN_TABLES: &'static [&'static str] = &[
        "BackFromAttractionScriptList",
        "CheckObjectScriptList",
        "EnterZoneScriptList",
        "HitDashScriptList",
        "HitThunderboltScriptList",
        "ReplaceScriptList",
        "TimeOutScriptList",
        "TouchAreaScriptList",
    ];

    fn read(bytes: &[u8]) -> Result<Self> {
        let mut cursor = std::io::Cursor::new(bytes);
        Ok(<ScriptListTableEntry as BinRead>::read(&mut cursor)?)
    }

    fn write_into(&self, out: &mut [u8]) -> Result<()> {
        let mut cursor = std::io::Cursor::new(out);
        self.write(&mut cursor)?;
        Ok(())
    }

    fn is_terminator(&self) -> bool {
        self.name_ptr == 0
    }

    fn set_field(&mut self, field: &str, value: i32) -> Result<()> {
        match field {
            "object_id" => self.object_id = value,
            "minimum_chapter" => self.minimum_chapter = value,
            "medium_chapter" => self.medium_chapter = value,
            "maximum_chapter" => self.maximum_chapter = value,
            "flag_value_condition" => self.flag_value_condition = value,
            "zone_id" => self.zone_id = value,
            "area_id" => self.area_id = value,
            "position_id" => self.position_id = value,
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
