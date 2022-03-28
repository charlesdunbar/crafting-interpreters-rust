mod lox;
mod scanner;
mod token;
mod token_type;

use crate::lox::Lox;
use std::env;

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
