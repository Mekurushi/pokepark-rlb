use crate::rlb_file::StringId;
use crate::string_pool::SerializedStringPoolContext;
use crate::util::{read_bytes, read_u32, read_u8, require_int};
use crate::TableEntry;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct WanderingDataTable {
    pokemon_unlock_id: u32,
    pokemon_friendship_id: u32,
    enabled: u8,
    _pad: [u8; 3],
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

    fn read<R, E>(
        data: &[u8],
        _base_offset: usize,
        _resolve_string: &mut R,
        _is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
        E: FnMut(u32) -> bool,
    {
        Ok(Self {
            pokemon_unlock_id: read_u32(data, 0x00)?,
            pokemon_friendship_id: read_u32(data, 0x04)?,
            enabled: read_u8(data, 0x08)?,
            _pad: read_bytes(data, 0x9)?,
        })
    }
    fn write(
        &self,
        out: &mut Vec<u8>,
        _base_offset: usize,
        _strings: &SerializedStringPoolContext<StringId>,
        _relocations: &mut Vec<u32>,
    ) -> Result<()> {
        out.extend_from_slice(&self.pokemon_unlock_id.to_be_bytes());
        out.extend_from_slice(&self.pokemon_friendship_id.to_be_bytes());
        out.extend_from_slice(&self.enabled.to_be_bytes());
        out.extend_from_slice(&self._pad);
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
