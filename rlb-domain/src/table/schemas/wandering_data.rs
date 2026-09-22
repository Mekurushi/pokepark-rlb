use crate::table::ParseContext;
use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::field::{FieldConstraint, FieldKind, IntegerKind};
use crate::table::serialization::RelocatableTable;
use crate::table::{RowBoundary, RowLayout, SchemaDescriptor, SchemaId};
use crate::util::checked_bool;
use crate::{FieldDescriptor, Row, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct WanderingDataTable {
    entries: Vec<WanderingDataEntry>,
    terminator: WanderingDataEntry,
}

impl WanderingDataTable {
    const ENTRY_SIZE: usize = WanderingDataEntry::SIZE;
    pub(crate) const SCHEMA: SchemaDescriptor = SchemaDescriptor {
        id: SchemaId::WanderingData,
        description: "Wandering Pokémon unlock and friendship data.",
        fields: WanderingDataEntry::FIELDS,
        rows: RowLayout {
            boundary: RowBoundary::Terminated,
            max_rows: None,
        },
    };

    pub(crate) fn parse(context: &ParseContext<'_>, root_address: usize) -> Result<Self> {
        let mut entries = Vec::new();
        let mut offset = root_address;

        loop {
            let mut de = EntryDeserializer::new(context, offset);
            let candidate = WanderingDataEntry::read(&mut de)?;
            let is_terminator = candidate.pokemon_unlock_id == Value::Integer(0xFFFF_FFFF)
                && candidate.pokemon_friendship_id == Value::Integer(0xFFFF_FFFF)
                && candidate.enabled == Value::Boolean(false);

            if is_terminator {
                return Ok(Self {
                    entries,
                    terminator: candidate,
                });
            }

            entries.push(candidate);
            offset += Self::ENTRY_SIZE;
        }
    }

    pub(crate) fn serialize(&self) -> Result<RelocatableTable<'_>> {
        let mut table = RelocatableTable::default();
        for entry in &self.entries {
            let mut serializer = EntrySerializer::new();
            entry.write(&mut serializer)?;
            serializer.finish(&mut table, Self::ENTRY_SIZE)?;
        }

        let mut serializer = EntrySerializer::new();
        self.terminator.write(&mut serializer)?;
        serializer.finish(&mut table, Self::ENTRY_SIZE)?;
        Ok(table)
    }

    pub(crate) fn fields(&self) -> &'static [FieldDescriptor] {
        WanderingDataEntry::FIELDS
    }

    pub(crate) fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        self.entries.get(index)?.get(field)
    }

    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        let entry = self
            .entries
            .get_mut(index)
            .ok_or_else(|| Error::Validation(format!("entry index {index} out of bounds")))?;
        entry.set(field, value)
    }

    pub(crate) fn append_row(&mut self, row: &Row) -> Result<usize> {
        let index = self.entries.len();
        self.entries.push(WanderingDataEntry::from_row(row)?);
        Ok(index)
    }
}

#[derive(Clone, Debug)]
struct WanderingDataEntry {
    pokemon_unlock_id: Value,
    pokemon_friendship_id: Value,
    enabled: Value,
    pad: [u8; 3],
}

impl WanderingDataEntry {
    pub(crate) const SIZE: usize = 0xC;
    pub(crate) const FIELDS: &'static [FieldDescriptor] = WANDERING_DATA_FIELDS;

    fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            pokemon_unlock_id: row
                .get("pokemon_unlock_id")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"pokemon_unlock_id\"".into()))?,
            pokemon_friendship_id: row.get("pokemon_friendship_id").cloned().ok_or_else(|| {
                Error::Validation("missing field \"pokemon_friendship_id\"".into())
            })?,
            enabled: row
                .get("enabled")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"enabled\"".into()))?,
            pad: [0; 3],
        })
    }

    pub(crate) fn get(&self, field: &str) -> Option<Value> {
        match field {
            "pokemon_unlock_id" => Some(self.pokemon_unlock_id.clone()),
            "pokemon_friendship_id" => Some(self.pokemon_friendship_id.clone()),
            "enabled" => Some(self.enabled.clone()),
            _ => None,
        }
    }

    pub(crate) fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "pokemon_unlock_id" => self.pokemon_unlock_id = value,
            "pokemon_friendship_id" => self.pokemon_friendship_id = value,
            "enabled" => self.enabled = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }

    pub(crate) fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        Ok(Self {
            pokemon_unlock_id: Value::Integer(de.read_u32()?),
            pokemon_friendship_id: Value::Integer(de.read_u32()?),
            enabled: Value::Boolean(checked_bool(de.read_u8()?, "enabled")?),
            pad: de.read_pad()?,
        })
    }
    pub(crate) fn write<'a>(&'a self, ser: &mut EntrySerializer<'a>) -> Result<()> {
        ser.write_u32(self.pokemon_unlock_id.as_integer()?);
        ser.write_u32(self.pokemon_friendship_id.as_integer()?);
        ser.write_u8(u8::from(self.enabled.as_bool()?));
        ser.write_pad(&self.pad);
        Ok(())
    }
}

const WANDERING_DATA_FIELDS: &[FieldDescriptor] = &[
    FieldDescriptor {
        name: "pokemon_unlock_id",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "pokemon_friendship_id",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "enabled",
        description: "",
        kind: FieldKind::Boolean,
        constraint: FieldConstraint::None,
    },
];
