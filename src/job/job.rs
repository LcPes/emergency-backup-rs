use std::ffi::OsStr;

/// Function to start a job. It starts the job inside an already existing process. It has to be executed only when the environmental variable **JOB_STARTED** is set to "TRUE"
pub fn start_job() {
    println!("Inside job");
}

/// Function to create a new job. It spawns a new process with specific environment variables to execute the job.
pub fn create_job() {
    let _ = std::process::Command::new(std::env::current_exe().unwrap())
        .env("INSIDE_JOB", "TRUE")
        .spawn()
        .expect("Unexpected error when creating background job.");
}

/// Function to kill an existing job.
pub fn kill_job() {
    let this_pid = sysinfo::get_current_pid().unwrap();
    let s = sysinfo::System::new_all();
    let process_name = OsStr::new("eb-rs");

    for process in s.processes_by_exact_name(&process_name) {
        let p_pid = process.pid();

        if p_pid != this_pid {
            process.kill();
        }
    }
}
