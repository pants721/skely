use std::{fs, io::Write, path::{Path, PathBuf}, process::Command, env};

use anyhow::{anyhow, Context, Result};

pub fn path_buf_to_string(p: &PathBuf) -> Result<String> {
    match p.to_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(anyhow!("Could not convert string to PathBuf")),
    }
}

pub fn touch(path: &PathBuf) -> Result<()> {
    match fs::OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn tilda_ize_path_str(item: &str) -> Result<String> {
    // TODO: Validate item
    let home_dir = home::home_dir().context("Could not get home directory")?;
    let home_str = path_buf_to_string(&home_dir)?;
    Ok(item.replace(&home_str, "~"))
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

pub fn open_editor(arg: &PathBuf, editor: &Option<String>) -> Result<()> {
    match editor {
        Some(editor) => {
            let output = Command::new("which")
                .arg(editor)
                .output()?;

            if output.status.success() {
                Command::new(editor).arg(arg).spawn()?.wait()?;
            } else {
                return Err(anyhow!(format!("Editor \"{}\" not found", editor)));
            }
        }
        None => {
            let editor = env::var_os("EDITOR").unwrap_or("vim".into());
            Command::new(editor)
                .arg(arg)
                .spawn()
                .context("$EDITOR enviroment variable is set incorrectly")?
                .wait()?;
        }
    }
    Ok(())
}

// Spaghetti code FIX PLEASE
pub fn replace_string_in_dir(input_path: &PathBuf, from: String, to: String) -> Result<()> {
    let paths = fs::read_dir(input_path)?;

    for dir_entry in paths {
        if dir_entry.as_ref().unwrap().path().is_dir() {
            replace_string_in_dir(&dir_entry?.path(), from.clone(), to.clone())?;
        } else {
            let data = fs::read_to_string(dir_entry.as_ref().unwrap().path())?;
            let new = data.replace(&from, &to);
            let mut file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(dir_entry?.path())?;
            file.write_all(new.as_bytes())?;
        }
    }

    Ok(())
}
