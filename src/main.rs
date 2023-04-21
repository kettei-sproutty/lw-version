use crate::cli::{Cli, Commands};
use crate::package_json::PackageJson;
use anyhow::Result;
use clap::Parser;

mod cli;
mod package_json;

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Ci => {
            let package_json = PackageJson::from(None);
            let (major, minor, patch) = package_json.parse_version()?;

            println!("Major: {:?}, Minor: {:?}, Patch: {:?}", major, minor, patch);
        }
        Commands::Major => println!("Major command!"),
        Commands::Minor => println!("Minor command!"),
        Commands::Patch => println!("Patch command!"),
        Commands::Version => println!("Version command!"),
    }

    Ok(())
}
