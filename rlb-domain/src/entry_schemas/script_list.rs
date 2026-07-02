use crate::entry_schemas::codec::{EntryDeserializer, EntrySerializer};
use crate::rlb_file::StringId;
use crate::util::require_int;
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
        self.name == Value::String(None)
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

    fn read<R, E>(de: &mut EntryDeserializer<'_, R, E>) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool,
    {
        Ok(Self {
            name: de.read_string_pointer()?,
            object_id: de.read_u32()?,
            minimum_chapter: de.read_u32()?,
            medium_chapter: de.read_u32()?,
            maximum_chapter: de.read_u32()?,
            flagname: de.read_string_pointer()?,
            flag_value_condition: de.read_u32()?,
            target_script: de.read_u8()?,
            pad_0x1d: de.read_pad()?,
            unknown: de.read_u32()?,
            entrypoint: de.read_string_pointer()?,
            zone_id: de.read_u32()?,
            area_id: de.read_u32()?,
            position_id: de.read_u32()?,
            pad_0x34: de.read_u32()?,
            after_script_entrypoint: de.read_string_pointer()?,
            animation: de.read_string_pointer()?,
            flagname2: de.read_string_pointer()?,
        })
    }
    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        ser.write_string_pointer(self.name)?;
        ser.write_u32(self.object_id);
        ser.write_u32(self.minimum_chapter);
        ser.write_u32(self.medium_chapter);
        ser.write_u32(self.maximum_chapter);
        ser.write_string_pointer(self.flagname)?;
        ser.write_u32(self.flag_value_condition);
        ser.write_u8(self.target_script);
        ser.write_pad(&self.pad_0x1d);
        ser.write_u32(self.unknown);
        ser.write_string_pointer(self.entrypoint)?;
        ser.write_u32(self.zone_id);
        ser.write_u32(self.area_id);
        ser.write_u32(self.position_id);
        ser.write_u32(self.pad_0x34);
        ser.write_string_pointer(self.after_script_entrypoint)?;
        ser.write_string_pointer(self.animation)?;
        ser.write_string_pointer(self.flagname2)?;

        Ok(())
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
