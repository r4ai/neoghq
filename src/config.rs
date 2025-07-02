use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Config {
    pub root: String,
}

impl Config {
    pub fn new(root: String) -> Self {
        Self { root }
    }

    pub fn load() -> Result<Self> {
        // Get the neoghq root directory
        let root = std::env::var("NEOGHQ_ROOT").unwrap_or_else(|_| "~/src/repos".to_string());

        // Expand ~ to home directory if needed
        let root = if root.starts_with("~/") {
            let home = std::env::var("HOME")
                .map_err(|_| anyhow::anyhow!("HOME environment variable is not set"))?;
            root.replace("~", &home)
        } else {
            root
        };

        Ok(Self::new(root))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new("/tmp/test".to_string());
        assert_eq!(config.root, "/tmp/test");
    }

    #[test]
    fn test_config_load_with_neoghq_root() {
        // Set NEOGHQ_ROOT environment variable
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", "/custom/root");
        }

        let config = Config::load().unwrap();
        assert_eq!(config.root, "/custom/root");

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

    #[test]
    fn test_config_load_with_default_root() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Set HOME and ensure NEOGHQ_ROOT is not set
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
            std::env::remove_var("NEOGHQ_ROOT");
        }

        let config = Config::load().unwrap();
        let expected = format!("{}/src/repos", temp_dir.path().display());
        assert_eq!(config.root, expected);

        // Clean up
        unsafe {
            std::env::remove_var("HOME");
        }
    }

    #[test]
    fn test_config_load_with_home_expansion() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Set HOME and NEOGHQ_ROOT with ~ prefix
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
            std::env::set_var("NEOGHQ_ROOT", "~/custom/repos");
        }

        let config = Config::load().unwrap();
        let expected = format!("{}/custom/repos", temp_dir.path().display());
        assert_eq!(config.root, expected);

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
            std::env::remove_var("HOME");
        }
    }

    #[test]
    fn test_config_load_home_expansion_error() {
        // Test the error case when HOME is not set but ~ expansion is needed
        unsafe {
            std::env::set_var("NEOGHQ_ROOT", "~/test/path");
            std::env::remove_var("HOME");
        }

        let result = Config::load();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("HOME environment variable is not set"));

        // Clean up
        unsafe {
            std::env::remove_var("NEOGHQ_ROOT");
        }
    }

}