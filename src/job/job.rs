use std::ffi::OsStr;

/// Function to create a new job. It spawns a new process with specific environment variables to execute the job.
pub fn create_job() {
    let _ = std::process::Command::new(std::env::current_exe().unwrap())
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
        s.process(this_pid).unwrap().kill();
    }

    for process in s.processes_by_exact_name(&process_name) {
        let p_pid = process.pid();

        if p_pid != this_pid {
            process.kill();
        }
    }
}
