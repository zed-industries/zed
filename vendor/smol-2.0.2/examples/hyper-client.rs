//! An HTTP+TLS client based on `hyper` and `async-native-tls`.
//!
//! Run with:
//!
//! ```
//! cargo run --example hyper-client
//! ```

use std::convert::TryInto;
use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::{bail, Context as _, Result};
use async_native_tls::TlsStream;
use http_body_util::{BodyStream, Empty};
use hyper::body::Incoming;
use hyper::{Request, Response};
use macro_rules_attribute::apply;
use smol::{io, net::TcpStream, prelude::*, Executor};
use smol_hyper::rt::FuturesIo;
use smol_macros::main;

/// Sends a request and fetches the response.
async fn fetch(
    ex: &Executor<'static>,
    req: Request<Empty<&'static [u8]>>,
) -> Result<Response<Incoming>> {
    // Connect to the HTTP server.
    let io = {
        let host = req.uri().host().context("cannot parse host")?;

        match req.uri().scheme_str() {
            Some("http") => {
                let stream = {
                    let port = req.uri().port_u16().unwrap_or(80);
                    TcpStream::connect((host, port)).await?
                };
                SmolStream::Plain(stream)
            }
            Some("https") => {
                // In case of HTTPS, establish a secure TLS connection first.
                let stream = {
                    let port = req.uri().port_u16().unwrap_or(443);
                    TcpStream::connect((host, port)).await?
                };
                let stream = async_native_tls::connect(host, stream).await?;
                SmolStream::Tls(stream)
            }
            scheme => bail!("unsupported scheme: {:?}", scheme),
        }
    };

    // Spawn the HTTP/1 connection.
    let (mut sender, conn) = hyper::client::conn::http1::handshake(FuturesIo::new(io)).await?;
    ex.spawn(async move {
        if let Err(e) = conn.await {
            println!("Connection failed: {:?}", e);
        }
    })
    .detach();

    // Get the result
    let result = sender.send_request(req).await?;
    Ok(result)
}

#[apply(main!)]
async fn main(ex: &Executor<'static>) -> Result<()> {
    // Create a request.
    let url: hyper::Uri = "https://www.rust-lang.org".try_into()?;
    let req = Request::builder()
        .header(
            hyper::header::HOST,
            url.authority().unwrap().clone().as_str(),
        )
        .uri(url)
        .body(Empty::new())?;

    // Fetch the response.
    let resp = fetch(ex, req).await?;
    println!("{:#?}", resp);

    // Read the message body.
    let body: Vec<u8> = BodyStream::new(resp.into_body())
        .try_fold(Vec::new(), |mut body, chunk| {
            if let Some(chunk) = chunk.data_ref() {
                body.extend_from_slice(chunk);
            }
            Ok(body)
        })
        .await?;
    println!("{}", String::from_utf8_lossy(&body));

    Ok(())
}

/// A TCP or TCP+TLS connection.
enum SmolStream {
    /// A plain TCP connection.
    Plain(TcpStream),

    /// A TCP connection secured by TLS.
    Tls(TlsStream<TcpStream>),
}

impl AsyncRead for SmolStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            SmolStream::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for SmolStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            SmolStream::Tls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_close(cx),
            SmolStream::Tls(stream) => Pin::new(stream).poll_close(cx),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_flush(cx),
            SmolStream::Tls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }
}
