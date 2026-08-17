#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]
#![cfg(not(miri))] // No socket in miri.

use std::io;
use tokio::net::TcpListener;
use tokio::runtime::Builder;

fn rt() -> tokio::runtime::Runtime {
    Builder::new_current_thread().enable_all().build().unwrap()
}

#[test]
fn test_is_rt_shutdown_err() {
    let rt1 = rt();
    let rt2 = rt();

    let listener = rt1.block_on(async { TcpListener::bind("127.0.0.1:0").await.unwrap() });

    drop(rt1);

    rt2.block_on(async {
        let res = listener.accept().await;
        assert!(res.is_err());
        let err = res.as_ref().unwrap_err();
        assert!(tokio::runtime::is_rt_shutdown_err(err));
    });
}

#[test]
fn test_is_not_rt_shutdown_err() {
    let err = io::Error::new(io::ErrorKind::Other, "some other error");
    assert!(!tokio::runtime::is_rt_shutdown_err(&err));

    let err = io::Error::new(io::ErrorKind::NotFound, "not found");
    assert!(!tokio::runtime::is_rt_shutdown_err(&err));
}

#[test]
#[cfg_attr(panic = "abort", ignore)]
fn test_join_error_panic() {
    let rt = rt();
    let handle = rt.spawn(async {
        panic!("oops");
    });

    let join_err = rt.block_on(handle).unwrap_err();
    let io_err: io::Error = join_err.into();
    assert!(!tokio::runtime::is_rt_shutdown_err(&io_err));
}

#[test]
fn test_join_error_cancelled() {
    let rt = rt();
    let handle = rt.spawn(async {
        std::future::pending::<()>().await;
    });
    handle.abort();
    let join_err = rt.block_on(handle).unwrap_err();
    let io_err: io::Error = join_err.into();
    assert!(!tokio::runtime::is_rt_shutdown_err(&io_err));
}

#[test]
fn test_other_error_kinds_and_strings() {
    // TimedOut
    let err = io::Error::new(io::ErrorKind::TimedOut, "timed out");
    assert!(!tokio::runtime::is_rt_shutdown_err(&err));

    // Interrupted
    let err = io::Error::from(io::ErrorKind::Interrupted);
    assert!(!tokio::runtime::is_rt_shutdown_err(&err));

    // String that contains the shutdown message but has a prefix/suffix
    let msg = "A Tokio 1.x context was found, but it is being shutdown. (extra info)";
    let err = io::Error::new(io::ErrorKind::Other, msg);
    assert!(!tokio::runtime::is_rt_shutdown_err(&err));

    let msg = "Error: A Tokio 1.x context was found, but it is being shutdown.";
    let err = io::Error::new(io::ErrorKind::Other, msg);
    assert!(!tokio::runtime::is_rt_shutdown_err(&err));
}
