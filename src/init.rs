use anyhow::{Context, Result};
use either::Either;
use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

struct Config {}

struct Flake {}

struct NixOS {}

fn import_nixos(path: Option<&Path>) -> Result<NixOS> {
    let default_path = PathBuf::from("/etc/nixos");
    read_dir(&path.unwrap_or(&default_path))
        .context("Failed to read NixOS configuration directory")?;
    Ok(nixos)
}

fn import_flake(path: Option<&Path>) -> Result<Flake> {
    let path = path.unwrap_or_else(|| Path::new("."));
    if path.is_dir() {
        path = &path.join("flake.nix");
    }
    let flake_string = read_to_string(path)?;
    Ok(())
}

fn generate_config(input: Either<Flake, NixOS>) -> Result<Config> {
    Ok(config)
}

fn update_config(Config) -> Result<Config> {}

fn check_existing_config(path Option<&Path>) -> Option<Config> {

}
