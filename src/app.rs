use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::cli::Commands;
use crate::settings::Settings;
use crate::common::{copy_recursively, is_yes, list_skeleton_vec, open_editor, path_buf_to_string, touch, skeletons_cfg_dir, check_cfg, replace_string_in_dir};

use crate::skeleton::Skeleton;

const PLACEHOLDER: &str = "PLACEHOLDER_NAME";

/// Central data structure for skely
pub struct App {
    pub items: Vec<Skeleton>,
    pub settings: Settings,
}

impl App {
    pub fn new() -> Self {
         Self { items: Vec::new() , settings: Settings::new() }
    }

    pub fn default() -> Result<Self> {
        check_cfg()?;

        let mut app: Self = Self::new();
        app.get_items_from_dir(skeletons_cfg_dir()?)
            .context("Could not fetch items from skely config directory")?;

        app.settings = Settings::default()?;

        Ok(app)

    }

    pub fn get_items_from_dir(&mut self, path: PathBuf) -> Result<()> {
        let paths = fs::read_dir(path)?;

        for dir_entry_res in paths {
            let item_path_buf = dir_entry_res?.path();
            self.items.push(Skeleton::from_path_buf(item_path_buf));
        }

        Ok(())
    }

    pub fn get_skeleton_by_id(&self, id: &str) -> Option<&Skeleton> {
        self.items.iter().find(|&item| item.id == *id.to_string())
    }

    pub fn handle_command(&self, command: Commands) -> Result<()> {
        match command {
            Commands::List { verbose } => self.list(verbose)?,
            Commands::Edit { id } => self.edit(id)?,
            Commands::Add {
                name,
                source,
                touch,
            } => self.add(name, source, touch)?,
            Commands::New { id, path, name } => self.new_project(id, path, name)?,
            Commands::Remove { id } => self.remove(id)?,
        }

        Ok(())
    }

    pub fn list(&self, verbose: bool) -> Result<()> {
        list_skeleton_vec(&self.items, verbose)?;
        Ok(())
    }

    pub fn edit(&self, skeleton_str: String) -> Result<()> {
        if let Some(skeleton) = self.get_skeleton_by_id(&skeleton_str) {
            open_editor(&skeleton.path, &self.settings.editor)?;
            Ok(())
        } else {
            Err(anyhow!("Skeleton not found"))
        }
    }

    pub fn add(&self, id: String, source: Option<PathBuf>, touch_var: bool) -> Result<()> {
        let mut path: PathBuf = skeletons_cfg_dir()?;
        path.push(format!("{id}.sk"));
        if path.exists() {
            Err(anyhow!(format!(
                "Skeleton at {} already exists",
                path_buf_to_string(&path)
            )))
        } else {
            match source {
                Some(source) => {
                    let mut dest_dir = skeletons_cfg_dir()?;
                    if source.is_dir() {
                        dest_dir.push(source.components().last().unwrap());
                        copy_recursively(source, dest_dir)
                            .context("Failed to copy directory recursivley")?;
                    } else if source.is_file() {
                        dest_dir.push(format!("{id}.sk"));
                        fs::copy(source, dest_dir)?;
                    }
                }
                None => {
                    if !touch_var {
                        open_editor(&path, &self.settings.editor).context("Failed to open editor")?;
                    } else {
                        touch(&path).context("Failed to create file")?;
                    }
                }
            }
            Ok(())
        }
    }

    pub fn new_project(&self, id: String, mut path: PathBuf, name: Option<String>) -> Result<()> {
        if let Some(skeleton) = self.get_skeleton_by_id(&id) {

            if path.exists() {
                return Err(anyhow!("Target directory already exists"));
            }

            if path_buf_to_string(&path) == "." {
                println!(
                    "This will copy all files in skeleton {id} to your current working directory."
                );
                println!("Are you sure you want to do this? (y/n) ");
                let mut input: String = String::new();
                std::io::stdin().read_line(&mut input)?;
                input.truncate(input.len() - 1);
                if is_yes(&input)? {
                    skeleton.copy_to_dir(&mut path)?;
                }
            } else {
                skeleton.copy_to_dir(&mut path)?;
            }

            let project_name = match name {
                Some(name_val) => name_val,
                None => path.file_name().unwrap().to_str().unwrap().to_string(),
            };

            let placeholder_name = match self.settings.placeholder.to_owned() {
                Some(name_val) => name_val,
                None => "TEMPLATE".to_string(),
            };
            replace_string_in_dir(&path, placeholder_name.to_string(), project_name)?;

            Ok(())
        } else {
            Err(anyhow!("Skeleton not found"))
        }
    }

    pub fn remove(&self, id: String) -> Result<()> {
        if let Some(skeleton) = self.get_skeleton_by_id(&id) {
            println!(
                "Are you sure you want to delete {}? (y/n) ",
                path_buf_to_string(&skeleton.path)
            );
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.truncate(input.len() - 1);
            if is_yes(&input)? {
                match skeleton.path.is_file() {
                    true => fs::remove_file(&skeleton.path)?,
                    false => fs::remove_dir_all(&skeleton.path)?,
                }
            }
            Ok(())
        } else {
            Err(anyhow!("Skeleton not found"))
        }
    }
}
