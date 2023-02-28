use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::common::{sk_cfg_dir, list_skeleton_vec, open_vim, path_buf_to_string};
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

    pub fn get_skeleton_by_id(&self, id: &str) -> Option<&Skeleton> {
        for item in self.items.iter() {
            if item.id == id.to_string() {
                return Some(item)
            }
        }
        None
    }

    // Cli functions

    pub fn list(&self) {
        list_skeleton_vec(&self.items);
    }

    pub fn edit(&self, skeleton_str: &str) -> Result<(), std::io::Error> {
        if let Some(skeleton) = self.get_skeleton_by_id(skeleton_str) {
            open_vim(&skeleton.path)?;
        } else {
            eprintln!("ERROR: Error opening skeleton to edit (App::edit())");
        }

        Ok(())
    }

    pub fn add(&self, id: &str) {
        let mut path: PathBuf = sk_cfg_dir();
        path.push(format!("{}.sk", id));
        if path.exists() {
            eprintln!("ERROR: Skeleton at {} already exists (App::add())", path_buf_to_string(&path));
        } else {
            open_vim(&path).unwrap_or_else(
                |err| eprintln!("ERROR: Error opening file in vim (App::add() {})", err)
            );
        }
    }

}


// Command Line Interface

//     Commands:
//     List                - List all configured skeletons
//     Edit <Skeleton>     - Edit a skeleton
//     Add <Name>          - Configure new skeleton
//     Add --source <Path> - Configure new skeleton from path
//     New <Path>          - Copy skeleton to specified directory
//     Remove <Skeleton>   - Remove configured skeleton and its files

//     Usage Examples:
//     sk list
//     sk edit rust (opens vim with the rust sk file/dir)
//     sk add rust (todo! maybe interactive dir creator)
//     sk add --source rust_sk/
//     sk new rust
//     sk remove javascript
