use std::fs;
use std::path::PathBuf;

use crate::cli::Commands;
use crate::common::{
    copy_recursively, is_yes, list_skeleton_vec, open_vim, path_buf_to_string, sk_cfg_dir,
};
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
            if item.id == *id.to_string() {
                return Some(item);
            }
        }
        None
    }

    // Cli functions

    // - List                       - List all configured skeletons
    // - Edit <Skeleton>            - Edit a skeleton
    // - Add <Name>                 - Configure new skeleton
    // - Add <Name> --source <Path> - Configure new skeleton from path
    // - New <Path>                 - Copy skeleton to specified directory
    // - Remove <Skeleton>          - Remove configured skeleton and its files
    pub fn handle_command(&self, command: Commands) -> Result<(), std::io::Error> {
        match command {
            Commands::List => self.list(),
            Commands::Edit { id } => self.edit(id)?,
            Commands::Add { name, source } => self.add(name, source),
            Commands::New { id, path } => self.new_project(id, path)?,
            Commands::Remove { id } => self.remove(id)?,
        }

        Ok(())
    }

    pub fn list(&self) {
        list_skeleton_vec(&self.items);
    }

    pub fn edit(&self, skeleton_str: String) -> Result<(), std::io::Error> {
        if let Some(skeleton) = self.get_skeleton_by_id(&skeleton_str) {
            open_vim(&skeleton.path)?;
        } else {
            eprintln!("ERROR: Error opening skeleton to edit (App::edit())");
        }

        Ok(())
    }

    pub fn add(&self, id: String, source: Option<PathBuf>) {
        let mut path: PathBuf = sk_cfg_dir();
        path.push(format!("{id}.sk"));
        if path.exists() {
            eprintln!(
                "ERROR: Skeleton at {} already exists (App::add())",
                path_buf_to_string(&path)
            );
        } else {
            match source {
                Some(source) => {
                    if source.is_dir() {
                        let mut dest_dir = sk_cfg_dir();
                        dest_dir.push(&source.components().last().unwrap());
                        dbg!(&source);
                        dbg!(&dest_dir);
                        copy_recursively(source, dest_dir).unwrap_or_else(|err| {
                            eprintln!(
                                "ERROR: Error copying skeleton from source (App::add() {err})"
                            )
                        });
                    } else if source.is_file() {
                        let mut dest_dir = sk_cfg_dir();
                        dest_dir.push(format!("{id}.sk"));
                        fs::copy(source, dest_dir).unwrap();
                    }
                }
                None => {
                    open_vim(&path).unwrap_or_else(|err| {
                        eprintln!("ERROR: Error opening file in vim (App::add() {err})")
                    });
                }
            }
        }
    }

    pub fn new_project(&self, id: String, path: PathBuf) -> Result<(), std::io::Error> {
        if let Some(skeleton) = self.get_skeleton_by_id(&id) {
            if path_buf_to_string(&path) == "." {
                println!(
                    "This will copy all files in skeleton {id} to your current working directory."
                );
                println!("Are you sure you want to do this? (y/n) ");
                let mut input: String = String::new();
                std::io::stdin().read_line(&mut input)?;
                input.truncate(input.len() - 1);
                if is_yes(&input) {
                    skeleton.copy_to_dir(path)?;
                }
            } else {
                skeleton.copy_to_dir(path)?;
            }
        } else {
            eprintln!("ERROR: Error fetching skeleton by id (App::new_project())");
        }
        Ok(())
    }

    pub fn remove(&self, id: String) -> Result<(), std::io::Error> {
        if let Some(skeleton) = self.get_skeleton_by_id(&id) {
            println!(
                "Are you sure you want to delete {}? (y/n) ",
                path_buf_to_string(&skeleton.path)
            );
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.truncate(input.len() - 1);
            if is_yes(&input) {
                match skeleton.path.is_file() {
                    true => fs::remove_file(&skeleton.path)?,
                    false => fs::remove_dir_all(&skeleton.path)?,
                }
            }
        } else {
            eprintln!("ERROR: Error fetching skeleton by id (App::remove())");
        }

        Ok(())
    }
}
