//! Test the proxy chaining capabilities
//!
//! This example make uses of several public proxy.

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    runtime::Runtime,
};
use tokio_socks::{tcp::socks5::Socks5Stream, Error};

const PROXY_ADDR: [&str; 2] = ["184.176.166.20:4145", "90.89.205.248:1080"]; // public proxies found here : http://spys.one/en/socks-proxy-list/
const DEST_ADDR: &str = "duckduckgo.com:80";

async fn connect_chained_proxy() -> Result<(), Error> {
    let proxy_stream = TcpStream::connect(PROXY_ADDR[0]).await?;
    let chained_proxy_stream = Socks5Stream::connect_with_socket(proxy_stream, PROXY_ADDR[1]).await?;
    let mut stream = Socks5Stream::connect_with_socket(chained_proxy_stream, DEST_ADDR).await?;

    stream.write_all(b"GET /\n\n").await?;

    let mut buf = Vec::new();
    let n = stream.read_to_end(&mut buf).await?;

    println!("{} bytes read\n\n{}", n, String::from_utf8_lossy(&buf));

    Ok(())
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(connect_chained_proxy()).unwrap();
}
