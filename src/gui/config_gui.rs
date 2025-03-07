use crate::config::config::create_configuration;
use crate::gui::gui::ExitStatus;
use crate::io::io::*;
use byte_unit::Byte;
use eframe::egui;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

/// App structure for egui's window implementation, contains three fields.
/// * exit_status: determine how the window has been closed.
/// * picked_paths: the array of picked paths associated with a bool to check if the path has been removed from the list.
/// * picked_device: the device picked from the list.
struct App {
    exit_status: Rc<RefCell<ExitStatus>>,
    picked_paths: Vec<(Folder, bool)>,
    picked_device: Option<Device>,
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
    fn show_config_gui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Choose up to five directories to save in case of emergency!");
            ui.label(format!(
                "Total size: {}",
                Byte::from(
                    self.picked_paths
                        .iter()
                        .map(|path| path.0.get_size())
                        .sum::<u64>()
                )
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
            ));

            ui.add_space(2.0);

            if self.picked_paths.len() < 5 && ui.button("Open directory…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let path_name = path.to_str().unwrap().to_string();
                    let path_size = fs_extra::dir::get_size(path).unwrap();

                    if !self
                        .picked_paths
                        .contains(&(Folder::new(path_name.clone(), path_size), false))
                    {
                        self.picked_paths
                            .push((Folder::new(path_name, path_size), false));
                    }
                }
            }

            for i in 0..self.picked_paths.len() {
                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        self.picked_paths[i].1 = true;
                    }

                    ui.label(format!("{}", self.picked_paths[i].0));
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
                        format!("{}", device),
                    );
                }
            });

            ui.add_space(2.0);

            if let Some(picked_device) = &self.picked_device {
                ui.horizontal(|ui| {
                    ui.label(format!("{}", picked_device));
                });
            }
        });

        egui::TopBottomPanel::bottom("bottom-panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                if ui.button("Start emergency backup!").clicked() {
                    let total_size = self
                        .picked_paths
                        .iter()
                        .map(|path| path.0.get_size())
                        .sum::<u64>();

                    if self.picked_paths.len() > 0
                        && self
                            .picked_device
                            .clone()
                            .is_some_and(|val| val.get_size() > total_size)
                    {
                        *self.exit_status.borrow_mut() = ExitStatus::COMPLETED;

                        let _ = create_configuration(
                            self.picked_device.take().unwrap().get_name(),
                            self.picked_paths
                                .clone()
                                .into_iter()
                                .map(|path| path.0.get_path())
                                .collect(),
                        );

                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });

        ctx.request_repaint_after(Duration::from_millis(200));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if *self.exit_status.borrow() == ExitStatus::PROCESSING {
            self.show_config_gui(ctx, frame);
        }
    }
}

/// Function to start the configuration gui, the caller waits until the gui is closed.
/// It returns the exit status.
pub fn start_config_gui() -> ExitStatus {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_resizable(false)
            .with_inner_size([640.0, 360.0])
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
