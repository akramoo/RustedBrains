# RustedBrains: Rust to Brainfuck Transpiler

A sophisticated transpiler that converts a subset of Rust syntax into optimized Brainfuck code. This project demonstrates compiler design principles including lexical analysis, parsing, abstract syntax tree construction, and code generation.

## 🚀 Features

- **Lexical Analysis**: Comprehensive tokenization with error reporting
- **Recursive Descent Parser**: Handles expressions, statements, and control flow
- **AST Generation**: Clean abstract syntax tree representation
- **Optimized Code Generation**: Efficient Brainfuck output with memory management
- **Error Handling**: Detailed error messages with position information
- **Memory Management**: Smart variable allocation and temporary cell management

## 📋 Supported Rust Subset

### Variable Declarations
```rust
let x = 42;          // Immutable variable
let mut y = 10;      // Mutable variable
```

### Assignments
```rust
x = 20;              // Variable assignment
y = x + 5;           // Expression assignment
```

### Arithmetic Operations
```rust
let sum = a + b;     // Addition
let diff = a - b;    // Subtraction
```

### Comparison Operations
```rust
if x == y { ... }    // Equality
if x != y { ... }    // Inequality  
if x < y { ... }     // Less than
if x > y { ... }     // Greater than
```

### Control Flow
```rust
// If statements
if condition {
    // body
}

// While loops
while condition {
    // body
}
```

### Output
```rust
print(variable);     // Print variable value
print(42);          // Print literal value
```

## 🏗️ Architecture

The transpiler is organized into several well-defined modules:

### Core Components

1. **Lexer** (`src/lexer.rs`)
   - Tokenizes source code into meaningful tokens
   - Handles keywords, operators, identifiers, and numbers
   - Provides detailed error reporting with position tracking

2. **Parser** (`src/parser.rs`)  
   - Implements recursive descent parsing
   - Builds abstract syntax tree (AST)
   - Handles operator precedence and associativity
   - Comprehensive syntax error reporting

3. **AST** (`src/ast.rs`)
   - Defines data structures for program representation
   - Implements visitor pattern for tree traversal
   - Provides builder methods for clean construction

4. **Code Generator** (`src/codegen.rs`)
   - Converts AST to optimized Brainfuck code
   - Manages memory allocation and variable mapping
   - Implements arithmetic and comparison operations
   - Optimizes for minimal code size and execution time

5. **Error Handling** (`src/error.rs`)
   - Centralized error management
   - Position-aware error reporting
   - Comprehensive error types

### Project Structure

```
rust2bf/
├── src/
│   ├── main.rs           # CLI interface and orchestration
│   ├── lexer.rs          # Lexical analysis
│   ├── parser.rs         # Syntax analysis  
│   ├── ast.rs            # AST definitions
│   ├── codegen.rs        # Code generation
│   └── error.rs          # Error handling
├── examples/
│   ├── simple.rs         # Basic variable operations
│   ├── control_flow.rs   # If statements and loops
│   └── arithmetic.rs     # Arithmetic and comparisons
├── tests/
│   └── integration_tests.rs  # End-to-end tests
├── Cargo.toml           # Project configuration
└── README.md            # This file
```

## 🛠️ Installation

### Prerequisites
- Rust 1.70.0 or later
- Cargo (comes with Rust)

### Building from Source
```bash
git clone https://github.com/yourusername/rust2bf.git
cd rust2bf
cargo build --release
```

The binary will be available at `target/release/rust2bf`

## 📖 Usage

### Command Line Interface
```bash
# Transpile a Rust file to Brainfuck
rust2bf input.rs

# The output will be saved as input.rs.bf
```

### Example Session
```bash
$ rust2bf examples/simple.rs

=== Source Code ===
let x = 10;
let mut y = 5;
print(x);
y = x + y;
print(y);

=== Tokens ===
[Let, Identifier("x"), Assign, Number(10), Semicolon, ...]

=== AST ===
[
    Let {
        name: "x",
        mutable: false,
        value: Number(10),
    },
    ...
]

=== Generated Brainfuck ===
++++++++++>+++++>[-]<<[>>+>+<<<-]>>>[<<<+>>>-]<<.>[-]<[>+>+<<-]>[<+>-]>>[<<+>>-]<<.

Brainfuck code saved to: examples/simple.rs.bf
```

## 🧠 Brainfuck Output Details

The transpiler generates optimized Brainfuck code with the following characteristics:

### Memory Layout
- **Variables**: Allocated sequentially starting from cell 0
- **Temporary Values**: Allocated starting from cell 100
- **Expression Results**: Use temporary cells for intermediate calculations

### Optimizations
- **Loop-based Initialization**: For values ≥ 10, uses loops instead of repeated `+`
- **Value Copying**: Preserves original values during operations
- **Memory Reuse**: Efficient temporary cell allocation
- **Minimal Movement**: Optimized pointer movement between cells

### Example Mappings

| Rust Code | Brainfuck Concept |
|-----------|-------------------|
| `let x = 10;` | Initialize cell 0 with value 10 |
| `print(x);` | Move to cell 0 and output |
| `x + y` | Copy values, perform addition |
| `if condition { ... }` | Use loop `[...]` with condition |
| `while condition { ... }` | Nested loops with condition re-evaluation |

## 🧪 Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration_tests
```

### Running Examples
```bash
# Test all examples
for example in examples/*.rs; do
    cargo run "$example"
done
```

## 🔍 Debugging and Development

### Debug Mode
For development and debugging, the transpiler provides verbose output showing each compilation stage.

### Adding New Features
1. **Extend AST**: Add new node types in `src/ast.rs`
2. **Update Lexer**: Add new tokens in `src/lexer.rs`
3. **Enhance Parser**: Add parsing logic in `src/parser.rs`
4. **Implement Codegen**: Add generation logic in `src/codegen.rs`

### Memory Management
The code generator uses a sophisticated memory management system:
- Variables are allocated fixed addresses
- Temporary calculations use higher memory addresses
- Copy operations preserve original values
- Efficient pointer movement minimizes code size

## ⚠️ Limitations

1. **Limited Type System**: Only supports integers
2. **No Functions**: Function definitions and calls not supported
3. **Simple I/O**: Only supports single character output
4. **Memory Bounds**: No bounds checking (inherent Brainfuck limitation)
5. **Complex Expressions**: Some complex mathematical expressions may not optimize perfectly

## 🛣️ Future Enhancements

- **Input Support**: Add support for reading input
- **String Literals**: Support for string output
- **Function Calls**: Basic function support
- **Optimization Passes**: Advanced code optimization
- **Better Error Recovery**: More robust error handling
- **IDE Integration**: Language server protocol support

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for new functionality
4. Ensure all tests pass (`cargo test`)
5. Format code (`cargo fmt`)
6. Run clippy (`cargo clippy`)
7. Commit changes (`git commit -m 'Add amazing feature'`)
8. Push to branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Development Setup
```bash
git clone https://github.com/yourusername/rust2bf.git
cd rust2bf
cargo build
cargo test
```

## 📚 References and Learning Resources

- [Brainfuck Language Specification](https://esolangs.org/wiki/Brainfuck)
- [Crafting Interpreters](https://craftinginterpreters.com/) - Excellent resource for compiler design
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - Official Rust documentation
- [Recursive Descent Parsing](https://en.wikipedia.org/wiki/Recursive_descent_parser)

## 📄 License

This project is dual-licensed under the MIT and Apache 2.0 licenses. See the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for details.

## 🏆 Acknowledgments

- The Rust community for excellent tooling and documentation
- The esoteric programming language community for Brainfuck
- Contributors to open-source compiler and transpiler projects

---

**Happy Transpiling!** 🦀➡️🧠
