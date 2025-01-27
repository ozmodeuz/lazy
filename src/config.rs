use semver::Version;
use sha2::Sha256;
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

pub struct Config {
    sources: Sources,
    hosts: Hosts,
}

struct Sources {
    sources: HashMap<String, Source>,
}

struct Source {
    url: Url,
    reference: Option<String>,
    sha256: Option<Sha256>,
}

struct Hosts {
    hosts: HashMap<String, Host>,
}

struct Host {
    platform: Platform,
    state_version: Version,
    modules: Vec<Module>,
    allow_unfree: Option<bool>,
}

pub enum Platform {
    X86_64,
    Aarch64,
}

struct Module {
    path: PathBuf,
}

pub struct Flake {
    inputs: HashMap<String, FlakeInput>,
    nixos_configurations: HashMap<String, NixOS>,
}

struct FlakeInput {
    url: Url,
    flake: Option<bool>,
    inputs: HashMap<String, FlakeInput>,
    follows: Option<String>,
}

pub struct NixOS {
    pub system: Platform,
    pub modules: Vec<Module>,
    pub state_version: Version,
}
