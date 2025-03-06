use auto_launch::AutoLaunchBuilder;
use log::info;
use simplelog::*;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use sysinfo::System;

/// Create a logger file and make it usable with the info! macro
pub fn create_cpu_logger() {
    let home_dir = env::var("HOME").expect("Unable to load the home directory");
    let path_dir = PathBuf::from(home_dir.clone())
        .join("Library")
        .join("Application Support")
        .join("eb-rs");
    let log_file_path = path_dir.join("process_cpu_usage.log");
    let log_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)
        .unwrap();

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        log_file,
    )])
    .unwrap();
}

/// Starts a new thread that tracks the CPU consumption of the eb-rs background process
pub fn start_cpu_tracker() {
    let _ = std::thread::spawn(|| {
        let mut sys = System::new_all();
        let interval = std::time::Duration::from_secs(300);

        std::thread::sleep(std::time::Duration::from_secs(5));

        loop {
            let this_pid = sysinfo::get_current_pid().unwrap();
            sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[this_pid]), true);
            let this_process = sys.process(this_pid).unwrap();
            let cpu_usage = this_process.cpu_usage();

            let message = format!("(PID: {}) CPU usage: ({:.3})%", this_pid, cpu_usage);

            info!("{}", message);

            std::thread::sleep(interval);
        }
    });
}

/// Setup autolaunch by creating a script with applescript and compiling to an .app used as launcher
pub fn setup_autolaunch() {
    let home_dir = env::var("HOME").expect("Unable to load the home directory");
    let path_dir = PathBuf::from(home_dir.clone())
        .join("Library")
        .join("Application Support")
        .join("eb-rs");
    let launcher_name = "eb-rs_launcher.app";
    let launcher_path = path_dir.join(launcher_name);

    let script = format!("do shell script \"HOME={} LAUNCH_JOB=TRUE /Users/$(whoami)/Applications/eb-rs.app/Contents/MacOS/eb-rs &\"", home_dir);
    let script_name = "autolaunch.scpt";
    let script_path = path_dir.join(script_name);

    let mut script_file = File::create(&script_path).unwrap();
    script_file.write(script.as_bytes()).unwrap();

    let mut create_launcher = Command::new("osacompile")
        .args(&[
            "-o",
            launcher_path.to_str().unwrap(),
            script_path.to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to create launcher");

    match create_launcher.wait() {
        Ok(_) => std::fs::remove_file(&script_path).unwrap(),
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    let atb = AutoLaunchBuilder::new()
        .set_app_name(launcher_name)
        .set_app_path(launcher_path.to_str().unwrap())
        .build();

    match atb {
        Ok(at) => {
            if at.is_enabled().is_ok_and(|enabled| enabled == false) {
                at.enable().unwrap();
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
