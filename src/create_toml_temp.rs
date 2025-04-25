use crate::utils::get_home_dir;

const DEFAULT_CONFIG: &str = r#"
# Lazydot config file

# Path to your dotfiles directory (must start with ~/)
dotfolder_path = "~/mydotfolder"

# List of dotfiles to manage (must start with ~/ or /)
paths = [
]

[defaults]
# Behavior on duplicate:
# ask, overwritehome, overwritedotfile, backuphome, skip
on_duplicate = "ask"
"#;

pub fn create_default_config_if_missing() {
    let config_file = get_home_dir().join(".config/lazydot.toml");
    if !config_file.exists() {
        std::fs::write(config_file, DEFAULT_CONFIG).expect("Failed to create default config");
    }
}
