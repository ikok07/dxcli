use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct MinifyOptions {
    #[arg(short, long, required=true, value_name = "path to file")]
    file: String
}

#[derive(Debug, Args)]
pub struct FormatOptions {
    #[arg(short, long, required=true, value_name = "path to file")]
    file: String
}

#[derive(Debug, Args)]
pub struct ValidateOptions {
    #[arg(short, long, required=true, value_name = "path to file")]
    file: String
}

#[derive(Debug, Subcommand)]
pub enum JSONMethod {
    Format {
        #[command(flatten)]
        options: FormatOptions
    },
    Minify {
        #[command(flatten)]
        options: MinifyOptions
    },
    Validate {
        #[command(flatten)]
        options: ValidateOptions
    }
}