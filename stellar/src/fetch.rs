use std::fs;
use std::path::{Path};
use cosmos_core::star::Star;
use std::fs::File;

pub fn fetch(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(path);
    let star_path = dir.join("star.toml");
    let star_str = fs::read_to_string(&star_path)?;
    let star: Star = toml::from_str(&star_str)?;

    let source = star.source.as_ref()
        .ok_or("No `source` field in star.toml")?;

    if source.starts_with("http://") {
        let filename = source.split('/').last().unwrap_or("source.tar.gz");
        let target = dir.join(".cache").join("src").join(filename);
        fs::create_dir_all(target.parent().unwrap())?;

        println!("🌐 Fetching {}", source);
        let response = cosmos_transport::fetch_bytes(source)
            .map_err(|e| format!("Failed to download: {}", e))?;
        let mut file = File::create(&target)?;
        std::io::copy(&mut response.as_slice(), &mut file)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        println!("✅ Saved to {}", target.display());
    } else if source.starts_with("file://") || Path::new(source).exists() {
        println!("⭐ Local source detected: {} (no fetch needed)", source);
    } else {
        return Err("Unsupported or missing source format".into());
    }

    Ok(())
}
