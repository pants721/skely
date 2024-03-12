use anyhow::{anyhow, Context, Result};
use cli::{Cli, Commands};
use colored::{ColoredString, Colorize};
use std::env;
use util::{replace_string_in_dir, replace_string_in_filenames, sk_cfg_path};
use std::{path::PathBuf};
use std::fs;

use clap::{Parser};

use crate::util::{copy_recursively, path_buf_filename, path_buf_to_string};

mod cli;
mod util;

static PLACEHOLDER_ENV_VAR: &str = "SK_PLACEHOLDER";

fn main() -> Result<()> {
    let args = Cli::parse(); 
    match args.command {
        Commands::Add { id, source } => add(source, id)?,
        Commands::New { id, path, name } => new(id, path, name)?,
        Commands::List => list()?,
        Commands::Remove { id, no_confirm } => remove(id, no_confirm)?,
    }

    Ok(())
}

fn add(source: PathBuf, id: Option<String>) -> Result<()> {
    let dest_path: PathBuf = sk_cfg_path()?.join("skeletons").join(
        match id {
            Some(id) => id,
            None => source.file_name().unwrap().to_str().unwrap().to_string()
        }
    );

    if !source.exists() {
        return Err(anyhow!(
            "Could not find file {:?}",
            dest_path.display()
        ))
    }

    if dest_path.exists() {
        return Err(anyhow!(
            "Skeleton at {:?} already exists!", 
            dest_path.display()
        ))
    }

    if source.is_dir() {
        copy_recursively(source, dest_path).context("Failed to copy source to skeletons directory")?;
    } else if source.is_file() {
        fs::copy(source, dest_path).context("Failed to copy source to skeletons directory")?;
    }

    Ok(())
}

fn new(id: String, path: Option<PathBuf>, name: Option<String>) -> Result<()> {
    let mut dest_path = path.unwrap_or(PathBuf::from(&id));
    let inferred_project_name = path_buf_filename(&dest_path)?;
    let skeleton_path = sk_cfg_path()?.join("skeletons").join(&id);

    if !skeleton_path.exists() {
        return Err(anyhow!("Could not find skeleton"));
    }

    if skeleton_path.is_file() {
        if dest_path.is_dir() {
            dest_path.push(&id);
        } 

        if dest_path.is_file() {
            return Err(anyhow!("Destination file already exists"));
        }

        // FIXME: Hacky
        if !dest_path.exists() && path_buf_to_string(&dest_path)?.ends_with('/') {
            return Err(anyhow!("Target directory does not exist"));
        }

        fs::File::create(&dest_path)?;
        fs::copy(&skeleton_path, &dest_path)?;
    } else if skeleton_path.is_dir() {
        if (dest_path.is_dir() && dest_path.read_dir()?.next().is_some()) || (dest_path.is_file()) {
            return Err(anyhow!("Target directory already exists and is not an empty directory"));
        }

        copy_recursively(&skeleton_path, &dest_path)?;
    }

    let placeholder = env::var(PLACEHOLDER_ENV_VAR).unwrap_or("PLACEHOLDER".to_string());

    let project_name = name.unwrap_or(inferred_project_name);

    replace_string_in_dir(&dest_path, &placeholder, &project_name)?;
    replace_string_in_filenames(&dest_path, &placeholder, &project_name)?;
    
    Ok(())
}

fn list() -> Result<()> {
    for entry in fs::read_dir(sk_cfg_path()?.join("skeletons"))?.flatten() {
        let path = entry.path();
        // lmao nice "refactor"
        let mut id: ColoredString = path_buf_filename(&path)?.into();

        if path.is_dir() {
            id = id.blue().bold();
        } else {
            id = id.white();
        }

        print!("{}  ", &id);
    }

    println!();

    Ok(())
}

fn remove(id: String, _no_confirm: bool) -> Result<()> {
    let skeleton_path = sk_cfg_path()?.join("skeletons").join(id);

    if !skeleton_path.exists() {
        return Err(anyhow!("Could not find skeleton"));
    }

    match skeleton_path.is_file() {
        true => fs::remove_file(&skeleton_path)?,
        false => fs::remove_dir_all(&skeleton_path)?,
    }

    Ok(())
}
