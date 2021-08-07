use super::error::{error, ErrorKind};
use super::value::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Environment<T> {
    pub values: HashMap<String, T>,
}

impl Environment<Value> {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &String, value: Value) {
        self.values.insert(name.clone(), value);
    }

    pub fn assign(&mut self, name: &String, value: Value) {
        if let Some(_) = self.get(name.as_str()) {
            self.values.insert(name.clone(), value);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value);
        }

        error(
            0,
            0,
            0,
            format!("undefined variable: {}", name),
            ErrorKind::RuntimeError,
        );
        None
    }
}
