use super::ast::{Expr, Stmt};
use super::environment::Environment;
use super::error::{error, ErrorKind};
use super::token::TokenType;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Interpreter<T> {
    pub environment: Environment<T>,
}

impl Interpreter<Value> {
    pub fn new(env: Environment<Value>) -> Interpreter<Value> {
        Interpreter { environment: env }
    }

    pub fn evaluate(&mut self, stmts: &Vec<Stmt>) -> Result<Value, ()> {
        let mut result = Value::Nil;
        for stmt in stmts {
            result = self.evaluate_stmt(stmt)?;
        }
        Ok(result)
    }

    pub fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::False => false,
            _ => true,
        }
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, ()> {
        match expr {
            Expr::Assignment(name, value) => {
                let result = self.evaluate_expr(&*value)?;
                self.environment.assign(&name.lexeme, result.clone());
                Ok(result)
            }
            Expr::Binary(l, op, r) => {
                // To correctly implement this, I can use combinators that attempt to resolve an expr to a value for a given type,
                // and continue until no more possible values are possible (which results in an error).
                let l_result: Option<f64> = self.evaluate_expr(&*l)?.into();
                let r_result: Option<f64> = self.evaluate_expr(&*r)?.into();
                match op.token_type {
                    TokenType::Plus => Ok(Value::Number(l_result.unwrap() + r_result.unwrap())),
                    TokenType::Minus => Ok(Value::Number(l_result.unwrap() - r_result.unwrap())),
                    TokenType::Star => Ok(Value::Number(l_result.unwrap() * r_result.unwrap())),
                    TokenType::Slash => Ok(Value::Number(l_result.unwrap() / r_result.unwrap())),
                    TokenType::BangEqual => Ok(match l_result.unwrap() != r_result.unwrap() {
                        true => Value::True,
                        false => Value::False,
                    }),
                    TokenType::EqualEqual => Ok(match l_result.unwrap() == r_result.unwrap() {
                        true => Value::True,
                        false => Value::False,
                    }),
                    TokenType::Less => Ok(match l_result.unwrap() < r_result.unwrap() {
                        true => Value::True,
                        false => Value::False,
                    }),
                    TokenType::LessEqual => Ok(match l_result.unwrap() <= r_result.unwrap() {
                        true => Value::True,
                        false => Value::False,
                    }),
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
            Expr::Grouping(expr) => self.evaluate_expr(&*expr),
            Expr::TrueLiteral => Ok(Value::True),
            Expr::FalseLiteral => Ok(Value::False),
            Expr::NumberLiteral(n) => Ok(Value::Number(*n)),
            Expr::StringLiteral(s) => Ok(Value::String((*s).clone())),
            Expr::Identifier(id) => Ok(Value::String((*id).clone())),
            Expr::NilLiteral => Ok(Value::Nil),
            Expr::Unary(op, expr) => match op.token_type {
                TokenType::Plus => {
                    let unary: Option<f64> = self.evaluate_expr(&*expr)?.into();
                    Ok(Value::Number(-unary.unwrap()))
                }
                TokenType::Bang => {
                    let unary: Option<bool> = self.evaluate_expr(&*expr)?.into();
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
            Expr::Variable(name) => {
                if let Some(value) = self.environment.get(&name.lexeme) {
                    return Ok((*value).clone());
                }
                Err(())
            }
            Expr::Logical(left, op, right) => {
                let left_result = self.evaluate_expr(left)?;
                match op.token_type {
                    TokenType::Or => {
                        if Interpreter::is_truthy(&left_result) {
                            return Ok(left_result);
                        }
                    }
                    _ => {
                        if !Interpreter::is_truthy(&left_result) {
                            return Ok(left_result);
                        }
                    }
                }

                let right_result = self.evaluate_expr(right)?;
                Ok(right_result)
            }
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

    fn evaluate_stmt(&mut self, stmt: &Stmt) -> Result<Value, ()> {
        match stmt {
            Stmt::Print(expr) => {
                println!("{}", self.evaluate_expr(expr)?);
                Ok(Value::Nil)
            }
            Stmt::ExprStmt(expr) => self.evaluate_expr(expr),
            Stmt::VarDeclaration(name, expr) => {
                let mut value = Value::Nil;
                if let Some(initializer) = expr {
                    value = self.evaluate_expr(&*initializer)?;
                }
                self.environment.define(&name.lexeme, value);
                Ok(Value::Nil)
            }
            Stmt::Block(stmts) => {
                let env = self.environment.clone();

                for stmt in stmts {
                    let _ = self.evaluate_stmt(&stmt);
                }

                self.environment = env.clone();
                Ok(Value::Nil)
            }
            Stmt::IfStmt(condition, then_branch, else_branch) => {
                if Interpreter::is_truthy(&self.evaluate_expr(condition)?) {
                    self.evaluate_stmt(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.evaluate_stmt(else_branch)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::While(condition, body) => {
                while Interpreter::is_truthy(&self.evaluate_expr(condition)?) {
                    let _ = self.evaluate_stmt(body);
                }
                Ok(Value::Nil)
            }
        }
    }
}
