// WASIp1 doesn't support direct socket operations
#![cfg(all(
    feature = "net",
    feature = "macros",
    feature = "rt",
    feature = "io-util",
    not(all(target_os = "wasi", target_env = "p1")),
))]

use tokio::net;
use tokio_test::assert_ok;

use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

#[tokio::test]
async fn lookup_socket_addr() {
    let addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();

    let actual = assert_ok!(net::lookup_host(addr).await).collect::<Vec<_>>();
    assert_eq!(vec![addr], actual);
}

#[tokio::test]
async fn lookup_str_socket_addr() {
    let addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();

    let actual = assert_ok!(net::lookup_host("127.0.0.1:8000").await).collect::<Vec<_>>();
    assert_eq!(vec![addr], actual);
}

// Note that WASIp2 _does_ support asynchronous name lookups without requiring a
// worker thread, so this test could be ungated for WASI if/when that's
// implemented.
#[cfg_attr(
    target_os = "wasi",
    ignore = "net::lookup_host requires multithreading, which WASI does not yet support"
)]
#[tokio::test]
#[cfg_attr(miri, ignore)] // No `getaddrinfo` in miri.
async fn resolve_dns() -> io::Result<()> {
    let mut hosts = net::lookup_host("localhost:3000").await?;
    let host = hosts.next().unwrap();

    let expected = if host.is_ipv4() {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000)
    } else {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 3000)
    };
    assert_eq!(host, expected);

    Ok(())
}
