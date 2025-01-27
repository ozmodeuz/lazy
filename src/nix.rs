use anyhow::{Context, Result};
use std::process::Command;

pub fn evaluate(
    input: &str,
    attr: Option<&str>,
    args: Option<&Vec<(&str, &str)>>,
) -> Result<String> {
    let mut command = Command::new("nix-instantiate");
    command.arg(input);
    command.arg("--eval");

    match attr {
        Some(attr) => {
            command.arg("--attr");
            command.arg(attr);
        }
        None => {}
    }

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

    let output = command
        .output()
        .context("Failed to execute nix-instantiate")?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)
            .context("Failed to convert evaluation result to string")?;
        Ok(stdout)
    } else {
        // Print the error message
        let stderr = String::from_utf8(output.stderr)
            .context("Failed to convert evaluation error to string")?;
        anyhow::bail!("Command failed:\n{}", stderr);
    }
}
