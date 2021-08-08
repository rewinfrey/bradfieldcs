use super::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    Print(Expr),
    VarDeclaration(Token, Option<Expr>),
    Block(Vec<Stmt>),
    IfStmt(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

#[derive(Debug)]
pub enum Expr {
    Assignment(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Identifier(String),
    TrueLiteral,
    FalseLiteral,
    NilLiteral,
    NumberLiteral(f64),
    StringLiteral(String),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{}", "(");
        match &*self {
            Expr::Assignment(name, value) => {
                let _ = write!(f, "{} = {}", name, value);
            }
            Expr::Binary(left, op, right) => {
                let _ = write!(f, "{} {} {}", op, left, right);
            }
            Expr::Grouping(expr) => {
                let _ = write!(f, "group {}", expr);
            }
            Expr::FalseLiteral => {
                let _ = write!(f, "false");
            }
            Expr::TrueLiteral => {
                let _ = write!(f, "true");
            }
            Expr::NilLiteral => {
                let _ = write!(f, "{}", String::from("nil"));
            }
            Expr::NumberLiteral(n) => {
                let _ = write!(f, "{}", n);
            }
            Expr::StringLiteral(s) => {
                let _ = write!(f, "{}", s);
            }
            Expr::Unary(op, expr) => {
                let _ = write!(f, "{} {}", op, expr);
            }
            Expr::Identifier(s) => {
                let _ = write!(f, "{}", s);
            }
            Expr::Variable(identifier) => {
                let _ = write!(f, "{}", identifier);
            }
        }
        write!(f, "{}", ")")
    }
}
