use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct LazyDotsArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a path
    AddPath(AddArgs),

    /// Remove a path
    RemovePath(RemoveArgs),

    /// Apply config
    ApplyConfig(ApplyConfigArg),
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// Path to add
    #[clap(value_parser)]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Path to remove
    #[clap(value_parser)]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct ApplyConfigArg {
}
