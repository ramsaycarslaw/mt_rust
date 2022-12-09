use std::collections::HashMap;
use crate::eval::*;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_from(enclosing: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::default(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            Err(anyhow!("Undefined variable '{}'", name))
        }
    }
}
