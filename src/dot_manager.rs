use crate::config::Config;
use crate::utils::{delete, expand_path, get_home_dir};
use std::fs;
use std::path::PathBuf;

pub struct DotManger {
    config: Config,
    home_dir: PathBuf,
    dotfolder_path: PathBuf,
}
impl DotManger {
    pub fn new() -> DotManger {
        // init Config
        let config = Config::new();

        // init dotfolder
        let dotfolder_path = expand_path(&config.dotfolder_path).unwrap();
        if !dotfolder_path.exists() {
            fs::create_dir_all(&dotfolder_path).unwrap();
        }
        if !dotfolder_path.is_dir() {
            panic!("{} is not a directory", dotfolder_path.display());
        }

        Self {
            home_dir: get_home_dir().unwrap(),
            config,
            dotfolder_path,
        }
    }

    pub fn sync(&self) {
        // init some consts

        for path in &self.config.paths {
            println!("+++++++++++++++++++++++++++++++++++++++++");
            let full_path = expand_path(&path).unwrap();
            if !full_path.starts_with(&self.home_dir) {
                panic!("{} is not inside the HOME directory", path);
            }

            let relative_path = full_path.strip_prefix(&self.home_dir).unwrap();
            let target_path = self.dotfolder_path.join(&relative_path);
            if !target_path.exists() {
                fs::create_dir_all(&target_path.parent().unwrap()).unwrap();
                self.copy_all(&full_path, &target_path).unwrap();

                dbg!(&full_path, &target_path);
                delete(&full_path).expect(&format!("Failed to delete file {:?}", full_path));
                std::os::unix::fs::symlink(&target_path, &full_path)
                    .expect(&format!("unable to hard link {:?}", target_path));
            }
        }
    }

    fn copy_all(&self, full_path: &PathBuf, target_path: &PathBuf) -> Result<(), std::io::Error> {
        if full_path.is_file() {
            let parent = target_path.parent().unwrap();
            fs::create_dir_all(parent)?;
            fs::copy(full_path, target_path)?;
            return Ok(());
        }
        if full_path.is_dir() {
            for entry in fs::read_dir(full_path)? {
                let entry = entry?.path();

                let relative_path = entry.strip_prefix(&self.home_dir).unwrap();
                let target_path = self.dotfolder_path.join(&relative_path);
                self.copy_all(&entry, &target_path)?
            }
        }
        Ok(())
    }
}
