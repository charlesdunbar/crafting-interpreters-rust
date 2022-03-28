use crate::token_type::TokenType;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ValidTokens {
    Float(f64),
    String(String),
}

impl fmt::Display for ValidTokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
        write!(
            f,
            "{:?} {:?} {}",
            self.l_type,
            self.lexeme,
            match self.literal.as_ref() {
                Some(a) => {
                    a.to_string()
                }
                None => {
                    "None".to_string()
                }
            }
        )
    }
}
