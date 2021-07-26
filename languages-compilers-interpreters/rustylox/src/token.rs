use std::fmt::Display;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // Multi-character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,

    Unknown,
}

#[derive(Clone, Debug)]
pub enum Object {
    Identifier(String),
    Number(f64),
    Bool(bool),
    String(String),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Identifier(identifier) => write!(f, "{}", identifier),
            Object::Number(number) => write!(f, "{}", number),
            Object::Bool(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: u32,
    pub column: u32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: u32,
        column: u32,
        literal: Option<Object>,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}
