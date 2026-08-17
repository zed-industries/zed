// Copyright 2021, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use command_fds::{CommandFdExt, FdMapping};
use std::fs::{File, read_dir, read_link};
use std::io::stdin;
use std::os::fd::AsFd;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// Print out a list of all open file descriptors.
fn list_fds() {
    let dir = read_dir("/proc/self/fd").unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let target = read_link(entry.path()).unwrap();
        println!("{:?} {:?}", entry, target);
    }
}

fn main() {
    list_fds();

    // Open a file.
    let file = File::open("Cargo.toml").unwrap();
    println!("File: {:?}", file);
    list_fds();

    // Prepare to run `ls -l /proc/self/fd` with some FDs mapped.
    let mut command = Command::new("ls");
    let stdin = stdin().as_fd().try_clone_to_owned().unwrap();
    command.arg("-l").arg("/proc/self/fd");
    command
        .fd_mappings(vec![
            // Map `file` as FD 3 in the child process.
            FdMapping {
                parent_fd: file.into(),
                child_fd: 3,
            },
            // Map this process's stdin as FD 5 in the child process.
            FdMapping {
                parent_fd: stdin,
                child_fd: 5,
            },
        ])
        .unwrap();
    unsafe {
        command.pre_exec(move || {
            println!("pre_exec");
            list_fds();
            Ok(())
        });
    }

    // Spawn the child process.
    println!("Spawning command");
    let mut child = command.spawn().unwrap();
    sleep(Duration::from_millis(100));
    println!("Spawned");
    list_fds();

    println!("Waiting for command");
    println!("{:?}", child.wait().unwrap());
}
