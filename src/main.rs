mod ast;
mod codegen;
mod error;
mod lexer;
mod parser;

use codegen::BrainfuckGenerator;
use error::TranspilerResult;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> TranspilerResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: rust2bf <filename>\nExample: rust2bf example.rs".into());
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .map_err(|e| format!("Could not read file '{}': {}", filename, e))?;

    println!("=== Source Code ===");
    println!("{}\n", contents);

    // Lexical analysis
    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.tokenize()?;
    println!("=== Tokens ===");
    println!("{:?}\n", tokens);

    // Syntax analysis
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    println!("=== AST ===");
    println!("{:#?}\n", ast);

    // Code generation
    let mut generator = BrainfuckGenerator::new();
    let brainfuck_code = generator.generate(&ast)?;

    println!("=== Generated Brainfuck ===");
    println!("{}\n", brainfuck_code);

    // Save output
    let output_filename = format!("{}.bf", filename.trim_end_matches(".rs"));
    fs::write(&output_filename, &brainfuck_code)
        .map_err(|e| format!("Could not write to '{}': {}", output_filename, e))?;

    println!("Brainfuck code saved to: {}", output_filename);
    Ok(())
}
