use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use eframe::egui;

use crate::gui::gui::ExitStatus;

/// Set a 15 seconds timer
const WARNING_WINDOW_DURATION: u64 = 15;

/// App structure for egui's window implementation, contains three fields.
/// * exit_status: determine how the window has been closed.
/// * time_left: the **Instant** object created at the creation time of the window.
struct App {
    exit_status: Rc<RefCell<ExitStatus>>,
    time_left: Instant,
}

impl App {
    /// App struct constructor.
    fn new(_cc: &eframe::CreationContext, exit_status: Rc<RefCell<ExitStatus>>) -> Self {
        let time_left = Instant::now();

        App {
            exit_status,
            time_left,
        }
    }

    /// Function to render the gui, to be called inside the update function of the eframe::App trait.
    /// It renders the time left before the window close itself, the time left is computed subtracting the **Instant** created at creation time and the current **Instant**.
    fn show_warning_window(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let time_left = Instant::now().duration_since(self.time_left.clone());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Warning!");
            ui.label("The backup process has started, if you want to cancel it press the button, otherwise wait for the backup to start!");
            ui.label(format!("The window will close automatically in {:?} seconds.", WARNING_WINDOW_DURATION - time_left.as_secs()));
            ctx.request_repaint_after(Duration::from_millis(50));
        });

        if time_left >= Duration::from_secs(WARNING_WINDOW_DURATION) {
            *self.exit_status.borrow_mut() = ExitStatus::UNCOMPLETED;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::TopBottomPanel::bottom("bottom-panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                if ui.button("Cancel").clicked() {
                    *self.exit_status.borrow_mut() = ExitStatus::COMPLETED;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if *self.exit_status.borrow() == ExitStatus::UNCOMPLETED {
            self.show_warning_window(ctx, frame);
        }
    }
}

/// Function to start the warning window, the caller waits until the window is closed.
/// It returns the exit status.
pub fn start_warning_window() -> ExitStatus {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_resizable(false)
            .with_inner_size([640.0, 240.0])
            .with_drag_and_drop(false)
            .with_close_button(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_always_on_top(),
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
