#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
    Number,
    Add,
    Negate,
    Identifier,
    EndOfFile,
    Lparen,
    Rparen,
    Error
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: i32,
    pub col: i32
}