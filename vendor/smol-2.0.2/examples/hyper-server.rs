//! An HTTP+TLS server based on `hyper` and `async-native-tls`.
//!
//! Run with:
//!
//! ```
//! cargo run --example hyper-server
//! ```
//!
//! Open in the browser any of these addresses:
//!
//! - http://localhost:8000/
//! - https://localhost:8001/ (accept the security prompt in the browser)
//!
//! Refer to `README.md` to see how to the TLS certificate was generated.

use std::net::{TcpListener, TcpStream};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use anyhow::Result;
use async_native_tls::{Identity, TlsAcceptor, TlsStream};
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Request, Response};
use macro_rules_attribute::apply;
use smol::{future, io, prelude::*, Async, Executor};
use smol_hyper::rt::{FuturesIo, SmolTimer};
use smol_macros::main;

/// Serves a request and returns a response.
async fn serve(req: Request<Incoming>) -> Result<Response<Full<&'static [u8]>>> {
    println!("Serving {}", req.uri());
    Ok(Response::new(Full::new("Hello from hyper!".as_bytes())))
}

/// Handle a new client.
async fn handle_client(client: Async<TcpStream>, tls: Option<TlsAcceptor>) -> Result<()> {
    // Wrap it in TLS if necessary.
    let client = match &tls {
        None => SmolStream::Plain(client),
        Some(tls) => {
            // In case of HTTPS, establish a secure TLS connection.
            SmolStream::Tls(tls.accept(client).await?)
        }
    };

    // Build the server.
    hyper::server::conn::http1::Builder::new()
        .timer(SmolTimer::new())
        .serve_connection(FuturesIo::new(client), service_fn(serve))
        .await?;

    Ok(())
}

/// Listens for incoming connections and serves them.
async fn listen(
    ex: &Arc<Executor<'static>>,
    listener: Async<TcpListener>,
    tls: Option<TlsAcceptor>,
) -> Result<()> {
    // Format the full host address.
    let host = &match tls {
        None => format!("http://{}", listener.get_ref().local_addr()?),
        Some(_) => format!("https://{}", listener.get_ref().local_addr()?),
    };
    println!("Listening on {}", host);

    loop {
        // Wait for a new client.
        let (client, _) = listener.accept().await?;

        // Spawn a task to handle this connection.
        ex.spawn({
            let tls = tls.clone();
            async move {
                if let Err(e) = handle_client(client, tls).await {
                    println!("Error while handling client: {}", e);
                }
            }
        })
        .detach();
    }
}

#[apply(main!)]
async fn main(ex: &Arc<Executor<'static>>) -> Result<()> {
    // Initialize TLS with the local certificate, private key, and password.
    let identity = Identity::from_pkcs12(include_bytes!("identity.pfx"), "password")?;
    let tls = TlsAcceptor::from(native_tls::TlsAcceptor::new(identity)?);

    // Start HTTP and HTTPS servers.
    let http = listen(
        ex,
        Async::<TcpListener>::bind(([127, 0, 0, 1], 8000))?,
        None,
    );
    let https = listen(
        ex,
        Async::<TcpListener>::bind(([127, 0, 0, 1], 8001))?,
        Some(tls),
    );
    future::try_zip(http, https).await?;
    Ok(())
}

/// A TCP or TCP+TLS connection.
enum SmolStream {
    /// A plain TCP connection.
    Plain(Async<TcpStream>),

    /// A TCP connection secured by TLS.
    Tls(TlsStream<Async<TcpStream>>),
}

impl AsyncRead for SmolStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            Self::Plain(s) => Pin::new(s).poll_read(cx, buf),
            Self::Tls(s) => Pin::new(s).poll_read(cx, buf),
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
            Self::Plain(s) => Pin::new(s).poll_write(cx, buf),
            Self::Tls(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            Self::Plain(s) => Pin::new(s).poll_close(cx),
            Self::Tls(s) => Pin::new(s).poll_close(cx),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            Self::Plain(s) => Pin::new(s).poll_close(cx),
            Self::Tls(s) => Pin::new(s).poll_close(cx),
        }
    }
}
