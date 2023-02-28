use home::home_dir;
use std::fs;
use std::fs::{create_dir_all, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

use crate::skeleton::Skeleton;

pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn check_cfg_dir() {
    let path = sk_cfg_dir();
    if !path.exists() {
        create_dir_all(path).unwrap_or_else(|err| {
            eprintln!(
                "ERROR: Error creating config directory (check_cfg_dir() {})",
                err.to_string()
            )
        });
    }
}

pub fn sk_cfg_dir() -> PathBuf {
    let mut path: PathBuf = PathBuf::new();
    if let Some(home_dir_path) = home_dir() {
        path.push(home_dir_path);
        path.push(".config");
        path.push("sk");
    } else {
        eprintln!("ERROR: Error fetching home directory (sk_cfg_dir())");
        panic!()
    }
    path
}

pub fn list_skeleton_vec(items: &Vec<Skeleton>) {
    for (index, item) in items.iter().enumerate() {
        if let Some(item_path) = item.path.to_str() {
            let single_file_str: &str = if item.path.is_file() { "File" } else { "Dir" };
            println!(
                "{}. {} [{}]: {}",
                index,
                capitalize(&item.id),
                tilda_ize_path_buf(item_path),
                single_file_str
            );
        }
    }
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn tilda_ize_path_buf(item: &str) -> String {
    if let Some(path_buf) = home_dir() {
        if let Some(str_opt) = path_buf.as_os_str().to_str() {
            item.replace(str_opt, "~")
        } else {
            eprintln!("ERROR: Error converting path_buf to &str (tilda_ize_path_buf())");
            String::new()
        }
    } else {
        eprintln!("ERROR: Error extracting value from Option<PathBuf> (tilda_ize_path_buf())");
        String::new()
    }
}

pub fn path_buf_to_string(p: &PathBuf) -> String {
    p.clone().into_os_string().into_string().unwrap()
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