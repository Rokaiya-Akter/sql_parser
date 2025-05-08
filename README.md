# Rust SQL Parser

This is a simple SQL parser written in Rust. The project focuses on parsing `SELECT` and `CREATE TABLE` statements, including `FROM`, `WHERE`, and `ORDER BY` clauses. The parser utilizes the Pratt parsing technique and includes a tokenizer, error handling, and a basic command-line interface (CLI).

## Project Overview

- **Tokenizer**: Converts a string input into tokens that the parser can process.
- **Pratt Expression Parser**: Implements the Pratt parsing technique to handle operator precedence and parsing of expressions.
- **SQL Parser**: Parses SQL statements into a database `Statement` using the tokenizer and Pratt expression parser.
- **Error Handling**: Handles errors gracefully using the `Result<T, E>` enum.
- **CLI**: Provides a command-line interface that interacts with the user and parses SQL queries.

## Features

- **Tokenizer**:
  - Handles single-character tokens (`+`, `-`, `*`, etc.)
  - Supports multi-character operators (`>=`, `<=`, `!=`)
  - Tokenizes numbers, strings, keywords, and identifiers
  - Utilizes an iterator for efficient token processing

- **Pratt Expression Parser**:
  - Correct operator precedence
  - Supports parentheses and binary/unary operations

- **SQL Parser**:
  - Parses basic `SELECT` and `CREATE TABLE` statements
  - Supports optional `WHERE` and `ORDER BY` clauses
  - Handles column definitions with types and constraints

- **Error Handling**:
  - Gracefully handles errors in tokenization, parsing, and SQL statement construction


## Installation
1. Clone the repository:
   git clone https://github.com/Rokaiya-Akter/sql_parser.git
   cd sql-parser

2. Install dependencies:
   cargo build

3. To run the parser:
   cargo run

4. To test the parser:
   cargo test

5. Build the project
cargo build
6. Run the parser
cargo run

7. Example SQL input:
Editing the main.rs file to use my own SQL string:
let sql = "SELECT name, age FROM users WHERE age = 18";
Output:
Matched keyword: 'SELECT'
Columns: ["name", "age"]
Table: users
Condition: age = 18
Remaining input:
8.To run any available tests:
cargo test
