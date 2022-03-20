use std::{fs, io, io::Write};

use crate::scanner::Scanner;
use crate::token::Token;

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn run_file(&self, cmd: &str) {
        let source = fs::read_to_string(cmd).expect("Got an error reading file");
        self.run(&source).expect("Error!");
        // Exit on errors
        if self.had_error {
            std::process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();

            let bytes = io::stdin()
                .read_line(&mut input)
                .expect("Failed to parse input!");

            // Catch Ctrl-D
            if bytes == 0 {
                break;
            }
            print!("{}", input);
            self.run(&input).unwrap();
            self.had_error = false;
        }
    }

    fn run(&self, source: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut scanner = Scanner::new(source.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens(self);
        // for token in tokens, println token
        for token in tokens {
            println!("{}", token)
        }
        Ok(true)
    }

    pub fn error(&self, line: usize, message: &str) {
        self.report(line, "", message)
    }

    fn report(&self, line: usize, l_where: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, l_where, message)
    }
}
