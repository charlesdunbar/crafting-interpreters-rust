use crate::token_type::TokenType;
use std::any::Any;
use std::fmt;

#[allow(dead_code)]
pub struct Token {
    l_type: TokenType,
    lexeme: String,
    literal: Box<dyn Any>,
    line: u64,
}

#[allow(dead_code)]
impl Token {
    pub fn new(l_type: TokenType, lexeme: String, literal: dyn Any, line: u64) -> Self {
        Token {
            l_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.l_type, self.lexeme, self.literal)
    }
}
