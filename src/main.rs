use anyhow::Result;
use clap::Parser;

use crate::app::App;
use crate::cli::Cli;

mod app;
mod cli;
mod common;
mod settings;
mod skeleton;
mod file_util;

fn main() -> Result<()> {
    let app: App = App::default()?;
    let args = Cli::parse();
    app.handle_command(args.command)?;
    Ok(())
}
