use anyhow::{anyhow, Context, Result};
use cli::{Cli, Commands};
use colored::{ColoredString, Colorize};
use util::sk_cfg_path;
use std::{fs::create_dir_all, path::PathBuf};
use std::fs;

use clap::{Parser, Subcommand};

use crate::util::{copy_recursively, path_buf_to_string};

mod cli;
mod util;

fn main() -> Result<()> {
    run()?;

    Ok(())
}

fn run() -> Result<()> {
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
        if !dest_path.exists() && path_buf_to_string(&dest_path)?.ends_with("/") {
            return Err(anyhow!("Target directory does not exist"));
        }

        fs::File::create(&dest_path)?;
        fs::copy(&skeleton_path, &dest_path)?;
    } else if skeleton_path.is_dir() {
        if (dest_path.is_dir() && !dest_path.read_dir()?.next().is_none()) || (dest_path.is_file()) {
            return Err(anyhow!("Target directory already exists and is not an empty directory"));
        }

        copy_recursively(&skeleton_path, &dest_path)?;
    }

    // TODO: Add name replacing
    
    Ok(())
}

fn list() -> Result<()> {
    for entry in fs::read_dir(sk_cfg_path()?.join("skeletons"))? {
        match entry {
            Ok(n) => {
                let path = n.path();
                // lmao nice "refactor"
                let mut id: ColoredString = path.file_name().unwrap().to_str().unwrap().to_string().into();

                if path.is_dir() {
                    id = id.blue().bold();
                } else {
                    id = id.white();
                }

                print!("{}  ", &id);
            },
            Err(_) => (),
        }
    }

    println!();

    Ok(())
}

fn remove(id: String, no_confirm: bool) -> Result<()> {
    let skeleton_path = sk_cfg_path()?.join("skeletons").join(&id);

    if !skeleton_path.exists() {
        return Err(anyhow!("Could not find skeleton"));
    }

    match skeleton_path.is_file() {
        true => fs::remove_file(&skeleton_path)?,
        false => fs::remove_dir_all(&skeleton_path)?,
    }

    Ok(())
}
