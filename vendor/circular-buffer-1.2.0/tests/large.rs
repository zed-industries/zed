// Copyright © 2023-2025 Andrea Corbellini and contributors
// SPDX-License-Identifier: BSD-3-Clause

#![cfg(feature = "std")]

use circular_buffer::CircularBuffer;

#[cfg(not(miri))]
const SIZE: usize = 2 * 1024 * 1024; // 2 MiB

#[cfg(miri)]
const SIZE: usize = 2 * 1024; // 2 KiB

#[test]
fn large_boxed() {
    let chunk = b"abcdefghijklmnopqrstuvxyz0123456789";
    let mut buf = CircularBuffer::<SIZE, u8>::boxed();
    let mut vec = Vec::new();

    assert_ne!(SIZE % chunk.len(), 0);

    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
    assert!(!buf.is_full());
    assert_eq!(buf.as_slices().0, &[][..]);
    assert_eq!(buf.as_slices().1, &[][..]);

    for _ in 0..(SIZE / chunk.len()) {
        buf.extend_from_slice(&chunk[..]);
        vec.extend_from_slice(&chunk[..]);

        assert_eq!(buf.len(), vec.len());
        assert!(!buf.is_empty());
        assert!(!buf.is_full());
        assert_eq!(buf.as_slices().0, &vec[..]);
        assert_eq!(buf.as_slices().1, &[][..]);
    }

    for _ in 0..(SIZE / chunk.len()) {
        buf.extend_from_slice(&chunk[..]);
        vec.extend_from_slice(&chunk[..]);

        assert_eq!(buf.len(), SIZE);
        assert!(!buf.is_empty());
        assert!(buf.is_full());
        assert_eq!(buf.as_slices().0, &vec[vec.len() - SIZE..SIZE]);
        assert_eq!(buf.as_slices().1, &vec[SIZE..]);
    }
}
