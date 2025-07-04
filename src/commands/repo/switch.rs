use crate::config::{Config, Env};
use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

pub fn execute(repo: String, worktree: Option<String>) -> Result<()> {
    let env = Env::load()?;
    let config = Config::load(env)?;

    execute_switch_command(repo, worktree, config)
}

fn parse_repo_name(repo: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow!(
            "Invalid repository format. Expected 'owner/repo', got: {}",
            repo
        ));
    }

    let owner = parts[0];
    let repo_name = parts[1];

    if owner.is_empty() || repo_name.is_empty() {
        return Err(anyhow!(
            "Invalid repository format. Owner and repo name cannot be empty"
        ));
    }

    Ok((owner.to_string(), repo_name.to_string()))
}

fn find_repository_path(root: &Path, owner: &str, repo: &str) -> Result<std::path::PathBuf> {
    // Look for repository in github.com first, then other hosts
    let hosts = ["github.com", "gitlab.com", "bitbucket.org"];

    for host in &hosts {
        let repo_path = root.join(host).join(owner).join(repo);
        if repo_path.exists() {
            return Ok(repo_path);
        }
    }

    // If not found in common hosts, search all hosts
    if root.exists() {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let host_path = entry.path();
                let repo_path = host_path.join(owner).join(repo);
                if repo_path.exists() {
                    return Ok(repo_path);
                }
            }
        }
    }

    Err(anyhow!("Repository not found: {}/{}", owner, repo))
}

fn find_default_worktree(repo_path: &Path) -> Result<std::path::PathBuf> {
    // Look for main branch first
    let main_path = repo_path.join("main");
    if main_path.exists() && main_path.is_dir() {
        return Ok(main_path);
    }

    // Look for master branch as fallback
    let master_path = repo_path.join("master");
    if master_path.exists() && master_path.is_dir() {
        return Ok(master_path);
    }

    // Look for any worktree (excluding .git)
    if repo_path.exists() {
        for entry in fs::read_dir(repo_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && path.file_name().unwrap() != ".git" {
                return Ok(path);
            }
        }
    }

    Err(anyhow!(
        "No worktree found in repository: {}",
        repo_path.display()
    ))
}

fn execute_switch_command(repo: String, worktree: Option<String>, config: Config) -> Result<()> {
    // Parse the repository name
    let (owner, repo_name) = parse_repo_name(&repo)?;

    // Find the repository path
    let repo_path = find_repository_path(&config.root, &owner, &repo_name)?;

    // Find the worktree path
    let worktree_path = if let Some(worktree_name) = worktree {
        let path = repo_path.join(&worktree_name);
        if !path.exists() {
            return Err(anyhow!(
                "Worktree '{}' not found in repository",
                worktree_name
            ));
        }
        path
    } else {
        find_default_worktree(&repo_path)?
    };

    // Output the path - this is what tools like shell functions will capture
    println!("{}", worktree_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use tempfile::TempDir;

    #[test]
    fn test_execute_repo_switch_success() {
        let temp_dir = TempDir::new().unwrap();
        let repo_name = "user/test-repo";

        // Set up environment variables for test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        // Create repository structure
        let repo_path = temp_dir
            .path()
            .join("github.com")
            .join("user")
            .join("test-repo");
        fs::create_dir_all(&repo_path).unwrap();
        fs::create_dir_all(repo_path.join("main")).unwrap();

        let result = execute(repo_name.to_string(), None);

        assert!(result.is_ok());

        // Verify that the path was output to stdout
        // This test should check that the correct path is printed
        let expected_path = repo_path.join("main");
        assert!(expected_path.exists());

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_repo_switch_repository_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let repo_name = "user/nonexistent-repo";

        // Set up environment variables for test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let result = execute(repo_name.to_string(), None);

        // Should fail when repository doesn't exist
        assert!(result.is_err());

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_repo_switch_invalid_repo_format() {
        let repo_name = "invalid-format";
        let result = execute(repo_name.to_string(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_default_worktree_prioritizes_main() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().join("test-repo");

        // Create multiple worktrees
        fs::create_dir_all(repo_path.join("feature-a")).unwrap();
        fs::create_dir_all(repo_path.join("feature-b")).unwrap();
        fs::create_dir_all(repo_path.join("main")).unwrap();

        let result = find_default_worktree(&repo_path);

        assert!(result.is_ok());
        let worktree_path = result.unwrap();
        assert_eq!(worktree_path.file_name().unwrap(), "main");
    }

    #[test]
    fn test_find_default_worktree_fallback_to_master() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().join("test-repo");

        // Create worktrees without main
        fs::create_dir_all(repo_path.join("feature-a")).unwrap();
        fs::create_dir_all(repo_path.join("master")).unwrap();
        fs::create_dir_all(repo_path.join("feature-b")).unwrap();

        let result = find_default_worktree(&repo_path);

        assert!(result.is_ok());
        let worktree_path = result.unwrap();
        assert_eq!(worktree_path.file_name().unwrap(), "master");
    }

    #[test]
    fn test_parse_repo_name_valid() {
        let result = parse_repo_name("user/repo");
        assert!(result.is_ok());
        let (owner, repo) = result.unwrap();
        assert_eq!(owner, "user");
        assert_eq!(repo, "repo");
    }

    #[test]
    fn test_parse_repo_name_invalid() {
        let result = parse_repo_name("invalid-format");
        assert!(result.is_err());

        let result = parse_repo_name("too/many/parts");
        assert!(result.is_err());

        let result = parse_repo_name("/missing-owner");
        assert!(result.is_err());

        let result = parse_repo_name("missing-repo/");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_repository_path() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create repository structure
        let repo_path = root.join("github.com").join("user").join("test-repo");
        fs::create_dir_all(&repo_path).unwrap();

        let result = find_repository_path(root, "user", "test-repo");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), repo_path);

        // Test with non-existent repository
        let result = find_repository_path(root, "user", "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_repo_switch_with_specific_worktree() {
        let temp_dir = TempDir::new().unwrap();
        let repo_name = "user/test-repo";

        // Set up environment variables for test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        // Create repository structure with multiple worktrees
        let repo_path = temp_dir
            .path()
            .join("github.com")
            .join("user")
            .join("test-repo");
        fs::create_dir_all(&repo_path).unwrap();
        fs::create_dir_all(repo_path.join("main")).unwrap();
        fs::create_dir_all(repo_path.join("dev")).unwrap();
        fs::create_dir_all(repo_path.join("feature")).unwrap();

        let result = execute(repo_name.to_string(), Some("dev".to_string()));

        assert!(result.is_ok());

        // Verify that the dev worktree path exists
        let expected_path = repo_path.join("dev");
        assert!(expected_path.exists());

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_repo_switch_with_nonexistent_worktree() {
        let temp_dir = TempDir::new().unwrap();
        let repo_name = "user/test-repo";

        // Set up environment variables for test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        // Create repository structure with only main worktree
        let repo_path = temp_dir
            .path()
            .join("github.com")
            .join("user")
            .join("test-repo");
        fs::create_dir_all(&repo_path).unwrap();
        fs::create_dir_all(repo_path.join("main")).unwrap();

        let result = execute(repo_name.to_string(), Some("nonexistent".to_string()));

        // Should fail when specified worktree doesn't exist
        assert!(result.is_err());

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_find_default_worktree_no_worktrees() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().join("empty-repo");
        fs::create_dir_all(&repo_path).unwrap();

        // Only create .git directory (no worktrees)
        fs::create_dir_all(repo_path.join(".git")).unwrap();

        let result = find_default_worktree(&repo_path);
        assert!(result.is_err());
    }
}
