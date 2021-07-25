use super::error::{error, ErrorKind};
use super::token::{Token, TokenType};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
    line: u32,
    column: u32,
    current: u32,
    start: u32,
    reserved: HashMap<&'a str, TokenType>,
    pub chars: Peekable<Chars<'a>>,
}

pub fn default_reserved<'a>() -> HashMap<&'static str, TokenType> {
    let mut reserved = HashMap::new();
    reserved.insert("if", TokenType::If);
    reserved.insert("and", TokenType::And);
    reserved.insert("class", TokenType::Class);
    reserved.insert("else", TokenType::Else);
    reserved.insert("false", TokenType::False);
    reserved.insert("for", TokenType::For);
    reserved.insert("fun", TokenType::Fun);
    reserved.insert("if", TokenType::If);
    reserved.insert("nil", TokenType::Nil);
    reserved.insert("or", TokenType::Or);
    reserved.insert("print", TokenType::Print);
    reserved.insert("return", TokenType::Return);
    reserved.insert("super", TokenType::Super);
    reserved.insert("this", TokenType::This);
    reserved.insert("true", TokenType::True);
    reserved.insert("var", TokenType::Var);
    reserved.insert("while", TokenType::While);
    reserved
}

impl<'a> Scanner<'a> {
    pub fn new(reserved: HashMap<&'static str, TokenType>, source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            line: 1,
            column: 0,
            current: 0,
            start: 0,
            reserved: reserved,
            chars: source.chars().peekable(),
        }
    }

    fn done(&self) -> bool {
        return self.current >= (self.source.len() as u32);
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            self.substring().to_string(),
            self.line,
            self.column,
        ));
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.column += 1;
        self.chars.next()
    }

    fn match_char(&mut self, c: char) -> bool {
        if let Some(next) = self.chars.peek() {
            if *next == c {
                self.chars.next();
                self.current += 1;
                return true;
            }
        }
        false
    }

    fn substring(&self) -> &str {
        &self.source[(self.start as usize)..(self.current as usize)]
    }

    fn string(&mut self) {
        loop {
            while *self.chars.peek().or_else(|| Some(&'\0')).unwrap() != '"' && !self.done() {
                if *self.chars.peek().unwrap() == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                self.advance();
            }

            if self.done() {
                error(
                    self.line,
                    self.start,
                    self.current,
                    String::from("Unterminated string"),
                    ErrorKind::ScanError,
                );
                return;
            }

            self.advance();
            self.add_token(TokenType::String);
            break;
        }
    }

    fn number(&mut self) {
        while self.chars.peek().unwrap_or(&'\0').is_digit(10) {
            self.advance();
        }

        if *self.chars.peek().unwrap_or(&'\0') == '.' {
            self.advance();

            while self.chars.peek().unwrap_or(&'\0').is_digit(10) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number);
    }

    fn identifier(&mut self) {
        while self.chars.peek().unwrap_or(&'❤').is_ascii_alphabetic() {
            self.advance();
        }

        let mut token_type = TokenType::Identifier;
        if let Some(reserved) = self.reserved.get(self.substring()) {
            token_type = reserved.clone();
        }
        self.add_token(token_type);
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),

                '!' => {
                    let token_type = if self.match_char('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(token_type);
                }
                '=' => {
                    let token_type = if self.match_char('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                }
                '<' => {
                    let token_type = if self.match_char('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(token_type);
                }
                '>' => {
                    let token_type = if self.match_char('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(token_type);
                }

                '/' => {
                    if self.match_char('/') {
                        loop {
                            match self.chars.peek() {
                                Some('\n') | None => {
                                    break;
                                }
                                _ => {
                                    self.advance();
                                }
                            }
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }

                ' ' => self.column += 1,
                '\t' => self.column += 4,
                '\r' => {}

                '\n' => {
                    self.line += 1;
                    self.column = 0;
                }

                '"' => self.string(),
                _ => {
                    if c.is_digit(10) {
                        self.number();
                    } else if c.is_ascii_alphabetic() {
                        self.identifier();
                    } else {
                        error(
                            self.line,
                            self.column,
                            self.column + (self.current - self.start),
                            String::from("Unknown character"),
                            ErrorKind::ScanError,
                        );
                        self.add_token(TokenType::Unknown);
                    }
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ErrorKind> {
        while !self.done() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            self.line,
            self.column,
        ));
        Ok(self.tokens.clone())
    }
}
