mod tokenizer;
mod parser;
mod ast;
mod error;

use tokenizer::Tokenizer;
use parser::parse_create_table;
use error::ParseError;

use std::io::{self, Write};

fn main() {
    loop {
        print!("sql> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let mut tokenizer = Tokenizer::new(&input);

        match parse_create_table(&mut tokenizer) {
            Ok(statement) => println!("Parsed statement: {:#?}", statement),
            Err(ParseError::TokenizerError(e)) => eprintln!("Tokenizer error: {}", e),
            Err(e) => eprintln!("Parse error: {:?}", e),
        }
    }
}
