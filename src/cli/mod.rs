mod json;
mod encode_decode;
mod uuid;
mod time;

use clap::{Parser, Subcommand};
pub use crate::cli::json::{JSONMethod, FormatOptions, MinifyOptions, ValidateOptions};
pub use crate::cli::encode_decode::{EncodeMethod, DecodeMethod, Base64Options, URLOptions, HexOptions};
pub use crate::cli::uuid::{UuidMethod, UuidOptions};
pub use crate::cli::time::{TimeMethod, TimeNowOptions, TimeFromUnixOptions, TimeToUnixOptions, TimeAgoOptions, TimeFormatOptions};

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
    },
    Uuid {
        #[command(subcommand)]
        method: UuidMethod
    },
    Time {
        #[command(subcommand)]
        method: TimeMethod
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}