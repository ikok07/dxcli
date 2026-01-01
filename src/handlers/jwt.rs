use std::collections::HashSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde_json::Value;
use crate::cli::{JWTDecodeOptions, JWTMethod, JWTVerifyOptions};
use crate::handlers::{Result, CommandHandlerError};

pub struct JWTHandler {}

impl JWTHandler {
    pub fn handle_method(method: &JWTMethod) -> Result {
        match method {
            JWTMethod::Decode {options} => Self::decode_jwt(options),
            JWTMethod::Verify {options} => Self::verify_jwt(options)
        }
    }

    fn decode_jwt(options: &JWTDecodeOptions) -> Result {
        match jsonwebtoken::dangerous::insecure_decode::<Value>(&options.token) {
            Ok(data) => {
                let json_header = serde_json::to_string_pretty(&data.header)
                    .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to convert jwt header to json! {err}"))))?;

                let json_claims = serde_json::to_string_pretty(&data.claims)
                    .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to convert jwt claims to json! {err}"))))?;

                return Ok(format!("{json_header}\n{json_claims}"));
            },
            Err(err) => Err(CommandHandlerError::RuntimeError(Some(format!("Failed to decode JWT! {err}"))))
        }
    }

    fn verify_jwt(options: &JWTVerifyOptions) -> Result {
        let key = DecodingKey::from_secret(options.secret.as_bytes());
        let mut validation = Validation::new(Algorithm::from(&options.algorithm));
        validation.required_spec_claims = HashSet::new();

        if options.required.is_some() {
            validation.required_spec_claims = HashSet::from_iter(options.required.as_ref().unwrap().iter().cloned())
        }

        match jsonwebtoken::decode::<Value>(&options.token, &key, &validation) {
            Ok(_) => Ok(String::from("The provided JWT Token is valid!")),
            Err(err) => Err(CommandHandlerError::RuntimeError(Some(format!("The JWT Token is invalid! {err}"))))
        }
    }
}