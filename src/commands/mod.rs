pub mod repo;
pub mod root;
pub mod worktree;

use crate::{
    cli::{Commands, RepoCommands, WorktreeCommands},
    config::Config,
};
use anyhow::Result;

pub fn execute_command(command: Commands, config: Config) -> Result<()> {
    match command {
        Commands::Repo { command } => execute_repo_command(command, config),
        Commands::Worktree { command } => execute_worktree_command(command, config),
        Commands::Root => root::execute(),
    }
}

fn execute_repo_command(command: RepoCommands, config: Config) -> Result<()> {
    match command {
        RepoCommands::Clone { url } => repo::clone::execute(config, url, None),
        RepoCommands::Create { repo, worktree } => repo::create::execute(repo, worktree),
        RepoCommands::Switch { repo, worktree } => repo::switch::execute(repo, worktree),
        RepoCommands::List { show_worktrees } => repo::list::execute(show_worktrees),
    }
}

fn execute_worktree_command(command: WorktreeCommands, _config: Config) -> Result<()> {
    match command {
        WorktreeCommands::Create { branch } => worktree::create::execute(branch),
        WorktreeCommands::Switch { branch } => worktree::switch::execute(branch),
        WorktreeCommands::Remove { branch } => worktree::remove::execute(branch),
        WorktreeCommands::Clean => worktree::clean::execute(),
        WorktreeCommands::Status => worktree::status::execute(),
        WorktreeCommands::List => worktree::list::execute(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn create_test_config() -> Config {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        Config {
            root: temp_dir.path().to_path_buf(),
        }
    }

    #[test]
    fn test_execute_command_root() {
        let config = create_test_config();
        let command = Commands::Root;

        let result = execute_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_command_repo_create() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };

        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let command = Commands::Repo {
            command: RepoCommands::Create {
                url: "https://github.com/user/repo".to_string(),
            },
        };

        let result = execute_command(command, config);
        assert!(result.is_ok());

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_command_repo_switch() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };

        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let command = Commands::Repo {
            command: RepoCommands::Switch {
                repo: "user/repo".to_string(),
            },
        };

        let result = execute_command(command, config);
        assert!(result.is_err()); // Should fail because repository doesn't exist

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_command_repo_list() {
        let config = create_test_config();
        let command = Commands::Repo {
            command: RepoCommands::List,
        };

        let result = execute_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_command_worktree_create() {
        let config = create_test_config();
        let command = Commands::Worktree {
            command: WorktreeCommands::Create {
                branch: "feature/test".to_string(),
            },
        };

        let result = execute_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_command_worktree_switch() {
        let config = create_test_config();
        let command = Commands::Worktree {
            command: WorktreeCommands::Switch {
                branch: "feature/test".to_string(),
            },
        };

        let result = execute_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_command_worktree_remove() {
        let config = create_test_config();
        let command = Commands::Worktree {
            command: WorktreeCommands::Remove {
                branch: "feature/test".to_string(),
            },
        };

        let result = execute_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_command_worktree_clean() {
        let config = create_test_config();
        let command = Commands::Worktree {
            command: WorktreeCommands::Clean,
        };

        let result = execute_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_command_worktree_status() {
        let config = create_test_config();
        let command = Commands::Worktree {
            command: WorktreeCommands::Status,
        };

        let result = execute_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_command_worktree_list() {
        let config = create_test_config();
        let command = Commands::Worktree {
            command: WorktreeCommands::List,
        };

        let result = execute_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_repo_command_clone() {
        let config = create_test_config();
        let command = RepoCommands::Clone {
            url: "https://github.com/user/repo".to_string(),
        };

        let result = execute_repo_command(command, config);
        assert!(result.is_err()); // Should fail because it's not a real repo
    }

    #[test]
    fn test_execute_repo_command_create() {
        // The create command should work with a temporary config
        // Set up environment variable to make the command work properly
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };

        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let command = RepoCommands::Create {
            url: "https://github.com/user/repo".to_string(),
        };

        let result = execute_repo_command(command, config);
        assert!(result.is_ok());

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_repo_command_switch() {
        // The switch command should fail when no repository exists
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };

        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let command = RepoCommands::Switch {
            repo: "user/repo".to_string(),
        };

        let result = execute_repo_command(command, config);
        assert!(result.is_err()); // Should fail because repository doesn't exist

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_repo_command_list() {
        let config = create_test_config();
        let command = RepoCommands::List;

        let result = execute_repo_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_worktree_command_create() {
        let config = create_test_config();
        let command = WorktreeCommands::Create {
            branch: "feature/test".to_string(),
        };

        let result = execute_worktree_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_worktree_command_switch() {
        let config = create_test_config();
        let command = WorktreeCommands::Switch {
            branch: "feature/test".to_string(),
        };

        let result = execute_worktree_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_worktree_command_remove() {
        let config = create_test_config();
        let command = WorktreeCommands::Remove {
            branch: "feature/test".to_string(),
        };

        let result = execute_worktree_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_worktree_command_clean() {
        let config = create_test_config();
        let command = WorktreeCommands::Clean;

        let result = execute_worktree_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_worktree_command_status() {
        let config = create_test_config();
        let command = WorktreeCommands::Status;

        let result = execute_worktree_command(command, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_worktree_command_list() {
        let config = create_test_config();
        let command = WorktreeCommands::List;

        let result = execute_worktree_command(command, config);
        assert!(result.is_ok());
    }
}
