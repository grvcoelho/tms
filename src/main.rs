mod cli;
mod error;
mod program;
mod project;
mod tmux;

use crate::cli::Cli;
use clap::Parser;
use error::Result;
use log::info;
use program::Program;

fn main() {
    // Initialize logger
    env_logger::init();

    if let Err(e) = run() {
        eprintln!("tms error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    info!("Starting tmux session manager");

    let cli = Cli::parse();

    let program = Program::new();

    info!("Checking dependencies");
    match program.ensure_required_dependencies(&["fzf", "git", "tmux"]) {
        Ok(_) => info!("All dependencies are installed"),
        Err(e) => {
            return Err(e);
        }
    }

    info!("Executing main program logic");
    program.execute(cli.term)?;

    info!("Program completed successfully");
    Ok(())
}
