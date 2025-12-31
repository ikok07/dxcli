use base64::alphabet::{STANDARD, URL_SAFE};
use base64::Engine;
use base64::engine::general_purpose::{NO_PAD, PAD};
use base64::engine::GeneralPurpose;
use crate::cli::{Base64Options, DecodeMethod, HexOptions, URLOptions};
use crate::handlers::CommandHandlerError;
use crate::handlers::CommandHandlerError::RuntimeError;

pub struct DecodeHandler {}

impl DecodeHandler {
    pub fn handle_method(method: &DecodeMethod) -> Result<String, CommandHandlerError> {
        match method {
            DecodeMethod::Base64 {options} => Self::decode_base64(options),
            DecodeMethod::URL {options} => Self::decode_url(options),
            DecodeMethod::Hex {options} => Self::decode_hex(options),
        }
    }

    fn decode_base64(options: &Base64Options) -> Result<String, CommandHandlerError> {
        let alphabet = if options.url_safe {URL_SAFE} else {STANDARD};
        let padding = if options.no_padding {NO_PAD} else {PAD};
        let engine = GeneralPurpose::new(&alphabet, padding);
        match engine.decode(&options.text) {
            Ok(result) => Ok(String::from_utf8_lossy(&result).to_string()),
            Err(err) => Err(RuntimeError(Some(format!("Failed to decode BASE64 string! {err}"))))
        }
    }

    fn decode_url(options: &URLOptions) -> Result<String, CommandHandlerError> {
        match urlencoding::decode(&options.text) {
            Ok(result) => Ok(result.to_string()),
            Err(err) => Err(RuntimeError(Some(format!("Failed to decode url string! {err}"))))
        }
    }

    fn decode_hex(options: &HexOptions) -> Result<String, CommandHandlerError> {
        match hex::decode(&options.text) {
            Ok(result) => Ok(String::from_utf8_lossy(&result).to_string()),
            Err(err) => Err(RuntimeError(Some(format!("Failed to decode hex string! {err}"))))
        }
    }
}