use std::result;

pub type TestError = String;
pub type TestResult = result::Result<(), TestError>;
