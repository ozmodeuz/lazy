use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn evaluate(file: &PathBuf, attr: &str, args: &Vec<(&str, &str)>) -> Result<String> {
    let mut command = Command::new("nix-instantiate");
    command.arg(file);
    command.arg("--eval");
    command.arg("--attr");
    command.arg(attr);
    for arg in args {
        command.arg("--arg");
        command.arg(arg.0);
        command.arg(arg.1);
    }

    let output = command
        .output()
        .context("Failed to execute `nix-instantiate` command")?;
    if output.status.success() {
        // Convert the output to a string and print it
        let stdout = String::from_utf8(output.stdout)
            .context("Failed to convert command output to UTF-8")?;
        Ok(stdout)
    } else {
        // Print the error message
        let stderr = String::from_utf8(output.stderr)
            .context("Failed to convert command error output to UTF-8")?;
        anyhow::bail!("Command failed:\n{}", stderr);
    }
}
