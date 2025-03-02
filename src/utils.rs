use simplelog::*;
use std::env;
use std::path::PathBuf;

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
