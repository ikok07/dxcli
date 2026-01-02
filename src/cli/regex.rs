use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum RegexMethod {
    #[command(about = "Text regex pattern")]
    Test {
        #[command(flatten)]
        options: RegexTestOptions
    },
    #[command(about = "Match regex pattern")]
    Match {
        #[command(flatten)]
        options: RegexMatchOptions
    },
    #[command(about = "Replace using regex")]
    Replace {
        #[command(flatten)]
        options: RegexReplaceOptions
    }
}

#[derive(Debug, Args)]
pub struct RegexTestOptions {
    #[arg(required = true, help = "RegEx pattern")]
    pub pattern: String,

    #[arg(required = true)]
    pub text: String
}

#[derive(Debug, Args)]
pub struct RegexMatchOptions {
    #[arg(required = true, help = "RegEx pattern")]
    pub pattern: String,

    #[arg(required = true)]
    pub text: String
}

#[derive(Debug, Args)]
pub struct RegexReplaceOptions {
    #[arg(required = true, help = "RegEx pattern")]
    pub pattern: String,

    #[arg(required = true, help = "Text which to place in the place of the matches")]
    pub replacement: String,

    #[arg(required = true)]
    pub text: String,
}