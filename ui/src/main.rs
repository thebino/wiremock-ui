use eframe::{egui, NativeOptions};
use eframe::egui::TextBuffer;

use ui::wiremock_app::WiremockApp;

fn main() {
    let options = NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(960.0, 480.0)),
        min_window_size: Some(egui::vec2(600.0, 360.0)),
        ..Default::default()
    };

    let app = WiremockApp::new();

    // GUI on the main thread
    eframe::run_native(
        "Wiremock Ui".as_str(),
        options,
        Box::new(|_cc| Box::new(app)),
    ).unwrap();
}
