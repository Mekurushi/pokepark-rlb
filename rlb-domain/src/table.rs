use crate::TableEntry;
use crate::entry_schemas::script_list::{BackFromAttractionScriptList, CheckObjectScriptList, EnterZoneScriptList, HitThunderboltScriptList, ReplaceScriptList, TimeOutScriptList, TouchAreaScriptList};
use crate::entry_schemas::single_pointer::fsb_file_list_data::FsbFileListDataEntry;
use crate::rlb_file::StringId;
use crate::table_view::TableView;
use rlb_error::Result;

#[derive(Debug, Clone)]
pub enum TableKind {
    BackFromAttractionScriptList(TableView<BackFromAttractionScriptList>),
    EnterZoneScriptList(TableView<EnterZoneScriptList>),
    FsbFileListData(TableView<FsbFileListDataEntry>),
    CheckObjectScriptList(TableView<CheckObjectScriptList>),
    HitThunderboltScriptList(TableView<HitThunderboltScriptList>),
    ReplaceScriptList(TableView<ReplaceScriptList>),
    TimeOutScriptList(TableView<TimeOutScriptList>),
    TouchAreaScriptList(TableView<TouchAreaScriptList>),
    Unknown,
}
#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    kind: TableKind,
}

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
        let kind = if name == BackFromAttractionScriptList::type_name() {
            TableKind::BackFromAttractionScriptList(TableView::discover(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)
        } else if name == FsbFileListDataEntry::type_name() {
            TableKind::FsbFileListData(TableView::discover(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)
        } else if name == EnterZoneScriptList::type_name() {
            TableKind::EnterZoneScriptList(TableView::discover(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)
        } else if name == CheckObjectScriptList::type_name() {
            TableKind::CheckObjectScriptList(TableView::discover(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)
        } else if name == HitThunderboltScriptList::type_name() {
            TableKind::HitThunderboltScriptList(TableView::discover(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)
        } else if name == ReplaceScriptList::type_name(){
            TableKind::ReplaceScriptList(TableView::discover(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)
        } else if name == TimeOutScriptList::type_name(){
            TableKind::TimeOutScriptList(TableView::discover(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)
        } else if name == TouchAreaScriptList::type_name(){
            TableKind::TouchAreaScriptList(TableView::discover(
                data,
                offset,
                resolve_string,
                is_relocated,
            )?)
        } else {
            TableKind::Unknown
        };

        Ok(Self {
            name: name.to_owned(),
            kind,
        })
    }
}
