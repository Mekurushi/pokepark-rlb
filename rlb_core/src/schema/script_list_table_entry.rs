use binrw::{BinRead, BinWrite};

use crate::error::{Error, Result};

pub const ENTRY_SIZE: usize = 0x44;

pub const TABLE_NAMES: &[&str] = &[
    "BackFromAttractionScriptList",
    "CheckObjectScriptList",
    "EnterZoneScriptList",
    "HitDashScriptList",
    "HitThunderboltScriptList",
    "ReplaceScriptList",
    "TimeOutScriptList",
    "TouchAreaScriptList",
];

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

impl ScriptListTableEntry {
    pub fn is_terminator(&self) -> bool {
        self.name_ptr == 0
    }

    pub fn read_be(bytes: &[u8]) -> Result<Self> {
        let mut cursor = std::io::Cursor::new(bytes);
        Ok(Self::read(&mut cursor)?)
    }

    pub fn write_be_into(&self, out: &mut [u8]) -> Result<()> {
        let mut cursor = std::io::Cursor::new(out);
        self.write(&mut cursor)?;
        Ok(())
    }

    pub fn set_i32_field(&mut self, field: &str, value: i32) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ScriptListTableEntry {
        ScriptListTableEntry {
            name_ptr: 0x100,
            object_id: 1,
            minimum_chapter: 2,
            medium_chapter: 3,
            maximum_chapter: 4,
            flagname_ptr: 0x200,
            flag_value_condition: 5,
            target_script: 6,
            pad_0x1d: [0; 3],
            unknown: 7,
            entrypoint_ptr: 0x300,
            zone_id: 8,
            area_id: 9,
            position_id: 10,
            pad_0x34: 0,
            after_script_entrypoint_ptr: 0x400,
            animation_ptr: 0x500,
            flagname2_ptr: 0x600,
        }
    }

    #[test]
    fn round_trips_through_bytes() {
        let entry = sample();
        let mut bytes = [0u8; ENTRY_SIZE];
        entry.write_be_into(&mut bytes).unwrap();
        let parsed = ScriptListTableEntry::read_be(&bytes).unwrap();
        assert_eq!(entry, parsed);
    }

    #[test]
    fn terminator_is_recognized_by_null_name_ptr() {
        let mut entry = sample();
        assert!(!entry.is_terminator());
        entry.name_ptr = 0;
        assert!(entry.is_terminator());
    }

    #[test]
    fn rejects_pointer_field_writes() {
        let mut entry = sample();
        let err = entry.set_i32_field("name_ptr", 42).unwrap_err();
        assert!(matches!(err, Error::TypeMismatch { .. }));
    }
}
