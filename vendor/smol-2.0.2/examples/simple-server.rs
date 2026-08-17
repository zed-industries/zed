//! A simple HTTP+TLS server based on `async-native-tls`.
//!
//! Run with:
//!
//! ```
//! cargo run --example simple-server
//! ```
//!
//! Open in the browser any of these addresses:
//!
//! - http://localhost:8000/
//! - https://localhost:8001/ (accept the security prompt in the browser)
//!
//! Refer to `README.md` to see how to the TLS certificate was generated.

use std::net::{TcpListener, TcpStream};

use anyhow::Result;
use async_native_tls::{Identity, TlsAcceptor};
use smol::{future, prelude::*, Async};

const RESPONSE: &[u8] = br#"
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 47

<!DOCTYPE html><html><body>Hello!</body></html>
"#;

/// Reads a request from the client and sends it a response.
async fn serve(mut stream: Async<TcpStream>, tls: Option<TlsAcceptor>) -> Result<()> {
    match tls {
        None => {
            println!("Serving http://{}", stream.get_ref().local_addr()?);
            stream.write_all(RESPONSE).await?;
        }
        Some(tls) => {
            println!("Serving https://{}", stream.get_ref().local_addr()?);

            // In case of HTTPS, establish a secure TLS connection first.
            match tls.accept(stream).await {
                Ok(mut stream) => {
                    stream.write_all(RESPONSE).await?;
                    stream.flush().await?;
                    stream.close().await?;
                }
                Err(err) => println!("Failed to establish secure TLS connection: {:#?}", err),
            }
        }
    }
    Ok(())
}

/// Listens for incoming connections and serves them.
async fn listen(listener: Async<TcpListener>, tls: Option<TlsAcceptor>) -> Result<()> {
    // Display the full host address.
    match &tls {
        None => println!("Listening on http://{}", listener.get_ref().local_addr()?),
        Some(_) => println!("Listening on https://{}", listener.get_ref().local_addr()?),
    }

    loop {
        // Accept the next connection.
        let (stream, _) = listener.accept().await?;
        let tls = tls.clone();

        // Spawn a background task serving this connection.
        smol::spawn(async move {
            if let Err(err) = serve(stream, tls).await {
                println!("Connection error: {:#?}", err);
            }
        })
        .detach();
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
