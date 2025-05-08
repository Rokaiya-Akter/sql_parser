use crate::tokenizer::{Token, Tokenizer};
use crate::ast::{Statement, ColumnDef};
use crate::error::ParseError;

pub fn parse_create_table(tokens: &mut Tokenizer) -> Result<Statement, ParseError> {
    // CREATE
    match tokens.next_token()? {
        Token::Keyword(ref k) if k.eq_ignore_ascii_case("CREATE") => {}
        _ => return Err(ParseError::ExpectedToken("CREATE".to_string())),
    }

    // TABLE
    match tokens.next_token()? {
        Token::Keyword(ref k) if k.eq_ignore_ascii_case("TABLE") => {}
        _ => return Err(ParseError::ExpectedToken("TABLE".to_string())),
    }

    // table name
    let table_name = match tokens.next_token()? {
        Token::Identifier(name) => name,
        _ => return Err(ParseError::ExpectedIdentifier),
    };

    // (
    if tokens.next_token()? != Token::LParen {
        return Err(ParseError::ExpectedToken("(".to_string()));
    }

    let mut columns = Vec::new();

    loop {
        // column name
        let name = match tokens.next_token()? {
            Token::Identifier(name) => name,
            Token::RParen => break,
            _ => return Err(ParseError::ExpectedIdentifier),
        };

        // column type
        let data_type = match tokens.next_token()? {
            Token::Keyword(dt) => dt,
            _ => return Err(ParseError::ExpectedType),
        };

        columns.push(ColumnDef {
            name,
            data_type,
            constraints: vec![],
        });

        match tokens.peek_token()? {
            Token::Comma => {
                tokens.next_token()?; // consume comma
            }
            Token::RParen => {
                tokens.next_token()?; // consume )
                break;
            }
            other => return Err(ParseError::UnexpectedToken(format!("{:?}", other))),
        }
    }

    Ok(Statement::CreateTable { name: table_name, columns })
}
