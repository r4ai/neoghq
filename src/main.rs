mod cli;
mod commands;

use clap::Parser;
use cli::Cli;
use commands::execute_command;

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = execute_command(cli.command) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_help_command_output() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
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
    fn test_get_command() {
        let output = Command::new("cargo")
            .args(&["run", "--", "get", "https://github.com/user/repo"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Get command: https://github.com/user/repo"));
    }
}