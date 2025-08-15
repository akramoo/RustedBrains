use std::env;
use std::fs;

fn main() {
    // Step 1: Read the file from args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rust2bf <filename>");
        return;
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Could not read file");

    println!("Source:\n{}", contents);

    // Step 2: Tokenize
    let tokens = tokenize(&contents);
    println!("Tokens: {:?}", tokens);
}

/// Splits source code into tokens (words, numbers, symbols)
fn tokenize(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in source.chars() {
        if ch.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else if is_symbol(ch) {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(ch.to_string());
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

/// Checks if a character is a special symbol in our toy Rust
fn is_symbol(ch: char) -> bool {
    matches!(
        ch,
        '=' | ';' | '{' | '}' | '(' | ')' | '+' | '-' | '!' | '<' | '>'
    )
}
