use std::result;
use md5::digest::typenum::op;
use regex::Regex;
use crate::cli::{RegexMatchOptions, RegexMethod, RegexReplaceOptions, RegexTestOptions};
use crate::handlers::{CommandHandlerError, Result};

pub struct RegexHandler {}

impl RegexHandler {
    pub fn handle_method(method: &RegexMethod) -> Result {
        match method {
            RegexMethod::Test {options} => Self::test_expr(options),
            RegexMethod::Match {options} => Self::match_expr(options),
            RegexMethod::Replace {options} => Self::replace_text(options),
        }
    }

    fn parse_regex(pattern: &str) -> result::Result<Regex, CommandHandlerError> {
        match Regex::new(pattern) {
            Ok(re) => Ok(re),
            Err(_) => Err(CommandHandlerError::RuntimeError(Some(String::from("Invalid RegEx expression!"))))
        }
    }

    fn test_expr(options: &RegexTestOptions) -> Result {
        let re = Self::parse_regex(&options.pattern)?;

        match re.find(&options.text) {
            Some(_) => Ok(String::from("Match found!")),
            None => Err(CommandHandlerError::NegativeResult(String::from("No match found!")))
        }
    }

    fn match_expr(options: &RegexMatchOptions) -> Result {
        let re = Self::parse_regex(&options.pattern)?;
        let mut result = String::new();
        for (capture_i, captures) in re.captures_iter(&options.text).enumerate() {
            if let Some(full_match) = captures.get(0) {
                result.push_str("------------\n");
                result.push_str(&format!("Match {}: \"{}\"\n", capture_i, full_match.as_str()));
                for group_i in 1..captures.len() {
                    if let Some(group) = captures.get(group_i) {
                        result.push_str(&format!("\tGroup {}: \"{}\"\n", group_i, group.as_str()))
                    }
                }
                result.push_str("------------\n");
            }
        }

        if result.is_empty() {
            return Err(CommandHandlerError::NegativeResult(String::from("No matches found!")));
        }

        return Ok(result);
    }

    fn replace_text(options: &RegexReplaceOptions) -> Result {
        let re = Self::parse_regex(&options.pattern)?;
        return Ok(re.replace_all(&options.text, &options.replacement).to_string());
    }
}