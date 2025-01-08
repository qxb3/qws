mod token;
mod lexer;

pub use token::Tokens;
pub use token::Token;

pub use lexer::Lexer;
pub use lexer::LexerErr;
pub use lexer::LexerResult;

pub fn lex(code: &String) -> LexerResult<Vec<Token>> {
    let mut lexer = Lexer::new(code);
    lexer.lex()
}
