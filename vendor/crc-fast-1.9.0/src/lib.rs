// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.
// Future proofing for no_std support
#![cfg_attr(not(feature = "std"), no_std)]

//! `crc-fast`
//! ===========
//!
//! Hardware-accelerated CRC calculation for
//! [all known CRC-32 and CRC-64 variants](https://reveng.sourceforge.io/crc-catalogue/all.htm)
//! using SIMD intrinsics which can exceed 100GiB/s for CRC-32 and 50GiB/s for CRC-64 on modern
//! systems.
//!
//! # Other languages
//!
//! Supplies a C-compatible shared library for use with other non-Rust languages. See
//! [PHP extension](https://github.com/awesomized/crc-fast-php-ext) example.
//!
//! # Background
//!
//! The implementation is based on Intel's
//! [Fast CRC Computation for Generic Polynomials Using PCLMULQDQ Instruction](https://web.archive.org/web/20131224125630/https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/fast-crc-computation-generic-polynomials-pclmulqdq-paper.pdf),
//! white paper though it folds 8-at-a-time, like other modern implementations, rather than the
//! 4-at-a-time as in Intel's paper.
//!
//! Works on `aarch64`, `x86_64`, and `x86` architectures, and is hardware-accelerated and optimized
//! for each architecture.
//!
//! Inspired by [`crc32fast`](https://crates.io/crates/crc32fast),
//! [`crc64fast`](https://crates.io/crates/crc64fast),
//! and [`crc64fast-nvme`](https://crates.io/crates/crc64fast-nvme), each of which only accelerates
//! a single, different CRC variant, and all of them were "reflected" variants.
//!
//! In contrast, this library accelerates _every known variant_ (and should accelerate any future
//! variants without changes), including all the "non-reflected" variants.
//!
//! # Usage
//!
//! ## Digest
//!
//! Implements the [digest::DynDigest](https://docs.rs/digest/latest/digest/trait.DynDigest.html)
//! trait for easier integration with existing code.
//!
//! ```rust
//! use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};
//!
//! let mut digest = Digest::new(Crc32IsoHdlc);
//! digest.update(b"1234");
//! digest.update(b"56789");
//! let checksum = digest.finalize();
//!
//! assert_eq!(checksum, 0xcbf43926);
//! ```
//!
//! ## Digest Write
//!
//! Implements the [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) trait for
//! easier integration with existing code.
//!
//! ```rust
//! use std::env;
//! use std::fs::File;
//! use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};
//!
//! // for example/test purposes only, use your own file path
//! let file_path = env::current_dir().expect("missing working dir").join("crc-check.txt");
//! let file_on_disk = file_path.to_str().unwrap();
//!
//! // actual usage
//! let mut digest = Digest::new(Crc32IsoHdlc);
//! let mut file = File::open(file_on_disk).unwrap();
//! std::io::copy(&mut file, &mut digest).unwrap();
//! let checksum = digest.finalize();
//!
//! assert_eq!(checksum, 0xcbf43926);
//! ```
//! ## checksum
//!```rust
//! use crc_fast::{checksum, CrcAlgorithm::Crc32IsoHdlc};
//!
//! let checksum = checksum(Crc32IsoHdlc, b"123456789");
//!
//! assert_eq!(checksum, 0xcbf43926);
//! ```
//!
//! ## checksum_combine
//!```rust
//! use crc_fast::{checksum, checksum_combine, CrcAlgorithm::Crc32IsoHdlc};
//!
//! let checksum_1 = checksum(Crc32IsoHdlc, b"1234");
//! let checksum_2 = checksum(Crc32IsoHdlc, b"56789");
//! let checksum = checksum_combine(Crc32IsoHdlc, checksum_1, checksum_2, 5);
//!
//! assert_eq!(checksum, 0xcbf43926);
//! ```
//!
//! ## checksum_file
//!```rust
//! use std::env;
//! use crc_fast::{checksum_file, CrcAlgorithm::Crc32IsoHdlc};
//!
//! // for example/test purposes only, use your own file path
//! let file_path = env::current_dir().expect("missing working dir").join("crc-check.txt");
//! let file_on_disk = file_path.to_str().unwrap();
//!
//! let checksum = checksum_file(Crc32IsoHdlc, file_on_disk, None);
//!
//! assert_eq!(checksum.unwrap(), 0xcbf43926);
//! ```
//!
//! ## Custom CRC Parameters
//!
//! For cases where you need to use CRC variants not included in the predefined algorithms,
//! you can define custom CRC parameters using `CrcParams::new()` and use the `*_with_params` functions.
//!
//! ## checksum_with_params
//!```rust
//! use crc_fast::{checksum_with_params, CrcParams};
//!
//! // Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
//! let custom_params = CrcParams::new(
//!     "CRC-32/CUSTOM",
//!     32,
//!     0x04c11db7,
//!     0xffffffff,
//!     true,
//!     0xffffffff,
//!     0xcbf43926,
//! );
//!
//! let checksum = checksum_with_params(custom_params, b"123456789");
//!
//! assert_eq!(checksum, 0xcbf43926);
//! ```
//!
//! # no_std Support
//!
//! Supports `no_std` environments. Use `default-features = false` in Cargo.toml.
//!
//! Note: When using this library in a `no_std` environment, the final binary must provide:
//! - A `#[panic_handler]` (e.g., via the `panic-halt` crate)
//! - A `#[global_allocator]` if using the `alloc` feature

// Provide a panic handler for no_std library checks
// Disabled with default-features = false (which binaries should use)
#[cfg(all(
    feature = "panic-handler",
    not(feature = "std"),
    not(test),
    not(doctest)
))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Provide a global allocator for no_std + alloc library checks
// Disabled with default-features = false (which binaries should use)
#[cfg(all(
    feature = "panic-handler",
    feature = "alloc",
    not(feature = "std"),
    not(test),
    not(doctest)
))]
#[global_allocator]
static ALLOCATOR: StubAllocator = StubAllocator;

#[cfg(all(
    feature = "panic-handler",
    feature = "alloc",
    not(feature = "std"),
    not(test),
    not(doctest)
))]
struct StubAllocator;

#[cfg(all(
    feature = "panic-handler",
    feature = "alloc",
    not(feature = "std"),
    not(test),
    not(doctest)
))]
unsafe impl core::alloc::GlobalAlloc for StubAllocator {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}

use crate::crc16::consts::{
    CRC16_ARC, CRC16_CDMA2000, CRC16_CMS, CRC16_DDS_110, CRC16_DECT_R, CRC16_DECT_X, CRC16_DNP,
    CRC16_EN_13757, CRC16_GENIBUS, CRC16_GSM, CRC16_IBM_3740, CRC16_IBM_SDLC,
    CRC16_ISO_IEC_14443_3_A, CRC16_KERMIT, CRC16_LJ1200, CRC16_M17, CRC16_MAXIM_DOW, CRC16_MCRF4XX,
    CRC16_MODBUS, CRC16_NRSC_5, CRC16_OPENSAFETY_A, CRC16_OPENSAFETY_B, CRC16_PROFIBUS,
    CRC16_RIELLO, CRC16_SPI_FUJITSU, CRC16_T10_DIF, CRC16_TELEDISK, CRC16_TMS37157, CRC16_UMTS,
    CRC16_USB, CRC16_XMODEM,
};

use crate::crc32::consts::{
    CRC32_AIXM, CRC32_AUTOSAR, CRC32_BASE91_D, CRC32_BZIP2, CRC32_CD_ROM_EDC, CRC32_CKSUM,
    CRC32_ISCSI, CRC32_ISO_HDLC, CRC32_JAMCRC, CRC32_MEF, CRC32_MPEG_2, CRC32_XFER,
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[cfg(feature = "std")]
use crate::crc32::fusion;

use crate::crc64::consts::{
    CRC64_ECMA_182, CRC64_GO_ISO, CRC64_MS, CRC64_NVME, CRC64_REDIS, CRC64_WE, CRC64_XZ,
};
use crate::structs::Calculator;
use crate::traits::CrcCalculator;
#[cfg(feature = "alloc")]
use digest::DynDigest;
#[cfg(feature = "alloc")]
use digest::InvalidBufferSize;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
use crate::feature_detection::get_arch_ops;
#[cfg(feature = "std")]
use std::fs::File;
#[cfg(feature = "std")]
use std::io::{Read, Write};

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::boxed::Box;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;

mod algorithm;
mod arch;
mod cache;
mod combine;
mod consts;
mod crc16;
mod crc32;
mod crc64;
mod enums;
mod feature_detection;
#[cfg(feature = "ffi")]
mod ffi;
mod generate;
mod structs;
mod test;
mod traits;

/// Supported CRC-16, CRC-32, and CRC-64 variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrcAlgorithm {
    /// Generic custom CRC variant that works with any supported width (16, 32, 64).
    /// The actual width is determined by the `width` field in `CrcParams`.
    CrcCustom,
    Crc16Arc,
    Crc16Cdma2000,
    Crc16Cms,
    Crc16Dds110,
    Crc16DectR,
    Crc16DectX,
    Crc16Dnp,
    Crc16En13757,
    Crc16Genibus,
    Crc16Gsm,
    Crc16Ibm3740,
    Crc16IbmSdlc,
    Crc16IsoIec144433A,
    Crc16Kermit,
    Crc16Lj1200,
    Crc16M17,
    Crc16MaximDow,
    Crc16Mcrf4xx,
    Crc16Modbus,
    Crc16Nrsc5,
    Crc16OpensafetyA,
    Crc16OpensafetyB,
    Crc16Profibus,
    Crc16Riello,
    Crc16SpiFujitsu,
    Crc16T10Dif,
    Crc16Teledisk,
    Crc16Tms37157,
    Crc16Umts,
    Crc16Usb,
    Crc16Xmodem,
    Crc32Aixm,
    Crc32Autosar,
    Crc32Base91D,
    Crc32Bzip2,
    Crc32CdRomEdc,
    Crc32Cksum,
    #[deprecated(
        since = "1.9.0",
        note = "Use CrcCustom instead, which works with any supported width (16, 32, 64)"
    )]
    Crc32Custom, // Custom CRC-32 implementation, not defined in consts
    Crc32Iscsi,
    Crc32IsoHdlc,
    Crc32Jamcrc,
    Crc32Mef,
    Crc32Mpeg2,
    Crc32Xfer,
    #[deprecated(
        since = "1.9.0",
        note = "Use CrcCustom instead, which works with any supported width (16, 32, 64)"
    )]
    Crc64Custom, // Custom CRC-64 implementation, not defined in consts
    Crc64Ecma182,
    Crc64GoIso,
    Crc64Ms,
    Crc64Nvme,
    Crc64Redis,
    Crc64We,
    Crc64Xz,
}

/// Internal storage for CRC folding keys that can accommodate different array sizes.
/// This enum allows future expansion to support larger folding distances while maintaining
/// backwards compatibility with existing const definitions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CrcKeysStorage {
    /// Current 23-key format for existing algorithms (supports up to 256-byte folding distances)
    KeysFold256([u64; 23]),
    /// Future 25-key format for potential expanded folding distances (testing purposes only)
    KeysFutureTest([u64; 25]),
}

impl CrcKeysStorage {
    /// Safe key access with bounds checking. Returns 0 for out-of-bounds indices.
    #[inline(always)]
    const fn get_key(self, index: usize) -> u64 {
        match self {
            CrcKeysStorage::KeysFold256(keys) => {
                if index < 23 {
                    keys[index]
                } else {
                    0
                }
            }
            CrcKeysStorage::KeysFutureTest(keys) => {
                if index < 25 {
                    keys[index]
                } else {
                    0
                }
            }
        }
    }

    /// Returns the number of keys available in this storage variant.
    #[inline(always)]
    const fn key_count(self) -> usize {
        match self {
            CrcKeysStorage::KeysFold256(_) => 23,
            CrcKeysStorage::KeysFutureTest(_) => 25,
        }
    }

    /// Const constructor for 23-key arrays (current format).
    #[inline(always)]
    const fn from_keys_fold_256(keys: [u64; 23]) -> Self {
        CrcKeysStorage::KeysFold256(keys)
    }

    /// Const constructor for 25-key arrays (future expansion testing).
    #[inline(always)]
    #[allow(dead_code)] // Reserved for future expansion
    const fn from_keys_fold_future_test(keys: [u64; 25]) -> Self {
        CrcKeysStorage::KeysFutureTest(keys)
    }

    /// Extracts keys as a [u64; 23] array for FFI compatibility.
    /// For variants with more than 23 keys, only the first 23 are returned.
    /// For variants with fewer keys, remaining slots are filled with 0.
    #[inline(always)]
    pub fn to_keys_array_23(self) -> [u64; 23] {
        match self {
            CrcKeysStorage::KeysFold256(keys) => keys,
            CrcKeysStorage::KeysFutureTest(keys) => {
                let mut result = [0u64; 23];
                result.copy_from_slice(&keys[..23]);
                result
            }
        }
    }
}

// Implement PartialEq between CrcKeysStorage and [u64; 23] for test compatibility
impl PartialEq<[u64; 23]> for CrcKeysStorage {
    fn eq(&self, other: &[u64; 23]) -> bool {
        self.to_keys_array_23() == *other
    }
}

impl PartialEq<CrcKeysStorage> for [u64; 23] {
    fn eq(&self, other: &CrcKeysStorage) -> bool {
        *self == other.to_keys_array_23()
    }
}

/// Parameters for CRC computation, including polynomial, initial value, and other settings.
#[derive(Clone, Copy, Debug)]
pub struct CrcParams {
    pub algorithm: CrcAlgorithm,
    pub name: &'static str,
    pub width: u8,
    pub poly: u64,
    pub init: u64,
    /// The init value in "algorithm form" for the SIMD implementation.
    ///
    /// For most CRC variants, this equals `init`. However, for reflected CRC-16 variants
    /// with non-symmetric init values (e.g., CRC-16/ISO-IEC-14443-3-A with init=0xC6C6),
    /// this stores the bit-reversed init value. This avoids runtime bit-reversal on every
    /// update() call, which would otherwise be needed because the SIMD algorithm operates
    /// on data in a different bit order than the catalog specification.
    ///
    /// Examples:
    /// - CRC-16/IBM-SDLC: init=0xFFFF, init_algorithm=0xFFFF (symmetric)
    /// - CRC-16/ISO-IEC-14443-3-A: init=0xC6C6, init_algorithm=0x6363 (0xC6C6.reverse_bits())
    pub init_algorithm: u64,
    pub refin: bool,
    pub refout: bool,
    pub xorout: u64,
    pub check: u64,
    pub keys: CrcKeysStorage,
}

/// Type alias for a function pointer that represents a CRC calculation function.
///
/// The function takes the following parameters:
/// - `state`: The current state of the CRC computation.
/// - `data`: A slice of bytes to be processed.
/// - `params`: The parameters for the CRC computation, such as polynomial, initial value, etc.
///
/// The function returns the updated state after processing the data.
///
/// Note: CrcParams is passed by reference to avoid copying the large struct (200+ bytes)
/// which causes significant overhead for small data sizes.
type CalculatorFn = fn(
    u64,        // state
    &[u8],      // data
    &CrcParams, // CRC implementation parameters
) -> u64;

/// Represents a CRC Digest, which is used to compute CRC checksums.
///
/// The `Digest` struct maintains the state of the CRC computation, including
/// the current state, the amount of data processed, the CRC parameters, and
/// the calculator function used to perform the CRC calculation.
#[derive(Copy, Clone, Debug)]
pub struct Digest {
    /// The current state of the CRC computation.
    state: u64,

    /// The total amount of data processed so far.
    amount: u64,

    /// The parameters for the CRC computation, such as polynomial, initial value, etc.
    params: CrcParams,

    /// The function used to perform the CRC calculation.
    calculator: CalculatorFn,
}

#[cfg(feature = "alloc")]
impl DynDigest for Digest {
    #[inline(always)]
    fn update(&mut self, data: &[u8]) {
        self.update(data);
    }

    #[inline(always)]
    fn finalize_into(self, buf: &mut [u8]) -> Result<(), InvalidBufferSize> {
        if buf.len() != self.output_size() {
            return Err(InvalidBufferSize);
        }

        let result = self.finalize();
        let be_bytes = result.to_be_bytes();
        let start = 8 - self.output_size();
        buf.copy_from_slice(&be_bytes[start..]);

        Ok(())
    }

    #[inline(always)]
    fn finalize_into_reset(&mut self, out: &mut [u8]) -> Result<(), InvalidBufferSize> {
        if out.len() != self.output_size() {
            return Err(InvalidBufferSize);
        }
        let result = self.finalize();
        self.reset();
        let be_bytes = result.to_be_bytes();
        let start = 8 - self.output_size();
        out.copy_from_slice(&be_bytes[start..]);
        Ok(())
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.reset();
    }

    #[inline(always)]
    fn output_size(&self) -> usize {
        self.params.width as usize / 8
    }

    fn box_clone(&self) -> Box<dyn DynDigest> {
        Box::new(*self)
    }
}

impl Digest {
    /// Creates a new `Digest` instance for the specified CRC algorithm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};
    ///
    /// let mut digest = Digest::new(Crc32IsoHdlc);
    /// digest.update(b"123456789");
    /// let checksum = digest.finalize();
    ///
    /// assert_eq!(checksum, 0xcbf43926);
    /// ```
    #[inline(always)]
    pub fn new(algorithm: CrcAlgorithm) -> Self {
        let (calculator, params) = get_calculator_params(algorithm);

        Self {
            state: params.init_algorithm,
            amount: 0,
            params,
            calculator,
        }
    }

    /// Creates a new `Digest` instance for the specified CRC algorithm with a custom initial state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};
    ///
    /// // CRC-32/ISO-HDLC with initial state of 0x00000000, instead of the default initial state
    /// // of 0xffffffff,
    /// let mut digest = Digest::new_with_init_state(Crc32IsoHdlc, 0x00000000);
    /// digest.update(b"123456789");
    /// let checksum = digest.finalize();
    ///
    /// // different initial state, so checksum will be different
    /// assert_eq!(checksum, 0xd202d277);
    ///
    /// let mut digest = Digest::new_with_init_state(Crc32IsoHdlc, 0xffffffff);
    /// digest.update(b"123456789");
    /// let checksum = digest.finalize();
    ///
    /// // same initial state as the default, so checksum will be the same
    /// assert_eq!(checksum, 0xcbf43926);
    /// ```
    #[inline(always)]
    pub fn new_with_init_state(algorithm: CrcAlgorithm, init_state: u64) -> Self {
        let (calculator, params) = get_calculator_params(algorithm);

        Self {
            state: init_state,
            amount: 0,
            params,
            calculator,
        }
    }

    /// Creates a new `Digest` instance with custom CRC parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crc_fast::{Digest, CrcParams};
    ///
    /// // Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
    /// let custom_params = CrcParams::new(
    ///     "CRC-32/CUSTOM",
    ///     32,
    ///     0x04c11db7,
    ///     0xffffffff,
    ///     true,
    ///     0xffffffff,
    ///     0xcbf43926,
    /// );
    ///
    /// let mut digest = Digest::new_with_params(custom_params);
    /// digest.update(b"123456789");
    /// let checksum = digest.finalize();
    ///
    /// assert_eq!(checksum, 0xcbf43926);
    /// ```
    #[inline(always)]
    pub fn new_with_params(params: CrcParams) -> Self {
        let calculator = Calculator::calculate as CalculatorFn;

        Self {
            state: params.init_algorithm,
            amount: 0,
            params,
            calculator,
        }
    }

    /// Updates the CRC state with the given data.
    #[inline(always)]
    pub fn update(&mut self, data: &[u8]) {
        self.state = (self.calculator)(self.state, data, &self.params);
        self.amount += data.len() as u64;
    }

    /// Finalizes the CRC computation and returns the result.
    #[inline(always)]
    pub fn finalize(&self) -> u64 {
        self.state ^ self.params.xorout
    }

    /// Finalizes the CRC computation, resets the state, and returns the result.
    #[inline(always)]
    pub fn finalize_reset(&mut self) -> u64 {
        let result = self.finalize();
        self.reset();

        result
    }

    /// Resets the CRC state to its initial value.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.state = self.params.init_algorithm;
        self.amount = 0;
    }

    /// Combines the CRC state with a second `Digest` instance.
    #[inline(always)]
    pub fn combine(&mut self, other: &Self) {
        self.amount += other.amount;
        let other_crc = other.finalize();

        // note the xorout for the input, since it's already been applied so it has to be removed,
        // and then re-adding it on the final output
        self.state = combine::checksums(
            self.state ^ self.params.xorout,
            other_crc,
            other.amount,
            &self.params,
        ) ^ self.params.xorout;
    }

    /// Gets the amount of data processed so far
    #[inline(always)]
    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    /// Gets the current CRC state.
    ///
    /// # Examples
    /// ```rust
    /// use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};
    ///
    /// let mut digest = Digest::new(Crc32IsoHdlc);
    /// digest.update(b"123456789");
    /// let state = digest.get_state();
    ///
    /// // non-finalized state, so it won't match the final checksum
    /// assert_eq!(state, 0x340bc6d9);
    ///
    /// // finalized state will match the checksum
    /// assert_eq!(digest.finalize(), 0xcbf43926);
    /// ```
    #[inline(always)]
    pub fn get_state(&self) -> u64 {
        self.state
    }
}

#[cfg(feature = "std")]
impl Write for Digest {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.update(buf);
        Ok(buf.len())
    }

    #[inline(always)]
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        let len: usize = bufs
            .iter()
            .map(|buf| {
                self.update(buf);
                buf.len()
            })
            .sum();

        Ok(len)
    }

    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    #[inline(always)]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.update(buf);

        Ok(())
    }
}

/// Computes the CRC checksum for the given data using the specified algorithm.
///
///```rust
/// use crc_fast::{checksum, CrcAlgorithm::Crc32IsoHdlc};
/// let checksum = checksum(Crc32IsoHdlc, b"123456789");
///
/// assert_eq!(checksum, 0xcbf43926);
/// ```
#[inline]
#[allow(deprecated)]
pub fn checksum(algorithm: CrcAlgorithm, buf: &[u8]) -> u64 {
    // avoid using get_calculator_params() here to reduce overhead for small data sizes
    match algorithm {
        CrcAlgorithm::Crc16Arc => {
            Calculator::calculate(CRC16_ARC.init, buf, &CRC16_ARC) ^ CRC16_ARC.xorout
        }
        CrcAlgorithm::Crc16Cdma2000 => {
            Calculator::calculate(CRC16_CDMA2000.init, buf, &CRC16_CDMA2000) ^ CRC16_CDMA2000.xorout
        }
        CrcAlgorithm::Crc16Cms => {
            Calculator::calculate(CRC16_CMS.init, buf, &CRC16_CMS) ^ CRC16_CMS.xorout
        }
        CrcAlgorithm::Crc16Dds110 => {
            Calculator::calculate(CRC16_DDS_110.init, buf, &CRC16_DDS_110) ^ CRC16_DDS_110.xorout
        }
        CrcAlgorithm::Crc16DectR => {
            Calculator::calculate(CRC16_DECT_R.init, buf, &CRC16_DECT_R) ^ CRC16_DECT_R.xorout
        }
        CrcAlgorithm::Crc16DectX => {
            Calculator::calculate(CRC16_DECT_X.init, buf, &CRC16_DECT_X) ^ CRC16_DECT_X.xorout
        }
        CrcAlgorithm::Crc16Dnp => {
            Calculator::calculate(CRC16_DNP.init, buf, &CRC16_DNP) ^ CRC16_DNP.xorout
        }
        CrcAlgorithm::Crc16En13757 => {
            Calculator::calculate(CRC16_EN_13757.init, buf, &CRC16_EN_13757) ^ CRC16_EN_13757.xorout
        }
        CrcAlgorithm::Crc16Genibus => {
            Calculator::calculate(CRC16_GENIBUS.init, buf, &CRC16_GENIBUS) ^ CRC16_GENIBUS.xorout
        }
        CrcAlgorithm::Crc16Gsm => {
            Calculator::calculate(CRC16_GSM.init, buf, &CRC16_GSM) ^ CRC16_GSM.xorout
        }
        CrcAlgorithm::Crc16Ibm3740 => {
            Calculator::calculate(CRC16_IBM_3740.init, buf, &CRC16_IBM_3740) ^ CRC16_IBM_3740.xorout
        }
        CrcAlgorithm::Crc16IbmSdlc => {
            Calculator::calculate(CRC16_IBM_SDLC.init, buf, &CRC16_IBM_SDLC) ^ CRC16_IBM_SDLC.xorout
        }
        CrcAlgorithm::Crc16IsoIec144433A => {
            Calculator::calculate(
                CRC16_ISO_IEC_14443_3_A.init_algorithm,
                buf,
                &CRC16_ISO_IEC_14443_3_A,
            ) ^ CRC16_ISO_IEC_14443_3_A.xorout
        }
        CrcAlgorithm::Crc16Kermit => {
            Calculator::calculate(CRC16_KERMIT.init, buf, &CRC16_KERMIT) ^ CRC16_KERMIT.xorout
        }
        CrcAlgorithm::Crc16Lj1200 => {
            Calculator::calculate(CRC16_LJ1200.init, buf, &CRC16_LJ1200) ^ CRC16_LJ1200.xorout
        }
        CrcAlgorithm::Crc16M17 => {
            Calculator::calculate(CRC16_M17.init, buf, &CRC16_M17) ^ CRC16_M17.xorout
        }
        CrcAlgorithm::Crc16MaximDow => {
            Calculator::calculate(CRC16_MAXIM_DOW.init, buf, &CRC16_MAXIM_DOW)
                ^ CRC16_MAXIM_DOW.xorout
        }
        CrcAlgorithm::Crc16Mcrf4xx => {
            Calculator::calculate(CRC16_MCRF4XX.init, buf, &CRC16_MCRF4XX) ^ CRC16_MCRF4XX.xorout
        }
        CrcAlgorithm::Crc16Modbus => {
            Calculator::calculate(CRC16_MODBUS.init, buf, &CRC16_MODBUS) ^ CRC16_MODBUS.xorout
        }
        CrcAlgorithm::Crc16Nrsc5 => {
            Calculator::calculate(CRC16_NRSC_5.init, buf, &CRC16_NRSC_5) ^ CRC16_NRSC_5.xorout
        }
        CrcAlgorithm::Crc16OpensafetyA => {
            Calculator::calculate(CRC16_OPENSAFETY_A.init, buf, &CRC16_OPENSAFETY_A)
                ^ CRC16_OPENSAFETY_A.xorout
        }
        CrcAlgorithm::Crc16OpensafetyB => {
            Calculator::calculate(CRC16_OPENSAFETY_B.init, buf, &CRC16_OPENSAFETY_B)
                ^ CRC16_OPENSAFETY_B.xorout
        }
        CrcAlgorithm::Crc16Profibus => {
            Calculator::calculate(CRC16_PROFIBUS.init, buf, &CRC16_PROFIBUS) ^ CRC16_PROFIBUS.xorout
        }
        CrcAlgorithm::Crc16Riello => {
            Calculator::calculate(CRC16_RIELLO.init_algorithm, buf, &CRC16_RIELLO)
                ^ CRC16_RIELLO.xorout
        }
        CrcAlgorithm::Crc16SpiFujitsu => {
            Calculator::calculate(CRC16_SPI_FUJITSU.init, buf, &CRC16_SPI_FUJITSU)
                ^ CRC16_SPI_FUJITSU.xorout
        }
        CrcAlgorithm::Crc16T10Dif => {
            Calculator::calculate(CRC16_T10_DIF.init, buf, &CRC16_T10_DIF) ^ CRC16_T10_DIF.xorout
        }
        CrcAlgorithm::Crc16Teledisk => {
            Calculator::calculate(CRC16_TELEDISK.init, buf, &CRC16_TELEDISK) ^ CRC16_TELEDISK.xorout
        }
        CrcAlgorithm::Crc16Tms37157 => {
            Calculator::calculate(CRC16_TMS37157.init_algorithm, buf, &CRC16_TMS37157)
                ^ CRC16_TMS37157.xorout
        }
        CrcAlgorithm::Crc16Umts => {
            Calculator::calculate(CRC16_UMTS.init, buf, &CRC16_UMTS) ^ CRC16_UMTS.xorout
        }
        CrcAlgorithm::Crc16Usb => {
            Calculator::calculate(CRC16_USB.init, buf, &CRC16_USB) ^ CRC16_USB.xorout
        }
        CrcAlgorithm::Crc16Xmodem => {
            Calculator::calculate(CRC16_XMODEM.init, buf, &CRC16_XMODEM) ^ CRC16_XMODEM.xorout
        }
        CrcAlgorithm::Crc32Aixm => {
            Calculator::calculate(CRC32_AIXM.init, buf, &CRC32_AIXM) ^ CRC32_AIXM.xorout
        }
        CrcAlgorithm::Crc32Autosar => {
            Calculator::calculate(CRC32_AUTOSAR.init, buf, &CRC32_AUTOSAR) ^ CRC32_AUTOSAR.xorout
        }
        CrcAlgorithm::Crc32Base91D => {
            Calculator::calculate(CRC32_BASE91_D.init, buf, &CRC32_BASE91_D) ^ CRC32_BASE91_D.xorout
        }
        CrcAlgorithm::Crc32Bzip2 => {
            Calculator::calculate(CRC32_BZIP2.init, buf, &CRC32_BZIP2) ^ CRC32_BZIP2.xorout
        }
        CrcAlgorithm::Crc32CdRomEdc => {
            Calculator::calculate(CRC32_CD_ROM_EDC.init, buf, &CRC32_CD_ROM_EDC)
                ^ CRC32_CD_ROM_EDC.xorout
        }
        CrcAlgorithm::Crc32Cksum => {
            Calculator::calculate(CRC32_CKSUM.init, buf, &CRC32_CKSUM) ^ CRC32_CKSUM.xorout
        }
        CrcAlgorithm::Crc32Custom => {
            panic!("Custom CRC-32 requires parameters via CrcParams::new()")
        }
        CrcAlgorithm::Crc32Iscsi => {
            crc32_iscsi_calculator(CRC32_ISCSI.init, buf, &CRC32_ISCSI) ^ CRC32_ISCSI.xorout
        }
        CrcAlgorithm::Crc32IsoHdlc => {
            crc32_iso_hdlc_calculator(CRC32_ISO_HDLC.init, buf, &CRC32_ISO_HDLC)
                ^ CRC32_ISO_HDLC.xorout
        }
        CrcAlgorithm::Crc32Jamcrc => {
            Calculator::calculate(CRC32_JAMCRC.init, buf, &CRC32_JAMCRC) ^ CRC32_JAMCRC.xorout
        }
        CrcAlgorithm::Crc32Mef => {
            Calculator::calculate(CRC32_MEF.init, buf, &CRC32_MEF) ^ CRC32_MEF.xorout
        }
        CrcAlgorithm::Crc32Mpeg2 => {
            Calculator::calculate(CRC32_MPEG_2.init, buf, &CRC32_MPEG_2) ^ CRC32_MPEG_2.xorout
        }
        CrcAlgorithm::Crc32Xfer => {
            Calculator::calculate(CRC32_XFER.init, buf, &CRC32_XFER) ^ CRC32_XFER.xorout
        }
        CrcAlgorithm::CrcCustom => {
            panic!("Custom CRC requires parameters via CrcParams::new()")
        }
        CrcAlgorithm::Crc64Custom => {
            panic!("Custom CRC-64 requires parameters via CrcParams::new()")
        }
        CrcAlgorithm::Crc64Ecma182 => {
            Calculator::calculate(CRC64_ECMA_182.init, buf, &CRC64_ECMA_182) ^ CRC64_ECMA_182.xorout
        }
        CrcAlgorithm::Crc64GoIso => {
            Calculator::calculate(CRC64_GO_ISO.init, buf, &CRC64_GO_ISO) ^ CRC64_GO_ISO.xorout
        }
        CrcAlgorithm::Crc64Ms => {
            Calculator::calculate(CRC64_MS.init, buf, &CRC64_MS) ^ CRC64_MS.xorout
        }
        CrcAlgorithm::Crc64Nvme => {
            Calculator::calculate(CRC64_NVME.init, buf, &CRC64_NVME) ^ CRC64_NVME.xorout
        }
        CrcAlgorithm::Crc64Redis => {
            Calculator::calculate(CRC64_REDIS.init, buf, &CRC64_REDIS) ^ CRC64_REDIS.xorout
        }
        CrcAlgorithm::Crc64We => {
            Calculator::calculate(CRC64_WE.init, buf, &CRC64_WE) ^ CRC64_WE.xorout
        }
        CrcAlgorithm::Crc64Xz => {
            Calculator::calculate(CRC64_XZ.init, buf, &CRC64_XZ) ^ CRC64_XZ.xorout
        }
    }
}

/// Computes the CRC checksum for the given data using custom CRC parameters.
///
/// # Examples
///
/// ```rust
/// use crc_fast::{checksum_with_params, CrcParams};
///
/// // Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
/// let custom_params = CrcParams::new(
///     "CRC-32/CUSTOM",
///     32,
///     0x04c11db7,
///     0xffffffff,
///     true,
///     0xffffffff,
///     0xcbf43926,
/// );
///
/// let checksum = checksum_with_params(custom_params, b"123456789");
///
/// assert_eq!(checksum, 0xcbf43926);
/// ```
pub fn checksum_with_params(params: CrcParams, buf: &[u8]) -> u64 {
    let calculator = Calculator::calculate as CalculatorFn;

    calculator(params.init, buf, &params) ^ params.xorout
}

/// Computes the CRC checksum for the given file using the specified algorithm.
///
/// Appears to be much faster (~2X) than using Writer and io::*, at least on Apple M2 Ultra
///
/// # Errors
///
/// This function will return an error if the file cannot be read.
///
/// # Examples
/// ### checksum_file
///```rust
/// use std::env;
/// use crc_fast::{checksum_file, CrcAlgorithm::Crc32IsoHdlc};
///
/// // for example/test purposes only, use your own file path
/// let file_path = env::current_dir().expect("missing working dir").join("crc-check.txt");
/// let file_on_disk = file_path.to_str().unwrap();
///
/// let checksum = checksum_file(Crc32IsoHdlc, file_on_disk, None);
///
/// assert_eq!(checksum.unwrap(), 0xcbf43926);
/// ```
#[cfg(feature = "std")]
#[inline(always)]
pub fn checksum_file(
    algorithm: CrcAlgorithm,
    path: &str,
    chunk_size: Option<usize>,
) -> Result<u64, std::io::Error> {
    checksum_file_with_digest(Digest::new(algorithm), path, chunk_size)
}

/// Computes the CRC checksum for the given file using custom CRC parameters.
///
/// Appears to be much faster (~2X) than using Writer and io::*, at least on Apple M2 Ultra
///
/// # Errors
///
/// This function will return an error if the file cannot be read.
///
/// # Examples
///
/// ```rust
/// use std::env;
/// use crc_fast::{checksum_file_with_params, CrcParams};
///
/// // for example/test purposes only, use your own file path
/// let file_path = env::current_dir().expect("missing working dir").join("crc-check.txt");
/// let file_on_disk = file_path.to_str().unwrap();
///
/// // Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
/// let custom_params = CrcParams::new(
///     "CRC-32/CUSTOM",
///     32,
///     0x04c11db7,
///     0xffffffff,
///     true,
///     0xffffffff,
///     0xcbf43926,
/// );
///
/// let checksum = checksum_file_with_params(custom_params, file_on_disk, None);
///
/// assert_eq!(checksum.unwrap(), 0xcbf43926);
/// ```
#[cfg(feature = "std")]
pub fn checksum_file_with_params(
    params: CrcParams,
    path: &str,
    chunk_size: Option<usize>,
) -> Result<u64, std::io::Error> {
    checksum_file_with_digest(Digest::new_with_params(params), path, chunk_size)
}

/// Computes the CRC checksum for the given file using the specified Digest.
///
/// # Errors
///
/// This function will return an error if the file cannot be read.
#[cfg(feature = "std")]
fn checksum_file_with_digest(
    mut digest: Digest,
    path: &str,
    chunk_size: Option<usize>,
) -> Result<u64, std::io::Error> {
    let mut file = File::open(path)?;

    // 512KiB KiB was fastest in my benchmarks on an Apple M2 Ultra
    //
    // 4KiB ~7GiB/s
    // 64KiB ~22 GiB/s
    // 512KiB ~24 GiB/s
    let chunk_size = chunk_size.unwrap_or(524288);

    let mut buf = vec![0; chunk_size];

    while let Ok(n) = file.read(&mut buf) {
        if n == 0 {
            break;
        }
        digest.update(&buf[..n]);
    }

    Ok(digest.finalize())
}

/// Combines two CRC checksums using the specified algorithm.
///
/// # Examples
///```rust
/// use crc_fast::{checksum, checksum_combine, CrcAlgorithm::Crc32IsoHdlc};
///
/// let checksum_1 = checksum(Crc32IsoHdlc, b"1234");
/// let checksum_2 = checksum(Crc32IsoHdlc, b"56789");
/// let checksum = checksum_combine(Crc32IsoHdlc, checksum_1, checksum_2, 5);
///
/// assert_eq!(checksum, 0xcbf43926);
/// ```
#[inline(always)]
pub fn checksum_combine(
    algorithm: CrcAlgorithm,
    checksum1: u64,
    checksum2: u64,
    checksum2_len: u64,
) -> u64 {
    let params = get_calculator_params(algorithm).1;

    combine::checksums(checksum1, checksum2, checksum2_len, &params)
}

/// Combines two CRC checksums using custom CRC parameters.
///
/// # Examples
///
/// ```rust
/// use crc_fast::{checksum_with_params, checksum_combine_with_params, CrcParams};
///
/// // Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
/// let custom_params = CrcParams::new(
///     "CRC-32/CUSTOM",
///     32,
///     0x04c11db7,
///     0xffffffff,
///     true,
///     0xffffffff,
///     0xcbf43926,
/// );
///
/// let checksum_1 = checksum_with_params(custom_params, b"1234");
/// let checksum_2 = checksum_with_params(custom_params, b"56789");
/// let checksum = checksum_combine_with_params(custom_params, checksum_1, checksum_2, 5);
///
/// assert_eq!(checksum, 0xcbf43926);
/// ```
pub fn checksum_combine_with_params(
    params: CrcParams,
    checksum1: u64,
    checksum2: u64,
    checksum2_len: u64,
) -> u64 {
    combine::checksums(checksum1, checksum2, checksum2_len, &params)
}

/// Returns the target used to calculate the CRC checksum for the specified algorithm.
///
/// This function provides visibility into the active performance tier being used for CRC calculations.
/// The target string follows the format `{architecture}-{intrinsics-family}-{intrinsics-features}`,
/// such as `aarch64-aes-sha3` or `x86_64-avx512-vpclmulqdq`.
///
/// The performance tier system provides graceful degradation across different hardware capabilities:
/// - **AArch64**: `aarch64-aes-sha3` (highest) → `aarch64-aes-pmull` (baseline)
/// - **x86_64**: `x86_64-avx512-vpclmulqdq` (highest) → `x86_64-avx512-pclmulqdq` (mid) → `x86_64-sse-pclmulqdq` (baseline)
/// - **x86**: `x86-sse-pclmulqdq` (baseline) → `software-fallback-tables` (fallback)
/// - **Other architectures**: `software-fallback-tables`
///
/// The tier selection is deterministic and consistent across runs on the same hardware,
/// combining compile-time and runtime feature detection for safety and optimal performance.
///
/// These strings are informational only, not stable, and shouldn't be relied on to match across
/// versions.
///
/// # Examples
///```rust
/// use crc_fast::{get_calculator_target, CrcAlgorithm::Crc32IsoHdlc};
///
/// let target = get_calculator_target(Crc32IsoHdlc);
/// println!("Using performance tier: {}", target);
/// // Example outputs:
/// // "aarch64-aes-sha3" - AArch64 with SHA3 and AES support
/// // "x86_64-avx512-vpclmulqdq" - x86_64 with VPCLMULQDQ support
/// // "x86_64-sse-pclmulqdq" - x86_64 baseline with SSE4.1 and PCLMULQDQ
/// ```
#[cfg(all(
    feature = "alloc",
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
pub fn get_calculator_target(_algorithm: CrcAlgorithm) -> String {
    let arch_ops = get_arch_ops();
    arch_ops.get_target_string()
}

/// Calculates the CRC-32/ISCSI checksum (commonly called "crc32c" in many, but not all,
/// implementations).
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iscsi
///
/// Nano-optimized to be faster than calling checksum() for this specific algorithm, since it avoids
/// the match statement overhead.
///
/// # Examples
///
/// ```rust
/// use crc_fast::crc32_iscsi;
/// let checksum = crc32_iscsi(b"123456789");
/// assert_eq!(checksum, 0xe3069283);
/// ```
#[inline(always)]
pub fn crc32_iscsi(data: &[u8]) -> u32 {
    crc32_iscsi_calculator(CRC32_ISCSI.init, data, &CRC32_ISCSI) as u32 ^ CRC32_ISCSI.xorout as u32
}

/// Calculates the CRC-32/ISO-HDLC checksum (commonly called "crc32" in many, but not all,
/// implementations).
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc
///
/// Nano-optimized to be faster than calling checksum() for this specific algorithm, since it avoids
/// the match statement overhead.
///
/// # Examples
///
/// ```rust
/// use crc_fast::crc32_iso_hdlc;
/// let checksum = crc32_iso_hdlc(b"123456789");
/// assert_eq!(checksum, 0xcbf43926);
/// ```
#[inline(always)]
pub fn crc32_iso_hdlc(data: &[u8]) -> u32 {
    crc32_iso_hdlc_calculator(CRC32_ISO_HDLC.init, data, &CRC32_ISO_HDLC) as u32
        ^ CRC32_ISO_HDLC.xorout as u32
}

/// Calculates the CRC-64/NVME checksum.
///
/// https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme
///
/// Nano-optimized to be faster than calling checksum() for this specific algorithm, since it avoids
/// the match statement overhead.
///
/// # Examples
///
/// ```rust
/// use crc_fast::crc64_nvme;
/// let checksum = crc64_nvme(b"123456789");
/// assert_eq!(checksum, 0xae8b14860a799888);
/// ```
#[inline(always)]
pub fn crc64_nvme(data: &[u8]) -> u64 {
    Calculator::calculate(CRC64_NVME.init, data, &CRC64_NVME) ^ CRC64_NVME.xorout
}

/// Fallback version of get_calculator_target for unsupported architectures
#[cfg(all(
    feature = "alloc",
    not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))
))]
pub fn get_calculator_target(_algorithm: CrcAlgorithm) -> String {
    extern crate alloc;
    use alloc::string::ToString;
    "software-fallback-tables".to_string()
}

/// Returns the calculator function and parameters for the specified CRC algorithm.
#[inline(always)]
#[allow(deprecated)]
fn get_calculator_params(algorithm: CrcAlgorithm) -> (CalculatorFn, CrcParams) {
    match algorithm {
        CrcAlgorithm::Crc16Arc => (Calculator::calculate as CalculatorFn, CRC16_ARC),
        CrcAlgorithm::Crc16Cdma2000 => (Calculator::calculate as CalculatorFn, CRC16_CDMA2000),
        CrcAlgorithm::Crc16Cms => (Calculator::calculate as CalculatorFn, CRC16_CMS),
        CrcAlgorithm::Crc16Dds110 => (Calculator::calculate as CalculatorFn, CRC16_DDS_110),
        CrcAlgorithm::Crc16DectR => (Calculator::calculate as CalculatorFn, CRC16_DECT_R),
        CrcAlgorithm::Crc16DectX => (Calculator::calculate as CalculatorFn, CRC16_DECT_X),
        CrcAlgorithm::Crc16Dnp => (Calculator::calculate as CalculatorFn, CRC16_DNP),
        CrcAlgorithm::Crc16En13757 => (Calculator::calculate as CalculatorFn, CRC16_EN_13757),
        CrcAlgorithm::Crc16Genibus => (Calculator::calculate as CalculatorFn, CRC16_GENIBUS),
        CrcAlgorithm::Crc16Gsm => (Calculator::calculate as CalculatorFn, CRC16_GSM),
        CrcAlgorithm::Crc16Ibm3740 => (Calculator::calculate as CalculatorFn, CRC16_IBM_3740),
        CrcAlgorithm::Crc16IbmSdlc => (Calculator::calculate as CalculatorFn, CRC16_IBM_SDLC),
        CrcAlgorithm::Crc16IsoIec144433A => (
            Calculator::calculate as CalculatorFn,
            CRC16_ISO_IEC_14443_3_A,
        ),
        CrcAlgorithm::Crc16Kermit => (Calculator::calculate as CalculatorFn, CRC16_KERMIT),
        CrcAlgorithm::Crc16Lj1200 => (Calculator::calculate as CalculatorFn, CRC16_LJ1200),
        CrcAlgorithm::Crc16M17 => (Calculator::calculate as CalculatorFn, CRC16_M17),
        CrcAlgorithm::Crc16MaximDow => (Calculator::calculate as CalculatorFn, CRC16_MAXIM_DOW),
        CrcAlgorithm::Crc16Mcrf4xx => (Calculator::calculate as CalculatorFn, CRC16_MCRF4XX),
        CrcAlgorithm::Crc16Modbus => (Calculator::calculate as CalculatorFn, CRC16_MODBUS),
        CrcAlgorithm::Crc16Nrsc5 => (Calculator::calculate as CalculatorFn, CRC16_NRSC_5),
        CrcAlgorithm::Crc16OpensafetyA => {
            (Calculator::calculate as CalculatorFn, CRC16_OPENSAFETY_A)
        }
        CrcAlgorithm::Crc16OpensafetyB => {
            (Calculator::calculate as CalculatorFn, CRC16_OPENSAFETY_B)
        }
        CrcAlgorithm::Crc16Profibus => (Calculator::calculate as CalculatorFn, CRC16_PROFIBUS),
        CrcAlgorithm::Crc16Riello => (Calculator::calculate as CalculatorFn, CRC16_RIELLO),
        CrcAlgorithm::Crc16SpiFujitsu => (Calculator::calculate as CalculatorFn, CRC16_SPI_FUJITSU),
        CrcAlgorithm::Crc16T10Dif => (Calculator::calculate as CalculatorFn, CRC16_T10_DIF),
        CrcAlgorithm::Crc16Teledisk => (Calculator::calculate as CalculatorFn, CRC16_TELEDISK),
        CrcAlgorithm::Crc16Tms37157 => (Calculator::calculate as CalculatorFn, CRC16_TMS37157),
        CrcAlgorithm::Crc16Umts => (Calculator::calculate as CalculatorFn, CRC16_UMTS),
        CrcAlgorithm::Crc16Usb => (Calculator::calculate as CalculatorFn, CRC16_USB),
        CrcAlgorithm::Crc16Xmodem => (Calculator::calculate as CalculatorFn, CRC16_XMODEM),
        CrcAlgorithm::Crc32Aixm => (Calculator::calculate as CalculatorFn, CRC32_AIXM),
        CrcAlgorithm::Crc32Autosar => (Calculator::calculate as CalculatorFn, CRC32_AUTOSAR),
        CrcAlgorithm::Crc32Base91D => (Calculator::calculate as CalculatorFn, CRC32_BASE91_D),
        CrcAlgorithm::Crc32Bzip2 => (Calculator::calculate as CalculatorFn, CRC32_BZIP2),
        CrcAlgorithm::Crc32CdRomEdc => (Calculator::calculate as CalculatorFn, CRC32_CD_ROM_EDC),
        CrcAlgorithm::Crc32Cksum => (Calculator::calculate as CalculatorFn, CRC32_CKSUM),
        CrcAlgorithm::Crc32Custom => {
            panic!("Custom CRC-32 requires parameters via CrcParams::new()")
        }
        CrcAlgorithm::Crc32Iscsi => (crc32_iscsi_calculator as CalculatorFn, CRC32_ISCSI),
        CrcAlgorithm::Crc32IsoHdlc => (crc32_iso_hdlc_calculator as CalculatorFn, CRC32_ISO_HDLC),
        CrcAlgorithm::Crc32Jamcrc => (Calculator::calculate as CalculatorFn, CRC32_JAMCRC),
        CrcAlgorithm::Crc32Mef => (Calculator::calculate as CalculatorFn, CRC32_MEF),
        CrcAlgorithm::Crc32Mpeg2 => (Calculator::calculate as CalculatorFn, CRC32_MPEG_2),
        CrcAlgorithm::Crc32Xfer => (Calculator::calculate as CalculatorFn, CRC32_XFER),
        CrcAlgorithm::CrcCustom => {
            panic!("Custom CRC requires parameters via CrcParams::new()")
        }
        CrcAlgorithm::Crc64Custom => {
            panic!("Custom CRC-64 requires parameters via CrcParams::new()")
        }
        CrcAlgorithm::Crc64Ecma182 => (Calculator::calculate as CalculatorFn, CRC64_ECMA_182),
        CrcAlgorithm::Crc64GoIso => (Calculator::calculate as CalculatorFn, CRC64_GO_ISO),
        CrcAlgorithm::Crc64Ms => (Calculator::calculate as CalculatorFn, CRC64_MS),
        CrcAlgorithm::Crc64Nvme => (Calculator::calculate as CalculatorFn, CRC64_NVME),
        CrcAlgorithm::Crc64Redis => (Calculator::calculate as CalculatorFn, CRC64_REDIS),
        CrcAlgorithm::Crc64We => (Calculator::calculate as CalculatorFn, CRC64_WE),
        CrcAlgorithm::Crc64Xz => (Calculator::calculate as CalculatorFn, CRC64_XZ),
    }
}

/// Calculates the CRC-32/ISCSI (commonly called "crc32c" in many, but not all, implementations)
/// checksum.
///
/// Because both aarch64 and x86 have native hardware support for CRC-32/ISCSI, we can use
/// fusion techniques to accelerate the calculation beyond what SIMD can do alone.
#[inline(always)]
fn crc32_iscsi_calculator(state: u64, data: &[u8], _params: &CrcParams) -> u64 {
    #[cfg(all(target_arch = "aarch64", feature = "std"))]
    {
        use crate::feature_detection::PerformanceTier;

        let arch_ops = get_arch_ops();
        match arch_ops.get_tier() {
            PerformanceTier::AArch64AesSha3 | PerformanceTier::AArch64Aes => {
                return fusion::crc32_iscsi(state as u32, data) as u64;
            }
            _ => {}
        }
    }

    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "std"))]
    {
        use crate::feature_detection::PerformanceTier;

        let arch_ops = get_arch_ops();
        match arch_ops.get_tier() {
            PerformanceTier::X86_64Avx512Vpclmulqdq
            | PerformanceTier::X86_64Avx512Pclmulqdq
            | PerformanceTier::X86_64SsePclmulqdq
            | PerformanceTier::X86SsePclmulqdq => {
                // fusion path requires both pclmulqdq (checked by tier) and sse4.2 (for CRC32 instructions)
                if is_x86_feature_detected!("sse4.2") {
                    return fusion::crc32_iscsi(state as u32, data) as u64;
                }
            }
            _ => {}
        }
    }

    Calculator::calculate(state, data, _params)
}

/// Calculates the CRC-32/ISO-HDLC (commonly called "crc32" in many, but not all, implementations)
/// checksum.
///
/// Because aarch64 has native hardware support for CRC-32/ISO-HDLC, we can use fusion techniques
/// to accelerate the calculation beyond what SIMD can do alone. x86 does not have native support,
/// so we use the traditional calculation.
#[inline(always)]
fn crc32_iso_hdlc_calculator(state: u64, data: &[u8], _params: &CrcParams) -> u64 {
    #[cfg(all(target_arch = "aarch64", feature = "std"))]
    {
        use crate::feature_detection::{get_arch_ops, PerformanceTier};
        let arch_ops = get_arch_ops();

        match arch_ops.get_tier() {
            PerformanceTier::AArch64AesSha3 | PerformanceTier::AArch64Aes => {
                return fusion::crc32_iso_hdlc(state as u32, data) as u64;
            }
            _ => {}
        }
    }

    Calculator::calculate(state, data, _params)
}

#[cfg(test)]
mod lib {
    #![allow(unused)]

    use super::*;
    use crate::test::consts::{TEST_ALL_CONFIGS, TEST_CHECK_STRING};
    use crate::test::enums::AnyCrcTestConfig;
    use cbindgen::Language::C;
    use cbindgen::Style::Both;
    use rand::{rng, Rng};
    use std::fs::{read, write};

    #[test]
    fn test_checksum_check() {
        for config in TEST_ALL_CONFIGS {
            assert_eq!(
                checksum(config.get_algorithm(), TEST_CHECK_STRING),
                config.get_check()
            );
        }
    }

    #[test]
    fn test_checksum_reference() {
        for config in TEST_ALL_CONFIGS {
            assert_eq!(
                checksum(config.get_algorithm(), TEST_CHECK_STRING),
                config.checksum_with_reference(TEST_CHECK_STRING)
            );
        }
    }

    #[test]
    fn test_checksum_with_custom_params() {
        crate::cache::clear_cache();

        // CRC-32 reflected
        assert_eq!(
            checksum_with_params(get_custom_crc32_reflected(), TEST_CHECK_STRING),
            CRC32_ISCSI.check,
        );

        // CRC-32 forward
        assert_eq!(
            checksum_with_params(get_custom_crc32_forward(), TEST_CHECK_STRING),
            CRC32_BZIP2.check,
        );

        // CRC-64 reflected
        assert_eq!(
            checksum_with_params(get_custom_crc64_reflected(), TEST_CHECK_STRING),
            CRC64_NVME.check,
        );

        // CRC-64 forward
        assert_eq!(
            checksum_with_params(get_custom_crc64_forward(), TEST_CHECK_STRING),
            CRC64_ECMA_182.check,
        );
    }

    #[test]
    fn test_get_custom_params() {
        crate::cache::clear_cache();

        assert_eq!(
            checksum_with_params(get_custom_crc32_reflected(), TEST_CHECK_STRING),
            CRC32_ISCSI.check,
        );

        assert_eq!(
            checksum_with_params(get_custom_crc32_forward(), TEST_CHECK_STRING),
            CRC32_BZIP2.check,
        );

        assert_eq!(
            checksum_with_params(get_custom_crc64_reflected(), TEST_CHECK_STRING),
            CRC64_NVME.check,
        );

        assert_eq!(
            checksum_with_params(get_custom_crc64_forward(), TEST_CHECK_STRING),
            CRC64_ECMA_182.check,
        );
    }

    #[test]
    fn test_get_calculator_target_format() {
        let target = get_calculator_target(CrcAlgorithm::Crc32IsoHdlc);

        // Target string should not be empty
        assert!(!target.is_empty());

        // Should follow the expected format with valid architecture prefixes
        let valid_prefixes = ["aarch64-", "x86_64-", "x86-", "software-"];
        assert!(
            valid_prefixes
                .iter()
                .any(|prefix| target.starts_with(prefix)),
            "Target '{}' should start with a valid architecture prefix",
            target
        );

        // Should contain intrinsics family and features information
        let parts: Vec<&str> = target.split('-').collect();
        assert!(
            parts.len() >= 3,
            "Target '{}' should have at least 3 parts: architecture-family-features",
            target
        );
    }

    #[test]
    fn test_get_calculator_target_consistency() {
        // Multiple calls should return the same result (deterministic)
        let target1 = get_calculator_target(CrcAlgorithm::Crc32IsoHdlc);
        let target2 = get_calculator_target(CrcAlgorithm::Crc32Iscsi);
        let target3 = get_calculator_target(CrcAlgorithm::Crc64Nvme);

        assert_eq!(
            target1, target2,
            "Target should be consistent across different CRC-32 algorithms"
        );
        assert_eq!(
            target1, target3,
            "Target should be consistent across CRC-32 and CRC-64 algorithms"
        );
    }

    #[test]
    fn test_get_calculator_target_uses_cached_detection() {
        // This test verifies that the function uses cached feature detection
        // by checking that multiple calls are consistent and don't perform
        // redundant feature detection

        let target1 = get_calculator_target(CrcAlgorithm::Crc32IsoHdlc);
        let target2 = get_calculator_target(CrcAlgorithm::Crc32IsoHdlc);

        assert_eq!(
            target1, target2,
            "Cached detection should return identical results"
        );
    }

    #[test]
    fn test_digest_updates_check() {
        for config in TEST_ALL_CONFIGS {
            check_digest(Digest::new(config.get_algorithm()), config.get_check());
        }
    }

    #[test]
    fn test_digest_updates_check_with_custom_params() {
        crate::cache::clear_cache();

        // CRC-32 reflected
        check_digest(
            Digest::new_with_params(get_custom_crc32_reflected()),
            CRC32_ISCSI.check,
        );

        // CRC-32 forward
        check_digest(
            Digest::new_with_params(get_custom_crc32_forward()),
            CRC32_BZIP2.check,
        );

        // CRC-64 reflected
        check_digest(
            Digest::new_with_params(get_custom_crc64_reflected()),
            CRC64_NVME.check,
        );

        // CRC-64 forward
        check_digest(
            Digest::new_with_params(get_custom_crc64_forward()),
            CRC64_ECMA_182.check,
        );
    }

    fn check_digest(mut digest: Digest, check: u64) {
        digest.update(b"123");
        digest.update(b"456");
        digest.update(b"789");
        assert_eq!(digest.finalize(), check,);
    }

    #[test]
    fn test_1024_length() {
        for config in TEST_ALL_CONFIGS {
            test_length(1024, config);
        }
    }

    /// Skipping for Miri runs due to time constraints, underlying code already covered by other
    /// tests.
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_small_all_lengths() {
        for config in TEST_ALL_CONFIGS {
            // Test each length from 1 to 255
            for len in 1..=255 {
                test_length(len, config);
            }
        }
    }

    /// Skipping for Miri runs due to time constraints, underlying code already covered by other
    /// tests.
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_medium_lengths() {
        for config in TEST_ALL_CONFIGS {
            // Test each length from 256 to 1024, which should fold and include handling remainders
            for len in 256..=1024 {
                test_length(len, config);
            }
        }
    }

    /// Skipping for Miri runs due to time constraints, underlying code already covered by other
    /// tests.
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_large_lengths() {
        for config in TEST_ALL_CONFIGS {
            // Test 1 MiB just before, at, and just after the folding boundaries
            for len in 1048575..1048577 {
                test_length(len, config);
            }
        }
    }

    fn test_length(length: usize, config: &AnyCrcTestConfig) {
        let mut data = vec![0u8; length];
        rng().fill(&mut data[..]);

        // Calculate expected CRC using the reference implementation
        let expected = config.checksum_with_reference(&data);

        let result = checksum(config.get_algorithm(), &data);

        assert_eq!(
            result,
            expected,
            "Failed for algorithm: {:?}, length: {}, expected: {:#x}, got: {:#x}",
            config.get_algorithm(),
            length,
            expected,
            result
        );
    }

    #[test]
    fn test_combine() {
        for config in TEST_ALL_CONFIGS {
            let algorithm = config.get_algorithm();
            let check = config.get_check();

            // checksums
            let checksum1 = checksum(algorithm, "1234".as_ref());
            let checksum2 = checksum(algorithm, "56789".as_ref());

            // checksum_combine()
            assert_eq!(checksum_combine(algorithm, checksum1, checksum2, 5), check);

            // Digest
            let mut digest1 = Digest::new(algorithm);
            digest1.update("1234".as_ref());

            let mut digest2 = Digest::new(algorithm);
            digest2.update("56789".as_ref());

            digest1.combine(&digest2);

            assert_eq!(digest1.finalize(), check)
        }
    }

    #[test]
    fn test_combine_with_custom_params() {
        crate::cache::clear_cache();

        // CRC-32 reflected
        let crc32_params = get_custom_crc32_reflected();
        let checksum1 = checksum_with_params(crc32_params, "1234".as_ref());
        let checksum2 = checksum_with_params(crc32_params, "56789".as_ref());
        assert_eq!(
            checksum_combine_with_params(crc32_params, checksum1, checksum2, 5),
            CRC32_ISCSI.check,
        );

        // CRC-32 forward
        let crc32_params = get_custom_crc32_forward();
        let checksum1 = checksum_with_params(crc32_params, "1234".as_ref());
        let checksum2 = checksum_with_params(crc32_params, "56789".as_ref());
        assert_eq!(
            checksum_combine_with_params(crc32_params, checksum1, checksum2, 5),
            CRC32_BZIP2.check,
        );

        // CRC-64 reflected
        let crc64_params = get_custom_crc64_reflected();
        let checksum1 = checksum_with_params(crc64_params, "1234".as_ref());
        let checksum2 = checksum_with_params(crc64_params, "56789".as_ref());
        assert_eq!(
            checksum_combine_with_params(crc64_params, checksum1, checksum2, 5),
            CRC64_NVME.check,
        );

        // CRC-64 forward
        let crc64_params = get_custom_crc64_forward();
        let checksum1 = checksum_with_params(crc64_params, "1234".as_ref());
        let checksum2 = checksum_with_params(crc64_params, "56789".as_ref());
        assert_eq!(
            checksum_combine_with_params(crc64_params, checksum1, checksum2, 5),
            CRC64_ECMA_182.check,
        );
    }

    /// Skipping for Miri runs due to isolation constraints, underlying code other than I/O already
    /// covered by other tests.
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_checksum_file() {
        // Create a test file with repeating zeros
        let test_file_path = "test/test_crc32_hash_file.bin";
        let data = vec![0u8; 1024 * 1024]; // 1 MiB of zeros
        if let Err(e) = write(test_file_path, &data) {
            eprintln!("Skipping test due to write error: {}", e);
            return;
        }

        for config in TEST_ALL_CONFIGS {
            let result = checksum_file(config.get_algorithm(), test_file_path, None).unwrap();
            assert_eq!(result, config.checksum_with_reference(&data));
        }

        std::fs::remove_file(test_file_path).unwrap();
    }

    /// Skipping for Miri runs due to isolation constraints, underlying code other than I/O already
    /// covered by other tests.
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_checksum_file_with_custom_params() {
        crate::cache::clear_cache();

        // Create a test file with repeating zeros
        let test_file_path = "test/test_crc32_hash_file_custom.bin";
        let data = vec![0u8; 1024 * 1024]; // 1 MiB of zeros
        if let Err(e) = write(test_file_path, &data) {
            eprintln!("Skipping test due to write error: {}", e);
            return;
        }

        // CRC-32 reflected
        check_file(
            get_custom_crc32_reflected(),
            test_file_path,
            CRC32_ISCSI.check,
        );

        // CRC-32 forward
        check_file(
            get_custom_crc32_forward(),
            test_file_path,
            CRC32_BZIP2.check,
        );

        // CRC-64 reflected
        check_file(
            get_custom_crc64_reflected(),
            test_file_path,
            CRC64_NVME.check,
        );

        // CRC-64 forward
        check_file(
            get_custom_crc64_forward(),
            test_file_path,
            CRC64_ECMA_182.check,
        );

        std::fs::remove_file(test_file_path).unwrap();
    }

    fn check_file(params: CrcParams, file_path: &str, check: u64) {
        let result = checksum_file_with_params(params, file_path, None).unwrap();
        assert_eq!(result, check);
    }

    /// Skipping for Miri runs due to isolation constraints, underlying code other than I/O already
    /// covered by other tests.
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_writer() {
        // Create a test file with repeating zeros
        let test_file_path = "test/test_crc32_writer_file.bin";
        let data = vec![0u8; 1024 * 1024]; // 1 MiB of zeros
        if let Err(e) = std::fs::write(test_file_path, &data) {
            eprintln!("Skipping test due to write error: {}", e);
            return;
        }

        for config in TEST_ALL_CONFIGS {
            let mut digest = Digest::new(config.get_algorithm());
            let mut file = File::open(test_file_path).unwrap();
            std::io::copy(&mut file, &mut digest).unwrap();
            assert_eq!(digest.finalize(), config.checksum_with_reference(&data));
        }

        std::fs::remove_file(test_file_path).unwrap();
    }
    #[test]
    fn test_digest_reset() {
        for config in TEST_ALL_CONFIGS {
            let mut digest = Digest::new(config.get_algorithm());
            digest.update(b"42");
            digest.reset();
            digest.update(TEST_CHECK_STRING);
            assert_eq!(digest.finalize(), config.get_check());
        }
    }

    #[test]
    fn test_digest_finalize_reset() {
        for config in TEST_ALL_CONFIGS {
            let check = config.get_check();

            let mut digest = Digest::new(config.get_algorithm());
            digest.update(TEST_CHECK_STRING);
            assert_eq!(digest.finalize_reset(), check);

            digest.update(TEST_CHECK_STRING);
            assert_eq!(digest.finalize(), check);
        }
    }

    #[test]
    fn test_digest_finalize_into() {
        for config in TEST_ALL_CONFIGS {
            let mut digest = Digest::new(config.get_algorithm());
            digest.update(TEST_CHECK_STRING);

            match digest.params.width {
                16 => {
                    let mut output = [0u8; 2];
                    digest.finalize_into(&mut output).unwrap();
                    let result = u16::from_be_bytes(output) as u64;
                    assert_eq!(result, config.get_check());
                }
                32 => {
                    let mut output = [0u8; 4];
                    digest.finalize_into(&mut output).unwrap();
                    let result = u32::from_be_bytes(output) as u64;
                    assert_eq!(result, config.get_check());
                }
                64 => {
                    let mut output = [0u8; 8];
                    digest.finalize_into(&mut output).unwrap();
                    let result = u64::from_be_bytes(output);
                    assert_eq!(result, config.get_check());
                }
                _ => panic!("Unsupported CRC width"),
            }
        }
    }

    #[test]
    fn test_digest_finalize_into_reset() {
        for config in TEST_ALL_CONFIGS {
            let mut digest = Digest::new(config.get_algorithm());
            digest.update(TEST_CHECK_STRING);

            let mut output: Vec<u8> = match digest.params.width {
                16 => vec![0u8; 2],
                32 => vec![0u8; 4],
                64 => vec![0u8; 8],
                _ => panic!("Unsupported CRC width"),
            };

            digest.finalize_into_reset(&mut output).unwrap();
            let result = match output.len() {
                2 => u16::from_be_bytes(output.try_into().unwrap()) as u64,
                4 => u32::from_be_bytes(output.try_into().unwrap()) as u64,
                8 => u64::from_be_bytes(output.try_into().unwrap()),
                _ => panic!("Unsupported CRC width"),
            };
            assert_eq!(result, config.get_check());

            digest.update(TEST_CHECK_STRING);
            assert_eq!(digest.finalize(), config.get_check());
        }
    }

    /// Tests whether the FFI header is up-to-date
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_ffi_header() -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            // Skip this test on Windows, since CRLF vs LF is a PITA
            eprintln!("Skipping test on Windows");

            return Ok(());
        }

        const HEADER: &str = "libcrc_fast.h";

        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|error| error.to_string())?;

        let mut expected = Vec::new();
        cbindgen::Builder::new()
            .with_crate(crate_dir)
            .with_include_guard("CRC_FAST_H")
            .with_header("/* crc_fast library C/C++ API - Copyright 2025 Don MacAskill */\n/* This header is auto-generated. Do not edit directly. */\n")
            // exclude internal implementation functions
            .exclude_item("crc32_iscsi_impl")
            .exclude_item("crc32_iso_hdlc_impl")
            .exclude_item("get_iscsi_target")
            .exclude_item("get_iso_hdlc_target")
            .exclude_item("ISO_HDLC_TARGET")
            .exclude_item("ISCSI_TARGET")
            .exclude_item("CrcParams")
            .rename_item("Digest", "CrcFastDigest")
            .with_style(Both)
            // generate C header
            .with_language(C)
            // with C++ compatibility
            .with_cpp_compat(true)
            .generate()
            .map_err(|error| error.to_string())?
            .write(&mut expected);

        // Convert the expected bytes to string for pattern replacement, since cbindgen
        // generates an annoying amount of empty contiguous newlines
        let header_content = String::from_utf8(expected).map_err(|error| error.to_string())?;

        // Replace excessive newlines (3 or more consecutive newlines) with 2 newlines
        let regex = regex::Regex::new(r"\n{3,}").map_err(|error| error.to_string())?;
        let cleaned_content = regex.replace_all(&header_content, "\n\n").to_string();

        // Convert back to bytes
        expected = cleaned_content.into_bytes();

        let actual = read(HEADER).map_err(|error| error.to_string())?;

        if expected != actual {
            write(HEADER, expected).map_err(|error| error.to_string())?;
            return Err(format!(
                "{HEADER} is not up-to-date, commit the generated file and try again"
            ));
        }

        Ok(())
    }

    fn get_custom_crc32_reflected() -> CrcParams {
        CrcParams::new(
            "Custom CRC-32/ISCSI",
            32,
            CRC32_ISCSI.poly,
            CRC32_ISCSI.init,
            CRC32_ISCSI.refin,
            CRC32_ISCSI.xorout,
            CRC32_ISCSI.check,
        )
    }

    fn get_custom_crc32_forward() -> CrcParams {
        CrcParams::new(
            "Custom CRC-32/BZIP2",
            32,
            CRC32_BZIP2.poly,
            CRC32_BZIP2.init,
            CRC32_BZIP2.refin,
            CRC32_BZIP2.xorout,
            CRC32_BZIP2.check,
        )
    }

    fn get_custom_crc64_reflected() -> CrcParams {
        CrcParams::new(
            "Custom CRC-64/NVME",
            64,
            CRC64_NVME.poly,
            CRC64_NVME.init,
            CRC64_NVME.refin,
            CRC64_NVME.xorout,
            CRC64_NVME.check,
        )
    }

    fn get_custom_crc64_forward() -> CrcParams {
        CrcParams::new(
            "Custom CRC-64/ECMA-182",
            64,
            CRC64_ECMA_182.poly,
            CRC64_ECMA_182.init,
            CRC64_ECMA_182.refin,
            CRC64_ECMA_182.xorout,
            CRC64_ECMA_182.check,
        )
    }

    #[test]
    #[allow(clippy::needless_range_loop)] // Intentionally testing indexed get_key() method
    fn test_crc_keys_storage_fold_256() {
        let test_keys = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        ];
        let storage = CrcKeysStorage::from_keys_fold_256(test_keys);

        // Test valid key access
        for i in 0..23 {
            assert_eq!(storage.get_key(i), test_keys[i]);
        }

        // Test out-of-bounds access returns 0
        assert_eq!(storage.get_key(23), 0);
        assert_eq!(storage.get_key(24), 0);
        assert_eq!(storage.get_key(100), 0);

        // Test key count
        assert_eq!(storage.key_count(), 23);
    }

    #[test]
    #[allow(clippy::needless_range_loop)] // Intentionally testing indexed get_key() method
    fn test_crc_keys_storage_future_test() {
        let test_keys = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ];
        let storage = CrcKeysStorage::from_keys_fold_future_test(test_keys);

        // Test valid key access
        for i in 0..25 {
            assert_eq!(storage.get_key(i), test_keys[i]);
        }

        // Test out-of-bounds access returns 0
        assert_eq!(storage.get_key(25), 0);
        assert_eq!(storage.get_key(26), 0);
        assert_eq!(storage.get_key(100), 0);

        // Test key count
        assert_eq!(storage.key_count(), 25);
    }

    #[test]
    #[allow(clippy::needless_range_loop)] // Intentionally testing indexed get_key() and get_key_checked() methods
    fn test_crc_params_safe_accessors() {
        // Create a test CrcParams with known keys
        let test_keys = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        ];
        let params = CrcParams {
            algorithm: CrcAlgorithm::Crc32IsoHdlc,
            name: "test",
            width: 32,
            poly: 0x04C11DB7,
            init: 0xFFFFFFFF,
            init_algorithm: 0xFFFFFFFF,
            refin: true,
            refout: true,
            xorout: 0xFFFFFFFF,
            check: 0xCBF43926,
            keys: CrcKeysStorage::from_keys_fold_256(test_keys),
        };

        // Test valid key access
        for i in 0..23 {
            assert_eq!(params.get_key(i), test_keys[i]);
            assert_eq!(params.get_key_checked(i), Some(test_keys[i]));
        }

        // Test out-of-bounds access
        assert_eq!(params.get_key(23), 0);
        assert_eq!(params.get_key(24), 0);
        assert_eq!(params.get_key(100), 0);

        assert_eq!(params.get_key_checked(23), None);
        assert_eq!(params.get_key_checked(24), None);
        assert_eq!(params.get_key_checked(100), None);

        // Test key count
        assert_eq!(params.key_count(), 23);
    }

    #[test]
    fn test_crc_keys_storage_const_constructors() {
        // Test that const constructors work in const context
        const TEST_KEYS_23: [u64; 23] = [1; 23];
        const TEST_KEYS_25: [u64; 25] = [2; 25];

        const STORAGE_256: CrcKeysStorage = CrcKeysStorage::from_keys_fold_256(TEST_KEYS_23);
        const STORAGE_FUTURE: CrcKeysStorage =
            CrcKeysStorage::from_keys_fold_future_test(TEST_KEYS_25);

        // Verify the const constructors work correctly
        assert_eq!(STORAGE_256.get_key(0), 1);
        assert_eq!(STORAGE_256.key_count(), 23);

        assert_eq!(STORAGE_FUTURE.get_key(0), 2);
        assert_eq!(STORAGE_FUTURE.key_count(), 25);
    }

    #[test]
    fn test_crc_keys_storage_bounds_safety() {
        let storage_256 = CrcKeysStorage::from_keys_fold_256([42; 23]);
        let storage_future = CrcKeysStorage::from_keys_fold_future_test([84; 25]);

        // Test edge cases for bounds checking
        assert_eq!(storage_256.get_key(22), 42); // Last valid index
        assert_eq!(storage_256.get_key(23), 0); // First invalid index

        assert_eq!(storage_future.get_key(24), 84); // Last valid index
        assert_eq!(storage_future.get_key(25), 0); // First invalid index

        // Test very large indices
        assert_eq!(storage_256.get_key(usize::MAX), 0);
        assert_eq!(storage_future.get_key(usize::MAX), 0);
    }
}
