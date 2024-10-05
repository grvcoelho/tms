mod cli;
mod commands;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => {
            commands::hello::execute("world");
        }
    }
}
