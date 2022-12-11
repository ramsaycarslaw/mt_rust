use std::collections::HashMap;

use crate::tokens::Token;
use crate::ast::Expression;
use crate::ast::Statement;
use anyhow::Result;

// global parser object
pub struct Parser {
    type_map: HashMap<String, String>,
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            type_map: HashMap::new(),
            tokens: tokens,
            pos: 0,
        }
    }

    pub fn get_type_map(&self) -> &HashMap<String, String> {
        &self.type_map
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut ast = Vec::new();

        loop {
            // try parse
            if let Some(stmt) = self.declaration() {
                ast.push(stmt);
            }

            if self.done() {
                break;
            }
        }
        ast
    }

    fn declaration(&mut self) -> Option<Statement> {
        if self.expect(&Token::Let) {
            self.advance();
            if let Ok(stmt) = self.var_declaration() {
                return Some(stmt);
            } else {
                return None;
            }
        } else if let Ok(statement) = self.statement() {
            Some(statement)
        } else {
            None
        }
    }

    fn var_declaration(&mut self) -> Result<Statement> {
        let name = self.expect_identifier()?;
        self.advance();
        self.consume(&Token::Colon, "Expected ':' after variable name".to_string());
        let ty = self.expect_identifier()?;

        self.advance();

        let initialiser = if self.expect(&Token::Equal) {
            self.advance();
            self.expression().ok()
        } else {
            println!("No initialiser");
            None
        };

        self.consume(&Token::SemiColon, "Expected ';' after variable declaration".to_string());
        self.type_map.insert(name.clone(), ty.clone());
        Ok(Statement::Let(name, ty, Box::new(initialiser.unwrap())))
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.expect(&Token::Print) {
            self.print_statement()
        } else if self.expect(&Token::If) {
            self.if_statement()
        } else if self.expect(&&Token::LeftBrace) {
            Ok(Statement::Block(self.block()?))
        } else if self.expect(&Token::While) {
            self.while_statement()
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        self.advance();
        while !self.expect(&Token::RightBrace) && !self.done() {
            statements.push(self.declaration().unwrap());
        }
        self.advance();
        Ok(statements)

    }

    fn while_statement(&mut self) -> Result<Statement> {
        self.advance();
        self.consume(&Token::LeftParen, "Expected '(' after 'while'".to_string());
        let condition = self.expression()?;
        self.consume(&Token::RightParen, "Expected ')' after condition".to_string());
        let body = self.statement()?;
        Ok(Statement::While(condition, Box::new(body)))
    }

    fn if_statement(&mut self) -> Result<Statement> {
        self.advance();
        self.consume(&Token::LeftParen, "Expected '(' after 'if'".to_string());
        let condition = self.expression()?;
        self.consume(&Token::RightParen, "Expected ')' after if condition".to_string());
        let consequence = self.statement()?;
        if self.expect(&Token::Else) {
            self.advance();
        Ok(Statement::IfElse (
            Box::new(condition),
            Box::new(consequence),
            Box::new(self.statement()?),
        ))
        } else {
            Ok(Statement::If (
                Box::new(condition),
                Box::new(consequence),
            ))
        }
    }

    fn print_statement(&mut self) -> Result<Statement> {
        self.advance();
        let expr = self.expression()?;
        self.consume(&Token::SemiColon, "Expected ; after print".to_string());
        Ok(Statement::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(&Token::SemiColon, "Expected ';' after expression.".to_string());
        Ok(Statement::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expression> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression> {
        let expr = self.or()?;

        println!("Assignment: {:?}", expr);

        if self.expect(&Token::Equal) {
            self.advance();
            let equals = self.assignment()?;
            if let Expression::Variable(name) = expr {
                return Ok(Expression::Assign(name, Box::new(equals)));
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression> {
        let mut expr = self.and()?;

        while self.expect(&Token::Or) {
            self.advance();
            let op = self.previous();
            let right = self.and()?;
            expr = Expression::Infix(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression> {
        let mut expr = self.equality()?;

        while self.expect(&Token::And) {
            self.advance();
            let op = self.previous();
            let right = self.equality()?;
            expr = Expression::Infix(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?;

        while self.expect(&Token::DoubleEqual) || self.expect(&Token::BangEqual) {
            self.advance();
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expression::Infix(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;

        while self.expect(&Token::Greater) || self.expect(&Token::GreaterEqual) || self.expect(&Token::Less) || self.expect(&Token::LessEqual) {
            self.advance();
            let op = self.previous();
            let right = self.term()?;
            expr = Expression::Infix(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    // plus and minus 
    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;

        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.factor()?;
                    expr = Expression::Infix(Box::new(expr), Token::Plus, Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.factor()?;
                    expr = Expression::Infix(Box::new(expr), Token::Minus, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    // times and divide
    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.prefix()?;

        loop {
            match self.peek() {
                Token::Times => {
                    self.advance();
                    let right = self.prefix()?;
                    expr = Expression::Infix(Box::new(expr), Token::Times, Box::new(right));
                }
                Token::Divide => {
                    self.advance();
                    let right = self.prefix()?;
                    expr = Expression::Infix(Box::new(expr), Token::Divide, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn prefix(&mut self) -> Result<Expression> {
        if self.expect(&Token::Minus) || self.expect(&Token::Bang) {
            self.advance();
            let op = self.previous();
            let right = self.atom()?;
            Ok(Expression::Prefix(op, Box::new(right)))
        } else {
            self.atom()
        }
    }

    fn atom(&mut self) -> Result<Expression> {
        match self.peek() {
            Token::Integer(_) => {
                match self.advance() {
                    Token::Integer(i) => Ok(Expression::Integer(i)),
                    _ => unreachable!(),
                }
            }
            Token::Float(_) => {
                match self.advance() {
                    Token::Float(f) => Ok(Expression::Float(f)),
                    _ => unreachable!(),
                }
            }
            Token::String(_) => {
                match self.advance() {
                    Token::String(s) => Ok(Expression::String(s)),
                    _ => unreachable!(),
                }
            }
            Token::Bool(_) => {
                match self.advance() {
                    Token::Bool(b) => Ok(Expression::Boolean(b)),
                    _ => unreachable!(),
                }
            }
            Token::Identifier(_) => {
                match self.advance() {
                    Token::Identifier(i) => Ok(Expression::Identifier(i)),
                    _ => unreachable!(),
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(&Token::RightParen, "Expected ')' after expression.".to_string());
                Ok(expr)
            }
            _ => Err(anyhow::anyhow!("Expected expression")),
        }
    }

    fn consume(&mut self, t: &Token, message: String) -> Token {
        if self.expect(t) {
           self.advance()
        } else {
            panic!("{}", message);
        }
    }

    // check the next token
    fn expect(&mut self, t: &Token) -> bool {
        if self.done() {
            return false;
        }
        if &self.peek() == t {
            return true;
        }
        false
    }

    fn expect_identifier(&mut self) -> Result<String> {
        if self.done() {
            return Err(anyhow::anyhow!("Expected identifier"));
        }
        match self.peek() {
            Token::Identifier(s) => Ok(s),
            _ => Err(anyhow::anyhow!("Expected identifier")),
        }
    }

    // advances the parser by 1 token
    fn advance(&mut self) -> Token {
        if !self.done() {
            self.pos += 1;
        }
        self.previous()
    }

    // is the parser done?
    fn done(&self) -> bool {
        self.peek() == Token::Eof
    }

    // get the next token early
   fn peek(&self) -> Token {
        self.tokens[self.pos].clone()
    }

   // last token
   fn previous(&self) -> Token {
        self.tokens[self.pos - 1].clone()
   }
}
