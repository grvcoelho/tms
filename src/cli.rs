use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional path or search term
    pub path: Option<String>,
}
