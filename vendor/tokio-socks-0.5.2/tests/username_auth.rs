mod common;

use common::*;
#[cfg(feature = "futures-io")]
use tokio_socks::io::Compat;
use tokio_socks::{
    tcp::socks5::{Socks5Listener, Socks5Stream},
    Result,
};

#[cfg(feature = "tokio")]
#[test]
fn connect_username_auth() -> Result<()> {
    let runtime = runtime().lock().unwrap();
    let conn = runtime.block_on(Socks5Stream::connect_with_password(
        PROXY_ADDR,
        ECHO_SERVER_ADDR,
        "mylogin",
        "mypassword",
    ))?;
    runtime.block_on(test_connect(conn))
}

#[cfg(feature = "tokio")]
#[test]
fn bind_username_auth() -> Result<()> {
    let bind = {
        let runtime = runtime().lock().unwrap();
        runtime.block_on(Socks5Listener::bind_with_password(
            PROXY_ADDR,
            ECHO_SERVER_ADDR,
            "mylogin",
            "mypassword",
        ))
    }?;
    test_bind(bind)
}

#[cfg(feature = "tokio")]
#[test]
fn connect_with_socket_username_auth() -> Result<()> {
    let runtime = runtime().lock().unwrap();
    let socket = runtime.block_on(connect_unix(UNIX_PROXY_ADDR))?;
    let conn = runtime.block_on(Socks5Stream::connect_with_password_and_socket(
        socket,
        ECHO_SERVER_ADDR,
        "mylogin",
        "mypassword",
    ))?;
    runtime.block_on(test_connect(conn))
}

#[cfg(feature = "tokio")]
#[test]
fn bind_with_socket_username_auth() -> Result<()> {
    let bind = {
        let runtime = runtime().lock().unwrap();
        let socket = runtime.block_on(connect_unix(UNIX_PROXY_ADDR))?;
        runtime.block_on(Socks5Listener::bind_with_password_and_socket(
            socket,
            ECHO_SERVER_ADDR,
            "mylogin",
            "mypassword",
        ))
    }?;
    test_bind(bind)
}

#[cfg(feature = "futures-io")]
#[test]
fn connect_with_socket_username_auth_futures_io() -> Result<()> {
    let runtime = futures_utils::runtime().lock().unwrap();
    let socket = Compat::new(runtime.block_on(futures_utils::connect_unix(UNIX_PROXY_ADDR))?);
    let conn = runtime.block_on(Socks5Stream::connect_with_password_and_socket(
        socket,
        ECHO_SERVER_ADDR,
        "mylogin",
        "mypassword",
    ))?;
    runtime.block_on(futures_utils::test_connect(conn))
}

#[cfg(feature = "futures-io")]
#[test]
fn bind_with_socket_username_auth_futures_io() -> Result<()> {
    let bind = {
        let runtime = futures_utils::runtime().lock().unwrap();
        let socket = Compat::new(runtime.block_on(futures_utils::connect_unix(UNIX_PROXY_ADDR))?);
        runtime.block_on(Socks5Listener::bind_with_password_and_socket(
            socket,
            ECHO_SERVER_ADDR,
            "mylogin",
            "mypassword",
        ))
    }?;
    futures_utils::test_bind(bind)
}
