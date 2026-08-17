use std::io::{BufRead, Write};
use std::process::Command;
use wasmtime_wasi::cli::StdinStream;
use wasmtime_wasi::p2::Pollable;

const VAR_NAME: &str = "__CHILD_PROCESS";

fn main() {
    if cfg!(miri) {
        return;
    }
    // Skip this tests if it looks like we're in a cross-compiled situation and
    // we're emulating this test for a different platform. In that scenario
    // emulators (like QEMU) tend to not report signals the same way and such.
    if wasmtime_test_util::cargo_test_runner().is_some() {
        return;
    }

    match std::env::var(VAR_NAME) {
        Ok(_) => child_process(),
        Err(_) => parent_process(),
    }

    fn child_process() {
        let mut result_write = std::io::stderr();
        let mut child_running = true;
        while child_running {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    'task: loop {
                        println!("child: creating stdin");
                        let mut stdin = wasmtime_wasi::cli::stdin().p2_stream();

                        println!("child: checking that stdin is not ready");
                        assert!(
                            tokio::time::timeout(
                                std::time::Duration::from_millis(100),
                                stdin.ready()
                            )
                            .await
                            .is_err(),
                            "stdin available too soon"
                        );

                        writeln!(&mut result_write, "start").unwrap();

                        println!("child: started");

                        let mut buffer = String::new();
                        loop {
                            println!("child: waiting for stdin to be ready");
                            stdin.ready().await;

                            println!("child: reading input");
                            // We can't effectively test for the case where stdin was closed, so panic if it is...
                            let bytes = stdin.read(1024).unwrap();

                            println!("child got: {bytes:?}");

                            buffer.push_str(std::str::from_utf8(bytes.as_ref()).unwrap());
                            if let Some((line, rest)) = buffer.split_once('\n') {
                                if line == "all done" {
                                    writeln!(&mut result_write, "done").unwrap();
                                    println!("child: exiting...");
                                    child_running = false;
                                    break 'task;
                                } else if line == "restart_runtime" {
                                    writeln!(&mut result_write, "restarting").unwrap();
                                    println!("child: restarting runtime...");
                                    break 'task;
                                } else if line == "restart_task" {
                                    writeln!(&mut result_write, "restarting").unwrap();
                                    println!("child: restarting task...");
                                    continue 'task;
                                } else {
                                    writeln!(&mut result_write, "{line}").unwrap();
                                }

                                buffer = rest.to_owned();
                            }
                        }
                    }
                });
            println!("child: runtime exited");
        }
        println!("child: exiting");
    }
}

fn parent_process() {
    let me = std::env::current_exe().unwrap();
    let mut cmd = Command::new(me);
    cmd.env(VAR_NAME, "1");
    cmd.stdin(std::process::Stdio::piped());

    if std::env::args().any(|arg| arg == "--nocapture") {
        cmd.stdout(std::process::Stdio::inherit());
    } else {
        cmd.stdout(std::process::Stdio::null());
    }

    cmd.stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn().unwrap();

    let mut stdin_write = child.stdin.take().unwrap();
    let mut result_read = std::io::BufReader::new(child.stderr.take().unwrap());

    let mut line = String::new();
    result_read.read_line(&mut line).unwrap();
    assert_eq!(line, "start\n");

    for i in 0..5 {
        let message = format!("some bytes {i}\n");
        stdin_write.write_all(message.as_bytes()).unwrap();
        line.clear();
        result_read.read_line(&mut line).unwrap();
        assert_eq!(line, message);
    }

    writeln!(&mut stdin_write, "restart_task").unwrap();
    line.clear();
    result_read.read_line(&mut line).unwrap();
    assert_eq!(line, "restarting\n");
    line.clear();

    result_read.read_line(&mut line).unwrap();
    assert_eq!(line, "start\n");

    for i in 0..10 {
        let message = format!("more bytes {i}\n");
        stdin_write.write_all(message.as_bytes()).unwrap();
        line.clear();
        result_read.read_line(&mut line).unwrap();
        assert_eq!(line, message);
    }

    writeln!(&mut stdin_write, "restart_runtime").unwrap();
    line.clear();
    result_read.read_line(&mut line).unwrap();
    assert_eq!(line, "restarting\n");
    line.clear();

    result_read.read_line(&mut line).unwrap();
    assert_eq!(line, "start\n");

    for i in 0..17 {
        let message = format!("even more bytes {i}\n");
        stdin_write.write_all(message.as_bytes()).unwrap();
        line.clear();
        result_read.read_line(&mut line).unwrap();
        assert_eq!(line, message);
    }

    writeln!(&mut stdin_write, "all done").unwrap();

    line.clear();
    result_read.read_line(&mut line).unwrap();
    assert_eq!(line, "done\n");
}
