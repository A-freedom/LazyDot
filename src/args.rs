use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct LazyDotsArgs {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(long, hide = true)]
    pub completion_shell: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a path
    AddPath(AddArgs),

    /// Remove a path
    RemovePath(RemoveArgs),

    /// Apply config
    ApplyConfig(ApplyConfigArg),

    /// Unlinking all the paths
    UnLinkAll,

    /// unlink a paths or a list of paths
    UnLink(UnLinkArgs),

    Completion {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
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
pub struct ApplyConfigArg {}

#[derive(Debug, Args)]
pub struct UnLinkArgs {
    /// Path to unlink
    #[clap(value_parser)]
    pub path: Vec<String>,
}
