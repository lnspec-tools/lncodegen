//! Implementing the args of the command line.
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long)]
    pub(crate) lang: Option<String>,
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// does testing things
    Generate {
        /// lists test values
        #[arg(short, long)]
        bolt: String,
        to: PathBuf,
    },
    /// Decode a unsgned lightning message in hex fromat
    Decode { from: String },
}
