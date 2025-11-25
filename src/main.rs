use std::env;
use glox::Glox;

mod glox;
mod token;
mod scanner;
mod expr;
mod parser;
mod error;
mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: glox program.lox")
    } else if args.len() == 2 {
        let filepath: &String = &args[1];
        let mut glox = Glox::new_from_file(filepath);
        glox.run_file(filepath);
    } else {
        let mut glox = Glox::new_for_prompt();
        glox.run_prompt();
    }
}
