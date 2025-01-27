use crate::config::{Config, Flake, NixOS, Platform};
use crate::nix::evaluate;

use anyhow::Result;
use std::path::PathBuf;

pub fn import_test(path: Option<PathBuf>) -> Result<()> {
    let path = get_nixos_config_path(path)?;
    let args = vec![("config", "{}"), ("pkgs", "{}")];
    let imports = evaluate(&path, "imports", &args)?;
    let state_version = evaluate(&path, "system.stateVersion", &args)?;
    println!("Imports: {}", imports);
    println!("StateVersion: {}", state_version);
    Ok(())
}

fn get_nixos_config_path(path: Option<PathBuf>) -> Result<PathBuf> {
    let default_path = PathBuf::from("/etc/nixos");
    let default_file = PathBuf::from("configuration.nix");
    match path {
        Some(path) if path.is_dir() => Ok(path.join(&default_file)),
        Some(path) if path.is_file() => Ok(path),
        Some(_) => Err(anyhow::anyhow!("Invalid path")),
        None => Ok(default_path.join(&default_file)),
    }
}

// fn import_nixos_config(path: PathBuf) -> Result<()> {
//     let string = read_to_string(path)?;
//     let ast = parse_nix(&string)?;
//     let system = ;

//     let system = find_node(&ast, "system").and_then(|n| get_node_value(&n));
//     let modules = find_node(&ast, "modules").and_then(|n| get_node_value(&n));
//     let state_version = find_node(&ast, "system.stateVersion").and_then(|n| get_node_value(&n));
//     println!("System: {:?}", system.unwrap_or("None".to_string()));
//     println!("Modules: {:?}: ", modules.unwrap_or("None".to_string()));
//     println!(
//         "StateVersion: {:?}",
//         state_version.unwrap_or("None".to_string())
//     );
//     Ok(())
// }

// fn get_flake_path(path: Option<PathBuf>) -> Result<PathBuf> {
//     let default_path = PathBuf::from(".");
//     let default_file = PathBuf::from("flake.nix");
//     match path {
//         Some(path) if path.is_dir() => Ok(path.join(&default_file)),
//         Some(path) if path.is_file() => Ok(path),
//         Some(_) => Err(anyhow::anyhow!("Invalid path")),
//         None => Ok(default_path.join(&default_file)),
//     }
// }

// // fn import_flake(path: Option<PathBuf>) -> Result<Flake> {
// //     let flake_file_path = get_flake_path(path)?;
// //     let flake_file = read_to_string(flake_file_path)?;
// //     let flake = parse_nix(&flake_file)?;
// //     init_from_flake(flake)?;
// //     Ok(())
// // }

// // fn import_lazy_config(path: Option<PathBuf>) -> Result<Config> {}

// // fn init_from_nixos(nixos: NixOS) -> Result<Config> {}

// // fn init_from_flake(flake: Flake) -> Result<Config> {}

// // fn init_from_existing(config: Config) -> Result<Config> {}
