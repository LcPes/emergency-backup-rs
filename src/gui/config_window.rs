use eframe::egui;
use std::{cell::RefCell, rc::Rc};

use crate::gui::gui::ExitStatus;
use crate::{create_config, devices::devices::get_ext_devices};

struct App {
    exit_status: Rc<RefCell<ExitStatus>>,
    picked_paths: Vec<(String, bool)>,
    picked_device: Option<String>,
}

impl App {
    fn new(_cc: &eframe::CreationContext, exit_status: Rc<RefCell<ExitStatus>>) -> Self {
        App {
            exit_status,
            picked_paths: Vec::new(),
            picked_device: None,
        }
    }

    fn show_config_window(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Choose up to five directories to save in case of emergency!");

            if self.picked_paths.len() < 5 && ui.button("Open directory…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let p = path.to_str().unwrap().to_string();
                    if !self.picked_paths.contains(&(p.clone(), false)) {
                        self.picked_paths.push((p, false));
                    }
                }
            }

            for i in 0..self.picked_paths.len() {
                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        self.picked_paths[i].1 = true;
                    }

                    ui.label(format!("{:?}", self.picked_paths[i].0));
                });
            }

            self.picked_paths.retain(|(_, flag)| !flag);

            ui.add_space(20.0);
            ui.heading("Choose an external device to use in case of emergency!");

            egui::ComboBox::new("select-menu", "").show_ui(ui, |ui| {
                for option in get_ext_devices() {
                    ui.selectable_value(
                        &mut self.picked_device,
                        Some(option.clone()),
                        option.to_string(),
                    );
                }
            });

            if let Some(picked_device) = &self.picked_device {
                ui.horizontal(|ui| {
                    ui.label("Picked external device:");
                    ui.monospace(picked_device);
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
                            self.picked_device.take().unwrap(),
                            self.picked_paths
                                .clone()
                                .into_iter()
                                .map(|path| path.0)
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

pub fn start_config_window() -> ExitStatus {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_resizable(false)
            .with_inner_size([640.0, 240.0])
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
