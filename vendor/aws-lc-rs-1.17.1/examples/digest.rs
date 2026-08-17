// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! digest - display the checksum for files.
//!
//! *digest* is an example program using *aws-lc-rs*. It can compute the digest (i.e., "checksum")
//! of files using any of the digest algorithms supported by *aws-lc-rs*.
//!
//! The program can be run from the command line using cargo:
//! ```
//! > cargo run --example digest -- -d sha256 LICENSE
//! ```
use aws_lc_rs::{digest, test};
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{Read, Result};

#[derive(ValueEnum, Clone, Copy, Debug)]
enum DigestType {
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256,
}

impl DigestType {
    fn digest(self) -> &'static digest::Algorithm {
        match self {
            DigestType::SHA1 => &digest::SHA1_FOR_LEGACY_USE_ONLY,
            DigestType::SHA256 => &digest::SHA256,
            DigestType::SHA384 => &digest::SHA384,
            DigestType::SHA512 => &digest::SHA512,
            DigestType::SHA512_256 => &digest::SHA512_256,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, name = "digest")]
struct Cli {
    #[arg(short, long, value_enum)]
    digest: Option<DigestType>,

    files: Vec<String>,
}

const BUFFER_SIZE: usize = 4096;

fn process(
    digest_alg: &'static digest::Algorithm,
    file: &mut dyn Read,
    name: &str,
) -> Result<digest::Digest> {
    // Initialize a digest context, which will be used to compute the digest.
    let mut digest_context = digest::Context::new(digest_alg);

    // byte buffer used to load bytes from the file into the digest context.
    let mut buffer = [0u8; BUFFER_SIZE];

    // loop over bytes of the file until reaching the end or getting an error.
    loop {
        //  Collect the next buffer of bytes from the file.
        let result = file.read(&mut buffer);
        match result {
            // When 0 bytes are returned, this indicates we've reached EOF.
            Ok(0) => {
                //  "finish" the context to compute the digest/checksum
                let digest = digest_context.finish();

                // Display the resulting checksum
                println!("{} {}", test::to_hex(digest.as_ref()), name);
                return Ok(digest);
            }
            // n indicates the number of bytes loaded into the buffer
            Ok(n) => {
                // Update the context with the next buffer of bytes
                digest_context.update(&buffer[0..n]);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let digest_alg = cli.digest.unwrap_or(DigestType::SHA1).digest();
    let mut error = None;

    if cli.files.is_empty() {
        if let Err(e) = process(digest_alg, &mut std::io::stdin(), "-") {
            // Display error information
            println!("digest: -: {e}");
            error = Some(e);
        }
    } else {
        for file_name in cli.files {
            if let Err(e) = File::open(&file_name)
                .and_then(|mut file| process(digest_alg, &mut file, &file_name))
            {
                // Display error information
                println!("digest: {file_name}: {e}");
                error = Some(e);
            }
        }
    }
    if let Some(e) = error {
        Err(e)
    } else {
        Ok(())
    }
}
