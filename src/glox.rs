use std::{io, fs, process};

use crate::token::Token;
use crate::scanner::Scanner;
pub struct Glox {
    had_error: bool,
    scanner: Scanner
}

impl Glox {
    pub fn new_from_file(filepath: &String) -> Self {
        let code = fs::read_to_string(filepath).expect("failed to read file: {filepath}");
        Glox { 
            had_error: false,
            scanner: Scanner::new(code)
        }
    }

    pub fn new_for_prompt() -> Self {
        Glox { 
            had_error: false,
            scanner: Scanner::new(String::new())
        }
    }

    pub fn run_file(&mut self, filepath: &String) {
        println!("Running file: {filepath}");
        
        self.run();
        if self.had_error {
            process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("$$$");
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("failed to read line");
            if line.is_empty() {
                break;
            }
            self.run();
            // Reset the error flag to allow users to keep entering commands
            self.had_error = false;
        }
    }

    fn run(&mut self) {
        let tokens: Vec<Token> = self.scanner.scan_tokens();
        for token in tokens {
            println!("{:#?}", token)
        }
    }

    fn error(&mut self, line: i32, msg: String) {
        Self::report(self, line, "".to_string(), msg);
    }

    fn report(&mut self, line: i32, loc: String, msg: String) {
        self.had_error = true;
        eprintln!("[line {line}] Error {loc}: {msg}")
    }
}

