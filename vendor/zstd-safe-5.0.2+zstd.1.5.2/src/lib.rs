#![no_std]
//! Minimal safe wrapper around zstd-sys.
//!
//! This crates provides a minimal translation of the [zstd-sys] methods.
//! For a more comfortable high-level library, see the [zstd] crate.
//!
//! [zstd-sys]: https://crates.io/crates/zstd-sys
//! [zstd]: https://crates.io/crates/zstd
//!
//! Most of the functions here map 1-for-1 to a function from
//! [the C zstd library][zstd-c] mentioned in their descriptions.
//! Check the [source documentation][doc] for more information on their
//! behaviour.
//!
//! [doc]: https://facebook.github.io/zstd/zstd_manual.html
//! [zstd-c]: https://facebook.github.io/zstd/
//!
//! Features denoted as experimental in the C library are hidden behind an
//! `experimental` feature.
#![cfg_attr(feature = "doc-cfg", feature(doc_cfg))]

#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
mod tests;

// Re-export zstd-sys
pub use zstd_sys;

/// How to compress data.
pub use zstd_sys::ZSTD_strategy as Strategy;

/// Reset directive.
pub use zstd_sys::ZSTD_ResetDirective as ResetDirective;

#[cfg(feature = "std")]
use std::os::raw::{c_char, c_int, c_ulonglong, c_void};

#[cfg(not(feature = "std"))]
use libc::{c_char, c_int, c_ulonglong, c_void};

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::str;

include!("constants.rs");

#[cfg(feature = "experimental")]
include!("constants_experimental.rs");

/// Represents the compression level used by zstd.
pub type CompressionLevel = i32;

/// Represents a possible error from the zstd library.
pub type ErrorCode = usize;

/// Wrapper result around most zstd functions.
///
/// Either a success code (usually number of bytes written), or an error code.
pub type SafeResult = Result<usize, ErrorCode>;

/// Returns true if code represents error.
fn is_error(code: usize) -> bool {
    unsafe { zstd_sys::ZSTD_isError(code) != 0 }
}

/// Parse the result code
///
/// Returns the number of bytes written if the code represents success,
/// or the error message code otherwise.
fn parse_code(code: usize) -> SafeResult {
    if !is_error(code) {
        Ok(code)
    } else {
        Err(code)
    }
}

fn ptr_void(src: &[u8]) -> *const c_void {
    src.as_ptr() as *const c_void
}

fn ptr_mut_void(dst: &mut (impl WriteBuf + ?Sized)) -> *mut c_void {
    dst.as_mut_ptr() as *mut c_void
}

pub fn version_number() -> u32 {
    unsafe { zstd_sys::ZSTD_versionNumber() as u32 }
}

pub fn version_string() -> &'static str {
    unsafe { c_char_to_str(zstd_sys::ZSTD_versionString()) }
}

/// Returns the minimum (fastest) compression level supported.
pub fn min_c_level() -> CompressionLevel {
    unsafe { zstd_sys::ZSTD_minCLevel() as CompressionLevel }
}

/// Returns the maximum (slowest) compression level supported.
pub fn max_c_level() -> CompressionLevel {
    unsafe { zstd_sys::ZSTD_maxCLevel() as CompressionLevel }
}

/// Wraps the `ZSTD_compress` function.
pub fn compress<C: WriteBuf + ?Sized>(
    dst: &mut C,
    src: &[u8],
    compression_level: CompressionLevel,
) -> SafeResult {
    unsafe {
        dst.write_from(|buffer, capacity| {
            parse_code(zstd_sys::ZSTD_compress(
                buffer,
                capacity,
                ptr_void(src),
                src.len(),
                compression_level,
            ))
        })
    }
}

/// Wraps the `ZSTD_decompress` function.
pub fn decompress<C: WriteBuf + ?Sized>(
    dst: &mut C,
    src: &[u8],
) -> SafeResult {
    unsafe {
        dst.write_from(|buffer, capacity| {
            parse_code(zstd_sys::ZSTD_decompress(
                buffer,
                capacity,
                ptr_void(src),
                src.len(),
            ))
        })
    }
}

/// Wraps the `ZSTD_getDecompressedSize` function.
#[deprecated(note = "Use ZSTD_getFrameContentSize instead")]
pub fn get_decompressed_size(src: &[u8]) -> u64 {
    unsafe {
        zstd_sys::ZSTD_getDecompressedSize(ptr_void(src), src.len()) as u64
    }
}

/// maximum compressed size in worst case single-pass scenario
pub fn compress_bound(src_size: usize) -> usize {
    unsafe { zstd_sys::ZSTD_compressBound(src_size) }
}

pub struct CCtx<'a>(NonNull<zstd_sys::ZSTD_CCtx>, PhantomData<&'a ()>);

impl Default for CCtx<'_> {
    fn default() -> Self {
        CCtx::create()
    }
}

impl CCtx<'static> {
    /// Tries to create a new context.
    ///
    /// Returns `None` if zstd returns a NULL pointer - may happen if allocation fails.
    pub fn try_create() -> Option<Self> {
        Some(CCtx(
            NonNull::new(unsafe { zstd_sys::ZSTD_createCCtx() })?,
            PhantomData,
        ))
    }

    /// Wrap `ZSTD_createCCtx`
    ///
    /// # Panics
    ///
    /// If zstd returns a NULL pointer.
    pub fn create() -> Self {
        Self::try_create()
            .expect("zstd returned null pointer when creating new context")
    }
}

impl<'a> CCtx<'a> {
    /// Wraps the `ZSTD_compressCCtx()` function
    pub fn compress<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
        compression_level: CompressionLevel,
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_compressCCtx(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                    compression_level,
                ))
            })
        }
    }

    /// Wraps the `ZSTD_compress2()` function.
    pub fn compress2<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_compress2(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                ))
            })
        }
    }

    /// Wraps the `ZSTD_compress_usingDict()` function.
    pub fn compress_using_dict<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
        dict: &[u8],
        compression_level: CompressionLevel,
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_compress_usingDict(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                    ptr_void(dict),
                    dict.len(),
                    compression_level,
                ))
            })
        }
    }

    /// Wraps the `ZSTD_compress_usingCDict()` function.
    pub fn compress_using_cdict<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
        cdict: &CDict<'_>,
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_compress_usingCDict(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                    cdict.0.as_ptr(),
                ))
            })
        }
    }

    pub fn init(&mut self, compression_level: CompressionLevel) -> usize {
        unsafe {
            zstd_sys::ZSTD_initCStream(self.0.as_ptr(), compression_level)
        }
    }

    /// Wraps the `ZSTD_initCStream_srcSize()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    #[deprecated]
    pub fn init_src_size(
        &mut self,
        compression_level: CompressionLevel,
        pledged_src_size: u64,
    ) -> usize {
        unsafe {
            zstd_sys::ZSTD_initCStream_srcSize(
                self.0.as_ptr(),
                compression_level as c_int,
                pledged_src_size as c_ulonglong,
            )
        }
    }

    /// Wraps the `ZSTD_initCStream_usingDict()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    #[deprecated]
    pub fn init_using_dict(
        &mut self,
        dict: &[u8],
        compression_level: CompressionLevel,
    ) -> SafeResult {
        let code = unsafe {
            zstd_sys::ZSTD_initCStream_usingDict(
                self.0.as_ptr(),
                ptr_void(dict),
                dict.len(),
                compression_level,
            )
        };
        parse_code(code)
    }

    /// Wraps the `ZSTD_initCStream_usingCDict()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    #[deprecated]
    pub fn init_using_cdict<'b>(&mut self, cdict: &CDict<'b>) -> SafeResult
    where
        'b: 'a, // Dictionary outlives the stream.
    {
        let code = unsafe {
            zstd_sys::ZSTD_initCStream_usingCDict(
                self.0.as_ptr(),
                cdict.0.as_ptr(),
            )
        };
        parse_code(code)
    }

    pub fn load_dictionary(&mut self, dict: &[u8]) -> SafeResult {
        parse_code(unsafe {
            zstd_sys::ZSTD_CCtx_loadDictionary(
                self.0.as_ptr(),
                ptr_void(dict),
                dict.len(),
            )
        })
    }

    /// Wraps the `ZSTD_CCtx_refCDict()` function.
    ///
    /// Dictionary must outlive the context.
    pub fn ref_cdict<'b>(&mut self, cdict: &CDict<'b>) -> SafeResult
    where
        'b: 'a,
    {
        parse_code(unsafe {
            zstd_sys::ZSTD_CCtx_refCDict(self.0.as_ptr(), cdict.0.as_ptr())
        })
    }

    pub fn ref_prefix<'b>(&mut self, prefix: &'b [u8]) -> SafeResult
    where
        'b: 'a,
    {
        parse_code(unsafe {
            zstd_sys::ZSTD_CCtx_refPrefix(
                self.0.as_ptr(),
                ptr_void(prefix),
                prefix.len(),
            )
        })
    }

    pub fn compress_stream<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
        input: &mut InBuffer<'_>,
    ) -> SafeResult {
        let mut output = output.wrap();
        let mut input = input.wrap();
        let code = unsafe {
            zstd_sys::ZSTD_compressStream(
                self.0.as_ptr(),
                ptr_mut(&mut output),
                ptr_mut(&mut input),
            )
        };
        parse_code(code)
    }

    /// Wraps the `ZSTD_compressStream2()` function.
    pub fn compress_stream2<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
        input: &mut InBuffer<'_>,
        end_op: zstd_sys::ZSTD_EndDirective,
    ) -> SafeResult {
        let mut output = output.wrap();
        let mut input = input.wrap();
        parse_code(unsafe {
            zstd_sys::ZSTD_compressStream2(
                self.0.as_ptr(),
                ptr_mut(&mut output),
                ptr_mut(&mut input),
                end_op,
            )
        })
    }

    /// Wraps the `ZSTD_flushStream()` function.
    pub fn flush_stream<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
    ) -> SafeResult {
        let mut output = output.wrap();
        let code = unsafe {
            zstd_sys::ZSTD_flushStream(self.0.as_ptr(), ptr_mut(&mut output))
        };
        parse_code(code)
    }

    /// Wraps the `ZSTD_endStream()` function.
    pub fn end_stream<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
    ) -> SafeResult {
        let mut output = output.wrap();
        let code = unsafe {
            zstd_sys::ZSTD_endStream(self.0.as_ptr(), ptr_mut(&mut output))
        };
        parse_code(code)
    }

    pub fn sizeof(&self) -> usize {
        unsafe { zstd_sys::ZSTD_sizeof_CCtx(self.0.as_ptr()) }
    }

    pub fn reset(&mut self, reset: ResetDirective) -> SafeResult {
        parse_code(unsafe {
            zstd_sys::ZSTD_CCtx_reset(self.0.as_ptr(), reset)
        })
    }

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    #[deprecated]
    pub fn reset_cstream(&mut self, pledged_src_size: u64) -> SafeResult {
        let code = unsafe {
            zstd_sys::ZSTD_resetCStream(
                self.0.as_ptr(),
                pledged_src_size as c_ulonglong,
            )
        };
        parse_code(code)
    }

    pub fn set_parameter(&mut self, param: CParameter) -> SafeResult {
        // TODO: Until bindgen properly generates a binding for this, we'll need to do it here.

        #[cfg(feature = "experimental")]
        use zstd_sys::ZSTD_cParameter::{
            ZSTD_c_experimentalParam1 as ZSTD_c_rsyncable,
            ZSTD_c_experimentalParam10 as ZSTD_c_stableOutBuffer,
            ZSTD_c_experimentalParam11 as ZSTD_c_blockDelimiters,
            ZSTD_c_experimentalParam12 as ZSTD_c_validateSequences,
            ZSTD_c_experimentalParam13 as ZSTD_c_useBlockSplitter,
            ZSTD_c_experimentalParam14 as ZSTD_c_useRowMatchFinder,
            ZSTD_c_experimentalParam15 as ZSTD_c_deterministicRefPrefix,
            ZSTD_c_experimentalParam2 as ZSTD_c_format,
            ZSTD_c_experimentalParam3 as ZSTD_c_forceMaxWindow,
            ZSTD_c_experimentalParam4 as ZSTD_c_forceAttachDict,
            ZSTD_c_experimentalParam5 as ZSTD_c_literalCompressionMode,
            ZSTD_c_experimentalParam6 as ZSTD_c_targetCBlockSize,
            ZSTD_c_experimentalParam7 as ZSTD_c_srcSizeHint,
            ZSTD_c_experimentalParam8 as ZSTD_c_enableDedicatedDictSearch,
            ZSTD_c_experimentalParam9 as ZSTD_c_stableInBuffer,
        };

        use zstd_sys::ZSTD_cParameter::*;
        use CParameter::*;

        let (param, value) = match param {
            #[cfg(feature = "experimental")]
            RSyncable(rsyncable) => (ZSTD_c_rsyncable, rsyncable as c_int),
            #[cfg(feature = "experimental")]
            Format(format) => (ZSTD_c_format, format as c_int),
            #[cfg(feature = "experimental")]
            ForceMaxWindow(force) => (ZSTD_c_forceMaxWindow, force as c_int),
            #[cfg(feature = "experimental")]
            ForceAttachDict(force) => (ZSTD_c_forceAttachDict, force as c_int),
            #[cfg(feature = "experimental")]
            TargetCBlockSize(value) => {
                (ZSTD_c_targetCBlockSize, value as c_int)
            }
            #[cfg(feature = "experimental")]
            SrcSizeHint(value) => (ZSTD_c_srcSizeHint, value as c_int),
            #[cfg(feature = "experimental")]
            EnableDedicatedDictSearch(enable) => {
                (ZSTD_c_enableDedicatedDictSearch, enable as c_int)
            }
            #[cfg(feature = "experimental")]
            StableInBuffer(stable) => (ZSTD_c_stableInBuffer, stable as c_int),
            #[cfg(feature = "experimental")]
            StableOutBuffer(stable) => {
                (ZSTD_c_stableOutBuffer, stable as c_int)
            }
            #[cfg(feature = "experimental")]
            BlockDelimiters(value) => (ZSTD_c_blockDelimiters, value as c_int),
            #[cfg(feature = "experimental")]
            ValidateSequences(validate) => {
                (ZSTD_c_validateSequences, validate as c_int)
            }
            #[cfg(feature = "experimental")]
            UseBlockSplitter(split) => {
                (ZSTD_c_useBlockSplitter, split as c_int)
            }
            #[cfg(feature = "experimental")]
            UseRowMatchFinder(mode) => {
                (ZSTD_c_useRowMatchFinder, mode as c_int)
            }
            #[cfg(feature = "experimental")]
            DeterministicRefPrefix(deterministic) => {
                (ZSTD_c_deterministicRefPrefix, deterministic as c_int)
            }
            CompressionLevel(level) => (ZSTD_c_compressionLevel, level),
            WindowLog(value) => (ZSTD_c_windowLog, value as c_int),
            HashLog(value) => (ZSTD_c_hashLog, value as c_int),
            ChainLog(value) => (ZSTD_c_chainLog, value as c_int),
            SearchLog(value) => (ZSTD_c_searchLog, value as c_int),
            MinMatch(value) => (ZSTD_c_minMatch, value as c_int),
            TargetLength(value) => (ZSTD_c_targetLength, value as c_int),
            Strategy(strategy) => (ZSTD_c_strategy, strategy as c_int),
            #[cfg(feature = "experimental")]
            LiteralCompressionMode(mode) => {
                (ZSTD_c_literalCompressionMode, mode as c_int)
            }
            EnableLongDistanceMatching(flag) => {
                (ZSTD_c_enableLongDistanceMatching, flag as c_int)
            }
            LdmHashLog(value) => (ZSTD_c_ldmHashLog, value as c_int),
            LdmMinMatch(value) => (ZSTD_c_ldmMinMatch, value as c_int),
            LdmBucketSizeLog(value) => {
                (ZSTD_c_ldmBucketSizeLog, value as c_int)
            }
            LdmHashRateLog(value) => (ZSTD_c_ldmHashRateLog, value as c_int),
            ContentSizeFlag(flag) => (ZSTD_c_contentSizeFlag, flag as c_int),
            ChecksumFlag(flag) => (ZSTD_c_checksumFlag, flag as c_int),
            DictIdFlag(flag) => (ZSTD_c_dictIDFlag, flag as c_int),

            NbWorkers(value) => (ZSTD_c_nbWorkers, value as c_int),

            JobSize(value) => (ZSTD_c_jobSize, value as c_int),

            OverlapSizeLog(value) => (ZSTD_c_overlapLog, value as c_int),
        };

        parse_code(unsafe {
            zstd_sys::ZSTD_CCtx_setParameter(self.0.as_ptr(), param, value)
        })
    }

    pub fn set_pledged_src_size(
        &mut self,
        pledged_src_size: u64,
    ) -> SafeResult {
        parse_code(unsafe {
            zstd_sys::ZSTD_CCtx_setPledgedSrcSize(
                self.0.as_ptr(),
                pledged_src_size as c_ulonglong,
            )
        })
    }

    /// Creates a copy of this context.
    ///
    /// This only works before any data has been compressed. An error will be
    /// returned otherwise.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    pub fn try_clone(
        &self,
        pledged_src_size: Option<u64>,
    ) -> Result<Self, ErrorCode> {
        let context = NonNull::new(unsafe { zstd_sys::ZSTD_createCCtx() })
            .ok_or(0usize)?;

        parse_code(unsafe {
            zstd_sys::ZSTD_copyCCtx(
                context.as_ptr(),
                self.0.as_ptr(),
                pledged_src_size.unwrap_or(CONTENTSIZE_UNKNOWN),
            )
        })?;

        Ok(CCtx(context, self.1))
    }

    /// Wraps the `ZSTD_getBlockSize()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    pub fn get_block_size(&self) -> usize {
        unsafe { zstd_sys::ZSTD_getBlockSize(self.0.as_ptr()) }
    }

    /// Wraps the `ZSTD_compressBlock()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    pub fn compress_block<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_compressBlock(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                ))
            })
        }
    }
    pub fn in_size() -> usize {
        unsafe { zstd_sys::ZSTD_CStreamInSize() }
    }

    pub fn out_size() -> usize {
        unsafe { zstd_sys::ZSTD_CStreamOutSize() }
    }
}

pub fn create_cctx<'a>() -> CCtx<'a> {
    CCtx::create()
}

impl<'a> Drop for CCtx<'a> {
    fn drop(&mut self) {
        unsafe {
            zstd_sys::ZSTD_freeCCtx(self.0.as_ptr());
        }
    }
}

unsafe impl<'a> Send for CCtx<'a> {}
// CCtx can't be shared across threads, so it does not implement Sync.

unsafe fn c_char_to_str(text: *const c_char) -> &'static str {
    #[cfg(not(feature = "std"))]
    {
        // To be safe, we need to compute right now its length
        let len = libc::strlen(text);

        // Cast it to a slice
        let slice = core::slice::from_raw_parts(text as *mut u8, len);
        // And hope it's still text.
        str::from_utf8(slice).expect("bad error message from zstd")
    }

    #[cfg(feature = "std")]
    {
        std::ffi::CStr::from_ptr(text)
            .to_str()
            .expect("bad error message from zstd")
    }
}

pub fn get_error_name(code: usize) -> &'static str {
    unsafe {
        let name = zstd_sys::ZSTD_getErrorName(code);
        c_char_to_str(name)
    }
}

/// Wraps the `ZSTD_compressCCtx()` function
pub fn compress_cctx(
    ctx: &mut CCtx<'_>,
    dst: &mut [u8],
    src: &[u8],
    compression_level: CompressionLevel,
) -> SafeResult {
    ctx.compress(dst, src, compression_level)
}

/// Wraps the `ZSTD_compress2()` function.
pub fn compress2(
    ctx: &mut CCtx<'_>,
    dst: &mut [u8],
    src: &[u8],
) -> SafeResult {
    ctx.compress2(dst, src)
}

/// A Decompression Context.
///
/// The lifetime references the potential dictionary used for this context.
///
/// If no dictionary was used, it will most likely be `'static`.
///
/// Same as `DStream`.
pub struct DCtx<'a>(NonNull<zstd_sys::ZSTD_DCtx>, PhantomData<&'a ()>);

impl Default for DCtx<'_> {
    fn default() -> Self {
        DCtx::create()
    }
}

impl DCtx<'static> {
    pub fn try_create() -> Option<Self> {
        Some(DCtx(
            NonNull::new(unsafe { zstd_sys::ZSTD_createDCtx() })?,
            PhantomData,
        ))
    }
    pub fn create() -> Self {
        Self::try_create()
            .expect("zstd returned null pointer when creating new context")
    }
}

impl<'a> DCtx<'a> {
    /// Wraps the `ZSTD_decompressDCtx()` function.
    pub fn decompress<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_decompressDCtx(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                ))
            })
        }
    }

    /// Wraps `ZSTD_decompress_usingDict`
    pub fn decompress_using_dict<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
        dict: &[u8],
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_decompress_usingDict(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                    ptr_void(dict),
                    dict.len(),
                ))
            })
        }
    }

    /// Wraps the `ZSTD_decompress_usingDDict()` function.
    pub fn decompress_using_ddict<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
        ddict: &DDict<'_>,
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_decompress_usingDDict(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                    ddict.0.as_ptr(),
                ))
            })
        }
    }

    /// Wraps the `ZSTD_initCStream()` function.
    ///
    /// Initializes an existing `DStream` for decompression.
    pub fn init(&mut self) -> usize {
        unsafe { zstd_sys::ZSTD_initDStream(self.0.as_ptr()) }
    }

    /// Wraps the `ZSTD_initDStream_usingDict()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    #[deprecated]
    pub fn init_using_dict(&mut self, dict: &[u8]) -> SafeResult {
        let code = unsafe {
            zstd_sys::ZSTD_initDStream_usingDict(
                self.0.as_ptr(),
                ptr_void(dict),
                dict.len(),
            )
        };
        parse_code(code)
    }

    /// Wraps the `ZSTD_initDStream_usingDDict()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    #[deprecated]
    pub fn init_using_ddict<'b>(&mut self, ddict: &DDict<'b>) -> SafeResult
    where
        'b: 'a,
    {
        let code = unsafe {
            zstd_sys::ZSTD_initDStream_usingDDict(
                self.0.as_ptr(),
                ddict.0.as_ptr(),
            )
        };
        parse_code(code)
    }

    /// Wraps the `ZSTD_resetDStream()` function.
    pub fn reset(&mut self) -> SafeResult {
        let code = unsafe {
            zstd_sys::ZSTD_DCtx_reset(
                self.0.as_ptr(),
                ResetDirective::ZSTD_reset_session_only,
            )
        };
        parse_code(code)
    }

    pub fn load_dictionary(&mut self, dict: &[u8]) -> SafeResult {
        parse_code(unsafe {
            zstd_sys::ZSTD_DCtx_loadDictionary(
                self.0.as_ptr(),
                ptr_void(dict),
                dict.len(),
            )
        })
    }

    pub fn ref_ddict<'b>(&mut self, ddict: &DDict<'b>) -> SafeResult
    where
        'b: 'a,
    {
        parse_code(unsafe {
            zstd_sys::ZSTD_DCtx_refDDict(self.0.as_ptr(), ddict.0.as_ptr())
        })
    }

    pub fn ref_prefix<'b>(&mut self, prefix: &'b [u8]) -> SafeResult
    where
        'b: 'a,
    {
        parse_code(unsafe {
            zstd_sys::ZSTD_DCtx_refPrefix(
                self.0.as_ptr(),
                ptr_void(prefix),
                prefix.len(),
            )
        })
    }

    pub fn set_parameter(&mut self, param: DParameter) -> SafeResult {
        #[cfg(feature = "experimental")]
        use zstd_sys::ZSTD_dParameter::{
            ZSTD_d_experimentalParam1 as ZSTD_d_format,
            ZSTD_d_experimentalParam2 as ZSTD_d_stableOutBuffer,
            ZSTD_d_experimentalParam3 as ZSTD_d_forceIgnoreChecksum,
            ZSTD_d_experimentalParam4 as ZSTD_d_refMultipleDDicts,
        };

        use zstd_sys::ZSTD_dParameter::*;
        use DParameter::*;

        let (param, value) = match param {
            #[cfg(feature = "experimental")]
            Format(format) => (ZSTD_d_format, format as c_int),
            #[cfg(feature = "experimental")]
            StableOutBuffer(stable) => {
                (ZSTD_d_stableOutBuffer, stable as c_int)
            }
            #[cfg(feature = "experimental")]
            ForceIgnoreChecksum(force) => {
                (ZSTD_d_forceIgnoreChecksum, force as c_int)
            }
            #[cfg(feature = "experimental")]
            RefMultipleDDicts(value) => {
                (ZSTD_d_refMultipleDDicts, value as c_int)
            }

            WindowLogMax(value) => (ZSTD_d_windowLogMax, value as c_int),
        };

        parse_code(unsafe {
            zstd_sys::ZSTD_DCtx_setParameter(self.0.as_ptr(), param, value)
        })
    }

    /// Wraps the `ZSTD_decompressStream()` function.
    pub fn decompress_stream<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
        input: &mut InBuffer<'_>,
    ) -> SafeResult {
        let mut output = output.wrap();
        let mut input = input.wrap();
        let code = unsafe {
            zstd_sys::ZSTD_decompressStream(
                self.0.as_ptr(),
                ptr_mut(&mut output),
                ptr_mut(&mut input),
            )
        };
        parse_code(code)
    }

    /// Wraps the `ZSTD_DStreamInSize()` function.
    ///
    /// Returns a hint for the recommended size of the input buffer for decompression.
    pub fn in_size() -> usize {
        unsafe { zstd_sys::ZSTD_DStreamInSize() }
    }

    /// Wraps the `ZSTD_DStreamOutSize()` function.
    ///
    /// Returns a hint for the recommended size of the output buffer for decompression.
    pub fn out_size() -> usize {
        unsafe { zstd_sys::ZSTD_DStreamOutSize() }
    }

    /// Wraps the `ZSTD_sizeof_DCtx()` function.
    pub fn sizeof(&self) -> usize {
        unsafe { zstd_sys::ZSTD_sizeof_DCtx(self.0.as_ptr()) }
    }

    /// Wraps the `ZSTD_decompressBlock()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    pub fn decompress_block<C: WriteBuf + ?Sized>(
        &mut self,
        dst: &mut C,
        src: &[u8],
    ) -> SafeResult {
        unsafe {
            dst.write_from(|buffer, capacity| {
                parse_code(zstd_sys::ZSTD_decompressBlock(
                    self.0.as_ptr(),
                    buffer,
                    capacity,
                    ptr_void(src),
                    src.len(),
                ))
            })
        }
    }

    /// Wraps the `ZSTD_insertBlock()` function.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    pub fn insert_block(&mut self, block: &[u8]) -> usize {
        unsafe {
            zstd_sys::ZSTD_insertBlock(
                self.0.as_ptr(),
                ptr_void(block),
                block.len(),
            )
        }
    }

    /// Creates a copy of this context.
    ///
    /// This only works before any data has been decompressed. An error will be
    /// returned otherwise.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    pub fn try_clone(&self) -> Result<Self, ErrorCode> {
        let context = NonNull::new(unsafe { zstd_sys::ZSTD_createDCtx() })
            .ok_or(0usize)?;

        unsafe { zstd_sys::ZSTD_copyDCtx(context.as_ptr(), self.0.as_ptr()) };

        Ok(DCtx(context, self.1))
    }
}

/// Prepares a new decompression context without dictionary.
pub fn create_dctx() -> DCtx<'static> {
    DCtx::create()
}

impl Drop for DCtx<'_> {
    fn drop(&mut self) {
        unsafe {
            zstd_sys::ZSTD_freeDCtx(self.0.as_ptr());
        }
    }
}

unsafe impl Send for DCtx<'_> {}
// DCtx can't be shared across threads, so it does not implement Sync.

/// Wraps the `ZSTD_decompressDCtx()` function.
pub fn decompress_dctx(
    ctx: &mut DCtx<'_>,
    dst: &mut [u8],
    src: &[u8],
) -> SafeResult {
    ctx.decompress(dst, src)
}

/// Wraps the `ZSTD_compress_usingDict()` function.
pub fn compress_using_dict(
    ctx: &mut CCtx<'_>,
    dst: &mut [u8],
    src: &[u8],
    dict: &[u8],
    compression_level: CompressionLevel,
) -> SafeResult {
    ctx.compress_using_dict(dst, src, dict, compression_level)
}

/// Wraps the `ZSTD_decompress_usingDict()` function.
pub fn decompress_using_dict(
    dctx: &mut DCtx<'_>,
    dst: &mut [u8],
    src: &[u8],
    dict: &[u8],
) -> SafeResult {
    dctx.decompress_using_dict(dst, src, dict)
}

/// Compression dictionary.
pub struct CDict<'a>(NonNull<zstd_sys::ZSTD_CDict>, PhantomData<&'a ()>);

impl CDict<'static> {
    pub fn create(
        dict_buffer: &[u8],
        compression_level: CompressionLevel,
    ) -> Self {
        Self::try_create(dict_buffer, compression_level)
            .expect("zstd returned null pointer when creating dict")
    }

    pub fn try_create(
        dict_buffer: &[u8],
        compression_level: CompressionLevel,
    ) -> Option<Self> {
        Some(CDict(
            NonNull::new(unsafe {
                zstd_sys::ZSTD_createCDict(
                    ptr_void(dict_buffer),
                    dict_buffer.len(),
                    compression_level,
                )
            })?,
            PhantomData,
        ))
    }
}

impl<'a> CDict<'a> {
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    pub fn create_by_reference(
        dict_buffer: &'a [u8],
        compression_level: CompressionLevel,
    ) -> Self {
        CDict(
            NonNull::new(unsafe {
                zstd_sys::ZSTD_createCDict_byReference(
                    ptr_void(dict_buffer),
                    dict_buffer.len(),
                    compression_level,
                )
            })
            .expect("zstd returned null pointer"),
            PhantomData,
        )
    }

    pub fn sizeof(&self) -> usize {
        unsafe { zstd_sys::ZSTD_sizeof_CDict(self.0.as_ptr()) }
    }

    pub fn get_dict_id(&self) -> u32 {
        unsafe { zstd_sys::ZSTD_getDictID_fromCDict(self.0.as_ptr()) as u32 }
    }
}

/// Wraps the `ZSTD_createCDict()` function.
pub fn create_cdict(
    dict_buffer: &[u8],
    compression_level: CompressionLevel,
) -> CDict<'static> {
    CDict::create(dict_buffer, compression_level)
}

impl<'a> Drop for CDict<'a> {
    fn drop(&mut self) {
        unsafe {
            zstd_sys::ZSTD_freeCDict(self.0.as_ptr());
        }
    }
}

unsafe impl<'a> Send for CDict<'a> {}
unsafe impl<'a> Sync for CDict<'a> {}

/// Wraps the `ZSTD_compress_usingCDict()` function.
pub fn compress_using_cdict(
    cctx: &mut CCtx<'_>,
    dst: &mut [u8],
    src: &[u8],
    cdict: &CDict<'_>,
) -> SafeResult {
    cctx.compress_using_cdict(dst, src, cdict)
}

/// A digested decompression dictionary.
pub struct DDict<'a>(NonNull<zstd_sys::ZSTD_DDict>, PhantomData<&'a ()>);

impl DDict<'static> {
    pub fn create(dict_buffer: &[u8]) -> Self {
        Self::try_create(dict_buffer)
            .expect("zstd returned null pointer when creating dict")
    }

    pub fn try_create(dict_buffer: &[u8]) -> Option<Self> {
        Some(DDict(
            NonNull::new(unsafe {
                zstd_sys::ZSTD_createDDict(
                    ptr_void(dict_buffer),
                    dict_buffer.len(),
                )
            })?,
            PhantomData,
        ))
    }
}

impl<'a> DDict<'a> {
    pub fn sizeof(&self) -> usize {
        unsafe { zstd_sys::ZSTD_sizeof_DDict(self.0.as_ptr()) }
    }

    /// Wraps the `ZSTD_createDDict_byReference()` function.
    ///
    /// The dictionary will keep referencing `dict_buffer`.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    pub fn create_by_reference(dict_buffer: &'a [u8]) -> Self {
        DDict(
            NonNull::new(unsafe {
                zstd_sys::ZSTD_createDDict_byReference(
                    ptr_void(dict_buffer),
                    dict_buffer.len(),
                )
            })
            .expect("zstd returned null pointer"),
            PhantomData,
        )
    }

    pub fn get_dict_id(&self) -> u32 {
        unsafe { zstd_sys::ZSTD_getDictID_fromDDict(self.0.as_ptr()) as u32 }
    }
}

/// Wraps the `ZSTD_createDDict()` function.
///
/// It copies the dictionary internally, so the resulting `DDict` is `'static`.
pub fn create_ddict(dict_buffer: &[u8]) -> DDict<'static> {
    DDict::create(dict_buffer)
}

impl<'a> Drop for DDict<'a> {
    fn drop(&mut self) {
        unsafe {
            zstd_sys::ZSTD_freeDDict(self.0.as_ptr());
        }
    }
}

unsafe impl<'a> Send for DDict<'a> {}
unsafe impl<'a> Sync for DDict<'a> {}

/// Wraps the `ZSTD_decompress_usingDDict()` function.
pub fn decompress_using_ddict(
    dctx: &mut DCtx<'_>,
    dst: &mut [u8],
    src: &[u8],
    ddict: &DDict<'_>,
) -> SafeResult {
    dctx.decompress_using_ddict(dst, src, ddict)
}

/// Compression stream.
///
/// Same as `CCtx`.
pub type CStream<'a> = CCtx<'a>;

// CStream can't be shared across threads, so it does not implement Sync.

/// Allocates a new `CStream`.
pub fn create_cstream<'a>() -> CStream<'a> {
    CCtx::create()
}

/// Prepares an existing `CStream` for compression at the given level.
pub fn init_cstream(
    zcs: &mut CStream<'_>,
    compression_level: CompressionLevel,
) -> usize {
    zcs.init(compression_level)
}

#[derive(Debug)]
/// Wrapper around an input buffer.
///
/// Bytes will be read starting at `src[pos]`.
///
/// `pos` will be updated after reading.
pub struct InBuffer<'a> {
    pub src: &'a [u8],
    pub pos: usize,
}

/// Describe a resizeable bytes container like `Vec<u8>`.
///
/// Can start from uninitialized memory, and will be partially filled.
///
/// Should be implemented by a contiguous chunk of memory.
///
/// The main implementors are:
/// * `Vec<u8>` and similar structures. These can start empty with a non-zero capacity, and they
///   will be resized to cover the data written.
/// * `[u8]` and `[u8; N]`. These must start already-initialized, and will not be resized. It will
///   be up to the caller to only use the part that was written.
pub unsafe trait WriteBuf {
    /// Returns the valid data part of this container. Should only cover initialized data.
    fn as_slice(&self) -> &[u8];

    /// Returns the full capacity of this container. May include uninitialized data.
    fn capacity(&self) -> usize;

    /// Returns a pointer to the start of the data.
    fn as_mut_ptr(&mut self) -> *mut u8;

    /// Indicates that the first `n` bytes of the container have been written.
    unsafe fn filled_until(&mut self, n: usize);

    /// Call the given closure using the pointer and capacity from `self`.
    ///
    /// Assumes the given function returns a parseable code, which if valid, represents how many
    /// bytes were written to `self`.
    ///
    /// The given closure must treat its first argument as pointing to potentially uninitialized
    /// memory, and should not read from it.
    ///
    /// In addition, it must have written at least `n` bytes contiguously from this pointer, where
    /// `n` is the returned value.
    unsafe fn write_from<F>(&mut self, f: F) -> SafeResult
    where
        F: FnOnce(*mut c_void, usize) -> SafeResult,
    {
        let res = f(ptr_mut_void(self), self.capacity());
        if let Ok(n) = res {
            self.filled_until(n);
        }
        res
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "std")))]
unsafe impl WriteBuf for std::vec::Vec<u8> {
    fn as_slice(&self) -> &[u8] {
        &self[..]
    }
    fn capacity(&self) -> usize {
        self.capacity()
    }
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }
    unsafe fn filled_until(&mut self, n: usize) {
        self.set_len(n);
    }
}

#[cfg(feature = "arrays")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "arrays")))]
unsafe impl<const N: usize> WriteBuf for [u8; N] {
    fn as_slice(&self) -> &[u8] {
        self
    }
    fn capacity(&self) -> usize {
        self.len()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        (&mut self[..]).as_mut_ptr()
    }

    unsafe fn filled_until(&mut self, _n: usize) {
        // Assume the slice is already initialized
    }
}

unsafe impl WriteBuf for [u8] {
    fn as_slice(&self) -> &[u8] {
        self
    }
    fn capacity(&self) -> usize {
        self.len()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }

    unsafe fn filled_until(&mut self, _n: usize) {
        // Assume the slice is already initialized
    }
}

/*
// This is possible, but... why?
unsafe impl<'a> WriteBuf for OutBuffer<'a, [u8]> {
    fn as_slice(&self) -> &[u8] {
        self.dst
    }
    fn capacity(&self) -> usize {
        self.dst.len()
    }
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.dst.as_mut_ptr()
    }
    unsafe fn filled_until(&mut self, n: usize) {
        self.pos = n;
    }
}
*/

#[derive(Debug)]
/// Wrapper around an output buffer.
///
/// `C` is usually either `[u8]` or `Vec<u8>`.
///
/// Bytes will be written starting at `dst[pos]`.
///
/// `pos` will be updated after writing.
///
/// # Invariant
///
/// `pos <= dst.capacity()`
pub struct OutBuffer<'a, C: WriteBuf + ?Sized> {
    pub dst: &'a mut C,
    pos: usize,
}

/// Convenience method to get a mut pointer from a mut ref.
fn ptr_mut<B>(ptr_void: &mut B) -> *mut B {
    ptr_void as *mut B
}

/// Interface between a C-level ZSTD_outBuffer and a rust-level `OutBuffer`.
///
/// Will update the parent buffer from the C buffer on drop.
struct OutBufferWrapper<'a, 'b, C: WriteBuf + ?Sized> {
    buf: zstd_sys::ZSTD_outBuffer,
    parent: &'a mut OutBuffer<'b, C>,
}

impl<'a, 'b: 'a, C: WriteBuf + ?Sized> Deref for OutBufferWrapper<'a, 'b, C> {
    type Target = zstd_sys::ZSTD_outBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<'a, 'b: 'a, C: WriteBuf + ?Sized> DerefMut
    for OutBufferWrapper<'a, 'b, C>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

impl<'a, C: WriteBuf + ?Sized> OutBuffer<'a, C> {
    /// Returns a new `OutBuffer` around the given slice.
    ///
    /// Starts with `pos = 0`.
    pub fn around(dst: &'a mut C) -> Self {
        OutBuffer { dst, pos: 0 }
    }

    /// Returns a new `OutBuffer` around the given slice, starting at the given position.
    ///
    /// # Panics
    ///
    /// If `pos >= dst.capacity()`.
    pub fn around_pos(dst: &'a mut C, pos: usize) -> Self {
        if pos >= dst.capacity() {
            panic!("Given position outside of the buffer bounds.");
        }

        OutBuffer { dst, pos }
    }

    /// Returns the current cursor position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets the new cursor position.
    ///
    /// # Panics
    ///
    /// If `pos > self.dst.capacity()`.
    ///
    /// # Safety
    ///
    /// Data up to `pos` must have actually been written to.
    pub unsafe fn set_pos(&mut self, pos: usize) {
        if pos > self.dst.capacity() {
            panic!("Given position outside of the buffer bounds.");
        }

        self.dst.filled_until(pos);

        self.pos = pos;
    }

    fn wrap<'b>(&'b mut self) -> OutBufferWrapper<'b, 'a, C> {
        OutBufferWrapper {
            buf: zstd_sys::ZSTD_outBuffer {
                dst: ptr_mut_void(self.dst),
                size: self.dst.capacity(),
                pos: self.pos,
            },
            parent: self,
        }
    }

    /// Returns the part of this buffer that was written to.
    pub fn as_slice<'b>(&'b self) -> &'a [u8]
    where
        'b: 'a,
    {
        let pos = self.pos;
        &self.dst.as_slice()[..pos]
    }
}

impl<'a, 'b, C: WriteBuf + ?Sized> Drop for OutBufferWrapper<'a, 'b, C> {
    fn drop(&mut self) {
        // Safe because we guarantee that data until `self.buf.pos` has been written.
        unsafe { self.parent.set_pos(self.buf.pos) };
    }
}

struct InBufferWrapper<'a, 'b> {
    buf: zstd_sys::ZSTD_inBuffer,
    parent: &'a mut InBuffer<'b>,
}

impl<'a, 'b: 'a> Deref for InBufferWrapper<'a, 'b> {
    type Target = zstd_sys::ZSTD_inBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<'a, 'b: 'a> DerefMut for InBufferWrapper<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

impl<'a> InBuffer<'a> {
    /// Returns a new `InBuffer` around the given slice.
    ///
    /// Starts with `pos = 0`.
    pub fn around(src: &'a [u8]) -> Self {
        InBuffer { src, pos: 0 }
    }

    /// Returns the current cursor position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets the new cursor position.
    ///
    /// # Panics
    ///
    /// If `pos > self.src.len()`.
    pub fn set_pos(&mut self, pos: usize) {
        if pos > self.src.len() {
            panic!("Given position outside of the buffer bounds.");
        }
        self.pos = pos;
    }

    fn wrap<'b>(&'b mut self) -> InBufferWrapper<'b, 'a> {
        InBufferWrapper {
            buf: zstd_sys::ZSTD_inBuffer {
                src: ptr_void(self.src),
                size: self.src.len(),
                pos: self.pos,
            },
            parent: self,
        }
    }
}

impl<'a, 'b> Drop for InBufferWrapper<'a, 'b> {
    fn drop(&mut self) {
        self.parent.set_pos(self.buf.pos);
    }
}

/// Wraps the `ZSTD_compressStream()` function.
pub fn compress_stream<C: WriteBuf + ?Sized>(
    zcs: &mut CStream<'_>,
    output: &mut OutBuffer<'_, C>,
    input: &mut InBuffer<'_>,
) -> SafeResult {
    zcs.compress_stream(output, input)
}

pub fn compress_stream2<C: WriteBuf + ?Sized>(
    cctx: &mut CCtx<'_>,
    output: &mut OutBuffer<'_, C>,
    input: &mut InBuffer<'_>,
    end_op: zstd_sys::ZSTD_EndDirective,
) -> SafeResult {
    cctx.compress_stream2(output, input, end_op)
}

/// Wraps the `ZSTD_flushStream()` function.
pub fn flush_stream<C: WriteBuf + ?Sized>(
    zcs: &mut CStream<'_>,
    output: &mut OutBuffer<'_, C>,
) -> SafeResult {
    zcs.flush_stream(output)
}

/// Wraps the `ZSTD_endStream()` function.
pub fn end_stream<C: WriteBuf + ?Sized>(
    zcs: &mut CStream<'_>,
    output: &mut OutBuffer<'_, C>,
) -> SafeResult {
    zcs.end_stream(output)
}

/// Wraps `ZSTD_CStreamInSize()`
pub fn cstream_in_size() -> usize {
    CCtx::in_size()
}

/// Wraps `ZSTD_CStreamOutSize()`
pub fn cstream_out_size() -> usize {
    CCtx::out_size()
}

/// A Decompression stream.
///
/// Same as `DCtx`.
pub type DStream<'a> = DCtx<'a>;

pub fn create_dstream() -> DStream<'static> {
    DStream::create()
}

/// Wraps the `ZSTD_initCStream()` function.
///
/// Initializes an existing `DStream` for decompression.
pub fn init_dstream(zds: &mut DStream<'_>) -> usize {
    zds.init()
}

/// Wraps the `ZSTD_decompressStream()` function.
pub fn decompress_stream<C: WriteBuf + ?Sized>(
    zds: &mut DStream<'_>,
    output: &mut OutBuffer<'_, C>,
    input: &mut InBuffer<'_>,
) -> SafeResult {
    zds.decompress_stream(output, input)
}

/// Wraps the `ZSTD_DStreamInSize()` function.
///
/// Returns a hint for the recommended size of the input buffer for decompression.
pub fn dstream_in_size() -> usize {
    DStream::in_size()
}

/// Wraps the `ZSTD_DStreamOutSize()` function.
///
/// Returns a hint for the recommended size of the output buffer for decompression.
pub fn dstream_out_size() -> usize {
    DStream::out_size()
}

/// Wraps the `ZSTD_findFrameCompressedSize()` function.
///
/// `src` should contain at least an entire frame.
pub fn find_frame_compressed_size(src: &[u8]) -> SafeResult {
    let code = unsafe {
        zstd_sys::ZSTD_findFrameCompressedSize(ptr_void(src), src.len())
    };
    parse_code(code)
}

/// Wraps the `ZSTD_getFrameContentSize()` function.
///
/// `src` should contain at least a frame header.
pub fn get_frame_content_size(src: &[u8]) -> u64 {
    unsafe { zstd_sys::ZSTD_getFrameContentSize(ptr_void(src), src.len()) }
}

/// Wraps the `ZSTD_findDecompressedSize()` function.
///
/// `src` should be exactly a sequence of ZSTD frames.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn find_decompressed_size(src: &[u8]) -> u64 {
    unsafe { zstd_sys::ZSTD_findDecompressedSize(ptr_void(src), src.len()) }
}

/// Wraps the `ZSTD_sizeofCCtx()` function.
pub fn sizeof_cctx(cctx: &CCtx<'_>) -> usize {
    cctx.sizeof()
}

/// Wraps the `ZSTD_sizeof_DCtx()` function.
pub fn sizeof_dctx(dctx: &DCtx<'_>) -> usize {
    dctx.sizeof()
}

/// Wraps the `ZSTD_sizeof_CStream()` function.
pub fn sizeof_cstream(zcs: &CStream<'_>) -> usize {
    zcs.sizeof()
}

/// Wraps the `ZSTD_sizeof_DStream()` function.
pub fn sizeof_dstream(zds: &DStream<'_>) -> usize {
    zds.sizeof()
}

/// Wraps the `ZSTD_sizeof_CDict()` function.
pub fn sizeof_cdict(cdict: &CDict<'_>) -> usize {
    cdict.sizeof()
}

/// Wraps the `ZSTD_sizeof_DDict()` function.
pub fn sizeof_ddict(ddict: &DDict<'_>) -> usize {
    ddict.sizeof()
}

/// Wraps the `ZSTD_createCDict_byReference()` function.
///
/// The dictionary will keep referencing `dict_buffer`.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn create_cdict_by_reference<'a>(
    dict_buffer: &'a [u8],
    compression_level: CompressionLevel,
) -> CDict<'a> {
    CDict::create_by_reference(dict_buffer, compression_level)
}

/// Wraps the `ZSTD_isFrame()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn is_frame(buffer: &[u8]) -> u32 {
    unsafe { zstd_sys::ZSTD_isFrame(ptr_void(buffer), buffer.len()) as u32 }
}

/// Wraps the `ZSTD_createDDict_byReference()` function.
///
/// The dictionary will keep referencing `dict_buffer`.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn create_ddict_by_reference(dict_buffer: &[u8]) -> DDict {
    DDict::create_by_reference(dict_buffer)
}

/// Wraps the `ZSTD_getDictID_fromDict()` function.
pub fn get_dict_id_from_dict(dict: &[u8]) -> u32 {
    unsafe {
        zstd_sys::ZSTD_getDictID_fromDict(ptr_void(dict), dict.len()) as u32
    }
}

/// Wraps the `ZSTD_getDictID_fromDDict()` function.
pub fn get_dict_id_from_ddict(ddict: &DDict<'_>) -> u32 {
    ddict.get_dict_id()
}

/// Wraps the `ZSTD_getDictID_fromFrame()` function.
pub fn get_dict_id_from_frame(src: &[u8]) -> u32 {
    unsafe {
        zstd_sys::ZSTD_getDictID_fromFrame(ptr_void(src), src.len()) as u32
    }
}

/// Wraps the `ZSTD_initCStream_srcSize()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[deprecated]
#[allow(deprecated)]
pub fn init_cstream_src_size(
    zcs: &mut CStream,
    compression_level: CompressionLevel,
    pledged_src_size: u64,
) -> usize {
    zcs.init_src_size(compression_level, pledged_src_size)
}

/// Wraps the `ZSTD_initCStream_usingDict()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[deprecated]
#[allow(deprecated)]
pub fn init_cstream_using_dict(
    zcs: &mut CStream,
    dict: &[u8],
    compression_level: CompressionLevel,
) -> SafeResult {
    zcs.init_using_dict(dict, compression_level)
}

/// Wraps the `ZSTD_initCStream_usingCDict()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[deprecated]
#[allow(deprecated)]
pub fn init_cstream_using_cdict<'a, 'b>(
    zcs: &mut CStream<'a>,
    cdict: &CDict<'b>,
) -> SafeResult
where
    'b: 'a, // Dictionary outlives the stream.
{
    zcs.init_using_cdict(cdict)
}

/// Wraps the `ZSTD_CCtx_loadDictionary()` function.
pub fn cctx_load_dictionary(cctx: &mut CCtx<'_>, dict: &[u8]) -> SafeResult {
    cctx.load_dictionary(dict)
}

/// Wraps the `ZSTD_CCtx_refCDict()` function.
///
/// Dictionary must outlive the context.
pub fn cctx_ref_cdict<'a, 'b>(
    cctx: &mut CCtx<'a>,
    cdict: &CDict<'b>,
) -> SafeResult
where
    'b: 'a,
{
    cctx.ref_cdict(cdict)
}

/// Wraps the `ZSTD_CCtx_refPrefix()` function.
///
/// Dictionary must outlive the prefix.
pub fn cctx_ref_prefix<'a, 'b>(
    cctx: &mut CCtx<'a>,
    prefix: &'b [u8],
) -> SafeResult
where
    'b: 'a,
{
    cctx.ref_prefix(prefix)
}

/// Wraps the `ZSTD_DCtx_loadDictionary()` function.
pub fn dctx_load_dictionary(dctx: &mut DCtx<'_>, dict: &[u8]) -> SafeResult {
    dctx.load_dictionary(dict)
}

/// Wraps the `ZSTD_DCtx_refDDict()` function.
pub fn dctx_ref_ddict<'a, 'b>(
    dctx: &mut DCtx<'a>,
    ddict: &'b DDict<'b>,
) -> SafeResult
where
    'b: 'a,
{
    dctx.ref_ddict(ddict)
}

/// Wraps the `ZSTD_DCtx_refPrefix()` function.
pub fn dctx_ref_prefix<'a, 'b>(
    dctx: &mut DCtx<'a>,
    prefix: &'b [u8],
) -> SafeResult
where
    'b: 'a,
{
    dctx.ref_prefix(prefix)
}

/// Wraps the `ZSTD_CCtx_reset()` function.
pub fn cctx_reset(cctx: &mut CCtx<'_>, reset: ResetDirective) -> SafeResult {
    cctx.reset(reset)
}

/// Wraps the `ZSTD_DCtx_reset()` function.
pub fn dctx_reset(dctx: &mut DCtx<'_>, reset: ResetDirective) -> SafeResult {
    parse_code(unsafe { zstd_sys::ZSTD_DCtx_reset(dctx.0.as_ptr(), reset) })
}

/// Wraps the `ZSTD_resetCStream()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[deprecated]
#[allow(deprecated)]
pub fn reset_cstream(zcs: &mut CStream, pledged_src_size: u64) -> SafeResult {
    zcs.reset_cstream(pledged_src_size)
}

/// Wraps the `ZSTD_initDStream_usingDict()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[deprecated]
#[allow(deprecated)]
pub fn init_dstream_using_dict(zds: &mut DStream, dict: &[u8]) -> SafeResult {
    zds.init_using_dict(dict)
}

/// Wraps the `ZSTD_initDStream_usingDDict()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[deprecated]
#[allow(deprecated)]
pub fn init_dstream_using_ddict<'a, 'b>(
    zds: &mut DStream<'a>,
    ddict: &DDict<'b>,
) -> SafeResult
where
    'b: 'a,
{
    zds.init_using_ddict(ddict)
}

/// Wraps the `ZSTD_resetDStream()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn reset_dstream(zds: &mut DStream) -> SafeResult {
    zds.reset()
}

#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FrameFormat {
    /// Regular zstd format.
    One = zstd_sys::ZSTD_format_e::ZSTD_f_zstd1 as u32,

    /// Skip the 4 bytes identifying the content as zstd-compressed data.
    Magicless = zstd_sys::ZSTD_format_e::ZSTD_f_zstd1_magicless as u32,
}

#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DictAttachPref {
    DefaultAttach =
        zstd_sys::ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach as u32,
    ForceAttach = zstd_sys::ZSTD_dictAttachPref_e::ZSTD_dictForceAttach as u32,
    ForceCopy = zstd_sys::ZSTD_dictAttachPref_e::ZSTD_dictForceCopy as u32,
    ForceLoad = zstd_sys::ZSTD_dictAttachPref_e::ZSTD_dictForceLoad as u32,
}

#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ParamSwitch {
    Auto = zstd_sys::ZSTD_paramSwitch_e::ZSTD_ps_auto as u32,
    Enable = zstd_sys::ZSTD_paramSwitch_e::ZSTD_ps_enable as u32,
    Disable = zstd_sys::ZSTD_paramSwitch_e::ZSTD_ps_disable as u32,
}

/// A compression parameter.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CParameter {
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    RSyncable(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    Format(FrameFormat),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    ForceMaxWindow(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    ForceAttachDict(DictAttachPref),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    LiteralCompressionMode(ParamSwitch),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    TargetCBlockSize(u32),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    SrcSizeHint(u32),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    EnableDedicatedDictSearch(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    StableInBuffer(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    StableOutBuffer(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    BlockDelimiters(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    ValidateSequences(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    UseBlockSplitter(ParamSwitch),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    UseRowMatchFinder(ParamSwitch),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    DeterministicRefPrefix(bool),

    CompressionLevel(CompressionLevel),

    WindowLog(u32),

    HashLog(u32),

    ChainLog(u32),

    SearchLog(u32),

    MinMatch(u32),

    TargetLength(u32),

    Strategy(Strategy),

    EnableLongDistanceMatching(bool),

    LdmHashLog(u32),

    LdmMinMatch(u32),

    LdmBucketSizeLog(u32),

    LdmHashRateLog(u32),

    ContentSizeFlag(bool),

    ChecksumFlag(bool),

    DictIdFlag(bool),

    /// Note: this will only work if the `zstdmt` feature is activated.
    NbWorkers(u32),

    JobSize(u32),

    OverlapSizeLog(u32),
}

/// A decompression parameter.
pub enum DParameter {
    WindowLogMax(u32),

    /// See `FrameFormat`.
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    Format(FrameFormat),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    StableOutBuffer(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    ForceIgnoreChecksum(bool),

    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    RefMultipleDDicts(bool),
}

/// Wraps the `ZSTD_DCtx_setParameter()` function.
pub fn dctx_set_parameter(
    dctx: &mut DCtx<'_>,
    param: DParameter,
) -> SafeResult {
    dctx.set_parameter(param)
}

/// Wraps the `ZSTD_CCtx_setParameter()` function.
pub fn cctx_set_parameter(
    cctx: &mut CCtx<'_>,
    param: CParameter,
) -> SafeResult {
    cctx.set_parameter(param)
}

/// Wraps the `ZSTD_CCtx_setPledgedSrcSize()` function.
pub fn cctx_set_pledged_src_size(
    cctx: &mut CCtx<'_>,
    pledged_src_size: u64,
) -> SafeResult {
    cctx.set_pledged_src_size(pledged_src_size)
}

/// Wraps the `ZDICT_trainFromBuffer()` function.
#[cfg(feature = "zdict_builder")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]
pub fn train_from_buffer<C: WriteBuf + ?Sized>(
    dict_buffer: &mut C,
    samples_buffer: &[u8],
    samples_sizes: &[usize],
) -> SafeResult {
    assert_eq!(samples_buffer.len(), samples_sizes.iter().sum());

    unsafe {
        dict_buffer.write_from(|buffer, capacity| {
            parse_code(zstd_sys::ZDICT_trainFromBuffer(
                buffer,
                capacity,
                ptr_void(samples_buffer),
                samples_sizes.as_ptr(),
                samples_sizes.len() as u32,
            ))
        })
    }
}

/// Wraps the `ZSTD_getDictID_fromDict()` function.
#[cfg(feature = "zdict_builder")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]
pub fn get_dict_id(dict_buffer: &[u8]) -> Option<u32> {
    let id = unsafe {
        zstd_sys::ZDICT_getDictID(ptr_void(dict_buffer), dict_buffer.len())
    };
    if id > 0 {
        Some(id)
    } else {
        None
    }
}

/// Wraps the `ZSTD_getBlockSize()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn get_block_size(cctx: &CCtx) -> usize {
    unsafe { zstd_sys::ZSTD_getBlockSize(cctx.0.as_ptr()) }
}

/// Wraps the `ZSTD_compressBlock()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn compress_block(
    cctx: &mut CCtx,
    dst: &mut [u8],
    src: &[u8],
) -> SafeResult {
    cctx.compress_block(dst, src)
}

/// Wraps the `ZSTD_decompressBlock()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn decompress_block(
    dctx: &mut DCtx,
    dst: &mut [u8],
    src: &[u8],
) -> SafeResult {
    dctx.decompress_block(dst, src)
}

/// Wraps the `ZSTD_insertBlock()` function.
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn insert_block(dctx: &mut DCtx, block: &[u8]) -> usize {
    dctx.insert_block(block)
}

/// Wraps the `ZSTD_decompressBound` function
#[cfg(feature = "experimental")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
pub fn decompress_bound(data: &[u8]) -> Result<u64, ErrorCode> {
    let bound =
        unsafe { zstd_sys::ZSTD_decompressBound(ptr_void(data), data.len()) };
    if is_error(bound as usize) {
        Err(bound as usize)
    } else {
        Ok(bound)
    }
}
