use uuid::{Uuid};
use crate::cli::{JSONMethod, UuidMethod, UuidOptions};
use crate::handlers::CommandHandlerError;

pub struct UuidHandler {}

impl UuidHandler {
    pub fn handle_method(method: &UuidMethod) -> Result<String, CommandHandlerError> {
        match method {
            UuidMethod::V4 {options} => Self::gen_uuid4(options),
            UuidMethod::V7 {options} => Self::gen_uuid7(options),
        }
    }

    fn gen_uuid4(options: &UuidOptions) -> Result<String, CommandHandlerError> {
        let count = if options.number.is_some() {options.number.unwrap()} else {1};
        let mut result = String::new();
        for i in 1..=count {
            result.push_str(&format!("{}. {}\n", i, Uuid::new_v4().to_string()))
        }
        Ok(result)
    }

    fn gen_uuid7(options: &UuidOptions) -> Result<String, CommandHandlerError> {
        let count = if options.number.is_some() {options.number.unwrap()} else {1};
        let mut result = String::new();
        for i in 1..=count {
            result.push_str(&format!("{}. {}\n", i, Uuid::now_v7().to_string()))
        }
        Ok(result)
    }
}