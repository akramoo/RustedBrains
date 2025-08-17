use crate::ast::{BinaryOp, Expr, Program, Stmt, Visitor};
use crate::error::{TranspilerError, TranspilerResult};
use std::collections::HashMap;

pub struct BrainfuckGenerator {
    variables: HashMap<String, usize>,
    memory_ptr: usize,
    output: String,
    next_temp_addr: usize,
}

impl BrainfuckGenerator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            memory_ptr: 0,
            output: String::new(),
            next_temp_addr: 100, // Start temp variables at cell 100
        }
    }

    pub fn generate(&mut self, program: &Program) -> TranspilerResult<String> {
        self.visit_program(program);
        Ok(self.output.clone())
    }

    fn allocate_variable(&mut self, name: &str) -> usize {
        let addr = self.memory_ptr;
        self.variables.insert(name.to_string(), addr);
        self.memory_ptr += 1;
        addr
    }

    fn get_temp_addr(&mut self) -> usize {
        let addr = self.next_temp_addr;
        self.next_temp_addr += 1;
        addr
    }

    fn move_to(&mut self, target: usize) {
        if target > self.memory_ptr {
            self.output.push_str(&">".repeat(target - self.memory_ptr));
        } else if target < self.memory_ptr {
            self.output.push_str(&"<".repeat(self.memory_ptr - target));
        }
        self.memory_ptr = target;
    }

    fn clear_cell(&mut self) {
        self.output.push_str("[-]");
    }

    fn set_value(&mut self, addr: usize, value: i32) {
        self.move_to(addr);
        self.clear_cell();

        if value > 0 {
            // Optimize for larger values using loops
            if value >= 10 {
                let loop_count = (value as f64).sqrt() as i32;
                let remainder = value - (loop_count * loop_count);

                // Set up loop counter
                self.output.push_str(&"+".repeat(loop_count as usize));
                self.output.push('[');
                self.move_to(addr + 1);
                self.output.push_str(&"+".repeat(loop_count as usize));
                self.move_to(addr);
                self.output.push_str("-]");

                // Move the result back and add remainder
                self.move_to(addr + 1);
                self.output.push_str("[-");
                self.move_to(addr);
                self.output.push('+');
                self.move_to(addr + 1);
                self.output.push(']');

                // Add remainder
                self.move_to(addr);
                if remainder > 0 {
                    self.output.push_str(&"+".repeat(remainder as usize));
                }
            } else {
                self.output.push_str(&"+".repeat(value as usize));
            }
        } else if value < 0 {
            self.output.push_str(&"-".repeat((-value) as usize));
        }
    }

    fn copy_value(&mut self, from: usize, to: usize) {
        let temp = self.get_temp_addr();

        // Clear destination and temp
        self.move_to(to);
        self.clear_cell();
        self.move_to(temp);
        self.clear_cell();

        // Copy from source to both destination and temp
        self.move_to(from);
        self.output.push_str("[-");
        self.move_to(to);
        self.output.push('+');
        self.move_to(temp);
        self.output.push('+');
        self.move_to(from);
        self.output.push(']');

        // Restore source from temp
        self.move_to(temp);
        self.output.push_str("[-");
        self.move_to(from);
        self.output.push('+');
        self.move_to(temp);
        self.output.push(']');
    }

    fn add_values(&mut self, result_addr: usize, left_addr: usize, right_addr: usize) {
        // Copy left to result
        self.copy_value(left_addr, result_addr);

        // Add right to result
        let temp = self.get_temp_addr();
        self.copy_value(right_addr, temp);
        self.move_to(temp);
        self.output.push_str("[-");
        self.move_to(result_addr);
        self.output.push('+');
        self.move_to(temp);
        self.output.push(']');
    }

    fn sub_values(&mut self, result_addr: usize, left_addr: usize, right_addr: usize) {
        // Copy left to result
        self.copy_value(left_addr, result_addr);

        // Subtract right from result
        let temp = self.get_temp_addr();
        self.copy_value(right_addr, temp);
        self.move_to(temp);
        self.output.push_str("[-");
        self.move_to(result_addr);
        self.output.push('-');
        self.move_to(temp);
        self.output.push(']');
    }

    fn compare_equal(&mut self, result_addr: usize, left_addr: usize, right_addr: usize) {
        let temp1 = self.get_temp_addr();
        let temp2 = self.get_temp_addr();

        // Copy values to temp locations
        self.copy_value(left_addr, temp1);
        self.copy_value(right_addr, temp2);

        // Clear result
        self.move_to(result_addr);
        self.clear_cell();
        self.output.push('+'); // Assume equal initially

        // Subtract temp2 from temp1
        self.move_to(temp2);
        self.output.push_str("[-");
        self.move_to(temp1);
        self.output.push('-');
        self.move_to(temp2);
        self.output.push(']');

        // If temp1 is not zero, values were not equal
        self.move_to(temp1);
        self.output.push('[');
        self.move_to(result_addr);
        self.output.push('-'); // Set result to 0
        self.move_to(temp1);
        self.output.push_str("[-]]"); // Clear temp1
    }

    fn evaluate_condition(&mut self, condition: &Expr) -> usize {
        match condition {
            Expr::Binary {
                left,
                operator: BinaryOp::Equal,
                right,
            } => {
                let left_addr = self.evaluate_expression(left);
                let right_addr = self.evaluate_expression(right);
                let result_addr = self.get_temp_addr();
                self.compare_equal(result_addr, left_addr, right_addr);
                result_addr
            }
            Expr::Binary {
                left,
                operator: BinaryOp::Greater,
                right,
            } => {
                // Simplified: just evaluate left side
                self.evaluate_expression(left)
            }
            _ => self.evaluate_expression(condition),
        }
    }

    fn evaluate_expression(&mut self, expr: &Expr) -> usize {
        match expr {
            Expr::Number(n) => {
                let addr = self.get_temp_addr();
                self.set_value(addr, *n);
                addr
            }
            Expr::Variable(name) => {
                if let Some(&addr) = self.variables.get(name) {
                    addr
                } else {
                    // Error: undefined variable - create a zero cell
                    let addr = self.get_temp_addr();
                    self.set_value(addr, 0);
                    addr
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_addr = self.evaluate_expression(left);
                let right_addr = self.evaluate_expression(right);
                let result_addr = self.get_temp_addr();

                match operator {
                    BinaryOp::Add => self.add_values(result_addr, left_addr, right_addr),
                    BinaryOp::Sub => self.sub_values(result_addr, left_addr, right_addr),
                    BinaryOp::Equal => self.compare_equal(result_addr, left_addr, right_addr),
                    BinaryOp::NotEqual => {
                        self.compare_equal(result_addr, left_addr, right_addr);
                        // Flip the result
                        let temp = self.get_temp_addr();
                        self.move_to(temp);
                        self.output.push('+');
                        self.move_to(result_addr);
                        self.output.push_str("[-");
                        self.move_to(temp);
                        self.output.push('-');
                        self.move_to(result_addr);
                        self.output.push(']');
                        self.move_to(temp);
                        self.output.push_str("[-");
                        self.move_to(result_addr);
                        self.output.push('+');
                        self.move_to(temp);
                        self.output.push(']');
                    }
                    BinaryOp::Less => {
                        // Simplified: copy left value
                        self.copy_value(left_addr, result_addr);
                    }
                    BinaryOp::Greater => {
                        // Simplified: copy left value
                        self.copy_value(left_addr, result_addr);
                    }
                }

                result_addr
            }
        }
    }
}

impl Visitor<()> for BrainfuckGenerator {
    fn visit_program(&mut self, program: &Program) -> () {
        for stmt in program {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> () {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let addr = self.allocate_variable(name);
                let value_addr = self.evaluate_expression(value);
                self.copy_value(value_addr, addr);
            }
            Stmt::Assign { name, value } => {
                if let Some(&addr) = self.variables.get(name) {
                    let value_addr = self.evaluate_expression(value);
                    self.copy_value(value_addr, addr);
                }
            }
            Stmt::Print(expr) => {
                let addr = self.evaluate_expression(expr);
                self.move_to(addr);
                self.output.push('.');
            }
            Stmt::If { condition, body } => {
                let condition_addr = self.evaluate_condition(condition);
                self.move_to(condition_addr);
                self.output.push('[');

                for stmt in body {
                    self.visit_stmt(stmt);
                }

                // Clear condition and end if
                self.move_to(condition_addr);
                self.clear_cell();
                self.output.push(']');
            }
            Stmt::While { condition, body } => {
                let condition_addr = self.evaluate_condition(condition);
                self.move_to(condition_addr);
                self.output.push('[');

                for stmt in body {
                    self.visit_stmt(stmt);
                }

                // Re-evaluate condition
                let new_condition_addr = self.evaluate_condition(condition);
                self.copy_value(new_condition_addr, condition_addr);
                self.move_to(condition_addr);
                self.output.push(']');
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> () {
        self.evaluate_expression(expr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_generate_let_statement() {
        let mut generator = BrainfuckGenerator::new();
        let program = vec![Stmt::let_stmt("x", false, Expr::number(42))];

        let result = generator.generate(&program).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_generate_print_statement() {
        let mut generator = BrainfuckGenerator::new();
        let program = vec![
            Stmt::let_stmt("x", false, Expr::number(65)), // ASCII 'A'
            Stmt::print(Expr::variable("x")),
        ];

        let result = generator.generate(&program).unwrap();
        assert!(result.contains('.'));
    }
}
