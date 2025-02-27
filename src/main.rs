use std::env;

use config::config::get_configuration;
use gui::config_gui::*;
use gui::gui::ExitStatus;
use gui::utils_gui::*;
use job::job::*;
use pattern_recognition::pattern_recognition::{PatternRecognition, RectanglePattern};

mod config;
mod gui;
mod io;
mod job;
mod pattern_recognition;

fn main() {
    let launch_job = env::var("LAUNCH_JOB").is_ok_and(|var| var == "TRUE");
    let inside_job = env::var("INSIDE_JOB").is_ok_and(|var| var == "TRUE");
    let configuration = get_configuration();

    if (launch_job || inside_job) && configuration.is_err() {
        // Display corruption gui
        start_corruption_gui();
        return;
    }

    if inside_job {
        let mut pt = PatternRecognition::<RectanglePattern>::new_rectangle_pattern();

        loop {
            if pt.recognize_pattern() {
                let exit_status = start_waning_gui();

                if exit_status == ExitStatus::COMPLETED {
                    // DO COPY
                }
            }
        }
    }

    if launch_job {
        kill_job();
        create_job();

        return;
    }

    let exit_status = start_config_gui();

    if exit_status == ExitStatus::COMPLETED {
        kill_job();
        create_job();
    }
}
