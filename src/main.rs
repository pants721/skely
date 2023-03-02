use clap::Parser;

use crate::app::App;
use crate::cli::Cli;
use crate::common::startup;

mod app;
mod cli;
mod common;
mod skeleton;

fn main() {
    let mut app: App = App::new();
    startup(&mut app);
    let args = Cli::parse();
    app.handle_command(args.command).unwrap_or_else(|err| {
        eprintln!("ERROR: Error handling command (App::handle_command() {err})")
    });
}
