//! Run this example with the following command in a terminal:
//!
//! ```console
//! $ echo -n 'example' | zstd | cargo run --example zstd_gzip --features="tokio,zstd,gzip" | gunzip -c
//! 7example
//! ```
//!
//! Note that the "7" prefix (input length) is printed to stdout but will likely show up as shown
//! above. This is not an encoding error; see the code in `main`.

use std::io::Result;

use async_compression::tokio::{bufread::ZstdDecoder, write::GzipEncoder};
use tokio::io::{stderr, stdin, stdout, BufReader};
use tokio::io::{
    AsyncReadExt as _,  // for `read_to_end`
    AsyncWriteExt as _, // for `write_all` and `shutdown`
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Read zstd encoded data from stdin and decode
    let mut reader = ZstdDecoder::new(BufReader::new(stdin()));
    let mut x: Vec<u8> = vec![];
    reader.read_to_end(&mut x).await?;

    // print to stderr the length of the decoded data
    let mut error = stderr();
    error.write_all(x.len().to_string().as_bytes()).await?;
    error.shutdown().await?;

    // print to stdin encoded gzip data
    let mut writer = GzipEncoder::new(stdout());
    writer.write_all(&x).await?;
    writer.shutdown().await?;

    // flush stdout
    let mut res = writer.into_inner();
    res.flush().await?;

    Ok(())
}
