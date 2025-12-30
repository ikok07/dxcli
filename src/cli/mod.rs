mod json;

use clap::{Parser, Subcommand};
use crate::args_parser::json::JSONMethod;

#[derive(Debug, Subcommand)]
pub enum Command {
    Json {
        #[command(subcommand)]
        method: JSONMethod
    }
}

#[derive(Debug, Parser)]
pub struct ArgsParser {
    #[command(subcommand)]
    pub command: Command
}