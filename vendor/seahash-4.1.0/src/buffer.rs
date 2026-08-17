//! A highly optimized version of SeaHash.

use std::slice;

use helper;

/// A SeaHash state.
#[derive(Clone)]
pub struct State {
    /// `a`
    a: u64,
    /// `b`
    b: u64,
    /// `c`
    c: u64,
    /// `d`
    d: u64,
    /// The number of written bytes.
    written: u64,
}

impl State {
    /// Create a new state vector with some initial values.
    pub fn new(a: u64, b: u64, c: u64, d: u64) -> State {
        State {
            a: a,
            b: b,
            c: c,
            d: d,
            written: 0,
        }
    }

    /// Hash a buffer with some seed.
    pub fn hash(buf: &[u8], (mut a, mut b, mut c, mut d): (u64, u64, u64, u64)) -> State {
        unsafe {
            // We use 4 different registers to store seperate hash states, because this allows us
            // to update them seperately, and consequently exploiting ILP to update the states in
            // parallel.

            // The pointer to the current bytes.
            let mut ptr = buf.as_ptr();
            // The end of the "main segment", i.e. the biggest buffer s.t. the length is divisible
            // by 32.
            let end_ptr = buf.as_ptr().offset(buf.len() as isize & !0x1F);

            while end_ptr > ptr {
                // Modern CPUs allow the pointer arithmetic to be done in place, hence not
                // introducing tmpvars.
                a ^= helper::read_u64(ptr);
                b ^= helper::read_u64(ptr.offset(8));
                c ^= helper::read_u64(ptr.offset(16));
                d ^= helper::read_u64(ptr.offset(24));

                // Increment the pointer.
                ptr = ptr.offset(32);

                // Diffuse the updated registers. We hope that each of these are executed in
                // parallel.
                a = helper::diffuse(a);
                b = helper::diffuse(b);
                c = helper::diffuse(c);
                d = helper::diffuse(d);
            }

            // Calculate the number of excessive bytes. These are bytes that could not be handled
            // in the loop above.
            let mut excessive = buf.len() as usize + buf.as_ptr() as usize - end_ptr as usize;
            // Handle the excessive bytes.
            match excessive {
                0 => {}
                1..=7 => {
                    // 1 or more excessive.

                    // Write the last excessive bytes (<8 bytes).
                    a ^= helper::read_int(slice::from_raw_parts(ptr as *const u8, excessive));

                    // Diffuse.
                    a = helper::diffuse(a);
                }
                8 => {
                    // 8 bytes excessive.

                    // Mix in the partial block.
                    a ^= helper::read_u64(ptr);

                    // Diffuse.
                    a = helper::diffuse(a);
                }
                9..=15 => {
                    // More than 8 bytes excessive.

                    // Mix in the partial block.
                    a ^= helper::read_u64(ptr);

                    // Write the last excessive bytes (<8 bytes).
                    excessive = excessive - 8;
                    b ^= helper::read_int(slice::from_raw_parts(ptr.offset(8), excessive));

                    // Diffuse.
                    a = helper::diffuse(a);
                    b = helper::diffuse(b);
                }
                16 => {
                    // 16 bytes excessive.

                    // Mix in the partial block.
                    a = helper::diffuse(a ^ helper::read_u64(ptr));
                    b = helper::diffuse(b ^ helper::read_u64(ptr.offset(8)));
                }
                17..=23 => {
                    // 16 bytes or more excessive.

                    // Mix in the partial block.
                    a ^= helper::read_u64(ptr);
                    b ^= helper::read_u64(ptr.offset(8));

                    // Write the last excessive bytes (<8 bytes).
                    excessive = excessive - 16;
                    c ^= helper::read_int(slice::from_raw_parts(ptr.offset(16), excessive));

                    // Diffuse.
                    a = helper::diffuse(a);
                    b = helper::diffuse(b);
                    c = helper::diffuse(c);
                }
                24 => {
                    // 24 bytes excessive.

                    // Mix in the partial block.
                    a ^= helper::read_u64(ptr);
                    b ^= helper::read_u64(ptr.offset(8));
                    c ^= helper::read_u64(ptr.offset(16));

                    // Diffuse.
                    a = helper::diffuse(a);
                    b = helper::diffuse(b);
                    c = helper::diffuse(c);
                }
                _ => {
                    // More than 24 bytes excessive.

                    // Mix in the partial block.
                    a ^= helper::read_u64(ptr);
                    b ^= helper::read_u64(ptr.offset(8));
                    c ^= helper::read_u64(ptr.offset(16));

                    // Write the last excessive bytes (<8 bytes).
                    excessive = excessive - 24;
                    d ^= helper::read_int(slice::from_raw_parts(ptr.offset(24), excessive));

                    // Diffuse.
                    a = helper::diffuse(a);
                    b = helper::diffuse(b);
                    c = helper::diffuse(c);
                    d = helper::diffuse(d);
                }
            }
        }

        State {
            a: a,
            b: b,
            c: c,
            d: d,
            written: buf.len() as u64,
        }
    }

    /// Write another 64-bit integer into the state.
    pub fn push(&mut self, x: u64) {
        // Mix `x` into `a`.
        let a = helper::diffuse(self.a ^ x);

        //  Rotate around.
        //  _______________________
        // |                       v
        // a <---- b <---- c <---- d
        self.a = self.b;
        self.b = self.c;
        self.c = self.d;
        self.d = a;

        // Increase the written bytes counter.
        self.written += 8;
    }

    /// Remove the most recently written 64-bit integer from the state.
    ///
    /// Given the value of the most recently written u64 `last`, remove it from the state.
    pub fn pop(&mut self, last: u64) {
        // Un-mix `last` from `d`. Removes the recently written data.
        let d = helper::undiffuse(self.d) ^ last;

        //  Rotate back.
        //  _______________________
        // v                       |
        // a ----> b ----> c ----> d
        self.d = self.c;
        self.c = self.b;
        self.b = self.a;
        self.a = d;

        // Decrese the written bytes counter.
        self.written -= 8;
    }

    /// Finalize the state.
    #[inline]
    pub fn finalize(self) -> u64 {
        let State {
            written,
            mut a,
            b,
            mut c,
            d,
        } = self;

        // XOR the states together. Even though XOR is commutative, it doesn't matter, because the
        // state vector's initial components are mutually distinct, and thus swapping even and odd
        // chunks will affect the result, because it is sensitive to the initial condition.
        a ^= b;
        c ^= d;
        a ^= c;
        // XOR the number of written bytes in order to make the excessive bytes zero-sensitive
        // (without this, two excessive zeros would be equivalent to three excessive zeros). This
        // is know as length padding.
        a ^= written;

        // We diffuse to make the excessive bytes discrete (i.e. small changes shouldn't give small
        // changes in the output).
        helper::diffuse(a)
    }
}

/// Hash some buffer.
///
/// This is a highly optimized implementation of SeaHash. It implements numerous techniques to
/// improve performance:
///
/// - Register allocation: This makes a great deal out of making sure everything fits into
///   registers such that minimal memory accesses are needed. This works quite successfully on most
///   CPUs, and the only time it reads from memory is when it fetches the data of the buffer.
/// - Bulk reads: Like most other good hash functions, we read 8 bytes a time. This obviously
///   improves performance a lot
/// - Independent updates: We make sure very few statements next to each other depends on the
///   other. This means that almost always the CPU will be able to run the instructions in parallel.
/// - Loop unrolling: The hot loop is unrolled such that very little branches (one every 32 bytes)
///   are needed.
///
/// and more.
///
/// The seed of this hash function is prechosen.
pub fn hash(buf: &[u8]) -> u64 {
    hash_seeded(
        buf,
        0x16f11fe89b0d677c,
        0xb480a793d8e6c86c,
        0x6fe2e5aaf078ebc9,
        0x14f994a4c5259381,
    )
}

/// Hash some buffer according to a chosen seed.
///
/// The keys are expected to be chosen from a uniform distribution. The keys should be mutually
/// distinct to avoid issues with collisions if the lanes are permuted.
///
/// This is not secure, as [the key can be extracted with a bit of computational
/// work](https://github.com/ticki/tfs/issues/5), as such, it is recommended to have a fallback
/// hash function (adaptive hashing) in the case of hash flooding. It can be considered unbroken if
/// the output is not known (i.e. no malicious party has access to the raw values of the keys, only
/// a permutation thereof).), however I absolutely do not recommend using it for this. If you want
/// to be strict, this should only be used as a layer of obfuscation, such that the fallback (e.g.
/// SipHash) is harder to trigger.
///
/// In the future, I might strengthen the security if possible while having backward compatibility
/// with the default initialization vector.
pub fn hash_seeded(buf: &[u8], a: u64, b: u64, c: u64, d: u64) -> u64 {
    State::hash(buf, (a, b, c, d)).finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    use reference;

    fn hash_match(a: &[u8]) {
        assert_eq!(hash(a), reference::hash(a));
        assert_eq!(
            hash_seeded(a, 1, 1, 1, 1),
            reference::hash_seeded(a, 1, 1, 1, 1)
        );
        assert_eq!(
            hash_seeded(a, 500, 2873, 2389, 9283),
            reference::hash_seeded(a, 500, 2873, 2389, 9283)
        );
        assert_eq!(
            hash_seeded(a, 238945723984, 872894734, 239478243, 28937498234),
            reference::hash_seeded(a, 238945723984, 872894734, 239478243, 28937498234)
        );
        assert_eq!(
            hash_seeded(a, !0, !0, !0, !0),
            reference::hash_seeded(a, !0, !0, !0, !0)
        );
        assert_eq!(
            hash_seeded(a, 0, 0, 0, 0),
            reference::hash_seeded(a, 0, 0, 0, 0)
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)] // very slow to run on miri
    fn zero() {
        let arr = [0; 4096];
        for n in 0..4096 {
            hash_match(&arr[0..n]);
        }
    }

    #[test]
    fn seq() {
        let mut buf = [0; 4096];
        for i in 0..4096 {
            buf[i] = i as u8;
        }
        hash_match(&buf);
    }

    #[test]
    fn position_depedent() {
        let mut buf1 = [0; 4098];
        for i in 0..4098 {
            buf1[i] = i as u8;
        }
        let mut buf2 = [0; 4098];
        for i in 0..4098 {
            buf2[i] = i as u8 ^ 1;
        }

        assert!(hash(&buf1) != hash(&buf2));
    }

    #[test]
    fn shakespear() {
        hash_match(b"to be or not to be");
        hash_match(b"love is a wonderful terrible thing");
    }

    #[test]
    fn zero_senitive() {
        assert_ne!(hash(&[1, 2, 3, 4]), hash(&[1, 0, 2, 3, 4]));
        assert_ne!(hash(&[1, 2, 3, 4]), hash(&[1, 0, 0, 2, 3, 4]));
        assert_ne!(hash(&[1, 2, 3, 4]), hash(&[1, 2, 3, 4, 0]));
        assert_ne!(hash(&[1, 2, 3, 4]), hash(&[0, 1, 2, 3, 4]));
        assert_ne!(hash(&[0, 0, 0]), hash(&[0, 0, 0, 0, 0]));
    }

    #[test]
    fn not_equal() {
        assert_ne!(hash(b"to be or not to be "), hash(b"to be or not to be"));
        assert_ne!(hash(b"jkjke"), hash(b"jkjk"));
        assert_ne!(hash(b"ijkjke"), hash(b"ijkjk"));
        assert_ne!(hash(b"iijkjke"), hash(b"iijkjk"));
        assert_ne!(hash(b"iiijkjke"), hash(b"iiijkjk"));
        assert_ne!(hash(b"iiiijkjke"), hash(b"iiiijkjk"));
        assert_ne!(hash(b"iiiiijkjke"), hash(b"iiiiijkjk"));
        assert_ne!(hash(b"iiiiiijkjke"), hash(b"iiiiiijkjk"));
        assert_ne!(hash(b"iiiiiiijkjke"), hash(b"iiiiiiijkjk"));
        assert_ne!(hash(b"iiiiiiiijkjke"), hash(b"iiiiiiiijkjk"));
        assert_ne!(hash(b"ab"), hash(b"bb"));
    }

    #[test]
    fn push() {
        let mut state = State::new(1, 2, 3, 4);
        state.push(!0);
        state.push(0);

        assert_eq!(
            hash_seeded(
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0],
                1,
                2,
                3,
                4
            ),
            state.finalize()
        );
    }

    #[test]
    fn pop() {
        let mut state = State::new(1, 2, 3, 4);
        state.push(!0);
        state.push(0);
        state.pop(0);

        assert_eq!(
            hash_seeded(
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                1,
                2,
                3,
                4
            ),
            state.finalize()
        );
    }
}
