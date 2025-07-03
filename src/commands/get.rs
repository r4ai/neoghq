use crate::config::Config;
use anyhow::{Result, anyhow};

pub fn execute(config: Config, url: String, branch: Option<String>) -> Result<()> {
    execute_get_command(url, branch, config)
}

fn parse_repository_url(url: &str) -> Result<(String, String, String)> {
    use url::Url;
    let url = url.strip_suffix(".git").unwrap_or(url);

    // Handle HTTPS URLs
    if url.starts_with("https://") {
        let url = Url::parse(url).map_err(|_| anyhow!("Invalid URL format: {url}"))?;
        let path = url.path().strip_prefix("/").unwrap_or(url.path());
        let path_parts: Vec<&str> = path.split('/').collect();
        let host = url
            .host_str()
            .ok_or_else(|| anyhow!("Missing host in URL"))?;
        let owner = path_parts
            .get(0)
            .ok_or_else(|| anyhow!("Missing owner in URL: {url}"))?;
        let repo = path_parts
            .get(1)
            .ok_or_else(|| anyhow!("Missing repo in URL: {url}"))?;
        return Ok((host.to_string(), owner.to_string(), repo.to_string()));
    }

    // Handle SSH URLs
    if url.starts_with("git@") {
        let url_without_prefix = url.strip_prefix("git@").unwrap();
        let parts: Vec<&str> = url_without_prefix.split(':').collect();
        let host = parts
            .get(0)
            .ok_or_else(|| anyhow!("Missing host in URL: {url}"))?;
        let owner_and_repo = parts
            .get(1)
            .ok_or_else(|| anyhow!("Missing owner and repo in URL: {url}"))?
            .split("/")
            .collect::<Vec<_>>();
        let owner = owner_and_repo
            .get(0)
            .ok_or_else(|| anyhow!("Missing owner in URL: {url}"))?;
        let repo = owner_and_repo
            .get(1)
            .ok_or_else(|| anyhow!("Missing repo in URL: {url}"))?;
        return Ok((host.to_string(), owner.to_string(), repo.to_string()));
    }

    Err(anyhow!("Invalid URL format"))
}

fn resolve_repository_path(
    root: &std::path::Path,
    host: &str,
    owner: &str,
    repo: &str,
    branch: &str,
) -> std::path::PathBuf {
    root.join(host).join(owner).join(repo).join(branch)
}

fn clone_repository_bare(url: &str, path: &std::path::Path) -> Result<()> {
    use std::fs;

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Clone as bare repository
    let mut builder = git2::build::RepoBuilder::new();
    builder.bare(true);

    builder.clone(url, path)?;

    Ok(())
}

fn create_worktree(
    bare_repo_path: &std::path::Path,
    worktree_path: &std::path::Path,
    branch: &str,
) -> Result<()> {
    use git2::Repository;
    use std::fs;

    // Create parent directories if they don't exist
    if let Some(parent) = worktree_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Open the bare repository
    let repo = Repository::open(bare_repo_path)?;

    // Create worktree
    let opts = git2::WorktreeAddOptions::new();
    repo.worktree(branch, worktree_path, Some(&opts))?;

    Ok(())
}

fn execute_get_command(url: String, branch: Option<String>, config: Config) -> Result<()> {

    // Parse the repository URL to extract host, owner, and repo
    let (host, owner, repo) = parse_repository_url(&url)?;

    // Determine the branch to use (default to "main" if not specified)
    let branch = branch.unwrap_or_else(|| "main".to_string());

    // Use the root from config
    let root = config.root;

    // Create repository and worktree paths
    let repo_dir = root.join(&host).join(&owner).join(&repo);
    let bare_repo_path = repo_dir.join(".git");
    let worktree_path = resolve_repository_path(&root, &host, &owner, &repo, &branch);

    // Clone the bare repository if it doesn't exist
    if !bare_repo_path.exists() {
        println!("Cloning {} into {}", url, bare_repo_path.display());
        clone_repository_bare(&url, &bare_repo_path)?;
    }

    // Create the worktree if it doesn't exist
    if !worktree_path.exists() {
        println!(
            "Creating worktree for branch '{}' in {}",
            branch, worktree_path.display()
        );
        create_worktree(&bare_repo_path, &worktree_path, &branch)?;
    }

    println!("Repository cloned successfully: {}", worktree_path.display());
    Ok(())
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod parse_tests {
    use super::*;

    #[test]
    fn test_parse_github_url() {
        let url = "https://github.com/user/repo.git";
        let result = parse_repository_url(url);

        assert!(result.is_ok());
        let (host, owner, repo) = result.unwrap();
        assert_eq!(host, "github.com");
        assert_eq!(owner, "user");
        assert_eq!(repo, "repo");
    }

    #[test]
    fn test_parse_github_ssh_url() {
        let url = "git@github.com:user/repo.git";
        let result = parse_repository_url(url);

        assert!(result.is_ok());
        let (host, owner, repo) = result.unwrap();
        assert_eq!(host, "github.com");
        assert_eq!(owner, "user");
        assert_eq!(repo, "repo");
    }

    #[test]
    fn test_parse_github_url_invalid_https_format() {
        let url = "https://github.com/single-part"; // Invalid: only one part after domain
        let result = parse_repository_url(url);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_github_url_invalid_https_format_2() {
        let url = "https://example..com";
        let result = parse_repository_url(url);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_github_ssh_url_invalid_format() {
        let url = "git@github.com:single-part"; // Invalid: only one part after colon
        let result = parse_repository_url(url);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_repository_url_invalid_format() {
        let url = "invalid-url-format"; // Completely invalid URL
        let result = parse_repository_url(url);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_url_missing_host() {
        let url = "https://";
        let result = parse_repository_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_url_missing_owner() {
        let url = "https://github.com/";
        let result = parse_repository_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_ssh_url_missing_host() {
        let url = "git@";
        let result = parse_repository_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_ssh_url_missing_owner_and_repo() {
        let url = "git@github.com:";
        let result = parse_repository_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_ssh_url_missing_repo() {
        let url = "git@github.com:user";
        let result = parse_repository_url(url);
        assert!(result.is_err());
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod clone_repository_tests {
    use super::*;

    #[test]
    fn test_clone_repository_bare() {
        let temp_dir = tempfile::tempdir().unwrap();
        let bare_repo_path = temp_dir.path().join("repo.git");

        let result = clone_repository_bare(
            "https://github.com/octocat/Hello-World.git",
            &bare_repo_path,
        );

        assert!(result.is_ok());
        assert!(bare_repo_path.exists());
        assert!(bare_repo_path.join("HEAD").exists());
        assert!(bare_repo_path.join("refs").exists());
    }

    #[test]
    fn test_clone_repository_bare_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let bare_repo_path = temp_dir.path().join("repo.git");

        let result =
            clone_repository_bare("https://github.com/r4ai/404_notfound.git", &bare_repo_path);

        assert!(result.is_err());
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod create_worktree_tests {
    use super::*;

    #[test]
    fn test_create_worktree() {
        let temp_dir = tempfile::tempdir().unwrap();
        let bare_repo_path = temp_dir.path().join("repo.git");
        let worktree_path = temp_dir.path().join("main");

        // First create a bare repository
        clone_repository_bare(
            "https://github.com/octocat/Hello-World.git",
            &bare_repo_path,
        )
        .unwrap();

        // Now test worktree creation
        let result = create_worktree(&bare_repo_path, &worktree_path, "main");

        assert!(result.is_ok());
        assert!(worktree_path.exists());
        assert!(worktree_path.join("README").exists());
    }

    #[test]
    fn test_create_worktree_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let bare_repo_path = temp_dir.path().join("repo.git");
        let worktree_path = temp_dir.path().join("main");

        let result = create_worktree(&bare_repo_path, &worktree_path, "main");

        assert!(result.is_err());
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod execute_tests {
    use super::*;
    use crate::config;

    #[test]
    fn test_resolve_repository_path() {
        let root = std::path::Path::new("/tmp/neoghq");
        let host = "github.com";
        let owner = "user";
        let repo = "repo";
        let branch = "main";

        let result = resolve_repository_path(root, host, owner, repo, branch);

        assert_eq!(result, std::path::PathBuf::from("/tmp/neoghq/github.com/user/repo/main"));
    }

    #[test]
    fn test_execute_public_function() {
        let temp_dir = tempfile::tempdir().unwrap();

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = Some("main".to_string());
        let env = config::Env {
            neoghq_root: Some(temp_dir.path().to_path_buf()),
            home: None,
        };
        let config = Config::load(env).unwrap();

        let result = execute(config, url, branch);

        assert!(result.is_ok());

        // Verify directory structure was created correctly
        let repo_path = temp_dir.path().join("github.com/octocat/Hello-World");
        assert!(repo_path.exists());
        assert!(repo_path.join(".git").exists()); // bare repo
        assert!(repo_path.join("main").exists()); // worktree
        assert!(repo_path.join("main/README").exists()); // worktree content
    }

    #[test]
    fn test_execute_when_repository_already_exists() {
        let temp_dir = tempfile::tempdir().unwrap();

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = Some("main".to_string());
        let env = config::Env {
            neoghq_root: Some(temp_dir.path().to_path_buf()),
            home: None,
        };
        let config = Config::load(env).unwrap();

        // First execution - creates the repository
        let result1 = execute_get_command(url.clone(), branch.clone(), config.clone());
        assert!(result1.is_ok());

        // Second execution - repository already exists, should skip cloning but create worktree if needed
        let result2 = execute_get_command(url, branch, config);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_execute_with_default_branch() {
        let temp_dir = tempfile::tempdir().unwrap();

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = None;
        let env = config::Env {
            neoghq_root: Some(temp_dir.path().to_path_buf()),
            home: None,
        };
        let config = Config::load(env).unwrap();

        let result = execute_get_command(url, branch, config);

        assert!(result.is_ok());
        assert!(
            temp_dir
                .path()
                .join("github.com/octocat/Hello-World/main")
                .exists()
        );
    }

    #[test]
    fn test_execute_get_command_invalid_url() {
        let temp_dir = tempfile::tempdir().unwrap();
        let url = "invalid-url".to_string();
        let branch = Some("main".to_string());
        let env = config::Env {
            neoghq_root: Some(temp_dir.path().to_path_buf()),
            home: None,
        };
        let config = Config::load(env).unwrap();
        let result = execute_get_command(url, branch, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_get_command_clone_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let url = "https://github.com/r4ai/404_notfound.git".to_string();
        let branch = Some("main".to_string());
        let env = config::Env {
            neoghq_root: Some(temp_dir.path().to_path_buf()),
            home: None,
        };
        let config = Config::load(env).unwrap();
        let result = execute_get_command(url, branch, config);
        assert!(result.is_err());
    }
}
