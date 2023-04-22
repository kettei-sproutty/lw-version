use crate::cli::{Cli, Commands};
use anyhow::{Context, Result};
use clap::Parser;
use git2::Repository;

mod cli;
mod git;
mod manifests;

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Ci => println!("Ci command!"),
        Commands::Major => println!("Major command!"),
        Commands::Minor => println!("Minor command!"),
        Commands::Patch => println!("Patch command!"),
        Commands::Version => println!("Version command!"),
    }

    Ok(())
}
