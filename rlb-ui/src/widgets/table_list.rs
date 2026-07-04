use crate::state::{AppState, LoadedFile};
use eframe::egui;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
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
        for table in file.tables() {
            let selected = *selected_table == Some(table.id);
            let text = format!("{}  ({})", table.label, table.entry_count);
            if ui.selectable_label(selected, text).clicked() {
                *selected_table = Some(table.id);
            }
        }
    });
}
