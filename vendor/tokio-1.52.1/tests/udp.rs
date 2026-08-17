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

use std::future::poll_fn;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::{io::ReadBuf, net::UdpSocket, time};
use tokio_test::assert_ok;

const MSG: &[u8] = b"hello";
const MSG_LEN: usize = MSG.len();

#[tokio::test]
async fn send_recv() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    sender.connect(receiver.local_addr()?).await?;
    receiver.connect(sender.local_addr()?).await?;

    sender.send(MSG).await?;

    let mut recv_buf = [0u8; 32];
    let len = receiver.recv(&mut recv_buf[..]).await?;

    assert_eq!(&recv_buf[..len], MSG);
    Ok(())
}

#[tokio::test]
async fn send_recv_poll() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    sender.connect(receiver.local_addr()?).await?;
    receiver.connect(sender.local_addr()?).await?;

    poll_fn(|cx| sender.poll_send(cx, MSG)).await?;

    let mut recv_buf = [0u8; 32];
    let mut read = ReadBuf::new(&mut recv_buf);
    poll_fn(|cx| receiver.poll_recv(cx, &mut read)).await?;

    assert_eq!(read.filled(), MSG);
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    target_os = "wasi",
    ignore = "temporarily disabled for WASI pending https://github.com/WebAssembly/wasi-libc/pull/734"
)]
async fn send_to_recv_closed_returns_err() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let receiver_addr = receiver.local_addr()?;
    drop(receiver);
    sender.connect(receiver_addr).await?;
    sender.send(MSG).await?;

    let mut recv_buf = [0u8; 32];
    let err = time::timeout(Duration::from_secs(5), sender.recv(&mut recv_buf))
        .await
        .expect("timed out instead of returning error")
        .unwrap_err();
    let errno = err.kind();

    assert!(
        // Linux/BSD returns ECONNREFUSED, but Windows will usually return ECONNRESET instead.
        matches!(
            errno,
            io::ErrorKind::ConnectionRefused | io::ErrorKind::ConnectionReset
        )
    );
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    target_os = "wasi",
    ignore = "temporarily disabled for WASI pending https://github.com/WebAssembly/wasi-libc/pull/734"
)]
async fn send_to_recv_from() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let receiver_addr = receiver.local_addr()?;
    sender.send_to(MSG, &receiver_addr).await?;

    let mut recv_buf = [0u8; 32];
    let (len, addr) = receiver.recv_from(&mut recv_buf[..]).await?;

    assert_eq!(&recv_buf[..len], MSG);
    assert_eq!(addr, sender.local_addr()?);
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    target_os = "wasi",
    ignore = "temporarily disabled for WASI pending https://github.com/WebAssembly/wasi-libc/pull/734"
)]
async fn send_to_recv_from_poll() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let receiver_addr = receiver.local_addr()?;
    poll_fn(|cx| sender.poll_send_to(cx, MSG, receiver_addr)).await?;

    let mut recv_buf = [0u8; 32];
    let mut read = ReadBuf::new(&mut recv_buf);
    let addr = poll_fn(|cx| receiver.poll_recv_from(cx, &mut read)).await?;

    assert_eq!(read.filled(), MSG);
    assert_eq!(addr, sender.local_addr()?);
    Ok(())
}

#[cfg_attr(target_os = "wasi", ignore = "WASI does not yet support peeking")]
#[tokio::test]
async fn send_to_peek_from() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let receiver_addr = receiver.local_addr()?;
    poll_fn(|cx| sender.poll_send_to(cx, MSG, receiver_addr)).await?;

    // peek
    let mut recv_buf = [0u8; 32];
    let (n, addr) = receiver.peek_from(&mut recv_buf).await?;
    assert_eq!(&recv_buf[..n], MSG);
    assert_eq!(addr, sender.local_addr()?);

    // peek
    let mut recv_buf = [0u8; 32];
    let (n, addr) = receiver.peek_from(&mut recv_buf).await?;
    assert_eq!(&recv_buf[..n], MSG);
    assert_eq!(addr, sender.local_addr()?);

    let mut recv_buf = [0u8; 32];
    let (n, addr) = receiver.recv_from(&mut recv_buf).await?;
    assert_eq!(&recv_buf[..n], MSG);
    assert_eq!(addr, sender.local_addr()?);

    Ok(())
}

#[cfg_attr(target_os = "wasi", ignore = "WASI does not yet support peeking")]
#[tokio::test]
async fn send_to_try_peek_from() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let receiver_addr = receiver.local_addr()?;
    poll_fn(|cx| sender.poll_send_to(cx, MSG, receiver_addr)).await?;

    // peek
    let mut recv_buf = [0u8; 32];

    loop {
        match receiver.try_peek_from(&mut recv_buf) {
            Ok((n, addr)) => {
                assert_eq!(&recv_buf[..n], MSG);
                assert_eq!(addr, sender.local_addr()?);
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                receiver.readable().await?;
            }
            Err(e) => return Err(e),
        }
    }

    // peek
    let mut recv_buf = [0u8; 32];
    let (n, addr) = receiver.peek_from(&mut recv_buf).await?;
    assert_eq!(&recv_buf[..n], MSG);
    assert_eq!(addr, sender.local_addr()?);

    let mut recv_buf = [0u8; 32];
    let (n, addr) = receiver.recv_from(&mut recv_buf).await?;
    assert_eq!(&recv_buf[..n], MSG);
    assert_eq!(addr, sender.local_addr()?);

    Ok(())
}

#[cfg_attr(target_os = "wasi", ignore = "WASI does not yet support peeking")]
#[tokio::test]
async fn send_to_peek_from_poll() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let receiver_addr = receiver.local_addr()?;
    poll_fn(|cx| sender.poll_send_to(cx, MSG, receiver_addr)).await?;

    let mut recv_buf = [0u8; 32];
    let mut read = ReadBuf::new(&mut recv_buf);
    let addr = poll_fn(|cx| receiver.poll_peek_from(cx, &mut read)).await?;

    assert_eq!(read.filled(), MSG);
    assert_eq!(addr, sender.local_addr()?);

    let mut recv_buf = [0u8; 32];
    let mut read = ReadBuf::new(&mut recv_buf);
    poll_fn(|cx| receiver.poll_peek_from(cx, &mut read)).await?;

    assert_eq!(read.filled(), MSG);
    let mut recv_buf = [0u8; 32];
    let mut read = ReadBuf::new(&mut recv_buf);

    poll_fn(|cx| receiver.poll_recv_from(cx, &mut read)).await?;
    assert_eq!(read.filled(), MSG);
    Ok(())
}

#[cfg_attr(target_os = "wasi", ignore = "WASI does not yet support peeking")]
#[tokio::test]
async fn peek_sender() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let sender_addr = sender.local_addr()?;
    let receiver_addr = receiver.local_addr()?;

    let msg = b"Hello, world!";
    sender.send_to(msg, receiver_addr).await?;

    let peeked_sender = receiver.peek_sender().await?;
    assert_eq!(peeked_sender, sender_addr);

    // Assert that `peek_sender()` returns the right sender but
    // doesn't remove from the receive queue.
    let mut recv_buf = [0u8; 32];
    let (read, received_sender) = receiver.recv_from(&mut recv_buf).await?;

    assert_eq!(&recv_buf[..read], msg);
    assert_eq!(received_sender, peeked_sender);

    Ok(())
}

#[cfg_attr(target_os = "wasi", ignore = "WASI does not yet support peeking")]
#[tokio::test]
async fn poll_peek_sender() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let sender_addr = sender.local_addr()?;
    let receiver_addr = receiver.local_addr()?;

    let msg = b"Hello, world!";
    poll_fn(|cx| sender.poll_send_to(cx, msg, receiver_addr)).await?;

    let peeked_sender = poll_fn(|cx| receiver.poll_peek_sender(cx)).await?;
    assert_eq!(peeked_sender, sender_addr);

    // Assert that `poll_peek_sender()` returns the right sender but
    // doesn't remove from the receive queue.
    let mut recv_buf = [0u8; 32];
    let mut read = ReadBuf::new(&mut recv_buf);
    let received_sender = poll_fn(|cx| receiver.poll_recv_from(cx, &mut read)).await?;

    assert_eq!(read.filled(), msg);
    assert_eq!(received_sender, peeked_sender);

    Ok(())
}

#[cfg_attr(target_os = "wasi", ignore = "WASI does not yet support peeking")]
#[tokio::test]
async fn try_peek_sender() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let sender_addr = sender.local_addr()?;
    let receiver_addr = receiver.local_addr()?;

    let msg = b"Hello, world!";
    sender.send_to(msg, receiver_addr).await?;

    let peeked_sender = loop {
        match receiver.try_peek_sender() {
            Ok(peeked_sender) => break peeked_sender,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                receiver.readable().await?;
            }
            Err(e) => return Err(e),
        }
    };

    assert_eq!(peeked_sender, sender_addr);

    // Assert that `try_peek_sender()` returns the right sender but
    // didn't remove from the receive queue.
    let mut recv_buf = [0u8; 32];
    // We already peeked the sender so there must be data in the receive queue.
    let (read, received_sender) = receiver.try_recv_from(&mut recv_buf).unwrap();

    assert_eq!(&recv_buf[..read], msg);
    assert_eq!(received_sender, peeked_sender);

    Ok(())
}

#[tokio::test]
async fn split() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let s = Arc::new(socket);
    let r = s.clone();

    let addr = s.local_addr()?;
    tokio::spawn(async move {
        s.send_to(MSG, &addr).await.unwrap();
    });
    let mut recv_buf = [0u8; 32];
    let (len, _) = r.recv_from(&mut recv_buf[..]).await?;
    assert_eq!(&recv_buf[..len], MSG);
    Ok(())
}

#[tokio::test]
async fn split_chan() -> std::io::Result<()> {
    // setup UdpSocket that will echo all sent items
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let addr = socket.local_addr().unwrap();
    let s = Arc::new(socket);
    let r = s.clone();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<(Vec<u8>, std::net::SocketAddr)>(1_000);
    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            s.send_to(&bytes, &addr).await.unwrap();
        }
    });

    tokio::spawn(async move {
        let mut buf = [0u8; 32];
        loop {
            let (len, addr) = r.recv_from(&mut buf).await.unwrap();
            tx.send((buf[..len].to_vec(), addr)).await.unwrap();
        }
    });

    // test that we can send a value and get back some response
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    sender.send_to(MSG, addr).await?;
    let mut recv_buf = [0u8; 32];
    let (len, _) = sender.recv_from(&mut recv_buf).await?;
    assert_eq!(&recv_buf[..len], MSG);
    Ok(())
}

#[tokio::test]
async fn split_chan_poll() -> std::io::Result<()> {
    // setup UdpSocket that will echo all sent items
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let addr = socket.local_addr().unwrap();
    let s = Arc::new(socket);
    let r = s.clone();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<(Vec<u8>, std::net::SocketAddr)>(1_000);
    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            poll_fn(|cx| s.poll_send_to(cx, &bytes, addr))
                .await
                .unwrap();
        }
    });

    tokio::spawn(async move {
        let mut recv_buf = [0u8; 32];
        let mut read = ReadBuf::new(&mut recv_buf);
        loop {
            let addr = poll_fn(|cx| r.poll_recv_from(cx, &mut read)).await.unwrap();
            tx.send((read.filled().to_vec(), addr)).await.unwrap();
        }
    });

    // test that we can send a value and get back some response
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    poll_fn(|cx| sender.poll_send_to(cx, MSG, addr)).await?;

    let mut recv_buf = [0u8; 32];
    let mut read = ReadBuf::new(&mut recv_buf);
    let _ = poll_fn(|cx| sender.poll_recv_from(cx, &mut read)).await?;
    assert_eq!(read.filled(), MSG);
    Ok(())
}

// # Note
//
// This test is purposely written such that each time `sender` sends data on
// the socket, `receiver` awaits the data. On Unix, it would be okay waiting
// until the end of the test to receive all the data. On Windows, this would
// **not** be okay because it's resources are completion based (via IOCP).
// If data is sent and not yet received, attempting to send more data will
// result in `ErrorKind::WouldBlock` until the first operation completes.
#[cfg_attr(
    target_os = "wasi",
    ignore = "WASI does not yet support multithreading"
)]
#[tokio::test]
async fn try_send_spawn() {
    const MSG2: &[u8] = b"world!";
    const MSG2_LEN: usize = MSG2.len();

    let sender = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let receiver = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    receiver
        .connect(sender.local_addr().unwrap())
        .await
        .unwrap();

    sender.writable().await.unwrap();

    let sent = &sender
        .try_send_to(MSG, receiver.local_addr().unwrap())
        .unwrap();
    assert_eq!(sent, &MSG_LEN);
    let mut buf = [0u8; 32];
    let mut received = receiver.recv(&mut buf[..]).await.unwrap();

    sender
        .connect(receiver.local_addr().unwrap())
        .await
        .unwrap();
    let sent = &sender.try_send(MSG2).unwrap();
    assert_eq!(sent, &MSG2_LEN);
    received += receiver.recv(&mut buf[..]).await.unwrap();

    std::thread::spawn(move || {
        let sent = &sender.try_send(MSG).unwrap();
        assert_eq!(sent, &MSG_LEN);
    })
    .join()
    .unwrap();
    received += receiver.recv(&mut buf[..]).await.unwrap();

    assert_eq!(received, MSG_LEN * 2 + MSG2_LEN);
}

#[tokio::test]
async fn try_send_recv() {
    // Create listener
    let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    // Create socket pair
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    // Connect the two
    client.connect(server.local_addr().unwrap()).await.unwrap();
    server.connect(client.local_addr().unwrap()).await.unwrap();

    for _ in 0..5 {
        loop {
            client.writable().await.unwrap();

            match client.try_send(b"hello world") {
                Ok(n) => {
                    assert_eq!(n, 11);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }

        loop {
            server.readable().await.unwrap();

            let mut buf = [0; 512];

            match server.try_recv(&mut buf) {
                Ok(n) => {
                    assert_eq!(n, 11);
                    assert_eq!(&buf[0..11], &b"hello world"[..]);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }
    }
}

#[tokio::test]
#[cfg_attr(
    target_os = "wasi",
    ignore = "temporarily disabled for WASI pending https://github.com/WebAssembly/wasi-libc/pull/734"
)]
async fn try_send_to_recv_from() {
    // Create listener
    let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let saddr = server.local_addr().unwrap();

    // Create socket pair
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let caddr = client.local_addr().unwrap();

    for _ in 0..5 {
        loop {
            client.writable().await.unwrap();

            match client.try_send_to(b"hello world", saddr) {
                Ok(n) => {
                    assert_eq!(n, 11);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }

        loop {
            server.readable().await.unwrap();

            let mut buf = [0; 512];

            match server.try_recv_from(&mut buf) {
                Ok((n, addr)) => {
                    assert_eq!(n, 11);
                    assert_eq!(addr, caddr);
                    assert_eq!(&buf[0..11], &b"hello world"[..]);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }
    }
}

#[tokio::test]
async fn try_recv_buf() {
    // Create listener
    let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    // Create socket pair
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    // Connect the two
    client.connect(server.local_addr().unwrap()).await.unwrap();
    server.connect(client.local_addr().unwrap()).await.unwrap();

    for _ in 0..5 {
        loop {
            client.writable().await.unwrap();

            match client.try_send(b"hello world") {
                Ok(n) => {
                    assert_eq!(n, 11);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }

        loop {
            server.readable().await.unwrap();

            let mut buf = Vec::with_capacity(512);

            match server.try_recv_buf(&mut buf) {
                Ok(n) => {
                    assert_eq!(n, 11);
                    assert_eq!(&buf[0..11], &b"hello world"[..]);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }
    }
}

#[tokio::test]
async fn recv_buf() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    sender.connect(receiver.local_addr()?).await?;
    receiver.connect(sender.local_addr()?).await?;

    sender.send(MSG).await?;
    let mut recv_buf = Vec::with_capacity(32);
    let len = receiver.recv_buf(&mut recv_buf).await?;

    assert_eq!(len, MSG_LEN);
    assert_eq!(&recv_buf[..len], MSG);
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    target_os = "wasi",
    ignore = "temporarily disabled for WASI pending https://github.com/WebAssembly/wasi-libc/pull/734"
)]
async fn try_recv_buf_from() {
    // Create listener
    let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let saddr = server.local_addr().unwrap();

    // Create socket pair
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let caddr = client.local_addr().unwrap();

    for _ in 0..5 {
        loop {
            client.writable().await.unwrap();

            match client.try_send_to(b"hello world", saddr) {
                Ok(n) => {
                    assert_eq!(n, 11);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }

        loop {
            server.readable().await.unwrap();

            let mut buf = Vec::with_capacity(512);

            match server.try_recv_buf_from(&mut buf) {
                Ok((n, addr)) => {
                    assert_eq!(n, 11);
                    assert_eq!(addr, caddr);
                    assert_eq!(&buf[0..11], &b"hello world"[..]);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }
    }
}

#[tokio::test]
#[cfg_attr(
    target_os = "wasi",
    ignore = "temporarily disabled for WASI pending https://github.com/WebAssembly/wasi-libc/pull/734"
)]
async fn recv_buf_from() -> std::io::Result<()> {
    let sender = UdpSocket::bind("127.0.0.1:0").await?;
    let receiver = UdpSocket::bind("127.0.0.1:0").await?;

    sender.connect(receiver.local_addr()?).await?;

    sender.send(MSG).await?;
    let mut recv_buf = Vec::with_capacity(32);
    let (len, caddr) = receiver.recv_buf_from(&mut recv_buf).await?;

    assert_eq!(len, MSG_LEN);
    assert_eq!(&recv_buf[..len], MSG);
    assert_eq!(caddr, sender.local_addr()?);
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    target_os = "wasi",
    ignore = "temporarily disabled for WASI pending https://github.com/WebAssembly/wasi-libc/pull/734"
)]
async fn poll_ready() {
    // Create listener
    let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let saddr = server.local_addr().unwrap();

    // Create socket pair
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let caddr = client.local_addr().unwrap();

    for _ in 0..5 {
        loop {
            assert_ok!(poll_fn(|cx| client.poll_send_ready(cx)).await);

            match client.try_send_to(b"hello world", saddr) {
                Ok(n) => {
                    assert_eq!(n, 11);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }

        loop {
            assert_ok!(poll_fn(|cx| server.poll_recv_ready(cx)).await);

            let mut buf = Vec::with_capacity(512);

            match server.try_recv_buf_from(&mut buf) {
                Ok((n, addr)) => {
                    assert_eq!(n, 11);
                    assert_eq!(addr, caddr);
                    assert_eq!(&buf[0..11], &b"hello world"[..]);
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{e:?}"),
            }
        }
    }
}

/// Macro to create a simple test to set and get a socket option.
macro_rules! test {
    // Test using the `arg`ument as expected return value.
    ($( #[ $attr: meta ] )* $get_fn: ident, $set_fn: ident ( $arg: expr ) ) => {
        test!($( #[$attr] )* $get_fn, $set_fn($arg), $arg);
    };
    ($( #[ $attr: meta ] )* $get_fn: ident, $set_fn: ident ( $arg: expr ), $expected: expr ) => {
        #[tokio::test]
        $( #[$attr] )*
        async fn $get_fn() {
            test!(__ "127.0.0.1:0", $get_fn, $set_fn($arg), $expected);
            #[cfg(not(target_os = "vita"))]
            test!(__ "[::1]:0", $get_fn, $set_fn($arg), $expected);
        }
    };
    // Only test using a IPv4 socket.
    (IPv4 $get_fn: ident, $set_fn: ident ( $arg: expr ) ) => {
        #[tokio::test]
        async fn $get_fn() {
            test!(__ "127.0.0.1:0", $get_fn, $set_fn($arg), $arg);
        }
    };
    // Only test using a IPv6 socket.
    (IPv6 $get_fn: ident, $set_fn: ident ( $arg: expr ) ) => {
        #[tokio::test]
        async fn $get_fn() {
            test!(__ "[::1]:0", $get_fn, $set_fn($arg), $arg);
        }
    };

    // Internal to this macro.
    (__ $addr: literal, $get_fn: ident, $set_fn: ident ( $arg: expr ), $expected: expr ) => {
        let socket = UdpSocket::bind($addr).await.expect("failed to create `UdpSocket`");

        let initial = socket.$get_fn().expect("failed to get initial value");
        let arg = $arg;
        assert_ne!(initial, arg, "initial value and argument are the same");

        socket.$set_fn(arg).expect("failed to set option");
        let got = socket.$get_fn().expect("failed to get value");
        let expected = $expected;
        assert_eq!(got, expected, "set and get values differ");
    };
}

#[cfg(not(target_os = "wasi"))] // WASI does not yet support broadcast
test!(broadcast, set_broadcast(true));

#[cfg(not(target_os = "wasi"))] // WASI does not yet support multicast
test!(IPv4 multicast_loop_v4, set_multicast_loop_v4(false));

#[cfg(target_os = "linux")] // broken on non-Linux platforms https://github.com/rust-lang/socket2/pull/630
test!(multicast_ttl_v4, set_multicast_ttl_v4(40));

#[cfg(not(target_os = "wasi"))] // WASI does not yet support multicast
test!(IPv6 multicast_loop_v6, set_multicast_loop_v6(false));

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

test!(IPv4 ttl, set_ttl(40));

#[cfg(not(any(
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "haiku",
    target_os = "wasi"
)))]
test!(IPv4 tos_v4, set_tos_v4(96));
