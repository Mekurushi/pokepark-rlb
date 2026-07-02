use crate::entry_schemas::codec::{EntryDeserializer, EntrySerializer};
use crate::entry_schemas::{FieldConstraint, FieldKind};
use crate::rlb_file::StringId;
use crate::util::checked_bool;
use crate::TableEntry;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct WanderingDataTable {
    pokemon_unlock_id: Value,
    pokemon_friendship_id: Value,
    enabled: Value,
    pad: [u8; 3],
}

impl TableEntry for WanderingDataTable {
    const SIZE: usize = 0xC;
    const FIELDS: &'static [FieldDescriptor] = &WANDERING_DATA_FIELDS;

    fn is_terminator(&self) -> bool {
        self.pokemon_unlock_id == Value::Integer(0xFFFFFFFF)
            && self.pokemon_friendship_id == Value::Integer(0xFFFFFFFF)
            && self.enabled == Value::Boolean(false)
    }
    fn get(&self, field: &str) -> Option<Value> {
        match field {
            "pokemon_unlock_id" => Some(self.pokemon_unlock_id),
            "pokemon_friendship_id" => Some(self.pokemon_friendship_id),
            "enabled" => Some(self.enabled),
            _ => None,
        }
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "pokemon_unlock_id" => self.pokemon_unlock_id = value,
            "pokemon_friendship_id" => self.pokemon_friendship_id = value,
            "enabled" => self.enabled = value,
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
            pokemon_unlock_id: Value::Integer(de.read_u32()?),
            pokemon_friendship_id: Value::Integer(de.read_u32()?),
            enabled: Value::Boolean(checked_bool(de.read_u8()?, "enabled")?),
            pad: de.read_pad()?,
        })
    }
    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        ser.write_u32(self.pokemon_unlock_id.as_integer()?);
        ser.write_u32(self.pokemon_friendship_id.as_integer()?);
        ser.write_u8(u8::from(self.enabled.as_bool()?));
        ser.write_pad(&self.pad);
        Ok(())
    }
}

pub const WANDERING_DATA_FIELDS: &[FieldDescriptor] = &[
    FieldDescriptor {
        name: "pokemon_unlock_id",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "pokemon_friendship_id",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "enabled",
        description: "",
        kind: FieldKind::Boolean,
        constraint: FieldConstraint::None,
    },
];
