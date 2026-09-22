use crate::table::ParseContext;
use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::field::{FieldConstraint, FieldKind, FloatKind, IntegerKind};
use crate::table::serialization::RelocatableTable;
use crate::table::{RowBoundary, RowLayout, SchemaDescriptor, SchemaId};
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct PokemonDispositionDataTable {
    entries: Vec<PokemonDispositionData>,
}

impl PokemonDispositionDataTable {
    const ENTRY_SIZE: usize = 0x34;
    const COUNT_OFFSET: usize = 0x1;
    pub(crate) const SCHEMA: SchemaDescriptor = SchemaDescriptor {
        id: SchemaId::PokemonDispositionData,
        description: "Pokemon disposition data",
        fields: PokemonDispositionData::FIELDS,
        rows: RowLayout {
            boundary: RowBoundary::CountedBy {
                schema: SchemaId::DispositionDataHeader,
                field: "pokemon_disposition_count",
            },
            max_rows: None,
        },
    };

    pub(crate) fn parse(context: &ParseContext<'_>, root_address: usize) -> Result<Self> {
        let header = context.table_offset("dispositionDataHeader")?;
        let count = usize::from(context.read_u8(header + Self::COUNT_OFFSET)?);
        let mut entries = Vec::with_capacity(count);
        for index in 0..count {
            let mut de = EntryDeserializer::new(context, root_address + index * Self::ENTRY_SIZE);
            entries.push(PokemonDispositionData::read(&mut de)?);
        }
        Ok(Self { entries })
    }

    pub(crate) fn serialize(&self) -> Result<RelocatableTable<'_>> {
        let mut table = RelocatableTable::default();
        for entry in &self.entries {
            let mut serializer = EntrySerializer::new();
            entry.write(&mut serializer)?;
            serializer.finish(&mut table, Self::ENTRY_SIZE)?;
        }
        Ok(table)
    }

    pub(crate) fn fields(&self) -> &'static [FieldDescriptor] {
        PokemonDispositionData::FIELDS
    }
    pub(crate) fn entry_count(&self) -> usize {
        self.entries.len()
    }
    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        self.entries.get(index)?.get(field)
    }
    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        self.entries
            .get_mut(index)
            .ok_or_else(|| Error::Validation(format!("entry index {index} out of bounds")))?
            .set(field, value)
    }
}

#[derive(Clone, Debug)]
struct PokemonDispositionData {
    object_id: Value,
    pad_0x02: [u8; 2],
    friendship_id: Value,
    walking_ai_enabled: Value,        // TODO: validate
    field_position_candidates: Value, // TODO: validate
    position_x: Value,
    position_y: Value,
    position_z: Value,
    rotation_y: Value,
    field_position_override: Value,
    pad_0x21: [u8; 3],
    unlock_id: Value,
    skill_game_type: Value,
    skill_game_start_position_group: Value, // TODO: validate
    damage_disposition: Value,              // TODO: validate
}

impl PokemonDispositionData {
    const FIELDS: &'static [FieldDescriptor] = &[
        FieldDescriptor {
            name: "object_id",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U16),
            constraint: FieldConstraint::IntegerRange {
                min: 0,
                max: u16::MAX as u32,
            },
        },
        FieldDescriptor {
            name: "friendship_id",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "walking_ai_enabled",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "field_position_candidates",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "position_x",
            description: "",
            kind: FieldKind::Float(FloatKind::F32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "position_y",
            description: "",
            kind: FieldKind::Float(FloatKind::F32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "position_z",
            description: "",
            kind: FieldKind::Float(FloatKind::F32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "rotation_y",
            description: "",
            kind: FieldKind::Float(FloatKind::F32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "field_position_override",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U8),
            constraint: FieldConstraint::IntegerRange {
                min: 0,
                max: u8::MAX as u32,
            },
        },
        FieldDescriptor {
            name: "unlock_id",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "skill_game_type",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "skill_game_start_position_group",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "damage_disposition",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U32),
            constraint: FieldConstraint::None,
        },
    ];

    fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        Ok(Self {
            object_id: Value::Integer(de.read_u16()?.into()),
            pad_0x02: de.read_pad()?,
            friendship_id: Value::Integer(de.read_u32()?),
            walking_ai_enabled: Value::Integer(de.read_u32()?),
            field_position_candidates: Value::Integer(de.read_u32()?),
            position_x: Value::Float(de.read_f32()?),
            position_y: Value::Float(de.read_f32()?),
            position_z: Value::Float(de.read_f32()?),
            rotation_y: Value::Float(de.read_f32()?),
            field_position_override: Value::Integer(de.read_u8()?.into()),
            pad_0x21: de.read_pad()?,
            unlock_id: Value::Integer(de.read_u32()?),
            skill_game_type: Value::Integer(de.read_u32()?),
            skill_game_start_position_group: Value::Integer(de.read_u32()?),
            damage_disposition: Value::Integer(de.read_u32()?),
        })
    }

    fn write<'a>(&'a self, ser: &mut EntrySerializer<'a>) -> Result<()> {
        ser.write_u16(
            u16::try_from(self.object_id.as_integer()?)
                .map_err(|_| Error::Validation("object_id exceeds u16 range".into()))?,
        );
        ser.write_pad(&self.pad_0x02);
        ser.write_u32(self.friendship_id.as_integer()?);
        ser.write_u32(self.walking_ai_enabled.as_integer()?);
        ser.write_u32(self.field_position_candidates.as_integer()?);
        ser.write_f32(self.position_x.as_float()?);
        ser.write_f32(self.position_y.as_float()?);
        ser.write_f32(self.position_z.as_float()?);
        ser.write_f32(self.rotation_y.as_float()?);
        ser.write_u8(
            u8::try_from(self.field_position_override.as_integer()?).map_err(|_| {
                Error::Validation("field_position_override exceeds u8 range".into())
            })?,
        );
        ser.write_pad(&self.pad_0x21);
        ser.write_u32(self.unlock_id.as_integer()?);
        ser.write_u32(self.skill_game_type.as_integer()?);
        ser.write_u32(self.skill_game_start_position_group.as_integer()?);
        ser.write_u32(self.damage_disposition.as_integer()?);
        Ok(())
    }

    fn get(&self, field: &str) -> Option<Value> {
        Some(match field {
            "object_id" => self.object_id.clone(),
            "friendship_id" => self.friendship_id.clone(),
            "walking_ai_enabled" => self.walking_ai_enabled.clone(),
            "field_position_candidates" => self.field_position_candidates.clone(),
            "position_x" => self.position_x.clone(),
            "position_y" => self.position_y.clone(),
            "position_z" => self.position_z.clone(),
            "rotation_y" => self.rotation_y.clone(),
            "field_position_override" => self.field_position_override.clone(),
            "unlock_id" => self.unlock_id.clone(),
            "skill_game_type" => self.skill_game_type.clone(),
            "skill_game_start_position_group" => self.skill_game_start_position_group.clone(),
            "damage_disposition" => self.damage_disposition.clone(),
            _ => return None,
        })
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "object_id" => self.object_id = value,
            "friendship_id" => self.friendship_id = value,
            "walking_ai_enabled" => self.walking_ai_enabled = value,
            "field_position_candidates" => self.field_position_candidates = value,
            "position_x" => self.position_x = value,
            "position_y" => self.position_y = value,
            "position_z" => self.position_z = value,
            "rotation_y" => self.rotation_y = value,
            "field_position_override" => self.field_position_override = value,
            "unlock_id" => self.unlock_id = value,
            "skill_game_type" => self.skill_game_type = value,
            "skill_game_start_position_group" => self.skill_game_start_position_group = value,
            "damage_disposition" => self.damage_disposition = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
}
