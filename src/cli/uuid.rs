use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum UuidMethod {
    V4 {
        #[command(flatten)]
        options: UuidOptions
    },
    V7 {
        #[command(flatten)]
        options: UuidOptions
    }
}

#[derive(Debug, Args)]
pub struct UuidOptions {
    #[arg(long, short, required = false, help = "Number of generated UUIDs")]
    pub number: Option<u32>
}