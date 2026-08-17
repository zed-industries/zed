//! Prints a file given as an argument to stdout.

use std::env::args;

use async_std::fs::File;
use async_std::io;
use async_std::prelude::*;
use async_std::task;

const LEN: usize = 16 * 1024; // 16 Kb

fn main() -> io::Result<()> {
    let path = args().nth(1).expect("missing path argument");

    task::block_on(async {
        let mut file = File::open(&path).await?;
        let mut stdout = io::stdout();
        let mut buf = vec![0u8; LEN];

        loop {
            // Read a buffer from the file.
            let n = file.read(&mut buf).await?;

            // If this is the end of file, clean up and return.
            if n == 0 {
                stdout.flush().await?;
                return Ok(());
            }

            // Write the buffer into stdout.
            stdout.write_all(&buf[..n]).await?;
        }
    })
}
