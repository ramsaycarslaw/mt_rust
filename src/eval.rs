use std::fmt::{Display, Formatter, Result};

use crate::ast::*;
use crate::tokens::*;

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Null => write!(f, "null"),
        }
    }
}

fn eval_expression(expr: &Expression) -> Value {
    match expr {
        Expression::Integer(i) => Value::Int(*i),
        Expression::Float(f) => Value::Float(*f),
        Expression::String(s) => Value::String(s.clone()),

        Expression::Infix(left, op, right) => {
            let left = eval_expression(left);
            let right = eval_expression(right);
            match op {
                Token::Plus => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l + r),
                    (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
                    (Value::String(l), Value::String(r)) => Value::String(l + &r),
                    _ => panic!("Invalid types for +"),
                },
                Token::Minus => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l - r),
                    (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
                    _ => panic!("Invalid types for -"),
                },
                Token::Times => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l * r),
                    (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
                    _ => panic!("Invalid types for *"),
                },
                Token::Divide => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Int(l / r),
                    (Value::Float(l), Value::Float(r)) => Value::Float(l / r),
                    _ => panic!("Invalid types for /"),
                },
                _ => panic!("Invalid infix operator"),
            }
        }

        _ => Value::Null,
    }
}

pub fn eval(ast: Vec<Statement>) -> Vec<Value> {
    let mut values = Vec::new();
    for x in ast {
        match x {
            Statement::Expression(e) => {
                let value = eval_expression(&e);
                values.push(value);
            },
            Statement::Print(expr) => {
                let value = eval_expression(&expr);
                println!("{}", value);
            }
        }
    }
    values
}
