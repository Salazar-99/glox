
pub struct Glox;

impl Glox {
    pub fn run_file(filepath: &String) {
        println!("Running file: {filepath}")
    }

    pub fn run_prompt() {
        println!("Starting REPL!")
    }

    fn run(code: &String) {
        println!("Running code: {code}")
    }
}

