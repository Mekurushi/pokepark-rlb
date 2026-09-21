use crate::io;
use crate::state::{AppState, LoadedFile, PendingAction, Status};
use crate::widgets;
use eframe::egui;
use std::path::PathBuf;

#[derive(Default)]
pub(crate) struct RlbUiApp {
    state: AppState,
}

impl eframe::App for RlbUiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if ui.ctx().input(|i| i.viewport().close_requested())
            && self.state.loaded.as_ref().is_some_and(|l| l.dirty)
            && self.state.pending_action.is_none()
        {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.state.pending_action = Some(PendingAction::Exit);
        }

        self.show_pending_action_dialog(ui.ctx());

        egui::Panel::top("toolbar").show(ui, |ui| {
            self.show_toolbar(ui);
        });

        egui::Panel::bottom("status_bar").show(ui, |ui| {
            self.show_status_bar(ui);
        });

        egui::Panel::left("table_list")
            .resizable(true)
            .default_size(240.0)
            .show(ui, |ui| {
                widgets::table_list::show(ui, &mut self.state);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            widgets::table_editor::show(ui, &mut self.state);
        });
    }
}

impl RlbUiApp {
    fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Open…").clicked() {
                self.open_file();
            }

            let has_file = self.state.loaded.is_some();
            if ui
                .add_enabled(has_file, egui::Button::new("Save"))
                .clicked()
            {
                self.save_file();
            }
            if ui
                .add_enabled(has_file, egui::Button::new("Save As…"))
                .clicked()
            {
                self.save_file_as();
            }

            if let Some(loaded) = &self.state.loaded {
                ui.separator();
                ui.label(loaded.path.display().to_string());
                if loaded.dirty {
                    ui.colored_label(egui::Color32::from_rgb(230, 170, 40), "unsaved changes");
                }
            }
        });
    }

    fn show_status_bar(&self, ui: &mut egui::Ui) {
        match &self.state.status {
            Some(status) if status.is_error => {
                ui.colored_label(egui::Color32::from_rgb(210, 70, 70), &status.message);
            }
            Some(status) => {
                ui.label(&status.message);
            }
            None => {
                ui.weak("Ready");
            }
        }
    }

    fn open_file(&mut self) {
        let Some(path) = io::pick_open_path() else {
            return;
        };

        if self.state.loaded.as_ref().is_some_and(|l| l.dirty) {
            self.state.pending_action = Some(PendingAction::Open(path));
            return;
        }

        self.load_path(path);
    }
    fn load_path(&mut self, path: PathBuf) {
        match io::load_file(&path) {
            Ok(file) => {
                self.state.status = Some(Status::info(format!("Loaded {}", path.display())));
                self.state.loaded = Some(LoadedFile {
                    file,
                    path,
                    selected_table: None,
                    dirty: false,
                });
            }
            Err(message) => self.state.status = Some(Status::error(message)),
        }
    }

    fn save_file(&mut self) {
        let Some(loaded) = &mut self.state.loaded else {
            return;
        };

        match io::save_file(&loaded.file, &loaded.path) {
            Ok(()) => {
                loaded.dirty = false;
                self.state.status = Some(Status::info(format!("Saved {}", loaded.path.display())));
            }
            Err(message) => self.state.status = Some(Status::error(message)),
        }
    }

    fn save_file_as(&mut self) {
        let Some(loaded) = &mut self.state.loaded else {
            return;
        };

        let suggested_name = loaded
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("output.rlb");

        let Some(path) = io::pick_save_path(suggested_name) else {
            return;
        };

        match io::save_file(&loaded.file, &path) {
            Ok(()) => {
                loaded.path = path;
                loaded.dirty = false;
                self.state.status = Some(Status::info(format!("Saved {}", loaded.path.display())));
            }
            Err(message) => self.state.status = Some(Status::error(message)),
        }
    }

    fn show_pending_action_dialog(&mut self, ctx: &egui::Context) {
        let Some(action) = &self.state.pending_action else {
            return;
        };
        let message = match action {
            PendingAction::Open(_) => "Opening a new file will discard your unsaved changes",
            PendingAction::Exit => "Quitting will discard your unsaved changes",
        };

        egui::Window::new("Unsaved changes")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(message);
                ui.horizontal(|ui| {
                    if ui.button("Discard changes").clicked() {
                        self.resolve_pending_action(ctx);
                    }
                    if ui.button("Cancel").clicked() {
                        self.state.pending_action = None;
                    }
                });
            });
    }
    fn resolve_pending_action(&mut self, ctx: &egui::Context) {
        match self.state.pending_action.take() {
            Some(PendingAction::Open(path)) => self.load_path(path),
            Some(PendingAction::Exit) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            None => {}
        }
    }
}
