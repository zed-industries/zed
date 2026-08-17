//-
// Copyright 2017, 2018, 2019, 2020 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{Arc, String, ToOwned, Vec};
use core::result::Result;
use core::{fmt, str, u8, convert::TryInto};
use crate::test_runner::{config, RngSeed};
use rand::{self, Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use rand_xorshift::XorShiftRng;

/// Identifies a particular RNG algorithm supported by proptest.
///
/// Proptest supports dynamic configuration of algorithms to allow it to
/// continue operating with persisted regression files and to allow the
/// configuration to be expressed in the `Config` struct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngAlgorithm {
    /// The [XorShift](https://rust-random.github.io/rand/rand_xorshift/struct.XorShiftRng.html)
    /// algorithm. This was the default up through and including Proptest 0.9.0.
    ///
    /// It is faster than ChaCha but produces lower quality randomness and has
    /// some pathological cases where it may fail to produce outputs that are
    /// random even to casual observation.
    ///
    /// The seed must be exactly 16 bytes.
    XorShift,
    /// The [ChaCha](https://rust-random.github.io/rand/rand_chacha/struct.ChaChaRng.html)
    /// algorithm. This became the default with Proptest 0.9.1.
    ///
    /// The seed must be exactly 32 bytes.
    ChaCha,
    /// This is not an actual RNG algorithm, but instead returns data directly
    /// from its "seed".
    ///
    /// This is useful when Proptest is being driven from some other entropy
    /// source, such as a fuzzer.
    ///
    /// If the seed is depleted, the RNG will return 0s forever.
    ///
    /// Note that in cases where a new RNG is to be derived from an existing
    /// one, *the data is split evenly between them*, regardless of how much
    /// entropy is actually needed. This means that combinators like
    /// `prop_perturb` and `prop_flat_map` can require extremely large inputs.
    PassThrough,
    /// This is equivalent to the `ChaCha` RNG, with the addition that it
    /// records the bytes used to create a value.
    ///
    /// This is useful when Proptest is used for fuzzing, and a corpus of
    /// initial inputs need to be created. Note that in these cases, you need
    /// to use the `TestRunner` API directly yourself instead of using the
    /// `proptest!` macro, as otherwise there is no way to obtain the bytes
    /// this captures.
    Recorder,
    #[allow(missing_docs)]
    #[doc(hidden)]
    _NonExhaustive,
}

impl Default for RngAlgorithm {
    fn default() -> Self {
        RngAlgorithm::ChaCha
    }
}

impl RngAlgorithm {
    pub(crate) fn persistence_key(self) -> &'static str {
        match self {
            RngAlgorithm::XorShift => "xs",
            RngAlgorithm::ChaCha => "cc",
            RngAlgorithm::PassThrough => "pt",
            RngAlgorithm::Recorder => "rc",
            RngAlgorithm::_NonExhaustive => unreachable!(),
        }
    }

    pub(crate) fn from_persistence_key(k: &str) -> Option<Self> {
        match k {
            "xs" => Some(RngAlgorithm::XorShift),
            "cc" => Some(RngAlgorithm::ChaCha),
            "pt" => Some(RngAlgorithm::PassThrough),
            "rc" => Some(RngAlgorithm::Recorder),
            _ => None,
        }
    }
}

// These two are only used for parsing the environment variable
// PROPTEST_RNG_ALGORITHM.
impl str::FromStr for RngAlgorithm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        RngAlgorithm::from_persistence_key(s).ok_or(())
    }
}
impl fmt::Display for RngAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.persistence_key())
    }
}

/// Proptest's random number generator.
#[derive(Clone, Debug)]
pub struct TestRng {
    rng: TestRngImpl,
}

#[derive(Clone, Debug)]
enum TestRngImpl {
    XorShift(XorShiftRng),
    ChaCha(ChaChaRng),
    PassThrough {
        off: usize,
        end: usize,
        data: Arc<[u8]>,
    },
    Recorder {
        rng: ChaChaRng,
        record: Vec<u8>,
    },
}

impl RngCore for TestRng {
    fn next_u32(&mut self) -> u32 {
        match &mut self.rng {
            TestRngImpl::XorShift(rng) => rng.next_u32(),
            TestRngImpl::ChaCha(rng) => rng.next_u32(),
            TestRngImpl::PassThrough { .. } => {
                let mut buf = [0; 4];
                self.fill_bytes(&mut buf[..]);
                u32::from_le_bytes(buf)
            }
            TestRngImpl::Recorder { rng, record } => {
                let read = rng.next_u32();
                record.extend_from_slice(&read.to_le_bytes());
                read
            }
        }
    }

    fn next_u64(&mut self) -> u64 {
        match &mut self.rng {
            TestRngImpl::XorShift(rng) => rng.next_u64(),
            TestRngImpl::ChaCha(rng) => rng.next_u64(),
            TestRngImpl::PassThrough { .. } => {
                let mut buf = [0; 8];
                self.fill_bytes(&mut buf[..]);
                u64::from_le_bytes(buf)
            }
            TestRngImpl::Recorder { rng, record } => {
                let read = rng.next_u64();
                record.extend_from_slice(&read.to_le_bytes());
                read
            }
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        match &mut self.rng {
            TestRngImpl::XorShift(rng) => rng.fill_bytes(dest),
            TestRngImpl::ChaCha(rng) => rng.fill_bytes(dest),
            TestRngImpl::PassThrough { off, end, data } => {
                let bytes_to_copy = dest.len().min(*end - *off);
                dest[.. bytes_to_copy].copy_from_slice(&data[*off .. *off + bytes_to_copy]);
                *off += bytes_to_copy;
                for i in bytes_to_copy .. dest.len() {
                    dest[i] = 0;
                }
            }
            TestRngImpl::Recorder { rng, record } => {
                rng.fill_bytes(dest);
                record.extend_from_slice(dest);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Seed {
    XorShift([u8; 16]),
    ChaCha([u8; 32]),
    PassThrough(Option<(usize, usize)>, Arc<[u8]>),
    Recorder([u8; 32]),
}

impl Seed {
    pub(crate) fn from_bytes(algorithm: RngAlgorithm, seed: &[u8]) -> Self {
        match algorithm {
            RngAlgorithm::XorShift => {
                assert_eq!(16, seed.len(), "XorShift requires a 16-byte seed");
                let mut buf = [0; 16];
                buf.copy_from_slice(seed);
                Seed::XorShift(buf)
            }

            RngAlgorithm::ChaCha => {
                assert_eq!(32, seed.len(), "ChaCha requires a 32-byte seed");
                let mut buf = [0; 32];
                buf.copy_from_slice(seed);
                Seed::ChaCha(buf)
            }

            RngAlgorithm::PassThrough => Seed::PassThrough(None, seed.into()),

            RngAlgorithm::Recorder => {
                assert_eq!(32, seed.len(), "Recorder requires a 32-byte seed");
                let mut buf = [0; 32];
                buf.copy_from_slice(seed);
                Seed::Recorder(buf)
            }

            RngAlgorithm::_NonExhaustive => unreachable!(),
        }
    }

    pub(crate) fn from_persistence(string: &str) -> Option<Seed> {
        fn from_base16(dst: &mut [u8], src: &str) -> Option<()> {
            if dst.len() * 2 != src.len() {
                return None;
            }

            for (dst_byte, src_pair) in
                dst.into_iter().zip(src.as_bytes().chunks(2))
            {
                *dst_byte =
                    u8::from_str_radix(str::from_utf8(src_pair).ok()?, 16)
                        .ok()?;
            }

            Some(())
        }

        let parts =
            string.trim().split(char::is_whitespace).collect::<Vec<_>>();
        RngAlgorithm::from_persistence_key(&parts[0]).and_then(
            |alg| match alg {
                RngAlgorithm::XorShift => {
                    if 5 != parts.len() {
                        return None;
                    }

                    let mut dwords = [0u32; 4];
                    for (dword, part) in
                        (&mut dwords[..]).into_iter().zip(&parts[1..])
                    {
                        *dword = part.parse().ok()?;
                    }

                    let mut seed = [0u8; 16];
                    for (chunk, dword) in seed.chunks_mut(4).zip(dwords) {
                        chunk.copy_from_slice(&dword.to_le_bytes());
                    }
                    Some(Seed::XorShift(seed))
                }

                RngAlgorithm::ChaCha => {
                    if 2 != parts.len() {
                        return None;
                    }

                    let mut seed = [0u8; 32];
                    from_base16(&mut seed, &parts[1])?;
                    Some(Seed::ChaCha(seed))
                }

                RngAlgorithm::PassThrough => {
                    if 1 == parts.len() {
                        return Some(Seed::PassThrough(None, vec![].into()));
                    }

                    if 2 != parts.len() {
                        return None;
                    }

                    let mut seed = vec![0u8; parts[1].len() / 2];
                    from_base16(&mut seed, &parts[1])?;
                    Some(Seed::PassThrough(None, seed.into()))
                }

                RngAlgorithm::Recorder => {
                    if 2 != parts.len() {
                        return None;
                    }

                    let mut seed = [0u8; 32];
                    from_base16(&mut seed, &parts[1])?;
                    Some(Seed::Recorder(seed))
                }

                RngAlgorithm::_NonExhaustive => unreachable!(),
            },
        )
    }

    pub(crate) fn to_persistence(&self) -> String {
        fn to_base16(dst: &mut String, src: &[u8]) {
            for byte in src {
                dst.push_str(&format!("{:02x}", byte));
            }
        }

        match *self {
            Seed::XorShift(ref seed) => {
                let dwords = [
                    u32::from_le_bytes(seed[0..4].try_into().unwrap()),
                    u32::from_le_bytes(seed[4..8].try_into().unwrap()),
                    u32::from_le_bytes(seed[8..12].try_into().unwrap()),
                    u32::from_le_bytes(seed[12..16].try_into().unwrap()),
                ];
                format!(
                    "{} {} {} {} {}",
                    RngAlgorithm::XorShift.persistence_key(),
                    dwords[0],
                    dwords[1],
                    dwords[2],
                    dwords[3]
                )
            }

            Seed::ChaCha(ref seed) => {
                let mut string =
                    RngAlgorithm::ChaCha.persistence_key().to_owned();
                string.push(' ');
                to_base16(&mut string, seed);
                string
            }

            Seed::PassThrough(bounds, ref data) => {
                let data =
                    bounds.map_or(&data[..], |(start, end)| &data[start..end]);
                let mut string =
                    RngAlgorithm::PassThrough.persistence_key().to_owned();
                string.push(' ');
                to_base16(&mut string, data);
                string
            }

            Seed::Recorder(ref seed) => {
                let mut string =
                    RngAlgorithm::Recorder.persistence_key().to_owned();
                string.push(' ');
                to_base16(&mut string, seed);
                string
            }
        }
    }
}

impl TestRng {
    /// Create a new RNG with the given algorithm and seed.
    ///
    /// Any RNG created with the same algorithm-seed pair will produce the same
    /// sequence of values on all systems and all supporting versions of
    /// proptest.
    ///
    /// ## Panics
    ///
    /// Panics if `seed` is not an appropriate length for `algorithm`.
    pub fn from_seed(algorithm: RngAlgorithm, seed: &[u8]) -> Self {
        TestRng::from_seed_internal(Seed::from_bytes(algorithm, seed))
    }

    /// Dumps the bytes obtained from the RNG so far (only works if the RNG is
    /// set to `Recorder`).
    ///
    /// ## Panics
    ///
    /// Panics if this RNG does not capture generated data.
    pub fn bytes_used(&self) -> Vec<u8> {
        match self.rng {
            TestRngImpl::Recorder { ref record, .. } => record.clone(),
            _ => panic!("bytes_used() called on non-Recorder RNG"),
        }
    }

    /// Construct a default TestRng from entropy.
    pub(crate) fn default_rng(seed: config::RngSeed, algorithm: RngAlgorithm) -> Self {
        #[cfg(feature = "std")]
        {
            Self {
                rng: match algorithm {
                    RngAlgorithm::XorShift => {
                        let rng = match seed {
                            RngSeed::Random => XorShiftRng::from_os_rng(),
                            RngSeed::Fixed(seed) => XorShiftRng::seed_from_u64(seed),
                        };
                        TestRngImpl::XorShift(rng)
                    }
                    RngAlgorithm::ChaCha => {
                        let rng = match seed {
                            RngSeed::Random => ChaChaRng::from_os_rng(),
                            RngSeed::Fixed(seed) => ChaChaRng::seed_from_u64(seed),
                        };
                        TestRngImpl::ChaCha(rng)
                    }
                    RngAlgorithm::PassThrough => {
                        panic!("cannot create default instance of PassThrough")
                    }
                    RngAlgorithm::Recorder => {
                        let rng =  match seed {
                            RngSeed::Random => ChaChaRng::from_os_rng(),
                            RngSeed::Fixed(seed) => ChaChaRng::seed_from_u64(seed),
                        };
                        TestRngImpl::Recorder {rng, record: Vec::new()}
                    },
                    RngAlgorithm::_NonExhaustive => unreachable!(),
                },
            }
        }
        #[cfg(all(
            not(feature = "std"),
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "hardware-rng"
        ))]
        {
            return Self::hardware_rng(algorithm);
        }
        #[cfg(not(feature = "std"))]
        {
            return Self::deterministic_rng(algorithm);
        }
    }

    const SEED_FOR_XOR_SHIFT: [u8; 16] = [
        0xf4, 0x16, 0x16, 0x48, 0xc3, 0xac, 0x77, 0xac, 0x72, 0x20, 0x0b, 0xea,
        0x99, 0x67, 0x2d, 0x6d,
    ];

    const SEED_FOR_CHA_CHA: [u8; 32] = [
        0xf4, 0x16, 0x16, 0x48, 0xc3, 0xac, 0x77, 0xac, 0x72, 0x20, 0x0b, 0xea,
        0x99, 0x67, 0x2d, 0x6d, 0xca, 0x9f, 0x76, 0xaf, 0x1b, 0x09, 0x73, 0xa0,
        0x59, 0x22, 0x6d, 0xc5, 0x46, 0x39, 0x1c, 0x4a,
    ];

    /// Returns a `TestRng` with a seed generated with the
    /// RdRand instruction on x86 machines.
    ///
    /// This is useful in `no_std` scenarios on x86 where we don't
    /// have a random number infrastructure but the `rdrand` instruction is
    /// available.
    #[cfg(all(
        not(feature = "std"),
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "hardware-rng"
    ))]
    pub fn hardware_rng(algorithm: RngAlgorithm) -> Self {
        use x86::random::{rdrand_slice, RdRand};

        Self::from_seed_internal(match algorithm {
            RngAlgorithm::XorShift => {
                // Initialize to a sane seed just in case
                let mut seed: [u8; 16] = TestRng::SEED_FOR_XOR_SHIFT;
                unsafe {
                    let r = rdrand_slice(&mut seed);
                    debug_assert!(r, "hardware_rng should only be called on machines with support for rdrand");
                }
                Seed::XorShift(seed)
            }
            RngAlgorithm::ChaCha => {
                // Initialize to a sane seed just in case
                let mut seed: [u8; 32] = TestRng::SEED_FOR_CHA_CHA;
                unsafe {
                    let r = rdrand_slice(&mut seed);
                    debug_assert!(r, "hardware_rng should only be called on machines with support for rdrand");
                }
                Seed::ChaCha(seed)
            }
            RngAlgorithm::PassThrough => {
                panic!("deterministic RNG not available for PassThrough")
            }
            RngAlgorithm::Recorder => {
                // Initialize to a sane seed just in case
                let mut seed: [u8; 32] = TestRng::SEED_FOR_CHA_CHA;
                unsafe {
                    let r = rdrand_slice(&mut seed);
                    debug_assert!(r, "hardware_rng should only be called on machines with support for rdrand");
                }
                Seed::Recorder(seed)
            }
            RngAlgorithm::_NonExhaustive => unreachable!(),
        })
    }

    /// Returns a `TestRng` with a particular hard-coded seed.
    ///
    /// The seed value will always be the same for a particular version of
    /// Proptest and algorithm, but may change across releases.
    ///
    /// This is useful for testing things like strategy implementations without
    /// risking getting "unlucky" RNGs which deviate from average behaviour
    /// enough to cause spurious failures. For example, a strategy for `bool`
    /// which is supposed to produce `true` 50% of the time might have a test
    /// which checks that the distribution is "close enough" to 50%. If every
    /// test run starts with a different RNG, occasionally there will be
    /// spurious test failures when the RNG happens to produce a very skewed
    /// distribution. Using this or `TestRunner::deterministic()` avoids such
    /// issues.
    pub fn deterministic_rng(algorithm: RngAlgorithm) -> Self {
        Self::from_seed_internal(match algorithm {
            RngAlgorithm::XorShift => {
                Seed::XorShift(TestRng::SEED_FOR_XOR_SHIFT)
            }
            RngAlgorithm::ChaCha => Seed::ChaCha(TestRng::SEED_FOR_CHA_CHA),
            RngAlgorithm::PassThrough => {
                panic!("deterministic RNG not available for PassThrough")
            }
            RngAlgorithm::Recorder => Seed::Recorder(TestRng::SEED_FOR_CHA_CHA),
            RngAlgorithm::_NonExhaustive => unreachable!(),
        })
    }

    /// Construct a TestRng by the perturbed randomized seed
    /// from an existing TestRng.
    pub(crate) fn gen_rng(&mut self) -> Self {
        Self::from_seed_internal(self.new_rng_seed())
    }

    /// Overwrite the given TestRng with the provided seed.
    pub(crate) fn set_seed(&mut self, seed: Seed) {
        *self = Self::from_seed_internal(seed);
    }

    /// Generate a new randomized seed, set it to this TestRng,
    /// and return the seed.
    pub(crate) fn gen_get_seed(&mut self) -> Seed {
        let seed = self.new_rng_seed();
        self.set_seed(seed.clone());
        seed
    }

    /// Randomize a perturbed randomized seed from the given TestRng.
    pub(crate) fn new_rng_seed(&mut self) -> Seed {
        match self.rng {
            TestRngImpl::XorShift(ref mut rng) => {
                let mut seed = rng.random::<[u8; 16]>();

                // Directly using XorShiftRng::from_seed() at this point would
                // result in rng and the returned value being exactly the same.
                // Perturb the seed with some arbitrary values to prevent this.
                for word in seed.chunks_mut(4) {
                    word[3] ^= 0xde;
                    word[2] ^= 0xad;
                    word[1] ^= 0xbe;
                    word[0] ^= 0xef;
                }

                Seed::XorShift(seed)
            }

            TestRngImpl::ChaCha(ref mut rng) => Seed::ChaCha(rng.random()),

            TestRngImpl::PassThrough {
                ref mut off,
                ref mut end,
                ref data,
            } => {
                let len = *end - *off;
                let child_start = *off + len / 2;
                let child_end = *off + len;
                *end = child_start;
                Seed::PassThrough(
                    Some((child_start, child_end)),
                    Arc::clone(data),
                )
            }

            TestRngImpl::Recorder { ref mut rng, .. } => {
                Seed::Recorder(rng.random())
            }
        }
    }

    /// Construct a TestRng from a given seed.
    fn from_seed_internal(seed: Seed) -> Self {
        Self {
            rng: match seed {
                Seed::XorShift(seed) => {
                    TestRngImpl::XorShift(XorShiftRng::from_seed(seed))
                }

                Seed::ChaCha(seed) => {
                    TestRngImpl::ChaCha(ChaChaRng::from_seed(seed))
                }

                Seed::PassThrough(bounds, data) => {
                    let (start, end) = bounds.unwrap_or((0, data.len()));
                    TestRngImpl::PassThrough {
                        off: start,
                        end,
                        data,
                    }
                }

                Seed::Recorder(seed) => TestRngImpl::Recorder {
                    rng: ChaChaRng::from_seed(seed),
                    record: Vec::new(),
                },
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::std_facade::Vec;

    use rand::{Rng, RngCore};

    use super::{RngAlgorithm, Seed, TestRng};
    use crate::arbitrary::any;
    use crate::strategy::*;

    proptest! {
        #[test]
        fn gen_parse_seeds(
            seed in prop_oneof![
                any::<[u8;16]>().prop_map(Seed::XorShift),
                any::<[u8;32]>().prop_map(Seed::ChaCha),
                any::<Vec<u8>>().prop_map(|data| Seed::PassThrough(None, data.into())),
                any::<[u8;32]>().prop_map(Seed::Recorder),
            ])
        {
            assert_eq!(seed, Seed::from_persistence(&seed.to_persistence()).unwrap());
        }

        #[test]
        fn rngs_dont_clone_self_on_genrng(
            seed in prop_oneof![
                any::<[u8;16]>().prop_map(Seed::XorShift),
                any::<[u8;32]>().prop_map(Seed::ChaCha),
                Just(()).prop_perturb(|_, mut rng| {
                    let mut buf = vec![0u8; 2048];
                    rng.fill_bytes(&mut buf);
                    Seed::PassThrough(None, buf.into())
                }),
                any::<[u8;32]>().prop_map(Seed::Recorder),
            ])
        {
            type Value = [u8;32];
            let orig = TestRng::from_seed_internal(seed);

            {
                let mut rng1 = orig.clone();
                let mut rng2 = rng1.gen_rng();
                assert_ne!(rng1.random::<Value>(), rng2.random::<Value>());
            }

            {
                let mut rng1 = orig.clone();
                let mut rng2 = rng1.gen_rng();
                let mut rng3 = rng1.gen_rng();
                let mut rng4 = rng2.gen_rng();
                let a = rng1.random::<Value>();
                let b = rng2.random::<Value>();
                let c = rng3.random::<Value>();
                let d = rng4.random::<Value>();
                assert_ne!(a, b);
                assert_ne!(a, c);
                assert_ne!(a, d);
                assert_ne!(b, c);
                assert_ne!(b, d);
                assert_ne!(c, d);
            }
        }
    }

    #[test]
    fn passthrough_rng_behaves_properly() {
        let mut rng = TestRng::from_seed(
            RngAlgorithm::PassThrough,
            &[
                0xDE, 0xC0, 0x12, 0x34, 0x56, 0x78, 0xFE, 0xCA, 0xEF, 0xBE,
                0xAD, 0xDE, 0x01, 0x02, 0x03,
            ],
        );

        assert_eq!(0x3412C0DE, rng.next_u32());
        assert_eq!(0xDEADBEEFCAFE7856, rng.next_u64());

        let mut buf = [0u8; 4];
        rng.fill_bytes(&mut buf[0..4]);
        assert_eq!([1, 2, 3, 0], buf);
        rng.fill_bytes(&mut buf[0..4]);
        assert_eq!([0, 0, 0, 0], buf);
    }
}
