//! A simple example showing how to crate size-sealed memfd.
//!
//! It creates a new memfd and seals to on a fixed 1K size.
//!
//! This is an example ONLY: do NOT panic/unwrap/assert
//! in production code!

extern crate memfd;

use std::io::{Seek, SeekFrom, Write};

fn main() {
    // Create a sealable memfd.
    let opts = memfd::MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("sized-1K").unwrap();

    // Resize to 1024B.
    mfd.as_file().set_len(1024).unwrap();

    // Add seals to prevent further resizing.
    mfd.add_seals(&[
        memfd::FileSeal::SealShrink,
        memfd::FileSeal::SealGrow
    ]).unwrap();

    // Prevent further sealing changes.
    mfd.add_seal(memfd::FileSeal::SealSeal).unwrap();

    // Write 1K of data, allowed by size seals.
    let data_1k = vec![0x00; 1024];
    let r = mfd.as_file().write_all(&data_1k);
    assert!(r.is_ok());
    mfd.as_file().seek(SeekFrom::Start(0)).unwrap();

    // Write 2K of data, now allowed by size seals.
    let data_2k = vec![0x11; 2048];
    let r = mfd.as_file().write_all(&data_2k);
    assert!(r.is_err());
    mfd.as_file().seek(SeekFrom::Start(0)).unwrap();

    // Try to resize to 2048B, not allowed by size seals.
    let r = mfd.as_file().set_len(2048);
    assert!(r.is_err());

    // Overwrite 1K of data, allowed by size seals.
    let data_1k = vec![0x22; 1024];
    let r = mfd.as_file().write_all(&data_1k);
    assert!(r.is_ok());
}
