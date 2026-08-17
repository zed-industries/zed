//! These tests are borrowed from the `std::process` test suite.

use std::env;
use std::io;
use std::str;

use async_process::{Command, Output, Stdio};
use futures_lite::{future, prelude::*};

#[test]
fn smoke() {
    future::block_on(async {
        let p = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "exit 0"]).spawn()
        } else {
            Command::new("true").spawn()
        };
        assert!(p.is_ok());
        let mut p = p.unwrap();
        assert!(p.status().await.unwrap().success());
    })
}

#[test]
fn smoke_driven() {
    future::block_on(
        async {
            async_process::driver().await;
        }
        .or(async {
            let p = if cfg!(target_os = "windows") {
                Command::new("cmd").args(["/C", "exit 0"]).spawn()
            } else {
                Command::new("true").spawn()
            };
            assert!(p.is_ok());
            let mut p = p.unwrap();
            assert!(p.status().await.unwrap().success());
        }),
    )
}

#[test]
fn smoke_failure() {
    assert!(Command::new("if-this-is-a-binary-then-the-world-has-ended")
        .spawn()
        .is_err());
}

#[test]
fn exit_reported_right() {
    future::block_on(async {
        let p = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "exit 1"]).spawn()
        } else {
            Command::new("false").spawn()
        };
        assert!(p.is_ok());
        let mut p = p.unwrap();
        assert!(p.status().await.unwrap().code() == Some(1));
        drop(p.status().await);
    })
}

#[test]
#[cfg(unix)]
fn signal_reported_right() {
    use std::os::unix::process::ExitStatusExt;

    future::block_on(async {
        let mut p = Command::new("/bin/sh")
            .arg("-c")
            .arg("read a")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        p.kill().unwrap();
        match p.status().await.unwrap().signal() {
            Some(9) => {}
            result => panic!("not terminated by signal 9 (instead, {result:?})"),
        }
    })
}

pub async fn run_output(mut cmd: Command) -> String {
    let p = cmd.spawn();
    assert!(p.is_ok());
    let mut p = p.unwrap();
    assert!(p.stdout.is_some());
    let mut ret = String::new();
    p.stdout
        .as_mut()
        .unwrap()
        .read_to_string(&mut ret)
        .await
        .unwrap();
    assert!(p.status().await.unwrap().success());
    ret
}

#[test]
fn stdout_works() {
    future::block_on(async {
        if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", "echo foobar"]).stdout(Stdio::piped());
            assert_eq!(run_output(cmd).await, "foobar\r\n");
        } else {
            let mut cmd = Command::new("echo");
            cmd.arg("foobar").stdout(Stdio::piped());
            assert_eq!(run_output(cmd).await, "foobar\n");
        }
    })
}

#[test]
#[cfg_attr(windows, ignore)]
fn set_current_dir_works() {
    future::block_on(async {
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c")
            .arg("pwd")
            .current_dir("/")
            .stdout(Stdio::piped());
        assert_eq!(run_output(cmd).await, "/\n");
    })
}

#[test]
#[cfg_attr(windows, ignore)]
fn stdin_works() {
    future::block_on(async {
        let mut p = Command::new("/bin/sh")
            .arg("-c")
            .arg("read line; echo $line")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        p.stdin
            .as_mut()
            .unwrap()
            .write("foobar".as_bytes())
            .await
            .unwrap();
        drop(p.stdin.take());
        let mut out = String::new();
        p.stdout
            .as_mut()
            .unwrap()
            .read_to_string(&mut out)
            .await
            .unwrap();
        assert!(p.status().await.unwrap().success());
        assert_eq!(out, "foobar\n");
    })
}

#[test]
fn test_process_status() {
    future::block_on(async {
        let mut status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "exit 1"])
                .status()
                .await
                .unwrap()
        } else {
            Command::new("false").status().await.unwrap()
        };
        assert!(status.code() == Some(1));

        status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "exit 0"])
                .status()
                .await
                .unwrap()
        } else {
            Command::new("true").status().await.unwrap()
        };
        assert!(status.success());
    })
}

#[test]
fn test_process_output_fail_to_start() {
    future::block_on(async {
        match Command::new("/no-binary-by-this-name-should-exist")
            .output()
            .await
        {
            Err(e) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
            Ok(..) => panic!(),
        }
    })
}

#[test]
fn test_process_output_output() {
    future::block_on(async {
        let Output {
            status,
            stdout,
            stderr,
        } = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .await
                .unwrap()
        } else {
            Command::new("echo").arg("hello").output().await.unwrap()
        };
        let output_str = str::from_utf8(&stdout).unwrap();

        assert!(status.success());
        assert_eq!(output_str.trim().to_string(), "hello");
        assert_eq!(stderr, Vec::new());
    })
}

#[test]
fn test_process_output_error() {
    future::block_on(async {
        let Output {
            status,
            stdout,
            stderr,
        } = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "mkdir ."])
                .output()
                .await
                .unwrap()
        } else {
            Command::new("mkdir").arg("./").output().await.unwrap()
        };

        assert!(status.code() == Some(1));
        assert_eq!(stdout, Vec::new());
        assert!(!stderr.is_empty());
    })
}

#[test]
fn test_finish_once() {
    future::block_on(async {
        let mut prog = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "exit 1"]).spawn().unwrap()
        } else {
            Command::new("false").spawn().unwrap()
        };
        assert!(prog.status().await.unwrap().code() == Some(1));
    })
}

#[test]
fn test_finish_twice() {
    future::block_on(async {
        let mut prog = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "exit 1"]).spawn().unwrap()
        } else {
            Command::new("false").spawn().unwrap()
        };
        assert!(prog.status().await.unwrap().code() == Some(1));
        assert!(prog.status().await.unwrap().code() == Some(1));
    })
}

#[test]
fn test_wait_with_output_once() {
    future::block_on(async {
        let prog = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "echo hello"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
        } else {
            Command::new("echo")
                .arg("hello")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
        };

        let Output {
            status,
            stdout,
            stderr,
        } = prog.output().await.unwrap();
        let output_str = str::from_utf8(&stdout).unwrap();

        assert!(status.success());
        assert_eq!(output_str.trim().to_string(), "hello");
        assert_eq!(stderr, Vec::new());
    })
}

#[cfg(all(unix, not(target_os = "android")))]
pub fn env_cmd() -> Command {
    Command::new("env")
}

#[cfg(target_os = "android")]
pub fn env_cmd() -> Command {
    let mut cmd = Command::new("/system/bin/sh");
    cmd.arg("-c").arg("set");
    cmd
}

#[cfg(windows)]
pub fn env_cmd() -> Command {
    let mut cmd = Command::new("cmd");
    cmd.arg("/c").arg("set");
    cmd
}

#[test]
fn test_override_env() {
    future::block_on(async {
        // In some build environments (such as chrooted Nix builds), `env` can
        // only be found in the explicitly-provided PATH env variable, not in
        // default places such as /bin or /usr/bin. So we need to pass through
        // PATH to our sub-process.
        let mut cmd = env_cmd();
        cmd.env_clear().env("RUN_TEST_NEW_ENV", "123");
        if let Some(p) = env::var_os("PATH") {
            cmd.env("PATH", p);
        }
        let result = cmd.output().await.unwrap();
        let output = String::from_utf8_lossy(&result.stdout).to_string();

        assert!(
            output.contains("RUN_TEST_NEW_ENV=123"),
            "didn't find RUN_TEST_NEW_ENV inside of:\n\n{output}"
        );
    })
}

#[test]
fn test_add_to_env() {
    future::block_on(async {
        let result = env_cmd()
            .env("RUN_TEST_NEW_ENV", "123")
            .output()
            .await
            .unwrap();
        let output = String::from_utf8_lossy(&result.stdout).to_string();

        assert!(
            output.contains("RUN_TEST_NEW_ENV=123"),
            "didn't find RUN_TEST_NEW_ENV inside of:\n\n{output}"
        );
    })
}

#[test]
fn test_capture_env_at_spawn() {
    future::block_on(async {
        let mut cmd = env_cmd();
        cmd.env("RUN_TEST_NEW_ENV1", "123");

        // This variable will not be present if the environment has already
        // been captured above.
        env::set_var("RUN_TEST_NEW_ENV2", "456");
        let result = cmd.output().await.unwrap();
        env::remove_var("RUN_TEST_NEW_ENV2");

        let output = String::from_utf8_lossy(&result.stdout).to_string();

        assert!(
            output.contains("RUN_TEST_NEW_ENV1=123"),
            "didn't find RUN_TEST_NEW_ENV1 inside of:\n\n{output}"
        );
        assert!(
            output.contains("RUN_TEST_NEW_ENV2=456"),
            "didn't find RUN_TEST_NEW_ENV2 inside of:\n\n{output}"
        );
    })
}

#[test]
#[cfg(unix)]
fn child_status_preserved_with_kill_on_drop() {
    future::block_on(async {
        let p = Command::new("true").kill_on_drop(true).spawn().unwrap();

        // Calling output, since it takes ownership of the child
        // Child::status would work, but without special care,
        // dropping p inside of output would kill the subprocess early,
        // and report the wrong exit status
        let res = p.output().await;
        assert!(res.unwrap().status.success());
    })
}

#[test]
#[cfg(windows)]
fn child_as_raw_handle() {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::System::Threading::GetProcessId;

    future::block_on(async {
        let p = Command::new("cmd.exe")
            .arg("/C")
            .arg("pause")
            .kill_on_drop(true)
            .spawn()
            .unwrap();

        let std_pid = p.id();
        assert!(std_pid > 0);

        let handle = p.as_raw_handle();

        // We verify that we have the correct handle by obtaining the PID
        // with the Windows API rather than via std:
        let win_pid = unsafe { GetProcessId(handle as _) };
        assert_eq!(win_pid, std_pid);
    })
}

#[test]
#[cfg(unix)]
fn test_spawn_multiple_with_stdio() {
    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c")
        .arg("echo foo; echo bar 1>&2")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    future::block_on(async move {
        let p1 = cmd.spawn().unwrap();
        let out1 = p1.output().await.unwrap();
        assert_eq!(out1.stdout, b"foo\n");
        assert_eq!(out1.stderr, b"bar\n");

        let p2 = cmd.spawn().unwrap();
        let out2 = p2.output().await.unwrap();
        assert_eq!(out2.stdout, b"foo\n");
        assert_eq!(out2.stderr, b"bar\n");
    });
}

#[cfg(unix)]
#[test]
fn test_into_inner() {
    futures_lite::future::block_on(async {
        use crate::Command;

        use std::io::Result;
        use std::process::Stdio;
        use std::str::from_utf8;

        use futures_lite::AsyncReadExt;

        let mut ls_child = Command::new("cat")
            .arg("Cargo.toml")
            .stdout(Stdio::piped())
            .spawn()?;

        let stdio: Stdio = ls_child.stdout.take().unwrap().into_stdio().await?;

        let mut echo_child = Command::new("grep")
            .arg("async")
            .stdin(stdio)
            .stdout(Stdio::piped())
            .spawn()?;

        let mut buf = vec![];
        let mut stdout = echo_child.stdout.take().unwrap();

        stdout.read_to_end(&mut buf).await?;
        dbg!(from_utf8(&buf).unwrap_or(""));

        Result::Ok(())
    })
    .unwrap();
}
