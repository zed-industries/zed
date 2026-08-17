/// Educational demonstration of the file descriptor reuse bug
///
/// This example shows:
/// 1. The BUG: How close_fd() causes fd reuse
/// 2. The FIX: How redirect_stdio() prevents fd reuse
///
/// Run with: cargo run --example demonstrate_fd_reuse_bug
use fork::{Fork, close_fd, fork, redirect_stdio, waitpid};
use std::fs::File;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::process::exit;

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║     DEMONSTRATION: File Descriptor Reuse Bug                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    demonstrate_bug();
    demonstrate_fix();

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                      SUMMARY                                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\nBUG: close_fd() frees fd 0,1,2 → files reuse them → corruption!");
    println!("FIX: redirect_stdio() keeps fd 0,1,2 busy → files get fd >= 3\n");
}

fn demonstrate_bug() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("PART 1: THE BUG (using close_fd)");
    println!("═══════════════════════════════════════════════════════════════\n");

    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();

            // Read what the child wrote
            let content = std::fs::read_to_string("/tmp/demo_bug.txt").unwrap_or_default();

            println!("\nResult with close_fd():");
            println!("  File content: '{}'", content.trim());

            if content.contains("This is debug output") {
                println!("  ⚠️  BUG DETECTED: Debug output corrupted the file!");
            } else {
                println!("  File only contains: {}", content.trim());
            }

            // Cleanup
            let _ = std::fs::remove_file("/tmp/demo_bug.txt");
        }
        Ok(Fork::Child) => {
            // Close stdio - THIS CAUSES THE BUG
            close_fd().unwrap();

            // Open a file - it will get a low fd (0, 1, or 2)
            let mut file = File::create("/tmp/demo_bug.txt").unwrap();
            let fd = file.as_raw_fd();

            println!("After close_fd():");
            println!("  File got fd = {}", fd);

            if fd < 3 {
                println!("  ⚠️  WARNING: File got fd < 3!");
                println!("  Any println! or panic will write to this file!\n");

                // Simulate what might happen with debug output
                // We'll write directly to fd=2 (stderr) to show the problem
                let debug_msg = b"This is debug output that should NOT be in the file!\n";
                unsafe {
                    // If another file got fd=2, this write goes to that file!
                    libc::write(2, debug_msg.as_ptr() as *const _, debug_msg.len());
                }
            }

            // Write intended data
            file.write_all(b"Expected data\n").unwrap();
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

fn demonstrate_fix() {
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PART 2: THE FIX (using redirect_stdio)");
    println!("═══════════════════════════════════════════════════════════════\n");

    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();

            // Read what the child wrote
            let content = std::fs::read_to_string("/tmp/demo_fix.txt").unwrap_or_default();

            println!("\nResult with redirect_stdio():");
            println!("  File content: '{}'", content.trim());

            if content.contains("debug output") {
                println!("  ❌ UNEXPECTED: Debug leaked to file");
            } else {
                println!("  ✅ SUCCESS: File only contains intended data!");
                println!("  Debug output went to /dev/null (discarded safely)");
            }

            // Cleanup
            let _ = std::fs::remove_file("/tmp/demo_fix.txt");
        }
        Ok(Fork::Child) => {
            // Redirect stdio to /dev/null - THIS FIXES THE BUG
            redirect_stdio().unwrap();

            // Open a file - it will get fd >= 3
            let mut file = File::create("/tmp/demo_fix.txt").unwrap();
            let fd = file.as_raw_fd();

            println!("After redirect_stdio():");
            println!("  File got fd = {}", fd);
            println!("  fd 0,1,2 are now occupied by /dev/null");

            if fd >= 3 {
                println!("  ✅ GOOD: File got fd >= 3");
                println!("  Any println! or panic goes to /dev/null (safe)\n");

                // Try to write debug output - it goes to /dev/null!
                let debug_msg = b"This is debug output that goes to /dev/null\n";
                unsafe {
                    // This write goes to /dev/null, not to our file!
                    libc::write(2, debug_msg.as_ptr() as *const _, debug_msg.len());
                }
            }

            // Write intended data
            file.write_all(b"Expected data\n").unwrap();
            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}
