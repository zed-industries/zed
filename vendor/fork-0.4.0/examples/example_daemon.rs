/// run with `cargo run --example example_daemon`
use fork::{Fork, daemon};
use std::process::Command;

fn main() {
    // Keep file descriptors open to print the pid of the daemon
    match daemon(false, true) {
        Ok(Fork::Child) => {
            Command::new("sleep")
                .arg("300")
                .output()
                .expect("failed to execute process");
        }
        Ok(Fork::Parent(pid)) => {
            println!("daemon pid: {}", pid);
        }
        Err(_) => {
            println!("Fork failed");
        }
    }
}
