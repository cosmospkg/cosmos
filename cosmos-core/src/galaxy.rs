use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use crate::star::Star;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::config::Config;
use crate::error::CosmosError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Galaxy {
    pub name: String,
    pub url: Option<String>,
    pub stars: HashMap<String, Star>, // name → star
    pub checksums: Option<HashMap<String, String>>, // name → checksum
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GalaxyMeta {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>, // e.g. "2025.01.01"
    pub stars: Option<HashMap<String, String>>, // name → version
    pub checksums: Option<HashMap<String, String>>, // name → checksum
}

#[derive(Clone, Copy, Debug)]
pub enum SyncLevel {
    MetaOnly,
    WithStars,
    Full,
}

#[derive(Debug, Error)]
pub enum GalaxyError {
    #[error("Galaxy not found: {0}")]
    Io(std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    Toml(toml::de::Error),
    #[error("Unsupported URL: {0}")]
    UnsupportedUrl(String),
}

impl From<std::io::Error> for GalaxyError {
    fn from(e: std::io::Error) -> Self {
        GalaxyError::Io(e)
    }
}

impl From<toml::de::Error> for GalaxyError {
    fn from(e: toml::de::Error) -> Self {
        GalaxyError::Toml(e)
    }
}

impl Galaxy {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: Some(url.to_string()),
            stars: HashMap::new(),
            checksums: None,
        }
    }

    pub fn is_local(&self) -> bool {
        if let Some(url) = &self.url {
            url.starts_with("file://") || url.starts_with("./") || url.starts_with("/")
        } else {
            true // if no URL, we assume it's local
        }
    }

    pub fn add_star(&mut self, star: Star) {
        self.stars.insert(star.name.clone(), star);
    }

    pub fn get_star(&self, name: &str) -> Option<&Star> {
        self.stars.get(name)
    }

    pub fn find_star(&self, name: &str, version: &str) -> Option<&Star> {
        self.stars.get(name).and_then(|star| {
            if star.version == version {
                Some(star)
            } else {
                None
            }
        })
    }

    pub fn has_star_named(&self, name: &str) -> bool {
        self.stars.contains_key(name)
    }

    pub fn load_all_from_config(config: &Config, offline: bool) -> Result<Vec<Galaxy>, GalaxyError> {
        let mut galaxies = vec![];

        for (name, url) in &config.galaxies {
            if url.starts_with("file://") || url.starts_with("/") || url.starts_with("./") {
                let mut url = url.clone();
                if url.starts_with("file://") {
                    url = url[7..].to_string(); // Remove "file://" prefix
                }

                let path = Path::new(&url);
                let galaxy = Galaxy::load(path, Some(url.clone()), offline)?;
                galaxies.push(galaxy);
            } else {
                let galaxy_cache_path = Path::new(&config.cache_dir).join("galaxies").join(name);

                if galaxy_cache_path.exists() {
                    let galaxy = Galaxy::load(&galaxy_cache_path, Some(url.clone()), offline)?;
                    galaxies.push(galaxy);
                } else {
                    eprintln!(
                        "⚠️  Galaxy cache missing for '{}'. Did you forget to run `cosmos sync`?",
                        name
                    );
                }
            }
        }

        Ok(galaxies)
    }

    pub fn load(galaxy_path: &Path, url: Option<String>, offline: bool) -> Result<Galaxy, GalaxyError> {
        let name = galaxy_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Invalid galaxy folder name"))?
            .to_string();

        let meta_path = galaxy_path.join("meta.toml");
        let meta_content = fs::read_to_string(&meta_path)?;
        let meta: GalaxyMeta = toml::from_str(&meta_content)?;

        let stars_path = galaxy_path.join("stars");
        let mut stars = HashMap::new();

        // fill stars with meta.stars first
        if let Some(declared) = &meta.stars {
            for (name, version) in declared.iter() {
                let star_path = stars_path.join(format!("{}.toml", name));
                if star_path.exists() {
                    let content = fs::read_to_string(&star_path)?;
                    match toml::from_str::<Star>(&content) {
                        Ok(star) => {
                            if &star.version == version {
                                stars.insert(star.name.clone(), star);
                            } else {
                                eprintln!(
                                    "⚠️  Version mismatch: {} expected {}, got {}",
                                    name, version, star.version
                                );
                            }
                        }
                        Err(err) => {
                            eprintln!("❌ Could not parse star file '{}': {}", star_path.display(), err);
                        }
                    }
                } else if !offline {
                    // we need to download the star file from remote
                    if let Some(url) = &url {
                        let star_url = format!("{}/stars/{}.toml", url.trim_end_matches('/'), name);
                        let star_dest = stars_path.join(format!("{}.toml", name));
                        Self::download_file(&star_url, &star_dest)?;
                        let content = fs::read_to_string(&star_dest)?;
                        match toml::from_str::<Star>(&content) {
                            Ok(star) => {
                                if &star.version == version {
                                    stars.insert(star.name.clone(), star);
                                } else {
                                    eprintln!(
                                        "⚠️  Version mismatch: {} expected {}, got {}",
                                        name, version, star.version
                                    );
                                }
                            }
                            Err(err) => {
                                eprintln!("❌ Could not parse downloaded star file '{}': {}", star_dest.display(), err);
                            }
                        }
                    }
                } else {
                    eprintln!("⚠️ Star file for '{}' not found in offline mode ({}). Did you forget to run `cosmos sync --stars`?", name, meta.name);

                }
            }
        }

        Ok(Galaxy {
            name,
            url,
            stars,
            checksums: meta.checksums,
        })
    }

    pub fn sync_all_from_config(config: &Config, level: SyncLevel) -> Result<(), GalaxyError> {
        for (name, url) in &config.galaxies {
            Galaxy::sync_from_remote(name, url, Path::new(&config.cache_dir), level)?;
        }
        Ok(())
    }

    pub fn sync_from_remote(
        name: &str,
        url: &str,
        cache_dir: &Path,
        level: SyncLevel,
    ) -> Result<(), GalaxyError> {
        if url.starts_with("https://") {
            return Err(GalaxyError::UnsupportedUrl(url.to_string()));
        }

        if url.starts_with("file://") || url.starts_with("./") || url.starts_with("/") {
            println!("⭐ Skipping sync for local Galaxy '{}'", name);
            return Ok(());
        }

        let galaxy_cache_dir = cache_dir.join("galaxies").join(name);
        let stars_dir = galaxy_cache_dir.join("stars");
        let packages_dir = galaxy_cache_dir.join("packages");

        fs::create_dir_all(&stars_dir)?;
        fs::create_dir_all(&packages_dir)?;

        let meta_url = format!("{}/meta.toml", url.trim_end_matches('/'));
        let meta_dest = galaxy_cache_dir.join("meta.toml");
        Self::download_file(&meta_url, &meta_dest)?;

        if matches!(level, SyncLevel::MetaOnly) {
            return Ok(());
        }

        let meta_content = fs::read_to_string(&meta_dest)?;
        let meta: GalaxyMeta = toml::from_str(&meta_content)?;

        let star_names: Vec<String> = meta.stars
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();

        for name in star_names {
            let star_url = format!("{}/stars/{}.toml", url.trim_end_matches('/'), name);
            let star_dest = stars_dir.join(format!("{}.toml", name));
            Self::download_file(&star_url, &star_dest)?;

            if matches!(level, SyncLevel::Full) {
                let star_contents = fs::read_to_string(&star_dest)?;
                let star: Star = toml::from_str(&star_contents)?;
                if let Some(source) = &star.source {
                    let mut source = source.clone();
                    if source.starts_with("./") || source.starts_with("/") {
                        let base_path = Path::new(url);
                        let stripped = source.trim_start_matches("./").trim_start_matches('/');
                        source = base_path.join(stripped)
                            .to_string_lossy()
                            .to_string()
                    }
                    let source = source.trim_end_matches('/');
                    println!("🔄 Syncing star source for '{}' ('{}')", star.name, source);
                    if cosmos_transport::supports_url(source) {
                        let filename = format!("{}-{}.tar.gz", star.name, star.version);
                        let tar_dest = packages_dir.join(filename);
                        Self::download_file(source, &tar_dest)?;
                    } else if source.starts_with("file://") || Path::new(source).exists() {
                        println!("⭐ Local source detected for star '{}'", star.name);
                    } else {
                        eprintln!("⚠️  Unsupported source format for star '{}'", star.name);
                    }
                }
            }
        }

        println!("✅ Synced Galaxy '{}'", name);

        Ok(())
    }

    fn download_file(url: &str, dest: &Path) -> Result<(), GalaxyError> {
        println!("📥 Downloading: {}", url);
        let response = cosmos_transport::fetch_bytes(url)
            .map_err(|e| GalaxyError::UnsupportedUrl(format!("Failed to download: {}", e)))?;

        let mut file = File::create(dest)?;
        std::io::copy(&mut response.as_slice(), &mut file)
            .map_err(|e| GalaxyError::UnsupportedUrl(format!("Failed to write file: {}", e)))?;

        println!("✅ Saved to: {}", dest.display());
        Ok(())
    }

    pub fn resolve_source_path(&self, source: &str, config: &Config) -> Result<PathBuf, CosmosError> {
        if source.starts_with("file://") {
            return Ok(Path::new(&source[7..]).to_path_buf());
        }

        let root = if self.is_local() {
            let url = self.url.as_ref().ok_or_else(|| CosmosError::MissingField(format!(
                "Local Galaxy '{}' missing URL. Define its path in [galaxies] config.",
                self.name
            )))?;
            Path::new(url).to_path_buf()
        } else {
            Path::new(&config.cache_dir).join("galaxies").join(&self.name)
        };

        Ok(root.join(source))
    }
}
