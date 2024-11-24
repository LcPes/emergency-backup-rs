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
    let get_config = get_config();

    if env::var("JOB_STARTED").is_ok_and(|var| var.eq("TRUE")) {
        start_job();
        start_warning_window();
        return;
    }

    if exists_job || (!exists_job && !get_config.is_ok()) {
        kill_job();
        let exit_status = start_config_window();

        if exit_status == ExitStatus::COMPLETED {
            create_job();
        }
    } else if !exists_job && get_config.is_ok() {
        create_job();
    }
}
