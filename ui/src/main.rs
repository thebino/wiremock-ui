use std::sync::mpsc;
use std::sync::mpsc::{RecvError, Sender};

use eframe::egui::{Context, TextBuffer};
use eframe::{egui, run_native, App, Frame, NativeOptions};

use wiremock::Wiremock;

const PADDING: f32 = 30.0;

#[derive(Default)]
struct WiremockApp {
    mock_path: Option<String>,
    output: Vec<String>,
    server_handle: Option<String>,
}

impl WiremockApp {
    fn new() -> Self {
        Self {
            mock_path: None,
            output: vec![],
            server_handle: None,
        }
    }

    fn render_top_panel(&mut self, ui: &mut egui::Ui, tx: Sender<String>) {
        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .min_height(26.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.horizontal(|ui| {
                        let path = self.mock_path.clone().unwrap();
                        let path = path.as_str();
                        // file
                        ui.label("Folder:");
                        ui.monospace(path.to_string());

                        ui.add_space(PADDING);

                        // start/stop server
                        let button_text = match &self.server_handle {
                            Some(_x) => "Stop server",
                            None => "Start server",
                        };

                        if ui.button(button_text).clicked() {
                            match &self.server_handle {
                                None => {
                                    Wiremock::start_server(path.to_string(), 7070, tx).unwrap();
                                    self.server_handle = Some("".to_string())
                                }
                                Some(_) => {
                                    // kill process
                                    //&self.server_handle.as_mut().unwrap().kill();
                                    self.server_handle = None
                                }
                            }
                        }
                    });
                });
            });
    }
}

#[derive(Debug, PartialEq)]
enum Enum {
    Started,
    ServicesTermsAndConditionsNotAccepted,
}

impl App for WiremockApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let (tx, rx) = mpsc::channel();

        // Main window
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.mock_path.is_some() {
                // top
                self.render_top_panel(ui, tx);

                // left
                egui::SidePanel::left("left_panel")
                    .resizable(true)
                    .default_width(360.0)
                    .width_range(120.0..=480.0)
                    .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Scenarios");
                        });
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // TODO: add each scenario
                            // text
                            // vec[option]

                            let mut radio: Enum = Enum::ServicesTermsAndConditionsNotAccepted;
                            ui.horizontal(|ui| {
                                ui.label("Scenario 1");
                                egui::ComboBox::from_label("")
                                    .selected_text(format!("{radio:?}"))
                                    .show_ui(ui, |ui| {
                                        ui.style_mut().wrap = Some(false);
                                        ui.set_min_width(60.0);
                                        ui.set_max_width(200.0);
                                        ui.selectable_value(&mut radio, Enum::Started, "Started");
                                        ui.selectable_value(
                                            &mut radio,
                                            Enum::ServicesTermsAndConditionsNotAccepted,
                                            format!(
                                                "{:?}",
                                                Enum::ServicesTermsAndConditionsNotAccepted
                                            ),
                                        );
                                    });
                            });
                        });
                    });

                // App screen
                ui.vertical(|ui| {
                    // output
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .auto_shrink([false; 2])
                        .show_viewport(ui, |ui, _viewport| {
                            ui.label("Server is not running...".to_string());

                            // TODO: receive channel
                            let received: Result<String, RecvError> = rx.recv();
                            if let Ok(f) = received {
                                self.output.append(&mut vec![f])
                            }

                            let _ = self.output.iter().map(|f| {
                                ui.label(format!("Message {}", f));
                            });
                        });
                });
            } else {
                // Setup screen
                ui.label("Select a folder containing the mappings and files.");
                if ui.button("Open folder…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.mock_path = Some(path.display().to_string());
                    }
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let options = NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(960.0, 480.0)),
        min_window_size: Some(egui::vec2(600.0, 360.0)),
        ..Default::default()
    };

    let ui = WiremockApp::new();

    run_native(
        "Wiremock Ui".as_str(),
        options,
        Box::new(|_cc| Box::new(ui)),
    )
    .unwrap();
}
