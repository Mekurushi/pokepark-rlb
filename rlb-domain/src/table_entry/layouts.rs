use crate::rlb_file::StringId;
use crate::util::checked_u32;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

pub(super) fn read_u32(data: &[u8], offset: usize) -> Result<u32> {
    data.get(offset..offset + 4)
        .and_then(|b| b.try_into().ok())
        .map(u32::from_be_bytes)
        .ok_or(Error::OffsetOutOfBounds {
            context: "entry field",
            offset,
            length: data.len(),
        })
}

pub(super) fn read_u8(data: &[u8], offset: usize) -> Result<u8> {
    data.get(offset).copied().ok_or(Error::OffsetOutOfBounds {
        context: "entry field",
        offset,
        length: data.len(),
    })
}

pub(super) fn read_bytes<const N: usize>(data: &[u8], offset: usize) -> Result<[u8; N]> {
    data.get(offset..offset + N)
        .and_then(|b| b.try_into().ok())
        .ok_or(Error::OffsetOutOfBounds {
            context: "entry field",
            offset,
            length: data.len(),
        })
}

pub(super) fn value_at<R, E>(
    data: &[u8],
    field_offset: usize,
    base_offset: usize,
    resolve_string: &mut R,
    is_relocated: &mut E,
) -> Result<Value>
where
    R: FnMut(u32) -> Result<StringId>,
    E: FnMut(u32) -> bool,
{
    let raw = read_u32(data, field_offset)?;
    let abs_offset = checked_u32(base_offset + field_offset, "calculating field offset")?;
    if is_relocated(abs_offset) {
        Ok(Value::Pointer(resolve_string(raw)?))
    } else {
        Ok(Value::Integer(raw))
    }
}

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

impl ScriptListEntry {
    pub fn read<R, E>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
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

    pub fn is_terminator(&self) -> bool {
        self.name == Value::Integer(0)
            && self.object_id == 0
            && self.minimum_chapter == 0
            && self.medium_chapter == 0
            && self.maximum_chapter == 0
    }
    pub fn get(&self, field: &str) -> Option<Value> {
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

    pub fn set(&mut self, field: &str, value: Value) -> Result<()> {
        fn require_int(field: &str, value: Value) -> Result<u32> {
            match value {
                Value::Integer(v) => Ok(v),
                Value::Pointer(_) => Err(Error::Validation(format!(
                    "field '{field}' expects an integer value, not a pointer"
                ))),
            }
        }

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

    pub fn size() -> usize {
        0x44
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SinglePointerEntry {
    pub script_name: Value,
}

impl SinglePointerEntry {
    pub fn read<R, E>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool,
    {
        Ok(Self {
            script_name: value_at(data, 0x00, base_offset, resolve_string, is_relocated)?,
        })
    }
    pub fn is_terminator(&self) -> bool {
        self.script_name == Value::Integer(0)
    }
    pub fn get(&self, field: &str) -> Option<Value> {
        match field {
            "script_name" => Some(self.script_name),
            _ => None,
        }
    }

    pub fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "script_name" => {
                self.script_name = value;
            }
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }

    pub fn size() -> usize {
        0x4
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

pub const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
}];
