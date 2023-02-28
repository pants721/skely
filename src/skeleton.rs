use crate::common::{check_cfg_dir, copy_recursively, path_buf_to_string, sk_cfg_dir, touch};
use home::home_dir;
use std::fs::create_dir_all;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Skeleton {
    pub id: String,
    pub path: PathBuf,
}

impl Skeleton {
    pub fn new(id: &str) -> Self {
        let mut path: PathBuf = sk_cfg_dir();
        let id_lower: String = id.to_lowercase();
        path.push(format!("{}.sk", id));
        check_cfg_dir();
        match touch(path.as_path()) {
            Ok(_) => (),
            Err(err) => eprintln!(
                "ERROR: Error creating config file (Skeleton::new() {})",
                err.to_string()
            ),
        }

        Self { id: id_lower, path }
    }

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
