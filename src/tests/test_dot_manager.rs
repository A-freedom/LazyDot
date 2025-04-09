mod test {
    use crate::config::Config;
    use crate::dot_manager::DotManger;
    use crate::tests::test_utils::test::setup_env;
    use crate::utils::expand_path;

    #[test]
    fn adding_and_removing_paths() {
        let tmp_home = setup_env();
        let binding = tmp_home.path().join(".config/app2/app_config.toml");
        let paths = ["~/.bashrc", ".config/app1", binding.to_str().unwrap()];
        let mut config = Config::new();

        // adding paths
        for path in paths {
            let result = config.add_path(path.to_string());
            assert!(result.is_ok());
        }
        // adding path that does not exist.
        vec!["~/some_path", ".absolute_path", "~/nested/path/config.csv"]
            .iter()
            .for_each(|path| {
                let result = config.add_path(path.to_string());
                assert_eq!(result.err().unwrap(), "Path does not exist");
            });

        // test the paths is as expected.
        assert_eq!(
            config.paths,
            vec![
                "~/.bashrc",
                "~/.config/app1",
                "~/.config/app2/app_config.toml"
            ]
        );

        let manager = DotManger::new();
        manager.sync();
        for result in config.paths.iter().map(|x| expand_path(x)) {
            let path = result.expect("Failed to get path");
            // test if the paths is still exist and is a symlink after running the `Sync`
            assert!(&path.exists());
            assert!(&path.is_symlink());
        }

        // test if you add paths that is already is symlink
        for path in paths {
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

        // test removing the paths
        for path in paths {
            config.remove_path(path.to_string());
        }

        assert!(config.paths.is_empty());
    }
}
