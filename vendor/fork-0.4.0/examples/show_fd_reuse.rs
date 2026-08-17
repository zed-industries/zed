/// Simple demonstration showing file descriptor reuse
///
/// Run with: cargo run --example show_fd_reuse
use fork::{Fork, close_fd, fork, redirect_stdio, waitpid};
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::process::exit;

fn main() {
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("DEMONSTRATION: File Descriptor Numbers");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Scenario 1: Using close_fd() - THE BUG");
    println!("───────────────────────────────────────────────────────────────");

    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();
        }
        Ok(Fork::Child) => {
            println!("\nBefore close_fd():");
            println!("  stdin  = fd 0");
            println!("  stdout = fd 1");
            println!("  stderr = fd 2");

            // Close stdio
            close_fd().unwrap();

            println!("\nAfter close_fd():");
            println!("  fd 0, 1, 2 are now FREE!\n");

            // Open files - they will get the freed fds
            let f1 = File::create("/tmp/test1.txt").unwrap();
            let f2 = File::create("/tmp/test2.txt").unwrap();
            let f3 = File::create("/tmp/test3.txt").unwrap();

            println!("Opening files:");
            println!("  test1.txt got fd = {} ⚠️  (was stdin!)", f1.as_raw_fd());
            println!("  test2.txt got fd = {} ⚠️  (was stdout!)", f2.as_raw_fd());
            println!("  test3.txt got fd = {} ⚠️  (was stderr!)", f3.as_raw_fd());

            println!("\nPROBLEM:");
            println!("  If code does println!() → writes to test2.txt!");
            println!("  If code panics → panic message goes to test3.txt!");
            println!("  → SILENT FILE CORRUPTION!\n");

            // Cleanup
            drop(f1);
            drop(f2);
            drop(f3);
            let _ = std::fs::remove_file("/tmp/test1.txt");
            let _ = std::fs::remove_file("/tmp/test2.txt");
            let _ = std::fs::remove_file("/tmp/test3.txt");

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("Scenario 2: Using redirect_stdio() - THE FIX");
    println!("───────────────────────────────────────────────────────────────");

    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();
        }
        Ok(Fork::Child) => {
            println!("\nBefore redirect_stdio():");
            println!("  stdin  = fd 0");
            println!("  stdout = fd 1");
            println!("  stderr = fd 2");

            // Redirect stdio to /dev/null
            redirect_stdio().unwrap();

            println!("\nAfter redirect_stdio():");
            println!("  fd 0 → /dev/null");
            println!("  fd 1 → /dev/null");
            println!("  fd 2 → /dev/null");
            println!("  (fds 0,1,2 remain OCCUPIED!)\n");

            // Open files - they will get higher fds
            let f1 = File::create("/tmp/test1.txt").unwrap();
            let f2 = File::create("/tmp/test2.txt").unwrap();
            let f3 = File::create("/tmp/test3.txt").unwrap();

            println!("Opening files:");
            println!("  test1.txt got fd = {} ✅ (safe!)", f1.as_raw_fd());
            println!("  test2.txt got fd = {} ✅ (safe!)", f2.as_raw_fd());
            println!("  test3.txt got fd = {} ✅ (safe!)", f3.as_raw_fd());

            println!("\nSOLUTION:");
            println!("  If code does println!() → goes to /dev/null (discarded)");
            println!("  If code panics → panic message goes to /dev/null");
            println!("  → Files are SAFE!\n");

            // Cleanup
            drop(f1);
            drop(f2);
            drop(f3);
            let _ = std::fs::remove_file("/tmp/test1.txt");
            let _ = std::fs::remove_file("/tmp/test2.txt");
            let _ = std::fs::remove_file("/tmp/test3.txt");

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("KEY INSIGHT:");
    println!("───────────────────────────────────────────────────────────────");
    println!("The OS kernel ALWAYS allocates the LOWEST available fd number.");
    println!();
    println!("close_fd():       Frees 0,1,2 → next open() returns 0,1,2");
    println!("redirect_stdio(): Keeps 0,1,2 busy → next open() returns 3,4,5");
    println!("═══════════════════════════════════════════════════════════════\n");
}
