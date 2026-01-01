use capitalize::Capitalize;
use stringcase::{camel_case, kebab_case, pascal_case, snake_case};
use crate::handlers::{Result};
use crate::cli::{TextMethod};

pub struct TextHandler {}

impl TextHandler {
    pub fn handle_method(method: &TextMethod) -> Result {
        match method {
            TextMethod::Upper {options} => Ok(options.text.to_uppercase()),
            TextMethod::Lower {options} => Ok(options.text.to_lowercase()),
            TextMethod::Title {options} => Ok(options.text.capitalize()),
            TextMethod::Camel {options} => Ok(camel_case(&options.text)),
            TextMethod::Pascal {options} => Ok(pascal_case(&options.text)),
            TextMethod::Snake {options} => Ok(snake_case(&options.text)),
            TextMethod::Kebab {options} => Ok(kebab_case(&options.text))
        }
    }
}