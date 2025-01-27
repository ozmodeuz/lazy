use crate::nix::evaluate_attr;

use anyhow::Result;
use either::{Left, Right};
use git2::{Direction, Remote};
use hostname;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
struct Config {
    sources: Sources,
    hosts: Hosts,
    settings: Settings,
}

#[derive(Debug)]
struct Settings {
    allow_modifications: bool,
    allow_unfree: bool,
}

#[derive(Debug)]
struct Sources {
    sources: HashMap<String, Source>,
}

#[derive(Debug)]
struct Source {
    url: Url,
    reference: String,
    commit: String,
    tarball: String,
}

#[derive(Debug)]
struct Hosts {
    hosts: HashMap<String, Host>,
}

#[derive(Debug)]
struct Host {
    architecture: Architecture,
    state_version: String,
    modules: Vec<Module>,
    allow_unfree: bool,
}

#[derive(Debug)]
enum Architecture {
    X86_64,
    Aarch64,
}

#[derive(Debug)]
struct Module {
    path: PathBuf,
}

struct Flake {
    inputs: HashMap<String, FlakeInput>,
    nixos_configurations: HashMap<String, NixOS>,
}

struct FlakeInput {
    url: Url,
    flake: bool,
    inputs: HashMap<String, FlakeInput>,
    follows: String,
}

struct NixOS {
    hostname: String,
    architecture: Architecture,
    modules: Vec<Module>,
    nixpkgs: Source,
    state_version: String,
}

pub fn init_test(path: Option<PathBuf>) -> Result<()> {
    let path = get_nixos_config_path(path)?;
    let nixos = import_nixos(path)?;
    init_from_nixos(nixos)?;
    Ok(())
}

fn get_architecture() -> Result<Architecture> {
    match std::env::consts::ARCH {
        "x86_64" => Ok(Architecture::X86_64),
        "aarch64" => Ok(Architecture::Aarch64),
        _ => Err(anyhow::anyhow!("Unsupported architecture")),
    }
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

fn import_nixos(path: PathBuf) -> Result<NixOS> {
    let hostname = hostname::get()?.to_string_lossy().into_owned();
    let state_version = get_state_version(&path)?;
    let architecture = get_architecture()?;
    let nixpkgs = get_latest_nixpkgs()?;
    Ok(NixOS {
        hostname,
        architecture,
        modules: Vec::new(),
        nixpkgs,
        state_version,
    })
}

// fn import_lazy_config(path: PathBuf) -> Result<Config> {
//     let string = fs::read_to_string(path)?
// }

fn get_state_version(path: &PathBuf) -> Result<String> {
    let args = vec![("config", "{}"), ("pkgs", "{}")];
    let attr = "system.stateVersion";
    let string = evaluate_attr(Right(&path), &attr, Some(&args))?;
    let json: serde_json::Value = serde_json::from_str(&string)?;
    Ok(json.to_string().trim_matches('"').to_string())
}

fn get_latest_nixpkgs() -> Result<Source> {
    let url = Url::parse("https://github.com/NixOS/nixpkgs")?;
    let reference = String::from("nixos-24.11");
    let commit = get_latest_git_commit(&url, &reference)?;
    Ok(Source {
        url,
        reference,
        commit,
        tarball: String::new(),
    })
}

fn init_from_nixos(nixos: NixOS) -> Result<()> {
    let host = Host {
        architecture: nixos.architecture,
        state_version: nixos.state_version,
        modules: Vec::new(),
        allow_unfree: false,
    };
    let mut hosts_map = HashMap::new();
    hosts_map.insert(nixos.hostname, host);
    let hosts = Hosts { hosts: hosts_map };
    let mut sources_map = HashMap::new();
    sources_map.insert(String::from("nixpkgs"), nixos.nixpkgs);
    let sources = Sources {
        sources: sources_map,
    };
    let settings = Settings {
        allow_modifications: false,
        allow_unfree: false,
    };
    let config = Config {
        sources,
        hosts,
        settings,
    };
    let nixos_config_path = create_nixos_dir()?;
    let lazy_nix_string = config_to_nix(config)?;
    let lazy_file_path = nixos_config_path.join("lazy.nix");
    fs::write(lazy_file_path, lazy_nix_string)?;
    Ok(())
}

fn init_from_existing(config: Config) -> Result<()> {
    Ok(())
}

fn config_to_nix(config: Config) -> Result<String> {
    let mut output = String::new();
    output.push_str("{\n");
    output.push_str("  sources = {\n");

    for (name, source) in &config.sources.sources {
        output.push_str(&format!(
            "    {} = {{
      url = \"{}\";
      ref = \"{}\";
      commit = \"{}\";
    }};\n",
            name, source.url, source.reference, source.commit,
        ));
    }
    output.push_str("  };\n");

    output.push_str("  hosts = {\n");
    for (name, host) in &config.hosts.hosts {
        output.push_str(&format!(
            "    {} = {{
      architecture = \"{}\";
      modules = [];
      stateVersion = \"{}\";
    }};\n",
            name,
            match host.architecture {
                Architecture::X86_64 => "x86_64",
                Architecture::Aarch64 => "aarch64",
            },
            host.state_version,
        ));
    }
    output.push_str("  };\n");

    output.push_str("}\n");
    Ok(output)
}

fn get_latest_git_commit(url: &Url, reference: &str) -> Result<String> {
    let mut remote = Remote::create_detached(url.as_str())?;
    remote.connect(Direction::Fetch)?;
    let full_reference = String::from("refs/heads/") + reference;
    let references = remote.list()?;
    let reference = references.iter().find(|r| r.name() == full_reference);
    match reference {
        Some(reference) => Ok(reference.oid().to_string()),
        None => Err(anyhow::anyhow!("Reference not found")),
    }
}

fn create_nixos_dir() -> Result<PathBuf> {
    let path = xdg::BaseDirectories::new()?.get_config_home().join("nixos");
    fs::create_dir_all(&path)?;
    Ok(path)
}

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

// // fn init_from_flake(flake: Flake) -> Result<Config> {}

// // fn init_from_existing(config: Config) -> Result<Config> {}
