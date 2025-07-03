use anyhow::Result;
use std::path::PathBuf;

const DEFAULT_NEOGHQ_ROOT: &str = "~/src/repos";

#[derive(Debug, Clone)]
pub struct Env {
    pub neoghq_root: Option<PathBuf>,
    pub home: Option<PathBuf>,
}

impl Env {
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn load() -> Result<Self> {
        let neoghq_root = std::env::var("NEOGHQ_ROOT")
            .ok()
            .map(|path| PathBuf::from(path));
        let home = dirs::home_dir();

        Ok(Self { neoghq_root, home })
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub root: PathBuf,
}

impl Config {
    pub fn load(env: Env) -> Result<Self> {
        let home_dir = env.home;

        // Get the neoghq root directory
        let root = env
            .neoghq_root
            .unwrap_or_else(|| PathBuf::from(DEFAULT_NEOGHQ_ROOT));

        // Expand the root path if it contains a tilde
        let root = if root.starts_with("~") {
            if let Some(home_dir) = &home_dir {
                let expanded_path = home_dir.join(
                    // this unwrap is safe because we checked that root starts with "~"
                    root.strip_prefix("~").unwrap(),
                );
                expanded_path.canonicalize()?
            } else {
                root.canonicalize()?
            }
        } else {
            root.canonicalize()?
        };

        Ok(Self { root })
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_config_load_with_neoghq_root() {
        let temp_dir = tempfile::tempdir().unwrap();
        let env = Env {
            neoghq_root: Some(temp_dir.path().to_path_buf()),
            home: None,
        };
        let config = Config::load(env).unwrap();

        assert_eq!(config.root, temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_config_load_with_default_root() {
        let temp_dir = tempfile::tempdir().unwrap();
        let src_repos_dir = temp_dir.path().join("src/repos");
        std::fs::create_dir_all(&src_repos_dir).unwrap();

        let env = Env {
            neoghq_root: None,
            home: Some(temp_dir.path().to_path_buf()),
        };
        let config = Config::load(env).unwrap();

        assert_eq!(config.root, src_repos_dir.canonicalize().unwrap());
    }

    #[test]
    fn test_config_load_with_home_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let neoghq_dir = temp_dir.path().join("neoghq");
        std::fs::create_dir_all(&neoghq_dir).unwrap();

        let env = Env {
            neoghq_root: Some(PathBuf::from("~/neoghq")),
            home: Some(temp_dir.path().to_path_buf()),
        };
        let config = Config::load(env).unwrap();

        assert_eq!(config.root, neoghq_dir.canonicalize().unwrap());
    }
}
