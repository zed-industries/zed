#![cfg(not(target_os = "windows"))]

use ntest::timeout;
use test_log::test;

use zbus::{Result, block_on, conn::Builder};

#[test]
#[timeout(15000)]
fn unixexec_connection_async() {
    block_on(test_unixexec_connection()).unwrap();
}

async fn test_unixexec_connection() -> Result<()> {
    let connection = Builder::address("unixexec:path=systemd-stdio-bridge")?
        .build()
        .await?;

    match connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "Hello",
            &(),
        )
        .await
    {
        Err(zbus::Error::MethodError(_, _, _)) => (),
        Err(e) => panic!("{}", e),

        _ => panic!(),
    };

    Ok(())
}
