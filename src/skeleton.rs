use crate::common::{copy_recursively, path_buf_to_string};
// use crate::common::{check_cfg_dir, sk_cfg_dir, touch};
// use anyhow::Context;
use anyhow::{Result, Context};
use std::fs::create_dir_all;
use std::{fs, path::PathBuf};

/// Data structure for storing a skeleton project's information
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Skeleton {
    pub id: String,
    pub path: PathBuf,
}

impl Skeleton {
    // pub fn new(id: &str) -> Result<Self> {
    //     let mut path: PathBuf = sk_cfg_dir()?;
    //     let id_lower: String = id.to_lowercase();
    //     path.push(format!("{id}.sk"));
    //     check_cfg_dir()?;
    //     touch(path.as_path()).context("Could not create .sk file")?;

    //     Ok(Self { id: id_lower, path })
    // }

    /// Constructor for a skeleton from a specified path
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

    /// Copy skeleton to specified path
    pub fn copy_to_dir(&self, path: &mut PathBuf) -> Result<()> {
        if !path.exists() && self.path.is_dir() {
            create_dir_all(&path)?;
        }

        if self.path.is_file() {
            if path_buf_to_string(path) == "." {
                path.push(format!("{}.sk", &self.id));
            }
            fs::File::create(&path)?;
            fs::copy(&self.path, &path).context("Could not copy file")?;
        } else if self.path.is_dir() {
            copy_recursively(&self.path, &path).context("Could not copy directory")?;
        }

        Ok(())
    }
}
