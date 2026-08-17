use std::hash::Hasher;
use std::slice;

use helper;

/// The streaming version of the algorithm.
#[derive(Clone, Copy)]
pub struct SeaHasher {
    /// The state of the hasher.
    state: (u64, u64, u64, u64),
    /// The number of bytes we have written in total
    written: u64,
    /// Our tail
    tail: u64,
    /// The number of bytes in the tail
    ntail: usize,
}

impl Default for SeaHasher {
    fn default() -> SeaHasher {
        SeaHasher::with_seeds(
            0x16f11fe89b0d677c,
            0xb480a793d8e6c86c,
            0x6fe2e5aaf078ebc9,
            0x14f994a4c5259381,
        )
    }
}

impl SeaHasher {
    /// Create a new `SeaHasher` with default state.
    pub fn new() -> SeaHasher {
        SeaHasher::default()
    }

    /// Construct a new `SeaHasher` given some seed.
    ///
    /// For maximum quality, these seeds should be chosen at random.
    pub fn with_seeds(k1: u64, k2: u64, k3: u64, k4: u64) -> SeaHasher {
        SeaHasher {
            state: (k1, k2, k3, k4),
            written: 0,
            tail: 0,
            ntail: 0,
        }
    }

    #[inline(always)]
    fn push(&mut self, x: u64) {
        let a = helper::diffuse(self.state.0 ^ x);
        self.state.0 = self.state.1;
        self.state.1 = self.state.2;
        self.state.2 = self.state.3;
        self.state.3 = a;
        self.written += 8;
    }

    #[inline(always)]
    fn push_bytes(&mut self, bytes: &[u8]) {
        // The start of the bytes that aren't in the tail
        let copied = core::cmp::min(8 - self.ntail, bytes.len());
        unsafe {
            let mut this = self.tail.to_le_bytes();
            let mut ptr = bytes.as_ptr();
            ptr.copy_to_nonoverlapping(this.as_mut_ptr().add(self.ntail), copied);
            // It will be at most 8
            if copied + self.ntail != 8 {
                self.ntail += copied;
                self.tail = u64::from_le_bytes(this);
            } else {
                self.push(u64::from_le_bytes(this));
                self.ntail = 0;
                self.tail = 0;

                // We've done the existing tail, now just do the rest in chunks of 4 x u64.
                ptr = ptr.offset(copied as isize);
                let end_ptr = ptr.offset((bytes.len() - copied) as isize & !0x1F);
                while end_ptr > ptr {
                    self.state.0 = helper::diffuse(self.state.0 ^ helper::read_u64(ptr));
                    self.state.1 = helper::diffuse(self.state.1 ^ helper::read_u64(ptr.offset(8)));
                    self.state.2 = helper::diffuse(self.state.2 ^ helper::read_u64(ptr.offset(16)));
                    self.state.3 = helper::diffuse(self.state.3 ^ helper::read_u64(ptr.offset(24)));

                    ptr = ptr.offset(32);
                    self.written += 32;
                }
                let mut excessive = bytes.len() + bytes.as_ptr() as usize - ptr as usize;
                match excessive {
                    0 => {
                        // input was a multiple of 4 x u64 bytes long; no new tail bytes.
                    }
                    1..=7 => {
                        self.tail =
                            helper::read_int(slice::from_raw_parts(ptr as *const u8, excessive));
                        self.ntail = excessive;
                        // self.written does not need to be updated as we only gathered self.tail
                        // bytes after larger chunks.
                    }
                    8 => {
                        self.push(helper::read_u64(ptr));
                        // self.written is updated by self.push
                    }
                    9..=15 => {
                        self.push(helper::read_u64(ptr));
                        excessive -= 8;
                        self.tail =
                            helper::read_int(slice::from_raw_parts(ptr.offset(8), excessive));
                        self.ntail = excessive;
                        // self.written is updated by self.push
                    }
                    16 => {
                        let a = helper::diffuse(self.state.0 ^ helper::read_u64(ptr));
                        let b = helper::diffuse(self.state.1 ^ helper::read_u64(ptr.offset(8)));
                        // rotate
                        self.state.0 = self.state.2;
                        self.state.1 = self.state.3;
                        self.state.2 = a;
                        self.state.3 = b;
                        self.written += 16;
                    }
                    17..=23 => {
                        let a = helper::diffuse(self.state.0 ^ helper::read_u64(ptr));
                        let b = helper::diffuse(self.state.1 ^ helper::read_u64(ptr.offset(8)));
                        // rotate
                        self.state.0 = self.state.2;
                        self.state.1 = self.state.3;
                        self.state.2 = a;
                        self.state.3 = b;
                        excessive -= 16;
                        self.tail =
                            helper::read_int(slice::from_raw_parts(ptr.offset(16), excessive));
                        self.ntail = excessive;
                        self.written += 16;
                    }
                    24 => {
                        let a = helper::diffuse(self.state.0 ^ helper::read_u64(ptr));
                        let b = helper::diffuse(self.state.1 ^ helper::read_u64(ptr.offset(8)));
                        let c = helper::diffuse(self.state.2 ^ helper::read_u64(ptr.offset(16)));
                        self.state.0 = self.state.3;
                        self.state.1 = a;
                        self.state.2 = b;
                        self.state.3 = c;
                        self.written += 24;
                    }
                    _ => {
                        let a = helper::diffuse(self.state.0 ^ helper::read_u64(ptr));
                        let b = helper::diffuse(self.state.1 ^ helper::read_u64(ptr.offset(8)));
                        let c = helper::diffuse(self.state.2 ^ helper::read_u64(ptr.offset(16)));
                        self.state.0 = self.state.3;
                        self.state.1 = a;
                        self.state.2 = b;
                        self.state.3 = c;
                        excessive -= 24;
                        self.tail =
                            helper::read_int(slice::from_raw_parts(ptr.offset(24), excessive));
                        self.ntail = excessive;
                        self.written += 24;
                    }
                }
            }
        }
    }
}

impl Hasher for SeaHasher {
    fn finish(&self) -> u64 {
        let a = if self.ntail > 0 {
            let tail = helper::read_int(&self.tail.to_le_bytes()[..self.ntail]);
            helper::diffuse(self.state.0 ^ tail)
        } else {
            self.state.0
        };
        helper::diffuse(
            a ^ self.state.1 ^ self.state.2 ^ self.state.3 ^ self.written + self.ntail as u64,
        )
    }

    fn write(&mut self, bytes: &[u8]) {
        self.push_bytes(bytes)
    }

    fn write_u64(&mut self, n: u64) {
        self.write(&n.to_le_bytes())
    }

    fn write_u8(&mut self, n: u8) {
        self.write(&n.to_le_bytes())
    }

    fn write_u16(&mut self, n: u16) {
        self.write(&n.to_le_bytes())
    }

    fn write_u32(&mut self, n: u32) {
        self.write(&n.to_le_bytes())
    }

    fn write_usize(&mut self, n: usize) {
        self.write(&n.to_le_bytes())
    }

    fn write_i64(&mut self, n: i64) {
        self.write(&n.to_le_bytes())
    }

    fn write_i8(&mut self, n: i8) {
        self.write(&n.to_le_bytes())
    }

    fn write_i16(&mut self, n: i16) {
        self.write(&n.to_le_bytes())
    }

    fn write_i32(&mut self, n: i32) {
        self.write(&n.to_le_bytes())
    }

    fn write_isize(&mut self, n: isize) {
        self.write(&n.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash_seeded;
    use std::hash::Hasher;

    #[test]
    fn chunked_equiv() {
        let test_buf: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        let mut stream_hasher1 = SeaHasher::default();
        Hasher::write(&mut stream_hasher1, test_buf);

        let mut stream_hasher2 = SeaHasher::default();
        Hasher::write(&mut stream_hasher2, &test_buf[..8]);
        Hasher::write(&mut stream_hasher2, &test_buf[8..]);

        let mut stream_hasher3 = SeaHasher::default();
        Hasher::write(&mut stream_hasher3, &test_buf[..3]);
        Hasher::write(&mut stream_hasher3, &test_buf[3..]);

        let mut stream_hasher4 = SeaHasher::default();
        Hasher::write_u16(&mut stream_hasher4, 0xffff);
        Hasher::write_u16(&mut stream_hasher4, 0xffff);
        Hasher::write_u32(&mut stream_hasher4, 0xffffffff);
        Hasher::write_u64(&mut stream_hasher4, 0);

        assert_eq!(stream_hasher1.finish(), stream_hasher2.finish());
        assert_eq!(stream_hasher1.finish(), stream_hasher3.finish());
        assert_eq!(stream_hasher1.finish(), stream_hasher4.finish());
    }

    #[test]
    fn match_optimized() {
        let test_buf: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        let mut sea_hasher = SeaHasher::with_seeds(
            0xe7b0c93ca8525013,
            0x011d02b854ae8182,
            0x7bcc5cf9c39cec76,
            0xfa336285d102d083,
        );
        sea_hasher.write(test_buf);
        let stream_hash = sea_hasher.finish();

        let buffer_hash = hash_seeded(
            test_buf,
            0xe7b0c93ca8525013,
            0x011d02b854ae8182,
            0x7bcc5cf9c39cec76,
            0xfa336285d102d083,
        );

        assert_eq!(buffer_hash, stream_hash)
    }
}
