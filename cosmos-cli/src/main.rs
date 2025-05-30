use std::collections::HashMap;
use std::fs;
use std::path::Path;
use clap::{arg, Parser, Subcommand};
use cosmos_core::config::Config;
use cosmos_core::galaxy::{Galaxy, SyncLevel};
use cosmos_core::installer::{install_star, uninstall_star};
use cosmos_core::resolver;
use cosmos_core::star::fetch_star;
use cosmos_universe::{SystemInfo, Universe};

#[derive(Parser)]
#[command(
    name = "cosmos",
    version,
    about = "A minimal, offline-first package manager",
    long_about = r#"

        __________  _____ __  _______  _____
       / ____/ __ \/ ___//  |/  / __ \/ ___/
      / /   / / / /\__ \/ /|_/ / / / /\__ \
     / /___/ /_/ /___/ / /  / / /_/ /___/ /
     \____/\____//____/_/  /_/\____//____/
          Package manager for entropy.

    "#,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a star or constellation
    Install {
        /// Name of the star to install
        #[arg(conflicts_with = "constellation")]
        name: Option<String>,

        /// Path to a constellation file
        #[arg(long)]
        constellation: Option<String>,

        /// Don't connect to remote galaxies (only use cache + local files)
        #[arg(long)]
        offline: bool,

        /// Root directory for installation (default: /)
        #[arg(long)]
        root: Option<String>,
    },

    /// Uninstall a star
    Uninstall {
        /// Name of the star to uninstall
        name: String,

        /// Root directory for uninstallation (default: /)
        #[arg(long)]
        root: Option<String>,
    },

    /// Update a star
    Update {
        /// Name of the star to update
        name: String,

        /// Don't connect to remote galaxies (only use cache + local files)
        #[arg(long)]
        offline: bool,

        /// Root directory for update (default: /)
        #[arg(long)]
        root: Option<String>,
    },

    /// Show the status of installed stars
    Status,

    /// Sync galaxies and stars
    Sync {
        /// Sync only stars
        #[arg(conflicts_with = "full", long)]
        stars: bool,

        /// Sync all stars and packages
        #[arg(long)]
        full: bool,
    },

    /// Show information about a star
    Show {
        name: String,
    },

    /// Search for stars
    Search {
        term: String,
    },

    /// Initialize the cosmos config and universe
    Init {
        #[arg(long)]
        root: Option<String>,
    },

    /// Add a galaxy to the config
    AddGalaxy {
        name: String,
        url: String,

        #[arg(long)]
        root: Option<String>,
    },

    /// Remove a galaxy from the config
    RemoveGalaxy {
        name: String,

        #[arg(long)]
        root: Option<String>,
    },

    /// List all galaxies in the config
    ListGalaxies {
        #[arg(long)]
        root: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { name, constellation, offline, root } => {
            let root_path = Path::new(root.as_deref().unwrap_or("/"));
            let config_path = root_path.join("etc/cosmos/config.toml");
            let universe_path = root_path.join("var/lib/cosmos/universe.toml");
            if !config_path.exists() {
                return Err("❌ Config does not exist at /etc/cosmos/config.toml".into());
            }
            let mut config = Config::from_file(config_path.to_str().unwrap())?;
            let mut universe = Universe::load(universe_path.to_str().unwrap())?;

            if let Some(root_override) = &root {
                config.install_dir = root_override.clone();
            }
            let galaxies = cosmos_core::galaxy::Galaxy::load_all_from_config(&config, offline)?;

            if let Some(path) = constellation {
                let constellation = cosmos_core::constellation::Constellation::from_file(&path)?;

                println!("✨ Installing constellation: {}", constellation.name);

                for member in constellation.members {
                    // split member into star name and version (if pinned)
                    let (member_name, version) = if let Some(idx) = member.rfind('@') {
                        (member[..idx].to_string(), member[idx+1..].to_string())
                    } else {
                        (member.clone(), "*".to_string())
                    };

                    let (star, galaxy) = cosmos_core::resolver::find_star(&galaxies, &member_name, &version)
                        .ok_or_else(|| format!("❌ Star '{}' not found in any Galaxy", member))?;

                    let star = cosmos_core::star::fetch_star(galaxy, &star.name, &config, offline)?;
                    install_star(&star, galaxy, &mut universe, &galaxies, &config, offline)?;
                }
            } else if let Some(name) = name {
                let (star, galaxy) = cosmos_core::resolver::find_star(&galaxies, &name, "*")
                    .ok_or_else(|| format!("❌ Star '{}' not found in any Galaxy", name))?;

                let star = cosmos_core::star::fetch_star(galaxy, &star.name, &config, offline)?;
                install_star(&star, galaxy, &mut universe, &galaxies, &config, offline)?;
            } else {
                eprintln!("❌ Must provide a star name or --constellation file");
            }

            if let Some(root_override) = &root {
                config.install_dir = root_override.clone();
                universe.save(&format!("{}/var/lib/cosmos/universe.toml", root_override))?;
            } else {
                universe.save("/var/lib/cosmos/universe.toml")?;
            }
        }

        Commands::Uninstall { name, root } => {
            let root_path = Path::new(root.as_deref().unwrap_or("/"));
            let universe_path = root_path.join("var/lib/cosmos/universe.toml");
            let mut universe = Universe::load(universe_path.to_str().unwrap())?;
            uninstall_star(&name, &mut universe, root_path)?;
            universe.save(&universe_path)?;
        }

        Commands::Status => {
            let universe = Universe::load("/var/lib/cosmos/universe.toml")?;

            println!("⭐ Installed Stars:");
            for (name, star) in &universe.installed {
                println!("- {} @ {}", name, star.version);
            }
        }

        Commands::Sync { stars, full } => {
            let config = Config::from_file("/etc/cosmos/config.toml")?;
            let level = if full {
                SyncLevel::Full
            } else if stars {
                SyncLevel::WithStars
            } else {
                SyncLevel::MetaOnly
            };

            Galaxy::sync_all_from_config(&config, level)?;
            println!("✅ Sync complete");
        }

        Commands::Show { name } => {
            let root_path = Path::new("/");
            let config_path = root_path.join("etc/cosmos/config.toml");
            if !config_path.exists() {
                return Err("❌ Config does not exist at /etc/cosmos/config.toml".into());
            }
            let config = Config::from_file(config_path.to_str().unwrap())?;
            let galaxies = Galaxy::load_all_from_config(&config, false)?;
            let (star_meta, galaxy) = resolver::find_star(&galaxies, &name, "*")
                .ok_or_else(|| format!("❌ Star '{}' not found in any Galaxy", name))?;

            let star = fetch_star(galaxy, &star_meta.name, &config, false)?;

            println!("⭐ {}", star.name);
            println!("Version: {}", star.version);
            if let Some(desc) = &star.description {
                println!("Description: {}", desc);
            }
            if let Some(license) = &star.license {
                println!("License: {}", license);
            }
            if let Some(deps) = &star.dependencies {
                if !deps.is_empty() {
                    println!("Dependencies:");
                    for (name, version) in deps {
                        println!("  - {} @ {}", name, version);
                    }
                }
            }
        }

        Commands::Search { term } => {
            let root_path = Path::new("/");
            let config_path = root_path.join("etc/cosmos/config.toml");
            if !config_path.exists() {
                return Err("❌ Config does not exist at /etc/cosmos/config.toml".into());
            }
            let config = Config::from_file(config_path.to_str().unwrap())?;
            let galaxies = Galaxy::load_all_from_config(&config, false)?;

            println!("🔍 Search results for '{}':", term);
            for galaxy in galaxies {
                for star in galaxy.stars.values() {
                    if star.name.contains(&term)
                        || star.description.as_ref().map_or(false, |d| d.contains(&term))
                    {
                        println!("⭐ {} [{}] ({})", star.name, star.version, galaxy.name);
                    }
                }
            }
        }

        Commands::Init { root } => {
            let root_path = Path::new(root.as_deref().unwrap_or("/"));
            let config_path = root_path.join("etc/cosmos/config.toml");
            let universe_path = root_path.join("var/lib/cosmos/universe.toml");

            if config_path.exists() {
                return Err("❌ Config already exists at /etc/cosmos/config.toml".into());
            }

            fs::create_dir_all("/etc/cosmos")?;
            Config::create_default(config_path.to_str().unwrap())?;
            println!("✅ Wrote config to /etc/cosmos/config.toml");

            if universe_path.exists() {
                println!("⚠️ Universe already exists at /var/lib/cosmos/universe.toml");
            } else {
                fs::create_dir_all("/var/lib/cosmos")?;

                let universe = Universe {
                    system: SystemInfo {
                        arch: std::env::consts::ARCH.to_string(),
                        version: "0.1.0".to_string(),
                    },
                    installed: HashMap::new(),
                };

                universe.save(universe_path)?;
                println!("✅ Initialized empty universe at /var/lib/cosmos/universe.toml");
            }
        }

        Commands::Update { name, offline, root } => {
            let root_path = Path::new(root.as_deref().unwrap_or("/"));
            let config_path = root_path.join("etc/cosmos/config.toml");
            let universe_path = root_path.join("var/lib/cosmos/universe.toml");
            if !config_path.exists() {
                return Err("❌ Config does not exist at /etc/cosmos/config.toml".into());
            }
            let mut config = Config::from_file(config_path.to_str().unwrap())?;
            let mut universe = Universe::load(universe_path.to_str().unwrap())?;

            if let Some(root_override) = &root {
                config.install_dir = root_override.clone();
            }

            let galaxies = Galaxy::load_all_from_config(&config, offline)?;
            let (latest_star, galaxy) = resolver::find_star(&galaxies, &name, "*")
                .ok_or_else(|| format!("❌ Star '{}' not found in any Galaxy", name))?;

            let installed = universe.installed.get(&name);

            if let Some(current) = installed {
                if let Ok(ordering) = cosmos_core::star::compare_versions(&current.version, &latest_star.version) {
                    if ordering != std::cmp::Ordering::Less {
                        println!("✅ '{}' is already up to date ({})", name, current.version);
                        return Ok(());
                    }
                }
                println!("🔁 Updating {}: {} → {}", name, current.version, latest_star.version);
            } else {
                println!("⭐ '{}' is not currently installed. Installing {}", name, latest_star.version);
            }

            let star = fetch_star(galaxy, &latest_star.name, &config, offline)?;
            install_star(&star, galaxy, &mut universe, &galaxies, &config, offline)?;
            println!("✅ Update complete for {}", name);
        }

        Commands::AddGalaxy { name, url, root } => {
            let root_path = Path::new(root.as_deref().unwrap_or("/"));
            let config_path = root_path.join("etc/cosmos/config.toml");

            if !config_path.exists() {
                return Err("❌ Config does not exist at /etc/cosmos/config.toml".into());
            }

            let mut config = Config::from_file(config_path.to_str().unwrap())?;
            config.galaxies.insert(name.clone(), url.clone());
            config.save(config_path.to_str().unwrap())?;

            println!("🌌 Galaxy '{}' added with URL: {}", name, url);
            println!("⚠️  Run `cosmos sync` to fetch the galaxy data.");
        }

        Commands::RemoveGalaxy { name, root } => {
            let root_path = Path::new(root.as_deref().unwrap_or("/"));
            let config_path = root_path.join("etc/cosmos/config.toml");

            if !config_path.exists() {
                return Err("❌ Config does not exist at /etc/cosmos/config.toml".into());
            }

            let mut config = Config::from_file(config_path.to_str().unwrap())?;
            config.galaxies.remove(&name);
            config.save(config_path.to_str().unwrap())?;

            println!("🌌 Galaxy '{}' removed.", name);
        }

        Commands::ListGalaxies { root } => {
            let root_path = Path::new(root.as_deref().unwrap_or("/"));
            let config_path = root_path.join("etc/cosmos/config.toml");

            if !config_path.exists() {
                return Err("❌ Config does not exist at /etc/cosmos/config.toml".into());
            }

            let config = Config::from_file(config_path.to_str().unwrap())?;
            println!("🌌 Galaxies:");
            for (name, url) in &config.galaxies {
                println!("- {}: {}", name, url);
            }
        }
    }

    Ok(())
}
