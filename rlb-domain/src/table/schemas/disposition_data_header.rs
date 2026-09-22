use crate::table::ParseContext;
use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::field::{FieldConstraint, FieldKind, IntegerKind};
use crate::table::serialization::RelocatableTable;
use crate::table::{RowBoundary, RowLayout, SchemaDescriptor, SchemaId};
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct DispositionDataHeaderTable {
    entry: DispositionDataHeader,
}

impl DispositionDataHeaderTable {
    const ENTRY_SIZE: usize = 0x8;

    pub(crate) const SCHEMA: SchemaDescriptor = SchemaDescriptor {
        id: SchemaId::DispositionDataHeader,
        description: "Counts for the disposition data tables",
        fields: DispositionDataHeader::FIELDS,
        rows: RowLayout {
            boundary: RowBoundary::Fixed { rows: 1 },
            max_rows: None,
        },
    };

    pub(crate) fn parse(context: &ParseContext<'_>, root_address: usize) -> Result<Self> {
        let mut de = EntryDeserializer::new(context, root_address);
        Ok(Self {
            entry: DispositionDataHeader::read(&mut de)?,
        })
    }

    pub(crate) fn serialize(&self) -> Result<RelocatableTable<'_>> {
        let mut table = RelocatableTable::default();
        let mut serializer = EntrySerializer::new();
        self.entry.write(&mut serializer)?;
        serializer.finish(&mut table, Self::ENTRY_SIZE)?;
        Ok(table)
    }

    pub(crate) fn fields(&self) -> &'static [FieldDescriptor] {
        DispositionDataHeader::FIELDS
    }

    pub(crate) fn entry_count(&self) -> usize {
        1
    }

    pub(crate) fn get_field(&self, index: usize, field: &str) -> Option<Value> {
        (index == 0).then(|| self.entry.get(field)).flatten()
    }

    pub(crate) fn set_field(&mut self, index: usize, field: &str, value: Value) -> Result<()> {
        if index != 0 {
            return Err(Error::Validation(format!(
                "entry index {index} out of bounds"
            )));
        }
        self.entry.set(field, value)
    }
}

#[derive(Clone, Debug)]
struct DispositionDataHeader {
    player_disposition_count: Value,
    pokemon_disposition_count: Value,
    item_disposition_count: Value,
    pad_0x03: u8,
    item_kind_total_num_count: Value,
    pad_0x05: [u8; 3],
}

impl DispositionDataHeader {
    const FIELDS: &'static [FieldDescriptor] = &[
        FieldDescriptor {
            name: "player_disposition_count",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U8),
            constraint: FieldConstraint::IntegerRange {
                min: 0,
                max: u8::MAX as u32,
            },
        },
        FieldDescriptor {
            name: "pokemon_disposition_count",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U8),
            constraint: FieldConstraint::IntegerRange {
                min: 0,
                max: u8::MAX as u32,
            },
        },
        FieldDescriptor {
            name: "item_disposition_count",
            description: "",
            kind: FieldKind::Integer(IntegerKind::U8),
            constraint: FieldConstraint::IntegerRange {
                min: 0,
                max: u8::MAX as u32,
            },
        },
        FieldDescriptor {
            name: "item_kind_total_num_count",
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
            player_disposition_count: Value::Integer(de.read_u8()?.into()),
            pokemon_disposition_count: Value::Integer(de.read_u8()?.into()),
            item_disposition_count: Value::Integer(de.read_u8()?.into()),
            pad_0x03: de.read_u8()?,
            item_kind_total_num_count: Value::Integer(de.read_u8()?.into()),
            pad_0x05: de.read_pad()?,
        })
    }

    fn write<'a>(&'a self, ser: &mut EntrySerializer<'a>) -> Result<()> {
        ser.write_u8(as_u8(&self.player_disposition_count)?);
        ser.write_u8(as_u8(&self.pokemon_disposition_count)?);
        ser.write_u8(as_u8(&self.item_disposition_count)?);
        ser.write_u8(self.pad_0x03);
        ser.write_u8(as_u8(&self.item_kind_total_num_count)?);
        ser.write_pad(&self.pad_0x05);
        Ok(())
    }

    fn get(&self, field: &str) -> Option<Value> {
        match field {
            "player_disposition_count" => Some(self.player_disposition_count.clone()),
            "pokemon_disposition_count" => Some(self.pokemon_disposition_count.clone()),
            "item_disposition_count" => Some(self.item_disposition_count.clone()),
            "item_kind_total_num_count" => Some(self.item_kind_total_num_count.clone()),
            _ => None,
        }
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "player_disposition_count" => self.player_disposition_count = value,
            "pokemon_disposition_count" => self.pokemon_disposition_count = value,
            "item_disposition_count" => self.item_disposition_count = value,
            "item_kind_total_num_count" => self.item_kind_total_num_count = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
}

fn as_u8(value: &Value) -> Result<u8> {
    u8::try_from(value.as_integer()?)
        .map_err(|_| Error::Validation("integer value exceeds u8 range".into()))
}
