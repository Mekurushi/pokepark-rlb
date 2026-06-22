use crate::FieldDescriptor;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScriptListEntry {
    pub name: u32,
    pub object_id: i32,
    pub minimum_chapter: i32,
    pub medium_chapter: i32,
    pub maximum_chapter: i32,
    pub flagname: u32,
    pub flag_value_condition: i32,
    pub target_script: u8,
    pub pad_0x1d: [u8; 3],
    pub unknown: i32,
    pub entrypoint: u32,
    pub zone_id: i32,
    pub area_id: i32,
    pub position_id: i32,
    pub pad_0x34: i32,
    pub after_script_entrypoint: u32,
    pub animation_ptr: u32,
    pub flagname2_ptr: u32,
}

#[derive(Debug)]
pub struct BackFromAttractionScriptList(pub ScriptListEntry);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SinglePointerEntry {
    pub script_name: u32,
}

#[derive(Debug)]
pub struct FsbFileListDataEntry(pub SinglePointerEntry);

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
