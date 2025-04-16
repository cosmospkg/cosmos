use std::fs;
use std::path::Path;
use std::collections::HashMap;
use cosmos_core::star::Star;
use cosmos_core::galaxy::GalaxyMeta;
use toml;
use cosmos_core::resolver::calculate_checksum;

pub fn galaxy_init(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("{}-galaxy", name);
    let dir = Path::new(&path);

    if dir.exists() {
        return Err("❌ Galaxy directory already exists".into());
    }

    fs::create_dir_all(dir.join("stars"))?;
    fs::create_dir_all(dir.join("packages"))?;

    // prompt for if using checksum, use dialoguer
    let use_checksum: bool = dialoguer::Confirm::new()
        .with_prompt("Enable global checksums for star sources?")
        .default(true)
        .interact()?;

    //().format("%Y.%m.%d").to_string();
    let today = chrono::Utc::now().format("%Y.%m.%d").to_string();
    let galaxy = GalaxyMeta {
        name: name.to_string(),
        description: Some(String::new()),
        version: Some(today),
        stars: Some(HashMap::new()),
        checksums: if use_checksum {
            Some(HashMap::new())
        } else {
            None
        },
    };

    let meta_str = toml::to_string_pretty(&galaxy)?;
    fs::write(dir.join("meta.toml"), meta_str)?;

    println!("✅ Initialized new galaxy at: {}", path);
    Ok(())
}

pub fn index_galaxy(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(path);
    let stars_dir = root.join("stars");
    let meta_path = root.join("meta.toml");

    let star_files = fs::read_dir(&stars_dir)?
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().map_or(false, |ext| ext == "toml"))
        .collect::<Vec<_>>();

    if star_files.is_empty() {
        println!("⚠️  No star TOMLs found in stars/ directory.");
    } else {
        println!("🔍 Found {} star definitions.", star_files.len());
    }

    let mut versions = HashMap::new();
    for entry in star_files {
        let path = entry.path();
        let content = fs::read_to_string(&path)?;
        let star: Star = toml::from_str(&content)?;
        println!("  - Indexed {} v{}", star.name, star.version);
        versions.insert(star.name.clone(), star.version.clone());
    }

    let use_checksum: bool = dialoguer::Confirm::new()
        .with_prompt("Do you want to run checksum validation?")
        .default(true)
        .interact()?;

    let mut checksums: HashMap<String, String> = HashMap::new();

    if use_checksum {
        let packages_dir = root.join("packages");
        // use star meta to get checksums of packages
        for (name, version) in &versions {
            let package_path = packages_dir.join(format!("{}-{}.tar.gz", name, version));
            if !package_path.exists() {
                println!("⚠️  Package {} v{} not found in packages/ directory. Is this a nebulae?", name, version);
                continue;
            }

            let checksum = calculate_checksum(&package_path)?;
            println!("🔒 Validated checksum for {} (v{}): {}", name, version, checksum);
            checksums.insert(name.to_string(), checksum);
        }
    }


    let mut meta: GalaxyMeta = toml::from_str(&fs::read_to_string(&meta_path)?)?;
    meta.stars = Some(versions);
    if use_checksum {
        meta.checksums = Some(checksums);
    } else {
        meta.checksums = None;
    }

    let updated = toml::to_string_pretty(&meta)?;
    fs::write(&meta_path, updated)?;

    println!("✅ Updated galaxy index at: {}", path);
    Ok(())
}