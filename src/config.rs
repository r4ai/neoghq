use anyhow::Result;

const DEFAULT_NEOGHQ_ROOT: &str = "~/src/repos";

#[derive(Debug, Clone)]
pub struct Env {
    pub neoghq_root: Option<String>,
}

impl Env {
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn load() -> Result<Self> {
        let neoghq_root = std::env::var("NEOGHQ_ROOT").ok();

        Ok(Self { neoghq_root })
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub root: String,
}

impl Config {
    pub fn load(env: Env) -> Result<Self> {
        // Get the neoghq root directory
        let root = env
            .neoghq_root
            .unwrap_or_else(|| DEFAULT_NEOGHQ_ROOT.to_string());

        Ok(Self { root })
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_config_load_with_neoghq_root() {
        let env = Env {
            neoghq_root: Some("/custom/path".to_string()),
        };
        let config = Config::load(env).unwrap();
        assert_eq!(config.root, "/custom/path");
    }

    #[test]
    fn test_config_load_with_default_root() {
        let env = Env { neoghq_root: None };
        let config = Config::load(env).unwrap();
        assert_eq!(config.root, DEFAULT_NEOGHQ_ROOT);
    }
}
