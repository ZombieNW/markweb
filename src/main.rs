#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod renderer;

use std::path::Path;

use eframe::egui;

use crate::renderer::parse;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(app::WINDOW_SIZE),
        ..Default::default()
    };

    let tree = parse(Path::new("README.md"));
    println!("{:?}", tree);

    return eframe::run_native(
        app::WINDOW_TITLE,
        options,
        Box::new(|_cc| Ok(Box::<app::MarkBrowserApp>::default())),
    );
}
