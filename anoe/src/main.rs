use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod codegen;

fn main() {
    // Check if filename is passed
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: anoc <source_file.anoe>");
        process::exit(1);
    }

    let filename = &args[1];
    let source_code = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename));

    // Step 1: Lexical analysis (tokenization)
    let tokens = lexer::lex(&source_code);

    // Step 2: Parse the tokens into an abstract syntax tree
    let ast = parser::parse(tokens);

    // Step 3: Generate code from AST (e.g., machine code or intermediate representation)
    codegen::generate(&ast);

    println!("Compilation complete.");
}
