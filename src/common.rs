use anyhow::{anyhow, Context, Result};
use colored::*;
use home::home_dir;
use std::fs;
use std::fs::{create_dir_all, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::skeleton::Skeleton;

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
        create_dir_all(path).context("Could not create skely config directory")?;
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
                single_file_str = "Single File";
                id_styled = item.id.to_string().white();
            } else {
                single_file_str = "Project";
                id_styled = item.id.to_string().blue().bold();
            };
            if !verbose {
                print!("{}  ", &id_styled);
                if item == items.iter().last().unwrap() {
                    println!();
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

mod tests {
    #[test]
    fn is_yes_test() {
        use super::is_yes;
        assert!(is_yes("y").unwrap());
        assert!(is_yes("yes").unwrap());
        assert!(is_yes("Y").unwrap());
        assert!(is_yes("yEs").unwrap());

        assert!(!is_yes("n").unwrap());
        assert!(!is_yes("no").unwrap());
        assert!(!is_yes("N").unwrap());
        assert!(!is_yes("nO").unwrap());

        assert!(is_yes("balls").is_err());
        assert!(is_yes("yesno").is_err());
        assert!(is_yes("I <3 Mia").is_err());
    }

    #[test]
    fn path_buf_to_string_test() {
        use std::path::PathBuf;
        use super::path_buf_to_string;
        let mut path1: PathBuf = PathBuf::new();
        path1.push("the");
        path1.push("dread");
        path1.push("pirate");
        path1.push("roberts");
        assert_eq!(path_buf_to_string(&path1), "the/dread/pirate/roberts");
    }
}
