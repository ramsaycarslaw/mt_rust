#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // 1 character tokens
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Times,
    Divide,
    Dot,
    SemiColon,
    Quote,
    Or,
    And,
    Xor,
    Bang,
    Equal,
    DoubleEqual,
    BangEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Comma,

    // invisible tokens
    WhiteSpace,
    NewLine,
    Eof,

    // literals
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),

    // keywords
    Print,

    // illegal
    Illegal(char, usize, usize),
}
