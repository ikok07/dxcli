use std::fs;
use crate::cli::{FormatOptions, JSONMethod, MinifyOptions, ValidateOptions};
use crate::handlers::CommandHandlerError;

pub struct JSONHandler {
    method: JSONMethod
}

impl JSONHandler {
    pub fn handle_method(method: &JSONMethod) -> Result<String, CommandHandlerError> {
        match method {
            JSONMethod::Format {options} => Self::format(options),
            JSONMethod::Minify {options} => Self::minify(options),
            JSONMethod::Validate {options} => Self::validate(options)
        }
    }

    fn format(options: &FormatOptions) -> Result<String, CommandHandlerError> {
        let json_str = {
            if options.file.is_some() {
                fs::read_to_string(options.file.as_ref().unwrap())
                    .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to read JSON file! {}", err))))?
            } else if options.content.is_some() {
                options.content.as_ref().unwrap().clone()
            } else {
                return Err(CommandHandlerError::MissingArgumentsSome(vec!["file".to_string(), "content".to_string()]))
            }
        };

        let json: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to parse JSON! {}", err))))?;

        let pretty_json = serde_json::to_string_pretty(&json)
            .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to prettify JSON! {}", err))))?;

        return Ok(pretty_json);
    }

    fn minify(options: &MinifyOptions) -> Result<String, CommandHandlerError> {
        Ok(String::from("Result from minify"))
    }

    fn validate(options: &ValidateOptions) -> Result<String, CommandHandlerError> {
        Ok(String::from("Result from validate"))
    }
}