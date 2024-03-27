use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug, PartialEq)]
#[command(name = "sk")]
#[command(bin_name = "sk")]
#[command(about = "A command line tool for using and managing skeleton projects", long_about = None)]
#[command(author = "Lucas Newcomb (pants721)")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum Commands {
    /// Lists all configured skeletons
    List,
    /// Adds skeleton to configured skeletons
    Add {
        /// Source to create skeleton from
        #[arg(required = true)]
        source: PathBuf,
        /// Identifier of skeleton
        #[arg(short, long)]
        id: Option<String>,
    },
    /// Creates a new project using skeleton
    New {
        /// Id of skeleton to use
        #[arg(required = true)]
        id: String,
        /// Desired project path
        path: Option<PathBuf>,
        /// Name to replace placeholder with. Defaults to directory name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Removes skeleton
    Remove {
        /// Id of skeleton to remove
        id: String,
        /// Removes without confirming
        #[arg(short, long)]
        no_confirm: bool,
    },
    /// Generates shell completion for provided shell
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}
