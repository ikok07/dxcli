use std::fs;
use std::error::Error;
use std::fmt::{Display, Formatter};
use miette::{NamedSource, SourceSpan, Diagnostic};
use crate::cli::{FormatOptions, JSONMethod, MinifyOptions, ValidateOptions};
use crate::handlers::CommandHandlerError;

#[derive(Debug, Diagnostic)]
struct JSONParseError {
    #[source_code]
    src: NamedSource<String>,

    #[label("error occurred here")]
    err_span: SourceSpan,

    message: String
}

impl Display for JSONParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for JSONParseError {}

pub struct JSONHandler {}

impl JSONHandler {
    pub fn handle_method(method: &JSONMethod) -> Result<String, CommandHandlerError> {
        match method {
            JSONMethod::Format {options} => Self::format(options),
            JSONMethod::Minify {options} => Self::minify(options),
            JSONMethod::Validate {options} => Self::validate(options)
        }
    }

    fn extract_json(file: Option<&String>, content: Option<&String>) -> Result<serde_json::Value, CommandHandlerError> {
        let mut src_name = String::new();
        let json_str = {
            if file.is_some() {
                src_name.push_str(file.unwrap());
                fs::read_to_string(file.unwrap())
                    .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to read JSON file! {}", err))))?
            } else if content.is_some() {
                src_name.push_str("Provided JSON content");
                content.unwrap().to_string()
            } else {
                return Err(CommandHandlerError::MissingArgumentsSome(vec!["file".to_string(), "content".to_string()]))
            }
        };

        return Ok(
            serde_json::from_str(&json_str)
                .map_err(|err| {
                    let line = err.line();
                    let col = err.column();

                    let offset = json_str.lines().take(line).map(|l| l.len() + 1).sum::<usize>() + col - 1;

                    let parse_error = JSONParseError {
                        src: NamedSource::new(src_name, json_str),
                        err_span: SourceSpan::new(offset.into(), 1),
                        message: format!("Failed to parse JSON: {err}")
                    };

                    CommandHandlerError::RuntimeError(Some(
                        format!("{:?}", miette::Report::new(parse_error))
                    ))
                })?
        );
    }

    fn format(options: &FormatOptions) -> Result<String, CommandHandlerError> {
        let json = Self::extract_json(options.file.as_ref(), options.content.as_ref())?;

        let pretty_json = serde_json::to_string_pretty(&json)
            .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to prettify JSON! {}", err))))?;

        return Ok(pretty_json);
    }

    fn minify(options: &MinifyOptions) -> Result<String, CommandHandlerError> {
        let json = Self::extract_json(options.file.as_ref(), options.content.as_ref())?;

        let minified_json = serde_json::to_string(&json)
            .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to minify JSON! {}", err))))?;;

        return Ok(minified_json);
    }

    fn validate(options: &ValidateOptions) -> Result<String, CommandHandlerError> {
        Self::extract_json(options.file.as_ref(), options.content.as_ref())?;
        return Ok("The provided JSON is valid!".to_string());
    }
}