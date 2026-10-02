#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(app::WINDOW_SIZE),
        ..Default::default()
    };

    return eframe::run_native(
        app::WINDOW_TITLE,
        options,
        Box::new(|_cc| Ok(Box::<app::MarkBrowserApp>::default())),
    );
}
