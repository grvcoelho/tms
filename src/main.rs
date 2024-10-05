mod cli;
mod commands;
mod utils;

use crate::cli::Cli;
use clap::Parser;
use std::process::exit;

fn main() {
    let cli = Cli::parse();

    // Check for required commands
    if let Err(e) = utils::check_required_commands() {
        eprintln!("Error: {}", e);
        exit(1);
    }

    if let Err(e) = commands::execute(cli.term) {
        eprintln!("Error: {}", e);
        exit(1);
    }
}
