use std::{env, fs, io, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    //print!("{}", args.len());
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(cmd: &str) {
    let source = fs::read_to_string(cmd).expect("Got an error reading file");
    run(&source);
}

fn run_prompt() {
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
    }
}

fn run(source: &str) {}
