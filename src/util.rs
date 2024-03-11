use std::path::PathBuf;
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result, Error};
use home::home_dir;

pub fn sk_cfg_path() -> Result<PathBuf> {
    match home_dir() {
        Some(h) => Ok(h.join(".config").join("sk")),
        None => Err(anyhow!("Could not get sk config directory"))
    }
}

pub fn path_buf_filename(path: &PathBuf) -> Result<String> {
    Ok(path.file_name().unwrap().to_str().unwrap().to_string())
}

pub fn path_buf_to_string(p: &PathBuf) -> Result<String> {
    match p.to_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(anyhow!("Could not convert string to PathBuf")),
    }
}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(&source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
