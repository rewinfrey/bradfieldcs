use super::ast::{Expr, Stmt};
use super::error::{error, ErrorKind};
use super::token::{Literal, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    loop_depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
            loop_depth: 0,
        }
    }

    /*
    program        -> statement* EOF ;

    declaration    -> varDecl
                    | statement ;

    statement      -> exprStmt
                    | printStmt
                    | block
                    | ifStmt
                    | whileStmt ;

    exprStmt       -> expression ";" ;
    printStmt      -> "print" expression ";" ;
    block          -> "{" declaration* "}" ;
    ifStmt         -> "if" "(" expression ")" statement
                    ( "else" statement )? ;
    whileStmt      -> "while" "(" expression ")" statement | break;

    varDecl        -> "var" identifier ( "=" expression )? ";" ;
    expression     -> assignment ;
    assignment     → identifier "=" assignment
                    | logic_or ;
    logic_or       -> logic_and ( "or" logic_and )* ;
    logic_and      -> equality ( "and" equality )* ;
    equality       → comparison ( ( "!=" | "==" ) comparison )* ;

    let binding    -> identifier "=" expression ;
    comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    term           → factor ( ( "-" | "+" ) factor )* ;
    factor         → unary ( ( "/" | "*" ) unary )* ;
    unary          → ( "!" | "-" ) unary | call ;

    call           -> primary ( "(" arguments? ")" )* ;
    arguments      -> expression ( "," expression )* ;

    primary        → NUMBER | STRING | "true" | "false" | "nil"
                    | "(" expression ")"
                    | identifier ;
    */
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ()> {
        let mut stmts = Vec::new();
        while !self.at_end() {
            stmts.push(self.declaration()?);
        }
        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, ()> {
        if self.match_token(vec![TokenType::Var]) {
            if let Ok(var_decl) = self.var_declaration() {
                return Ok(var_decl);
            }
        }
        self.statement().or_else(|_| {
            self.synchronize();
            Err(())
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::Identifier, "Expect variable name.");
        let name = self.previous();

        let mut initializer = None;
        if self.match_token(vec![TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );

        Ok(Stmt::VarDeclaration(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, ()> {
        if self.match_token(vec![TokenType::Break]) {
            if self.loop_depth < 1 {
                error(
                    self.previous().line,
                    self.previous().column,
                    self.previous().column,
                    String::from("Must be inside loop to use 'break' statement."),
                    ErrorKind::ParseError,
                );
            }
            self.consume(TokenType::Semicolon, "Expect ';' after 'break'.");
            return Ok(Stmt::Break);
        }

        if self.match_token(vec![TokenType::For]) {
            return self.for_stmt();
        }

        if self.match_token(vec![TokenType::If]) {
            return self.if_stmt();
        }

        if self.match_token(vec![TokenType::Print]) {
            return self.print_stmt();
        }

        if self.match_token(vec![TokenType::While]) {
            return self.while_stmt();
        }

        if self.match_token(vec![TokenType::LeftBrace]) {
            return self.block();
        }

        self.expression_statement()
    }

    fn for_stmt(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        let initializer: Option<Stmt>;
        if self.match_token(vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(vec![TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after loop increment.");

        self.loop_depth += 1;
        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::Block(vec![body, Stmt::ExprStmt(increment)]);
        }

        body = match condition {
            None => Stmt::While(Expr::TrueLiteral, Box::new(body)),
            Some(expr) => Stmt::While(expr, Box::new(body)),
        };

        self.loop_depth -= 1;
        match initializer {
            None => Ok(body),
            Some(expr) => Ok(Stmt::Block(vec![expr, body])),
        }
    }

    fn if_stmt(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.");

        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;

        if self.match_token(vec![TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }
        Ok(Stmt::IfStmt(condition, then_branch, else_branch))
    }

    fn while_stmt(&mut self) -> Result<Stmt, ()> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.");
        self.loop_depth += 1;
        let body = Box::new(self.statement()?);
        self.loop_depth -= 1;
        Ok(Stmt::While(condition, body))
    }

    fn block(&mut self) -> Result<Stmt, ()> {
        let mut stmts = Vec::new();

        while !self.check(TokenType::RightBrace) {
            stmts.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        Ok(Stmt::Block(stmts))
    }

    fn print_stmt(&mut self) -> Result<Stmt, ()> {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Ok(Stmt::Print(value?))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ()> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Ok(Stmt::ExprStmt(expr?))
    }

    fn expression(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn and(&mut self) -> Result<Expr, ()> {
        let mut expr = self.equality()?;

        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ()> {
        let mut expr = self.and()?;

        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        let expr = self.or()?;

        if self.match_token(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(name) => {
                    return Ok(Expr::Assignment(name, Box::new(value)));
                }
                _ => self.error(equals, "Invalid assignment target"),
            }
        }

        Ok(expr)
    }

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

        self.call()
    }

    fn call(&mut self) -> Result<Expr, ()> {
        let mut expr = self.primary();

        loop {
            if self.match_token(vec![TokenType::LeftParen]) {
                expr = self.finish_call(&expr?);
            } else {
                break;
            }
        }

        expr
    }

    fn finish_call(&mut self, callee: &Expr) -> Result<Expr, ()> {
        let mut args = Vec::new();

        if !self.match_token(vec![TokenType::RightParen]) {
            loop {
                if args.len() > 255 {
                    let next = self.peek();
                    error(
                        next.line,
                        next.column,
                        next.column,
                        String::from("Can't have more than 255 arguments."),
                        ErrorKind::ParseError,
                    );
                }
                args.push(self.expression()?);
                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after arguments.");
        let paren = self.previous();

        return Ok(Expr::Call(Box::new(callee.clone()), paren, args));
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
            return Ok(Expr::Variable(self.previous()));
        }

        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' to close expression");
            return Ok(Expr::Grouping(Box::new(expr?)));
        }

        self.error(self.peek(), "Expect expression");
        Err(())
    }

    // Shared utilities between implementations.

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
