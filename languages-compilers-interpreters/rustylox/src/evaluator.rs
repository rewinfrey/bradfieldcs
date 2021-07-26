use super::ast::Expr;
use super::error::{error, ErrorKind};

pub fn evaluate(ast: Expr) -> Result<f64, ()> {
    match ast {
        Expr::Binary(l, op, r) => {
            let l_result = evaluate(*l);
            let r_result = evaluate(*r);
            match op.lexeme.as_str() {
                "+" => Ok(l_result? + r_result?),
                "-" => Ok(l_result? - r_result?),
                "*" => Ok(l_result? * r_result?),
                "/" => Ok(l_result? / r_result?),
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
        Expr::NumberLiteral(n) => Ok(n),
        Expr::Unary(op, expr) => {
            let unary = evaluate(*expr);
            match op.lexeme.as_str() {
                "-" => Ok(-unary?),
                "+" => Ok(unary?),
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
