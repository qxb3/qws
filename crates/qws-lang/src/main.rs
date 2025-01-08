mod token;
mod lexer;

use std::process;
use lexer::Lexer;

fn main() {
    let code = include_str!("../examples/simple.qws").to_string();

    let mut lexer = Lexer::new(&code);
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err.to_string());
            process::exit(1);
        }
    };

    println!("{:#?}", tokens);
}
