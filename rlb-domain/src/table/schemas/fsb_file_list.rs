use crate::table::ParseContext;
use crate::table::codec::{EntryDeserializer, EntrySerializer};
use crate::table::field::{FieldConstraint, FieldKind};
use crate::table::serialization::RelocatableTable;
use crate::table::{RowBoundary, RowLayout, SchemaDescriptor, SchemaId};
use crate::{FieldDescriptor, Row, Value};
use rlb_error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct FsbFileListTable {
    entries: Vec<FsbFileListData>,
    terminator: FsbFileListData,
}

impl FsbFileListTable {
    pub(crate) fn create(rows: &[Row]) -> Result<Self> {
        let mut table = Self {
            entries: Vec::new(),
            terminator: FsbFileListData {
                script_name: Value::String(None),
            },
        };
        for row in rows {
            table.append_row(row)?;
        }
        Ok(table)
    }

    const ENTRY_SIZE: usize = FsbFileListData::SIZE;
    pub(crate) const SCHEMA: SchemaDescriptor = SchemaDescriptor {
        id: SchemaId::FsbFileList,
        description: "Script file names referenced by script-list tables",
        fields: FsbFileListData::FIELDS,
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
            let candidate = FsbFileListData::read(&mut de)?;

            if candidate.script_name == Value::String(None) {
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
        FsbFileListData::FIELDS
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

    pub(crate) fn append_row(&mut self, row: &Row) -> Result<usize> {
        let entry = FsbFileListData {
            script_name: row
                .get("script_name")
                .cloned()
                .ok_or_else(|| Error::Validation("missing field \"script_name\"".into()))?,
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
struct FsbFileListData {
    script_name: Value,
}

impl FsbFileListData {
    pub(crate) const SIZE: usize = 0x4;
    pub(crate) const FIELDS: &'static [FieldDescriptor] = FSB_FILE_LIST_FIELDS;

    pub(crate) fn get(&self, field: &str) -> Option<Value> {
        match field {
            "script_name" => Some(self.script_name.clone()),
            _ => None,
        }
    }

    pub(crate) fn set(&mut self, field: &str, value: Value) -> Result<()> {
        match field {
            "script_name" => {
                self.script_name = value;
            }
            _ => return Err(Error::Validation(format!("unknown field: '{field}'"))),
        }
        Ok(())
    }

    pub(crate) fn read(de: &mut EntryDeserializer<'_, '_>) -> Result<Self> {
        let script = de.read_string_pointer()?;

        Ok(Self {
            script_name: script,
        })
    }
    pub(crate) fn write<'a>(&'a self, ser: &mut EntrySerializer<'a>) -> Result<()> {
        ser.write_string_pointer(&self.script_name)?;
        Ok(())
    }
}

const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
    description: "",
    kind: FieldKind::String,
    constraint: FieldConstraint::None,
}];
