use base64::alphabet::{STANDARD, URL_SAFE};
use base64::Engine;
use base64::engine::general_purpose::{NO_PAD, PAD};
use base64::engine::GeneralPurpose;
use crate::cli::{Base64Options, EncodeMethod, HexOptions, URLOptions};
use crate::handlers::{Result};

pub struct EncodeHandler {}

impl EncodeHandler {
    pub fn handle_method(method: &EncodeMethod) -> Result {
        match method {
            EncodeMethod::Base64 {options} => Self::encode_base64(options),
            EncodeMethod::URL {options} => Self::encode_url(options),
            EncodeMethod::Hex {options} => Self::encode_hex(options),
        }
    }

    fn encode_base64(options: &Base64Options) -> Result {
        let alphabet = if options.url_safe {URL_SAFE} else {STANDARD};
        let padding = if options.no_padding {NO_PAD} else {PAD};
        Ok(
            GeneralPurpose::new(&alphabet, padding).encode(&options.text)
        )
    }

    fn encode_url(options: &URLOptions) -> Result {
        Ok(urlencoding::encode(&options.text).to_string())
    }

    fn encode_hex(options: &HexOptions) -> Result {
        Ok(hex::encode(&options.text))
    }
}