//! An HTTP+TLS server based on `async-h1` and `async-native-tls`.
//!
//! Run with:
//!
//! ```
//! cargo run --example async-h1-server
//! ```
//!
//! Open in the browser any of these addresses:
//!
//! - http://localhost:8000/
//! - https://localhost:8001/ (accept the security prompt in the browser)
//!
//! Refer to `README.md` to see how to the TLS certificate was generated.

use std::net::TcpListener;

use anyhow::Result;
use async_native_tls::{Identity, TlsAcceptor};
use http_types::{Request, Response, StatusCode};
use smol::{future, Async};

/// Serves a request and returns a response.
async fn serve(req: Request) -> http_types::Result<Response> {
    println!("Serving {}", req.url());

    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/plain");
    res.set_body("Hello from async-h1!");
    Ok(res)
}

/// Listens for incoming connections and serves them.
async fn listen(listener: Async<TcpListener>, tls: Option<TlsAcceptor>) -> Result<()> {
    // Format the full host address.
    let host = match &tls {
        None => format!("http://{}", listener.get_ref().local_addr()?),
        Some(_) => format!("https://{}", listener.get_ref().local_addr()?),
    };
    println!("Listening on {}", host);

    loop {
        // Accept the next connection.
        let (stream, _) = listener.accept().await?;

        // Spawn a background task serving this connection.
        let task = match &tls {
            None => {
                let stream = async_dup::Arc::new(stream);
                smol::spawn(async move {
                    if let Err(err) = async_h1::accept(stream, serve).await {
                        println!("Connection error: {:#?}", err);
                    }
                })
            }
            Some(tls) => {
                // In case of HTTPS, establish a secure TLS connection first.
                match tls.accept(stream).await {
                    Ok(stream) => {
                        let stream = async_dup::Arc::new(async_dup::Mutex::new(stream));
                        smol::spawn(async move {
                            if let Err(err) = async_h1::accept(stream, serve).await {
                                println!("Connection error: {:#?}", err);
                            }
                        })
                    }
                    Err(err) => {
                        println!("Failed to establish secure TLS connection: {:#?}", err);
                        continue;
                    }
                }
            }
        };

        // Detach the task to let it run in the background.
        task.detach();
    }
}

fn main() -> Result<()> {
    // Initialize TLS with the local certificate, private key, and password.
    let identity = Identity::from_pkcs12(include_bytes!("identity.pfx"), "password")?;
    let tls = TlsAcceptor::from(native_tls::TlsAcceptor::new(identity)?);

    // Start HTTP and HTTPS servers.
    smol::block_on(async {
        let http = listen(Async::<TcpListener>::bind(([127, 0, 0, 1], 8000))?, None);
        let https = listen(
            Async::<TcpListener>::bind(([127, 0, 0, 1], 8001))?,
            Some(tls),
        );
        future::try_zip(http, https).await?;
        Ok(())
    })
}
