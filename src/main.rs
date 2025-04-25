mod args;
mod config;
mod dot_manager;
mod tests {
    pub mod test_dot_manager;
    pub mod test_utils;
}
mod create_toml_temp;
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
        Command::Register(add_args) => {
            let mut config = Config::new();
            for path in add_args.paths {
                config.add_path(path).expect("failed to add path");
            }
        }
        Command::Unregister(remove_args) => {
            let mut config = Config::new();
            for path in remove_args.paths {
                config.remove_path(path);
            }
        }
        Command::Deploy(_apply_args) => {
            let manager = DotManager::new();
            manager.sync();
        }
        Command::GenerateCompletion { shell } => {
            let mut cmd = LazyDotsArgs::command();
            generate(shell, &mut cmd, "lazydot", &mut io::stdout());
        }
        Command::Clean(delink_args) => {
            let manager = DotManager::new();
            match delink_args.all {
                true => {
                    manager.delink_all();
                }
                false => {
                    manager.delink(&delink_args.paths);
                }
            }
        }
    }
}
