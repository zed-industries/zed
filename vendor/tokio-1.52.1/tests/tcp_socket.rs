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

use std::time::Duration;
use tokio::net::TcpSocket;
use tokio_test::assert_ok;

#[tokio::test]
async fn basic_usage_v4() {
    // Create server
    let addr = assert_ok!("127.0.0.1:0".parse());
    let srv = assert_ok!(TcpSocket::new_v4());
    assert_ok!(srv.bind(addr));

    let srv = assert_ok!(srv.listen(128));

    // Create client & connect
    let addr = srv.local_addr().unwrap();
    let cli = assert_ok!(TcpSocket::new_v4());
    let _cli = assert_ok!(cli.connect(addr).await);

    // Accept
    let _ = assert_ok!(srv.accept().await);
}

#[tokio::test]
async fn basic_usage_v6() {
    // Create server
    let addr = assert_ok!("[::1]:0".parse());
    let srv = assert_ok!(TcpSocket::new_v6());
    assert_ok!(srv.bind(addr));

    let srv = assert_ok!(srv.listen(128));

    // Create client & connect
    let addr = srv.local_addr().unwrap();
    let cli = assert_ok!(TcpSocket::new_v6());
    let _cli = assert_ok!(cli.connect(addr).await);

    // Accept
    let _ = assert_ok!(srv.accept().await);
}

#[tokio::test]
async fn bind_before_connect() {
    // Create server
    let any_addr = assert_ok!("127.0.0.1:0".parse());
    let srv = assert_ok!(TcpSocket::new_v4());
    assert_ok!(srv.bind(any_addr));

    let srv = assert_ok!(srv.listen(128));

    // Create client & connect
    let addr = srv.local_addr().unwrap();
    let cli = assert_ok!(TcpSocket::new_v4());
    assert_ok!(cli.bind(any_addr));
    let _cli = assert_ok!(cli.connect(addr).await);

    // Accept
    let _ = assert_ok!(srv.accept().await);
}

#[cfg_attr(target_os = "wasi", ignore = "WASI does not yet support `SO_LINGER`")]
#[tokio::test]
async fn basic_linger() {
    // Create server
    let addr = assert_ok!("127.0.0.1:0".parse());
    let srv = assert_ok!(TcpSocket::new_v4());
    assert_ok!(srv.bind(addr));

    assert!(srv.linger().unwrap().is_none());

    srv.set_zero_linger().unwrap();
    assert_eq!(srv.linger().unwrap(), Some(Duration::new(0, 0)));
}

/// Macro to create a simple test to set and get a socket option.
macro_rules! test {
    // Test using the `arg`ument as expected return value.
    ($( #[ $attr: meta ] )* $get_fn: ident, $set_fn: ident ( $arg: expr ) ) => {
        test!($( #[$attr] )* $get_fn, $set_fn($arg), $arg);
    };
    ($( #[ $attr: meta ] )* $get_fn: ident, $set_fn: ident ( $arg: expr ), $expected: expr ) => {
        #[test]
        $( #[$attr] )*
        fn $get_fn() {
            test!(__ new_v4, $get_fn, $set_fn($arg), $expected);
            #[cfg(not(target_os = "vita"))]
            test!(__ new_v6, $get_fn, $set_fn($arg), $expected);
        }
    };
    // Only test using a IPv4 socket.
    (IPv4 $get_fn: ident, $set_fn: ident ( $arg: expr ) ) => {
        #[test]
        fn $get_fn() {
            test!(__ new_v4, $get_fn, $set_fn($arg), $arg);
        }
    };
    // Only test using a IPv6 socket.
    (IPv6 $get_fn: ident, $set_fn: ident ( $arg: expr ) ) => {
        #[test]
        fn $get_fn() {
            test!(__ new_v6, $get_fn, $set_fn($arg), $arg);
        }
    };

    // Internal to this macro.
    (__ $constructor: ident, $get_fn: ident, $set_fn: ident ( $arg: expr ), $expected: expr ) => {
        let socket = TcpSocket::$constructor().expect("failed to create `TcpSocket`");

        let initial = socket.$get_fn().expect("failed to get initial value");
        let arg = $arg;
        assert_ne!(initial, arg, "initial value and argument are the same");

        socket.$set_fn(arg).expect("failed to set option");
        let got = socket.$get_fn().expect("failed to get value");
        let expected = $expected;
        assert_eq!(got, expected, "set and get values differ");
    };
}

const SET_BUF_SIZE: u32 = 4096;
// Linux doubles the buffer size for kernel usage, and exposes that when
// retrieving the buffer size.

#[cfg(not(any(target_os = "android", target_os = "linux")))]
const GET_BUF_SIZE: u32 = SET_BUF_SIZE;

#[cfg(any(target_os = "android", target_os = "linux"))]
const GET_BUF_SIZE: u32 = 2 * SET_BUF_SIZE;

test!(keepalive, set_keepalive(true));

test!(reuseaddr, set_reuseaddr(true));

#[cfg(all(
    unix,
    not(target_os = "solaris"),
    not(target_os = "illumos"),
    not(target_os = "cygwin"),
))]
test!(reuseport, set_reuseport(true));

test!(
    send_buffer_size,
    set_send_buffer_size(SET_BUF_SIZE),
    GET_BUF_SIZE
);

test!(
    recv_buffer_size,
    set_recv_buffer_size(SET_BUF_SIZE),
    GET_BUF_SIZE
);

test!(
    #[cfg_attr(target_os = "wasi", ignore = "WASI does not yet support `SO_LINGER`")]
    #[expect(deprecated, reason = "set_linger is deprecated")]
    linger,
    set_linger(Some(Duration::from_secs(10)))
);

test!(nodelay, set_nodelay(true));

#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "cygwin",
))]
test!(IPv6 tclass_v6, set_tclass_v6(96));

#[cfg(not(any(
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "haiku",
    target_os = "wasi"
)))]
test!(IPv4 tos_v4, set_tos_v4(96));
