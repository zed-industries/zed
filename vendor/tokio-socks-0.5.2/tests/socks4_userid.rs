mod common;

use common::*;
#[cfg(feature = "futures-io")]
use tokio_socks::io::Compat;
use tokio_socks::{tcp::socks4::*, Result};

#[cfg(feature = "tokio")]
#[test]
fn connect_userid() -> Result<()> {
    let runtime = runtime().lock().unwrap();
    let conn = runtime.block_on(Socks4Stream::connect_with_userid(
        SOCKS4_PROXY_ADDR,
        ECHO_SERVER_ADDR,
        "mylogin",
    ))?;
    runtime.block_on(test_connect(conn))
}

#[cfg(feature = "tokio")]
#[test]
fn bind_userid() -> Result<()> {
    let bind = {
        let runtime = runtime().lock().unwrap();
        runtime.block_on(Socks4Listener::bind_with_userid(
            SOCKS4_PROXY_ADDR,
            ECHO_SERVER_ADDR,
            "mylogin",
        ))
    }?;
    test_bind_socks4(bind)
}

#[cfg(feature = "tokio")]
#[test]
fn connect_with_socket_userid() -> Result<()> {
    let runtime = runtime().lock().unwrap();
    let socket = runtime.block_on(connect_unix(UNIX_SOCKS4_PROXY_ADDR))?;
    let conn = runtime.block_on(Socks4Stream::connect_with_userid_and_socket(
        socket,
        ECHO_SERVER_ADDR,
        "mylogin",
    ))?;
    runtime.block_on(test_connect(conn))
}

#[cfg(feature = "tokio")]
#[test]
fn bind_with_socket_userid() -> Result<()> {
    let bind = {
        let runtime = runtime().lock().unwrap();
        let socket = runtime.block_on(connect_unix(UNIX_SOCKS4_PROXY_ADDR))?;
        runtime.block_on(Socks4Listener::bind_with_user_and_socket(
            socket,
            ECHO_SERVER_ADDR,
            "mylogin",
        ))
    }?;
    test_bind_socks4(bind)
}

#[cfg(feature = "futures-io")]
#[test]
fn connect_with_socket_userid_futures_io() -> Result<()> {
    let runtime = futures_utils::runtime().lock().unwrap();
    let socket = Compat::new(runtime.block_on(futures_utils::connect_unix(UNIX_SOCKS4_PROXY_ADDR))?);
    let conn = runtime.block_on(Socks4Stream::connect_with_userid_and_socket(
        socket,
        ECHO_SERVER_ADDR,
        "mylogin",
    ))?;
    runtime.block_on(futures_utils::test_connect(conn))
}

#[cfg(feature = "futures-io")]
#[test]
fn bind_with_socket_userid_futures_io() -> Result<()> {
    let bind = {
        let runtime = futures_utils::runtime().lock().unwrap();
        let socket = Compat::new(runtime.block_on(futures_utils::connect_unix(UNIX_SOCKS4_PROXY_ADDR))?);
        runtime.block_on(Socks4Listener::bind_with_user_and_socket(
            socket,
            ECHO_SERVER_ADDR,
            "mylogin",
        ))
    }?;
    futures_utils::test_bind_socks4(bind)
}
