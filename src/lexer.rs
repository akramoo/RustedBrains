use crate::ast::Token;
use crate::error::{TranspilerError, TranspilerResult};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.next();

        Self {
            input: chars,
            position: 0,
            current_char,
        }
    }

    pub fn tokenize(&mut self) -> TranspilerResult<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            if matches!(token, Token::Eof) {
                break;
            }
            tokens.push(token);
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn next_token(&mut self) -> TranspilerResult<Option<Token>> {
        self.skip_whitespace();

        match self.current_char {
            None => Ok(Some(Token::Eof)),
            Some(ch) => {
                let token = match ch {
                    '=' => self.handle_equals(),
                    '!' => self.handle_exclamation(),
                    '+' => {
                        self.advance();
                        Token::Plus
                    }
                    '-' => {
                        self.advance();
                        Token::Minus
                    }
                    '<' => {
                        self.advance();
                        Token::Less
                    }
                    '>' => {
                        self.advance();
                        Token::Greater
                    }
                    ';' => {
                        self.advance();
                        Token::Semicolon
                    }
                    '{' => {
                        self.advance();
                        Token::LeftBrace
                    }
                    '}' => {
                        self.advance();
                        Token::RightBrace
                    }
                    '(' => {
                        self.advance();
                        Token::LeftParen
                    }
                    ')' => {
                        self.advance();
                        Token::RightParen
                    }
                    c if c.is_ascii_digit() => self.read_number()?,
                    c if c.is_alphabetic() || c == '_' => self.read_identifier(),
                    c => {
                        return Err(TranspilerError::with_position(
                            format!("Unexpected character: '{}'", c),
                            self.position,
                        ));
                    }
                };
                Ok(Some(token))
            }
        }
    }

    fn handle_equals(&mut self) -> Token {
        self.advance();
        if self.current_char == Some('=') {
            self.advance();
            Token::Equal
        } else {
            Token::Assign
        }
    }

    fn handle_exclamation(&mut self) -> Token {
        self.advance();
        if self.current_char == Some('=') {
            self.advance();
            Token::NotEqual
        } else {
            Token::Exclamation
        }
    }

    fn read_number(&mut self) -> TranspilerResult<Token> {
        let mut number = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        number.parse::<i32>().map(Token::Number).map_err(|_| {
            TranspilerError::with_position(format!("Invalid number: {}", number), self.position)
        })
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "let" => Token::Let,
            "mut" => Token::Mut,
            "print" => Token::Print,
            "if" => Token::If,
            "while" => Token::While,
            _ => Token::Identifier(identifier),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
        self.position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(42),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new("== != < >");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Equal,
                Token::NotEqual,
                Token::Less,
                Token::Greater,
                Token::Eof,
            ]
        );
    }
}
