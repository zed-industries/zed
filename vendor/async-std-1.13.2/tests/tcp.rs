#![cfg(not(target_os = "unknown"))]

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

const THE_WINTERS_TALE: &[u8] = b"
    Each your doing,
    So singular in each particular,
    Crowns what you are doing in the present deed,
    That all your acts are queens.
";

#[test]
fn connect() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let t = task::spawn(async move { listener.accept().await });

        let stream2 = TcpStream::connect(&addr).await?;
        let stream1 = t.await?.0;

        assert_eq!(stream1.peer_addr()?, stream2.local_addr()?);
        assert_eq!(stream2.peer_addr()?, stream1.local_addr()?);

        Ok(())
    })
}

#[test]
fn incoming_read() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        task::spawn(async move {
            let mut stream = TcpStream::connect(&addr).await?;
            stream.write_all(THE_WINTERS_TALE).await?;
            io::Result::Ok(())
        });

        let mut buf = vec![0; 1024];
        let mut incoming = listener.incoming();
        let mut stream = incoming.next().await.unwrap()?;

        let n = stream.read(&mut buf).await?;
        assert_eq!(&buf[..n], THE_WINTERS_TALE);

        Ok(())
    })
}

#[test]
fn smoke_std_stream_to_async_listener() -> io::Result<()> {
    use std::io::Write;

    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        let mut std_stream = std::net::TcpStream::connect(&addr)?;
        std_stream.write_all(THE_WINTERS_TALE)?;

        let mut buf = vec![0; 1024];
        let mut incoming = listener.incoming();
        let mut stream = incoming.next().await.unwrap()?;

        let n = stream.read(&mut buf).await?;
        assert_eq!(&buf[..n], THE_WINTERS_TALE);

        Ok(())
    })
}

#[test]
fn smoke_async_stream_to_std_listener() -> io::Result<()> {
    use std::io::Read;

    let std_listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let addr = std_listener.local_addr()?;

    task::block_on(async move {
        let mut stream = TcpStream::connect(&addr).await?;
        stream.write_all(THE_WINTERS_TALE).await?;
        io::Result::Ok(())
    })?;

    let mut buf = vec![0; 1024];
    let mut incoming = std_listener.incoming();
    let mut stream = incoming.next().unwrap()?;

    let n = stream.read(&mut buf).unwrap();
    assert_eq!(&buf[..n], THE_WINTERS_TALE);

    Ok(())
}

#[test]
fn cloned_streams() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        let mut stream = TcpStream::connect(&addr).await?;
        let mut cloned_stream = stream.clone();
        let mut incoming = listener.incoming();
        let mut write_stream = incoming.next().await.unwrap()?;
        write_stream.write_all(b"Each your doing").await?;

        let mut buf = [0; 15];
        stream.read_exact(&mut buf[..8]).await?;
        cloned_stream.read_exact(&mut buf[8..]).await?;

        assert_eq!(&buf[..15], b"Each your doing");

        Ok(())
    })
}
