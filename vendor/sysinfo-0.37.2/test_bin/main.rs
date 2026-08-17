// Does nothing and just exits after waiting for 30 seconds.

use std::process::{Child, Command};

fn maybe_start_child(last: String, args: &[String]) -> Option<Child> {
    if last == "1" {
        let mut cmd = Command::new(&args[0]);
        for arg in &args[1..] {
            cmd.arg(arg);
        }
        Some(cmd.spawn().expect("failed to run command"))
    } else {
        None
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let child = args.pop().and_then(|last| maybe_start_child(last, &args));
    if child.is_some() {
        std::thread::sleep(std::time::Duration::from_secs(3));
    } else {
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
