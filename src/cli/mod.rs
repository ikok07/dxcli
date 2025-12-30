mod json;

use clap::{Parser, Subcommand};
pub use crate::cli::json::{JSONMethod, FormatOptions, MinifyOptions, ValidateOptions};

#[derive(Debug, Subcommand)]
pub enum Command {
    Json {
        #[command(subcommand)]
        method: JSONMethod
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}