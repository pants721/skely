use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;

use crate::cli::Commands;
use crate::cli_util::is_yes;
use crate::file_util;
use crate::settings::Settings;

use crate::skeleton::Skeleton;

/// Central data structure for skely
pub struct App {
    pub items: Vec<Skeleton>,
    pub settings: Settings,
}

impl App {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            settings: Settings::new(),
        }
    }

    pub fn default() -> Result<Self> {
        Self::create_config()?;

        let mut app: Self = Self::new();
        app.items_from_dir(Self::skeletons_path()?)
            .context("Could not fetch items from skely config directory")?;

        app.settings = Settings::load(Self::config_file_path()?)?;
        Settings::create_default_cfg_file(Self::config_file_path()?)?;

        Ok(app)
    }

    pub fn create_config() -> Result<()> {
        let path = Self::skeletons_path()?;
        if !path.exists() {
            create_dir_all(Self::skeletons_path()?).context("Could not create config directory")?;
        }

        if !Self::config_file_path()?.exists() {
            eprintln!("Config file (config.toml) does not exist. Creating...");
            Settings::create_default_cfg_file(Self::config_file_path()?)?;
        }
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let mut path: PathBuf = PathBuf::new();
        if let Some(home_dir_path) = home::home_dir() {
            path.push(home_dir_path);
            path.push(".config");
            path.push("sk");
            Ok(path)
        } else {
            Err(anyhow!("Could not fetch home directory"))
        }
    }

    pub fn config_file_path() -> Result<PathBuf> {
        let mut file_dir = Self::config_path()?;
        file_dir.push("config.toml");
        Ok(file_dir)
    }

    pub fn skeletons_path() -> Result<PathBuf> {
        let mut file_dir = Self::config_path()?;
        file_dir.push("skeletons");
        Ok(file_dir)
    }

    pub fn items_from_dir(&mut self, path: PathBuf) -> Result<()> {
        let paths = fs::read_dir(path)?;

        for dir_entry_res in paths {
            let item_path_buf = dir_entry_res?.path();
            self.items.push(Skeleton::from_path_buf(item_path_buf));
        }

        Ok(())
    }

    pub fn skeleton_by_id(&self, id: &str) -> Option<&Skeleton> {
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
            Commands::Remove { id, no_confirm } => self.remove(id, no_confirm)?,
        }

        Ok(())
    }

    pub fn list(&self, verbose: bool) -> Result<()> {
        self.print_skeletons(verbose)?;
        Ok(())
    }

    pub fn edit(&self, skeleton_str: String) -> Result<()> {
        if let Some(skeleton) = self.skeleton_by_id(&skeleton_str) {
            file_util::open_editor(&skeleton.path, &self.settings.editor)?;
            Ok(())
        } else {
            Err(anyhow!("Skeleton not found"))
        }
    }

    pub fn add(&self, id: String, source: Option<PathBuf>, touch_var: bool) -> Result<()> {
        let mut path: PathBuf = Self::skeletons_path()?;
        path.push(format!("{id}.sk"));
        if path.exists() {
            Err(anyhow!(format!(
                "Skeleton at {} already exists",
                file_util::path_buf_to_string(&path)
            )))
        } else {
            match source {
                Some(source) => {
                    let mut dest_dir = Self::skeletons_path()?;
                    if source.is_dir() {
                        // dest_dir.push(source.components().last().unwrap());
                        dest_dir.push(&id);
                        file_util::copy_recursively(source, dest_dir)
                            .context("Failed to copy directory recursivley")?;
                    } else if source.is_file() {
                        dest_dir.push(format!("{id}.sk"));
                        fs::copy(source, dest_dir)?;
                    }
                }
                None => {
                    if !touch_var {
                        file_util::open_editor(&path, &self.settings.editor)
                            .context("Failed to open editor")?;
                    } else {
                        file_util::touch(&path).context("Failed to create file")?;
                    }
                }
            }
            Ok(())
        }
    }

    pub fn new_project(&self, id: String, path: Option<PathBuf>, name: Option<String>) -> Result<()> {
        let mut path = match path {
            Some(p) => p,
            None => PathBuf::from(&id),
        };

        if path.exists() {
            return Err(anyhow!("Target directory already exists"));
        }

        if let Some(skeleton) = self.skeleton_by_id(&id) {
            if path.exists() {
                return Err(anyhow!("Target directory already exists"));
            }

            if file_util::path_buf_to_string(&path) == "." {
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
                // dont like this
                None => path
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
            };

            if let Some(name_val) = self.settings.placeholder.to_owned() {
                file_util::replace_string_in_dir(&path, name_val, project_name)?;
            };

            Ok(())
        } else {
            Err(anyhow!("Skeleton not found"))
        }
    }

    pub fn remove(&self, id: String, no_confirm: bool) -> Result<()> {
        if let Some(skeleton) = self.skeleton_by_id(&id) {
            let mut input: String = "yes".to_string();
            if !no_confirm {
                println!(
                    "Are you sure you want to delete {}? (y/n) ",
                    file_util::path_buf_to_string(&skeleton.path)
                );
                std::io::stdin().read_line(&mut input)?;
                input.truncate(input.len() - 1);
            }

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

    // TODO: Pass writer
    pub fn print_skeletons(&self, verbose: bool) -> Result<()> {
        for item in self.items.iter() {
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
                    if item == self.items.iter().last().unwrap() {
                        println!();
                    }
                } else if verbose {
                    println!(
                        "  {} [{}]: {}",
                        &id_styled,
                        file_util::tilda_ize_path_str(item_path)?,
                        single_file_str
                        );
                }
            }
        }
        Ok(())
    }
}
