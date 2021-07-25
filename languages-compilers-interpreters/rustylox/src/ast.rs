use super::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{}", "(");
        match &*self {
            Expr::Binary(left, op, right) => {
                let _ = write!(f, "{} {} {}", op, left, right);
            }
            Expr::Grouping(expr) => {
                let _ = write!(f, "group {}", expr);
            }
            Expr::Literal(lit) => {
                let _ = write!(f, "{}", lit.lexeme);
            }
            Expr::Unary(op, expr) => {
                let _ = write!(f, "{} {}", op, expr);
            }
        }
        write!(f, "{}", ")")
    }
}
