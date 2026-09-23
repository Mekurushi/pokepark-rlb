use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::serialization::RelocatableTable;
use crate::table::{
    FieldConstraint, FieldDescriptor, FieldKind, FloatKind, IntegerKind, ParseContext, RowBoundary,
    RowLayout, SchemaDescriptor, SchemaId,
};
use crate::{Row, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct ItemDispositionDataTable {
    entries: Vec<ItemDispositionData>,
}
impl ItemDispositionDataTable {
    pub(crate) fn create(rows: &[Row]) -> Result<Self> {
        let mut table = Self {
            entries: Vec::new(),
        };
        for row in rows {
            table.append_row(row)?;
        }
        Ok(table)
    }

    const ENTRY_SIZE: usize = 0x24;
    pub(crate) const SCHEMA: SchemaDescriptor = SchemaDescriptor {
        id: SchemaId::ItemDispositionData,
        description: "Item disposition data",
        fields: ItemDispositionData::FIELDS,
        rows: RowLayout {
            boundary: RowBoundary::CountedBy {
                schema: SchemaId::DispositionDataHeader,
                field: "item_disposition_count",
            },
            max_rows: None,
        },
    };
    pub(crate) fn parse(context: &ParseContext<'_>, root: usize) -> Result<Self> {
        let header = context.table_offset("dispositionDataHeader")?;
        let count = usize::from(context.read_u8(header + 2)?);
        let mut entries = Vec::with_capacity(count);
        for index in 0..count {
            let mut de = EntryDeserializer::new(context, root + index * Self::ENTRY_SIZE);
            entries.push(ItemDispositionData::read(&mut de)?);
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
        ItemDispositionData::FIELDS
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
        let entry = ItemDispositionData {
            object_id: row
                .get("object_id")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"object_id\"".into()))?,
            object_type: row
                .get("object_type")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"object_type\"".into()))?,
            item_kind: row
                .get("item_kind")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"item_kind\"".into()))?,
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
            unknown_0x18: row
                .get("unknown_0x18")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"unknown_0x18\"".into()))?,
            unknown_0x1c: row
                .get("unknown_0x1c")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"unknown_0x1c\"".into()))?,
            unknown_0x20: row
                .get("unknown_0x20")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"unknown_0x20\"".into()))?,
        };
        let index = self.entries.len();
        self.entries.push(entry);
        Ok(index)
    }

    pub(crate) fn remove_row(&mut self, index: usize) -> Result<()> {
        if index >= self.entries.len() {
            return Err(Error::Validation(format!(
                "row index {index} out of bounds"
            )));
        }
        self.entries.remove(index);
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ItemDispositionData {
    object_id: Value,
    object_type: Value,
    item_kind: Value,
    position_x: Value,
    position_y: Value,
    position_z: Value,
    rotation_y: Value,
    unknown_0x18: Value,
    unknown_0x1c: Value,
    unknown_0x20: Value,
}
impl ItemDispositionData {
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
            name: "object_type",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U16),
            constraint: FieldConstraint::IntegerRange {
                min: 0,
                max: u16::MAX as u32,
            },
        },
        FieldDescriptor {
            name: "item_kind",
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
            name: "unknown_0x18",
            description: "",
            kind: FieldKind::Float(FloatKind::F32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "unknown_0x1c",
            description: "",
            kind: FieldKind::Float(FloatKind::F32),
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "unknown_0x20",
            description: "",
            kind: FieldKind::Float(FloatKind::F32),
            constraint: FieldConstraint::None,
        },
    ];
    fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        Ok(Self {
            object_id: Value::Integer(de.read_u16()?.into()),
            object_type: Value::Integer(de.read_u16()?.into()),
            item_kind: Value::Integer(de.read_u32()?),
            position_x: Value::Float(de.read_f32()?),
            position_y: Value::Float(de.read_f32()?),
            position_z: Value::Float(de.read_f32()?),
            rotation_y: Value::Float(de.read_f32()?),
            unknown_0x18: Value::Float(de.read_f32()?),
            unknown_0x1c: Value::Float(de.read_f32()?),
            unknown_0x20: Value::Float(de.read_f32()?),
        })
    }
    fn write<'a>(&'a self, ser: &mut EntrySerializer<'a>) -> Result<()> {
        ser.write_u16(as_u16(&self.object_id, "object_id")?);
        ser.write_u16(as_u16(&self.object_type, "object_type")?);
        ser.write_u32(self.item_kind.as_integer()?);
        ser.write_f32(self.position_x.as_float()?);
        ser.write_f32(self.position_y.as_float()?);
        ser.write_f32(self.position_z.as_float()?);
        ser.write_f32(self.rotation_y.as_float()?);
        ser.write_f32(self.unknown_0x18.as_float()?);
        ser.write_f32(self.unknown_0x1c.as_float()?);
        ser.write_f32(self.unknown_0x20.as_float()?);
        Ok(())
    }
    fn get(&self, field: &str) -> Option<Value> {
        Some(match field {
            "object_id" => self.object_id.clone(),
            "object_type" => self.object_type.clone(),
            "item_kind" => self.item_kind.clone(),
            "position_x" => self.position_x.clone(),
            "position_y" => self.position_y.clone(),
            "position_z" => self.position_z.clone(),
            "rotation_y" => self.rotation_y.clone(),
            "unknown_0x18" => self.unknown_0x18.clone(),
            "unknown_0x1c" => self.unknown_0x1c.clone(),
            "unknown_0x20" => self.unknown_0x20.clone(),
            _ => return None,
        })
    }
    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "object_id" => self.object_id = value,
            "object_type" => self.object_type = value,
            "item_kind" => self.item_kind = value,
            "position_x" => self.position_x = value,
            "position_y" => self.position_y = value,
            "position_z" => self.position_z = value,
            "rotation_y" => self.rotation_y = value,
            "unknown_0x18" => self.unknown_0x18 = value,
            "unknown_0x1c" => self.unknown_0x1c = value,
            "unknown_0x20" => self.unknown_0x20 = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
}
fn as_u16(value: &Value, field: &str) -> Result<u16> {
    u16::try_from(value.as_integer()?)
        .map_err(|_| Error::Validation(format!("{field} exceeds u16 range")))
}
