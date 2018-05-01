extern crate fs_utils;
extern crate tempdir;

mod utils {
    use std::path::{Path, PathBuf};

    pub fn fixture_at(name: &str) -> PathBuf {
        Path::new(file!())
            .parent()
            .map(|p| p.join("fixtures").join(name))
            .unwrap()
    }
}

mod copy_directory {
    use tempdir::TempDir;
    use fs_utils::{check, copy, remove};
    use super::utils::fixture_at;

    #[test]
    fn it_cleans_up_an_empty_folder() {
        let empty_folder = TempDir::new("dest").unwrap();
        remove::cleanup_folder(empty_folder.path()).unwrap();
        assert!(check::is_folder_empty(empty_folder.path()).unwrap());
    }

    #[test]
    fn it_cleans_up_a_folder_with_nested_folders_and_files() {
        let (source, dest) = (fixture_at("source-1"), TempDir::new("dest").unwrap());
        let dest_path = copy::copy_directory(&source, dest.path()).unwrap();

        remove::cleanup_folder(&dest_path).unwrap();
        assert!(check::is_folder_empty(&dest_path).unwrap());
    }
}
