#[derive(Debug)]
pub enum ParseError {
    TokenizerError(String),
    Unsupported(String),
}
