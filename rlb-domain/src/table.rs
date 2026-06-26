use crate::TableEntry;
use crate::rlb_file::StringId;
use crate::table_entry::entry_schemas::{
    BackFromAttractionScriptList, CheckObjectScriptList, EnterZoneScriptList, FsbFileListDataEntry,
};
use crate::table_view::TableView;
use rlb_error::Result;

#[derive(Debug, Clone)]
pub enum TableKind {
    BackFromAttractionScriptList(TableView<BackFromAttractionScriptList>),
    EnterZoneScriptList(TableView<EnterZoneScriptList>),
    FsbFileListData(TableView<FsbFileListDataEntry>),
    CheckObjectScriptList(TableView<CheckObjectScriptList>),
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
        } else {
            TableKind::Unknown
        };

        Ok(Self {
            name: name.to_owned(),
            kind,
        })
    }
}
