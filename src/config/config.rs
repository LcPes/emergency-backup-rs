use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    env,
    io::{Read, Write},
    path::PathBuf,
};

/// Enum to handle various configuration errors
pub enum ConfigError {
    ConfigCreationError,
    ConfigCorrupted,
}

///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
pub struct Config {
    device_name: String,
    path_names: Vec<String>,
}

impl Config {
    pub fn new(device_name: String, path_names: Vec<String>) -> Self {
        Config {
            device_name,
            path_names,
        }
    }

    pub fn get_device_name(&self) -> String {
        self.device_name.clone()
    }

    pub fn get_path_names(&self) -> Vec<String> {
        self.path_names.clone()
    }
}

/// Function to create the configuration file and directory structure, it compiles the file with paths and device informations.
/// The configuration file uses a json format.
pub fn create_configuration(
    device_name: String,
    path_names: Vec<String>,
) -> Result<(), ConfigError> {
    let home_dir = env::var("HOME").expect("Unable to load the home directory");
    let path_dir = PathBuf::from(home_dir.clone())
        .join("Library")
        .join("Application Support")
        .join("eb-rs");
    let path_config = path_dir.join("config.json");

    if let Err(_) = std::fs::create_dir(path_dir) {
        return Err(ConfigError::ConfigCreationError);
    }

    let config_file = std::fs::File::create(path_config);

    if let Err(_) = config_file {
        return Err(ConfigError::ConfigCreationError);
    }

    let config = Config::new(device_name, path_names);

    if config_file
        .unwrap()
        .write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
        .is_err()
    {
        return Err(ConfigError::ConfigCreationError);
    }

    Ok(())
}

/// Function to get the configuration from the configuration file.
/// It returns a Config struct.
pub fn get_configuration() -> Result<Config, ConfigError> {
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

    let mut data = String::new();

    if config_file.unwrap().read_to_string(&mut data).is_err() {
        return Err(ConfigError::ConfigCorrupted);
    }

    let config: Result<Config, serde_json::Error> = serde_json::from_str(&data);

    if config.is_err() {
        return Err(ConfigError::ConfigCorrupted);
    }

    Ok(config.unwrap())
}
