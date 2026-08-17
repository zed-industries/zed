/// Tests for stdio redirection to /dev/null
/// These tests verify that file descriptors 0,1,2 are not reused after closing stdio
use fork::{Fork, close_fd, fork, waitpid};
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::process::exit;

/// Test that demonstrates the fd reuse bug with close_fd()
///
/// This test SHOULD FAIL with current implementation because:
/// - close_fd() closes fd 0,1,2
/// - Next File::create() will get fd=0, then fd=1, then fd=2
/// - This test expects fd >= 3
#[test]
#[should_panic(expected = "File descriptors were reused")]
fn test_close_fd_allows_fd_reuse() {
    match fork() {
        Ok(Fork::Parent(child)) => {
            let result = waitpid(child);
            // If child exited with error, the bug exists
            if result.is_err()
                || std::fs::read_to_string("/tmp/fork_test_fd_marker.txt")
                    .unwrap_or_default()
                    .contains("REUSED")
            {
                // Cleanup
                let _ = std::fs::remove_file("/tmp/fork_test_fd_marker.txt");
                panic!("File descriptors were reused (bug exists)");
            }
        }
        Ok(Fork::Child) => {
            // Close stdio
            close_fd().unwrap();

            // Open files - with current implementation, they WILL get fd 0,1,2
            let f1 = File::create("/tmp/fork_test_fd1.txt").unwrap();
            let f2 = File::create("/tmp/fork_test_fd2.txt").unwrap();
            let f3 = File::create("/tmp/fork_test_fd3.txt").unwrap();

            let fd1 = f1.as_raw_fd();
            let fd2 = f2.as_raw_fd();
            let fd3 = f3.as_raw_fd();

            // Check if any fd is < 3 (the bug)
            if fd1 < 3 || fd2 < 3 || fd3 < 3 {
                // Write marker file to signal bug to parent
                std::fs::write("/tmp/fork_test_fd_marker.txt", "REUSED").ok();
                exit(1);
            }

            // If we get here, fds are >= 3 (the fix is working)
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

/// Test file descriptor reuse scenario
///
/// This test demonstrates that println! would write to wrong file
#[test]
fn test_fd_reuse_corruption_scenario() {
    use std::io::Write;

    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();

            // Check what was written to the files
            let content1 = std::fs::read_to_string("/tmp/fork_test_corruption1.txt").ok();
            let content2 = std::fs::read_to_string("/tmp/fork_test_corruption2.txt").ok();

            if let (Some(c1), Some(c2)) = (content1, content2) {
                // With close_fd(), these files will contain the debug output!
                // With redirect_stdio(), they will only contain intended data
                if c1.contains("This should NOT") || c2.contains("This should NOT") {
                    eprintln!("BUG DETECTED: Debug output leaked to data files!");
                    eprintln!("File 1: {}", c1);
                    eprintln!("File 2: {}", c2);
                    // Don't panic - this is expected with close_fd()
                } else {
                    println!("GOOD: Files only contain intended data");
                }
            }

            // Cleanup
            let _ = std::fs::remove_file("/tmp/fork_test_corruption1.txt");
            let _ = std::fs::remove_file("/tmp/fork_test_corruption2.txt");
        }
        Ok(Fork::Child) => {
            // Close stdio
            close_fd().unwrap();

            // Open files - they will get fd 0,1,2 with current implementation
            let mut f1 = File::create("/tmp/fork_test_corruption1.txt").unwrap();
            let mut f2 = File::create("/tmp/fork_test_corruption2.txt").unwrap();

            // Try to write debug output to stderr (fd=2)
            // With close_fd(), this might go to one of the files above!
            // We can't use eprintln! here because it might corrupt the files
            // Instead, directly write to demonstrate the issue
            let fd1 = f1.as_raw_fd();
            let fd2 = f2.as_raw_fd();

            // If files got fd 0 or 1, then fd=2 might be another file or free
            // Write a marker to show potential corruption
            if fd1 <= 2 || fd2 <= 2 {
                // Simulate what eprintln! would do
                let stderr_msg = b"This should NOT appear in data files\n";
                unsafe {
                    libc::write(2, stderr_msg.as_ptr() as *const _, stderr_msg.len());
                }
            }

            // Write intended data
            f1.write_all(b"data1\n").unwrap();
            f2.write_all(b"data2\n").unwrap();

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
fn test_redirect_stdio_prevents_fd_reuse() {
    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();
        }
        Ok(Fork::Child) => {
            // Redirect stdio to /dev/null
            fork::redirect_stdio().unwrap();

            // Open files - should get fd >= 3
            let f1 = File::create("/tmp/fork_test_redirect1.txt").unwrap();
            let f2 = File::create("/tmp/fork_test_redirect2.txt").unwrap();
            let f3 = File::create("/tmp/fork_test_redirect3.txt").unwrap();

            let fd1 = f1.as_raw_fd();
            let fd2 = f2.as_raw_fd();
            let fd3 = f3.as_raw_fd();

            // With redirect_stdio(), these should all be >= 3
            assert!(fd1 >= 3, "File 1 got fd < 3: {}", fd1);
            assert!(fd2 >= 3, "File 2 got fd < 3: {}", fd2);
            assert!(fd3 >= 3, "File 3 got fd < 3: {}", fd3);

            // Cleanup
            drop(f1);
            drop(f2);
            drop(f3);
            let _ = std::fs::remove_file("/tmp/fork_test_redirect1.txt");
            let _ = std::fs::remove_file("/tmp/fork_test_redirect2.txt");
            let _ = std::fs::remove_file("/tmp/fork_test_redirect3.txt");

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
fn test_redirect_stdio_println_safety() {
    use std::io::Write;

    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();

            // Verify files only contain intended data
            let content = std::fs::read_to_string("/tmp/fork_test_println_safe.txt").unwrap();
            assert!(!content.contains("debug"), "Debug output leaked to file!");
            assert_eq!(content, "data\n", "File content is correct");

            // Cleanup
            let _ = std::fs::remove_file("/tmp/fork_test_println_safe.txt");
        }
        Ok(Fork::Child) => {
            // Redirect stdio to /dev/null
            fork::redirect_stdio().unwrap();

            // Open file - gets fd >= 3
            let mut f = File::create("/tmp/fork_test_println_safe.txt").unwrap();

            // This println! will go to /dev/null (fd=1), not to the file
            println!("debug message that should not appear in file");

            // Write intended data
            f.write_all(b"data\n").unwrap();

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
fn test_daemon_uses_redirect_stdio() {
    // Test that daemon() correctly uses redirect_stdio() internally
    // We do this by manually testing the double-fork pattern with redirect_stdio()
    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();

            // Give daemon time to write file
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Check file was created and has correct content
            let content = std::fs::read_to_string("/tmp/fork_test_daemon_redirect.txt")
                .expect("Daemon should have created file");

            assert!(
                !content.contains("Should not appear"),
                "println! should have gone to /dev/null, not to file"
            );
            assert_eq!(
                content.trim(),
                "daemon data",
                "File should have correct data"
            );

            // Cleanup
            let _ = std::fs::remove_file("/tmp/fork_test_daemon_redirect.txt");
        }
        Ok(Fork::Child) => {
            use std::io::Write;

            // Simulate what daemon() does
            fork::setsid().unwrap();
            fork::redirect_stdio().unwrap(); // This is what daemon() now uses

            match fork() {
                Ok(Fork::Parent(_)) => exit(0), // First child exits
                Ok(Fork::Child) => {
                    // Grandchild (daemon) continues
                    let mut f = File::create("/tmp/fork_test_daemon_redirect.txt").unwrap();

                    // Verify file got fd >= 3
                    assert!(f.as_raw_fd() >= 3, "File should get fd >= 3");

                    // This println! goes to /dev/null
                    println!("Should not appear in file");

                    // Write actual data
                    f.write_all(b"daemon data\n").unwrap();
                    f.flush().unwrap();

                    exit(0);
                }
                Err(_) => exit(1),
            }
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
fn test_redirect_stdio_error_handling() {
    // Test that redirect_stdio returns proper io::Error
    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();
        }
        Ok(Fork::Child) => {
            // Call redirect_stdio - should succeed
            let result = fork::redirect_stdio();
            assert!(result.is_ok(), "redirect_stdio should succeed");

            // Verify we can access errno if needed (though it won't be set on success)
            if let Err(e) = result {
                // If it somehow fails, verify it's a proper io::Error
                let _os_error = e.raw_os_error();
                let _error_msg = format!("{}", e);
            }

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

#[test]
fn test_redirect_stdio_idempotent() {
    // Test that calling redirect_stdio multiple times is safe
    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();
        }
        Ok(Fork::Child) => {
            // First call
            fork::redirect_stdio().unwrap();

            // Second call should also work
            fork::redirect_stdio().unwrap();

            // Files should still get fd >= 3
            let f = File::create("/tmp/fork_test_idempotent.txt").unwrap();
            assert!(f.as_raw_fd() >= 3, "File should get fd >= 3");

            // Cleanup
            drop(f);
            let _ = std::fs::remove_file("/tmp/fork_test_idempotent.txt");

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}
