mod json;
mod encode_decode;
mod uuid;
mod time;
mod text;
mod hash;

use clap::{Parser, Subcommand};
pub use crate::cli::json::{JSONMethod, FormatOptions, MinifyOptions, ValidateOptions};
pub use crate::cli::encode_decode::{EncodeMethod, DecodeMethod, Base64Options, URLOptions, HexOptions};
pub use crate::cli::hash::{HashMethod, HashTextOptions, HashFileOptions, HashVerifyOptions, HashAlgorithm};
pub use crate::cli::uuid::{UuidMethod, UuidOptions};
pub use crate::cli::time::{TimeMethod, TimeNowOptions, TimeFromUnixOptions, TimeToUnixOptions, TimeAgoOptions, TimeFormatOptions};
pub use crate::cli::text::{TextMethod, TextOptions};

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Format, validate, and manipulate JSON data")]
    Json {
        #[command(subcommand)]
        method: JSONMethod
    },
    #[command(about = "Encode data to various formats")]
    Encode {
        #[command(subcommand)]
        method: EncodeMethod
    },
    #[command(about = "Decode data from various formats")]
    Decode {
        #[command(subcommand)]
        method: DecodeMethod
    },
    #[command(about = "Generate and validate cryptographic hashes")]
    Hash {
        #[command(subcommand)]
        method: HashMethod
    },
    #[command(about = "Generate UUIDs")]
    Uuid {
        #[command(subcommand)]
        method: UuidMethod
    },
    #[command(about = "Convert and format dates and timestamps")]
    Time {
        #[command(subcommand)]
        method: TimeMethod
    },
    #[command(about = "Text manipulation utilities")]
    Text {
        #[command(subcommand)]
        method: TextMethod
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, short, global = true, required = false, help = "Path to file where to save the results")]
    pub output: Option<String>
}