use std::path::{Path, PathBuf};
use std::io;
use std::env::current_dir;
use std::ffi::OsString;

pub fn destination_dir<P: AsRef<Path>>(source_dir: P, destination_dir: P) -> PathBuf {
    destination_dir.as_ref()
        .join(source_dir.as_ref()
            .file_name()
            .unwrap_or(&current_dir().unwrap().file_name().unwrap()))
}

pub fn copy_directory(source_dir: &Path, destination_dir: &Path) -> Result<PathBuf, io::Error> {
    let dest = ::destination_dir(source_dir, destination_dir);
    Ok(dest)
}
