#![allow(clippy::needless_pass_by_value)]

use serde_bytes::{ByteArray, ByteBuf, Bytes};

fn _bytes_eq_slice(bytes: &Bytes, slice: &[u8]) -> bool {
    bytes == slice
}

fn _bytebuf_eq_vec(bytebuf: ByteBuf, vec: Vec<u8>) -> bool {
    bytebuf == vec
}

fn _bytes_eq_bytestring(bytes: &Bytes) -> bool {
    bytes == b"..."
}

fn _bytearray_eq_bytestring<const N: usize>(bytes: &ByteArray<N>) -> bool {
    bytes == &[0u8; N]
}

fn _bytearray_eq_bytearray<const N: usize>(bytes: &ByteArray<N>, other: &ByteArray<N>) -> bool {
    bytes == other
}
