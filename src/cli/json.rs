use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum JSONMethod {
    #[command(about = "Format/prettify JSON")]
    Format {
        #[command(flatten)]
        options: FormatOptions
    },
    #[command(about = "Minify JSON")]
    Minify {
        #[command(flatten)]
        options: MinifyOptions
    },
    #[command(about = "Validate JSON syntax")]
    Validate {
        #[command(flatten)]
        options: ValidateOptions
    }
}


#[derive(Debug, Args)]
pub struct FormatOptions {
    #[arg(long, short, value_name = "path to file", conflicts_with = "content")]
    pub file: Option<String>,

    #[arg(long, value_name = "json content", conflicts_with = "file")]
    pub content: Option<String>
}

#[derive(Debug, Args)]
pub struct MinifyOptions {
    #[arg(long, short, value_name = "path to file", conflicts_with = "content")]
    pub file: Option<String>,

    #[arg(long, value_name = "json content", conflicts_with = "file")]
    pub content: Option<String>
}

#[derive(Debug, Args)]
pub struct ValidateOptions {
    #[arg(long, short, value_name = "path to file", conflicts_with = "content")]
    pub file: Option<String>,

    #[arg(long, value_name = "json content", conflicts_with = "file")]
    pub content: Option<String>
}