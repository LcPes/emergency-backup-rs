use eframe::egui::TextBuffer;
use std::{collections::HashMap, env, io::Write, path::PathBuf};

pub enum ConfigError {
    ConfigCreationError,
    ConfigCorrupted,
}

pub fn check_config() -> bool {
    let home_dir = std::env::var("HOME").expect("Unable to load the home directory");
    let path_dir = PathBuf::from(home_dir)
        .join("Library")
        .join("Application Support")
        .join("eb-rs");
    let path_config = path_dir.join("config.conf");

    if let Err(_) = std::fs::File::open(path_config) {
        return false;
    }

    if get_config().is_err() {
        return false;
    }

    true
}

pub fn create_config(
    selected_device: String,
    selected_paths: Vec<String>,
) -> Result<(), ConfigError> {
    let home_dir = env::var("HOME").expect("Unable to load the home directory");
    let path_dir = PathBuf::from(home_dir.clone())
        .join("Library")
        .join("Application Support")
        .join("eb-rs");
    let path_config = path_dir.join("config.conf");

    println!("{:?}", home_dir);
    println!("{:?}", path_config);

    if let Err(_) = std::fs::create_dir(path_dir) {
        return Err(ConfigError::ConfigCreationError);
    }

    let config_file = std::fs::File::create(path_config);

    if let Err(_) = config_file {
        return Err(ConfigError::ConfigCreationError);
    }

    let buf = String::from(
        "SELECTED_DEVICE=".to_string()
            + selected_device.as_str()
            + "\nSELECTED_PATHS="
            + selected_paths.join(";").as_str(),
    );

    if std::fs::File::write(&mut config_file.unwrap(), buf.as_bytes()).is_err() {
        return Err(ConfigError::ConfigCreationError);
    }

    Ok(())
}

pub fn get_config() -> Result<(String, Vec<String>), ConfigError> {
    let home_dir = std::env::var("HOME").expect("Unable to load the home directory");
    let path_dir = PathBuf::from(home_dir)
        .join("Library")
        .join("Application Support")
        .join("eb-rs");
    let path_config = path_dir.join("config.conf");
    let config_file = std::fs::File::open(path_config);

    if config_file.is_err() {
        return Err(ConfigError::ConfigCorrupted);
    }

    let mut buf = String::new();

    if std::io::Read::read_to_string(&mut config_file.unwrap(), &mut buf).is_err() {
        return Err(ConfigError::ConfigCorrupted);
    }

    let map = buf
        .split('\n')
        .map(|opt| {
            let opt_vec = opt.split('=').collect::<Vec<&str>>();
            let mut vec = Vec::new();

            if opt_vec[0].eq("SELECTED_PATHS") {
                opt_vec[1]
                    .split(';')
                    .for_each(|path| vec.push(path.to_string()));
            } else if opt_vec[0].eq("SELECTED_DEVICE") {
                vec.push(opt_vec[1].to_string());
            }

            (opt_vec[0].to_string(), vec)
        })
        .collect::<HashMap<String, Vec<String>>>();

    let selected_device = map.get("SELECTED_DEVICE");
    let selected_paths = map.get("SELECTED_PATHS");

    if selected_device.is_none() || selected_paths.is_none() {
        return Err(ConfigError::ConfigCorrupted);
    }

    Ok((
        selected_device.unwrap().clone()[0].take(),
        selected_paths.unwrap().clone(),
    ))
}
