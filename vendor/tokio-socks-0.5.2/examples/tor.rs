//! Test the tor proxy capabilities
//!
//! This example requires a running tor proxy.

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    runtime::Runtime,
};
use tokio_socks::{tcp::socks5::Socks5Stream, Error};

const PROXY_ADDR: &str = "127.0.0.1:9050";
const ONION_ADDR: &str = "3g2upl4pq6kufc4m.onion:80"; // DuckDuckGo

async fn connect() -> Result<(), Error> {
    let target = Socks5Stream::tor_resolve(PROXY_ADDR, "duckduckgo.com:0").await?;
    eprintln!("duckduckgo.com = {:?}", target);
    let target = Socks5Stream::tor_resolve_ptr(PROXY_ADDR, "176.34.155.23:0").await?;
    eprintln!("176.34.155.23 = {:?}", target);

    let mut conn = Socks5Stream::connect(PROXY_ADDR, ONION_ADDR).await?;
    conn.write_all(b"GET /\n\n").await?;

    let mut buf = Vec::new();
    let n = conn.read_to_end(&mut buf).await?;

    println!("{} bytes read\n\n{}", n, String::from_utf8_lossy(&buf));

    Ok(())
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(connect()).unwrap();
}
