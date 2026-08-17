/// run with `cargo run --example example_touch_pid`
use fork::{Fork, daemon};
use std::fs::OpenOptions;
use std::process::Command;

fn main() {
    match daemon(false, false) {
        Ok(Fork::Child) => {
            Command::new("sleep")
                .arg("300")
                .output()
                .expect("failed to execute process");
        }
        Ok(Fork::Parent(pid)) => {
            // touch file with name like pid
            let file_name = format!("/tmp/{}.pid", pid);
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_name)
                .expect("failed to open file");
        }
        Err(_) => {
            println!("Fork failed");
        }
    }
}
