use crate::rlb_file::StringId;
use crate::table_entry::layouts::{
    SCRIPT_LIST_FIELDS, ScriptListEntry,
};
use crate::table_entry::{FieldDescriptor, TableEntry};
use crate::value::Value;
use rlb_error::{Error, Result};
use rlb_format::RelocationTable;

#[derive(Debug, Clone)]
pub struct EnterZoneScriptListEntry(pub ScriptListEntry);

impl TableEntry for EnterZoneScriptListEntry {
    fn type_name() -> &'static str {
        "EnterZoneScriptList"
    }

    fn fields(&self) -> &[FieldDescriptor] {
        SCRIPT_LIST_FIELDS
    }

    fn is_terminator(&self) -> bool {
        self.0.name == Value::Integer(0)
            && self.0.object_id == 0
            && self.0.minimum_chapter == 0
            && self.0.medium_chapter == 0
            && self.0.maximum_chapter == 0
    }

    fn get(&self, field: &str) -> Option<Value> {
        let e = &self.0;
        match field {
            "name" => Some(e.name),
            "object_id" => Some(Value::Integer(e.object_id)),
            "minimum_chapter" => Some(Value::Integer(e.minimum_chapter)),
            "medium_chapter" => Some(Value::Integer(e.medium_chapter)),
            "maximum_chapter" => Some(Value::Integer(e.maximum_chapter)),
            "flagname" => Some(e.flagname),
            "flag_value_condition" => Some(Value::Integer(e.flag_value_condition)),
            "target_script" => Some(Value::Integer(u32::from(e.target_script))),
            "unknown" => Some(Value::Integer(e.unknown)),
            "entrypoint" => Some(e.entrypoint),
            "zone_id" => Some(Value::Integer(e.zone_id)),
            "area_id" => Some(Value::Integer(e.area_id)),
            "position_id" => Some(Value::Integer(e.position_id)),
            "pad_0x34" => Some(Value::Integer(e.pad_0x34)),
            "after_script_entrypoint" => Some(e.after_script_entrypoint),
            "animation" => Some(e.animation),
            "flagname2" => Some(e.flagname2),
            _ => None,
        }
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        fn require_int(field: &str, value: Value) -> Result<u32> {
            match value {
                Value::Integer(v) => Ok(v),
                Value::Pointer(_) => Err(Error::Validation(format!(
                    "field '{field}' expects an integer value, not a pointer"
                ))),
            }
        }

        let e = &mut self.0;
        match field {
            "name" => e.name = value,
            "object_id" => e.object_id = require_int(field, value)?,
            "minimum_chapter" => e.minimum_chapter = require_int(field, value)?,
            "medium_chapter" => e.medium_chapter = require_int(field, value)?,
            "maximum_chapter" => e.maximum_chapter = require_int(field, value)?,
            "flagname" => e.flagname = value,
            "flag_value_condition" => e.flag_value_condition = require_int(field, value)?,
            //TODO: validate range
            "target_script" => e.target_script = require_int(field, value)? as u8,
            "unknown" => e.unknown = require_int(field, value)?,
            "entrypoint" => e.entrypoint = value,
            "zone_id" => e.zone_id = require_int(field, value)?,
            "area_id" => e.area_id = require_int(field, value)?,
            "position_id" => e.position_id = require_int(field, value)?,
            "pad_0x34" => e.pad_0x34 = require_int(field, value)?,
            "after_script_entrypoint" => e.after_script_entrypoint = value,
            "animation" => e.animation = value,
            "flagname2" => e.flagname2 = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
    fn size() -> usize {
        0x44
    }

    fn read<R>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        relocation_table: &RelocationTable,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
    {
        ScriptListEntry::read(data, base_offset, resolve_string, relocation_table).map(Self)
    }
}
