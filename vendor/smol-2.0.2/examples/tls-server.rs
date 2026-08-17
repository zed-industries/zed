//! A TCP server secured by TLS based on `async-native-tls`.
//!
//! First start a server:
//!
//! ```
//! cargo run --example tls-server
//! ```
//!
//! Then start a client:
//!
//! ```
//! cargo run --example tls-client
//! ```

use std::net::{TcpListener, TcpStream};

use anyhow::Result;
use async_native_tls::{Identity, TlsAcceptor, TlsStream};
use smol::{io, Async};

/// Echoes messages from the client back to it.
async fn echo(stream: TlsStream<Async<TcpStream>>) -> Result<()> {
    let stream = async_dup::Mutex::new(stream);
    io::copy(&stream, &mut &stream).await?;
    Ok(())
}

fn main() -> Result<()> {
    // Initialize TLS with the local certificate, private key, and password.
    let identity = Identity::from_pkcs12(include_bytes!("identity.pfx"), "password")?;
    let tls = TlsAcceptor::from(native_tls::TlsAcceptor::new(identity)?);

    smol::block_on(async {
        // Create a listener.
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 7001))?;
        println!("Listening on {}", listener.get_ref().local_addr()?);
        println!("Now start a TLS client.");

        // Accept clients in a loop.
        loop {
            let (stream, _) = listener.accept().await?;
            let stream = tls.accept(stream).await?;
            println!(
                "Accepted client: {}",
                stream.get_ref().get_ref().peer_addr()?
            );

            // Spawn a task that echoes messages from the client back to it.
            smol::spawn(echo(stream)).detach();
        }
    })
}
