//! An HTTP+TLS client based on `async-h1` and `async-native-tls`.
//!
//! Run with:
//!
//! ```
//! cargo run --example async-h1-client
//! ```

use std::net::{TcpStream, ToSocketAddrs};

use anyhow::{bail, Context as _, Error, Result};
use http_types::{Method, Request, Response};
use smol::{prelude::*, Async};
use url::Url;

/// Sends a request and fetches the response.
async fn fetch(req: Request) -> Result<Response> {
    // Figure out the host and the port.
    let host = req.url().host().context("cannot parse host")?.to_string();
    let port = req
        .url()
        .port_or_known_default()
        .context("cannot guess port")?;

    // Connect to the host.
    let socket_addr = {
        let host = host.clone();
        smol::unblock(move || (host.as_str(), port).to_socket_addrs())
            .await?
            .next()
            .context("cannot resolve address")?
    };
    let stream = Async::<TcpStream>::connect(socket_addr).await?;

    // Send the request and wait for the response.
    let resp = match req.url().scheme() {
        "http" => async_h1::connect(stream, req).await.map_err(Error::msg)?,
        "https" => {
            // In case of HTTPS, establish a secure TLS connection first.
            let stream = async_native_tls::connect(&host, stream).await?;
            async_h1::connect(stream, req).await.map_err(Error::msg)?
        }
        scheme => bail!("unsupported scheme: {}", scheme),
    };
    Ok(resp)
}

fn main() -> Result<()> {
    smol::block_on(async {
        // Create a request.
        let addr = "https://www.rust-lang.org";
        let req = Request::new(Method::Get, Url::parse(addr)?);

        // Fetch the response.
        let mut resp = fetch(req).await?;
        println!("{:#?}", resp);

        // Read the message body.
        let mut body = Vec::new();
        resp.read_to_end(&mut body).await?;
        println!("{}", String::from_utf8_lossy(&body));

        Ok(())
    })
}
