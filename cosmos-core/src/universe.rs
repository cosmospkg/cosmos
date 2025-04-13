use cosmos_universe::{Universe, InstalledStar};
use crate::star::Star;

pub fn record_install(universe: &mut Universe, star: &Star, files: Vec<String>) {
    let installed = InstalledStar {
        name: star.name.clone(),
        version: star.version.clone(),
        files,
    };
    universe.installed.insert(star.name.clone(), installed);
}
