// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![cfg(feature = "cli")]

use std::fs;
use std::process::Command;

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_flag_parsing() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "-b",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command should succeed with -b flag"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Algorithm: CRC-32/ISCSI"));
    assert!(stdout.contains("Throughput:"));
    assert!(stdout.contains("GiB/s"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_with_size_parameter() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "--size",
            "1024",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Data Size: 1,024 bytes"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_with_duration_parameter() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "--duration",
            "1.0",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Duration: 1."));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_invalid_size() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "--size",
            "0",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Size must be greater than 0"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_invalid_duration() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "--duration",
            "0",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Duration must be greater than 0"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_with_file_input() {
    // Create a temporary test file
    let test_file = "test_benchmark_file.txt";
    fs::write(test_file, "Hello, benchmark world!").expect("Failed to create test file");

    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "-f",
            test_file,
            "--duration",
            "0.5",
        ])
        .output()
        .expect("Failed to execute command");

    // Clean up
    let _ = fs::remove_file(test_file);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Data Size: 23 bytes"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_with_string_input() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "-s",
            "test string",
            "--duration",
            "0.5",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Data Size: 11 bytes"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_different_algorithms() {
    let algorithms = ["CRC-32/ISCSI", "CRC-64/NVME"];

    for algorithm in &algorithms {
        let output = Command::new("cargo")
            .args([
                "run",
                "--features",
                "cli",
                "--bin",
                "checksum",
                "--",
                "-a",
                algorithm,
                "-b",
                "--duration",
                "0.5",
            ])
            .output()
            .expect("Failed to execute command");

        assert!(
            output.status.success(),
            "Algorithm {} should work",
            algorithm
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(&format!("Algorithm: {}", algorithm)));
    }
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_size_without_benchmark_flag() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "--size",
            "1024",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--size and --duration can only be used with -b flag"));
}

#[test]
#[cfg_attr(miri, ignore)] // Miri doesn't allow this due to isolation restrictions
fn test_benchmark_nonexistent_file() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--features",
            "cli",
            "--bin",
            "checksum",
            "--",
            "-a",
            "CRC-32/ISCSI",
            "-b",
            "-f",
            "nonexistent_file.txt",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("File not found"));
}
