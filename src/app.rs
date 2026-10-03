use std::path::Path;

use egui::Ui;
use markdown::mdast::Node;

use crate::renderer::parse;

// Window-related Constants
pub const WINDOW_SIZE: [f32; 2] = [640.0, 480.0];
pub const WINDOW_TITLE: &str = "MarkBrowser";

// Application struct boilerplate
pub struct MarkBrowserApp {
    current_path: String,
    ast: Option<Node>,
    error: Option<String>,
}

impl Default for MarkBrowserApp {
    fn default() -> Self {
        let current_path = "README.md".to_string();
        let ast = parse(Path::new(&current_path)).ok();

        return Self {
            current_path,
            ast,
            error: None,
        };
    }
}

impl eframe::App for MarkBrowserApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, ui_draw);
    }
}

// Actual Rendering Code
fn ui_draw(ui: &mut Ui) {
    ui.heading("Hello World!");
}
