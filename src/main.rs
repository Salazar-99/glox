use std::env;
mod glox;
use glox::Glox;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: glox program.lox")
    } else if args.len() == 2 {
        let filepath: &String = &args[1];
        Glox::run_file(filepath);
    } else {
        Glox::run_prompt();
    }
}
