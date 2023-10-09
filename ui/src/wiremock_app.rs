use std::process::Child;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use eframe::{egui, Frame};
use eframe::egui::Context;

use wiremock::Wiremock;

use crate::scenario::Scenario;

const PADDING: f32 = 30.0;

pub struct WiremockApp {
    pub tx: Sender<u32>,
    pub rx: Receiver<u32>,
    pub mock_path: Option<String>,
    pub output: Vec<String>,
    pub scenarios: Vec<Scenario>,
    pub wiremock_process: Option<Child>,
    pub wiremock_output: Option<JoinHandle<String>>,
}

impl eframe::App for WiremockApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Main window
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.mock_path.is_none() {
                // Setup screen
                self.render_setup(ui);
            } else {
                // Main screen
                self.render_main(ui);
            }
        });
    }
}

impl WiremockApp {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            tx,
            rx,
            // TODO set default to `None`
            mock_path: Some("".to_string()),
            output: vec![],
            wiremock_process: None,
            wiremock_output: None,
            // TODO: set default to `None`
            scenarios: vec![Scenario {
                name: "test".to_string(),
                selected: "Updated".to_string(),
                states: vec!["Started".to_string(), "Updated".to_string()],
            }],
        }
    }

    fn render_setup(&mut self, ui: &mut egui::Ui) {
        ui.label("Select a folder containing the mappings and files.");
        if ui.button("Open folder…").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                self.mock_path = Some(path.display().to_string());
            }
        }
    }

    fn render_main(&mut self, ui: &mut egui::Ui) {
        // top
        self.render_top_panel(ui);

        // left
        self.render_left_panel(ui);

        // App screen
        self.render_main_panel(ui);
    }

    fn render_top_panel(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("top_panel").resizable(false).min_height(26.0).show_inside(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    let path = self.mock_path.clone().unwrap();
                    let path = path.as_str();
                    // file
                    ui.label("Folder:");
                    ui.monospace(path.to_string());

                    ui.add_space(PADDING);

                    // start/stop server
                    let button_text = match &self.wiremock_process {
                        Some(_) => "Stop server",
                        None => "Start server",
                    };

                    if ui.button(button_text).clicked() {
                        match &self.wiremock_process {
                            None => {
                                let (child, handle) = Wiremock::start_server(path.to_string(), 7070, self.tx.clone());
                                self.wiremock_process = child;
                                self.wiremock_output = Some(handle);
                            }
                            Some(_) => {
                                Wiremock::stop_server(
                                    self.wiremock_process.as_mut().unwrap(),
                                    self.wiremock_output.as_mut().unwrap(),
                                );
                                self.wiremock_process = None
                            }
                        }
                    }
                });
            });
        });
    }

    fn render_left_panel(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("left_panel").resizable(true).default_width(360.0).width_range(120.0..=480.0).show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Scenarios");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                // iterate through scenarios
                for scenario in &self.scenarios {
                    ui.horizontal(|ui| {
                        ui.label(&scenario.name.to_string());
                        egui::ComboBox::from_label("").selected_text(scenario.selected.clone()).show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(60.0);
                            ui.set_max_width(200.0);
                            for state in &scenario.states {
                                let _ = ui.selectable_label(
                                    scenario.selected.eq(state),
                                    state,
                                );
                                // TODO: handle selection
                            }
                        });
                    });
                }
            });
        });
    }

    fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // output
            egui::ScrollArea::vertical().stick_to_bottom(true).auto_shrink([false; 2]).show_viewport(ui, |ui, _viewport| {
                ui.label("Server is not running...".to_string());

                // loop {
                //     println!("{}", rx.recv().unwrap());
                //     thread::sleep(Duration::from_secs(1));
                // }
                // TODO: receive channel
                let received: Result<u32, TryRecvError> = self.rx.try_recv();
                if let Ok(f) = received {
                    println!("{}", f);
                    thread::sleep(Duration::from_secs(1));
                    // self.output.append(&mut vec![f])
                }

                let _ = self.output.iter().map(|f| {
                    ui.label(format!("Message {}", f));
                });
            });
        });
    }
}
