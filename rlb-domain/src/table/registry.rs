use crate::entry_schemas::TableEntry;
use crate::macros::declare_tables;
use crate::rlb_file::StringId;
use crate::table::entry_list::EntryList;
use crate::FieldDescriptor;
use crate::Value;
use rlb_error::Result;

declare_tables! {

    ScriptList {
        entry: crate::entry_schemas::script_list::ScriptListEntry,

        tables: [
            "BackFromAttractionScriptList",
            "ReplaceScriptList",
            "CheckObjectScriptList",
            "EnterZoneScriptList",
            "HitDashScriptList",
            "HitThunderboltScriptList",
            "TimeOutScriptList",
            "TouchAreaScriptList",
        ]
    }


    SinglePointer {
        entry: crate::entry_schemas::fsb_file_list::FsbFileListData,

        tables: [
            "FsbFileListData",
        ]
    }
    WanderingData {
        entry: crate::entry_schemas::wandering_data::WanderingDataTable,

        tables: [
            "WanderingDataTable",
        ]
    }

}
