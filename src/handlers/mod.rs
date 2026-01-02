mod json;
mod encode;
mod decode;
mod uuid;
mod time;
mod text;
mod hash;
mod jwt;
mod regex;
mod lorem;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::result;

use crate::cli::{Command};
use crate::handlers::decode::DecodeHandler;
use crate::handlers::encode::EncodeHandler;
use crate::handlers::hash::HashHandler;
use crate::handlers::json::JSONHandler;
use crate::handlers::jwt::JWTHandler;
use crate::handlers::lorem::LoremHandler;
use crate::handlers::regex::RegexHandler;
use crate::handlers::text::TextHandler;
use crate::handlers::time::TimeHandler;
use crate::handlers::uuid::UuidHandler;
use crate::utils::TestError;

#[derive(Debug)]
pub enum CommandHandlerError {
    RuntimeError(Option<String>),
    NegativeResult(String),
    MissingArguments(Vec<String>),
    MissingArgumentsSome(Vec<String>),
    UnknownError
}

impl Display for CommandHandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandHandlerError::RuntimeError(message) => write!(f, "{}", message.as_ref().unwrap_or(&String::from("A runtime error occurred!"))),
            CommandHandlerError::NegativeResult(message) => write!(f, "{message}"),
            CommandHandlerError::MissingArguments(args) => write!(f, "Required arguments: {:?}", args),
            CommandHandlerError::MissingArgumentsSome(args) => write!(f, "At least one of the following arguments must be provided: {:?}", args),
            CommandHandlerError::UnknownError => write!(f, "Unknown error")
        }
    }
}

impl Error for CommandHandlerError {}

impl Into<TestError> for CommandHandlerError {
    fn into(self) -> TestError {
        return format!("{self}")
    }
}

pub type Result = result::Result<String, CommandHandlerError>;

pub struct CommandHandler {
    command: Command
}

impl CommandHandler {
    pub fn new(command: Command) -> Self {
        CommandHandler {command}
    }

    pub fn handle(&self) -> Result {
        return match &self.command {
            Command::Json {method} => JSONHandler::handle_method(method),
            Command::Encode {method} => EncodeHandler::handle_method(method),
            Command::Decode {method} => DecodeHandler::handle_method(method),
            Command::Hash {method} => HashHandler::handle_method(method),
            Command::Uuid {method} => UuidHandler::handle_method(method),
            Command::Time {method} => TimeHandler::handle_method(method),
            Command::Text {method} => TextHandler::handle_method(method),
            Command::Jwt {method} => JWTHandler::handle_method(method),
            Command::Regex {method} => RegexHandler::handle_method(method),
            Command::Lorem {method} => LoremHandler::handle_method(method)
        }
    }
}