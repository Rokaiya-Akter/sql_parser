use crate::ast::Expr;
use crate::tokenizer::{Token, Tokenizer};
use crate::error::ParseError;

pub fn parse_expression(tokens: &mut Tokenizer, min_prec: u8) -> Result<Expr, ParseError> {
    let mut left = match tokens.next_token()? {
        Token::Minus => Expr::Unary {
            op: "-".to_string(),
            expr: Box::new(parse_expression(tokens, 100)?),
        },
        Token::Number(n) => Expr::Literal(n),
        Token::String(s) => Expr::Literal(s),
        Token::Identifier(id) => Expr::Identifier(id),
        Token::LParen => {
            let expr = parse_expression(tokens, 0)?;
            if tokens.next_token()? != Token::RParen {
                return Err(ParseError::ExpectedToken(")".to_string()));
            }
            expr
        }
        t => return Err(ParseError::UnexpectedToken(format!("{:?}", t))),
    };

    loop {
        let op = match tokens.peek_token()? {
            Some(Token::Plus) => "+",
            Some(Token::Minus) => "-",
            Some(Token::Star) => "*",
            Some(Token::Slash) => "/",
            Some(Token::Equal) => "=",
            Some(Token::NotEqual) => "!=",
            Some(Token::Less) => "<",
            Some(Token::Greater) => ">",
            Some(Token::LessEqual) => "<=",
            Some(Token::GreaterEqual) => ">=",
            _ => break,
        };

        let prec = get_precedence(op);
        if prec < min_prec {
            break;
        }
        tokens.next_token()?; // consume operator

        let mut right = parse_expression(tokens, prec + 1)?;
        left = Expr::Binary {
            left: Box::new(left),
            op: op.to_string(),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn get_precedence(op: &str) -> u8 {
    match op {
        "*" | "/" => 5,
        "+" | "-" => 4,
        "=" | "!=" | "<" | "<=" | ">" | ">=" => 3,
        _ => 0,
    }
}
