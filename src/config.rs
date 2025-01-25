//TODO: Add anyhow contexts everywhere...

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use toml;
use xdg;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    hosts: HashMap<String, Host>,
    users: HashMap<String, User>,
    sources: HashMap<String, Source>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Host {
    platform: String,
    verison: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    #[serde(rename = "home-manager")]
    home_manager: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Source {
    #[serde(rename = "type")]
    source_type: String,
    url: String,
    ref_: String,
    commit: String,
}

// TODO: break this up into main.rs

pub fn initialize() -> Result<()> {
    let out_path = get_lazy_directory()?;
    create_default_config(&out_path)
}

fn get_lazy_directory() -> Result<PathBuf> {
    let base_dir = xdg::BaseDirectories::with_prefix("lazy")?;
    let config_home = base_dir.get_config_home();
    if !config_home.exists() && config_home.is_dir() {
        fs::create_dir_all(&config_home)?
    };
    Ok(config_home)
}

fn create_default_config(out_path: &Path) -> Result<()> {
    // FIXME: Can't just be a template! Needs to build up from existing nix config.
    let init_path = Path::new("init");
    let lazy_toml_name = "lazy.toml";
    fs::copy(
        init_path.join(lazy_toml_name),
        &out_path.join(lazy_toml_name),
    )?;
    let gitignore_name = ".gitignore";
    fs::copy(
        init_path.join(gitignore_name),
        &out_path.join(gitignore_name),
    )?;
    Ok(())
}

fn parse_config(path: &Path) -> Result<Config> {
    let config_string = fs::read_to_string(path)?;
    let config = toml::from_str(&config_string)?;
    Ok(config)
}
