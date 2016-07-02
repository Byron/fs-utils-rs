//! Utilities to help working with the file-system.
use std::path::{Path, PathBuf};
use std::io;
use std::env::current_dir;

/// Return the computed destination directory, given a source directory.
pub fn destination_dir<P: AsRef<Path>>(source_dir: P, destination_dir: P) -> PathBuf {
    let source_dir = source_dir.as_ref()
                               .canonicalize()
                               .unwrap_or_else(|_| source_dir.as_ref().to_path_buf());
    destination_dir.as_ref().join(source_dir.file_name().unwrap_or("ROOT".as_ref()))
}

pub fn copy_directory(source_dir: &Path, destination_dir: &Path) -> Result<PathBuf, io::Error> {
    let dest = ::destination_dir(source_dir, destination_dir);
    Ok(dest)
}
