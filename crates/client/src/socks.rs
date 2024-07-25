use std::pin::Pin;

use futures::io::{AsyncRead, AsyncWrite};
use http_client::proxy::Proxy;
use tokio_socks::io::Compat;

pub enum SocksVersion {
    V4,
    V5,
}

pub enum SocksStream<S> {
    NoProxy(S),
    Socks4(tokio_socks::tcp::Socks4Stream<Compat<S>>),
    Socks5(tokio_socks::tcp::Socks5Stream<Compat<S>>),
}

pub fn get_socks_proxy(proxy: &Proxy) -> Option<((String, u16), SocksVersion)> {
    let Some(proxy_uri) = proxy.to_uri() else {
        return None;
    };
    let Some(schema) = proxy_uri.scheme_str() else {
        return None;
    };
    let socks_version = if schema.starts_with("socks4") {
        SocksVersion::V4
    } else if schema.starts_with("socks") {
        SocksVersion::V5
    } else {
        return None;
    };
    if let (Some(host), Some(port)) = (proxy_uri.host(), proxy_uri.port_u16()) {
        Some(((host.to_string(), port), socks_version))
    } else {
        None
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for SocksStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        match self.get_mut() {
            SocksStream::NoProxy(s) => AsyncRead::poll_read(Pin::new(s), cx, buf),
            SocksStream::Socks4(s4) => AsyncRead::poll_read(Pin::new(s4), cx, buf),
            SocksStream::Socks5(s5) => AsyncRead::poll_read(Pin::new(s5), cx, buf),
        }
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for SocksStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        match self.get_mut() {
            SocksStream::NoProxy(s) => AsyncWrite::poll_write(Pin::new(s), cx, buf),
            SocksStream::Socks4(s4) => AsyncWrite::poll_write(Pin::new(s4), cx, buf),
            SocksStream::Socks5(s5) => AsyncWrite::poll_write(Pin::new(s5), cx, buf),
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            SocksStream::NoProxy(s) => AsyncWrite::poll_flush(Pin::new(s), cx),
            SocksStream::Socks4(s4) => AsyncWrite::poll_flush(Pin::new(s4), cx),
            SocksStream::Socks5(s5) => AsyncWrite::poll_flush(Pin::new(s5), cx),
        }
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        match self.get_mut() {
            SocksStream::NoProxy(s) => AsyncWrite::poll_close(Pin::new(s), cx),
            SocksStream::Socks4(s4) => AsyncWrite::poll_close(Pin::new(s4), cx),
            SocksStream::Socks5(s5) => AsyncWrite::poll_close(Pin::new(s5), cx),
        }
    }
}
