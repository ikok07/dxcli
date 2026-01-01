use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum RegexMethod {
    Test {
        #[command(flatten)]
        options: RegexTestOptions
    },
    Match {
        #[command(flatten)]
        options: RegexMatchOptions
    },
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