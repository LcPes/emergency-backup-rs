use std::env;

use config::config::get_configuration;
use gui::config_gui::*;
use gui::gui::ExitStatus;
use gui::utils_gui::*;
use io::io::execute_copy;
use job::job::*;
use pattern_recognition::pattern_recognition::{PatternRecognition, RectanglePattern};
use utils::setup_autolaunch;

mod config;
mod gui;
mod io;
mod job;
mod pattern_recognition;
mod utils;

fn main() {
    let launch_job = env::var("LAUNCH_JOB").is_ok_and(|var| var == "TRUE");
    let inside_job = env::var("INSIDE_JOB").is_ok_and(|var| var == "TRUE");
    let configuration = get_configuration();

    if inside_job {
        let mut pt = PatternRecognition::<RectanglePattern>::new_rectangle_pattern();
        utils::create_cpu_logger();
        utils::start_cpu_tracker();

        loop {
            if pt.recognize_pattern() {
                let exit_status = start_warning_gui();

                if exit_status == ExitStatus::COMPLETED {
                    let device_name = configuration.clone().unwrap().get_device_name();
                    let path_names = configuration.clone().unwrap().get_path_names();

                    execute_copy(device_name, path_names);
                }

                create_job();
                kill_job(true);
            }
        }
    }

    if launch_job {
        kill_job(false);
        create_job();

        return;
    }

    let exit_status = start_config_gui();
    println!("{}", std::env::current_dir().unwrap().to_str().unwrap());

    if exit_status == ExitStatus::COMPLETED {
        setup_autolaunch();
        kill_job(false);
        create_job();
    }
}
