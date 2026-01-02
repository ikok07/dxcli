use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum EncodeMethod {
    #[command(about = "Encode to Base64")]
    Base64 {
        #[command(flatten)]
        options: Base64Options
    },
    #[command(about = "URL encode")]
    URL {
        #[command(flatten)]
        options: URLOptions
    },
    #[command(about = "Encode to hexadecimal")]
    Hex {
        #[command(flatten)]
        options: HexOptions
    },
}

#[derive(Debug, Subcommand)]
pub enum DecodeMethod {
    #[command(about = "Decode from Base64")]
    Base64 {
        #[command(flatten)]
        options: crate::cli::Base64Options
    },
    #[command(about = "URL decode")]
    URL {
        #[command(flatten)]
        options: crate::cli::URLOptions
    },
    #[command(about = "Decode from hexadecimal")]
    Hex {
        #[command(flatten)]
        options: crate::cli::HexOptions
    },
}

#[derive(Debug, Args)]
pub struct Base64Options {
    #[arg(required = true, help = "The text to be encoded")]
    pub text: String,

    #[arg(long, required = false, help = "If the generated string should be url safe")]
    pub url_safe: bool,

    #[arg(long, required = false, help = "If the generated string should have padding bytes at the end")]
    pub no_padding: bool
}

#[derive(Debug, Args)]
pub struct URLOptions {
    #[arg(required = true, help = "Text to be encoded")]
    pub text: String
}

#[derive(Debug, Args)]
pub struct HexOptions {
    #[arg(required = true, help = "Text to be encoded")]
    pub text: String
}