use crate::TableEntry;
use crate::entry_schemas::codec::{EntryDeserializer, EntrySerializer};
use crate::entry_schemas::{FieldConstraint, FieldKind, Terminator};
use crate::string_pool::StringPool;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct ScriptListTable {
    entries: Vec<ScriptListEntry>,
    terminator: ScriptListEntry,
}

impl ScriptListTable {
    const ENTRY_SIZE: usize = <ScriptListEntry as TableEntry>::SIZE;

    pub(crate) fn parse<R, E>(
        data: &[u8],
        root_address: usize,
        resolve_string: &mut R,
        is_relocated: &mut E,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
        let mut entries = Vec::new();
        let mut offset = root_address;

        loop {
            let bytes =
                data.get(offset..offset + Self::ENTRY_SIZE)
                    .ok_or(Error::UnexpectedEof {
                        context: "parsing ScriptList record",
                    })?;
            let mut de = EntryDeserializer::new(bytes, offset, resolve_string, is_relocated);

            if let Some(terminator) = <ScriptListEntry as Terminator>::recognize(&mut de)? {
                return Ok(Self {
                    entries,
                    terminator,
                });
            }

            let mut de = EntryDeserializer::new(bytes, offset, resolve_string, is_relocated);
            entries.push(ScriptListEntry::read(&mut de)?);
            offset += Self::ENTRY_SIZE;
        }
    }

    pub(crate) fn serialize(
        &self,
        out: &mut Vec<u8>,
        base_offset: usize,
        strings: &StringPool,
        relocations: &mut Vec<u32>,
    ) -> Result<()> {
        for (index, entry) in self.entries.iter().enumerate() {
            let mut serializer =
                EntrySerializer::new(base_offset + index * Self::ENTRY_SIZE, strings, relocations);
            <ScriptListEntry as TableEntry>::write(entry, &mut serializer)?;
            serializer.finish(out, Self::ENTRY_SIZE)?;
        }

        let mut serializer = EntrySerializer::new(
            base_offset + self.entries.len() * Self::ENTRY_SIZE,
            strings,
            relocations,
        );
        <ScriptListEntry as Terminator>::write(&self.terminator, &mut serializer)?;
        serializer.finish(out, <ScriptListEntry as Terminator>::SIZE)
    }

    pub(crate) fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        for entry in &self.entries {
            <ScriptListEntry as TableEntry>::visit_strings(entry, visit)?;
        }
        <ScriptListEntry as Terminator>::visit_strings(&self.terminator, visit)
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
pub struct ScriptListEntry {
    pub name: Value,
    pub object_id: Value,
    pub minimum_chapter: Value,
    pub medium_chapter: Value,
    pub maximum_chapter: Value,
    pub flagname: Value,
    pub flag_value_condition: Value,
    pub target_script: Value,
    pub pad_0x1d: [u8; 3],
    pub unknown: Value,
    pub entrypoint: Value,
    pub zone_id: Value,
    pub area_id: Value,
    pub position_id: Value,
    pub pad_0x34: Value,
    pub after_script_entrypoint: Value,
    pub animation: Value,
    pub flagname2: Value,
}

impl TableEntry for ScriptListEntry {
    const SIZE: usize = 0x44;
    const FIELDS: &'static [FieldDescriptor] = SCRIPT_LIST_FIELDS;

    fn get(&self, field: &str) -> Option<Value> {
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
    fn set(&mut self, field: &str, value: Value) -> rlb_error::Result<()> {
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

    fn read<R, E>(de: &mut EntryDeserializer<'_, R, E>) -> Result<Self>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
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
    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
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

    fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        for value in [
            &self.name,
            &self.flagname,
            &self.entrypoint,
            &self.after_script_entrypoint,
            &self.animation,
            &self.flagname2,
        ] {
            if let Value::String(Some(value)) = value {
                visit(value)?;
            }
        }
        Ok(())
    }
}

impl Terminator for ScriptListEntry {
    const SIZE: usize = <Self as TableEntry>::SIZE;

    fn recognize<R, E>(de: &mut EntryDeserializer<'_, R, E>) -> Result<Option<Self>>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
        let candidate = <Self as TableEntry>::read(de)?;
        let is_terminator = candidate.name == Value::String(None)
            && candidate.object_id == Value::Integer(0)
            && candidate.minimum_chapter == Value::Integer(0)
            && candidate.medium_chapter == Value::Integer(0)
            && candidate.maximum_chapter == Value::Integer(0);

        Ok(is_terminator.then_some(candidate))
    }

    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        <Self as TableEntry>::write(self, ser)
    }

    fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        <Self as TableEntry>::visit_strings(self, visit)
    }
}

pub const SCRIPT_LIST_FIELDS: &[FieldDescriptor] = &[
    FieldDescriptor {
        name: "name",
        description: "",
        kind: FieldKind::String,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "object_id",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "minimum_chapter",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "medium_chapter",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "maximum_chapter",
        description: "",
        kind: FieldKind::Integer,
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
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "target_script",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::TableIndex {
            table: "FsbFileListData",
        },
    },
    FieldDescriptor {
        name: "unknown",
        description: "",
        kind: FieldKind::Integer,
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
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "area_id",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "position_id",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "pad_0x34",
        description: "",
        kind: FieldKind::Integer,
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
