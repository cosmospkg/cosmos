use clap::{Parser, Subcommand};

mod new_star;
mod build_star;
mod fetch;
mod validate;
mod galaxy;

#[derive(Parser)]
#[command(
    name = "stellar",
    version,
    about = "Build tools for the Cosmos package manager",
    long_about = r#"

        ___________________    __    ___    ____
       / ___/_  __/ ____/ /   / /   /   |  / __ \
       \__ \ / / / __/ / /   / /   / /| | / /_/ /
      ___/ // / / /___/ /___/ /___/ ___ |/ _, _/
     /____//_/ /_____/_____/_____/_/  |_/_/ |_|
                     Stellar CLI
     A build tool for the Cosmos package manager
       A minimal, offline-first package manager

    "#,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new star (interactive)
    NewStar {
        name: String,
    },

    /// Build a star package
    BuildStar {
        path: String,
    },

    /// Fetch a star from a galaxy
    Fetch {
        path: String,
    },

    /// Validate a star package
    Validate {
        path: String,
    },

    /// Initialize a new galaxy
    GalaxyInit {
        name: String,
    },

    /// Update the galaxy index with star definitions
    IndexGalaxy {
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::NewStar { name } => new_star::new_star(&name).unwrap(),
        Commands::BuildStar { path } => build_star::build_star(&path).unwrap(),
        Commands::Fetch { path } => fetch::fetch(&path).unwrap(),
        Commands::Validate { path } => validate::validate(&path).unwrap(),
        Commands::GalaxyInit { name } => galaxy::galaxy_init(&name).unwrap(),
        Commands::IndexGalaxy { path } => galaxy::index_galaxy(&path).unwrap(),
    }
}