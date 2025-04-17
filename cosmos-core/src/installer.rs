use std::fs;
use std::io::Seek;
use std::path::Path;
use flate2::read::GzDecoder;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use tar::Archive;
use cosmos_transport::supports_url;
use crate::{star::Star, galaxy::Galaxy, config::Config, universe::record_install, error::CosmosError, resolver};

use cosmos_universe::Universe;
use crate::resolver::calculate_checksum;

pub fn install_star(
    star: &Star,
    origin: &Galaxy,
    universe: &mut Universe,
    galaxies: &[Galaxy],
    config: &Config,
    offline: bool,
) -> Result<(), CosmosError> {
    println!("⭐ Installing star: {} {}", star.name, star.version);

    for (dep_name, constraint) in star.get_dependencies() {
        if universe.satisfies(&dep_name, &constraint) {
            continue;
        }

        let (dep_star, dep_galaxy) = resolver::find_star(galaxies, &dep_name, &constraint)
            .ok_or_else(|| CosmosError::DependencyError(format!(
                "Dependency '{}' not found in any Galaxy", dep_name
            )))?;

        install_star(dep_star, dep_galaxy, universe, galaxies, config, offline)?;
    }

    if star.star_type.as_deref() == Some("nebula") || star.star_type.as_deref() == Some("meta") {
        println!("🌀 Nebula '{}' does not extract files or run scripts. Installation has been logged", star.name);
        record_install(universe, star, vec![]);
        return Ok(());
    }

    let filename = format!("{}-{}.tar.gz", star.name, star.version);

    let mut tarball_path = Path::new(&config.cache_dir)
        .join("galaxies")
        .join(&origin.name)
        .join("packages")
        .join(&filename);

    if !tarball_path.exists() {
        if let Some(source) = &star.source {
            let mut resolved_source = source.clone();

            if source.starts_with("./") || source.starts_with("/") {
                if let Some(base) = &origin.url {
                    let base_path = Path::new(base);
                    let stripped = source.trim_start_matches("./").trim_start_matches('/');
                    resolved_source = base_path.join(stripped).to_string_lossy().to_string();
                }
            }

            let resolved = if cosmos_transport::supports_url(&resolved_source) {
                if offline {
                    return Err(CosmosError::DownloadFailed(format!(
                        "Missing tarball for '{}' and offline mode is enabled",
                        star.name
                    )));
                }
                println!("🌐 Downloading tarball: {}", resolved_source);
                let response = cosmos_transport::fetch_bytes(&resolved_source)
                    .map_err(|e| CosmosError::DownloadFailed(format!("Failed to download: {}", e)))?;

                fs::create_dir_all(tarball_path.parent().unwrap())?;
                let mut file = fs::File::create(&tarball_path)?;
                std::io::copy(&mut response.as_slice(), &mut file)?;
                None
            } else if resolved_source.starts_with("file://") || Path::new(&resolved_source).exists() {
                let path = origin.resolve_source_path(&resolved_source, config)?;
                if !path.exists() {
                    return Err(CosmosError::DownloadFailed(format!(
                        "Local source path does not exist: {}",
                        path.display()
                    )));
                }
                Some(path)
            } else {
                return Err(CosmosError::DownloadFailed(format!(
                    "Unsupported source format for '{}': '{}'",
                    star.name, resolved_source
                )));
            };

            if let Some(source_path) = resolved {
                println!("⭐ Using local tarball at: {}", source_path.display());
                tarball_path = source_path;
            }
        } else {
            return Err(CosmosError::MissingField(format!(
                "Star '{}' has no source and no tarball cached",
                star.name
            )));
        }
    }

    // verify checksum of tarball (if applicable)
    if let Some(checksums) = &origin.checksums {
        println!("🔍 Verifying checksum for '{}'", filename);
        if let Some(expected) = checksums.get(&star.name) {
            let actual = calculate_checksum(&tarball_path)
                .map_err(|e| CosmosError::ChecksumFailed(format!("Checksum calculation failed: {}", e)))?;

            if actual != *expected {
                return Err(CosmosError::ChecksumFailed(format!(
                    "Checksum mismatch for '{}': expected {}, got {}",
                    star.name, expected, actual
                )));
            } else {
                println!("🔒 Checksum verified for '{}'", filename);
            }
        } else {
            println!("⚠️ No checksum found for '{}'", filename);
        }
    } else {
        println!("⚠️ No checksum validation for '{}'", filename);
    }

    let temp_dir = tempfile::tempdir()?;
    let extracted_files = extract_star(temp_dir.path(), &tarball_path)?;
    let mut installed_files: Vec<String> = vec![];

    star.validate_checksums(temp_dir.path())
        .map_err(|e| CosmosError::ChecksumFailed(format!("Checksum validation failed: {}", e)))?;

    if let Some(script) = &star.install_script {
        let full_script = temp_dir.path().join(script);

        if script.ends_with(".lua") || script.ends_with(".nova") {
            println!("🔧 Running Nova install script: {}", script);
            nova::run_nova_script(full_script.to_str().unwrap(), temp_dir.path(), Path::new(&config.install_dir), &mut installed_files)?;
        } else {
            println!("🔧 Running shell install script: {}", script);
            run_install_script(full_script.to_str().unwrap(), temp_dir.path())?;
            installed_files = extracted_files.clone();
        }
    } else {
        let source_dir = temp_dir.path().join("files");
        if source_dir.exists() {
            println!("📁 No install script. Copying files/* to {}", config.install_dir);
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.copy_inside = true;

            let entries = fs::read_dir(&source_dir)?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()?;

            copy_items(&entries, Path::new(&config.install_dir), &options)
                .map_err(|e| CosmosError::CopyFailed(format!("Failed to copy files: {}", e)))?;

            for entry in entries {
                let path = entry;
                let relative_path = path.strip_prefix(&source_dir)
                    .map_err(|e| CosmosError::CopyFailed(format!("Failed to strip prefix: {}", e)))?;
                let full_path = Path::new(&config.install_dir).join(relative_path);
                installed_files.push(format!("/{}", full_path.to_string_lossy()));
            }
        } else {
            println!("⚠️  No install script and no files/ directory. Nothing to do.");
        }
    }

    record_install(universe, star, installed_files);
    println!("✅ Installed: {}", star.name);
    Ok(())
}

pub fn extract_star(
    temp_dir: &Path,
    tarball_path: &Path
) -> Result<Vec<String>, CosmosError> {
    let mut file = fs::File::open(tarball_path)?;
    file.rewind()?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    let mut installed_files = vec![];
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();
        let full_path = temp_dir.join(path.clone());
        entry.unpack(&full_path)?;
        installed_files.push(
            format!("/{}", path.to_string_lossy().trim_start_matches("./"))
        );
    }

    Ok(installed_files)
}

fn run_install_script(script: &str, temp_dir: &Path) -> Result<(), CosmosError> {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .current_dir(temp_dir)
        .status()?;

    if !status.success() {
        return Err(CosmosError::ScriptFailed(script.into()));
    }

    Ok(())
}

pub fn uninstall_star(name: &str, universe: &mut Universe, install_root: &Path) -> Result<(), CosmosError> {
    let installed = universe.installed.get(name)
        .ok_or_else(|| CosmosError::MissingField(format!("Star '{}' is not installed", name)))?;

    println!("🗑️  Uninstalling star: {} {}", name, installed.version);

    for file in &installed.files {
        let path = install_root.join(&file[1..]);
        if path.exists() {
            println!("  - Removing {}", path.display());
            fs::remove_file(path)?;
        } else {
            println!("  - Skipped missing {}", path.display());
        }
    }

    universe.installed.remove(name);
    println!("❌ Uninstalled: {}", name);
    Ok(())
}