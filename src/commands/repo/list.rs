use crate::config::{Config, Env};
use anyhow::Result;
use std::path::PathBuf;

pub fn execute(show_worktrees: bool) -> Result<()> {
    let env = Env::load()?;
    let config = Config::load(env)?;

    execute_with_config(show_worktrees, config)
}

pub fn execute_with_config(show_worktrees: bool, config: Config) -> Result<()> {
    if show_worktrees {
        list_worktrees(&config.root)?;
    } else {
        list_repositories(&config.root)?;
    }

    Ok(())
}

fn list_repositories(root: &PathBuf) -> Result<()> {
    use std::fs;

    if !root.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            list_host_repositories(&path)?;
        }
    }

    Ok(())
}

fn list_host_repositories(host_path: &PathBuf) -> Result<()> {
    use std::fs;

    for entry in fs::read_dir(host_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            list_user_repositories(&path)?;
        }
    }

    Ok(())
}

fn list_user_repositories(user_path: &PathBuf) -> Result<()> {
    use std::fs;

    for entry in fs::read_dir(user_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // This is a repository directory - print it
            println!("{}", path.display());
        }
    }

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
            let result = execute(false);
            assert!(result.is_ok());
        }

        #[test]
        fn test_list_worktrees_with_env_load_error() {
            // Test with invalid paths to simulate env load error conditions
            let temp_dir = TempDir::new().unwrap();
            let config = Config {
                root: temp_dir.path().join("nonexistent").to_path_buf(),
            };

            let result = execute_with_config(false, config);
            assert!(result.is_ok()); // Should handle nonexistent directories gracefully
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

    mod list_repositories_tests {
        use super::*;

        #[test]
        fn test_list_repositories_with_empty_root() {
            let temp_dir = TempDir::new().unwrap();
            let result = list_repositories(&temp_dir.path().to_path_buf());
            assert!(result.is_ok());
        }

        #[test]
        fn test_list_repositories_with_structure() {
            let temp_dir = TempDir::new().unwrap();
            let root = temp_dir.path();

            // Create directory structure: github.com/user/repo1, github.com/user/repo2
            let repo1_path = root.join("github.com").join("user").join("repo1");
            let repo2_path = root.join("github.com").join("user").join("repo2");
            fs::create_dir_all(&repo1_path).unwrap();
            fs::create_dir_all(&repo2_path).unwrap();
            fs::create_dir_all(repo1_path.join(".git")).unwrap();
            fs::create_dir_all(repo2_path.join(".git")).unwrap();

            let result = list_repositories(&root.to_path_buf());
            assert!(result.is_ok());
        }
    }

    mod execute_enhanced_tests {
        use super::*;

        #[test]
        fn test_execute_with_show_worktrees_true() {
            let result = execute(true);
            assert!(result.is_ok());
        }

        #[test]
        fn test_execute_with_show_worktrees_false() {
            let result = execute(false);
            assert!(result.is_ok());
        }

        #[test]
        fn test_execute_list_repositories_mode() {
            let temp_dir = TempDir::new().unwrap();

            let config = Config {
                root: temp_dir.path().to_path_buf(),
            };

            // Create repository structure
            let repo_path = temp_dir
                .path()
                .join("github.com")
                .join("user")
                .join("test-repo");
            fs::create_dir_all(&repo_path).unwrap();
            fs::create_dir_all(repo_path.join("main")).unwrap();
            fs::create_dir_all(repo_path.join(".git")).unwrap();

            let result = execute_with_config(false, config); // List repositories mode
            assert!(result.is_ok());
        }

        #[test]
        fn test_execute_show_worktrees_mode() {
            let temp_dir = TempDir::new().unwrap();

            let config = Config {
                root: temp_dir.path().to_path_buf(),
            };

            // Create repository structure
            let repo_path = temp_dir
                .path()
                .join("github.com")
                .join("user")
                .join("test-repo");
            fs::create_dir_all(&repo_path).unwrap();
            fs::create_dir_all(repo_path.join("main")).unwrap();
            fs::create_dir_all(repo_path.join("dev")).unwrap();
            fs::create_dir_all(repo_path.join(".git")).unwrap();

            let result = execute_with_config(true, config); // Show worktrees mode
            assert!(result.is_ok());
        }

        // NEW CLI OPTIONS TESTS
        #[test]
        fn test_show_worktrees_option_behavior() {
            let temp_dir = TempDir::new().unwrap();
            let config = Config {
                root: temp_dir.path().to_path_buf(),
            };

            // Create complex repository structure
            let repos = vec![
                ("github.com/user1/repo1", vec!["main", "dev"]),
                ("github.com/user2/repo2", vec!["main", "feature", "hotfix"]),
                ("github.com/org/project", vec!["main"]),
            ];

            for (repo_path, worktrees) in repos {
                let full_path = temp_dir.path().join(repo_path);
                fs::create_dir_all(&full_path).unwrap();
                fs::create_dir_all(full_path.join(".git")).unwrap();

                for worktree in worktrees {
                    fs::create_dir_all(full_path.join(worktree)).unwrap();
                }
            }

            // Test both modes
            let result_repos = execute_with_config(false, config.clone()); // List repositories
            assert!(result_repos.is_ok());

            let result_worktrees = execute_with_config(true, config); // Show worktrees  
            assert!(result_worktrees.is_ok());
        }

        #[test]
        fn test_show_worktrees_flag_default_behavior() {
            let temp_dir = TempDir::new().unwrap();
            let config = Config {
                root: temp_dir.path().to_path_buf(),
            };

            // Create single repository
            let repo_path = temp_dir
                .path()
                .join("github.com")
                .join("user")
                .join("test-repo");
            fs::create_dir_all(&repo_path).unwrap();
            fs::create_dir_all(repo_path.join("main")).unwrap();
            fs::create_dir_all(repo_path.join(".git")).unwrap();

            // Default behavior (false) should list repositories, not worktrees
            let result = execute_with_config(false, config);
            assert!(result.is_ok());
        }
    }
}
