use std::time::Duration;

use async_std::io;
use async_std::task;

#[test]
#[should_panic(expected = "timed out")]
#[cfg(not(any(
    target_os = "unknown",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "x86",
)))] // stdin tests fail when running through cross
fn io_timeout_timedout() {
    task::block_on(async {
        io::timeout(Duration::from_secs(1), async {
            let stdin = io::stdin();
            let mut line = String::new();
            let _n = stdin.read_line(&mut line).await?;
            Ok(())
        })
        .await
        .unwrap(); // We should panic with a timeout error
    });
}

#[test]
#[should_panic(expected = "My custom error")]
fn io_timeout_future_err() {
    task::block_on(async {
        io::timeout(Duration::from_secs(1), async {
            Err::<(), io::Error>(io::Error::new(io::ErrorKind::Other, "My custom error"))
        })
        .await
        .unwrap(); // We should panic with our own error
    });
}

#[test]
fn io_timeout_future_ok() {
    task::block_on(async {
        io::timeout(Duration::from_secs(1), async { Ok(()) })
            .await
            .unwrap(); // We shouldn't panic at all
    });
}
