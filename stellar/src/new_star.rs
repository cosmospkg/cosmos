use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use dialoguer::{Input, Select};
use std::fs::File;
use cosmos_core::star::Star;

pub fn new_star(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(name);
    if dir.exists() {
        return Err("❌ Error: Directory already exists".into());
    }

    fs::create_dir_all(dir.join("files"))?;

    let version: String = Input::new()
        .with_prompt("🔢 Version")
        .default("1.0.0".into())
        .interact_text()?;

    let description: String = Input::new()
        .with_prompt("📝 Description")
        .default("".into())
        .interact_text()?;

    let author_name: String = Input::new()
        .with_prompt("👤 Author name")
        .default("Anonymous".into())
        .interact_text()?;

    let author_email: String = Input::new()
        .with_prompt("📧 Author email")
        .default("none@example.com".into())
        .interact_text()?;

    let mut authors = HashMap::new();
    authors.insert(author_name, author_email);

    loop {
        let more_authors: bool = dialoguer::Confirm::new()
            .with_prompt("Add another author?")
            .default(false)
            .interact()?;
        if !more_authors {
            break;
        }
        let author_name: String = Input::new()
            .with_prompt("👤 Author name")
            .default("Anonymous".into())
            .interact_text()?;
        let author_email: String = Input::new()
            .with_prompt("📧 Author email")
            .default("none@example.com".into())
            .interact_text()?;
        authors.insert(author_name, author_email);
    }


    let license: String = Input::new()
        .with_prompt("📜 License")
        .default("MIT".into())
        .interact_text()?;

    // choose if they want to add dependencies
    let add_dependencies: bool = dialoguer::Confirm::new()
        .with_prompt("Add dependencies?")
        .default(false)
        .interact()?;
    let mut dependencies = HashMap::new();
    if add_dependencies {
        loop {
            let dep_name: String = Input::new()
                .with_prompt("🔗 Dependency name")
                .default("".into())
                .interact_text()?;
            let dep_version: String = Input::new()
                .with_prompt("🔗 Dependency version")
                .default("1.0.0".into())
                .interact_text()?;
            dependencies.insert(dep_name, dep_version);
            let more_dependencies: bool = dialoguer::Confirm::new()
                .with_prompt("Add another dependency?")
                .default(false)
                .interact()?;
            if !more_dependencies {
                break;
            }
        }
    }

    let type_options = vec!["normal", "nebula"];
    let selected = Select::new()
        .with_prompt("🧬 Type")
        .default(0)
        .items(&type_options)
        .interact()?;
    let star_type = type_options[selected].to_string();

    let star = Star {
        name: name.to_string(),
        version,
        description: Some(description),
        star_type: Some(star_type),
        install_script: Some("install.lua".to_string()),
        dependencies: Some(dependencies),
        source: None,
        license: Some(license),
        authors,
        checksums: None,
    };

    let toml_str = toml::to_string_pretty(&star)?;
    fs::write(dir.join("star.toml"), toml_str)?;

    if selected == 0 {
        let mut f = File::create(dir.join("install.lua"))?;
        writeln!(f, "function install()\n  -- copy(\"bin/tool\", \"/usr/bin/tool\")\nend")?;
    } else {
        println!("⚠️ You selected a nebula, please remember that nebulas are metapackages and cannot contain files or install scripts.");
        if !add_dependencies {
            println!("⚠️ Nebula stars should have dependencies. Please add them to the star.toml file.");
        }
    }

    println!("✅ Created new star at ./{}/", name);
    Ok(())
}