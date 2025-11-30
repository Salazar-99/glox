use std::{fs, io, process};

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;
pub struct Glox {
    had_error: bool,
    scanner: Scanner,
    interpreter: Interpreter,
}

impl Glox {
    pub fn new_from_file(filepath: &String) -> Self {
        let code = fs::read_to_string(filepath).expect("failed to read file: {filepath}");
        Glox {
            had_error: false,
            scanner: Scanner::new(code),
            interpreter: Interpreter::new(),
        }
    }

    pub fn new_for_prompt() -> Self {
        Glox {
            had_error: false,
            scanner: Scanner::new(String::new()),
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, filepath: &String) {
        println!("{}", format!("Running file: {}", filepath));

        self.run();
        if self.had_error {
            process::exit(65);
        }
    }

    // TODO: Fix this prompt, it's not printing and is not passing the line to the scanner.
    pub fn run_prompt(&mut self) {
        loop {
            print!("$$$");
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("failed to read line");
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
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(expr) => match self.interpreter.interpret(&expr) {
                Ok(result) => println!("{:?}", result),
                Err(e) => {
                    println!("Error: {}", e)
                }
            },
            Err(e) => {
                self.had_error = true;
                eprintln!("{}", e)
            }
        }
    }
}
