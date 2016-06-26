extern crate fs_utils;

mod destination_dir {
    use fs_utils::destination_dir;
    use std::path::PathBuf;
    use std::env::set_current_dir;
    
    #[test]
    #[should_panic]
    /// Changes CWD, and thus needs its own file to be sandboxed
    fn it_can_work_with_relative_paths_but_not_if_that_is_root_as_well() {
        set_current_dir("/").unwrap();
        assert_eq!(destination_dir("", "dest"), PathBuf::from("dest/fs-utils"));
    }
    
}