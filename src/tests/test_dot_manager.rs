mod test {
    use crate::config::Config;
    use crate::dot_manager::DotManger;
    use crate::utils::{copy_all, expand_path, get_home_dir};
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    pub fn setup_env_once() {
        // Save the original working dir if needed later
        let path_var = "lazydot_path_test";
        if std::env::var(path_var).is_err() {
            let cwd = std::env::current_dir().expect("Failed to get current dir");
            unsafe {
                std::env::set_var(path_var, cwd.to_str().expect("Invalid UTF-8 in cwd"));
            }
        }

        let old_dir = std::env::var(path_var).expect("Missing lazydot_path_test var");
        std::env::set_current_dir(&old_dir).expect("Failed to set current dir");

        // Create or get temp home
        let lazy_temp_home = std::env::var("lazy_temp_home").unwrap_or_else(|_| {
            let temp_path = tempdir().expect("Failed to create temp dir").into_path();
            let temp_str = temp_path
                .to_str()
                .expect("Invalid UTF-8 in temp path")
                .to_string();
            unsafe {
                std::env::set_var("lazy_temp_home", &temp_str);
            }
            temp_str
        });

        let dir = Path::new(&lazy_temp_home).to_path_buf();

        // Set HOME
        unsafe {
            std::env::set_var("HOME", dir.to_str().expect("Invalid UTF-8 in temp home"));
        }

        // Copy fake home structure if it exists
        let fake_env_path = PathBuf::from("src/tests/Data/fake_env");
        if !fake_env_path.exists() {
            panic!("fake_env not found at {:?}", fake_env_path);
        }

        copy_all(&fake_env_path, &dir).expect("Failed to copy fake_env to temp HOME");

        // Set CWD to fake HOME
        std::env::set_current_dir(&dir).expect("Failed to change CWD to temp HOME");
    }

    fn get_testing_paths() -> Vec<String> {
        let env_home = get_home_dir().expect("Failed to get home dir");
        let binding = env_home.join(".config/app2/app_config.toml");
        let paths = ["~/.bashrc", ".config/app1", binding.to_str().unwrap()];
        paths.map(|t| t.to_string()).to_vec()
    }

    #[test]
    #[serial_test::serial]
    fn adding_and_removing_paths() {
        setup_env_once();
        let mut config = Config::new();
        let paths = get_testing_paths();

        // adding paths
        for path in &paths {
            let result = config.add_path(path.to_string());
            assert!(result.is_ok());
        }
        // adding path that does not exist.
        vec![
            "~/some_path",
            ".absolute_path",
            "~/nested/path/config.csv",
            "null",
        ]
        .iter()
        .for_each(|path| {
            let result = config.add_path(path.to_string());
            assert_eq!(result.err().unwrap(), "Path does not exist");
        });
        vec![
            "~/",
            "",
            get_home_dir().unwrap().to_str().unwrap(),
        ]
            .iter()
            .for_each(|path| {
                let result = config.add_path(path.to_string());
                assert_eq!(result.err().unwrap(), "You can't add your home as path");
            });
    }

    #[test]
    #[serial_test::serial]
    fn path_normalization() {
        setup_env_once();
        let config = Config::new();
        assert_eq!(
            config.paths,
            vec![
                "~/.bashrc",
                "~/.config/app1",
                "~/.config/app2/app_config.toml"
            ]
        );
    }

    #[test]
    #[serial_test::serial]
    fn sync_and_delink() {
        setup_env_once();
        let mut config = Config::new();
        let paths = get_testing_paths();
        let manager = DotManger::new();

        manager.sync();
        for result in config.paths.iter().map(|x| expand_path(x)) {
            let path = result.expect("Failed to get path");
            // test if the paths is still exist and is a symlink after running the `Sync`
            assert!(&path.exists());
            assert!(&path.is_symlink());
        }

        // test if you add paths that is already is symlink
        for path in &paths {
            let result = config.add_path(path.to_string());
            assert!(result.is_ok());
        }

        // test de_link_all
        manager.delink_all();
        for result in config.paths.iter().map(|x| expand_path(x)) {
            let path = result.expect("Failed to get path");
            // test if the paths is still exist and is a symlink after running the `Sync`
            assert!(&path.exists());
            assert!(!&path.is_symlink());
        }
    }
    #[test]
    #[serial_test::serial]
    fn removing_paths() {
        let mut config = Config::new();
        let paths = get_testing_paths();

        for path in &paths {
            config.remove_path(path.to_string());
        }
        assert!(config.paths.is_empty());
    }
}
