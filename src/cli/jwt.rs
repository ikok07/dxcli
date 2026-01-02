use clap::{Args, Subcommand, ValueEnum};
use jsonwebtoken::Algorithm;

#[derive(Debug, Subcommand)]
pub enum JWTMethod {
    #[command(about = "Decode JWT token")]
    Decode {
        #[command(flatten)]
        options: JWTDecodeOptions
    },
    #[command(about = "Verify JWT token")]
    Verify {
        #[command(flatten)]
        options: JWTVerifyOptions
    },
}

#[derive(Debug, Args)]
pub struct JWTDecodeOptions {
    #[arg(required = true, help = "JWT token string")]
    pub token: String
}

#[derive(Debug, Args)]
pub struct JWTVerifyOptions {
    #[arg(required = true, help = "JWT token string")]
    pub token: String,

    #[arg(required = true, help = "JWT token secret")]
    pub secret: String,

    #[arg(long, short, required = false, default_value = "HS256")]
    pub algorithm: JWTAlgorithm,

    #[arg(long, short, required = false, help = "Comma separated list of required claims (exp,sub,...)", value_delimiter = ',')]
    pub required: Option<Vec<String>>
}

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "UPPERCASE")]
pub enum JWTAlgorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    PS256,
    PS384,
    PS512,
    EdDSA,
}

impl From<&JWTAlgorithm> for Algorithm {
    fn from(value: &JWTAlgorithm) -> Self {
        match value {
            JWTAlgorithm::HS256 => Algorithm::HS256,
            JWTAlgorithm::HS384 => Algorithm::HS384,
            JWTAlgorithm::HS512 => Algorithm::HS512,
            JWTAlgorithm::RS256 => Algorithm::RS256,
            JWTAlgorithm::RS384 => Algorithm::RS384,
            JWTAlgorithm::RS512 => Algorithm::RS512,
            JWTAlgorithm::ES256 => Algorithm::ES256,
            JWTAlgorithm::ES384 => Algorithm::ES384,
            JWTAlgorithm::PS256 => Algorithm::PS256,
            JWTAlgorithm::PS384 => Algorithm::PS384,
            JWTAlgorithm::PS512 => Algorithm::PS512,
            JWTAlgorithm::EdDSA => Algorithm::EdDSA,
        }
    }
}