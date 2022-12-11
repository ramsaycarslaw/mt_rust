use crate::tokens::Token;

#[derive(Debug, PartialEq)]
// Expression compiles to a literal
pub enum Expression {
    Prefix(Token, Box<Expression>),
    Infix(Box<Expression>, Token, Box<Expression>),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Variable(String),
    Assign(String, Box<Expression>),
    Null,
}

// Statement compiles to an expression
pub enum Statement {
    Expression(Expression),
    If(Box<Expression>, Box<Statement>),
    IfElse(Box<Expression>, Box<Statement>, Box<Statement>),
    While(Expression, Box<Statement>),
    Let(String, String, Box<Expression>),
    Block(Vec<Statement>),
    Print(Expression),
}
