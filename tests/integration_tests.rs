use std::process::Command;
use std::str;

#[test]
fn test_help_output() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    assert!(stdout.contains("A simple system monitoring tool"));
    assert!(stdout.contains("--interval"));
    assert!(stdout.contains("--once"));
    assert!(stdout.contains("--no-color"));
}

#[test]
fn test_once_mode_produces_output() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--once"])
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    // Check for expected sections
    assert!(stdout.contains("SYSTEM MONITOR"));
    assert!(stdout.contains("CPU Usage:"));
    assert!(stdout.contains("Memory:"));
    assert!(stdout.contains("Disk Usage:"));

    // Should contain progress bars (with █ character)
    assert!(stdout.contains("█"));

    // Should exit cleanly
    assert!(output.status.success());
}

#[test]
fn test_no_color_flag_removes_ansi_codes() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--once", "--no-color"])
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    // Should contain basic output
    assert!(stdout.contains("CPU Usage:"));
    assert!(stdout.contains("Memory:"));

    // Should NOT contain ANSI color codes
    assert!(!stdout.contains("\x1B[31m")); // No red
    assert!(!stdout.contains("\x1B[33m")); // No yellow
    assert!(!stdout.contains("\x1B[32m")); // No green
    assert!(!stdout.contains("\x1B[0m")); // No reset
}

#[test]
fn test_with_color_flag_contains_ansi_codes() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--once"]) // Default is color enabled
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    // Should contain basic output
    assert!(stdout.contains("CPU Usage:"));

    // Should contain ANSI codes (at least reset codes)
    assert!(stdout.contains("\x1B[0m"));
}

#[test]
fn test_custom_interval_accepted() {
    // This test just verifies the CLI accepts the interval arg
    // We use --once so it doesn't run forever
    let output = Command::new("cargo")
        .args(&["run", "--", "--interval", "5", "--once"])
        .output()
        .expect("Failed to execute command");

    // Should exit successfully and produce normal output
    assert!(output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("SYSTEM MONITOR"));
}

#[test]
fn test_invalid_interval_fails() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--interval", "not-a-number"])
        .output()
        .expect("Failed to execute command");

    // Should fail with non-zero exit code
    assert!(!output.status.success());
}

#[test]
fn test_output_format_structure() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--once", "--no-color"])
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();

    // Basic structure checks
    assert!(lines.iter().any(|line| line.contains("SYSTEM MONITOR")));

    // Should have CPU line followed by progress bar
    let cpu_line_idx = lines
        .iter()
        .position(|line| line.contains("CPU Usage:"))
        .unwrap();
    assert!(lines[cpu_line_idx + 1].contains("["));
    assert!(lines[cpu_line_idx + 1].contains("]"));

    // Should have Memory line followed by progress bar
    let mem_line_idx = lines
        .iter()
        .position(|line| line.contains("Memory:"))
        .unwrap();
    assert!(lines[mem_line_idx + 1].contains("["));
    assert!(lines[mem_line_idx + 1].contains("]"));
}
