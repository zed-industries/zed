//! Advanced integration tests for complex fork patterns
//!
//! This module tests advanced usage patterns and combinations of functions.
//! These tests verify:
//! - Classic double-fork daemon pattern
//! - Session creation and management (setsid)
//! - Directory changes in child processes (chdir)
//! - Process isolation between parent and child
//! - Process group queries (getpgrp)
//!
//! These tests combine multiple fork operations to test real-world
//! daemon creation patterns and process management scenarios.

mod common;

use common::{get_test_dir, setup_test_dir, wait_for_file};
use fork::{Fork, chdir, fork, getpgrp, setsid, waitpid};
use std::{env, fs, process::exit, thread, time::Duration};

#[test]
fn test_double_fork_daemon_pattern() {
    let test_dir = setup_test_dir(get_test_dir("int_double_fork"));
    let daemon_pid_file = test_dir.join("daemon.pid");

    // First fork
    match fork().expect("First fork failed") {
        Fork::Parent(_child) => {
            // Original parent waits for daemon to create PID file
            // Use longer timeout for CI environments
            assert!(
                wait_for_file(&daemon_pid_file, 1000),
                "Daemon PID file should exist"
            );

            // Tests the classic double-fork daemon pattern
            // Expected behavior:
            // 1. First fork creates a child process
            // 2. Child calls setsid() to create new session (becomes session leader)
            // 3. Child forks again (grandchild)
            // 4. First child exits (leaving grandchild orphaned)
            // 5. Grandchild is not session leader (prevents controlling terminal acquisition)
            // 6. Grandchild writes its PID to file
            // 7. This is the standard daemon creation pattern
            let pid_str = fs::read_to_string(&daemon_pid_file).expect("Failed to read PID file");
            let daemon_pid: i32 = pid_str.trim().parse().expect("Failed to parse daemon PID");
            assert!(daemon_pid > 0, "Daemon PID should be positive");

            // Cleanup
            fs::remove_file(&daemon_pid_file).ok();
        }
        Fork::Child => {
            // First child - create new session
            setsid().expect("setsid failed");

            // Second fork to ensure we're not session leader
            match fork().expect("Second fork failed") {
                Fork::Parent(_) => {
                    // First child exits
                    exit(0);
                }
                Fork::Child => {
                    // This is the daemon process
                    let pid = unsafe { libc::getpid() };
                    let pgid = getpgrp().expect("getpgrp failed");

                    // Write PID to file
                    fs::write(&daemon_pid_file, format!("{}", pid))
                        .expect("Failed to write daemon PID");

                    // Daemon should be in its own process group
                    assert!(pgid > 0);

                    exit(0);
                }
            }
        }
    }
}

#[test]
fn test_setsid_creates_new_session() {
    let test_dir = setup_test_dir(get_test_dir("int_double_fork"));
    let session_file = test_dir.join("session.info");

    match fork().expect("Fork failed") {
        Fork::Parent(_child) => {
            thread::sleep(Duration::from_millis(50));

            let content = fs::read_to_string(&session_file).expect("Failed to read session file");
            let parts: Vec<&str> = content.trim().split(',').collect();

            let sid: i32 = parts[0].parse().expect("Failed to parse SID");
            let pid: i32 = parts[1].parse().expect("Failed to parse PID");
            let pgid: i32 = parts[2].parse().expect("Failed to parse PGID");

            // After setsid, PID should equal PGID (session leader)
            assert_eq!(pid, pgid, "Process should be session leader");
            assert_eq!(sid, pid, "SID should equal PID for session leader");
            // Tests session creation and management with setsid()
            // Expected behavior:
            // 1. Child process calls setsid()
            // 2. setsid() creates new session and returns SID
            // 3. Child becomes session leader (PID == PGID == SID)
            // 4. This is Step 2 of the daemon pattern
            // 5. Child writes SID, PID, PGID to file for verification

            // Cleanup
            fs::remove_file(&session_file).ok();
        }
        Fork::Child => {
            // Create new session
            let sid = setsid().expect("setsid failed");
            let pid = unsafe { libc::getpid() };
            let pgid = getpgrp().expect("getpgrp failed");

            fs::write(&session_file, format!("{},{},{}", sid, pid, pgid))
                .expect("Failed to write session info");

            exit(0);
        }
    }
}

#[test]
fn test_chdir_changes_directory() {
    let test_dir = setup_test_dir(get_test_dir("int_chdir"));
    let dir_file = test_dir.join("directory.info");

    match fork().expect("Fork failed") {
        Fork::Parent(_child) => {
            thread::sleep(Duration::from_millis(50));
            // Tests directory change in child process
            // Expected behavior:
            // 1. Child process calls chdir()
            // 2. chdir() changes current directory to root (/)
            // 3. Child verifies current directory is /
            // 4. Child writes directory path to file
            // 5. Parent verifies child changed directory correctly

            let content = fs::read_to_string(&dir_file).expect("Failed to read dir file");
            assert_eq!(content.trim(), "/", "Directory should be root");

            // Cleanup
            fs::remove_file(&dir_file).ok();
        }
        Fork::Child => {
            // Change to root
            chdir().expect("chdir failed");

            let current = env::current_dir().expect("Failed to get current dir");
            fs::write(&dir_file, current.to_str().unwrap())
                .expect("Failed to write directory info");

            exit(0);
        }
    }
}

#[test]
// Tests process isolation between parent and child
// Expected behavior:
// 1. Parent writes data to file before fork
// 2. Child can see parent's file (same filesystem)
// 3. Child writes its own file
// 4. Parent can see child's file after fork completes
// 5. Both processes can access shared filesystem but have separate memory
fn test_process_isolation() {
    let test_dir = setup_test_dir(get_test_dir("int_isolation"));
    let parent_file = test_dir.join("parent.txt");
    let child_file = test_dir.join("child.txt");

    // Parent writes before fork
    fs::write(&parent_file, "parent data").expect("Failed to write parent file");

    match fork().expect("Fork failed") {
        Fork::Parent(_child) => {
            thread::sleep(Duration::from_millis(50));

            // Parent file should still exist
            assert!(parent_file.exists(), "Parent file should exist");

            // Child should have created its own file
            assert!(child_file.exists(), "Child file should exist");

            let child_content = fs::read_to_string(&child_file).expect("Failed to read child file");
            assert_eq!(child_content.trim(), "child data");

            // Cleanup
            fs::remove_file(&parent_file).ok();
            fs::remove_file(&child_file).ok();
        }
        Fork::Child => {
            // Child can see parent's file
            assert!(parent_file.exists(), "Child should see parent file");

            // Child writes its own file
            fs::write(&child_file, "child data").expect("Failed to write child file");

            exit(0);
            // Tests process group queries with getpgrp()
            // Expected behavior:
            // 1. Both parent and child can call getpgrp()
            // 2. Both return valid positive PGID values
            // 3. Initially parent and child share same process group
            // 4. Used to verify process group membership
            // 5. Critical for session and job control
        }
    }
}

#[test]
fn test_getpgrp_returns_process_group() {
    match fork().expect("Fork failed") {
        Fork::Parent(_child) => {
            let parent_pgid = getpgrp().expect("getpgrp failed");
            assert!(parent_pgid > 0, "Parent PGID should be positive");

            thread::sleep(Duration::from_millis(50));
        }
        Fork::Child => {
            let child_pgid = getpgrp().expect("getpgrp failed");
            assert!(child_pgid > 0, "Child PGID should be positive");

            exit(0);
        }
    }
}

#[test]
fn test_chdir_error_handling() {
    // Test that chdir returns proper io::Error
    match fork().expect("Fork failed") {
        Fork::Parent(child) => {
            waitpid(child).expect("waitpid failed");
        }
        Fork::Child => {
            // chdir() to root should succeed
            let result = chdir();
            assert!(result.is_ok(), "chdir to root should succeed");

            // Verify we're actually in root
            let cwd = env::current_dir().expect("Failed to get current dir");
            assert_eq!(cwd.to_str().unwrap(), "/", "Should be in root directory");

            exit(0);
        }
    }
}

#[test]
fn test_chdir_returns_io_error() {
    // Test that chdir returns a proper io::Error type
    match fork().expect("Fork failed") {
        Fork::Parent(child) => {
            waitpid(child).expect("waitpid failed");
        }
        Fork::Child => {
            // Call chdir and verify return type
            let result: std::io::Result<()> = chdir();

            // Should succeed
            assert!(result.is_ok());

            // If it were to fail, we could access error details
            if let Err(e) = result {
                let _errno = e.raw_os_error();
                let _msg = format!("{}", e);
            }

            exit(0);
        }
    }
}
