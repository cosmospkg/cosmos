use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::process;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;
use fs_extra::copy_items;
use tempfile::tempdir;
use fs_extra::dir::CopyOptions;
use cosmos_core::star::Star;
use dialoguer::{Input, Select};
use cosmos_core::resolver::calculate_checksum;

pub fn build_star(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(path);
    let star_path = dir.join("star.toml");

    if !star_path.exists() {
        eprintln!("❌ Error: star.toml not found in {}", dir.display());
        process::exit(1);
    }

    let star_str = fs::read_to_string(&star_path)?;
    let mut star: Star = toml::from_str(&star_str)?;

    let install_lua = dir.join("install.lua");
    let install_sh = dir.join("install.sh");

    println!("🔍 Found star definition: {}", star_path.display());
    if install_lua.exists() {
        println!("🔍 Found Lua install script: {}", install_lua.display());
    }
    if install_sh.exists() {
        println!("🔍 Found shell install script: {}", install_sh.display());
    }
    if dir.join("files").exists() {
        println!("📁 Found files directory: {}", dir.join("files").display());
    }

    if install_lua.exists() && install_sh.exists() {
        eprintln!("❌ Error: Both install.lua and install.sh exist. Please use only one.");
        process::exit(1);
    }

    // ask user if they want to include checksum
    let include_checksum: bool = Input::new()
        .with_prompt("🔍 Include checksum?")
        .default(true)
        .interact()?;

    let temp = tempdir()?;
    let staging = temp.path();

    if install_lua.exists() {
        nova::run_nova_build_script(install_lua.to_str().unwrap(), staging, staging)
            .map_err(|e| format!("Nova build error: {:?}", e))?;
    }

    let files_dir = dir.join("files");
    if files_dir.exists() {
        let mut checksums: HashMap<String, String> = HashMap::new();

        // if the user wants to include checksums, calculate them, save the name as relative path
        // not full staging path

        let entries = fs::read_dir(&files_dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        if include_checksum {
            for entry in &entries {
                let relative_path = entry.strip_prefix(&files_dir).unwrap();
                let checksum = calculate_checksum(entry)?;
                checksums.insert(relative_path.to_string_lossy().to_string(), checksum);
            }
            star.checksums = Some(checksums);
        }

        let mut opts = CopyOptions::new();
        opts.copy_inside = true;
        opts.overwrite = true;

        copy_items(&entries, staging, &opts)?;
    }

    if install_lua.exists() {
        let target = staging.join("install.lua");
        println!("📁 Copying Lua script → {}", target.display());
        fs::copy(&install_lua, &target)?;
    } else if install_sh.exists() {
        let target = staging.join("install.sh");
        println!("📁 Copying shell script → {}", target.display());
        fs::copy(&install_sh, &target)?;
    }

    fs::create_dir_all("dist")?;
    let tar_path = format!("dist/{}-{}.tar.gz", star.name, star.version);
    let tar_gz = File::create(&tar_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all(".", staging)?;
    tar.finish()?;

    println!("✅ Successfully built star package: {}", tar_path);
    Ok(())
} 