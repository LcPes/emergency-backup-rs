use std::env;

use config::config::*;
use gui::config_window::*;
use gui::gui::ExitStatus;
use gui::warning_window::*;
use job::job::*;

mod config;
mod devices;
mod gui;
mod job;

fn main() {
    let exists_job = exists_job();
    let config = get_config();
    let job_started = env::var("JOB_STARTED").is_ok_and(|var| var == "TRUE");

    if job_started {
        start_job();
        let exit_status = start_warning_window();

        if exit_status == ExitStatus::COMPLETED {
            // DO COPY
        }

        return;
    }

    if exists_job || (!exists_job && !config.is_ok()) {
        kill_job();
        let exit_status = start_config_window();

        if exit_status == ExitStatus::COMPLETED {
            create_job();
        }
    } else if !exists_job && config.is_ok() {
        create_job();
    }
}
