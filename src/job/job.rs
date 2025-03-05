use std::ffi::OsStr;
use std::path::PathBuf;
use std::{env, process};

/// Function to create a new job. It spawns a new process with specific environment variables to execute the job.
pub fn create_job() {
    let home_dir = env::var("HOME").expect("Unable to load the home directory");
    let path_dir = PathBuf::from(home_dir.clone())
        .join("Applications")
        .join("eb-rs.app")
        .join("Contents")
        .join("MacOS")
        .join("eb-rs");
    let executable_path = match std::env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(_) => path_dir.clone().to_str().unwrap().into(),
    };

    let _ = process::Command::new(executable_path)
        .current_dir(home_dir.clone())
        .env("INSIDE_JOB", "TRUE")
        .spawn()
        .expect("Unexpected error when creating background job.");
}

/// Function to kill an existing job.
pub fn kill_job(this: bool) {
    let this_pid = sysinfo::get_current_pid().unwrap();
    let s = sysinfo::System::new_all();
    let process_name = OsStr::new("eb-rs");

    if this {
        process::exit(1);
    }

    for process in s.processes_by_exact_name(&process_name) {
        let p_pid = process.pid();

        if p_pid != this_pid {
            process.kill();
        }
    }
}
