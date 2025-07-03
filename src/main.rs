#![cfg_attr(coverage, feature(coverage_attribute))]

mod cli;
mod commands;
mod config;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use commands::execute_command;

fn main() -> Result<()> {
    let env = config::Env::load()?;
    let config = config::Config::load(env)?;
    let cli = Cli::parse();

    execute_command(cli.command, config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_help_command_output() {
        let output = Command::new("cargo")
            .args(["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();

        assert!(stdout.contains("Git Worktree-Based Repository Manager"));
        assert!(stdout.contains("Usage:"));
        assert!(stdout.contains("Commands:"));
        assert!(stdout.contains("get"));
        assert!(stdout.contains("list"));
        assert!(stdout.contains("remove"));
        assert!(stdout.contains("root"));
        assert!(stdout.contains("create"));
    }

    #[test]
    fn test_get_command_attempts_clone() {
        let temp_dir = tempfile::tempdir().unwrap();

        let output = Command::new("cargo")
            .args(["run", "--", "get", "https://github.com/user/repo"])
            .env("NEOGHQ_ROOT", temp_dir.path())
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8(output.stderr).unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        // The command should attempt to clone and show appropriate output
        assert!(
            stderr.contains("Cloning https://github.com/user/repo")
                || stderr.contains("remote authentication required")
                || stderr.contains("repository not found")
                || stdout.contains("Cloning https://github.com/user/repo")
                || stderr.contains("no callback set")
        );
    }
}
