use std::env;

use config::config::get_configuration;
use gui::config_window::*;
use gui::gui::ExitStatus;
use gui::warning_window::*;
use job::job::*;

mod config;
mod gui;
mod io;
mod job;

fn main() {
    let launch_job = env::var("LAUNCH_JOB").is_ok_and(|var| var == "TRUE");
    let inside_job = env::var("INSIDE_JOB").is_ok_and(|var| var == "TRUE");
    let configuration = get_configuration();

    if (launch_job || inside_job) && configuration.is_err() {
        // Display corruption gui

        return;
    }

    while inside_job {
        start_job();

        let exit_status = start_warning_window();

        if exit_status == ExitStatus::COMPLETED {
            // DO COPY
        }
    }

    if launch_job {
        kill_job();
        create_job();

        return;
    }

    let exit_status = start_config_window();

    if exit_status == ExitStatus::COMPLETED {
        kill_job();
        create_job();
    }
}
