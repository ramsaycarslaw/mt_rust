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
    DoubleQuote,

    // invisible tokens
    WhiteSpace,
    NewLine,
    Eof,

    // literals
    Integer(i64),
    Float(f64),
    String(String),

    // illegal
    Illegal(char, usize, usize),
}
