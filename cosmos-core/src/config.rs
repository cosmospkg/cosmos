use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub galaxies: HashMap<String, String>, // name → url or path
    pub install_dir: String,
    pub cache_dir: String
    // TODO (Phase 2/3): strict_mode: bool, // for GalaxyMeta version mismatch
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(path)?;
        let parsed: Self = toml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(parsed)
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let content = toml::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn create_default(path: &str) -> Result<(), std::io::Error> {
        let mut galaxies = HashMap::new();
        galaxies.insert("core".to_string(), "file:///mnt/usb/galaxies/core".to_string());
        let default_config = Config {
            galaxies,
            install_dir: "/".to_string(),
            cache_dir: "/var/cache/cosmos".to_string(),
        };
        default_config.save(path)
    }
}