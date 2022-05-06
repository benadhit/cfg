use std::env::args;
use std::io::{self, stdout, BufRead, Write};

mod scanner;
use scanner::*;
mod token;
mod token_type;

mod error;
use error::*;

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Can't Run File"),
        _ => {
            println!("Usage:cfg [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    if run(buf).is_err() {
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    println!("Shell built by tinyend 2022");
    print!("> ");
    stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line);
        } else {
            break;
        }
        print!("> ");
        stdout().flush().unwrap();
    }
}

fn run(source: String) -> Result<(), CfgError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
