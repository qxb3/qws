#[derive(Debug)]
pub enum Tokens {
    Identifier,

    String,
    Int,
    Float,
    Boolean,

    SString,
    Script,
    Css,

    Equal,
    Comma,

    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBracket
}

#[derive(Debug)]
pub struct Token {
    pub r#type: Tokens,
    pub value: Option<String>,
    pub line: usize,
    pub col: usize
}

impl Token {
    pub fn new(r#type: Tokens, value: Option<String>, line: usize, col: usize) -> Self {
        Self {
            r#type,
            value,
            line,
            col
        }
    }
}
