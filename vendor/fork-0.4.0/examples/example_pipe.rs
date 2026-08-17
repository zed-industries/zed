/// run with `cargo run --example example_pipe`
use fork::{Fork, fork, setsid};
use os_pipe::pipe;
use std::io::prelude::*;
use std::process::{Command, Stdio, exit};

fn main() {
    // Create a pipe for communication
    let (mut reader, writer) = pipe().expect("Failed to create pipe");

    match fork() {
        Ok(Fork::Child) => match fork() {
            Ok(Fork::Child) => {
                setsid().expect("Failed to setsid");
                match Command::new("sleep")
                    .arg("300")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                {
                    Ok(child) => {
                        println!("Child pid: {}", child.id());

                        // Write child pid to the pipe
                        let mut writer = writer; // Shadowing to prevent move errors
                        writeln!(writer, "{}", child.id()).expect("Failed to write to pipe");

                        exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error running command: {:?}", e);
                        exit(1);
                    }
                }
            }
            Ok(Fork::Parent(_)) => exit(0),
            Err(e) => {
                eprintln!("Error spawning process: {:?}", e);
                exit(1)
            }
        },
        Ok(Fork::Parent(_)) => {
            drop(writer);

            // Read the child pid from the pipe
            let mut child_pid_str = String::new();
            reader
                .read_to_string(&mut child_pid_str)
                .expect("Failed to read from pipe");

            if let Ok(child_pid) = child_pid_str.trim().parse::<i32>() {
                println!("Received child pid: {}", child_pid);
            } else {
                eprintln!("Failed to parse child pid");
            }
        }
        Err(e) => eprintln!("Error spawning process: {:?}", e),
    }
}
