use eframe::egui;
use std::{cell::RefCell, rc::Rc};

use crate::config::config::create_config;
use crate::gui::gui::ExitStatus;
use crate::io::io::get_ext_devices;

/// App structure for egui's window implementation, contains three fields.
/// * exit_status: determine how the window has been closed.
/// * picked_paths: the array of picked paths associated with a bool to check if the path has been removed from the list.
/// * picked_device: the device picked from the list.
struct App {
    exit_status: Rc<RefCell<ExitStatus>>,
    picked_paths: Vec<((String, u64), bool)>,
    picked_device: Option<(String, u64)>,
}

impl App {
    /// App struct constructor.
    fn new(_cc: &eframe::CreationContext, exit_status: Rc<RefCell<ExitStatus>>) -> Self {
        App {
            exit_status,
            picked_paths: Vec::new(),
            picked_device: None,
        }
    }

    /// Function to render the gui, to be called inside the update function of the eframe::App trait.
    /// It renders two main components.
    /// * a file picker to choose a path, the list of choosen paths and a button to remove them.
    /// * a drop-down menu to choose between the external devices.
    fn show_config_window(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Choose up to five directories to save in case of emergency!");
            ui.label(format!(
                "Total size: {:?}",
                self.picked_paths.iter().map(|path| path.0 .1).sum::<u64>()
            ));

            if self.picked_paths.len() < 5 && ui.button("Open directory…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let path_name = path.to_str().unwrap().to_string();
                    let path_size = fs_extra::dir::get_size(path).unwrap();

                    if !self
                        .picked_paths
                        .contains(&((path_name.clone(), path_size), false))
                    {
                        self.picked_paths.push(((path_name, path_size), false));
                    }
                }
            }

            for i in 0..self.picked_paths.len() {
                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        self.picked_paths[i].1 = true;
                    }

                    ui.label(format!(
                        "{:?}, {:?}",
                        self.picked_paths[i].0 .0, self.picked_paths[i].0 .1
                    ));
                });
            }

            self.picked_paths.retain(|(_, flag)| !flag);

            ui.add_space(20.0);
            ui.heading("Choose an external device to use in case of emergency!");

            egui::ComboBox::new("select-menu", "").show_ui(ui, |ui| {
                for device in get_ext_devices() {
                    ui.selectable_value(
                        &mut self.picked_device,
                        Some(device.clone()),
                        format!("{:?}, {:?}", device.0, device.1),
                    );
                }
            });

            if let Some(picked_device) = &self.picked_device {
                ui.horizontal(|ui| {
                    ui.label("Picked external device:");
                    ui.monospace(format!("{:?}, {:?}", picked_device.0, picked_device.1));
                });
            }
        });

        egui::TopBottomPanel::bottom("bottom-panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                if ui.button("Start emergency backup!").clicked() {
                    if self.picked_paths.len() > 0 && self.picked_device.is_some() {
                        *self.exit_status.borrow_mut() = ExitStatus::COMPLETED;

                        let _ = create_config(
                            self.picked_device.take().unwrap().0,
                            self.picked_paths
                                .clone()
                                .into_iter()
                                .map(|path| path.0 .0)
                                .collect(),
                        );

                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if *self.exit_status.borrow() == ExitStatus::UNCOMPLETED {
            self.show_config_window(ctx, frame);
        }
    }
}

/// Function to start the configuration window, the caller waits until the window is closed.
/// It returns the exit status.
pub fn start_config_window() -> ExitStatus {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_resizable(false)
            .with_inner_size([840.0, 240.0])
            .with_maximize_button(false)
            .with_drag_and_drop(false),
        vsync: false,
        run_and_return: true,
        ..Default::default()
    };

    let exit_status = Rc::new(RefCell::new(ExitStatus::default()));

    let _ = eframe::run_native(
        "emergency-backup-rs",
        options,
        Box::new(|_cc| Ok(Box::new(App::new(_cc, exit_status.clone())))),
    );

    exit_status.take()
}
