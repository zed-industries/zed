//! A simple HTTP+TLS client based on `async-native-tls`.
//!
//! Run with:
//!
//! ```
//! cargo run --example simple-client
//! ```

use std::net::{TcpStream, ToSocketAddrs};

use anyhow::{bail, Context as _, Result};
use smol::{prelude::*, Async};
use url::Url;

/// Sends a GET request and fetches the response.
async fn fetch(addr: &str) -> Result<Vec<u8>> {
    // Parse the URL.
    let url = Url::parse(addr)?;
    let host = url.host().context("cannot parse host")?.to_string();
    let port = url.port_or_known_default().context("cannot guess port")?;
    let path = url.path().to_string();
    let query = match url.query() {
        Some(q) => format!("?{}", q),
        None => String::new(),
    };

    // Construct a request.
    let req = format!(
        "GET {}{} HTTP/1.1\r\nHost: {}\r\nAccept: */*\r\nConnection: close\r\n\r\n",
        path, query, host,
    );

    // Connect to the host.
    let socket_addr = {
        let host = host.clone();
        smol::unblock(move || (host.as_str(), port).to_socket_addrs())
            .await?
            .next()
            .context("cannot resolve address")?
    };
    let mut stream = Async::<TcpStream>::connect(socket_addr).await?;

    // Send the request and wait for the response.
    let mut resp = Vec::new();
    match url.scheme() {
        "http" => {
            stream.write_all(req.as_bytes()).await?;
            stream.read_to_end(&mut resp).await?;
        }
        "https" => {
            // In case of HTTPS, establish a secure TLS connection first.
            let mut stream = async_native_tls::connect(&host, stream).await?;
            stream.write_all(req.as_bytes()).await?;
            stream.read_to_end(&mut resp).await?;
        }
        scheme => bail!("unsupported scheme: {}", scheme),
    }

    Ok(resp)
}

fn main() -> Result<()> {
    smol::block_on(async {
        let addr = "https://www.rust-lang.org";
        let resp = fetch(addr).await?;
        println!("{}", String::from_utf8_lossy(&resp));
        Ok(())
    })
}
