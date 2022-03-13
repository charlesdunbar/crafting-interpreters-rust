use crate::token_type::TokenType;
use std::fmt;

#[allow(dead_code)]
struct Token<T> {
    l_type: TokenType,
    lexeme: String,
    literal: T,
    line: u64,
}

#[allow(dead_code)]
impl<T> Token<T> {
    pub fn new(l_type: TokenType, lexeme: String, literal: T, line: u64) -> Self {
        Token {
            l_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl<T> fmt::Display for Token<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.l_type, self.lexeme, self.literal)
    }
}
