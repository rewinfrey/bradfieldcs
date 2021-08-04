use super::ast::Expr;
use super::error::{error, ErrorKind};
use super::token::{Literal, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ()> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.comparison();

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Ok(Expr::Binary(Box::new(expr?), operator, Box::new(right?)));
        }

        expr
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous();
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn at_end(&self) -> bool {
        self.tokens[self.current].token_type == TokenType::EOF
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut expr = self.term();

        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Ok(Expr::Binary(Box::new(expr?), operator, Box::new(right?)));
        }

        expr
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut expr = self.factor();

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Ok(Expr::Binary(Box::new(expr?), operator, Box::new(right?)));
        }

        expr
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut expr = self.unary();

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Ok(Expr::Binary(Box::new(expr?), operator, Box::new(right?)));
        }

        expr
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Ok(Expr::Unary(operator, Box::new(right?)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        if self.match_token(vec![TokenType::False]) {
            if let Some(Literal::False) = self.previous().literal {
                return Ok(Expr::FalseLiteral);
            } else {
                self.error(self.previous(), "Expected boolean");
                return Err(());
            }
        }

        if self.match_token(vec![TokenType::True]) {
            if let Some(Literal::True) = self.previous().literal {
                return Ok(Expr::TrueLiteral);
            } else {
                self.error(self.previous(), "Expected boolean");
                return Err(());
            }
        }

        if self.match_token(vec![TokenType::Nil]) {
            return Ok(Expr::NilLiteral);
        }

        if self.match_token(vec![TokenType::Number]) {
            if let Some(Literal::Number(n)) = self.previous().literal {
                return Ok(Expr::NumberLiteral(n));
            } else {
                self.error(self.previous(), "Expected number");
                return Err(());
            }
        }

        if self.match_token(vec![TokenType::String]) {
            if let Some(Literal::String(s)) = self.previous().literal {
                return Ok(Expr::StringLiteral(s));
            } else {
                self.error(self.previous(), "Expected string");
                return Err(());
            }
        }

        if self.match_token(vec![TokenType::Identifier]) {
            if let Some(Literal::Identifier(s)) = self.previous().literal {
                return Ok(Expr::Identifier(s));
            } else {
                self.error(self.previous(), "Expected identifier");
                return Err(());
            }
        }

        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' to close expression");
            return Ok(Expr::Grouping(Box::new(expr?)));
        }

        self.error(self.peek(), "Expect expression");
        Err(())
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(token_type) {
            return self.advance();
        }

        self.error(self.peek(), message)
    }

    fn error(&self, token: Token, message: &str) {
        let mut msg = message.to_string();
        if token.token_type == TokenType::EOF {
            msg += " at end of input";
        }
        error(
            token.line,
            token.column,
            token.column,
            message.to_string(),
            ErrorKind::ParseError,
        );
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            }
        }
    }
}
