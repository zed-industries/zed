//! A slow, but clear reference implementation of SeaHash.
//!
//! # Specification
//!
//! The input buffer is padded with null bytes until the length is divisible by 8.
//!
//! We start out with state
//!
//! ```notest
//! a = 0x16f11fe89b0d677c
//! b = 0xb480a793d8e6c86c
//! c = 0x6fe2e5aaf078ebc9
//! d = 0x14f994a4c5259381
//! ```
//!
//! If a seed is given, each of the initial state component are modularly multiplied by the seed.
//!
//! From the stream, we read one 64-bit block (in little-endian) at a time.  This number, `n`,
//! determines the new state by:
//!
//! ```notest
//! a' = b
//! b' = c
//! c' = d
//! d' = g(a ⊕ n)
//! ```
//!
//! `g(x)` is defined as `g(x) = j(h(j(x)))` with `h(x) = (x ≫ 32) ≫ (x ≫ 60)` and `j(x) ≡ px (mod
//! 2^64)` with `p = 0x7ed0e9fa0d94a33`.
//!
//! Let the final state be `(x, y, z, w)`. Then the final result is given by `H = g(x ⊕ y ⊕ z ⊕ w ⊕
//! l)` where `l` is the number of bytes in the original buffer.

use helper;

/// Read an integer in little-endian.
fn read_int(int: &[u8]) -> u64 {
    debug_assert!(
        int.len() <= 8,
        "The buffer length of the integer must be less than or equal to \
                  the one of an u64."
    );

    // Start at 0.
    let mut x = 0;
    for &i in int.iter().rev() {
        // Shift up a byte.
        x <<= 8;
        // Set the lower byte.
        x |= i as u64;
    }

    x
}

/// A hash state.
struct State {
    /// The `a` substate.
    a: u64,
    /// The `b` substate.
    b: u64,
    /// The `c` substate.
    c: u64,
    /// The `d` substate.
    d: u64,
}

impl State {
    /// Write a 64-bit integer to the state.
    fn write_u64(&mut self, x: u64) {
        let mut a = self.a;

        // Mix `x` into `a`.
        a = helper::diffuse(a ^ x);

        //  Rotate around.
        //  _______________________
        // |                       v
        // a <---- b <---- c <---- d
        self.a = self.b;
        self.b = self.c;
        self.c = self.d;
        self.d = a;
    }

    /// Calculate the final hash.
    fn finish(self, total: usize) -> u64 {
        // Even though XORing is commutative, it doesn't matter, because the state vector's initial
        // components are mutually distinct, and thus swapping even and odd chunks will affect the
        // result, because it is sensitive to the initial condition. To add discreteness, we
        // diffuse.
        helper::diffuse(
            self.a ^ self.b ^ self.c ^ self.d
            // We XOR in the number of written bytes to make it zero-sensitive when excessive bytes
            // are written (0u32.0u8 ≠ 0u16.0u8).
            ^ total as u64,
        )
    }

    /// Create a new state with some initial values (seed).
    fn with_seeds(k1: u64, k2: u64, k3: u64, k4: u64) -> State {
        State {
            // These values are randomly generated.
            a: k1,
            b: k2,
            c: k3,
            d: k4,
        }
    }
}

/// A reference implementation of SeaHash.
///
/// This is bloody slow when compared to the optimized version. This is because SeaHash was
/// specifically designed to take all sorts of hardware and software hacks into account to achieve
/// maximal performance, but this makes code significantly less readable. As such, this version has
/// only one goal: to make the algorithm readable and understandable.
pub fn hash(buf: &[u8]) -> u64 {
    hash_seeded(
        buf,
        0x16f11fe89b0d677c,
        0xb480a793d8e6c86c,
        0x6fe2e5aaf078ebc9,
        0x14f994a4c5259381,
    )
}

/// The seeded version of the reference implementation.
pub fn hash_seeded(buf: &[u8], k1: u64, k2: u64, k3: u64, k4: u64) -> u64 {
    // Initialize the state.
    let mut state = State::with_seeds(k1, k2, k3, k4);

    // Partition the rounded down buffer into chunks of 8 bytes, and iterate over them. The last
    // block might not be 8 bytes long.
    for int in buf.chunks(8) {
        // Read the chunk into an integer and write into the state.
        state.write_u64(read_int(int));
    }

    // Finish the hash state and return the final value.
    state.finish(buf.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shakespear() {
        assert_eq!(hash(b"to be or not to be"), 1988685042348123509);
    }
}
