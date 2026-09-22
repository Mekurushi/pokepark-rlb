use crate::table::ParseContext;
use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::field::{FieldConstraint, FieldKind, IntegerKind};
use crate::table::serialization::RelocatableTable;
use crate::table::{RowBoundary, RowLayout, SchemaDescriptor, SchemaId};
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct ScriptListTable {
    entries: Vec<ScriptListEntry>,
    terminator: ScriptListEntry,
}

impl ScriptListTable {
    const ENTRY_SIZE: usize = ScriptListEntry::SIZE;
    pub(crate) const SCHEMA: SchemaDescriptor = SchemaDescriptor {
        id: SchemaId::ScriptList,
        description: "script wiring",
        fields: ScriptListEntry::FIELDS,
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
            let candidate = ScriptListEntry::read(&mut de)?;
            let is_terminator = candidate.name == Value::String(None)
                && candidate.object_id == Value::Integer(0)
                && candidate.minimum_chapter == Value::Integer(0)
                && candidate.medium_chapter == Value::Integer(0)
                && candidate.maximum_chapter == Value::Integer(0);

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
        ScriptListEntry::FIELDS
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
}

#[derive(Clone, Debug)]
struct ScriptListEntry {
    name: Value,
    object_id: Value,
    minimum_chapter: Value,
    medium_chapter: Value,
    maximum_chapter: Value,
    flagname: Value,
    flag_value_condition: Value,
    target_script: Value,
    pad_0x1d: [u8; 3],
    unknown: Value,
    entrypoint: Value,
    zone_id: Value,
    area_id: Value,
    position_id: Value,
    pad_0x34: Value,
    after_script_entrypoint: Value,
    animation: Value,
    flagname2: Value,
}

impl ScriptListEntry {
    pub(crate) const SIZE: usize = 0x44;
    pub(crate) const FIELDS: &'static [FieldDescriptor] = SCRIPT_LIST_FIELDS;

    pub(crate) fn get(&self, field: &str) -> Option<Value> {
        match field {
            "name" => Some(self.name.clone()),
            "object_id" => Some(self.object_id.clone()),
            "minimum_chapter" => Some(self.minimum_chapter.clone()),
            "medium_chapter" => Some(self.medium_chapter.clone()),
            "maximum_chapter" => Some(self.maximum_chapter.clone()),
            "flagname" => Some(self.flagname.clone()),
            "flag_value_condition" => Some(self.flag_value_condition.clone()),
            "target_script" => Some(self.target_script.clone()),
            "unknown" => Some(self.unknown.clone()),
            "entrypoint" => Some(self.entrypoint.clone()),
            "zone_id" => Some(self.zone_id.clone()),
            "area_id" => Some(self.area_id.clone()),
            "position_id" => Some(self.position_id.clone()),
            "pad_0x34" => Some(self.pad_0x34.clone()),
            "after_script_entrypoint" => Some(self.after_script_entrypoint.clone()),
            "animation" => Some(self.animation.clone()),
            "flagname2" => Some(self.flagname2.clone()),
            _ => None,
        }
    }
    pub(crate) fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "name" => self.name = value,
            "object_id" => self.object_id = value,
            "minimum_chapter" => self.minimum_chapter = value,
            "medium_chapter" => self.medium_chapter = value,
            "maximum_chapter" => self.maximum_chapter = value,
            "flagname" => self.flagname = value,
            "flag_value_condition" => self.flag_value_condition = value,
            //TODO: validate range
            "target_script" => self.target_script = value,
            "unknown" => self.unknown = value,
            "entrypoint" => self.entrypoint = value,
            "zone_id" => self.zone_id = value,
            "area_id" => self.area_id = value,
            "position_id" => self.position_id = value,
            "pad_0x34" => self.pad_0x34 = value,
            "after_script_entrypoint" => self.after_script_entrypoint = value,
            "animation" => self.animation = value,
            "flagname2" => self.flagname2 = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }

    pub(crate) fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        Ok(Self {
            name: de.read_string_pointer()?,
            object_id: Value::Integer(de.read_u32()?),
            minimum_chapter: Value::Integer(de.read_u32()?),
            medium_chapter: Value::Integer(de.read_u32()?),
            maximum_chapter: Value::Integer(de.read_u32()?),
            flagname: de.read_string_pointer()?,
            flag_value_condition: Value::Integer(de.read_u32()?),
            target_script: Value::Integer(de.read_u8()?.into()),
            pad_0x1d: de.read_pad()?,
            unknown: Value::Integer(de.read_u32()?),
            entrypoint: de.read_string_pointer()?,
            zone_id: Value::Integer(de.read_u32()?),
            area_id: Value::Integer(de.read_u32()?),
            position_id: Value::Integer(de.read_u32()?),
            pad_0x34: Value::Integer(de.read_u32()?),
            after_script_entrypoint: de.read_string_pointer()?,
            animation: de.read_string_pointer()?,
            flagname2: de.read_string_pointer()?,
        })
    }
    pub(crate) fn write<'a>(&'a self, ser: &mut EntrySerializer<'a>) -> Result<()> {
        ser.write_string_pointer(&self.name)?;
        ser.write_u32(self.object_id.as_integer()?);
        ser.write_u32(self.minimum_chapter.as_integer()?);
        ser.write_u32(self.medium_chapter.as_integer()?);
        ser.write_u32(self.maximum_chapter.as_integer()?);
        ser.write_string_pointer(&self.flagname)?;
        ser.write_u32(self.flag_value_condition.as_integer()?);
        ser.write_u8(self.target_script.as_integer()? as u8); // TODO: width based checks
        ser.write_pad(&self.pad_0x1d);
        ser.write_u32(self.unknown.as_integer()?);
        ser.write_string_pointer(&self.entrypoint)?;
        ser.write_u32(self.zone_id.as_integer()?);
        ser.write_u32(self.area_id.as_integer()?);
        ser.write_u32(self.position_id.as_integer()?);
        ser.write_u32(self.pad_0x34.as_integer()?);
        ser.write_string_pointer(&self.after_script_entrypoint)?;
        ser.write_string_pointer(&self.animation)?;
        ser.write_string_pointer(&self.flagname2)?;

        Ok(())
    }
}

const SCRIPT_LIST_FIELDS: &[FieldDescriptor] = &[
    FieldDescriptor {
        name: "name",
        description: "",
        kind: FieldKind::String,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "object_id",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "minimum_chapter",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "medium_chapter",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "maximum_chapter",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "flagname",
        description: "",
        kind: FieldKind::String,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "flag_value_condition",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "target_script",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U8),
        constraint: FieldConstraint::TableIndex {
            table: "FsbFileListData",
        },
    },
    FieldDescriptor {
        name: "unknown",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "entrypoint",
        description: "",
        kind: FieldKind::String,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "zone_id",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "area_id",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "position_id",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "pad_0x34",
        description: "",
        kind: FieldKind::Integer(IntegerKind::U32),
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "after_script_entrypoint",
        description: "",
        kind: FieldKind::String,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "animation",
        description: "",
        kind: FieldKind::String,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "flagname2",
        description: "",
        kind: FieldKind::String,
        constraint: FieldConstraint::None,
    },
];
