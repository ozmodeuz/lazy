pub struct Nix {
    args: Vec<String>,
}

impl Nix {
    pub fn new() -> Self {
        Self {
            //TODO: Review these args...
            args: vec![
                "--extra-experimental-features".to_string(),
                "nix-command".to_string(),
                "--extra-experimental-features".to_string(),
                "flakes".to_string(),
                "--option".to_string(),
                "warn-dirty".to_string(),
                "false".to_string(),
                "--keep-going".to_string(),
            ],
        }
    }
}
