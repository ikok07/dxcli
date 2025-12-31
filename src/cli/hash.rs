use clap::{Args, Subcommand, ValueEnum};

#[derive(Debug, Subcommand)]
pub enum HashMethod {
    Md5 {
        #[command(flatten)]
        options: HashTextOptions
    },
    Sha256 {
        #[command(flatten)]
        options: HashTextOptions
    },
    Sha512 {
        #[command(flatten)]
        options: HashTextOptions
    },
    File {
        #[command(flatten)]
        options: HashFileOptions
    },
    Verify {
        #[command(flatten)]
        options: HashVerifyOptions
    }
}

#[derive(Debug, Args)]
pub struct HashTextOptions {
    #[arg(required = true, help = "Text to hash")]
    pub text: String
}

#[derive(Debug, Args)]
pub struct HashFileOptions {
    #[arg(required = true, help = "Path to file")]
    pub path: String,

    #[arg(long, short, required = false, help = "Hash algorithm", default_value = "sha256")]
    pub algorithm: HashAlgorithm
}

#[derive(Debug, Args)]
pub struct HashVerifyOptions {
    #[arg(required = true, help = "Text to hash")]
    pub text: String,

    #[arg(required = true, help = "Expected hash")]
    pub expected: String,

    #[arg(long, short, required = false, help = "Hash algorithm", default_value = "sha256")]
    pub algorithm: HashAlgorithm
}

#[derive(Debug, Clone, ValueEnum)]
pub enum HashAlgorithm {
    Md5,
    Sha256,
    Sha512
}