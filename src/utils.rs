use std::{env, fs};
use std::path::{ PathBuf};

pub fn check_path(path: &str) -> Result<String, String> {
    let input_path = expand_path(path)?;

    if !input_path.exists() {
        return Err("Path does not exist".to_string());
    }

    let home = get_home_dir()?;

    // Important: don't resolve symlinks. We want the path *as passed*, not the real target.
    let cleaned_path = input_path;

    if !cleaned_path.starts_with(&home) {
        return Err("Path is outside of the home directory".to_string());
    }

    let relative = cleaned_path
        .strip_prefix(&home)
        .map_err(|_| "Failed to strip home prefix".to_string())?;

    Ok(format!("~/{}", relative.display()))
}

pub fn get_home_dir() -> Result<PathBuf, String> {
    env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| "Could not determine home directory".to_string())
}


pub fn expand_path(input: &str) -> Result<PathBuf, String> {
    let mut path = if input.starts_with("~/") {
        let home = get_home_dir()?;
        home.join(&input[2..])
    } else {
        PathBuf::from(input)
    };

    if !path.is_absolute() {
        let cwd = env::current_dir()
            .map_err(|_| "Failed to get current directory".to_string())?;
        path = cwd.join(path);
    }

    Ok(path)
}

pub fn delete(path: &PathBuf) -> Result<(), String> {
    if path.is_file() || path.is_symlink() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    // Check if it's a directory and remove the directory recursively
    else if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| e.to_string())?;
    }
    // If it's neither a symlink, file, nor directory
    else {
        return Err("Path is not a valid symlink, file, or directory".to_string());
    }

    Ok(())
}

