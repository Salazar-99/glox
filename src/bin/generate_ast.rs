use std::env;

const SYNTAX_GRAMMAR: [&str; 4] = [
    "Binary : Expr left, Token operator, Expr right",
    "Grouping : Expr expression",
    "Literal : Literal value", // TODO: Will this work? Map TokenType::Literal to our Literal enum?
    "Unary : Token operator, Expr right"
];

fn generate_ast(output_dir: &String) {
    // TODO: Implement this?
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 || args.len() < 2 {
        panic!("Usage: generate_ast <output directory>")
    } else {
        let output_dir: &String = &args[1];
        generate_ast(output_dir);
    } 
}