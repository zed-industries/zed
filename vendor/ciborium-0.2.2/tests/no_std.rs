// SPDX-License-Identifier: Apache-2.0

#![cfg(all(feature = "serde", not(feature = "std")))]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use ciborium::{de::from_reader, ser::into_writer};

#[test]
fn decode() {
    assert_eq!(from_reader::<u8, &[u8]>(&[7u8][..]).unwrap(), 7);
}

#[test]
fn eof() {
    from_reader::<u8, &[u8]>(&[]).unwrap_err();
}

#[test]
fn encode_slice() {
    let mut buffer = [0u8; 1];
    into_writer(&3u8, &mut buffer[..]).unwrap();
    assert_eq!(buffer[0], 3);
}

#[test]
fn encode_vec() {
    let mut buffer = Vec::with_capacity(1);
    into_writer(&3u8, &mut buffer).unwrap();
    assert_eq!(buffer[0], 3);
}

#[test]
fn oos() {
    into_writer(&3u8, &mut [][..]).unwrap_err();
}
