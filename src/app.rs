use std::fs;
use std::path::PathBuf;

use crate::common::sk_cfg_dir;
use crate::skeleton::Skeleton;

pub struct App {
    pub items: Vec<Skeleton>,
}

impl App {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn get_items_from_dir(&mut self, path: PathBuf) -> Result<(), std::io::Error> {
        let paths = fs::read_dir(path)?;

        for dir_entry_res in paths {
            let item_path_buf = dir_entry_res?.path();
            self.items.push(Skeleton::from_path_buf(item_path_buf));
        }

        Ok(())
    }
}
