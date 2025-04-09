use std::fs;
use std::path::PathBuf;

mod helpers {
    pub fn create_file(path: &PathBuf, content: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    pub fn create_dir(path: &PathBuf) {
        fs::create_dir_all(path).unwrap();
    }

    pub fn setup_env() -> TempDir {
        let dir = tempdir().unwrap();
        unsafe {
            std::env::set_var("HOME", dir.path());
        }
        let fake_evn_path: PathBuf = PathBuf::from("src/tests/Data/fake_env");
        let target_evn_path: PathBuf = PathBuf::from(dir.path());
        copy_all(&fake_evn_path, &target_evn_path).expect("Can not copy fake Home");
        dir
    }
}