use crate::error::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    LParen,
    RParen,
    Comma,
    EOF,
}

pub struct Tokenizer {
    chars: Vec<char>,
    pos: usize,
    peeked: Option<Token>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            peeked: None,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        if let Some(tok) = self.peeked.take() {
            return Ok(tok);
        }

        self.skip_whitespace();

        if self.pos >= self.chars.len() {
            return Ok(Token::EOF);
        }

        let ch = self.chars[self.pos];

        match ch {
            '(' => {
                self.pos += 1;
                Ok(Token::LParen)
            }
            ')' => {
                self.pos += 1;
                Ok(Token::RParen)
            }
            ',' => {
                self.pos += 1;
                Ok(Token::Comma)
            }
            c if c.is_alphabetic() => {
                let ident = self.read_while(|ch| ch.is_alphanumeric() || ch == '_');
                let upper = ident.to_uppercase();
                if matches!(upper.as_str(), "CREATE" | "TABLE" | "INT" | "VARCHAR") {
                    Ok(Token::Keyword(upper))
                } else {
                    Ok(Token::Identifier(ident))
                }
            }
            _ => Err(ParseError::UnexpectedChar(ch)),
        }
    }

    pub fn peek_token(&mut self) -> Result<Token, ParseError> {
        if self.peeked.is_none() {
            self.peeked = Some(self.next_token()?);
        }
        Ok(self.peeked.clone().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn read_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.pos < self.chars.len() && test(self.chars[self.pos]) {
            result.push(self.chars[self.pos]);
            self.pos += 1;
        }
        result
    }
}
