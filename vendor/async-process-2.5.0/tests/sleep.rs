//! Sleep test.

use async_process::Command;
use futures_lite::future::block_on;

#[cfg(unix)]
#[test]
fn unix_sleep() {
    block_on(async {
        let status = Command::new("sleep").arg("1").status().await.unwrap();
        assert!(status.success());
    });
}

#[cfg(windows)]
#[test]
fn windows_sleep() {
    block_on(async {
        let status = Command::new("ping")
            .args(["-n", "5", "127.0.0.1"])
            .status()
            .await
            .unwrap();
        assert!(status.success());
    });
}
