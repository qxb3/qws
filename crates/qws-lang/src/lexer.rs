use std::{iter::Peekable, str::Chars};

use crate::token::{Token, Tokens};

#[derive(Debug)]
pub struct LexerErr {
    pub message: String,
    pub char: Option<char>,
    pub line: usize,
    pub col: usize
}

impl ToString for LexerErr {
    fn to_string(&self) -> String {
        format!(
            "{}.\nEncountered on:\n  line  : {}\n  col   : {}\n  char  : {:?}",
            self.message,
            self.line,
            self.col,
            self.char
        )
    }
}

pub type LexerResult<T> = Result<T, LexerErr>;

#[derive(Debug)]
pub struct Lexer<'a> {
    current_char: Option<char>,
    chars: Peekable<Chars<'a>>,
    line: usize,
    col: usize
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a String) -> Self {
        let mut chars = code.chars().peekable();

        Self {
            current_char: chars.next(),
            chars,
            line: 1,
            col: 1
        }
    }

    pub fn lex(&mut self) -> LexerResult<Vec<Token>> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(char) = self.current_char {
            // Skip comment
            if char == '-' && self.peek(['-']) {
                while let Some(char) = self.current_char {
                    if char == '\n' {
                        break;
                    }

                    self.col += 1;
                    self.advance();
                }

                continue;
            }

            // New line
            if char == '\n' {
                self.line += 1;
                self.col = 1;

                self.advance();

                continue;
            }

            // Whitespace
            if char.is_whitespace() {
                self.col += 1;
                self.advance();

                continue;
            }

            // String
            if char == '"' {
                let string = self.lex_string()?;
                tokens.push(string);

                continue;
            }

            // Script
            if char == 'S' && self.peek("cript".chars()) {
                let script = self.lex_raw(Tokens::Script)?;
                tokens.push(script);

                continue;
            }

            // Css
            if char == 'C' && self.peek("ss".chars()) {
                let css = self.lex_raw(Tokens::Css)?;
                tokens.push(css);

                continue;
            }

            // Int / Float
            if char.is_numeric() {
                let string = self.lex_number()?;
                tokens.push(string);

                continue;
            }

            // Identifier
            if (char.is_alphanumeric() || char == '_') && !char.is_numeric() {
                let identifier = self.lex_identifier()?;
                tokens.push(identifier);

                continue;
            }

            // Symbols
            let symbol = self.lex_symbols()?;
            tokens.push(symbol);
        }

        Ok(tokens)
    }

    fn lex_string(&mut self) -> LexerResult<Token> {
        let mut buff = String::new();

        // Skip '"'
        self.col += 1;
        self.advance();

        while let Some(char) = self.current_char {
            if char == '"' {
                self.col += 1;
                self.advance();
                break;
            }

            buff.push(char.to_owned());
            self.advance();
        }

        Ok(Token::new(
            Tokens::String,
            Some(buff),
            self.line,
            self.col
        ))
    }

    fn lex_number(&mut self) -> LexerResult<Token> {
        let mut buff = String::new();
        let mut is_float = false;

        while let Some(char) = self.current_char {
            match char {
                '0'..='9' => buff.push(char),
                '.' if !is_float => {
                    is_float = true;
                    buff.push(char);
                },
                _ => break
            }

            self.col += 1;
            self.advance();
        }

        let token_type = match is_float {
            true => Tokens::Float,
            false => Tokens::Int
        };

        return Ok(Token::new(
            token_type,
            Some(buff),
            self.line,
            self.col
        ))
    }

    fn lex_identifier(&mut self) -> LexerResult<Token> {
        let mut buff = String::new();

        while let Some(char) = self.current_char {
            if !char.is_alphanumeric() && char != '_' {
                break;
            }

            buff.push(char);

            self.col += 1;
            self.advance();
        }

        let r#type = match buff.as_str() {
            "true" | "false"    => Tokens::Boolean,
            _                   => Tokens::Identifier
        };

        Ok(Token::new(
            r#type,
            Some(buff),
            self.line,
            self.col
        ))
    }

    fn lex_symbols(&mut self) -> LexerResult<Token> {
        if let Some(char) = self.current_char {
            let r#type = match char {
                '=' => Some(Tokens::Equal),
                ',' => Some(Tokens::Comma),

                '{' => Some(Tokens::OpenCurly),
                '}' => Some(Tokens::CloseCurly),

                '[' => Some(Tokens::OpenBracket),
                ']' => Some(Tokens::CloseBracket),

                _ => None
            };

            if let Some(r#type) = r#type {
                self.advance();
                self.col += 1;

                return Ok(Token::new(
                    r#type,
                    None,
                    self.line,
                    self.col
                ))
            }

            return Err(LexerErr {
                message: "Unrecognized symbol".to_string(),
                char: Some(char),
                line: self.line,
                col: self.col
            });
        }

        Err(LexerErr {
            message: "Unexpected end of input".to_string(),
            char: None,
            line: self.line,
            col: self.col
        })
    }

    fn lex_raw(&mut self, r#type: Tokens) -> LexerResult<Token> {
        let mut buff = String::new();
        let mut braces = 0;

        while let Some(char) = self.current_char {
            if char == '{' {
                braces += 1;

                self.col += 1;
                self.advance();

                break;
            }

            self.advance();
        }

        while braces > 0 {
            if let Some(char) = self.current_char {
                match char {
                    '{' => braces += 1,
                    '}' => braces -= 1,
                    _ => {}
                }

                buff.push(char);

                self.col += 1;
                self.advance();
            }
        }

        Ok(Token::new(
            r#type,
            Some(buff),
            self.line,
            self.col
        ))
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next();
    }

    fn peek<T>(&mut self, peeks: T) -> bool
    where
        T: IntoIterator,
        T::Item: Into<char>
    {
        let mut chars = self.chars.clone();

        for char in peeks {
            if let Some(c) = chars.next() {
                if c != char.into() {
                    return false;
                }
            }
        }

        return true;
    }
}
