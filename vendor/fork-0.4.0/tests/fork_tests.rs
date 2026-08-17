//! Fork functionality integration tests
//!
//! This module tests the core `fork()` and `waitpid()` functions.
//! These tests verify:
//! - Basic fork/waitpid functionality
//! - Parent-child process communication via files
//! - Multiple child process management
//! - Environment variable inheritance across fork
//! - Command execution in child processes
//! - PID uniqueness between parent and child
//! - Proper process synchronization with waitpid
//!
//! All tests use temporary files for parent-child communication since
//! forked processes have separate memory spaces.

mod common;

use common::{get_test_dir, setup_test_dir};
use fork::{Fork, fork, waitpid};
use std::{
    env, fs,
    process::{Command, exit},
    thread,
    time::Duration,
};

#[test]
// Tests basic fork() functionality with waitpid()
// Expected behavior:
// 1. fork() returns Ok(Fork::Parent(pid)) in parent with child PID
// 2. fork() returns Ok(Fork::Child) in child
// 3. Child PID is positive
// 4. waitpid() successfully waits for child to exit
// 5. No zombie processes remain
fn test_fork_basic() {
    match fork() {
        Ok(Fork::Parent(child)) => {
            assert!(child > 0, "Child PID should be positive");

            // Wait for child
            assert!(waitpid(child).is_ok(), "waitpid should succeed");
        }
        Ok(Fork::Child) => {
            // Child just exits
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
// Tests parent-child communication using files
// Expected behavior:
// 1. Parent and child have separate memory spaces
// 2. Child writes a message to a file
// 3. Parent reads the message after child completes
// 4. Message content matches what child wrote
// 5. Demonstrates file-based IPC pattern
fn test_fork_parent_child_communication() {
    let test_dir = setup_test_dir(get_test_dir("fork_communication"));
    let message_file = test_dir.join("message.txt");

    match fork() {
        Ok(Fork::Parent(child)) => {
            // Wait for child to write
            thread::sleep(Duration::from_millis(50));

            // Read message from child
            let message = fs::read_to_string(&message_file).expect("Failed to read message file");
            assert_eq!(message.trim(), "hello from child");

            waitpid(child).expect("Failed to wait for child");

            // Cleanup
            fs::remove_file(&message_file).ok();
        }
        Ok(Fork::Child) => {
            // Write message
            fs::write(&message_file, "hello from child").expect("Failed to write message");
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
fn test_fork_multiple_children() {
    let mut children = Vec::new();
    // Tests creating and managing multiple child processes
    // Expected behavior:
    // 1. Parent creates 3 child processes sequentially
    // 2. Each child exits with a different exit code
    // 3. Parent tracks all child PIDs
    // 4. Parent successfully waits for all children
    // 5. No zombie processes remain

    for i in 0..3 {
        match fork() {
            Ok(Fork::Parent(child)) => {
                children.push(child);
            }
            Ok(Fork::Child) => {
                // Each child exits with different code
                exit(i);
            }
            Err(_) => panic!("Fork {} failed", i),
        }
    }

    // Parent waits for all children
    assert_eq!(children.len(), 3, "Should have 3 children");

    for child in children {
        assert!(waitpid(child).is_ok(), "Failed to wait for child {}", child);
    }
}

#[test]
fn test_fork_child_inherits_environment() {
    let test_dir = setup_test_dir(get_test_dir("fork_env"));
    // Tests environment variable inheritance across fork
    // Expected behavior:
    // 1. Parent sets an environment variable before fork
    // 2. Child inherits parent's environment
    // 3. Child can read the environment variable
    // 4. Child writes variable value to file for verification
    // 5. Demonstrates environment inheritance
    let env_file = test_dir.join("env.txt");

    // Set a test environment variable
    let test_var = "FORK_TEST_VAR";
    let test_value = "test_value_12345";
    unsafe {
        env::set_var(test_var, test_value);
    }

    match fork() {
        Ok(Fork::Parent(child)) => {
            thread::sleep(Duration::from_millis(50));

            let content = fs::read_to_string(&env_file).expect("Failed to read env file");
            assert_eq!(content.trim(), test_value);

            waitpid(child).expect("Failed to wait for child");

            // Cleanup
            fs::remove_file(&env_file).ok();
            unsafe {
                env::remove_var(test_var);
            }
        }
        Ok(Fork::Child) => {
            // Child should have inherited the environment
            let value = env::var(test_var).expect("Environment variable not found");
            fs::write(&env_file, value).expect("Failed to write env file");
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}
// Tests that child process can execute external commands
// Expected behavior:
// 1. Child process forks successfully
// 2. Child executes 'echo' command
// 3. Command output is captured
// 4. Output is written to file
// 5. Parent verifies command executed successfully

#[test]
fn test_fork_child_can_execute_commands() {
    let test_dir = get_test_dir("fork");
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    let output_file = test_dir.join("command_output.txt");

    match fork() {
        Ok(Fork::Parent(child)) => {
            thread::sleep(Duration::from_millis(100));

            assert!(output_file.exists(), "Output file should exist");
            let content = fs::read_to_string(&output_file).expect("Failed to read output");
            assert!(!content.is_empty(), "Output should not be empty");

            waitpid(child).expect("Failed to wait for child");

            // Cleanup
            fs::remove_file(&output_file).ok();
        }
        Ok(Fork::Child) => {
            // Execute a command and save output
            let output = Command::new("echo")
                .arg("child executed command")
                .output()
                // Tests that parent and child have unique PIDs
                // Expected behavior:
                // 1. Parent records its PID before fork
                // 2. Child records its PID after fork
                // 3. Child PID differs from parent PID
                // 4. fork() returns correct child PID to parent
                // 5. PIDs match between fork return value and actual child PID
                .expect("Failed to execute command");

            fs::write(&output_file, &output.stdout).expect("Failed to write output");
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
fn test_fork_child_has_different_pid() {
    let test_dir = get_test_dir("fork");
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    let pid_file = test_dir.join("pids.txt");

    let parent_pid = unsafe { libc::getpid() };

    match fork() {
        Ok(Fork::Parent(child)) => {
            thread::sleep(Duration::from_millis(50));

            let content = fs::read_to_string(&pid_file).expect("Failed to read pid file");
            let child_pid: i32 = content.trim().parse().expect("Failed to parse PID");

            assert_ne!(
                parent_pid, child_pid,
                "Parent and child should have different PIDs"
            );
            assert_eq!(
                child, child_pid,
                "Child PID from fork() should match actual child PID"
            );
            // Tests that waitpid() properly synchronizes parent-child execution
            // Expected behavior:
            // 1. Parent forks and immediately checks for marker file
            // 2. Marker file doesn't exist yet (child hasn't run)
            // 3. Parent calls waitpid() to wait for child
            // 4. Child creates marker file before exiting
            // 5. After waitpid(), marker file exists (child completed)

            waitpid(child).expect("Failed to wait for child");

            // Cleanup
            fs::remove_file(&pid_file).ok();
        }
        Ok(Fork::Child) => {
            let child_pid = unsafe { libc::getpid() };
            fs::write(&pid_file, format!("{}", child_pid)).expect("Failed to write PID");
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
fn test_waitpid_waits_for_child() {
    let test_dir = get_test_dir("fork");
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    let marker_file = test_dir.join("wait_marker.txt");

    match fork() {
        Ok(Fork::Parent(child)) => {
            // Marker should not exist yet
            assert!(
                !marker_file.exists(),
                "Marker should not exist before child runs"
            );

            // Wait for child
            waitpid(child).expect("Failed to wait for child");

            // Now marker should exist
            assert!(
                marker_file.exists(),
                "Marker should exist after child completes"
            );

            // Cleanup
            fs::remove_file(&marker_file).ok();
        }
        Ok(Fork::Child) => {
            // Sleep a bit to ensure parent checks first
            thread::sleep(Duration::from_millis(50));

            // Create marker
            fs::write(&marker_file, "done").expect("Failed to write marker");
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}
