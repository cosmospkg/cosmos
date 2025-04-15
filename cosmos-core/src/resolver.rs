use std::path::Path;
use crate::galaxy::Galaxy;
use crate::star::Star;
use semver::{Version, VersionReq};

pub fn satisfies_constraint(installed: &str, constraint: &str) -> bool {
    let version = match Version::parse(installed) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let req = match VersionReq::parse(constraint) {
        Ok(r) => r,
        Err(_) => return false,
    };

    req.matches(&version)
}

pub fn find_star<'a>(
    galaxies: &'a [Galaxy],
    name: &str,
    constraint: &str,
) -> Option<(&'a Star, &'a Galaxy)> {
    for galaxy in galaxies {
        if let Some(star) = galaxy.get_star(name) {
            if satisfies_constraint(&star.version, constraint) {
                return Some((star, galaxy));
            }
        }
    }
    None
}

pub fn calculate_checksum(file_path: &Path) -> Result<String, std::io::Error> {
    use sha2::{Sha256, Digest};
    use std::fs::File;
    use std::io::{BufReader, Read};

    // if it is a directory, error
    if file_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Provided path is a directory, not a file",
        ));
    }

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}