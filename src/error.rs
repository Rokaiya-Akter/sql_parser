#[derive(Debug)]
pub enum ParseError {
    ExpectedToken(String),
    ExpectedIdentifier,
    ExpectedType,
    UnexpectedToken(String),
    UnexpectedChar(char),
    TokenizerError(String),
}

impl From<String> for ParseError {
    fn from(msg: String) -> Self {
        ParseError::TokenizerError(msg)
    }
}
