//! Implementing the args of the command line.
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long)]
    pub(crate) lang: String,
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// does testing things
    Gen {
        /// lists test values
        #[arg(short, long)]
        bolt: String,
        to: PathBuf,
    },
}
