use crate::tokens::Token;

pub enum Expression {
    Prefix(Token, Box<Expression>),
    Infix(Box<Expression>, Token, Box<Expression>),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Function(Vec<String>, Vec<Expression>),
    Call(Box<Expression>, Token, Vec<Expression>),
    Array(Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Hash(Vec<(Expression, Expression)>),
    Return(Box<Expression>),
    Let(String, Box<Expression>),
    Block(Vec<Expression>),
}

pub enum Statement {
    Expression(Expression),
    Print(Expression),
}
