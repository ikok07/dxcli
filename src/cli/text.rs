use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum TextMethod {
    #[command(about = "Convert to uppercase")]
    Upper {
        #[command(flatten)]
        options: TextOptions
    },
    #[command(about = "Convert to lowercase")]
    Lower {
        #[command(flatten)]
        options: TextOptions
    },
    #[command(about = "Capitalize text")]
    Title {
        #[command(flatten)]
        options: TextOptions
    },
    #[command(about = "Convert to camelCase")]
    Camel {
        #[command(flatten)]
        options: TextOptions
    },
    #[command(about = "Convert to PascalCase")]
    Pascal {
        #[command(flatten)]
        options: TextOptions
    },
    #[command(about = "Convert to snake_case")]
    Snake {
        #[command(flatten)]
        options: TextOptions
    },
    #[command(about = "Convert to kebab-case")]
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