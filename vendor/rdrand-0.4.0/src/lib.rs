// Copyright © 2014, Simonas Kazlauskas <rdrand@kazlauskas.me>
//
// Permission to use, copy, modify, and/or distribute this software for any purpose with or without
// fee is hereby granted, provided that the above copyright notice and this permission notice
// appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
// SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
// AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
// NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//! An implementation of random number generators based on `rdrand` and `rdseed` instructions.
//!
//! The random number generators provided by this crate are fairly slow (the latency for these
//! instructions is pretty high), but provide high quality random bits. Caveat is: neither AMD’s
//! nor Intel’s designs are public and therefore are not verifiable for lack of backdoors.
//!
//! Unless you know what you are doing, use the random number generators provided by the `rand`
//! crate (such as `OsRng`) instead.
//!
//! Here are a measurements for select processor architectures. Check [Agner’s instruction tables]
//! for up-to-date listings.
//!
//! <table>
//!   <tr>
//!     <th>Architecture</th>
//!     <th colspan="3">Latency (cycles)</th>
//!     <th>Maximum throughput (per core)</th>
//!   </tr>
//!   <tr>
//!     <td></td>
//!     <td>u16</td>
//!     <td>u32</td>
//!     <td>u64</td>
//!     <td></td>
//!   </tr>
//!   <tr>
//!     <td>AMD Ryzen</td>
//!     <td>~1200</td>
//!     <td>~1200</td>
//!     <td>~2500</td>
//!     <td>~12MB/s @ 3.7GHz</td>
//!   </tr>
//!   <tr>
//!     <td>Intel Skylake</td>
//!     <td>460</td>
//!     <td>460</td>
//!     <td>460</td>
//!     <td>~72MB/s @ 4.2GHz</td>
//!   </tr>
//!   <tr>
//!     <td>Intel Haswell</td>
//!     <td>320</td>
//!     <td>320</td>
//!     <td>320</td>
//!     <td>~110MB/s @ 4.4GHz</td>
//!   </tr>
//! </table>
//!
//! [Agner’s instruction tables]: http://agner.org/optimize/
#![cfg_attr(not(feature = "std"), no_std)]

extern crate rand_core;

#[cfg(feature = "std")]
extern crate core;

pub mod changelog;

use rand_core::{RngCore, CryptoRng, Error, ErrorKind};
use core::slice;

const RETRY_LIMIT: u8 = 127;

#[cold]
#[inline(never)]
pub(crate) fn busy_loop_fail() -> ! {
    panic!("hardware generator failure");
}

/// A cryptographically secure statistically uniform, non-periodic and non-deterministic random bit
/// generator.
///
/// Note that this generator may be implemented using a deterministic algorithm that is reseeded
/// routinely from a non-deterministic entropy source to achieve the desirable properties.
///
/// This generator is a viable replacement to any generator, however, since nobody has audited
/// Intel or AMD hardware yet, the usual disclaimers as to their suitability apply.
///
/// It is potentially faster than `OsRng`, but is only supported on more recent Intel (Ivy Bridge
/// and later) and AMD (Ryzen and later) processors.
#[derive(Clone, Copy)]
pub struct RdRand(());

/// A cryptographically secure non-deterministic random bit generator.
///
/// This generator produces high-entropy output and is suited to seed other pseudo-random
/// generators.
///
/// This instruction currently is only available in Intel Broadwell (and later) and AMD Ryzen
/// processors.
///
/// This generator is not intended for general random number generation purposes and should be used
/// to seed other generators implementing [rand_core::SeedableRng].
#[derive(Clone, Copy)]
pub struct RdSeed(());

impl CryptoRng for RdRand {}
impl CryptoRng for RdSeed {}

mod arch {
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::*;
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::*;

    #[cfg(target_arch = "x86")]
    pub(crate) unsafe fn _rdrand64_step(dest: &mut u64) -> i32 {
        let mut ret1: u32 = ::core::mem::uninitialized();
        let mut ret2: u32 = ::core::mem::uninitialized();
        if _rdrand32_step(&mut ret1) != 0 && _rdrand32_step(&mut ret2) != 0 {
            *dest = (ret1 as u64) << 32 | (ret2 as u64);
            1
        } else {
            0
        }
    }

    #[cfg(target_arch = "x86")]
    pub(crate) unsafe fn _rdseed64_step(dest: &mut u64) -> i32 {
        let mut ret1: u32 = ::core::mem::uninitialized();
        let mut ret2: u32 = ::core::mem::uninitialized();
        if _rdseed32_step(&mut ret1) != 0 && _rdseed32_step(&mut ret2) != 0 {
            *dest = (ret1 as u64) << 32 | (ret2 as u64);
            1
        } else {
            0
        }
    }
}

#[cfg(not(feature = "std"))]
macro_rules! is_x86_feature_detected {
    ("rdrand") => {{
        if cfg!(target_feature="rdrand") {
            true
        } else if cfg!(target_env = "sgx") {
            false
        } else {
            const FLAG : u32 = 1 << 30;
            unsafe { ::arch::__cpuid(1).ecx & FLAG == FLAG }
        }
    }};
    ("rdseed") => {{
        if cfg!(target_feature = "rdseed") {
            true
        } else if cfg!(target_env = "sgx") {
            false
        } else {
            const FLAG : u32 = 1 << 18;
            unsafe { ::arch::__cpuid(7).ebx & FLAG == FLAG }
        }
    }};
}

macro_rules! loop_rand {
    ($el: ty, $step: path) => { {
        let mut idx = 0;
        loop {
            let mut el: $el = ::core::mem::uninitialized();
            if $step(&mut el) != 0 {
                break Some(el);
            } else if idx == RETRY_LIMIT {
                break None;
            }
            idx += 1;
        }
    } }
}

macro_rules! impl_rand {
    ($gen:ident, $feat:tt, $step16: path, $step32:path, $step64:path,
     maxstep = $maxstep:path, maxty = $maxty: ty) => {
        impl $gen {
            /// Create a new instance of the random number generator.
            ///
            /// This constructor checks whether the CPU the program is running on supports the
            /// instruction necessary for this generator to operate. If the instruction is not
            /// supported, an error is returned.
            pub fn new() -> Result<Self, Error> {
                if is_x86_feature_detected!($feat) {
                    Ok($gen(()))
                } else {
                    Err(Error::new(rand_core::ErrorKind::Unavailable,
                                   "the instruction is not supported"))
                }
            }

            /// Generate a single random `u16` value.
            ///
            /// The underlying instruction may fail for variety reasons (such as actual hardware
            /// failure or exhausted entropy), however the exact reason for the failure is not
            /// usually exposed.
            ///
            /// This method will retry calling the instruction a few times, however if all the
            /// attempts fail, it will return `None`.
            ///
            /// In case `None` is returned, the caller should assume that an non-recoverable
            /// hardware failure has occured and use another random number genrator instead.
            #[inline(always)]
            pub fn try_next_u16(&self) -> Option<u16> {
                #[target_feature(enable = $feat)]
                unsafe fn imp()
                -> Option<u16> {
                    loop_rand!(u16, $step16)
                }
                unsafe { imp() }
            }

            /// Generate a single random `u32` value.
            ///
            /// The underlying instruction may fail for variety reasons (such as actual hardware
            /// failure or exhausted entropy), however the exact reason for the failure is not
            /// usually exposed.
            ///
            /// This method will retry calling the instruction a few times, however if all the
            /// attempts fail, it will return `None`.
            ///
            /// In case `None` is returned, the caller should assume that an non-recoverable
            /// hardware failure has occured and use another random number genrator instead.
            #[inline(always)]
            pub fn try_next_u32(&self) -> Option<u32> {
                #[target_feature(enable = $feat)]
                unsafe fn imp()
                -> Option<u32> {
                    loop_rand!(u32, $step32)
                }
                unsafe { imp() }
            }

            /// Generate a single random `u64` value.
            ///
            /// The underlying instruction may fail for variety reasons (such as actual hardware
            /// failure or exhausted entropy), however the exact reason for the failure is not
            /// usually exposed.
            ///
            /// This method will retry calling the instruction a few times, however if all the
            /// attempts fail, it will return `None`.
            ///
            /// In case `None` is returned, the caller should assume that an non-recoverable
            /// hardware failure has occured and use another random number genrator instead.
            ///
            /// Note, that on 32-bit targets, there’s no underlying instruction to generate a
            /// 64-bit number, so it is emulated with the 32-bit version of the instruction.
            #[inline(always)]
            pub fn try_next_u64(&self) -> Option<u64> {
                #[target_feature(enable = $feat)]
                unsafe fn imp()
                -> Option<u64> {
                    loop_rand!(u64, $step64)
                }
                unsafe { imp() }
            }
        }

        impl RngCore for $gen {
            /// Generate a single random `u32` value.
            ///
            /// The underlying instruction may fail for variety reasons (such as actual hardware
            /// failure or exhausted entropy), however the exact reason for the failure is not
            /// usually exposed.
            ///
            /// # Panic
            ///
            /// This method will retry calling the instruction a few times, however if all the
            /// attempts fail, it will `panic`.
            ///
            /// In case `panic` occurs, the caller should assume that an non-recoverable
            /// hardware failure has occured and use another random number genrator instead.
            #[inline(always)]
            fn next_u32(&mut self) -> u32 {
                if let Some(result) = self.try_next_u32() {
                    result
                } else {
                    busy_loop_fail()
                }
            }

            /// Generate a single random `u64` value.
            ///
            /// The underlying instruction may fail for variety reasons (such as actual hardware
            /// failure or exhausted entropy), however the exact reason for the failure is not
            /// usually exposed.
            ///
            /// Note, that on 32-bit targets, there’s no underlying instruction to generate a
            /// 64-bit number, so it is emulated with the 32-bit version of the instruction.
            ///
            /// # Panic
            ///
            /// This method will retry calling the instruction a few times, however if all the
            /// attempts fail, it will `panic`.
            ///
            /// In case `panic` occurs, the caller should assume that an non-recoverable
            /// hardware failure has occured and use another random number genrator instead.
            #[inline(always)]
            fn next_u64(&mut self) -> u64 {
                if let Some(result) = self.try_next_u64() {
                    result
                } else {
                    busy_loop_fail()
                }
            }

            /// Fill a buffer `dest` with random data.
            ///
            /// See `try_fill_bytes` for a more extensive documentation.
            ///
            /// # Panic
            ///
            /// This method will panic any time `try_fill_bytes` would return an error.
            #[inline(always)]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                if let Err(_) = self.try_fill_bytes(dest) {
                    busy_loop_fail()
                }
            }

            /// Fill a buffer `dest` with random data.
            ///
            /// This method will use the most appropriate variant of the instruction available on
            /// the machine to achieve the greatest single-core throughput, however it has a
            /// slightly higher setup cost than the plain `next_u32` or `next_u64` methods.
            ///
            /// The underlying instruction may fail for variety reasons (such as actual hardware
            /// failure or exhausted entropy), however the exact reason for the failure is not
            /// usually exposed.
            ///
            /// This method will retry calling the instruction a few times, however if all the
            /// attempts fail, it will return an error.
            ///
            /// If an error is returned, the caller should assume that an non-recoverable hardware
            /// failure has occured and use another random number genrator instead.
            #[inline(always)]
            fn try_fill_bytes(&mut self, dest: &mut [u8])
            -> Result<(), Error> {
                #[target_feature(enable = $feat)]
                unsafe fn imp(dest: &mut [u8])
                -> Result<(), Error>
                {
                    unsafe fn imp_less_fast(mut dest: &mut [u8], word: &mut $maxty,
                                            buffer: &mut &[u8])
                    -> Result<(), Error>
                    {
                        while !dest.is_empty() {
                            if buffer.is_empty() {
                                if let Some(w) = loop_rand!($maxty, $maxstep) {
                                    *word = w;
                                    *buffer = slice::from_raw_parts(
                                        word as *const _ as *const u8,
                                        ::core::mem::size_of::<$maxty>()
                                    );
                                } else {
                                    return Err(Error::new(ErrorKind::Unexpected,
                                                          "hardware generator failure"));
                                }
                            }

                            let len = dest.len().min(buffer.len());
                            let (copy_src, leftover) = buffer.split_at(len);
                            let (copy_dest, dest_leftover) = { dest }.split_at_mut(len);
                            *buffer = leftover;
                            dest = dest_leftover;
                            ::core::ptr::copy_nonoverlapping(
                                copy_src.as_ptr(), copy_dest.as_mut_ptr(), len
                            );
                        }
                        Ok(())
                    }

                    let destlen = dest.len();
                    if destlen > ::core::mem::size_of::<$maxty>() {
                            let (left, mid, right) = dest.align_to_mut();
                            let mut word = 0;
                            let mut buffer: &[u8] = &[];

                            for el in mid {
                                if let Some(val) = loop_rand!($maxty, $maxstep) {
                                    *el = val;
                                } else {
                                    return Err(Error::new(ErrorKind::Unexpected,
                                                          "hardware generator failure"));
                                }
                            }

                            imp_less_fast(left, &mut word, &mut buffer)?;
                            imp_less_fast(right, &mut word, &mut buffer)
                    } else {
                        let mut word = 0;
                        let mut buffer: &[u8] = &[];
                        imp_less_fast(dest, &mut word, &mut buffer)
                    }
                }
                unsafe { imp(dest) }
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl_rand!(RdRand, "rdrand",
           ::arch::_rdrand16_step, ::arch::_rdrand32_step, ::arch::_rdrand64_step,
           maxstep = ::arch::_rdrand64_step, maxty = u64);
#[cfg(target_arch = "x86_64")]
impl_rand!(RdSeed, "rdseed",
           ::arch::_rdseed16_step, ::arch::_rdseed32_step, ::arch::_rdseed64_step,
           maxstep = ::arch::_rdseed64_step, maxty = u64);
#[cfg(target_arch = "x86")]
impl_rand!(RdRand, "rdrand",
           ::arch::_rdrand16_step, ::arch::_rdrand32_step, ::arch::_rdrand64_step,
           maxstep = ::arch::_rdrand32_step, maxty = u32);
#[cfg(target_arch = "x86")]
impl_rand!(RdSeed, "rdseed",
           ::arch::_rdseed16_step, ::arch::_rdseed32_step, ::arch::_rdseed64_step,
           maxstep = ::arch::_rdseed32_step, maxty = u32);

#[test]
fn rdrand_works() {
    let _ = RdRand::new().map(|mut r| {
        r.next_u32();
        r.next_u64();
    });
}

#[test]
fn fill_fills_all_bytes() {
    let _ = RdRand::new().map(|mut r| {
        let mut peach;
        let mut banana;
        let mut start = 0;
        let mut end = 128;
        'outer: while start < end {
            banana = [0; 128];
            for _ in 0..512 {
                peach = [0; 128];
                r.fill_bytes(&mut peach[start..end]);
                for (b, p) in banana.iter_mut().zip(peach.iter()) {
                    *b = *b | *p;
                }
                if (&banana[start..end]).iter().all(|x| *x != 0) {
                    assert!(banana[..start].iter().all(|x| *x == 0), "all other values must be 0");
                    assert!(banana[end..].iter().all(|x| *x == 0), "all other values must be 0");
                    if start < 17 {
                        start += 1;
                    } else {
                        end -= 3;
                    }
                    continue 'outer;
                }
            }
            panic!("wow, we broke it? {} {} {:?}", start, end, &banana[..])
        }
    });
}

#[test]
fn rdseed_works() {
    let _ = RdSeed::new().map(|mut r| {
        r.next_u32();
        r.next_u64();
    });
}
