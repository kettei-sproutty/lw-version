use anyhow::Result;
use clap::{Parser, Subcommand};
mod package_json;
use crate::package_json::PackageJson;

#[derive(Debug, Parser)]
#[command(name = "lw-version")]
#[command(
    about = "A simple and lightweight versioning tool",
    long_about = "Versioning tool based on Semantic Versioning (https://semver.org)"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        about = "Generates a semantic version string based on the commit history.",
        long_about = "This command analyzes the commit history of the current branch and generates a semantic version number \
        (MAJOR.MINOR.PATCH) according to the Semantic Versioning specification. It increments the MAJOR version when backwards \
        incompatible changes are detected, the MINOR version when new features are added backwards compatibly, and the PATCH \
        version when only backwards compatible bug fixes are applied. This allows you to easily track the evolution of versions \
        and scopes of changes in your project's release history."
    )]
    Ci,
    #[command(
        about = "Increments the major version number.",
        long_about = "his command increments the major version number of your project. It should be used when you make incompatible \
        API changes, according to the Semantic Versioning specification. It updates the version in your project's package.json file \
        and creates a Git tag for the new major version."
    )]
    Major,
    #[command(about = "Increments the minor version number.")]
    Minor,
    #[command(about = "Increments the patch version number.")]
    Patch,
    #[command(about = "Version release")]
    Version,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Ci => {
            let package_json = PackageJson::default();
            let (major, minor, patch) = package_json.parse_version().unwrap();

            println!("Major: {:?}, Minor: {:?}, Patch: {:?}", major, minor, patch);
        }
        Commands::Major => println!("Major command!"),
        Commands::Minor => println!("Minor command!"),
        Commands::Patch => println!("Patch command!"),
        Commands::Version => println!("Version command!"),
    }

    Ok(())
}
