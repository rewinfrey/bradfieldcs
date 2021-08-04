use super::ast::Expr;
use super::error::{error, ErrorKind};
use super::token::TokenType;
use std::fmt::Display;

#[derive(Debug, Display)]
pub enum Value {
    Nil,
    True,
    False,
    Number(f64),
    String(String),
}

impl Into<Option<f64>> for Value {
    fn into(self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }
}

impl Into<Option<String>> for Value {
    fn into(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

impl Into<Option<bool>> for Value {
    fn into(self) -> Option<bool> {
        match self {
            Value::True => Some(true),
            Value::False => Some(false),
            _ => None,
        }
    }
}

pub fn evaluate(ast: Expr) -> Result<Value, ()> {
    match ast {
        Expr::Binary(l, op, r) => {
            // To correctly implement this, I can use combinators that attempt to resolve an expr to a value for a given type,
            // and continue until no more possible values are possible (which results in an error).
            let l_result: Option<f64> = evaluate(*l)?.into();
            let r_result: Option<f64> = evaluate(*r)?.into();
            match op.token_type {
                TokenType::Plus => Ok(Value::Number(l_result.unwrap() + r_result.unwrap())),
                TokenType::Minus => Ok(Value::Number(l_result.unwrap() - r_result.unwrap())),
                TokenType::Star => Ok(Value::Number(l_result.unwrap() * r_result.unwrap())),
                TokenType::Slash => Ok(Value::Number(l_result.unwrap() / r_result.unwrap())),
                _ => {
                    error(
                        op.line,
                        op.column,
                        op.column,
                        String::from("Unknown operator error"),
                        ErrorKind::EvaluatorError,
                    );
                    Err(())
                }
            }
        }
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::TrueLiteral => Ok(Value::True),
        Expr::FalseLiteral => Ok(Value::False),
        Expr::NumberLiteral(n) => Ok(Value::Number(n)),
        Expr::StringLiteral(s) => Ok(Value::String(s)),
        Expr::Identifier(id) => Ok(Value::String(id)),
        Expr::NilLiteral => Ok(Value::Nil),
        Expr::Unary(op, expr) => match op.token_type {
            TokenType::Plus => {
                let unary: Option<f64> = evaluate(*expr)?.into();
                Ok(Value::Number(-unary.unwrap()))
            }
            TokenType::Bang => {
                let unary: Option<bool> = evaluate(*expr)?.into();
                match !unary.unwrap() {
                    true => Ok(Value::True),
                    false => Ok(Value::False),
                }
            }
            _ => {
                error(
                    0,
                    0,
                    0,
                    String::from("Unknown unary operator error"),
                    ErrorKind::EvaluatorError,
                );
                Err(())
            }
        },
        _ => {
            error(
                0,
                0,
                0,
                String::from("Unknown expression error"),
                ErrorKind::EvaluatorError,
            );
            Err(())
        }
    }
}
