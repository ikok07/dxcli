mod json;
mod encode_decode;

use clap::{Parser, Subcommand};
pub use crate::cli::json::{JSONMethod, FormatOptions, MinifyOptions, ValidateOptions};
pub use crate::cli::encode_decode::{EncodeMethod, DecodeMethod, Base64Options, URLOptions, HexOptions};

#[derive(Debug, Subcommand)]
pub enum Command {
    Json {
        #[command(subcommand)]
        method: JSONMethod
    },
    Encode {
        #[command(subcommand)]
        method: EncodeMethod
    },
    Decode {
        #[command(subcommand)]
        method: DecodeMethod
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}