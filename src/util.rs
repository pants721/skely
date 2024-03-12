use std::{io::Write, path::PathBuf};
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
use home::home_dir;

pub fn sk_cfg_path() -> Result<PathBuf> {
    match home_dir() {
        Some(h) => Ok(h.join(".config").join("sk")),
        None => Err(anyhow!("Could not get sk config directory"))
    }
}

pub fn path_buf_filename(path: &Path) -> Result<String> {
    Ok(path.file_name().unwrap().to_str().unwrap().to_string())
}

pub fn path_buf_to_string(p: &Path) -> Result<String> {
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

pub fn replace_string_in_dir(input_path: &PathBuf, from: String, to: String) -> Result<()> {
    if input_path.is_file() {
        replace_string_in_file(input_path, from.clone(), to.clone())?;
        return Ok(());
    }

    let paths = fs::read_dir(input_path)?;

    for dir_entry in paths {
        if dir_entry.as_ref().unwrap().path().is_dir() {
            replace_string_in_dir(&dir_entry?.path(), from.clone(), to.clone())?;
        } else {
            replace_string_in_file(&dir_entry?.path(), from.clone(), to.clone())?;
        }
    }

    Ok(())
}

pub fn replace_string_in_file(path: &PathBuf, from: String, to: String) -> Result<()> {
    let data = fs::read_to_string(path)?;
    let new = data.replace(&from, &to);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(new.as_bytes())?;

    Ok(())
}
