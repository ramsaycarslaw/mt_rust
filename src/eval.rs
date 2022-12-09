use std::fmt::{Display, Formatter, Result};

use crate::ast::*;
use crate::tokens::*;
use crate::environment::*;

#[derive(Debug, PartialEq, Clone)]
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
        Expression::Boolean(b) => Value::Bool(*b),
        Expression::String(s) => Value::String(s.clone()),

        Expression::Prefix(t, e) => {
            let right = eval_expression(&e);
            match t {
                Token::Minus => match right {
                    Value::Int(i) => Value::Int(-i),
                    Value::Float(fl) => Value::Float(-fl),
                    _ => panic!("Invalid operand for prefix '-'"),
                },
                Token::Bang => match right {
                    Value::Bool(b) => Value::Bool(!b),
                    _ => panic!("Invalid operand for prefix '!'"),
                },
                _ => panic!("Invalid prefix operator"),
            }
        }

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
                Token::DoubleEqual => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Bool(l == r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l == r),
                    (Value::Bool(l), Value::Bool(r)) => Value::Bool(l == r),
                    (Value::String(l), Value::String(r)) => Value::Bool(l == r),
                    (Value::Null, Value::Null) => Value::Bool(true),
                    _ => Value::Bool(false),
                },
                Token::Or => match (left, right) {
                    (Value::Bool(l), Value::Bool(r)) => Value::Bool(l || r),
                    _ => panic!("Invalid types for ||"),
                },
                Token::And => match (left, right) {
                    (Value::Bool(l), Value::Bool(r)) => Value::Bool(l && r),
                    _ => panic!("Invalid types for &&"),
                },
                Token::BangEqual => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Bool(l != r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l != r),
                    (Value::Bool(l), Value::Bool(r)) => Value::Bool(l != r),
                    (Value::String(l), Value::String(r)) => Value::Bool(l != r),
                    (Value::Null, Value::Null) => Value::Bool(false),
                    _ => Value::Bool(true),
                },
                Token::Greater => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Bool(l > r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l > r),
                    _ => panic!("Invalid types for >"),
                },
                Token::Less => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Bool(l < r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l < r),
                    _ => panic!("Invalid types for <"),
                },
                Token::GreaterEqual => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Bool(l >= r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l >= r),
                    _ => panic!("Invalid types for >="),
                },
                Token::LessEqual => match (left, right) {
                    (Value::Int(l), Value::Int(r)) => Value::Bool(l <= r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l <= r),
                    _ => panic!("Invalid types for <="),
                },
                _ => panic!("Invalid infix operator"),
            }
        }

        _ => Value::Null,
    }
}

fn eval_statement(x: &Statement) -> Value {
    let mut env = Environment::new();
    match x {
        Statement::Expression(e) => {
            return eval_expression(&e);
        },
        Statement::Print(expr) => {
            let value = eval_expression(&expr);
            println!("{}", value);
            return value;
        }
        Statement::If(condition, consequence) => {
            let condition = eval_expression(&condition);
            if let Value::Bool(b) = condition {
                if b {
                    eval_statement(&consequence);
                }
            }
            return Value::Null;
        }
        Statement::IfElse(condition, consequence, alternative) => {
            let condition = eval_expression(&condition);
            if let Value::Bool(b) = condition {
                if b {
                    eval_statement(&consequence);
                } else {
                    eval_statement(&alternative);
                }
            }
            return Value::Null;
        }
        Statement::Let(name, _ty, expr) => {
            env.define(name.clone(), eval_expression(&expr));
            return Value::Null;
        }
    }
}

pub fn eval(ast: Vec<Statement>) -> Vec<Value> {
    let mut values = Vec::new();
    for x in ast {
        values.push(eval_statement(&x));
    }
    values
}
