use crate::Value;
use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::serialization::RelocatableTable;
use crate::table::{
    FieldConstraint, FieldDescriptor, FieldKind, IntegerKind, ParseContext, RowBoundary, RowLayout,
    SchemaDescriptor, SchemaId,
};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct FlagTable {
    entries: Vec<FlagEntry>,
    terminator: FlagEntry,
}

impl FlagTable {
    const ENTRY_SIZE: usize = 0x08;
    pub(crate) const SCHEMA: SchemaDescriptor = SchemaDescriptor {
        id: SchemaId::FlagTable,
        description: "flags and their bit widths",
        fields: FlagEntry::FIELDS,
        rows: RowLayout {
            boundary: RowBoundary::Terminated,
            max_rows: None,
        },
    };

    pub(crate) fn parse(context: &ParseContext<'_>, root: usize) -> Result<Self> {
        let mut entries = Vec::new();
        let mut offset = root;

        loop {
            let mut de = EntryDeserializer::new(context, offset);
            let candidate = FlagEntry::read(&mut de)?;

            if candidate.flag_name == Value::String(None) {
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
        FlagEntry::FIELDS
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
struct FlagEntry {
    flag_name: Value,
    bit_width: Value,
    pad_0x05: [u8; 3],
}

impl FlagEntry {
    const FIELDS: &'static [FieldDescriptor] = &[
        FieldDescriptor {
            name: "flag_name",
            description: "",
            kind: FieldKind::String,
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "bit_width",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U8),
            constraint: FieldConstraint::IntegerRange {
                min: 0,
                max: u8::MAX as u32,
            },
        },
    ];

    fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        Ok(Self {
            flag_name: de.read_string_pointer()?,
            bit_width: Value::Integer(de.read_u8()?.into()),
            pad_0x05: de.read_pad()?,
        })
    }

    fn write<'a>(&'a self, serializer: &mut EntrySerializer<'a>) -> Result<()> {
        serializer.write_string_pointer(&self.flag_name)?;
        serializer.write_u8(
            u8::try_from(self.bit_width.as_integer()?)
                .map_err(|_| Error::Validation("bit_width exceeds u8 range".into()))?,
        );
        serializer.write_pad(&self.pad_0x05);
        Ok(())
    }

    fn get(&self, field: &str) -> Option<Value> {
        Some(match field {
            "flag_name" => self.flag_name.clone(),
            "bit_width" => self.bit_width.clone(),
            _ => return None,
        })
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "flag_name" => self.flag_name = value,
            "bit_width" => self.bit_width = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
}
