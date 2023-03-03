use anyhow::{anyhow, Context, Result};
use home::home_dir;
use colored::*;
use std::fs;
use std::fs::{create_dir_all, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::app::App;
use crate::skeleton::Skeleton;

pub fn startup(app: &mut App) -> Result<()> {
    check_cfg_dir()?;
    app.get_items_from_dir(sk_cfg_dir()?)
        .context("Could not fetch items from skelly config directory")?;
    Ok(())
}

#[allow(dead_code)]
pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn check_cfg_dir() -> Result<()> {
    let path = sk_cfg_dir()?;
    if !path.exists() {
        create_dir_all(path).context("Could not create skelly config directory")?;
    }
    Ok(())
}

pub fn sk_cfg_dir() -> Result<PathBuf> {
    let mut path: PathBuf = PathBuf::new();
    if let Some(home_dir_path) = home_dir() {
        path.push(home_dir_path);
        path.push(".config");
        path.push("sk");
        Ok(path)
    } else {
        Err(anyhow!("Could not fetch home directory"))
    }
}

pub fn list_skeleton_vec(items: &[Skeleton], verbose: bool) -> Result<()> {
    for item in items.iter() {
        if let Some(item_path) = item.path.to_str() {
            let single_file_str: &str;
            let id_styled;
            if item.path.is_file() {
                single_file_str = "File";
                id_styled = item.id.to_string().white();
            } else {
                single_file_str = "Directory";
                id_styled = item.id.to_string().blue().bold();
            };
            if !verbose {
                print!("{}  ", &id_styled);
                if &item == &items.iter().last().unwrap() {
                    println!("");
                }
            } else if verbose {
                println!(
                    "  {} [{}]: {}",
                    &id_styled,
                    tilda_ize_path_buf(item_path)?,
                    single_file_str
                );
            }
        }
    }
    Ok(())
}

// pub fn capitalize(s: &str) -> String {
//     let mut c = s.chars();
//     match c.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
//     }
// }

fn tilda_ize_path_buf(item: &str) -> Result<String> {
    if let Some(path_buf) = home_dir() {
        if let Some(str_opt) = path_buf.as_os_str().to_str() {
            Ok(item.replace(str_opt, "~"))
        } else {
            Err(anyhow!("Could not convert path_buf to &str"))
        }
    } else {
        Err(anyhow!("Could not extract value from Option<Pathbuf>"))
    }
}

pub fn path_buf_to_string(p: &Path) -> String {
    p.to_path_buf().into_os_string().into_string().unwrap()
}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
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

pub fn open_vim(arg: &PathBuf) -> Result<()> {
    Command::new("vim").arg(arg).spawn()?.wait()?;
    Ok(())
}

pub fn is_yes(input: &str) -> Result<bool> {
    let input = input.to_lowercase();
    if input == "yes" || input == "y" {
        Ok(true)
    } else if input == "no" || input == "n" {
        Ok(false)
    } else {
        Err(anyhow!("Invalud user input"))
    }
}
