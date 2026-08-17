//! An example of extracting a file in an archive.
//!
//! Takes a tarball on standard input, looks for an entry with a listed file
//! name as the first argument provided, and then prints the contents of that
//! file to stdout.

#[cfg(feature = "runtime-async-std")]
use async_std::{
    io::{copy, stdin, stdout},
    path::Path,
    stream::StreamExt,
};
use std::env::args_os;
#[cfg(feature = "runtime-tokio")]
use std::path::Path;
#[cfg(feature = "runtime-tokio")]
use tokio::io::{copy, stdin, stdout};
#[cfg(feature = "runtime-tokio")]
use tokio_stream::StreamExt;

use async_tar::Archive;

async fn inner_main() {
    let first_arg = args_os().nth(1).unwrap();
    let filename = Path::new(&first_arg);
    let ar = Archive::new(stdin());
    let mut entries = ar.entries().unwrap();
    while let Some(file) = entries.next().await {
        let mut f = file.unwrap();
        if f.path().unwrap() == filename {
            copy(&mut f, &mut stdout()).await.unwrap();
        }
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
