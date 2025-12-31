use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum TextMethod {
    Upper {
        #[command(flatten)]
        options: TextOptions
    },
    Lower {
        #[command(flatten)]
        options: TextOptions
    },
    Title {
        #[command(flatten)]
        options: TextOptions
    },
    Camel {
        #[command(flatten)]
        options: TextOptions
    },
    Pascal {
        #[command(flatten)]
        options: TextOptions
    },
    Snake {
        #[command(flatten)]
        options: TextOptions
    },
    Kebab {
        #[command(flatten)]
        options: TextOptions
    },
}

#[derive(Debug, Args)]
pub struct TextOptions {
    #[arg(required = true, help = "Text to transform")]
    pub text: String
}