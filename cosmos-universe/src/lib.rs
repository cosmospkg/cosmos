use std::collections::HashMap;
use std::fs;
use std::path::Path;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Universe {
    pub system: SystemInfo,
    pub installed: HashMap<String, InstalledStar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub arch: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstalledStar {
    pub version: String,
    pub files: Vec<String>,
    pub name: String,
}

impl Universe {
    /// Load a universe from a TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let parsed = toml::from_str(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(parsed)
    }

    /// Save the universe to a TOML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let toml = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, toml)?;
        Ok(())
    }

    /// Check if a star is installed (optionally with a version constraint)
    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }

    /// Insert or update a star
    pub fn record_star(&mut self, name: &str, version: &str, files: Vec<String>) {
        self.installed.insert(
            name.to_string(),
            InstalledStar {
                name: name.to_string(),
                version: version.to_string(),
                files,
            },
        );
    }

    /// Remove a star from the universe
    pub fn uninstall_star(&mut self, name: &str) {
        self.installed.remove(name);
    }

    pub fn satisfies(&self, name: &str, constraint: &str) -> bool {
        let Some(star) = self.installed.get(name) else {
            return false;
        };

        let Ok(version) = Version::parse(&star.version) else {
            return false;
        };

        let Ok(requirement) = VersionReq::parse(constraint) else {
            return false;
        };

        requirement.matches(&version)
    }
}
