use anyhow::{Result};
use either::Either;
use rnix;
use std::{
    fs::read_to_string,
    path::PathBuf,
};

struct Config {}

struct Flake {}

struct NixOS {}

pub fn import_nixos(path: Option<PathBuf>) -> Result<()> {
    let default_path = PathBuf::from("/etc/nixos");
    let default_file = PathBuf::from("configuration.nix");
    let config_file = match path {
        Some(path) => {
            if path.is_dir() {
                path.join(&default_file)
            } else if path.is_file() {
                path
            } else {
                return Err(anyhow::anyhow!("invalid path"));
            }
        },
        None => {
            default_path.join(&default_file)
        }
    };
    println!("Importing NixOS configuration from {}...", config_file.display());
    let nixos_string = read_to_string(config_file)?;
    let nixos = rnix::ast::Root::parse(&nixos_string);
    if !nixos.errors().is_empty() {
        return Err(anyhow::anyhow!("Errors while parsing: {:?}", nixos.errors()));
    }    
    let attr_set = "inputs";
    let value = get_attr_value(nixos.syntax(), &attr_set);
    println!("{:?}: {:?}", attr_set, value.unwrap());
    Ok(())
}

fn get_attr_value(syntax: rnix::SyntaxNode, attr_set: &str) -> Option<String> {
    let mut value: Option<String> = None;
    for node in syntax.descendants() {
        if node.kind() == rnix::SyntaxKind::NODE_ATTRPATH_VALUE {
            for child in node.children() {
                if child.to_string() == attr_set {
                    value = match node.last_child() {
                        Some(node) => Some(node.text().to_string()),
                        None => return None,
                    };
                }
            }
        }
    }
    return value.or(None);
}

// fn import_flake(path: Option<&Path>) -> Result<Flake> {
//     let mut path = path.unwrap_or_else(|| Path::new("."));
//     if path.is_dir() {
//         path = &path.join("flake.nix");
//     }
//     let flake_string = read_to_string(path)?;
//     Ok(())
// }

// fn generate_config(input: Either<Flake, NixOS>) -> Result<Config> {
//     Ok(config)
// }

// fn update_config(Config) -> Result<Config> {}

// fn import_lazy_config(path: Option<&Path>) -> Result<Config> {
//     let path = path.unwrap_or_else(|| Path::new("."));
//     if path.is_dir() {
//         path = &path.join("lazy.nix");
//     }
//     let lazy_config_string = read_to_string(path)?;
//     Ok(())   
// }
