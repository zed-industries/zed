#![cfg(not(target_os = "unknown"))]

use async_std::io;
use async_std::net::UdpSocket;
use async_std::task;

const THE_MERCHANT_OF_VENICE: &[u8] = b"
    If you prick us, do we not bleed?
    If you tickle us, do we not laugh?
    If you poison us, do we not die?
    And if you wrong us, shall we not revenge?
";

#[test]
fn send_recv_peek() -> io::Result<()> {
    task::block_on(async {
        let socket1 = UdpSocket::bind("127.0.0.1:0").await?;
        let socket2 = UdpSocket::bind("127.0.0.1:0").await?;

        socket1.connect(socket2.local_addr()?).await?;
        socket2.connect(socket1.local_addr()?).await?;
        assert_eq!(socket1.peer_addr()?, socket2.local_addr()?);
        socket1.send(THE_MERCHANT_OF_VENICE).await?;

        let mut buf = [0u8; 1024];
        let n = socket2.peek(&mut buf).await?;
        assert_eq!(&buf[..n], THE_MERCHANT_OF_VENICE);

        let n = socket2.recv(&mut buf).await?;
        assert_eq!(&buf[..n], THE_MERCHANT_OF_VENICE);

        Ok(())
    })
}
