use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{io::Write, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use home::home_dir;

pub fn sk_cfg_path() -> Result<PathBuf> {
    match home_dir() {
        Some(h) => Ok(h.join(".config").join("sk")),
        None => Err(anyhow!("Could not get sk config directory")),
    }
}

pub fn path_buf_filename(path: &Path) -> Result<String> {
    Ok(path.file_name().unwrap().to_str().unwrap().to_string())
}

pub fn path_buf_to_string(p: &Path) -> Result<String> {
    match p.to_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(anyhow!("could not convert string to PathBuf")),
    }
}

pub fn copy_recursively(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create directory {}", destination.display()))?;
    for entry in fs::read_dir(source)
        .with_context(|| format!("failed to read source directory {}", source.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let dest_path = destination.join(entry.file_name());
        if path.is_dir() {
            copy_recursively(&path, &dest_path)?;
        } else if path.is_file() {
            fs::copy(&path, &dest_path).with_context(|| {
                format!(
                    "failed to copy file {} to {}",
                    &entry.path().display(),
                    &destination.display()
                )
            })?;
        }
    }

    Ok(())
}

pub fn replace_string_in_dir(input_path: &PathBuf, from: &str, to: &str) -> Result<()> {
    if input_path.is_file() {
        replace_string_in_file(input_path, from, to)
            .with_context(|| format!("failed to replace placeholder string in {}", input_path.display()))?;
        return Ok(());
    }

    let paths = fs::read_dir(input_path)?;

    for dir_entry in paths {
        let entry = match dir_entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                continue; // Skip to the next entry if there's an error
            }
        };
        
        let entry_path = entry.path();
        if entry_path.is_dir() {
            if let Err(e) = replace_string_in_dir(&entry_path, from, to) {
                eprintln!("{}", e.context(format!("failed to replace string in {}", entry_path.display())));
            }
        } else if let Err(e) = replace_string_in_file(&entry_path, from, to) {
            eprintln!(
                "{}", 
                &e.context(format!("failed to replace string in {}", entry_path.display()))
            );
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
    if is_executable(path)? {
        return Err(anyhow!("{} is executable", path.display()));
    }

    let data =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let new = data.replace(from, to);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(new.as_bytes())
        .with_context(|| format!("failed to write {}", path.display()))?;

    Ok(())
}

/// returns new path
pub fn replace_string_in_filename(path: &PathBuf, from: &str, to: &str) -> Result<PathBuf> {
    let file_name = path_buf_filename(path)?;
    // we use this and not Path::file_stem() because we want PLACEHOLDER.tar.gz to convert to
    // REPLACED.tar.gz not REPLACED.gz
    if file_name.split('.').next() == Some(from) {
        let new_name = file_name.replace(from, to);
        let new_path = path.parent().unwrap().join(new_name);

        fs::rename(path, &new_path)?;
        return Ok(new_path);
    }
    Ok(path.to_path_buf())
}

pub fn replace_string_in_filenames(path: &PathBuf, from: &str, to: &str) -> Result<()> {
    let new_path = replace_string_in_filename(path, from, to)?;
    if new_path.is_dir() {
        for entry in fs::read_dir(&new_path).with_context(|| format!("failed to read {}", &new_path.display()))?
            .flatten()
            {
                let entry_path = entry.path();
                replace_string_in_filenames(&entry_path, from, to)?;
            }
    }
    Ok(())
}
