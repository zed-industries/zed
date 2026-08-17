//! Daemon-specific integration tests
//!
//! This module tests the `daemon()` function which creates a detached background process.
//! These tests verify:
//! - Process detachment and proper PID management
//! - Directory handling (chdir vs nochdir)
//! - Process group and session management
//! - File descriptor handling (noclose option)
//! - Command execution in daemon context
//! - Absence of controlling terminal
//!
//! Note: These tests fork twice (the daemon pattern) so they run in separate
//! processes to avoid terminating the test runner when daemon() calls exit(0).

mod common;

use common::{get_unique_test_dir, setup_test_dir, wait_for_file};
use fork::{Fork, daemon, fork};
use std::{
    env, fs,
    process::{Command, exit},
};

#[test]
fn test_daemon_creates_detached_process() {
    // Tests that daemon() successfully creates a detached background process
    // Expected behavior:
    // 1. Parent process forks
    // 2. First child creates new session and forks again
    // 3. First child exits (daemon() calls exit(0))
    // 4. Grandchild (daemon) is detached and writes its PID
    // 5. Daemon changes to root directory (nochdir=false)
    // 6. Daemon has valid PID > 0
    let test_dir = setup_test_dir(get_unique_test_dir("daemon_creates_detached"));
    let marker_file = test_dir.join("daemon.marker");

    // Fork the test to avoid daemon() calling exit(0) on parent
    match fork().expect("Failed to fork") {
        Fork::Parent(_) => {
            // Parent waits for marker file to be created
            assert!(
                wait_for_file(&marker_file, 500),
                "Daemon should have created marker file"
            );

            // Read PID from marker file
            let content = fs::read_to_string(&marker_file).expect("Failed to read marker file");
            let daemon_pid: i32 = content.trim().parse().expect("Failed to parse PID");
            assert!(daemon_pid > 0, "Daemon PID should be positive");

            // Cleanup
            let _ = fs::remove_dir_all(&test_dir);
        }
        Fork::Child => {
            // Child calls daemon()
            if let Ok(Fork::Child) = daemon(false, true) {
                // This is the daemon process
                // Write our PID to marker file
                let pid = unsafe { libc::getpid() };
                fs::write(&marker_file, format!("{}", pid)).expect("Failed to write marker file");

                // Verify we're in root directory
                let current = env::current_dir().expect("Failed to get current dir");
                assert_eq!(current.to_str(), Some("/"));

                exit(0);
            }
            // Parent of daemon exits (daemon() calls exit(0) for us)
        }
    }
}

#[test]
fn test_daemon_with_nochdir() {
    // Tests that daemon(nochdir=true) preserves the current working directory
    // Expected behavior:
    // 1. Test changes to a specific directory before calling daemon()
    // 2. daemon(true, true) is called (nochdir=true, noclose=true)
    // 3. Daemon process should remain in the same directory (not /)
    // 4. Daemon writes current directory to file for verification
    let test_dir = setup_test_dir(get_unique_test_dir("daemon_nochdir"));
    let marker_file = test_dir.join("nochdir.marker");

    // Change to test directory
    env::set_current_dir(&test_dir).expect("Failed to change directory");

    match fork().expect("Failed to fork") {
        Fork::Parent(_) => {
            assert!(
                wait_for_file(&marker_file, 500),
                "Daemon should have created marker file"
            );

            // Cleanup
            let _ = fs::remove_dir_all(&test_dir);
        }
        Fork::Child => {
            if let Ok(Fork::Child) = daemon(true, true) {
                // Daemon with nochdir=true should preserve directory
                let current = env::current_dir().expect("Failed to get current dir");

                // Write confirmation to marker file
                fs::write(&marker_file, format!("{}", current.display()))
                    .expect("Failed to write marker file");

                // Directory should still be test_dir, not root
                assert_ne!(current.to_str(), Some("/"));

                exit(0);
            }
        }
    }
}

#[test]
fn test_daemon_process_group() {
    // Tests that daemon creates proper process group structure
    // Expected behavior:
    // 1. daemon() performs double-fork pattern
    // 2. After double-fork, daemon is NOT a session leader (PID != PGID)
    // 3. This prevents daemon from acquiring a controlling terminal
    // 4. Both PID and PGID are positive values
    // 5. Daemon writes PID,PGID to file for verification
    let test_dir = setup_test_dir(get_unique_test_dir("daemon_process_group"));
    let marker_file = test_dir.join("pgid.marker");

    match fork().expect("Failed to fork") {
        Fork::Parent(_) => {
            assert!(
                wait_for_file(&marker_file, 500),
                "Daemon should have created marker file"
            );

            // Read and verify process group info
            let content = fs::read_to_string(&marker_file).expect("Failed to read marker file");
            let parts: Vec<&str> = content.trim().split(',').collect();
            assert_eq!(parts.len(), 2);

            let pid: i32 = parts[0].parse().expect("Failed to parse PID");
            let pgid: i32 = parts[1].parse().expect("Failed to parse PGID");

            // Daemon (after double-fork) should NOT be session leader
            // but should be in a new process group
            assert!(pid > 0, "PID should be positive");
            assert!(pgid > 0, "PGID should be positive");
            assert_ne!(
                pid, pgid,
                "Daemon (after double-fork) should NOT be session leader"
            );

            // Cleanup
            let _ = fs::remove_dir_all(&test_dir);
        }
        Fork::Child => {
            if let Ok(Fork::Child) = daemon(false, true) {
                let pid = unsafe { libc::getpid() };
                let pgid = unsafe { libc::getpgrp() };

                fs::write(&marker_file, format!("{},{}", pid, pgid))
                    .expect("Failed to write marker file");

                exit(0);
            }
        }
    }
}

#[test]
fn test_daemon_with_command_execution() {
    // Tests that daemon can execute commands successfully
    // Expected behavior:
    // 1. Daemon process is created
    // 2. Daemon executes a shell command
    // 3. Command output is written to a file
    // 4. Parent can verify command executed correctly
    // 5. Tests real-world daemon usage pattern
    let test_dir = setup_test_dir(get_unique_test_dir("daemon_command_exec"));
    let output_file = test_dir.join("command.output");

    match fork().expect("Failed to fork") {
        Fork::Parent(_) => {
            assert!(
                wait_for_file(&output_file, 500),
                "Command output file should exist"
            );

            let content = fs::read_to_string(&output_file).expect("Failed to read output file");
            assert!(
                content.contains("hello from daemon"),
                "Output should contain expected text"
            );

            // Cleanup
            let _ = fs::remove_dir_all(&test_dir);
        }
        Fork::Child => {
            if let Ok(Fork::Child) = daemon(false, true) {
                // Execute a command in the daemon
                Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "echo 'hello from daemon' > {}",
                        output_file.display()
                    ))
                    .output()
                    .expect("Failed to execute command");

                exit(0);
            }
        }
    }
}

#[test]
fn test_daemon_no_controlling_terminal() {
    // Tests that daemon has no controlling terminal
    // Expected behavior:
    // 1. Daemon process is created
    // 2. Daemon calls 'tty' command to check for terminal
    // 3. tty command should return "not a tty" or similar error
    // 4. This confirms daemon is properly detached from terminal
    // 5. Critical for background service behavior
    let test_dir = setup_test_dir(get_unique_test_dir("daemon_no_tty"));
    let tty_file = test_dir.join("tty.info");

    match fork().expect("Failed to fork") {
        Fork::Parent(_) => {
            assert!(wait_for_file(&tty_file, 500), "TTY info file should exist");

            let content = fs::read_to_string(&tty_file).expect("Failed to read tty file");
            // When daemon has no controlling terminal, tty command should fail or return "not a tty"
            assert!(
                content.contains("not a tty") || content.contains("No such"),
                "Daemon should have no controlling terminal, got: {}",
                content
            );

            // Cleanup
            let _ = fs::remove_dir_all(&test_dir);
        }
        Fork::Child => {
            if let Ok(Fork::Child) = daemon(false, true) {
                // Check if we have a controlling terminal
                let output = Command::new("tty")
                    .output()
                    .expect("Failed to run tty command");

                let tty_output = if output.stdout.is_empty() {
                    String::from_utf8_lossy(&output.stderr).to_string()
                } else {
                    String::from_utf8_lossy(&output.stdout).to_string()
                };

                fs::write(&tty_file, tty_output).expect("Failed to write tty file");

                exit(0);
            }
        }
    }
}
