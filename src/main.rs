use anyhow::Result;
use clap::Parser;

use crate::app::App;
use crate::cli::Cli;

mod app;
mod cli;
mod common;
mod skeleton;

fn main() -> Result<()> {
    let mut app: App = App::new();
    app.run()?;
    let args = Cli::parse();
    app.handle_command(args.command)?;
    Ok(())
}
