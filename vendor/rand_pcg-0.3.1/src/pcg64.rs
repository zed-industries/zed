// Copyright 2018 Developers of the Rand project.
// Copyright 2017 Paul Dicker.
// Copyright 2014-2017 Melissa O'Neill and PCG Project contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! PCG random number generators

use core::fmt;
use rand_core::{impls, le, Error, RngCore, SeedableRng};
#[cfg(feature = "serde1")] use serde::{Deserialize, Serialize};

// This is the default multiplier used by PCG for 64-bit state.
const MULTIPLIER: u64 = 6364136223846793005;

/// A PCG random number generator (XSH RR 64/32 (LCG) variant).
///
/// Permuted Congruential Generator with 64-bit state, internal Linear
/// Congruential Generator, and 32-bit output via "xorshift high (bits),
/// random rotation" output function.
///
/// This is a 64-bit LCG with explicitly chosen stream with the PCG-XSH-RR
/// output function. This combination is the standard `pcg32`.
///
/// Despite the name, this implementation uses 16 bytes (128 bit) space
/// comprising 64 bits of state and 64 bits stream selector. These are both set
/// by `SeedableRng`, using a 128-bit seed.
///
/// Note that two generators with different stream parameter may be closely
/// correlated.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Lcg64Xsh32 {
    state: u64,
    increment: u64,
}

/// [`Lcg64Xsh32`] is also officially known as `pcg32`.
pub type Pcg32 = Lcg64Xsh32;

impl Lcg64Xsh32 {
    /// Multi-step advance functions (jump-ahead, jump-back)
    ///
    /// The method used here is based on Brown, "Random Number Generation
    /// with Arbitrary Stride,", Transactions of the American Nuclear
    /// Society (Nov. 1994).  The algorithm is very similar to fast
    /// exponentiation.
    ///
    /// Even though delta is an unsigned integer, we can pass a
    /// signed integer to go backwards, it just goes "the long way round".
    ///
    /// Using this function is equivalent to calling `next_32()` `delta`
    /// number of times.
    #[inline]
    pub fn advance(&mut self, delta: u64) {
        let mut acc_mult: u64 = 1;
        let mut acc_plus: u64 = 0;
        let mut cur_mult = MULTIPLIER;
        let mut cur_plus = self.increment;
        let mut mdelta = delta;

        while mdelta > 0 {
            if (mdelta & 1) != 0 {
                acc_mult = acc_mult.wrapping_mul(cur_mult);
                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
            }
            cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
            cur_mult = cur_mult.wrapping_mul(cur_mult);
            mdelta /= 2;
        }
        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
    }

    /// Construct an instance compatible with PCG seed and stream.
    ///
    /// Note that two generators with different stream parameters may be closely
    /// correlated.
    ///
    /// PCG specifies the following default values for both parameters:
    ///
    /// - `state = 0xcafef00dd15ea5e5`
    /// - `stream = 0xa02bdbf7bb3c0a7`
    // Note: stream is 1442695040888963407u64 >> 1
    pub fn new(state: u64, stream: u64) -> Self {
        // The increment must be odd, hence we discard one bit:
        let increment = (stream << 1) | 1;
        Lcg64Xsh32::from_state_incr(state, increment)
    }

    #[inline]
    fn from_state_incr(state: u64, increment: u64) -> Self {
        let mut pcg = Lcg64Xsh32 { state, increment };
        // Move away from inital value:
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }

    #[inline]
    fn step(&mut self) {
        // prepare the LCG for the next round
        self.state = self
            .state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(self.increment);
    }
}

// Custom Debug implementation that does not expose the internal state
impl fmt::Debug for Lcg64Xsh32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lcg64Xsh32 {{}}")
    }
}

/// We use a single 127-bit seed to initialise the state and select a stream.
/// One `seed` bit (lowest bit of `seed[8]`) is ignored.
impl SeedableRng for Lcg64Xsh32 {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);

        // The increment must be odd, hence we discard one bit:
        Lcg64Xsh32::from_state_incr(seed_u64[0], seed_u64[1] | 1)
    }
}

impl RngCore for Lcg64Xsh32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        self.step();

        // Output function XSH RR: xorshift high (bits), followed by a random rotate
        // Constants are for 64-bit state, 32-bit output
        const ROTATE: u32 = 59; // 64 - 5
        const XSHIFT: u32 = 18; // (5 + 32) / 2
        const SPARE: u32 = 27; // 64 - 32 - 5

        let rot = (state >> ROTATE) as u32;
        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right(rot)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
