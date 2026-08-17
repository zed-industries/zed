//! Tests for `Child::adopt_raw_pid`.

#![cfg(target_os = "macos")]

use async_process::Child;
use futures_lite::future::block_on;

#[test]
fn adopt_and_wait() {
    block_on(async {
        let child = std::process::Command::new("sh")
            .arg("-c")
            .arg("exit 7")
            .spawn()
            .unwrap();
        let pid = child.id();
        // Dropping a `std::process::Child` does not wait on the process, so the
        // adoption safety contract is upheld.
        drop(child);

        let mut adopted = unsafe { Child::adopt_raw_pid(pid, true, false) }.unwrap();
        assert_eq!(adopted.id(), pid);
        let status = adopted.status().await.unwrap();
        assert_eq!(status.code(), Some(7));

        // The status must be cached rather than waited for again.
        assert_eq!(adopted.try_status().unwrap().unwrap().code(), Some(7));
    });
}

#[test]
fn adopt_and_kill() {
    block_on(async {
        let child = std::process::Command::new("sleep")
            .arg("60")
            .spawn()
            .unwrap();
        let pid = child.id();
        drop(child);

        let mut adopted = unsafe { Child::adopt_raw_pid(pid, true, false) }.unwrap();
        assert_eq!(adopted.try_status().unwrap(), None);
        adopted.kill().unwrap();
        let status = adopted.status().await.unwrap();
        assert!(!status.success());
    });
}

#[test]
fn adopt_invalid_pid() {
    assert!(unsafe { Child::adopt_raw_pid(0, true, false) }.is_err());
    assert!(unsafe { Child::adopt_raw_pid(u32::MAX, true, false) }.is_err());
}
