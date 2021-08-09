use std::fmt::Display;

#[derive(Clone, Debug, Display)]
pub enum Value {
    Break,
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
