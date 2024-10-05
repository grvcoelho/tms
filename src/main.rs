mod cli;
mod commands;
mod error;
mod project;
mod tmux;

use crate::cli::Cli;
use clap::Parser;
use commands::Program;
use error::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let program = Program::new();
    program.ensure_required_dependencies(&["fzf", "git", "tmux"])?;
    program.execute(cli.term)?;

    Ok(())
}
