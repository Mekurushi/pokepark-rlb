use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::serialization::RelocatableTable;
use crate::table::{
    FieldConstraint, FieldDescriptor, FieldKind, FloatKind, ParseContext, RowBoundary, RowLayout,
    SchemaDescriptor, SchemaId,
};
use crate::{Row, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct PlayerDispositionDataTable {
    entries: Vec<PlayerDispositionData>,
}

impl PlayerDispositionDataTable {
    const ENTRY_SIZE: usize = 0x10;
    pub(crate) const SCHEMA: SchemaDescriptor = SchemaDescriptor {
        id: SchemaId::PlayerDispositionData,
        description: "Player disposition data",
        fields: PlayerDispositionData::FIELDS,
        rows: RowLayout {
            boundary: RowBoundary::CountedBy {
                schema: SchemaId::DispositionDataHeader,
                field: "player_disposition_count",
            },
            max_rows: None,
        },
    };
    pub(crate) fn parse(context: &ParseContext<'_>, root: usize) -> Result<Self> {
        let header = context.table_offset("dispositionDataHeader")?;
        let count = usize::from(context.read_u8(header)?);
        let mut entries = Vec::with_capacity(count);
        for index in 0..count {
            let mut de = EntryDeserializer::new(context, root + index * Self::ENTRY_SIZE);
            entries.push(PlayerDispositionData::read(&mut de)?);
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
        PlayerDispositionData::FIELDS
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

    pub(crate) fn append_row(&mut self, row: &Row) -> Result<usize> {
        let entry = PlayerDispositionData {
            position_x: row
                .get("position_x")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"position_x\"".into()))?,
            position_y: row
                .get("position_y")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"position_y\"".into()))?,
            position_z: row
                .get("position_z")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"position_z\"".into()))?,
            rotation_y: row
                .get("rotation_y")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"rotation_y\"".into()))?,
        };
        let index = self.entries.len();
        self.entries.push(entry);
        Ok(index)
    }
}
// I think this could be unused in the game TODO: validate
#[derive(Clone, Debug)]
struct PlayerDispositionData {
    position_x: Value,
    position_y: Value,
    position_z: Value,
    rotation_y: Value,
}
impl PlayerDispositionData {
    const FIELDS: &'static [FieldDescriptor] = &[
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
    ];
    fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        Ok(Self {
            position_x: Value::Float(de.read_f32()?),
            position_y: Value::Float(de.read_f32()?),
            position_z: Value::Float(de.read_f32()?),
            rotation_y: Value::Float(de.read_f32()?),
        })
    }
    fn write<'a>(&'a self, ser: &mut EntrySerializer<'a>) -> Result<()> {
        ser.write_f32(self.position_x.as_float()?);
        ser.write_f32(self.position_y.as_float()?);
        ser.write_f32(self.position_z.as_float()?);
        ser.write_f32(self.rotation_y.as_float()?);
        Ok(())
    }
    fn get(&self, field: &str) -> Option<Value> {
        Some(match field {
            "position_x" => self.position_x.clone(),
            "position_y" => self.position_y.clone(),
            "position_z" => self.position_z.clone(),
            "rotation_y" => self.rotation_y.clone(),
            _ => return None,
        })
    }
    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "position_x" => self.position_x = value,
            "position_y" => self.position_y = value,
            "position_z" => self.position_z = value,
            "rotation_y" => self.rotation_y = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
}
