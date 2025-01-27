use anyhow::{Context, Result};
use either::Either;
use std::path::PathBuf;
use std::process::Command;

pub fn evaluate_attr(
    input: Either<&str, &PathBuf>,
    attr: &str,
    args: Option<&Vec<(&str, &str)>>,
) -> Result<String> {
    let mut command = Command::new("nix-instantiate");
    command.arg("--eval");

    command.arg(input);
    command.arg("--attr");
    command.arg(attr);

    match args {
        Some(args) => {
            for arg in args {
                command.arg("--arg");
                command.arg(arg.0);
                command.arg(arg.1);
            }
        }
        None => {}
    }

    command.arg("--json");

    let output = command
        .output()
        .context("Failed to execute nix-instantiate")?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    } else {
        // Print the error message
        let stderr = String::from_utf8(output.stderr)?;
        anyhow::bail!("Command failed:\n{}", stderr);
    }
}
