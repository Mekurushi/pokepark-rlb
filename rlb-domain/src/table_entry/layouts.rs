use crate::rlb_file::StringId;
use crate::{FieldDescriptor, Value};
use rlb_error::{Error, Result};
use rlb_format::RelocationTable;
use crate::util::checked_u32;

pub(super) fn read_u32(data: &[u8], offset: usize) -> Result<u32> {
    data.get(offset..offset + 4)
        .and_then(|b| b.try_into().ok())
        .map(u32::from_be_bytes)
        .ok_or(Error::OffsetOutOfBounds {
            context: "entry field",
            offset,
            length: data.len(),
        })
}

pub(super) fn read_u8(data: &[u8], offset: usize) -> Result<u8> {
    data.get(offset).copied().ok_or(Error::OffsetOutOfBounds {
        context: "entry field",
        offset,
        length: data.len(),
    })
}

pub(super) fn read_bytes<const N: usize>(data: &[u8], offset: usize) -> Result<[u8; N]> {
    data.get(offset..offset + N)
        .and_then(|b| b.try_into().ok())
        .ok_or(Error::OffsetOutOfBounds {
            context: "entry field",
            offset,
            length: data.len(),
        })
}

pub(super) fn value_at<R>(
    data: &[u8],
    field_offset: usize,
    base_offset: usize,
    resolve_string: &mut R,
    relocations: &RelocationTable,
) -> Result<Value>
where
    R: FnMut(u32) -> Result<StringId>,
{
    let raw = read_u32(data, field_offset)?;
    let abs_offset = checked_u32(base_offset + field_offset, "calculating field offset")?;
    if relocations.is_relocated(abs_offset) {
        Ok(Value::Pointer(resolve_string(raw)?))
    } else {
        Ok(Value::Integer(raw))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScriptListEntry {
    pub name: Value,
    pub object_id: u32,
    pub minimum_chapter: u32,
    pub medium_chapter: u32,
    pub maximum_chapter: u32,
    pub flagname: Value,
    pub flag_value_condition: u32,
    pub target_script: u8,
    pub pad_0x1d: [u8; 3],
    pub unknown: u32,
    pub entrypoint: Value,
    pub zone_id: u32,
    pub area_id: u32,
    pub position_id: u32,
    pub pad_0x34: u32,
    pub after_script_entrypoint: Value,
    pub animation: Value,
    pub flagname2: Value,
}

impl ScriptListEntry {
    pub fn read<R>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        relocations: &RelocationTable,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
    {
        Ok(Self {
            name: value_at(data, 0x00, base_offset, resolve_string, relocations)?,
            object_id: read_u32(data, 0x04)?,
            minimum_chapter: read_u32(data, 0x08)?,
            medium_chapter: read_u32(data, 0x0C)?,
            maximum_chapter: read_u32(data, 0x10)?,
            flagname: value_at(data, 0x14, base_offset, resolve_string, relocations)?,
            flag_value_condition: read_u32(data, 0x18)?,
            target_script: read_u8(data, 0x1C)?,
            pad_0x1d: read_bytes(data, 0x1D)?,
            unknown: read_u32(data, 0x20)?,
            entrypoint: value_at(data, 0x24, base_offset, resolve_string, relocations)?,
            zone_id: read_u32(data, 0x28)?,
            area_id: read_u32(data, 0x2C)?,
            position_id: read_u32(data, 0x30)?,
            pad_0x34: read_u32(data, 0x34)?,
            after_script_entrypoint: value_at(
                data,
                0x38,
                base_offset,
                resolve_string,
                relocations,
            )?,
            animation: value_at(data, 0x3C, base_offset, resolve_string, relocations)?,
            flagname2: value_at(data, 0x40, base_offset, resolve_string, relocations)?,
        })
    }
}



#[derive(Clone, Copy, Debug)]
pub struct SinglePointerEntry {
    pub script_name: Value,
}

impl SinglePointerEntry {
    pub fn read<R>(
        data: &[u8],
        base_offset: usize,
        resolve_string: &mut R,
        relocations: &RelocationTable,
    ) -> Result<Self>
    where
        R: FnMut(u32) -> Result<StringId>,
    {
        Ok(Self {
            script_name: value_at(data, 0x00, base_offset, resolve_string, relocations)?
        })
    }
}


pub const SCRIPT_LIST_FIELDS: &[FieldDescriptor] = &[
    FieldDescriptor { name: "name" },
    FieldDescriptor { name: "object_id" },
    FieldDescriptor {
        name: "minimum_chapter",
    },
    FieldDescriptor {
        name: "medium_chapter",
    },
    FieldDescriptor {
        name: "maximum_chapter",
    },
    FieldDescriptor { name: "flagname" },
    FieldDescriptor {
        name: "flag_value_condition",
    },
    FieldDescriptor {
        name: "target_script",
    },
    FieldDescriptor { name: "unknown" },
    FieldDescriptor { name: "entrypoint" },
    FieldDescriptor { name: "zone_id" },
    FieldDescriptor { name: "area_id" },
    FieldDescriptor {
        name: "position_id",
    },
    FieldDescriptor { name: "pad_0x34" },
    FieldDescriptor {
        name: "after_script_entrypoint",
    },
    FieldDescriptor { name: "animation" },
    FieldDescriptor { name: "flagname2" },
];

pub const FSB_FILE_LIST_FIELDS: &[FieldDescriptor] = &[FieldDescriptor {
    name: "script_name",
}];
