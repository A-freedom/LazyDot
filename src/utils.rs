use crate::config::{Config, DuplicateBehavior};
use crate::dot_manager::DotManager;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::{env, fs};
use tempfile::tempdir;

pub fn check_path(path: &str) -> Result<String, String> {
    let input_path = expand_path(path)?;

    if !input_path.exists() {
        return Err("Path does not exist".to_string());
    }

    let home = get_home_dir()?;

    if input_path.eq(&home) {
        return Err("You can't add your home as path".to_string());
    }
    if !input_path.starts_with(&home) {
        return Err("Path is outside of the home directory".to_string());
    }

    let relative = input_path
        .strip_prefix(&home)
        .map_err(|_| "Failed to strip home prefix".to_string())?;

    Ok(format!("~/{}", relative.display()))
}

pub fn get_home_dir() -> PathBuf {
    PathBuf::from(get_home_dir_string())
}

pub fn expand_path(input: &str) -> Result<PathBuf, String> {
    let mut path = if input.starts_with("~/") {
        let home = get_home_dir();
        home.join(&input[2..])
    } else {
        PathBuf::from(input)
    };

    if !path.is_absolute() {
        let cwd = env::current_dir().map_err(|_| "Failed to get current directory".to_string())?;
        path = cwd.join(path);
    }

    Ok(path)
}

pub fn delete(path: &PathBuf) {
    if path.is_file() || path.is_symlink() {
        fs::remove_file(path).expect(&format!("Failed to delete {}", path.display()));
    }
    // Check if it's a directory and remove the directory recursively
    else if path.is_dir() {
        fs::remove_dir_all(path).expect(&format!("Failed to delete {}", path.display()));
    }
    // If it's neither a symlink, file, nor directory
    else {
        panic!(
            "Path: \"{}\" is not a valid file, or directory",
            path.display()
        );
    }
}

pub fn copy_all(source_path: &PathBuf, target_path: &PathBuf) -> Result<(), std::io::Error> {
    if !source_path.exists() {
        return Err(std::io::Error::new(
            ErrorKind::NotFound,
            format!("Source does not exist: {}", source_path.display()),
        ));
    }
    if source_path.is_file() {
        let parent = target_path
            .parent()
            .expect("Failed to get parent directory");
        fs::create_dir_all(parent).expect("Failed to create directory");
        fs::copy(source_path, target_path).expect("Failed to copy file");
        return Ok(());
    }
    if source_path.is_dir() {
        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let entry_path = entry.path();

            // Compute relative path from source root
            let relative = entry_path
                .strip_prefix(source_path)
                .expect("Failed to get relative path");

            let nested_target = target_path.join(relative);
            copy_all(&entry_path, &nested_target)?;
        }
    } else {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            "Can not copy what is not a file or directory",
        ));
    }
    Ok(())
}

fn get_relative_path(path: &String) -> Result<PathBuf, String> {
    // Expand ~ or $HOME to absolute path
    let path_in_home = expand_path(path).expect("Failed to expand path");

    let relative_path = path_in_home
        .strip_prefix(get_home_dir())
        .expect("Failed to strip prefix from home dir")
        .to_path_buf();
    Ok(relative_path)
}

pub fn get_path_in_dotfolder(path_in_home: &PathBuf) -> Result<PathBuf, String> {
    let config = Config::new();
    let relative_path = get_relative_path(&path_in_home.to_str().unwrap().to_string())?;
    let path_in_dotfolder = expand_path(&config.dotfolder_path)?.join(&relative_path);
    Ok(path_in_dotfolder)
}

/// Resets test environment by:
/// - Returning to project root
/// - Creating a fresh temporary HOME
/// - Copying the fake environment
/// - Setting CWD to the new fake HOME
pub fn reset_test_environment() {
    // Make sure we always start from the project directory
    let project_root_var = "lazydot_path_test";
    if env::var(project_root_var).is_err() {
        let cwd = env::current_dir().expect("Failed to get current dir");
        unsafe {
            std::env::set_var(
                project_root_var,
                cwd.to_str().expect("Invalid UTF-8 in cwd"),
            );
        }
    }
    let root = std::env::var(project_root_var).expect("Missing lazydot_path_test var");
    std::env::set_current_dir(&root).expect("Failed to set current dir");

    // Create a new temporary home directory
    let temp_home_path = tempdir().expect("Failed to create temp dir").into_path();

    // Set HOME to the new fake temp dir
    unsafe {
        env::set_var(
            "HOME",
            temp_home_path.to_str().expect("Invalid UTF-8 in temp home"),
        );
    }

    // Copy fake home structure into temp HOME
    let fake_env_path = PathBuf::from("src/tests/Data/fake_env");
    if !fake_env_path.exists() {
        panic!("fake_env not found at {:?}", fake_env_path);
    }

    copy_all(&fake_env_path, &temp_home_path).expect("Failed to copy fake_env to temp HOME");

    // Set current working directory to fake HOME
    env::set_current_dir(temp_home_path.to_str().unwrap())
        .expect("Failed to change CWD to temp HOME");
}

/// Static test paths for config testing
pub fn mock_dotfile_paths() -> Vec<String> {
    let env_home = get_home_dir();
    let extra = env_home.join(".config/app2/app_config2.toml");
    let paths = ["~/.bashrc", ".config/app1", extra.to_str().unwrap()];
    paths.map(|t| t.to_string()).to_vec()
}

/// Creates a config with the test paths added
pub fn init_config_with_paths() -> Config {
    let mut config = Config::new();
    mock_dotfile_paths()
        .iter()
        .for_each(|path| config.add_path(path.clone()).expect("Failed to add path"));
    config
}

/// Prepares and syncs the config using the given duplication strategy
pub fn sync_config_with_manager(duplicate_behavior: DuplicateBehavior) -> (Config, DotManager) {
    let mut config = init_config_with_paths();
    config.defaults.on_duplicate = duplicate_behavior;
    config.save();
    let manager = DotManager::new();
    manager.sync();
    (config, manager)
}
