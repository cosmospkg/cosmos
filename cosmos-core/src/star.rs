use std::collections::HashMap;
use std::fs;
use std::path::Path;
use semver::Version;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::error::CosmosError;
use crate::galaxy::Galaxy;
use crate::resolver::calculate_checksum;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Star {
    pub name: String,
    pub version: String,
    pub authors: HashMap<String, String>,
    #[serde(rename = "type")]
    pub star_type: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
    pub install_script: Option<String>,
    pub source: Option<String>,
    pub checksums: Option<HashMap<String, String>>
}

impl Star {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let parsed: Self = toml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(parsed)
    }

    pub fn get_dependencies(&self) -> Vec<(String, String)> {
        self.dependencies
            .as_ref()
            .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }

    pub fn uses_nova(&self) -> bool {
        if let Some(script) = &self.install_script {
            script.ends_with(".lua") || script.ends_with(".nova")
        } else {
            false
        }
    }

    pub fn validate_checksums(&self, extracted_path: &Path) -> Result<bool, CosmosError> {
        if let Some(checksums) = &self.checksums {
            for (file, checksum) in checksums {
                // we need to insert the files directory into the path
                // as it is saved relative to the files directory in the star.toml
                // so that we can calculate the checksum

                let file_path = Path::new(extracted_path).join("files").join(file);
                if !file_path.exists() {
                    eprintln!("❌ File not found: {}", &file_path.display());
                    return Err(CosmosError::FileNotFound(format!("File not found: {}", file)));
                }

                let canonical = file_path.canonicalize()?;
                if !canonical.starts_with(extracted_path.join("files")) {
                    return Err(CosmosError::SecurityError(format!("Illegal path: {}", file)));
                }


                // verify that checksum is a valid sha256 hex string

                let checksum = checksum.to_lowercase();

                if checksum.len() != 64 || !checksum.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err(CosmosError::InvalidChecksum(format!("Invalid checksum: {}", checksum)));
                }

                println!("🔍 Verifying checksum for {}: {}", file, checksum);
                let file_checksum = calculate_checksum(&file_path)
                    .map_err(|e| CosmosError::ChecksumFailed(format!("Failed to calculate checksum: {}", e)))?;
                if file_checksum != *checksum {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}

pub fn compare_versions(current: &str, other: &str) -> Result<std::cmp::Ordering, CosmosError> {
    let self_version = Version::parse(current)
        .map_err(|e| CosmosError::SemverError(format!("Failed to parse version: {}", e)))?;

    let other_version = Version::parse(other)
        .map_err(|e| CosmosError::SemverError(format!("Failed to parse version: {}", e)))?;

    Ok(self_version.cmp(&other_version))
}

pub fn fetch_star(
    galaxy: &Galaxy,
    star_name: &str,
    config: &Config,
    offline: bool,
) -> Result<Star, CosmosError> {
    // 1. Already loaded in memory?
    if let Some(star) = galaxy.get_star(star_name) {
        return Ok(star.clone());
    }

    // 2. Local galaxy = must be file-based
    if galaxy.is_local() {
        let local_path = Path::new(
            galaxy.url.as_ref().ok_or_else(|| CosmosError::MissingField("Missing galaxy URL for local repo".to_string()))?
        )
            .join("stars")
            .join(format!("{}.toml", star_name));

        let content = fs::read_to_string(&local_path)?;
        return Ok(toml::from_str(&content)?);
    }

    // 3. Check cached star.toml
    let star_path = Path::new(&config.cache_dir)
        .join("galaxies")
        .join(&galaxy.name)
        .join("stars")
        .join(format!("{}.toml", star_name));

    if star_path.exists() {
        let content = fs::read_to_string(&star_path)?;
        return Ok(toml::from_str(&content)?);
    }

    // 4. Remote download if allowed
    if offline {
        return Err(CosmosError::DownloadFailed(format!(
            "Star '{}' not cached and offline mode is enabled",
            star_name
        )));
    }

    let base_url = galaxy.url.as_ref().ok_or_else(|| {
        CosmosError::MissingField(format!("Missing Galaxy URL for '{}'.", galaxy.name))
    })?;

    if base_url.starts_with("https://") {
        return Err(CosmosError::DownloadFailed("HTTPS is not supported".to_string()));
    }

    let url = format!("{}/stars/{}.toml", base_url.trim_end_matches('/'), star_name);
    println!("🌐 Downloading star metadata: {}", url);

    let response = cosmos_transport::fetch_bytes(&url)
        .map_err(|e| CosmosError::DownloadFailed(format!("Failed to download: {}", e)))?;

    let content = String::from_utf8(response)
        .map_err(|e| CosmosError::DownloadFailed(format!("Failed to parse response: {}", e)))?;

    fs::create_dir_all(star_path.parent().unwrap())?;
    fs::write(&star_path, &content)?;

    Ok(toml::from_str(&content)?)
}