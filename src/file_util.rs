use std::{fs, io::Write, path::{Path, PathBuf}, process::Command};

use anyhow::{anyhow, Context, Result};

pub fn path_buf_to_string(p: &PathBuf) -> String {
    p.to_path_buf().into_os_string().into_string().unwrap()
}

pub fn touch(path: &PathBuf) -> Result<()> {
    match fs::OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn tilda_ize_path_str(item: &str) -> Result<String> {
    if let Some(path_buf) = home::home_dir() {
        if let Some(str_opt) = path_buf.as_os_str().to_str() {
            Ok(item.replace(str_opt, "~"))
        } else {
            Err(anyhow!("Could not convert path_buf to &str"))
        }
    } else {
        Err(anyhow!("Could not extract value from Option<Pathbuf>"))
    }
}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
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

pub fn open_editor(arg: &PathBuf, editor_opt: &Option<String>) -> Result<()> {
    match editor_opt {
        Some(editor) => {
            let output = Command::new("which")
                .arg(editor)
                .output()
                .context("Failed to execute command")?;

            if output.status.success() {
                Command::new(editor).arg(arg).spawn()?.wait()?;
            } else {
                return Err(anyhow!("Editor not found"));
            }
        }
        None => {
            // TODO: use $EDITOR
            #[rustfmt::skip]
            // Editors (in order)
            let editors = vec![
                "nvim",
                "hx", // helix
                "vim",
                "micro",
                "nano",
                "emacs", // dont be mad, i use emacs, its just slow for these purposes
                "vi",
                "pico",
                "amp",
                "ne", // nice editor :)
            ];

            for editor in editors {
                let output = Command::new("which")
                    .arg(editor)
                    .output()
                    .context("Failed to execute command")?;

                if output.status.success() {
                    Command::new(editor).arg(arg).spawn()?.wait()?;
                    break;
                }
            }
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
