use crate::tokenizer::{Tokenizer, Token};
use crate::ast::{SelectStatement, Expression};
use crate::error::ParseError;

pub fn parse_create_table(_tokenizer: &mut Tokenizer) -> Result<(), ParseError> {
    Err(ParseError::Unsupported("CREATE TABLE not implemented".into()))
}

pub fn parse_select(tokenizer: &mut Tokenizer) -> Result<SelectStatement, ParseError> {
    tokenizer.expect_keyword("SELECT").map_err(ParseError::TokenizerError)?;

    // Parse column list
    let mut columns = Vec::new();
    loop {
        let col = tokenizer.expect_identifier().map_err(ParseError::TokenizerError)?;
        columns.push(col);

        if let Some(Token::Comma) = tokenizer.peek_token() {
            tokenizer.next_token(); // consume comma
        } else {
            break;
        }
    }

    tokenizer.expect_keyword("FROM").map_err(ParseError::TokenizerError)?;
    let table = tokenizer.expect_identifier().map_err(ParseError::TokenizerError)?;

    let condition = if let Some(Token::Keyword(k)) = tokenizer.peek_token() {
        if k == "WHERE" {
            tokenizer.next_token(); // consume WHERE
            let column = tokenizer.expect_identifier().map_err(ParseError::TokenizerError)?;
            tokenizer.expect_token(Token::Equal).map_err(ParseError::TokenizerError)?;
            match tokenizer.next_token() {
                Some(Token::Number(n)) => Some(Expression::Equals(column, *n)),
                other => return Err(ParseError::TokenizerError(format!("Expected number, got {:?}", other))),
            }
        } else {
            None
        }
    } else {
        None
    };

    Ok(SelectStatement {
        columns,
        table,
        condition,
    })
}
