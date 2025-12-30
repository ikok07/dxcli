mod json;

use std::fmt::{write, Display, Formatter};
use crate::cli::Command;
use crate::handlers::json::JSONHandler;

#[derive(Debug)]
pub enum CommandHandlerError {
    RuntimeError(Option<String>),
    MissingArguments(Vec<String>),
    MissingArgumentsSome(Vec<String>),
    UnknownError
}

impl Display for CommandHandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandHandlerError::RuntimeError(message) => write!(f, "{}", message.as_ref().unwrap_or(&String::from("A runtime error occurred!"))),
            CommandHandlerError::MissingArguments(args) => write!(f, "Required arguments: {:?}", args),
            CommandHandlerError::MissingArgumentsSome(args) => write!(f, "At least one of the following arguments must be provided: {:?}", args),
            CommandHandlerError::UnknownError => write!(f, "Unknown error")
        }
    }
}

pub struct CommandHandler {
    command: Command
}

impl CommandHandler {
    pub fn new(command: Command) -> Self {
        CommandHandler {command}
    }

    pub fn handle(&self) -> Result<String, CommandHandlerError> {
        return match &self.command {
            Command::Json {method} => JSONHandler::handle_method(method)
        }
    }
}