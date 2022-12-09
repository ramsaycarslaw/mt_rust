use std::collections::HashMap;

use crate::ast::*;
use crate::tokens::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Null,
}

fn string_to_type(s: &str) -> Type {
    match s {
        "int64" => Type::Integer,
        "float64" => Type::Float,
        "string" => Type::String,
        "bool" => Type::Boolean,
        "null" => Type::Null,
        _ => panic!("Invalid type"),
    }
}

pub fn ty_of(expr: &Expression, map: HashMap<String, String>) -> Type {
    match expr {
        Expression::Integer(_) => Type::Integer,
        Expression::Float(_) => Type::Float,
        Expression::Boolean(_) => Type::Boolean,
        Expression::String(_) => Type::String,
        Expression::Identifier(name) => {
            string_to_type(map.get(name).unwrap())
        }
        Expression::Infix(e1, op, e2) => {
            match op {
                Token::BangEqual | Token::DoubleEqual | Token::Greater | Token::GreaterEqual |
                    Token::Less | Token::LessEqual | Token::And | Token::Or | Token::Xor => {
                    match (ty_of(e1, map.clone()), ty_of(e2, map.clone())) {
                        (Type::Boolean, Type::Boolean) => Type::Boolean,
                        (Type::Integer, Type::Integer) => Type::Boolean,
                        (Type::Float, Type::Float) => Type::Boolean,
                        (Type::String, Type::String) => Type::Boolean,
                        _ => panic!("Type mismatch for logical infix operator"),
                    }
                }

                Token::Plus => {
                    match (ty_of(e1, map.clone()), ty_of(e2, map.clone())) {
                        (Type::Integer, Type::Integer) => Type::Integer,
                        (Type::Float, Type::Float) => Type::Float,
                        (Type::String, Type::String) => Type::String,
                        _ => panic!("Type mismatch for infix operator +"),
                    }
                }

                Token::Minus | Token::Times | Token::Divide => {
                    match (ty_of(e1, map.clone()), ty_of(e2, map.clone())) {
                        (Type::Integer, Type::Integer) => Type::Integer,
                        (Type::Float, Type::Float) => Type::Float,
                        _ => panic!("Invalid types for infix operator"),
                    }
                }
                _ => panic!("Invalid infix operator"),
            }
        }
        Expression::Null => Type::Null,
        _ => panic!("Not implemented"),
    }
}

fn typecheck_statement(stmt: &Statement, map: HashMap<String, String>)  -> Type {
    match stmt {
        Statement::Expression(expr) => ty_of(expr, map.clone()),
        Statement::Print(e) => ty_of(e, map.clone()),
        Statement::If(e, s) => {
            if ty_of(e, map.clone()) != Type::Boolean {
                panic!("Condition must be boolean")
            }
            typecheck_statement(s, map.clone())
        }
        Statement::IfElse(e, s1, s2) => {
            if ty_of(e, map.clone()) != Type::Boolean {
                panic!("Condition must be boolean")
            }
            let t1 = typecheck_statement(s1, map.clone());
            let t2 = typecheck_statement(s2, map.clone());
            if t1 == t2 {
                t1
            } else {
                panic!("Type mismatch: {:?} and {:?}", t1, t2)
            }
        }
        Statement::Let(_, s, e) => {
            if ty_of(e, map.clone()) == string_to_type(s) {
                ty_of(e, map.clone())
            } else {
                panic!("Type mismatch: {:?} and {:?}", ty_of(e, map.clone()), string_to_type(s))
            }
        }
        Statement::Block(stmts) => {
            let mut t = Type::Null;
            for stmt in stmts {
                t = typecheck_statement(stmt, map.clone());
            }
            t
        }
    }
}

pub fn typecheck(ast: &Vec<Statement>, type_map: HashMap<String, String>) -> Vec<Type> {
    let mut types = Vec::new();
    for stmt in ast {
        types.push(typecheck_statement(&stmt, type_map.clone()));
    }
    types
}
