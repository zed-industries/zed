//! An example of listing the file names of entries in an archive.
//!
//! Takes a tarball on stdin and prints out all of the entries inside.

#[cfg(feature = "runtime-async-std")]
use async_std::{io::stdin, stream::StreamExt};
#[cfg(feature = "runtime-tokio")]
use tokio::io::stdin;
#[cfg(feature = "runtime-tokio")]
use tokio_stream::StreamExt;

use async_tar::Archive;

async fn inner_main() {
    let ar = Archive::new(stdin());
    let mut entries = ar.entries().unwrap();
    while let Some(file) = entries.next().await {
        let f = file.unwrap();
        println!("{}", f.path().unwrap().display());
    }
}

#[cfg(feature = "runtime-async-std")]
fn main() {
    async_std::task::block_on(inner_main());
}

#[cfg(feature = "runtime-tokio")]
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(inner_main());
}
