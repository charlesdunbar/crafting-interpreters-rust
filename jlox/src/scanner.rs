use crate::token::Token;
use crate::token_type::TokenType::*;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(EOF, "".to_string(), None, self.line));
        &self.tokens
    }

    fn scan_token(&self) {}

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
