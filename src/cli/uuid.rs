use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum UuidMethod {
    #[command(about = "Generate UUID v4 (random)")]
    V4 {
        #[command(flatten)]
        options: UuidOptions
    },
    #[command(about = "Generate UUID v7 (time-based)")]
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