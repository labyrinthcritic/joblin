use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    /// Directory containing job files (default is ~/.config/joblin/jobs)
    #[arg(long)]
    pub jobs_dir: Option<PathBuf>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    Check,
}
