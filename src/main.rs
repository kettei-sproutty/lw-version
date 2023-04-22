use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod manifests;
mod utils;

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
