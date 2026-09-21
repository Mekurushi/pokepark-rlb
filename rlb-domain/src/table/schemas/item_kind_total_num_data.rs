use crate::Value;
use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::serialization::RelocatableTable;
use crate::table::{FieldConstraint, FieldDescriptor, FieldKind, ParseContext};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct ItemKindTotalNumDataTable {
    entries: Vec<ItemKindTotalNumData>,
}
impl ItemKindTotalNumDataTable {
    const ENTRY_SIZE: usize = 0x8;
    pub(crate) fn parse(context: &ParseContext<'_>, root: usize) -> Result<Self> {
        let header = context.table_offset("dispositionDataHeader")?;
        let count = usize::from(context.read_u8(header + 4)?);
        let mut entries = Vec::with_capacity(count);
        for index in 0..count {
            let mut de = EntryDeserializer::new(context, root + index * Self::ENTRY_SIZE);
            entries.push(ItemKindTotalNumData::read(&mut de)?);
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
        ItemKindTotalNumData::FIELDS
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
struct ItemKindTotalNumData {
    item_kind: Value,
    amount: Value,
}
impl ItemKindTotalNumData {
    const FIELDS: &'static [FieldDescriptor] = &[
        FieldDescriptor {
            name: "item_kind",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
        FieldDescriptor {
            name: "amount",
            description: "",
            kind: FieldKind::Integer,
            constraint: FieldConstraint::None,
        },
    ];
    fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        Ok(Self {
            item_kind: Value::Integer(de.read_u32()?),
            amount: Value::Integer(de.read_u32()?),
        })
    }
    fn write<'a>(&'a self, ser: &mut EntrySerializer<'a>) -> Result<()> {
        ser.write_u32(self.item_kind.as_integer()?);
        ser.write_u32(self.amount.as_integer()?);
        Ok(())
    }
    fn get(&self, field: &str) -> Option<Value> {
        Some(match field {
            "item_kind" => self.item_kind.clone(),
            "amount" => self.amount.clone(),
            _ => return None,
        })
    }
    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "item_kind" => self.item_kind = value,
            "amount" => self.amount = value,
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }
}
