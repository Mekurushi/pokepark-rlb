use crate::state::{AppState, LoadedFile};
use eframe::egui;

pub(crate) fn show(ui: &mut egui::Ui, state: &mut AppState) {
    let Some(loaded) = &mut state.loaded else {
        ui.weak("Open an .rlb file");
        return;
    };

    ui.heading("Tables");
    ui.separator();

    let LoadedFile {
        file,
        selected_table,
        ..
    } = loaded;

    egui::ScrollArea::vertical().show(ui, |ui| {
        let Ok(tables) = file.tables() else {
            ui.weak("Unable to read tables");
            return;
        };
        for table in tables {
            let selected = *selected_table == Some(table.id);
            let text = format!("{}  ({})", table.label, table.entry_count);
            if ui.selectable_label(selected, text).clicked() {
                *selected_table = Some(table.id);
            }
        }
    });
}
