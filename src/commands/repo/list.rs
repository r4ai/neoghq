use crate::config::{Config, Env};
use anyhow::Result;
use std::path::PathBuf;

pub fn execute() -> Result<()> {
    let env = Env::load()?;
    let config = Config::load(env)?;

    list_worktrees(&config.root)?;

    Ok(())
}

fn list_worktrees(root: &PathBuf) -> Result<()> {
    use std::fs;

    if !root.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            list_host_worktrees(&path)?;
        }
    }

    Ok(())
}

fn list_host_worktrees(host_path: &PathBuf) -> Result<()> {
    use std::fs;

    for entry in fs::read_dir(host_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            list_user_worktrees(&path)?;
        }
    }

    Ok(())
}

fn list_user_worktrees(user_path: &PathBuf) -> Result<()> {
    use std::fs;

    for entry in fs::read_dir(user_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            list_repo_worktrees(&path)?;
        }
    }

    Ok(())
}

fn list_repo_worktrees(repo_path: &PathBuf) -> Result<()> {
    use std::fs;

    for entry in fs::read_dir(repo_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && path.file_name().unwrap() != ".git" {
            println!("{}", path.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    mod execute_tests {
        use super::*;

        #[test]
        fn test_list_command_executes_successfully() {
            let result = execute();
            assert!(result.is_ok());
        }

        #[test]
        fn test_list_worktrees_with_env_load_error() {
            unsafe {
                std::env::set_var("HOME", "/nonexistent");
                std::env::remove_var("NEOGHQ_ROOT");
            }

            let result = execute();
            assert!(result.is_ok());

            unsafe {
                std::env::remove_var("HOME");
            }
        }
    }

    mod list_worktrees_tests {
        use super::*;

        #[test]
        fn test_list_worktrees_with_empty_root() {
            let temp_dir = TempDir::new().unwrap();
            let result = list_worktrees(&temp_dir.path().to_path_buf());
            assert!(result.is_ok());
        }

        #[test]
        fn test_list_worktrees_with_nonexistent_root() {
            let temp_dir = TempDir::new().unwrap();
            let nonexistent_path = temp_dir.path().join("nonexistent");
            let result = list_worktrees(&nonexistent_path);
            assert!(result.is_ok());
        }

        #[test]
        fn test_list_worktrees_with_structure() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create directory structure: github.com/user/repo/main
            let repo_path = root.join("github.com").join("user").join("repo");
            fs::create_dir_all(&repo_path).unwrap();
            fs::create_dir_all(repo_path.join("main")).unwrap();
            fs::create_dir_all(repo_path.join("feature")).unwrap();
            fs::create_dir_all(repo_path.join(".git")).unwrap();

            let result = list_worktrees(&root.to_path_buf());
            assert!(result.is_ok());
        }

        #[test]
        fn test_list_worktrees_with_files_in_root() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create file in root (should be ignored)
            fs::write(root.join("somefile.txt"), "content").unwrap();

            let result = list_worktrees(&root.to_path_buf());
            assert!(result.is_ok());
        }
    }

    mod list_host_worktrees_tests {
        use super::*;

        #[test]
        fn test_list_host_worktrees_with_read_error() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create a file instead of directory to trigger read error
            let file_path = root.join("notadirectory");
            fs::write(&file_path, "content").unwrap();

            let result = list_host_worktrees(&file_path);
            assert!(result.is_err());
        }

        #[test]
        fn test_list_host_worktrees_with_files() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create file in host directory (should be ignored)
            fs::write(root.join("somefile.txt"), "content").unwrap();

            let result = list_host_worktrees(&root.to_path_buf());
            assert!(result.is_ok());
        }
    }

    mod list_user_worktrees_tests {
        use super::*;

        #[test]
        fn test_list_user_worktrees_with_read_error() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create a file instead of directory to trigger read error
            let file_path = root.join("notadirectory");
            fs::write(&file_path, "content").unwrap();

            let result = list_user_worktrees(&file_path);
            assert!(result.is_err());
        }

        #[test]
        fn test_list_user_worktrees_with_files() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create file in user directory (should be ignored)
            fs::write(root.join("somefile.txt"), "content").unwrap();

            let result = list_user_worktrees(&root.to_path_buf());
            assert!(result.is_ok());
        }
    }

    mod list_repo_worktrees_tests {
        use super::*;

        #[test]
        fn test_list_repo_worktrees_with_read_error() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create a file instead of directory to trigger read error
            let file_path = root.join("notadirectory");
            fs::write(&file_path, "content").unwrap();

            let result = list_repo_worktrees(&file_path);
            assert!(result.is_err());
        }

        #[test]
        fn test_list_repo_worktrees_with_files() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create file in repo directory (should be ignored)
            fs::write(root.join("somefile.txt"), "content").unwrap();

            let result = list_repo_worktrees(&root.to_path_buf());
            assert!(result.is_ok());
        }
    }
}
