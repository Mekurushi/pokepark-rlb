use crate::macros::declare_tables;
use crate::rlb_file::StringId;
use crate::table_view::TableView;
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
        entry: crate::entry_schemas::single_pointer::SinglePointerEntry,

        tables: [
            "FsbFileListData",
        ]
    }

}
