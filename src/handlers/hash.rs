use std::fs;
use md5::{Md5, Digest as Md5Digest};
use sha2::{Sha256, Sha512, Digest as Sha256Digest};
use crate::cli::{HashAlgorithm, HashFileOptions, HashMethod, HashVerifyOptions};
use crate::handlers::{Result, CommandHandlerError};

pub struct HashHandler {}

impl HashHandler {
    pub fn handle_method(method: &HashMethod) -> Result {
        match method {
            HashMethod::Md5 {options} => Ok(Self::hash_md5(&options.text)),
            HashMethod::Sha256 {options} => Ok(Self::hash_sha256(&options.text)),
            HashMethod::Sha512 {options} => Ok(Self::hash_sha512(&options.text)),
            HashMethod::File {options} => Self::hash_file(options),
            HashMethod::Verify {options} => Self::verify_hash(options)
        }
    }

    fn hash_md5(text: &str) -> String { hex::encode(Md5::digest(text.as_bytes())) }
    fn hash_sha256(text: &str) -> String { hex::encode(Sha256::digest(text.as_bytes())) }
    fn hash_sha512(text: &str) -> String { hex::encode(Sha512::digest(text.as_bytes())) }

    fn hash_file(options: &HashFileOptions) -> Result {
        let file_content = fs::read_to_string(&options.path)
            .map_err(|err| CommandHandlerError::RuntimeError(Some(format!("Failed to read file contents! {err}"))))?;

        return Ok(
            match options.algorithm {
                HashAlgorithm::Md5 => Self::hash_md5(&file_content),
                HashAlgorithm::Sha256 => Self::hash_sha256(&file_content),
                HashAlgorithm::Sha512 => Self::hash_sha512(&file_content),
            }
        );
    }

    fn verify_hash(options: &HashVerifyOptions) -> Result {
        let new_hash = {
            match options.algorithm {
                HashAlgorithm::Md5 => Self::hash_md5(&options.text),
                HashAlgorithm::Sha256 => Self::hash_sha256(&options.text),
                HashAlgorithm::Sha512 => Self::hash_sha512(&options.text),
            }
        };

        if new_hash == options.expected {
            return Ok(String::from("The provided text matches the expected hash!"))
        }

        return Err(CommandHandlerError::NegativeResult("The provided string doesn't match the expected one!".to_string()));
    }
}