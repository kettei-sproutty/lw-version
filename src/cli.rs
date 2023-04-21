use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "lw-version")]
#[command(
    about = "A simple and lightweight versioning tool",
    long_about = "Versioning tool based on Semantic Versioning (https://semver.org)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
