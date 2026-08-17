//! An example of listing raw entries in an archive.
//!
//! Takes a tarball on stdin and prints out all of the entries inside.

#[cfg(feature = "runtime-async-std")]
use async_std::{io::stdin, prelude::*};
#[cfg(feature = "runtime-tokio")]
use tokio::io::stdin;
#[cfg(feature = "runtime-tokio")]
use tokio_stream::StreamExt;

use async_tar::Archive;

async fn inner_main() {
    let ar = Archive::new(stdin());
    let mut i = 0;
    let mut entries = ar.entries_raw().unwrap();
    while let Some(file) = entries.next().await {
        println!("-------------------------- Entry {}", i);
        let mut f = file.unwrap();
        println!("path: {}", f.path().unwrap().display());
        println!("size: {}", f.header().size().unwrap());
        println!("entry size: {}", f.header().entry_size().unwrap());
        println!("link name: {:?}", f.link_name().unwrap());
        println!("file type: {:#x}", f.header().entry_type().as_byte());
        println!("mode: {:#o}", f.header().mode().unwrap());
        println!("uid: {}", f.header().uid().unwrap());
        println!("gid: {}", f.header().gid().unwrap());
        println!("mtime: {}", f.header().mtime().unwrap());
        println!("username: {:?}", f.header().username().unwrap());
        println!("groupname: {:?}", f.header().groupname().unwrap());

        if f.header().as_ustar().is_some() {
            println!("kind: UStar");
        } else if f.header().as_gnu().is_some() {
            println!("kind: GNU");
        } else {
            println!("kind: normal");
        }

        if let Ok(Some(extensions)) = f.pax_extensions().await {
            println!("pax extensions:");
            for e in extensions {
                let e = e.unwrap();
                println!(
                    "\t{:?} = {:?}",
                    String::from_utf8_lossy(e.key_bytes()),
                    String::from_utf8_lossy(e.value_bytes())
                );
            }
        }
        i += 1;
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
