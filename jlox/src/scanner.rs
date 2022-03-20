use std::any::Any;

use crate::lox::Lox;
use crate::token::Token;
use crate::token_type::TokenType::{self, *};
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

    pub fn scan_tokens(&mut self, l: &Lox) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            //println!("About to scan token. Source is {:?}, Start is {}, Current is {}, Line is {}", self.source, self.start, self.current, self.line);
            self.scan_token(l);
        }
        self.tokens
            .push(Token::new(EOF, "".to_string(), None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self, l: &Lox) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LEFT_PAREN, None),
            ')' => self.add_token(RIGHT_PAREN, None),
            '{' => self.add_token(LEFT_BRACE, None),
            '}' => self.add_token(RIGHT_BRACE, None),
            ',' => self.add_token(COMMA, None),
            '.' => self.add_token(DOT, None),
            '-' => self.add_token(MINUS, None),
            '+' => self.add_token(PLUS, None),
            ';' => self.add_token(SEMICOLON, None),
            '*' => self.add_token(STAR, None),
            '!' => {
                if self.match_char('=') {
                    self.add_token(BANG_EQUAL, None)
                } else {
                    self.add_token(BANG, None)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(EQUAL_EQUAL, None)
                } else {
                    self.add_token(EQUAL, None)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(LESS_EQUAL, None)
                } else {
                    self.add_token(LESS, None)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(GREATER_EQUAL, None)
                } else {
                    self.add_token(GREATER, None)
                }
            }
            '\n' => self.line += 1,
            _ => l.error(self.line, "Unexpected character."),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, l_type: TokenType, literal: Option<Box<dyn Any>>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(l_type, text.to_string(), literal, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }
}
