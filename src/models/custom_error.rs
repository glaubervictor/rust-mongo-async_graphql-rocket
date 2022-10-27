use std::string::ParseError;

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

impl From<ParseError> for CustomError {
    fn from(source: ParseError) -> Self {
        Self {
            message: source.to_string(),
        }
    }
}

impl From<&str> for CustomError {
    fn from(source: &str) -> Self {
        Self {
            message: String::from(source),
        }
    }
}