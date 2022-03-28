use crate::token_type::TokenType;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ValidTokens {
    Float(f64),
    String(String),
}

pub struct Token {
    l_type: TokenType,
    lexeme: String,
    literal: Option<Box<ValidTokens>>,
    line: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(
        l_type: TokenType,
        lexeme: String,
        literal: Option<Box<ValidTokens>>,
        line: usize,
    ) -> Self {
        Self {
            l_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let lit = match self.literal {
        //     Some => self.literal.unwrap(),
        //     None => None
        // };
        write!(f, "{:?} {:?} {:#?}", self.l_type, self.lexeme, self.literal)
    }
}
