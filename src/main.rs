mod args;
mod config;
mod dot_manager;
mod tests {
    pub mod test_dot_manager;
    pub mod test_utils;

}
mod utils;

use crate::args::Command;
use crate::dot_manager::DotManager;
use args::LazyDotsArgs;
use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use config::Config;
use std::io;

fn main() {
    let args = LazyDotsArgs::parse();
    // Handle shell completion generation
    if let Some(shell) = args.completion_shell {
        let mut cmd = LazyDotsArgs::command();
        let shell: Shell = shell.parse().expect("Invalid shell type");
        generate(shell, &mut cmd, "lazydot", &mut io::stdout());
        return;
    }
    match args.command {
        Command::AddPath(add_args) => {
            let mut config = Config::new();
            config.add_path(add_args.path).expect("failed to add path");
        }
        Command::RemovePath(remove_args) => {
            let mut config = Config::new();
            config.remove_path(remove_args.path);
        }
        Command::ApplyConfig(apply_args) => {
            dbg!(apply_args);
            let manager = DotManager::new();
            manager.sync();
        }
        Command::Completion { shell } => {
            let mut cmd = LazyDotsArgs::command();
            generate(shell, &mut cmd, "lazydot", &mut io::stdout());
        }
        Command::UnLinkAll {} => {
            let manager = DotManager::new();
            manager.delink_all();
        }
        Command::UnLink (delink_args) => {
            let manager = DotManager::new();
            manager.delink(&delink_args.path);
        }
    }
}
