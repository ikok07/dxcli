use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum LoremMethod {
    #[command(about = "Generate random words")]
    Words {
        #[command(flatten)]
        options: LoremOptions
    },
    #[command(about = "Generate random sentences")]
    Sentences {
        #[command(flatten)]
        options: LoremOptions
    },
    #[command(about = "Generate random paragraphs")]
    Paragraphs {
        #[command(flatten)]
        options: LoremOptions
    }
}

#[derive(Debug, Args)]
pub struct LoremOptions {
    #[arg(required = true)]
    pub count: u32
}