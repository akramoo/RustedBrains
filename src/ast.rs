#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Identifier(String),
    Number(i32),

    // Keywords
    Let,
    Mut,
    Print,
    If,
    While,

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Equal,    // ==
    NotEqual, // !=
    Less,     // <
    Greater,  // >

    // Delimiters
    Semicolon,   // ;
    LeftBrace,   // {
    RightBrace,  // }
    LeftParen,   // (
    RightParen,  // )
    Exclamation, // !

    // Special
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    Greater,
}

// Rest of the file remains the same...
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Print(Expr),
    If {
        condition: Expr,
        body: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}

pub type Program = Vec<Stmt>;

// Utility trait for AST traversal
pub trait Visitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
}

// Helper functions for AST construction
impl Expr {
    pub fn number(value: i32) -> Self {
        Expr::Number(value)
    }

    pub fn variable(name: impl Into<String>) -> Self {
        Expr::Variable(name.into())
    }

    pub fn binary(left: Expr, op: BinaryOp, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        }
    }
}

impl Stmt {
    pub fn let_stmt(name: impl Into<String>, mutable: bool, value: Expr) -> Self {
        Stmt::Let {
            name: name.into(),
            mutable,
            value,
        }
    }

    pub fn assign(name: impl Into<String>, value: Expr) -> Self {
        Stmt::Assign {
            name: name.into(),
            value,
        }
    }

    pub fn print(expr: Expr) -> Self {
        Stmt::Print(expr)
    }

    pub fn if_stmt(condition: Expr, body: Vec<Stmt>) -> Self {
        Stmt::If { condition, body }
    }

    pub fn while_stmt(condition: Expr, body: Vec<Stmt>) -> Self {
        Stmt::While { condition, body }
    }
}
