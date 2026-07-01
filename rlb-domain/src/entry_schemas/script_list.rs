use crate::rlb_file::StringId;
use crate::string_pool::SerializedStringPoolContext;
use crate::util::{read_bytes, read_u32, read_u8, require_int, value_at, write_value};
use crate::TableEntry;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct ScriptListEntry {
    pub name: Value,
    pub object_id: u32,
    pub minimum_chapter: u32,
    pub medium_chapter: u32,
    pub maximum_chapter: u32,
    pub flagname: Value,
    pub flag_value_condition: u32,
    pub target_script: u8,
    pub pad_0x1d: [u8; 3],
    pub unknown: u32,
    pub entrypoint: Value,
    pub zone_id: u32,
    pub area_id: u32,
    pub position_id: u32,
    pub pad_0x34: u32,
    pub after_script_entrypoint: Value,
    pub animation: Value,
    pub flagname2: Value,
}

impl TableEntry for ScriptListEntry {
    const SIZE: usize = 0x44;
    fn fields(&self) -> &[FieldDescriptor] {
        SCRIPT_LIST_FIELDS
    }

    fn is_terminator(&self) -> bool {
        self.name == Value::Integer(0)
            && self.object_id == 0
            && self.minimum_chapter == 0
            && self.medium_chapter == 0
            && self.maximum_chapter == 0
    }

    fn get(&self, field: &str) -> Option<Value> {
        match field {
            "name" => Some(self.name),
            "object_id" => Some(Value::Integer(self.object_id)),
            "minimum_chapter" => Some(Value::Integer(self.minimum_chapter)),
            "medium_chapter" => Some(Value::Integer(self.medium_chapter)),
            "maximum_chapter" => Some(Value::Integer(self.maximum_chapter)),
            "flagname" => Some(self.flagname),
            "flag_value_condition" => Some(Value::Integer(self.flag_value_condition)),
            "target_script" => Some(Value::Integer(u32::from(self.target_script))),
            "unknown" => Some(Value::Integer(self.unknown)),
            "entrypoint" => Some(self.entrypoint),
            "zone_id" => Some(Value::Integer(self.zone_id)),
            "area_id" => Some(Value::Integer(self.area_id)),
            "position_id" => Some(Value::Integer(self.position_id)),
            "pad_0x34" => Some(Value::Integer(self.pad_0x34)),
            "after_script_entrypoint" => Some(self.after_script_entrypoint),
            "animation" => Some(self.animation),
            "flagname2" => Some(self.flagname2),
            _ => None,
        }
    }
    fn set(&mut self, field: &str, value: Value) -> rlb_error::Result<()> {
        match field {
            "name" => self.name = value,
            "object_id" => self.object_id = require_int(field, value)?,
            "minimum_chapter" => self.minimum_chapter = require_int(field, value)?,
            "medium_chapter" => self.medium_chapter = require_int(field, value)?,
            "maximum_chapter" => self.maximum_chapter = require_int(field, value)?,
            "flagname" => self.flagname = value,
            "flag_value_condition" => self.flag_value_condition = require_int(field, value)?,
            //TODO: validate range
            "target_script" => self.target_script = require_int(field, value)? as u8,
            "unknown" => self.unknown = require_int(field, value)?,
            "entrypoint" => self.entrypoint = value,
            "zone_id" => self.zone_id = require_int(field, value)?,
            "area_id" => self.area_id = require_int(field, value)?,
            "position_id" => self.position_id = require_int(field, value)?,
            "pad_0x34" => self.pad_0x34 = require_int(field, value)?,
            "after_script_entrypoint" => self.after_script_entrypoint = value,
            "animation" => self.animation = value,
            "flagname2" => self.flagname2 = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }

    fn read<R, E>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> rlb_error::Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool,
    {
        Ok(Self {
            name: value_at(data, 0x00, base_offset, resolve_string, is_relocated)?,
            object_id: read_u32(data, 0x04)?,
            minimum_chapter: read_u32(data, 0x08)?,
            medium_chapter: read_u32(data, 0x0C)?,
            maximum_chapter: read_u32(data, 0x10)?,
            flagname: value_at(data, 0x14, base_offset, resolve_string, is_relocated)?,
            flag_value_condition: read_u32(data, 0x18)?,
            target_script: read_u8(data, 0x1C)?,
            pad_0x1d: read_bytes(data, 0x1D)?,
            unknown: read_u32(data, 0x20)?,
            entrypoint: value_at(data, 0x24, base_offset, resolve_string, is_relocated)?,
            zone_id: read_u32(data, 0x28)?,
            area_id: read_u32(data, 0x2C)?,
            position_id: read_u32(data, 0x30)?,
            pad_0x34: read_u32(data, 0x34)?,
            after_script_entrypoint: value_at(
                data,
                0x38,
                base_offset,
                resolve_string,
                is_relocated,
            )?,
            animation: value_at(data, 0x3C, base_offset, resolve_string, is_relocated)?,
            flagname2: value_at(data, 0x40, base_offset, resolve_string, is_relocated)?,
        })
    }
    fn write(
        &self,
        base_offset: usize,
        strings: &SerializedStringPoolContext<StringId>,
        relocations: &mut Vec<u32>,
    ) -> Result<Vec<u8>> {
        let mut entry: Vec<u8> = Vec::with_capacity(Self::SIZE);

        write_value(
            self.name,
            0x00,
            base_offset,
            &mut entry,
            strings,
            relocations,
        )?;
        entry.extend_from_slice(&self.object_id.to_be_bytes());
        entry.extend_from_slice(&self.minimum_chapter.to_be_bytes());
        entry.extend_from_slice(&self.medium_chapter.to_be_bytes());
        entry.extend_from_slice(&self.maximum_chapter.to_be_bytes());
        write_value(
            self.flagname,
            0x14,
            base_offset,
            &mut entry,
            strings,
            relocations,
        )?;
        entry.extend_from_slice(&self.flag_value_condition.to_be_bytes());
        entry.push(self.target_script);
        entry.extend_from_slice(&self.pad_0x1d);
        entry.extend_from_slice(&self.unknown.to_be_bytes());
        write_value(
            self.entrypoint,
            0x24,
            base_offset,
            &mut entry,
            strings,
            relocations,
        )?;
        entry.extend_from_slice(&self.zone_id.to_be_bytes());
        entry.extend_from_slice(&self.area_id.to_be_bytes());
        entry.extend_from_slice(&self.position_id.to_be_bytes());
        entry.extend_from_slice(&self.pad_0x34.to_be_bytes());
        write_value(
            self.after_script_entrypoint,
            0x38,
            base_offset,
            &mut entry,
            strings,
            relocations,
        )?;
        write_value(
            self.animation,
            0x3C,
            base_offset,
            &mut entry,
            strings,
            relocations,
        )?;
        write_value(
            self.flagname2,
            0x40,
            base_offset,
            &mut entry,
            strings,
            relocations,
        )?;
        if entry.len() != Self::SIZE {
            return Err(Error::SerializationMismatch {
                expected: Self::SIZE as u32,
                actual: entry.len(),
            });
        }
        Ok(entry)
    }
}
pub const SCRIPT_LIST_FIELDS: &[FieldDescriptor] = &[
    FieldDescriptor { name: "name" },
    FieldDescriptor { name: "object_id" },
    FieldDescriptor {
        name: "minimum_chapter",
    },
    FieldDescriptor {
        name: "medium_chapter",
    },
    FieldDescriptor {
        name: "maximum_chapter",
    },
    FieldDescriptor { name: "flagname" },
    FieldDescriptor {
        name: "flag_value_condition",
    },
    FieldDescriptor {
        name: "target_script",
    },
    FieldDescriptor { name: "unknown" },
    FieldDescriptor { name: "entrypoint" },
    FieldDescriptor { name: "zone_id" },
    FieldDescriptor { name: "area_id" },
    FieldDescriptor {
        name: "position_id",
    },
    FieldDescriptor { name: "pad_0x34" },
    FieldDescriptor {
        name: "after_script_entrypoint",
    },
    FieldDescriptor { name: "animation" },
    FieldDescriptor { name: "flagname2" },
];
