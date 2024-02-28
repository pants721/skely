use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use itertools::Itertools;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;

use crate::cli::Commands;
use crate::cli_util::{self, is_yes};
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
            create_dir_all(path).context("Could not create config directory")?;
        }

        if !Self::config_file_path()?.exists() {
            eprintln!("Config file (config.toml) does not exist. Creating...");
            Settings::create_default_cfg_file(Self::config_file_path()?)?;
        }
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        match home::home_dir() {
            Some(home) => {
                Ok([home, ".config".into(), "sk".into()].iter().collect()) 
            },
            None => Err(anyhow!("Could not fetch home directory")),
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

        // for dir_entry_res in paths {
        //     let item_path_buf = dir_entry_res?.path();
        //     self.items.push(Skeleton::from_path_buf(item_path_buf)?);
        // }
        // FIXME: Really bad code
        self.items.extend(
            paths.into_iter()
            .map_ok(|dir_entry| dir_entry.path())
            .filter_map(|x| x.ok())
            .map(|path| Skeleton::from_path_buf(path))
            .filter_map(|x| x.ok())
        );


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
        match self.skeleton_by_id(&skeleton_str) {
            Some(skeleton) => file_util::open_editor(&skeleton.path, &self.settings.editor),
            None => Err(anyhow!("Skeleton not found")),
        }
    }

    pub fn add(&self, id: String, source: Option<PathBuf>, touch: bool) -> Result<()> {
        let mut path: PathBuf = Self::skeletons_path()?;
        path.push(&id);

        if (path.exists() && path.is_dir()) || path.with_extension("sk").exists() {
            return Err(anyhow!(format!(
                "Skeleton at {} already exists",
                file_util::path_buf_to_string(&path)?
            )));
        } 

        match source {
            Some(source) => {
                if source.is_dir() {
                    file_util::copy_recursively(source, path)
                        .context("Failed to copy directory to skely folder")?;
                } else if source.is_file() {
                    fs::copy(source, path.with_extension("sk"))?;
                }
            }
            None => {
                if !touch {
                    file_util::open_editor(&path.with_extension("sk"), &self.settings.editor)
                        .context("Failed to open editor")?;
                } else {
                    file_util::touch(&path.with_extension("sk")).context("Failed to create file")?;
                }
            }
        }
        Ok(())
    }

    pub fn new_project(&self, id: String, path: Option<PathBuf>, name: Option<String>) -> Result<()> {
        let mut path = match path {
            Some(p) => p,
            None => PathBuf::from(&id),
        };

        if path.exists() {
            return Err(anyhow!("Target directory already exists"));
        }

        match self.skeleton_by_id(&id) {
            Some(skeleton) => {
                if file_util::path_buf_to_string(&path)? == "." {
                    println!(
                        "This will copy all files in skeleton {id} to your current working directory."
                        );
                    println!("Are you sure you want to do this? (y/n) ");

                    let mut input: String = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    input.truncate(input.len() - 1);

                    if !is_yes(&input)? {
                        return Ok(());
                    }
                } 
                skeleton.copy_to_dir(&mut path)?;

                // FIXME: Bad
                let project_name = name.unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());
                match &self.settings.placeholder {
                    Some(placeholder) => {
                        file_util::replace_string_in_dir(&path, placeholder.to_string(), project_name)?;
                    }
                    None => ()
                };
                Ok(())
            },
            None => Err(anyhow!("Skeleton not found")),

        }
    }

    pub fn remove(&self, id: String, no_confirm: bool) -> Result<()> {
        match self.skeleton_by_id(&id) {
            Some(skeleton) => {
                if !no_confirm {
                    let mut input = String::new();
                    println!(
                        "Are you sure you want to delete {}? (y/n) ",
                        file_util::path_buf_to_string(&skeleton.path)?
                        );
                    std::io::stdin().read_line(&mut input)?;
                    if !cli_util::is_yes(&input)? {
                        return Ok(());
                    }
                }
                match skeleton.path.is_file() {
                    true => fs::remove_file(&skeleton.path)?,
                    false => fs::remove_dir_all(&skeleton.path)?,
                }
                Ok(())
            },
            None => Err(anyhow!("Skeleton not found")),
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
