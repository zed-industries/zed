#![warn(rust_2018_idioms)]
// WASIp1 doesn't support bind
// No `socket` on miri.
#![cfg(all(
    feature = "net",
    feature = "macros",
    feature = "rt",
    feature = "io-util",
    not(all(target_os = "wasi", target_env = "p1")),
    not(miri)
))]

use tokio::net::TcpListener;

use std::net;

#[test]
#[should_panic]
#[cfg_attr(miri, ignore)] // No `socket` in miri.
fn no_runtime_panics_binding_net_tcp_listener() {
    let listener = net::TcpListener::bind("127.0.0.1:0").expect("failed to bind listener");
    let _ = TcpListener::try_from(listener);
}
