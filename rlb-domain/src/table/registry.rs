use crate::macros::declare_tables;
use crate::table::terminated_list::TerminatedList;

declare_tables! {

    ScriptList {
        body: TerminatedList<
            crate::entry_schemas::script_list::ScriptListEntry,
            crate::entry_schemas::script_list::ScriptListEntry,
        >,

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
        body: TerminatedList<crate::entry_schemas::fsb_file_list::FsbFileListData,crate::entry_schemas::fsb_file_list::FsbFileListData,>,

        tables: [
            "FsbFileListData",
        ]
    }

    WanderingData {
        body: TerminatedList<
            crate::entry_schemas::wandering_data::WanderingDataTable,
            crate::entry_schemas::wandering_data::WanderingDataTable,
        >,

        tables: [
            "WanderingDataTable",
        ]
    }

}
