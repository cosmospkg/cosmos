use std::fs;
use std::path::Path;
use cosmos_core::star::Star;
use semver::Version;

pub fn validate(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(path);
    let star_path = dir.join("star.toml");
    let star_str = fs::read_to_string(&star_path)?;
    let star: Star = toml::from_str(&star_str)?;

    if star.name.trim().is_empty() {
        return Err("❌ Error: Missing or empty `name` field.".into());
    }

    if star.version.trim().is_empty() {
        return Err("❌ Error: Missing `version` field.".into());
    }

    Version::parse(&star.version)?;

    match &star.star_type {
        Some(ty) if ty == "normal" || ty == "nebula" || ty == "meta" => {}
        Some(_) => return Err("❌ Error: Invalid type. Must be 'normal', 'nebula', or 'meta'.".into()),
        None => return Err("❌ Error: Missing `type` field.".into()),
    }

    if star.authors.is_empty() {
        return Err("❌ Error: The `authors` field is required and cannot be empty.".into());
    }

    let lua = dir.join("install.lua");
    let sh = dir.join("install.sh");
    if lua.exists() && sh.exists() {
        return Err("❌ Error: Both install.lua and install.sh exist. Please remove one.".into());
    }

    println!("✅ Valid star: {}-{}", star.name, star.version);
    Ok(())
}
