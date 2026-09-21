use crate::macros::declare_tables;

declare_tables! {

    ScriptList {
        body: crate::entry_schemas::script_list::ScriptListTable,

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
        body: crate::entry_schemas::fsb_file_list::FsbFileListTable,

        tables: [
            "FsbFileListData",
        ]
    }

    WanderingData {
        body: crate::entry_schemas::wandering_data::WanderingDataTable,

        tables: [
            "WanderingDataTable",
        ]
    }

}
