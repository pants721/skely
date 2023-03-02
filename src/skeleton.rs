// use crate::common::{check_cfg_dir, sk_cfg_dir, touch};
use crate::common::copy_recursively;
use std::fs::create_dir_all;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Skeleton {
    pub id: String,
    pub path: PathBuf,
}

impl Skeleton {
    // pub fn new(id: &str) -> Self {
    //     let mut path: PathBuf = sk_cfg_dir();
    //     let id_lower: String = id.to_lowercase();
    //     path.push(format!("{id}.sk"));
    //     check_cfg_dir();
    //     match touch(path.as_path()) {
    //         Ok(_) => (),
    //         Err(err) => eprintln!("ERROR: Error creating config file (Skeleton::new() {err})"),
    //     }

    //     Self { id: id_lower, path }
    // }

    pub fn from_path_buf(path: PathBuf) -> Self {
        let mut trimmed_file_name = String::new();
        if let Some(file_name) = path.file_name() {
            trimmed_file_name = file_name.to_string_lossy().replace(".sk", "");
        }

        Self {
            id: trimmed_file_name,
            path,
        }
    }

    pub fn copy_to_dir(&self, mut path: PathBuf) -> Result<(), std::io::Error> {
        if !path.exists() {
            create_dir_all(&path)?;
        }

        if self.path.is_file() {
            path.push(format!("{}.sk", &self.id));
            fs::copy(&self.path, &path)?;
        } else if path.is_dir() {
            copy_recursively(&self.path, &path)?;
        }

        Ok(())
    }
}
