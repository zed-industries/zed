#![cfg(not(target_os = "unknown"))]

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use async_std::net::ToSocketAddrs;
use async_std::task;

fn blocking_resolve<A>(a: A) -> Result<Vec<SocketAddr>, String>
where
    A: ToSocketAddrs,
    A::Iter: Send,
{
    let socket_addrs = task::block_on(a.to_socket_addrs());
    match socket_addrs {
        Ok(a) => Ok(a.collect()),
        Err(e) => Err(e.to_string()),
    }
}

#[test]
fn to_socket_addr_ipaddr_u16() {
    let a = Ipv4Addr::new(77, 88, 21, 11);
    let p = 12345;
    let e = SocketAddr::V4(SocketAddrV4::new(a, p));
    assert_eq!(Ok(vec![e]), blocking_resolve((a, p)));
}

#[test]
fn to_socket_addr_str_u16() {
    let a = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(77, 88, 21, 11), 24352));
    assert_eq!(Ok(vec![a]), blocking_resolve(("77.88.21.11", 24352)));

    let a = SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1),
        53,
        0,
        0,
    ));
    assert_eq!(Ok(vec![a]), blocking_resolve(("2a02:6b8:0:1::1", 53)));

    let a = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23924));
    #[cfg(not(target_env = "sgx"))]
    assert!(blocking_resolve(("localhost", 23924)).unwrap().contains(&a));
    #[cfg(target_env = "sgx")]
    let _ = a;
}

#[test]
fn to_socket_addr_str() {
    let a = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(77, 88, 21, 11), 24352));
    assert_eq!(Ok(vec![a]), blocking_resolve("77.88.21.11:24352"));

    let a = SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1),
        53,
        0,
        0,
    ));
    assert_eq!(Ok(vec![a]), blocking_resolve("[2a02:6b8:0:1::1]:53"));

    let a = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 23924));
    #[cfg(not(target_env = "sgx"))]
    assert!(blocking_resolve("localhost:23924").unwrap().contains(&a));
    #[cfg(target_env = "sgx")]
    let _ = a;
}

#[test]
fn to_socket_addr_string() {
    let a = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(77, 88, 21, 11), 24352));
    let s: &str = "77.88.21.11:24352";
    assert_eq!(Ok(vec![a]), blocking_resolve(s));

    let s: &String = &"77.88.21.11:24352".to_string();
    assert_eq!(Ok(vec![a]), blocking_resolve(s));

    let s: String = "77.88.21.11:24352".to_string();
    assert_eq!(Ok(vec![a]), blocking_resolve(s));
}

// FIXME: figure out why this fails on openbsd and fix it
#[test]
#[cfg(not(any(windows, target_os = "openbsd")))]
fn to_socket_addr_str_bad() {
    assert!(blocking_resolve("1200::AB00:1234::2552:7777:1313:34300").is_err());
}
