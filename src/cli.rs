use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq)]
#[command(name = "sk")]
#[command(bin_name = "sk")]
#[command(about = "A command line tool for using and managing skeleton projects", long_about = None)]
#[command(author = "Lucas Newcomb (pants721)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum Commands {
    /// Lists all configured skeletons
    List {
        #[arg(short, long)]
        verbose: bool,
    },
    /// Opens skeleton to edit
    #[command(arg_required_else_help = true)]
    Edit {
        /// Id of skeleton to edit
        id: String,
    },
    /// Adds skeleton to configured skeletons
    Add {
        /// Name of skeleton
        #[arg(required = true)]
        name: String,
        /// Optional source to create skeleton from
        #[arg(short, long)]
        source: Option<PathBuf>,
        /// Creates .sk file without opening editor
        #[arg(short, long)]
        touch: bool,
    },
    /// Creates a new project from specified skeleton
    New {
        /// Id of skeleton to copy
        #[arg(required = true)]
        id: String,
        /// Desired project path
        #[arg(required = true)]
        path: PathBuf,
        /// Optional name, defaults to directory name
        name: Option<String>,
    },
    /// Removes skeleton
    Remove {
        /// Id of skeleton to remove
        id: String,
    },
}
