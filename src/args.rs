use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author,
    version = "0.2",
    about = "Lazydot: CLI tool to manage and deploy your dotfiles efficiently",
    long_about = "Lazydot automates symlink creation for your configuration files, enabling consistent environments across multiple systems."
)]
pub struct LazyDotsArgs {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(long, hide = true)]
    pub completion_shell: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Register one or more dotfile paths in your config.
    #[clap(visible_alias = "r")]
    Register(RegisterArgs),

    /// Remove one or more paths from your config.
    #[clap(visible_alias = "u")]
    Unregister(UnregisterArgs),

    /// Create or update all symlinks according to current config.
    #[clap(visible_alias = "d")]
    Deploy(DeployArgs),

    #[clap(
        alias = "D",
        long_about = "Unlink selected paths and restore files from dotfolder.\n\
        Does not edit the config or delete dotfolder files.\n\
        Use `unregister` + `deploy` to fully stop managing a path."
    )]
    DisableLink(DisableLinkArgs),

    /// Output shell completion script for given shell.
    #[clap(visible_alias = "g",hide = true)]
    GenerateCompletion {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Debug, Args)]
pub struct RegisterArgs {
    /// Path to add (at least one required)
    #[arg(value_parser, required = true, num_args = 1..)]
    pub paths: Vec<String>,
}

#[derive(Debug, Args)]
pub struct UnregisterArgs {
    /// Path to remove (at least one required)
    #[arg(value_parser, required = true, num_args = 1..)]
    pub paths: Vec<String>,
}

#[derive(Debug, Args)]
pub struct DeployArgs {}

#[derive(Debug, Args)]
pub struct DisableLinkArgs {
    /// Unlink all managed symlinks
    #[clap(long = "all", short = 'a', action)]
    pub all: bool,

    /// Specific paths to unlink
    #[arg(value_parser, num_args = 1.., required_unless_present = "all")]
    pub paths: Vec<String>,
}
