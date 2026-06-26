use crate::TableEntry;
use crate::entry_schemas::script_list::HitDashScriptList;
use crate::entry_schemas::script_list::{
    BackFromAttractionScriptList, CheckObjectScriptList, EnterZoneScriptList,
    HitThunderboltScriptList, ReplaceScriptList, TimeOutScriptList, TouchAreaScriptList,
};
use crate::entry_schemas::single_pointer::FsbFileListDataEntry;
use crate::rlb_file::StringId;
use crate::table_view::TableView;
use rlb_error::Result;

#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    kind: TableKind,
}

macro_rules! table_types {
    ($m:ident) => {
        $m! {
            BackFromAttractionScriptList => BackFromAttractionScriptList,
            EnterZoneScriptList           => EnterZoneScriptList,
            FsbFileListDataEntry          => FsbFileListData,
            CheckObjectScriptList         => CheckObjectScriptList,
            HitDashScriptList      => HitDashScriptList,
            HitThunderboltScriptList      => HitThunderboltScriptList,
            ReplaceScriptList             => ReplaceScriptList,
            TimeOutScriptList             => TimeOutScriptList,
            TouchAreaScriptList           => TouchAreaScriptList,
        }
    };
}
macro_rules! make_table_kind {
    (
        $(
            $entry:ident => $variant:ident,
        )*
    ) => {
        #[derive(Debug, Clone)]
        pub enum TableKind {
            $(
                $variant(TableView<$entry>),
            )*
            Unknown,
        }
    };
}

table_types!(make_table_kind);

macro_rules! make_resolve {
    (
        $(
            $entry:ident => $variant:ident,
        )*
    ) => {
        impl Table {
            pub fn resolve<R, E>(
                name: &str,
                data: &[u8],
                offset: usize,
                resolve_string: &mut R,
                is_relocated: &mut E,
            ) -> Result<Self>
            where
                R: FnMut(u32) -> Result<StringId>,
                E: FnMut(u32) -> bool,
            {
                $(
                    if name == $entry::type_name() {
                        return Ok(Self {
                            name: name.to_owned(),
                            kind: TableKind::$variant(
                                TableView::discover(
                                    data,
                                    offset,
                                    resolve_string,
                                    is_relocated,
                                )?,
                            ),
                        });
                    }
                )*

                Ok(Self {
                    name: name.to_owned(),
                    kind: TableKind::Unknown,
                })
            }
        }
    };
}

table_types!(make_resolve);
