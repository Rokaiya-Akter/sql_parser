mod tokenizer;
mod parser;
mod ast;
mod error;

use tokenizer::Tokenizer;
use parser::{parse_create_table, parse_select};

fn main() {
    // Hardcoded SQL string
    let sql = "SELECT name, age FROM users WHERE age = 18";

    let mut tokenizer = Tokenizer::new(sql);

    // Try parsing SELECT statement first
    match parse_select(&mut tokenizer) {
        Ok(statement) => println!("Parsed statement: {:#?}", statement),
        Err(_) => {
            // If SELECT fails, try CREATE TABLE parsing
            let mut tokenizer = Tokenizer::new(sql); // Reset tokenizer
            match parse_create_table(&mut tokenizer) {
                Ok(statement) => println!("Parsed statement: {:#?}", statement),
                Err(e) => eprintln!("Parse error: {:?}", e),
            }
        }
    }
}
