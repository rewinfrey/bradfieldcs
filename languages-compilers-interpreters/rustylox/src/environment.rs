use super::error::{error, ErrorKind};
use super::value::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Environment<T> {
    pub values: HashMap<String, T>,
    pub enclosing: Option<Box<Environment<T>>>,
}

impl Environment<Value> {
    pub fn new(enclosing: Option<Box<Environment<Value>>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: enclosing,
        }
    }

    pub fn define(&mut self, name: &String, value: Value) {
        self.values.insert(name.clone(), value);
    }

    pub fn assign(&mut self, name: &String, value: Value) {
        if let Some(_) = self.values.get(name.as_str()) {
            self.values.insert(name.clone(), value.clone());
            return;
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value.clone());
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
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
