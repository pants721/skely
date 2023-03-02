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
#[command(about = "A command line tool for creating skeleton projects", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    List,
    #[command(arg_required_else_help = true)]
    Edit {
        id: String,
    },
    Add {
        #[arg(required = true)]
        name: String,
        source: Option<PathBuf>,
    },
    New {
        #[arg(required = true)]
        id: String,
        #[arg(required = true)]
        path: PathBuf,
    },
    Remove {
        id: String,
    },
}
