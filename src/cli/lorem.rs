use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum LoremMethod {
    Words {
        #[command(flatten)]
        options: LoremOptions
    },
    Sentences {
        #[command(flatten)]
        options: LoremOptions
    },
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