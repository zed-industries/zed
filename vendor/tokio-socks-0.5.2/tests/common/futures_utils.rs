use std::{
    future::Future,
    io::{Read, Write},
    net::{SocketAddr, TcpStream as StdTcpStream},
    sync::Mutex,
};

use futures_util::{io::copy, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use once_cell::sync::OnceCell;
use smol::net::{unix::UnixStream, TcpListener};
use tokio_socks::{
    io::Compat,
    tcp::{socks4::Socks4Listener, socks5::Socks5Listener},
    Error,
    Result,
};

use super::*;

pub async fn echo_server() -> Result<()> {
    let listener = TcpListener::bind(&SocketAddr::from(([0, 0, 0, 0], 10007))).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        smol::spawn(async move {
            let (mut reader, mut writer) = stream.split();
            copy(&mut reader, &mut writer).await.unwrap();
        })
        .detach();
    }
}

pub async fn reply_response<S: AsyncRead + AsyncWrite + Unpin>(mut socket: S) -> Result<[u8; 5]> {
    socket.write_all(MSG).await?;
    let mut buf = [0; 5];
    socket.read_exact(&mut buf).await?;
    Ok(buf)
}

pub async fn test_connect<S: AsyncRead + AsyncWrite + Unpin>(socket: S) -> Result<()> {
    let res = reply_response(socket).await?;
    assert_eq!(&res[..], MSG);
    Ok(())
}

pub fn test_bind<S: 'static + AsyncRead + AsyncWrite + Unpin + Send>(
    listener: Socks5Listener<Compat<S>>,
) -> Result<()> {
    let bind_addr = listener.bind_addr().to_owned();
    runtime().lock().unwrap().spawn(async move {
        let stream = listener.accept().await.unwrap();
        let (mut reader, mut writer) = stream.split();
        copy(&mut reader, &mut writer).await.unwrap();
    });

    let mut tcp = StdTcpStream::connect(bind_addr)?;
    tcp.write_all(MSG)?;
    let mut buf = [0; 5];
    tcp.read_exact(&mut buf[..])?;
    assert_eq!(&buf[..], MSG);
    Ok(())
}

pub async fn connect_unix(proxy_addr: &str) -> Result<UnixStream> {
    UnixStream::connect(proxy_addr).await.map_err(Error::Io)
}

pub struct Runtime;

impl Runtime {
    pub fn spawn<F, T>(&self, future: F)
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        smol::spawn(future).detach();
    }

    pub fn block_on<F, T>(&self, future: F) -> T
    where F: Future<Output = T> {
        smol::block_on(future)
    }
}

pub fn runtime() -> &'static Mutex<Runtime> {
    static RUNTIME: OnceCell<Mutex<Runtime>> = OnceCell::new();
    RUNTIME.get_or_init(|| {
        smol::spawn(async { echo_server().await.expect("Unable to bind") }).detach();
        Mutex::new(Runtime)
    })
}

pub fn test_bind_socks4<S: 'static + AsyncRead + AsyncWrite + Unpin + Send>(
    listener: Socks4Listener<Compat<S>>,
) -> Result<()> {
    let bind_addr = listener.bind_addr().to_owned();
    runtime().lock().unwrap().spawn(async move {
        let stream = listener.accept().await.unwrap();
        let (mut reader, mut writer) = AsyncReadExt::split(stream);
        copy(&mut reader, &mut writer).await.unwrap();
    });

    let mut tcp = StdTcpStream::connect(bind_addr)?;
    tcp.write_all(MSG)?;
    let mut buf = [0; 5];
    tcp.read_exact(&mut buf[..])?;
    assert_eq!(&buf[..], MSG);
    Ok(())
}
