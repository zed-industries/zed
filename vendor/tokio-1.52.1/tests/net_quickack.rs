#![warn(rust_2018_idioms)]
#![cfg(all(
    feature = "net",
    any(
        target_os = "linux",
        target_os = "android",
        target_os = "fuchsia",
        target_os = "cygwin",
    )
))]
#![cfg(not(miri))] // No `socket` in miri.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::oneshot;

#[tokio::test]
async fn socket_works_with_quickack() {
    const MESSAGE: &str = "Hello, tokio!";

    let (tx_port, rx_port) = oneshot::channel();

    let server = tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tx_port.send(addr.port()).unwrap();

        let (mut stream, _) = listener.accept().await.unwrap();
        stream.set_quickack(true).unwrap();
        assert!(stream.quickack().unwrap());

        stream.write_all(MESSAGE.as_bytes()).await.unwrap();

        let mut buf = vec![0; MESSAGE.len()];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(buf, MESSAGE.as_bytes());

        // There is nothing special about setting quickack to false
        // at this point, we just want to test the `false` case.
        stream.set_quickack(false).unwrap();
        assert!(!stream.quickack().unwrap());

        stream.shutdown().await.unwrap();
    });

    let port = rx_port.await.unwrap();
    let client = tokio::spawn(async move {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{port}"))
            .await
            .unwrap();
        stream.set_quickack(true).unwrap();
        assert!(stream.quickack().unwrap());

        let mut buf = vec![0; MESSAGE.len()];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(buf, MESSAGE.as_bytes());

        stream.write_all(MESSAGE.as_bytes()).await.unwrap();

        // There is nothing special about setting quickack to false
        // at this point, we just want to test the `false` case.
        stream.set_quickack(false).unwrap();
        assert!(!stream.quickack().unwrap());

        stream.shutdown().await.unwrap();
    });

    tokio::try_join!(server, client).unwrap();
}
