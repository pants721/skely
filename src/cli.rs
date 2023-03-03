use std::path::PathBuf;

use clap::{Parser, Subcommand};

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

#[derive(Parser)]
#[command(name = "sk")]
#[command(bin_name = "sk")]
#[command(about = "A command line tool for using and managing skeleton projects", long_about = None)]
#[command(author = "Lucas Newcomb (pants721)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
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
    },
    /// Creates a new project from specified skeleton
    New {
        /// Id of skeleton to copy
        #[arg(required = true)]
        id: String,
        /// Desired project path
        #[arg(required = true)]
        path: PathBuf,
    },
    /// Removes skeleton
    Remove {
        /// Id of skeleton to remove
        id: String,
    },
}
