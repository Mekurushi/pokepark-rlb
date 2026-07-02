use crate::entry_schemas::codec::{EntryDeserializer, EntrySerializer};
use crate::rlb_file::StringId;
use crate::util::require_int;
use crate::TableEntry;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct WanderingDataTable {
    pokemon_unlock_id: u32,
    pokemon_friendship_id: u32,
    enabled: u8,
    pad: [u8; 3],
}

impl TableEntry for WanderingDataTable {
    const SIZE: usize = 0xC;
    fn fields(&self) -> &[FieldDescriptor] {
        WANDERING_DATA_FIELDS
    }
    fn is_terminator(&self) -> bool {
        self.pokemon_unlock_id == 0xFFFFFFFF
            && self.pokemon_friendship_id == 0xFFFFFFFF
            && self.enabled == 0
    }
    fn get(&self, field: &str) -> Option<Value> {
        match field {
            "pokemon_unlock_id" => Some(Value::Integer(self.pokemon_unlock_id)),
            "pokemon_friendship_id" => Some(Value::Integer(self.pokemon_friendship_id)),
            "enabled" => Some(Value::Integer(self.enabled as u32)),
            _ => None,
        }
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "pokemon_unlock_id" => self.pokemon_unlock_id = require_int(field, value)?,
            "pokemon_friendship_id" => self.pokemon_friendship_id = require_int(field, value)?,
            "enabled" => self.enabled = require_int(field, value)? as u8, // TODO: bool type
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
            pokemon_unlock_id: de.read_u32()?,
            pokemon_friendship_id: de.read_u32()?,
            enabled: de.read_u8()?,
            pad: de.read_pad()?,
        })
    }
    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        ser.write_u32(self.pokemon_unlock_id);
        ser.write_u32(self.pokemon_friendship_id);
        ser.write_u8(self.enabled);
        ser.write_pad(&self.pad);
        Ok(())
    }
}

pub const WANDERING_DATA_FIELDS: &[FieldDescriptor] = &[
    FieldDescriptor {
        name: "pokemon_unlock_id",
    },
    FieldDescriptor {
        name: "pokemon_friendship_id",
    },
    FieldDescriptor { name: "enabled" },
];
