use super::ast::Expr;
use super::error::{error, ErrorKind};
use std::fmt::Display;

#[derive(Debug, Display)]
pub enum Value {
    Nil,
    Bool(bool),
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
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
}

pub fn evaluate(ast: Expr) -> Result<Value, ()> {
    match ast {
        Expr::Binary(l, op, r) => {
            let l_result: Option<f64> = evaluate(*l)?.into();
            let r_result: Option<f64> = evaluate(*r)?.into();
            match op.lexeme.as_str() {
                "+" => Ok(Value::Number(l_result.unwrap() + r_result.unwrap())),
                "-" => Ok(Value::Number(l_result.unwrap() - r_result.unwrap())),
                "*" => Ok(Value::Number(l_result.unwrap() * r_result.unwrap())),
                "/" => Ok(Value::Number(l_result.unwrap() / r_result.unwrap())),
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
        Expr::NumberLiteral(n) => Ok(Value::Number(n)),
        Expr::Unary(op, expr) => {
            let unary: Option<f64> = evaluate(*expr)?.into();
            match op.lexeme.as_str() {
                "-" => Ok(Value::Number(-unary.unwrap())),
                "+" => Ok(Value::Number(unary.unwrap())),
                _ => {
                    error(0, 0, 0, String::from("Unknown unary operator error"), ErrorKind::EvaluatorError);
                    Err(())
                }
            }
        }
        _ => {
            error(0, 0, 0, String::from("Unknown expression error"), ErrorKind::EvaluatorError);
            Err(())
        }
        // Question: what is the best way to handle heterogeneous result types?
        // Expr::Identifier(s) => Ok(0),
        // Expr::BoolLiteral(bool) => Ok(bool),
        // BoolLiteral(bool),
        // NilLiteral,
        // StringLiteral(String),
    }
}
