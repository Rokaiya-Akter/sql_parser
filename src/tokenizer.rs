#[derive(Debug, Clone, PartialEq)]
pub enum Token {

    Keyword(String),
    Identifier(String),
    Number(i64),
    Symbol(char),
    Comma,
    Equal,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
    tokens: Vec<Token>,
    index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else if c == ',' {
                tokens.push(Token::Comma);
                chars.next();
            } else if c == '=' {
                tokens.push(Token::Equal);
                chars.next();
            } else if c == ';' {
                chars.next();
            } else if c.is_alphabetic() {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let keyword = ident.to_uppercase();
                if ["SELECT", "FROM", "WHERE", "CREATE", "TABLE"].contains(&keyword.as_str()) {
                    tokens.push(Token::Keyword(keyword));
                } else {
                    tokens.push(Token::Identifier(ident));
                }
            } else if c.is_numeric() {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            } else {
                tokens.push(Token::Symbol(c));
                chars.next();
            }
        }

        Self {
            input,
            pos: 0,
            tokens,
            index: 0,
        }
    }

    pub fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.index);
        self.index += 1;
        tok
    }

    pub fn expect_keyword(&mut self, expected: &str) -> Result<(), String> {
        match self.next_token() {
            Some(Token::Keyword(k)) if k == expected => Ok(()),
            other => Err(format!("Expected keyword '{}', got {:?}", expected, other)),
        }
    }

    pub fn expect_identifier(&mut self) -> Result<String, String> {
        match self.next_token() {
            Some(Token::Identifier(s)) => Ok(s.clone()),
            other => Err(format!("Expected identifier, got {:?}", other)),
        }
    }

    pub fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        match self.next_token() {
            Some(tok) if *tok == expected => Ok(()),
            other => Err(format!("Expected token {:?}, got {:?}", expected, other)),
        }
    }

    pub fn peek_keyword(&self) -> Option<String> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.index) {
            Some(k.clone())
        } else {
            None
        }
    }
}
