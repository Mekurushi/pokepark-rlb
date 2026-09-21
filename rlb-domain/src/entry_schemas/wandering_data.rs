use crate::TableEntry;
use crate::entry_schemas::codec::{EntryDeserializer, EntrySerializer};
use crate::entry_schemas::{FieldConstraint, FieldKind, Terminator};
use crate::string_pool::StringPool;
use crate::util::checked_bool;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct WanderingDataTable {
    entries: Vec<WanderingDataEntry>,
    terminator: WanderingDataEntry,
}

impl WanderingDataTable {
    const ENTRY_SIZE: usize = <WanderingDataEntry as TableEntry>::SIZE;

    pub(crate) fn discover<R, E>(
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
                        context: "parsing WanderingData record",
                    })?;
            let mut de = EntryDeserializer::new(bytes, offset, resolve_string, is_relocated);

            if let Some(terminator) = <WanderingDataEntry as Terminator>::recognize(&mut de)? {
                return Ok(Self {
                    entries,
                    terminator,
                });
            }

            let mut de = EntryDeserializer::new(bytes, offset, resolve_string, is_relocated);
            entries.push(WanderingDataEntry::read(&mut de)?);
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
            <WanderingDataEntry as TableEntry>::write(entry, &mut serializer)?;
            serializer.finish(out, Self::ENTRY_SIZE)?;
        }

        let mut serializer = EntrySerializer::new(
            base_offset + self.entries.len() * Self::ENTRY_SIZE,
            strings,
            relocations,
        );
        <WanderingDataEntry as Terminator>::write(&self.terminator, &mut serializer)?;
        serializer.finish(out, <WanderingDataEntry as Terminator>::SIZE)
    }

    pub(crate) fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        for entry in &self.entries {
            <WanderingDataEntry as TableEntry>::visit_strings(entry, visit)?;
        }
        <WanderingDataEntry as Terminator>::visit_strings(&self.terminator, visit)
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
}

#[derive(Clone, Debug)]
pub struct WanderingDataEntry {
    pokemon_unlock_id: Value,
    pokemon_friendship_id: Value,
    enabled: Value,
    pad: [u8; 3],
}

impl TableEntry for WanderingDataEntry {
    const SIZE: usize = 0xC;
    const FIELDS: &'static [FieldDescriptor] = WANDERING_DATA_FIELDS;

    fn get(&self, field: &str) -> Option<Value> {
        match field {
            "pokemon_unlock_id" => Some(self.pokemon_unlock_id.clone()),
            "pokemon_friendship_id" => Some(self.pokemon_friendship_id.clone()),
            "enabled" => Some(self.enabled.clone()),
            _ => None,
        }
    }

    fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "pokemon_unlock_id" => self.pokemon_unlock_id = value,
            "pokemon_friendship_id" => self.pokemon_friendship_id = value,
            "enabled" => self.enabled = value,
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
            pokemon_unlock_id: Value::Integer(de.read_u32()?),
            pokemon_friendship_id: Value::Integer(de.read_u32()?),
            enabled: Value::Boolean(checked_bool(de.read_u8()?, "enabled")?),
            pad: de.read_pad()?,
        })
    }
    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        ser.write_u32(self.pokemon_unlock_id.as_integer()?);
        ser.write_u32(self.pokemon_friendship_id.as_integer()?);
        ser.write_u8(u8::from(self.enabled.as_bool()?));
        ser.write_pad(&self.pad);
        Ok(())
    }

    fn visit_strings(&self, _visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        Ok(())
    }
}

impl Terminator for WanderingDataEntry {
    const SIZE: usize = <Self as TableEntry>::SIZE;

    fn recognize<R, E>(de: &mut EntryDeserializer<'_, R, E>) -> Result<Option<Self>>
    where
        R: FnMut(u32) -> Result<String>,
        E: FnMut(u32) -> bool,
    {
        let candidate = <Self as TableEntry>::read(de)?;
        let is_terminator = candidate.pokemon_unlock_id == Value::Integer(0xFFFF_FFFF)
            && candidate.pokemon_friendship_id == Value::Integer(0xFFFF_FFFF)
            && candidate.enabled == Value::Boolean(false);

        Ok(is_terminator.then_some(candidate))
    }

    fn write(&self, ser: &mut EntrySerializer<'_>) -> Result<()> {
        <Self as TableEntry>::write(self, ser)
    }

    fn visit_strings(&self, visit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        <Self as TableEntry>::visit_strings(self, visit)
    }
}

pub const WANDERING_DATA_FIELDS: &[FieldDescriptor] = &[
    FieldDescriptor {
        name: "pokemon_unlock_id",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "pokemon_friendship_id",
        description: "",
        kind: FieldKind::Integer,
        constraint: FieldConstraint::None,
    },
    FieldDescriptor {
        name: "enabled",
        description: "",
        kind: FieldKind::Boolean,
        constraint: FieldConstraint::None,
    },
];
