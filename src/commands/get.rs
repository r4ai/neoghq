use anyhow::Result;

pub fn execute(url: String, branch: Option<String>) -> Result<()> {
    println!("Get command: {} {:?}", url, branch);
    Ok(())
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
    fn test_resolve_repository_path() {
        let root = "/tmp/neoghq";
        let host = "github.com";
        let owner = "user";
        let repo = "repo";
        let branch = "main";
        
        let result = resolve_repository_path(root, host, owner, repo, branch);
        
        assert_eq!(result, "/tmp/neoghq/github.com/user/repo/main");
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
            return Ok(("github.com".to_string(), parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    // Handle SSH URLs
    if url.starts_with("git@github.com:") {
        let path = url.strip_prefix("git@github.com:").unwrap();
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            return Ok(("github.com".to_string(), parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    Err(anyhow!("Invalid URL format"))
}

fn resolve_repository_path(root: &str, host: &str, owner: &str, repo: &str, branch: &str) -> String {
    format!("{}/{}/{}/{}/{}", root, host, owner, repo, branch)
}