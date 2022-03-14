mod scanner;
mod token;
mod token_type;

use token::Token;

use crate::scanner::Scanner;
use std::{env, fs, io, io::Write};

fn main() {
    let mut lox = Lox { had_error: false };
    let args: Vec<String> = env::args().collect();

    //print!("{}", args.len());
    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_file(&self, cmd: &str) {
        let source = fs::read_to_string(cmd).expect("Got an error reading file");
        self.run(&source).expect("Error!");
        // Exit on errors
        if self.had_error {
            std::process::exit(65);
        }
    }

    fn run_prompt(&mut self) {
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
        let tokens: &Vec<Token> = scanner.scan_tokens();
        // for token in tokens, println token
        for token in tokens {
            println!("{}", token)
        }
        Ok(true)
    }
}
