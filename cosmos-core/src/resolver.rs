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