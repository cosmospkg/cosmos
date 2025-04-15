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
        dependencies: None,
        source: None,
        license: None,
        authors,
        checksums: None,
    };

    let toml_str = toml::to_string_pretty(&star)?;
    fs::write(dir.join("star.toml"), toml_str)?;

    let mut f = File::create(dir.join("install.lua"))?;
    writeln!(f, "function install()\n  -- copy(\"bin/tool\", \"/usr/bin/tool\")\nend")?;

    println!("✅ Created new star at ./{}/", name);
    Ok(())
}