/// Visual demonstration of file descriptor reuse bug
///
/// This creates output files showing what fd numbers files receive
/// Run with: cargo run --example visual_fd_demo
use fork::{Fork, close_fd, fork, redirect_stdio, waitpid};
use std::fs::File;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::process::exit;

fn main() {
    println!("\nRunning demonstration...\n");

    demo_close_fd();
    demo_redirect_stdio();

    println!("═══════════════════════════════════════════════════════════════");
    println!("Results written to /tmp/fd_demo_*.txt");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Show results
    show_results();
}

fn demo_close_fd() {
    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();
        }
        Ok(Fork::Child) => {
            // Write to file before closing stdio (so we can see output)
            let mut report = File::create("/tmp/fd_demo_close_fd.txt").unwrap();

            writeln!(report, "SCENARIO 1: Using close_fd()").unwrap();
            writeln!(report, "══════════════════════════════════════").unwrap();
            writeln!(report).unwrap();

            // Close stdio
            close_fd().unwrap();

            writeln!(report, "After close_fd():").unwrap();
            writeln!(report, "  fd 0, 1, 2 are now FREE").unwrap();
            writeln!(report).unwrap();

            // Open test files
            let f1 = File::create("/tmp/fd_demo_file1.txt").unwrap();
            let f2 = File::create("/tmp/fd_demo_file2.txt").unwrap();
            let f3 = File::create("/tmp/fd_demo_file3.txt").unwrap();

            writeln!(report, "Opened files:").unwrap();
            writeln!(
                report,
                "  file1.txt → fd {} ⚠️  BUG: Reused stdin!",
                f1.as_raw_fd()
            )
            .unwrap();
            writeln!(
                report,
                "  file2.txt → fd {} ⚠️  BUG: Reused stdout!",
                f2.as_raw_fd()
            )
            .unwrap();
            writeln!(
                report,
                "  file3.txt → fd {} ⚠️  BUG: Reused stderr!",
                f3.as_raw_fd()
            )
            .unwrap();
            writeln!(report).unwrap();

            writeln!(report, "DANGER:").unwrap();
            writeln!(report, "  - println!() would write to file2.txt").unwrap();
            writeln!(report, "  - panic!() would write to file3.txt").unwrap();
            writeln!(report, "  - Silent file corruption!").unwrap();

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

fn demo_redirect_stdio() {
    match fork() {
        Ok(Fork::Parent(child)) => {
            waitpid(child).unwrap();
        }
        Ok(Fork::Child) => {
            // Write to file before redirecting (so we can see output)
            let mut report = File::create("/tmp/fd_demo_redirect_stdio.txt").unwrap();

            writeln!(report, "SCENARIO 2: Using redirect_stdio()").unwrap();
            writeln!(report, "══════════════════════════════════════").unwrap();
            writeln!(report).unwrap();

            // Redirect stdio to /dev/null
            redirect_stdio().unwrap();

            writeln!(report, "After redirect_stdio():").unwrap();
            writeln!(report, "  fd 0 → /dev/null").unwrap();
            writeln!(report, "  fd 1 → /dev/null").unwrap();
            writeln!(report, "  fd 2 → /dev/null").unwrap();
            writeln!(report, "  (fds 0,1,2 remain OCCUPIED)").unwrap();
            writeln!(report).unwrap();

            // Open test files
            let f1 = File::create("/tmp/fd_demo_file1_safe.txt").unwrap();
            let f2 = File::create("/tmp/fd_demo_file2_safe.txt").unwrap();
            let f3 = File::create("/tmp/fd_demo_file3_safe.txt").unwrap();

            writeln!(report, "Opened files:").unwrap();
            writeln!(
                report,
                "  file1.txt → fd {} ✅ SAFE: Higher fd!",
                f1.as_raw_fd()
            )
            .unwrap();
            writeln!(
                report,
                "  file2.txt → fd {} ✅ SAFE: Higher fd!",
                f2.as_raw_fd()
            )
            .unwrap();
            writeln!(
                report,
                "  file3.txt → fd {} ✅ SAFE: Higher fd!",
                f3.as_raw_fd()
            )
            .unwrap();
            writeln!(report).unwrap();

            writeln!(report, "SAFETY:").unwrap();
            writeln!(report, "  - println!() goes to /dev/null (discarded)").unwrap();
            writeln!(report, "  - panic!() goes to /dev/null (discarded)").unwrap();
            writeln!(report, "  - Files are protected!").unwrap();

            exit(0);
        }
        Err(_) => panic!("Fork failed"),
    }
}

fn show_results() {
    println!("BUG (close_fd):");
    println!("───────────────────────────────────────────────────────────────");
    if let Ok(content) = std::fs::read_to_string("/tmp/fd_demo_close_fd.txt") {
        print!("{}", content);
    }

    println!("\n");
    println!("FIX (redirect_stdio):");
    println!("───────────────────────────────────────────────────────────────");
    if let Ok(content) = std::fs::read_to_string("/tmp/fd_demo_redirect_stdio.txt") {
        print!("{}", content);
    }

    // Cleanup
    let _ = std::fs::remove_file("/tmp/fd_demo_close_fd.txt");
    let _ = std::fs::remove_file("/tmp/fd_demo_redirect_stdio.txt");
    let _ = std::fs::remove_file("/tmp/fd_demo_file1.txt");
    let _ = std::fs::remove_file("/tmp/fd_demo_file2.txt");
    let _ = std::fs::remove_file("/tmp/fd_demo_file3.txt");
    let _ = std::fs::remove_file("/tmp/fd_demo_file1_safe.txt");
    let _ = std::fs::remove_file("/tmp/fd_demo_file2_safe.txt");
    let _ = std::fs::remove_file("/tmp/fd_demo_file3_safe.txt");
}
