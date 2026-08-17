// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Pseudo random number generators are algorithms to produce *apparently
//! random* numbers deterministically, and usually fairly quickly.
//! 
//! So long as the algorithm is computationally secure, is initialised with
//! sufficient entropy (i.e. unknown by an attacker), and its internal state is
//! also protected (unknown to an attacker), the output will also be
//! *computationally secure*. Computationally Secure Pseudo Random Number
//! Generators (CSPRNGs) are thus suitable sources of random numbers for
//! cryptography. There are a couple of gotchas here, however. First, the seed
//! used for initialisation must be unknown. Usually this should be provided by
//! the operating system and should usually be secure, however this may not
//! always be the case (especially soon after startup). Second, user-space
//! memory may be vulnerable, for example when written to swap space, and after
//! forking a child process should reinitialise any user-space PRNGs. For this
//! reason it may be preferable to source random numbers directly from the OS
//! for cryptographic applications.
//! 
//! PRNGs are also widely used for non-cryptographic uses: randomised
//! algorithms, simulations, games. In these applications it is usually not
//! important for numbers to be cryptographically *unguessable*, but even
//! distribution and independence from other samples (from the point of view
//! of someone unaware of the algorithm used, at least) may still be important.
//! Good PRNGs should satisfy these properties, but do not take them for
//! granted; Wikipedia's article on 
//! [Pseudorandom number generators](https://en.wikipedia.org/wiki/Pseudorandom_number_generator)
//! provides some background on this topic.
//! 
//! Care should be taken when seeding (initialising) PRNGs. Some PRNGs have
//! short periods for some seeds. If one PRNG is seeded from another using the
//! same algorithm, it is possible that both will yield the same sequence of
//! values (with some lag).

mod chacha;
mod isaac;
mod isaac64;
mod xorshift;

pub use self::chacha::ChaChaRng;
pub use self::isaac::IsaacRng;
pub use self::isaac64::Isaac64Rng;
pub use self::xorshift::XorShiftRng;
