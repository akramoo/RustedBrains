use crate::ast::{BinaryOp, Expr, Program, Stmt, Token};
use crate::error::{TranspilerError, TranspilerResult};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> TranspilerResult<Program> {
        let mut statements = Vec::new();

        while !self.is_at_end() && !matches!(self.peek(), Token::Eof) {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> TranspilerResult<Stmt> {
        match self.peek() {
            Token::Let => self.let_statement(),
            Token::Print => self.print_statement(),
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::Identifier(_) => self.assignment_statement(),
            _ => Err(TranspilerError::with_position(
                format!("Unexpected token: {:?}", self.peek()),
                self.current,
            )),
        }
    }

    fn let_statement(&mut self) -> TranspilerResult<Stmt> {
        self.consume(Token::Let, "Expected 'let'")?;

        let mutable = if matches!(self.peek(), Token::Mut) {
            self.advance();
            true
        } else {
            false
        };

        let name = self.consume_identifier("Expected variable name")?;
        self.consume(Token::Assign, "Expected '=' after variable name")?;
        let value = self.expression()?;
        self.consume_if_present(Token::Semicolon);

        Ok(Stmt::let_stmt(name, mutable, value))
    }

    fn assignment_statement(&mut self) -> TranspilerResult<Stmt> {
        let name = self.consume_identifier("Expected variable name")?;
        self.consume(Token::Assign, "Expected '=' in assignment")?;
        let value = self.expression()?;
        self.consume_if_present(Token::Semicolon);

        Ok(Stmt::assign(name, value))
    }

    fn print_statement(&mut self) -> TranspilerResult<Stmt> {
        self.consume(Token::Print, "Expected 'print'")?;
        self.consume(Token::LeftParen, "Expected '(' after 'print'")?;
        let expr = self.expression()?;
        self.consume(Token::RightParen, "Expected ')' after expression")?;
        self.consume_if_present(Token::Semicolon);

        Ok(Stmt::print(expr))
    }

    fn if_statement(&mut self) -> TranspilerResult<Stmt> {
        self.consume(Token::If, "Expected 'if'")?;
        let condition = self.expression()?;
        let body = self.block()?;

        Ok(Stmt::if_stmt(condition, body))
    }

    fn while_statement(&mut self) -> TranspilerResult<Stmt> {
        self.consume(Token::While, "Expected 'while'")?;
        let condition = self.expression()?;
        let body = self.block()?;

        Ok(Stmt::while_stmt(condition, body))
    }

    fn block(&mut self) -> TranspilerResult<Vec<Stmt>> {
        self.consume(Token::LeftBrace, "Expected '{'")?;

        let mut statements = Vec::new();
        while !matches!(self.peek(), Token::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}'")?;
        Ok(statements)
    }

    fn expression(&mut self) -> TranspilerResult<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> TranspilerResult<Expr> {
        let mut expr = self.comparison()?;

        while matches!(self.peek(), Token::Equal | Token::NotEqual) {
            let op = match self.advance() {
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::binary(expr, op, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> TranspilerResult<Expr> {
        let mut expr = self.term()?;

        while matches!(self.peek(), Token::Less | Token::Greater) {
            let op = match self.advance() {
                Token::Less => BinaryOp::Less,
                Token::Greater => BinaryOp::Greater,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expr::binary(expr, op, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> TranspilerResult<Expr> {
        let mut expr = self.factor()?;

        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::binary(expr, op, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> TranspilerResult<Expr> {
        let mut expr = self.primary()?;

        while matches!(self.peek(), Token::Multiply | Token::Divide) {
            let op = match self.advance() {
                Token::Multiply => BinaryOp::Mul,
                Token::Divide => BinaryOp::Div,
                _ => unreachable!(),
            };
            let right = self.primary()?;
            expr = Expr::binary(expr, op, right);
        }

        Ok(expr)
    }

    fn primary(&mut self) -> TranspilerResult<Expr> {
        match self.peek() {
            Token::Number(n) => {
                let num = *n;
                self.advance();
                Ok(Expr::number(num))
            }
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance();
                Ok(Expr::variable(var_name))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(Token::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(TranspilerError::with_position(
                format!("Unexpected token in expression: {:?}", self.peek()),
                self.current,
            )),
        }
    }

    // Helper methods
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1).unwrap_or(&Token::Eof)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn consume(&mut self, expected: Token, message: &str) -> TranspilerResult<()> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(TranspilerError::with_position(
                format!("{}, got {:?}", message, self.peek()),
                self.current,
            ))
        }
    }

    fn consume_if_present(&mut self, token: Token) -> bool {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume_identifier(&mut self, message: &str) -> TranspilerResult<String> {
        match self.peek() {
            Token::Identifier(name) => {
                let result = name.clone();
                self.advance();
                Ok(result)
            }
            _ => Err(TranspilerError::with_position(
                message.to_string(),
                self.current,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_let_statement() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        matches!(ast[0], Stmt::Let { .. });
    }

    #[test]
    fn test_parse_expression() {
        let mut lexer = Lexer::new("let x = 1 + 2 * 3;");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        // Should parse as: 1 + (2 * 3) due to operator precedence
        if let Stmt::Let { value, .. } = &ast[0] {
            matches!(value, Expr::Binary { .. });
        }
    }
}
