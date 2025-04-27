use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"
# Lazydot Configuration File

# Path to the dotfiles folder (must start with ~/)
dotfolder_path = "~/mydotfolder"

# List of dotfile paths to manage (each must start with ~/ or /)
# paths = [
#     "~/example/.bashrc","
#     "~/example/.bashrc",
#     "~/example/.vimrc",
# ]
paths = [
    "~/.config/lazydot.toml",
]

[defaults]
# Behavior when a duplicate file is found at the destination:
# - ask: Prompt the user to decide
# - overwritehome: Overwrite the file in HOME with the dotfolder version
# - overwritedotfile: Overwrite the dotfolder copy with the HOME version
# - backuphome: Backup the HOME file before overwriting
# - skip: Do nothing and skip the conflict
on_duplicate = "ask"

# Behavior after a link is disabled (delinked):
# - remove: Remove the file from the dotfolder after restoring it to HOME (default)
# - keep: Keep the file in the dotfolder even after restoring it to HOME
on_delink = "remove"
"#;

pub fn create_default_config(config_file: &PathBuf) {
    std::fs::write(config_file, DEFAULT_CONFIG).expect("Failed to create default config");
}
