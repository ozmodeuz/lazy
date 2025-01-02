use super::errors::Error;

use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, fs, path::PathBuf};
use xdg;

#[derive(Debug, Deserialize)]
pub struct Config {
    inputs: Inputs,
    hosts: HashMap<String, Host>,
    users: HashMap<String, User>,
}

#[derive(Debug, Deserialize)]
struct Inputs {
    nixpkgs: String,
}

#[derive(Debug, Deserialize)]
struct Host {
    stateversion: String,
    system: String,
}

#[derive(Debug, Deserialize)]
struct User {
    #[serde(rename = "home-manager")]
    home_manager: bool,
    stateversion: String,
}

pub fn get_config_directory() -> Result<PathBuf, Error> {
    match xdg::BaseDirectories::with_prefix("sysconf") {
        Ok(base_dir) => {
            let config_dir = base_dir.get_config_home();
            if config_dir.is_dir() {
                Ok(config_dir)
            } else {
                match fs::create_dir_all(&config_dir) {
                    Ok(()) => Ok(config_dir),
                    Err(error) => Err(Error::IOError(error)),
                }
            }
        }
        Err(error) => Err(Error::BaseDirectoriesError(error)),
    }
}

pub fn get_config_file(path: PathBuf) -> Result<PathBuf, Error> {
    let sysconf_file = path.join("sysconf.yaml");
    if sysconf_file.is_file() {
        Ok(sysconf_file)
    } else {
        match fs::copy("init/sysconf.yaml", &sysconf_file) {
            Ok(..) => Ok(sysconf_file),
            Err(error) => Err(Error::IOError(error)),
        }
    }
}

pub fn parse_config_file(path: PathBuf) -> Result<Config, Error> {
    match fs::read_to_string(path) {
        Ok(yaml_string) => match serde_yaml::from_str::<Config>(&yaml_string) {
            Ok(config) => Ok(config),
            Err(error) => Err(Error::YamlError(error)),
        },
        Err(error) => Err(Error::IOError(error)),
    }
}

// pub fn write_default_config(path: &PathBuf) -> Result<(), Error> {
//     match fs::copy(DEFAULT_CONFIG_FILE, path) {
//         Ok(..) => Ok(()),
//         Err(error) => Err(Error::IOError(error)),
//     }
// }

// pub fn write_default_gitignore(path: &PathBuf) -> Result<(), Error> {
//     match fs::copy(DEFAULT_GITIGNORE_FILE, path) {
//         Ok(..) => Ok(()),
//         Err(error) => Err(Error::IOError(error)),
//     }
// }
