mod app;
mod io;
mod state;
mod widgets;

use app::RlbUiApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "RLB Editor",
        options,
        Box::new(|_cc| Ok(Box::new(RlbUiApp::default()))),
    )
}
