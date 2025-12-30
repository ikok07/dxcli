use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct FormatOptions {
    #[arg(long, short, value_name = "path to file", conflicts_with = "content")]
    pub file: Option<String>,

    #[arg(long, value_name = "json content", conflicts_with = "file")]
    pub content: Option<String>
}

#[derive(Debug, Args)]
pub struct MinifyOptions {
    #[arg(required=true, value_name = "path to file")]
    pub file: String
}

#[derive(Debug, Args)]
pub struct ValidateOptions {
    #[arg(required=true, value_name = "path to file")]
    pub file: String
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