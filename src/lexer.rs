use crate::tokens;

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    // constructor
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            line: 1,
            read_position: 0,
            ch: '\0',
        };
        
        l.read_char();
        l
    }

    // read a character from the input string and advance the position
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn back_char(&mut self) {
        self.read_position -= 1;
        self.position = self.read_position;
        self.ch = self.input.chars().nth(self.position).unwrap();
    }

    // read a number and return it
    fn read_number(&mut self) -> tokens::Token {
        // read while digit or decimal point, if decimal point, read until digit
        // then return float
        let mut s = String::new();
        while self.ch.is_digit(10) {
            s.push(self.ch);
            self.read_char();
        }
        if self.ch == '.' {
            s.push(self.ch);
            self.read_char();
            while self.ch.is_digit(10) {
                s.push(self.ch);
                self.read_char();
            }
            self.back_char();
            tokens::Token::Float(s.parse().unwrap())
        } else {
            self.back_char();
            tokens::Token::Integer(s.parse().unwrap())
        }
    }

    fn read_string(&mut self) -> tokens::Token {
        let mut s = String::new();
        self.read_char();
        while self.ch != '"' {
            s.push(self.ch);
            self.read_char();
        }
        tokens::Token::String(s)
    }

    fn match_token(&mut self) -> tokens::Token {
        match self.ch {
            // single character tokens
            '(' => tokens::Token::LeftParen,
            ')' => tokens::Token::RightParen,
            '+' => tokens::Token::Plus,
            '-' => tokens::Token::Minus,
            '*' => tokens::Token::Times,
            '/' => tokens::Token::Divide,
            '.' => tokens::Token::Dot,
            ';' => tokens::Token::SemiColon,
            '\'' => tokens::Token::Quote,
            '"' => self.read_string(),

            // whitespace
            ' ' => tokens::Token::WhiteSpace,
            '\t' => tokens::Token::WhiteSpace,
            '\r' => tokens::Token::WhiteSpace,

            // numbers
            '0'..='9' => self.read_number(),

            // line
            '\n' => {
                self.line += 1;
                tokens::Token::NewLine
            }

            '\0' => tokens::Token::Eof,

            // illegal
            _ => tokens::Token::Illegal(self.ch, self.position, self.line),
        }
    }

    fn strip_whitespace(&mut self, orig_tokens: Vec<tokens::Token>) -> Vec<tokens::Token> {
        let mut tokens = Vec::new();
        for token in orig_tokens {
            match token {
                tokens::Token::WhiteSpace => (),
                tokens::Token::NewLine => (),
                _ => tokens.push(token),
            }
        }
        tokens
    }

    // lexes the input string
    pub fn lex(&mut self, debug: bool) -> Vec<tokens::Token> {
        let mut tokens = Vec::new();
        while self.ch != '\0' {
            tokens.push(self.match_token());
            self.read_char();
        }
        tokens.push(self.match_token());

        if debug {
            println!("Tokens: {:?}", self.strip_whitespace(tokens.clone()));
        }
        self.strip_whitespace(tokens)
    }
}
