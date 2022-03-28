use std::collections::HashMap;

use crate::lox::Lox;
use crate::token::Token;
use crate::token::ValidTokens;
use crate::token_type::TokenType::{self, *};
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                (String::from("and"), AND),
                (String::from("class"), CLASS),
                (String::from("else"), ELSE),
                (String::from("false"), FALSE),
                (String::from("for"), FOR),
                (String::from("fun"), FUN),
                (String::from("if"), IF),
                (String::from("nil"), NIL),
                (String::from("or"), OR),
                (String::from("print"), PRINT),
                (String::from("return"), RETURN),
                (String::from("super"), SUPER),
                (String::from("this"), THIS),
                (String::from("true"), TRUE),
                (String::from("var"), VAR),
                (String::from("while"), WHILE),
            ]),
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
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.multiline(l);
                } else {
                    self.add_token(SLASH, None)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(l),
            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    l.error(self.line, "Unexpected character.")
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let result = self.keywords.get(&text).cloned();
        let l_type = match result {
            Some(keyword) => keyword,
            None => IDENTIFIER,
        };
        self.add_token(l_type, None)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            NUMBER,
            Some(Box::new(ValidTokens::Float(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            ))),
        )
    }

    fn string(&mut self, l: &Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            l.error(self.line, "Unterminated string.")
        }

        self.advance(); // The closing ".

        // Trim the surrounding quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(STRING, Some(Box::new(ValidTokens::String(value))));
    }

    fn multiline(&mut self, l: &Lox) {
        while !(self.is_at_end() || self.peek() == '*' && self.peek_next() == '/') {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            l.error(self.line, "Unterminated multiline comment.")
        }

        self.advance(); // The closing *.
        self.advance(); // The closing /.
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, l_type: TokenType, literal: Option<Box<ValidTokens>>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(l_type, text.to_string(), literal, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
}
