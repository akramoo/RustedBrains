use std::fs;
use std::process::Command;

#[test]
fn test_compile_simple_example() {
    let output = Command::new("cargo")
        .args(&["run", "examples/simple.txt"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Generated Brainfuck"));
}

#[test]
fn test_output_file_creation() {
    let _output = Command::new("cargo")
        .args(&["run", "examples/simple.txt"])
        .output()
        .expect("Failed to execute command");

    assert!(fs::metadata("examples/simple.txt.bf").is_ok());

    // Cleanup
    let _ = fs::remove_file("examples/simple.txt.bf");
}

#[test]
fn test_error_handling() {
    let output = Command::new("cargo")
        .args(&["run", "nonexistent.rs"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Error"));
}
