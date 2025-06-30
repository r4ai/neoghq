use anyhow::Result;

pub fn execute(url: String, branch: Option<String>) -> Result<()> {
    execute_get_command(url, branch)
}

#[cfg(test)]
mod tests {
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
    fn test_resolve_repository_path() {
        let root = "/tmp/neoghq";
        let host = "github.com";
        let owner = "user";
        let repo = "repo";
        let branch = "main";

        let result = resolve_repository_path(root, host, owner, repo, branch);

        assert_eq!(result, "/tmp/neoghq/github.com/user/repo/main");
    }

    #[test]
    fn test_clone_repository_bare() {
        let temp_dir = tempfile::tempdir().unwrap();
        let bare_repo_path = temp_dir.path().join("repo.git");

        // This test will fail until we implement clone_repository_bare
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
    fn test_execute_public_function() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Set a temporary NEOGHQ_ROOT for this test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = Some("main".to_string());

        let result = execute(url, branch);

        assert!(result.is_ok());

        // Verify directory structure was created correctly
        let repo_path = temp_dir.path().join("github.com/octocat/Hello-World");
        assert!(repo_path.exists());
        assert!(repo_path.join(".git").exists()); // bare repo
        assert!(repo_path.join("main").exists()); // worktree
        assert!(repo_path.join("main/README").exists()); // worktree content

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_with_default_branch() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Set a temporary NEOGHQ_ROOT for this test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = None; // No branch specified, should default to "main"

        let result = execute_get_command(url, branch);

        assert!(result.is_ok());

        // Verify directory structure was created correctly with default branch "main"
        let repo_path = temp_dir.path().join("github.com/octocat/Hello-World");
        assert!(repo_path.exists());
        assert!(repo_path.join(".git").exists()); // bare repo
        assert!(repo_path.join("main").exists()); // default branch worktree
        assert!(repo_path.join("main/README").exists()); // worktree content

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_with_default_root() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Set HOME to temp directory and ensure NEOGHQ_ROOT is not set
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
            std::env::remove_var("NEOGHQ_ROOT");
        }

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = Some("main".to_string());

        let result = execute_get_command(url, branch);

        assert!(result.is_ok());

        // Verify directory structure was created in default location ~/src/repos
        let default_path = temp_dir.path().join("src/repos/github.com/octocat/Hello-World");
        assert!(default_path.exists());
        assert!(default_path.join(".git").exists()); // bare repo
        assert!(default_path.join("main").exists()); // worktree
        assert!(default_path.join("main/README").exists()); // worktree content

        // Clean up
        unsafe {
            std::env::remove_var("HOME");
        }
    }

    #[test]
    fn test_execute_with_home_directory_expansion() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        // Set HOME environment variable to temp directory
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
            std::env::set_var("NEOGHQ_ROOT", "~/test/repos"); // Use ~ to trigger expansion
        }

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = Some("main".to_string());

        let result = execute_get_command(url, branch);

        assert!(result.is_ok());

        // Verify directory structure was created in expanded path
        let expanded_path = temp_dir.path().join("test/repos/github.com/octocat/Hello-World");
        assert!(expanded_path.exists());
        assert!(expanded_path.join(".git").exists()); // bare repo
        assert!(expanded_path.join("main").exists()); // worktree
        assert!(expanded_path.join("main/README").exists()); // worktree content

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
            std::env::remove_var("HOME");
        }
    }

    #[test]
    fn test_execute_integration() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Set a temporary NEOGHQ_ROOT for this test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let url = "https://github.com/octocat/Hello-World.git";
        let branch = Some("main".to_string());

        let result = execute_get_command(url.to_string(), branch);

        assert!(result.is_ok());

        // Verify directory structure was created correctly
        let repo_path = temp_dir.path().join("github.com/octocat/Hello-World");
        assert!(repo_path.exists());
        assert!(repo_path.join(".git").exists()); // bare repo
        assert!(repo_path.join("main").exists()); // worktree
        assert!(repo_path.join("main/README").exists()); // worktree content

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_when_repository_already_exists() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Set a temporary NEOGHQ_ROOT for this test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = Some("main".to_string());

        // First execution - creates the repository
        let result1 = execute_get_command(url.clone(), branch.clone());
        assert!(result1.is_ok());

        // Second execution - repository already exists, should skip cloning but create worktree if needed
        let result2 = execute_get_command(url, branch);
        assert!(result2.is_ok());

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_execute_when_both_repo_and_worktree_exist() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Set a temporary NEOGHQ_ROOT for this test
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", temp_dir.path());
        }

        let url = "https://github.com/octocat/Hello-World.git".to_string();
        let branch = Some("main".to_string());

        // First execution - creates both repository and worktree
        let result1 = execute_get_command(url.clone(), branch.clone());
        assert!(result1.is_ok());

        // Second execution - both already exist, should skip both cloning and worktree creation
        let result2 = execute_get_command(url, branch);
        assert!(result2.is_ok());

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }
}

fn parse_repository_url(url: &str) -> Result<(String, String, String)> {
    use anyhow::anyhow;

    let url = url.strip_suffix(".git").unwrap_or(url);

    // Handle HTTPS URLs
    if url.starts_with("https://github.com/") {
        let path = url.strip_prefix("https://github.com/").unwrap();
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            return Ok((
                "github.com".to_string(),
                parts[0].to_string(),
                parts[1].to_string(),
            ));
        }
    }

    // Handle SSH URLs
    if url.starts_with("git@github.com:") {
        let path = url.strip_prefix("git@github.com:").unwrap();
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            return Ok((
                "github.com".to_string(),
                parts[0].to_string(),
                parts[1].to_string(),
            ));
        }
    }

    Err(anyhow!("Invalid URL format"))
}

fn resolve_repository_path(
    root: &str,
    host: &str,
    owner: &str,
    repo: &str,
    branch: &str,
) -> String {
    format!("{}/{}/{}/{}/{}", root, host, owner, repo, branch)
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

fn execute_get_command(url: String, branch: Option<String>) -> Result<()> {
    use std::path::Path;

    // Parse the repository URL to extract host, owner, and repo
    let (host, owner, repo) = parse_repository_url(&url)?;

    // Determine the branch to use (default to "main" if not specified)
    let branch = branch.unwrap_or_else(|| "main".to_string());

    // Get the neoghq root directory
    let root = std::env::var("NEOGHQ_ROOT").unwrap_or_else(|_| "~/src/repos".to_string());

    // Expand ~ to home directory if needed
    let root = if root.starts_with("~/") {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        root.replace("~", &home)
    } else {
        root
    };

    // Create repository and worktree paths
    let repo_dir = format!("{}/{}/{}/{}", root, host, owner, repo);
    let bare_repo_path = Path::new(&repo_dir).join(".git");
    let worktree_path = resolve_repository_path(&root, &host, &owner, &repo, &branch);

    // Clone the bare repository if it doesn't exist
    if !bare_repo_path.exists() {
        println!("Cloning {} into {}", url, bare_repo_path.display());
        clone_repository_bare(&url, &bare_repo_path)?;
    }

    // Create the worktree if it doesn't exist
    if !Path::new(&worktree_path).exists() {
        println!(
            "Creating worktree for branch '{}' in {}",
            branch, worktree_path
        );
        create_worktree(&bare_repo_path, Path::new(&worktree_path), &branch)?;
    }

    println!("Repository cloned successfully: {}", worktree_path);
    Ok(())
}
