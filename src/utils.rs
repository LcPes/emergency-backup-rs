use auto_launch::AutoLaunchBuilder;
use log::info;
use plist::{dictionary, to_writer_xml, Value};
use simplelog::*;
use std::collections::HashMap;
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
#[warn(dead_code)]
pub fn setup_autolaunch_applescript() {
    let home_dir = env::var("HOME").expect("Unable to load the home directory");
    let path_dir = PathBuf::from(home_dir.clone())
        .join("Library")
        .join("Application Support")
        .join("eb-rs");
    let launcher_name = "eb-rs_launcher.app";
    let launcher_path = path_dir.join(launcher_name);

    let script = format!("do shell script \"HOME={} LAUNCH_JOB=TRUE /Users/$(whoami)/Applications/eb-rs.app/Contents/MacOS/eb-rs\"", home_dir);
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

pub fn setup_autolaunch_launchd() {
    let home_dir = env::var("HOME").expect("Unable to load the home directory");
    let app_path = PathBuf::from(home_dir.clone())
        .join("Applications")
        .join("eb-rs.app")
        .join("Contents")
        .join("MacOS")
        .join("eb-rs");
    let launch_agents_path = PathBuf::from(home_dir.clone())
        .join("Library")
        .join("LaunchAgents");
    let plist_name = "com.eb-rs";
    let plist_path =
        PathBuf::from(launch_agents_path.clone()).join(format!("{}.plist", plist_name));

    let atb = AutoLaunchBuilder::new()
        .set_app_name(plist_name)
        .set_app_path(app_path.to_str().unwrap())
        .set_use_launch_agent(true)
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

    let plist_file = File::open(&plist_path).unwrap();
    let mut plist_data = Value::from_reader(plist_file).unwrap();

    if let Value::Dictionary(plist_dict) = &mut plist_data {
        let mut env_vars = HashMap::new();
        env_vars.insert("HOME".to_string(), Value::String(home_dir));
        env_vars.insert("INSIDE_JOB".to_string(), Value::String("TRUE".to_string()));

        let env_var_key = "EnvironmentVariables".to_string();
        let keepalive_key = "KeepAlive".to_string();

        if plist_dict.contains_key(&env_var_key) {
            plist_dict.remove(&env_var_key);
        }

        let env_vars_dict = dictionary::Dictionary::from_iter(env_vars);

        plist_dict.insert(keepalive_key, Value::Boolean(true));
        plist_dict.insert(env_var_key, Value::Dictionary(env_vars_dict));
    }

    let new_plist = File::create(&plist_path).unwrap();
    to_writer_xml(new_plist, &plist_data).unwrap();

    let _ = Command::new("launchctl")
        .arg("remove")
        .arg(plist_name)
        .output();

    let user_id_out = Command::new("id").arg("-u").output().unwrap();
    let user_id = String::from_utf8(user_id_out.stdout)
        .unwrap()
        .trim()
        .to_string();

    let _ = Command::new("launchctl")
        .arg("bootstrap")
        .arg(format!("gui/{}", user_id))
        .arg(plist_path.to_str().unwrap())
        .output();
}
