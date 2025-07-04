use crate::config::{Config, Env};
use anyhow::{Result, anyhow};
use git2::Repository;
use std::fs;
use std::path::Path;

pub fn execute(repo: String, worktree: Option<String>) -> Result<()> {
    let env = Env::load()?;
    let config = Config::load(env)?;

    execute_create_command(repo, worktree, config)
}

#[cfg(test)]
pub fn execute_with_config(repo: String, worktree: Option<String>, config: Config) -> Result<()> {
    execute_create_command(repo, worktree, config)
}

fn parse_repo_name(repo: &str) -> Result<(String, String, String)> {
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

    // Default to github.com for user/repo format
    Ok((
        "github.com".to_string(),
        owner.to_string(),
        repo_name.to_string(),
    ))
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
            .first()
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
            .first()
            .ok_or_else(|| anyhow!("Missing host in URL: {url}"))?;
        let owner_and_repo = parts
            .get(1)
            .ok_or_else(|| anyhow!("Missing owner and repo in URL: {url}"))?
            .split("/")
            .collect::<Vec<_>>();
        let owner = owner_and_repo
            .first()
            .ok_or_else(|| anyhow!("Missing owner in URL: {url}"))?;
        let repo = owner_and_repo
            .get(1)
            .ok_or_else(|| anyhow!("Missing repo in URL: {url}"))?;
        return Ok((host.to_string(), owner.to_string(), repo.to_string()));
    }

    Err(anyhow!("Invalid URL format"))
}

fn create_bare_repository(path: &Path) -> Result<()> {
    use git2::{Signature, Time};

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Initialize bare repository
    let repo = Repository::init_bare(path)?;

    // Create initial commit with empty tree
    let signature = Signature::new("neoghq", "neoghq@example.com", &Time::new(0, 0))?;
    let tree_id = repo.treebuilder(None)?.write()?;
    let tree = repo.find_tree(tree_id)?;

    let _initial_commit = repo.commit(
        Some("refs/heads/main"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;

    Ok(())
}

fn create_worktree(bare_repo_path: &Path, worktree_path: &Path, branch: &str) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = worktree_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Open the bare repository
    let repo = Repository::open(bare_repo_path)?;

    // Create worktree
    let branch_ref = format!("refs/heads/{branch}");
    let mut opts = git2::WorktreeAddOptions::new();

    if let Ok(reference) = repo.find_reference(&branch_ref) {
        // Branch exists, use it directly
        opts.reference(Some(&reference));
        repo.worktree(branch, worktree_path, Some(&opts))?;
    } else {
        // Branch doesn't exist, create from main branch
        if let Ok(main_ref) = repo.find_reference("refs/heads/main") {
            opts.reference(Some(&main_ref));
            repo.worktree(branch, worktree_path, Some(&opts))?;
        } else {
            // Fallback: let git2 handle it (might create from HEAD)
            repo.worktree(branch, worktree_path, Some(&opts))?;
        }
    }

    Ok(())
}

fn execute_create_command(
    repo_input: String,
    worktree: Option<String>,
    config: Config,
) -> Result<()> {
    // For now, support both URL and user/repo format
    let (host, owner, repo) = if repo_input.contains("://") || repo_input.starts_with("git@") {
        // Parse as URL
        parse_repository_url(&repo_input)?
    } else {
        // Parse as user/repo format
        parse_repo_name(&repo_input)?
    };

    // Use the root from config
    let root = config.root;

    // Create repository and worktree paths
    let repo_dir = root.join(&host).join(&owner).join(&repo);
    let bare_repo_path = repo_dir.join(".git");
    let branch_name = worktree.unwrap_or_else(|| "main".to_string());
    let worktree_path = repo_dir.join(&branch_name);

    // Create the bare repository if it doesn't exist
    if !bare_repo_path.exists() {
        println!("Creating bare repository at {}", bare_repo_path.display());
        create_bare_repository(&bare_repo_path).map_err(|e| {
            anyhow!(
                "Failed to create bare repository at {}: {}",
                bare_repo_path.display(),
                e
            )
        })?;
    } else {
        println!(
            "Bare repository already exists at {}",
            bare_repo_path.display()
        );
    }

    // Create the worktree if it doesn't exist
    if !worktree_path.exists() {
        println!(
            "Creating {} worktree at {}",
            branch_name,
            worktree_path.display()
        );
        // Only try to create worktree if we have a valid repository
        if let Err(e) = create_worktree(&bare_repo_path, &worktree_path, &branch_name) {
            // If worktree creation fails and bare repo already existed,
            // it might be an empty/invalid repository - recreate it
            if bare_repo_path.exists() {
                println!("Repository exists but is invalid, recreating...");
                // Remove both .git directory and any existing worktree directory
                std::fs::remove_dir_all(&bare_repo_path).map_err(|err| {
                    anyhow!(
                        "Failed to remove invalid repository at {}: {}",
                        bare_repo_path.display(),
                        err
                    )
                })?;
                if worktree_path.exists() {
                    std::fs::remove_dir_all(&worktree_path).map_err(|err| {
                        anyhow!(
                            "Failed to remove existing worktree directory at {}: {}",
                            worktree_path.display(),
                            err
                        )
                    })?;
                }
                create_bare_repository(&bare_repo_path).map_err(|err| {
                    anyhow!(
                        "Failed to recreate bare repository at {}: {}",
                        bare_repo_path.display(),
                        err
                    )
                })?;
                create_worktree(&bare_repo_path, &worktree_path, &branch_name).map_err(|err| {
                    anyhow!(
                        "Failed to create {} worktree at {}: {}",
                        branch_name,
                        worktree_path.display(),
                        err
                    )
                })?;
            } else {
                return Err(anyhow!(
                    "Failed to create {} worktree at {}: {}",
                    branch_name,
                    worktree_path.display(),
                    e
                ));
            }
        }
    } else {
        println!(
            "Worktree '{}' already exists at {}",
            branch_name,
            worktree_path.display()
        );
    }

    println!("Repository created successfully: {}", repo_dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_repository_url_https() {
        let url = "https://github.com/user/repo.git";
        let result = parse_repository_url(url);

        assert!(result.is_ok());
        let (host, owner, repo) = result.unwrap();
        assert_eq!(host, "github.com");
        assert_eq!(owner, "user");
        assert_eq!(repo, "repo");
    }

    #[test]
    fn test_parse_repository_url_ssh() {
        let url = "git@github.com:user/repo.git";
        let result = parse_repository_url(url);

        assert!(result.is_ok());
        let (host, owner, repo) = result.unwrap();
        assert_eq!(host, "github.com");
        assert_eq!(owner, "user");
        assert_eq!(repo, "repo");
    }

    #[test]
    fn test_parse_repository_url_invalid() {
        let url = "invalid-url";
        let result = parse_repository_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_bare_repository() {
        let temp_dir = TempDir::new().unwrap();
        let bare_repo_path = temp_dir.path().join("test.git");

        let result = create_bare_repository(&bare_repo_path);

        assert!(result.is_ok());
        assert!(bare_repo_path.exists());
        assert!(bare_repo_path.join("HEAD").exists());
        assert!(bare_repo_path.join("refs").exists());
        assert!(bare_repo_path.join("refs/heads/main").exists());
    }

    #[test]
    fn test_create_worktree_with_existing_branch() {
        let temp_dir = TempDir::new().unwrap();
        let bare_repo_path = temp_dir.path().join("test.git");
        let worktree_path = temp_dir.path().join("main");

        // First create a bare repository
        create_bare_repository(&bare_repo_path).unwrap();

        // Now test worktree creation
        let result = create_worktree(&bare_repo_path, &worktree_path, "main");

        assert!(result.is_ok());
        assert!(worktree_path.exists());
    }

    #[test]
    fn test_execute_repo_create_success() {
        let temp_dir = TempDir::new().unwrap();
        let url = "https://github.com/user/new-repo".to_string();

        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };

        let result = execute_with_config(url.clone(), None, config);

        assert!(result.is_ok());

        // Verify that the repository structure was created
        let repo_path = temp_dir
            .path()
            .join("github.com")
            .join("user")
            .join("new-repo");
        assert!(repo_path.exists());
        assert!(repo_path.join(".git").exists()); // bare repo
        assert!(repo_path.join("main").exists()); // main branch worktree
    }

    #[test]
    fn test_execute_repo_create_invalid_url() {
        let url = "invalid-url".to_string();
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };
        let result = execute_with_config(url, None, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_repo_create_existing_repo() {
        let temp_dir = TempDir::new().unwrap();
        let url = "https://github.com/user/existing-repo".to_string();

        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };

        // Create existing repo structure
        let repo_path = temp_dir
            .path()
            .join("github.com")
            .join("user")
            .join("existing-repo");
        fs::create_dir_all(&repo_path).unwrap();
        fs::create_dir_all(repo_path.join(".git")).unwrap();

        let result = execute_with_config(url.clone(), None, config);

        // Should not fail if repo already exists
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_repo_create_user_repo_format() {
        let temp_dir = TempDir::new().unwrap();
        let repo = "user/test-repo".to_string();

        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };

        let result = execute_with_config(repo.clone(), None, config);

        assert!(result.is_ok());

        // Verify that the repository structure was created
        let repo_path = temp_dir
            .path()
            .join("github.com")
            .join("user")
            .join("test-repo");
        assert!(repo_path.exists());
        assert!(repo_path.join(".git").exists()); // bare repo
        assert!(repo_path.join("main").exists()); // main branch worktree
    }

    #[test]
    fn test_execute_repo_create_with_custom_worktree() {
        let temp_dir = TempDir::new().unwrap();
        let repo = "user/custom-worktree-test".to_string();
        let worktree = Some("dev".to_string());

        let config = Config {
            root: temp_dir.path().to_path_buf(),
        };

        let result = execute_with_config(repo.clone(), worktree, config);

        assert!(result.is_ok());

        // Verify that the repository structure was created with custom worktree
        let repo_path = temp_dir
            .path()
            .join("github.com")
            .join("user")
            .join("custom-worktree-test");
        assert!(repo_path.exists());
        assert!(repo_path.join(".git").exists()); // bare repo
        assert!(repo_path.join("dev").exists()); // dev branch worktree
    }

    #[test]
    fn test_parse_repo_name_valid() {
        let result = parse_repo_name("user/repo");
        assert!(result.is_ok());
        let (host, owner, repo) = result.unwrap();
        assert_eq!(host, "github.com");
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

    // NEW CLI OPTIONS TESTS
    #[test]
    fn test_worktree_option_validation() {
        // Test with different worktree names, each in separate directories
        let test_cases = vec!["dev", "feature", "hotfix", "release-1.0"];

        for worktree_name in test_cases {
            let temp_dir = TempDir::new().unwrap();
            let config = Config {
                root: temp_dir.path().to_path_buf(),
            };

            let result = execute_with_config(
                format!("user/test-repo-{worktree_name}"),
                Some(worktree_name.to_string()),
                config.clone(),
            );
            assert!(result.is_ok(), "Failed with worktree name: {worktree_name}");

            // Verify worktree directory was created
            let repo_path = temp_dir
                .path()
                .join("github.com")
                .join("user")
                .join(format!("test-repo-{worktree_name}"));
            assert!(repo_path.join(worktree_name).exists());
        }
    }

    #[test]
    fn test_user_repo_format_various_combinations() {
        let test_cases = vec![
            ("owner/simple", "github.com", "owner", "simple"),
            (
                "user-name/repo-name",
                "github.com",
                "user-name",
                "repo-name",
            ),
            ("org123/project456", "github.com", "org123", "project456"),
        ];

        for (input, expected_host, expected_owner, expected_repo) in test_cases {
            let result = parse_repo_name(input);
            assert!(result.is_ok(), "Failed to parse: {input}");

            let (host, owner, repo) = result.unwrap();
            assert_eq!(host, expected_host);
            assert_eq!(owner, expected_owner);
            assert_eq!(repo, expected_repo);
        }
    }
}
