use egui::Ui;

// Window-related Constants
pub const WINDOW_SIZE: [f32; 2] = [640.0, 480.0];
pub const WINDOW_TITLE: &str = "MarkBrowser";

// Application struct boilerplate
#[derive(Default)]
pub struct MarkBrowserApp {}
impl eframe::App for MarkBrowserApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, ui_draw);
    }
}

// Actual Rendering Code
fn ui_draw(ui: &mut Ui) {
    ui.heading("Hello World!");
}
