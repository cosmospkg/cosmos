use std::path::Path;
use crate::galaxy::Galaxy;
use crate::star::{compare_versions, Star};
use crate::error::CosmosError;

pub fn satisfies_constraint(installed: &str, constraint: &str) -> Result<bool, CosmosError> {
    let comparison = compare_versions(installed, constraint)?;
    match comparison {
        std::cmp::Ordering::Less => Ok(false),
        std::cmp::Ordering::Equal => Ok(true),
        std::cmp::Ordering::Greater => Ok(true),
    }
}

pub fn find_star<'a>(
    galaxies: &'a [Galaxy],
    name: &str,
    constraint: &str,
) -> Option<(&'a Star, &'a Galaxy)> {
    for galaxy in galaxies {
        if let Some(star) = galaxy.get_star(name) {
            // TODO: keep or panic on error?
            let satisfies = satisfies_constraint(&star.version, constraint);
            if satisfies.is_ok() && satisfies.unwrap() {
                return Some((star, galaxy));
            } else {
                eprintln!(
                    "❌ Error: {} does not satisfy the version constraint {} due to parsing error",
                    star.name, constraint
                );
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

    if !file_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
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