use std::os::unix::fs::PermissionsExt;
use std::{io::Write, path::PathBuf};
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
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

pub fn replace_string_in_dir(input_path: &PathBuf, from: &str, to: &str) -> Result<()> {
    if input_path.is_file() {
        replace_string_in_file(input_path, from, to).context(format!("Failed to replace placeholder string in {}", input_path.display()))?;
        return Ok(());
    }

    let paths = fs::read_dir(input_path)?;

    for dir_entry in paths {
        let entry_path = dir_entry?.path();
        if entry_path.is_dir() {
            replace_string_in_dir(&entry_path, from, to).context(format!("Failed to replace placeholder string in {}", entry_path.display()))?;
        } else {
            replace_string_in_file(&entry_path, from, to).context(format!("Failed to replace placeholder string in {}", entry_path.display()))?;
        }
    }

    Ok(())
}

// SOURCE: https://www.reddit.com/r/rust/comments/leewn4/how_to_check_if_a_file_is_executable/
fn is_executable(path: &Path) -> Result<bool> {
    let permissions = path.metadata()?.permissions();

    Ok(permissions.mode() & 0o111 != 0)
}

pub fn replace_string_in_file(path: &PathBuf, from: &str, to: &str) -> Result<()> {
    if is_executable(&path)? {
        return Ok(())
    }

    let data = fs::read_to_string(path).context(format!("Failed to read {}", path.display()))?;
    let new = data.replace(&from, &to);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(new.as_bytes())?;

    Ok(())
}

// FIXME: something about this code feels innificent like control flow wise but im tired
pub fn replace_string_in_filenames(path: &PathBuf, from: &str, to: &str) -> Result<()> {
    for entry in fs::read_dir(path)?.flatten() {
        let path = entry.path(); 

        let file_name = path_buf_filename(&path)?;
        if file_name.split('.').next().unwrap() == from {
            let new_name = file_name.replace(&from, &to);
            let new_path = path.parent().unwrap().join(new_name);

            fs::rename(&path, &new_path)?;

            if new_path.is_dir() {
                replace_string_in_filenames(&new_path, from, to)?;
            }
        } else {
            replace_string_in_filenames(&path, from, to)?;
        }
    }
    Ok(())
}
