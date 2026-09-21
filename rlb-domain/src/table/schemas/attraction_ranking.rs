use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::serialization::RelocatableTable;
use crate::table::{FieldConstraint, FieldDescriptor, FieldKind, ParseContext};
use crate::Value;
use rlb_error::{Error, Result};

//usable rows are capped at 26
#[derive(Clone, Debug)]
pub(crate) struct AttractionRankingTable {
    entries: Vec<AttractionRankingEntry>,
    terminator: AttractionRankingEntry,
}

impl AttractionRankingTable {
    const ENTRY_SIZE: usize = 0x1c;
    const TERMINATOR_FRIENDSHIP_ID: u32 = 0xc9;

    pub(crate) fn parse(context: &ParseContext<'_>, root: usize) -> Result<Self> {
        let mut entries = Vec::new();
        let mut offset = root;

        loop {
            let mut de = EntryDeserializer::new(context, offset);
            let candidate = AttractionRankingEntry::read(&mut de)?;

            if candidate.friendship_id == Value::Integer(Self::TERMINATOR_FRIENDSHIP_ID) {
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
        AttractionRankingEntry::FIELDS
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
struct AttractionRankingEntry {
    friendship_id: Value,
    record_target: Value,
    bonus_record_target: Value,
    unknown_0x0c: Value,
    unknown_0x10: Value,
    unknown_0x14: Value,
    unknown_0x18: Value,
}

impl AttractionRankingEntry {
    const FIELDS: &'static [FieldDescriptor] = &[
        FieldDescriptor {
            name: "friendship_id",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "record_target",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "bonus_record_target",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "unknown_0x0c",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "unknown_0x10",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "unknown_0x14",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "unknown_0x18",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
    ];

    fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        Ok(Self {
            friendship_id: Value::Integer(de.read_u32()?),
            record_target: Value::Integer(de.read_u32()?),
            bonus_record_target: Value::Integer(de.read_u32()?),
            unknown_0x0c: Value::Integer(de.read_u32()?),
            unknown_0x10: Value::Integer(de.read_u32()?),
            unknown_0x14: Value::Integer(de.read_u32()?),
            unknown_0x18: Value::Integer(de.read_u32()?),
        })
    }

    fn write<'a>(&'a self, serializer: &mut EntrySerializer<'a>) -> Result<()> {
        serializer.write_u32(self.friendship_id.as_integer()?);
        serializer.write_u32(self.record_target.as_integer()?);
        serializer.write_u32(self.bonus_record_target.as_integer()?);
        serializer.write_u32(self.unknown_0x0c.as_integer()?);
        serializer.write_u32(self.unknown_0x10.as_integer()?);
        serializer.write_u32(self.unknown_0x14.as_integer()?);
        serializer.write_u32(self.unknown_0x18.as_integer()?);
        Ok(())
    }

    fn get(&self, field: &str) -> Option<Value> {
        Some(match field {
            "friendship_id" => self.friendship_id.clone(),
            "record_target" => self.record_target.clone(),
            "bonus_record_target" => self.bonus_record_target.clone(),
            "unknown_0x0c" => self.unknown_0x0c.clone(),
            "unknown_0x10" => self.unknown_0x10.clone(),
            "unknown_0x14" => self.unknown_0x14.clone(),
            "unknown_0x18" => self.unknown_0x18.clone(),
            _ => return None,
        })
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "friendship_id" => self.friendship_id = value,
            "record_target" => self.record_target = value,
            "bonus_record_target" => self.bonus_record_target = value,
            "unknown_0x0c" => self.unknown_0x0c = value,
            "unknown_0x10" => self.unknown_0x10 = value,
            "unknown_0x14" => self.unknown_0x14 = value,
            "unknown_0x18" => self.unknown_0x18 = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
}
