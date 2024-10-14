use clap::Parser;

/// A Tmux Session Manager for Git projects
#[derive(Parser)]
#[command(
    name = "tms",
    author = "Guilherme Coelho",
    version,
    about = "Manages tmux sessions for git projects",
    long_about = "This tool helps manage Tmux sessions for Git projects. It integrates with ghq for repository management and fzf for fuzzy finding.",
    after_help = "EXAMPLES:
    # Open fzf to select a project from all ghq repositories
    tms

    # Open a specific directory as a tmux session
    tms /path/to/your/project

    # Clone (if necessary) and open a GitHub repository
    tms username/repository

    # Fuzzy find for an existing project using a keyword
    tms search_keyword"
)]
pub struct Cli {
    /// Optional search term or path to project
    #[arg(value_name = "TERM")]
    pub term: Option<String>,

    // Open in a new window instead of a new session
    #[arg(long, short = 'w')]
    pub window: bool,
}
