use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::offset_of;
use core::{mem, ptr};

use crate::allocator::Allocator;
use crate::compress::compress_block;
use crate::crctable::BZ2_CRC32TABLE;
use crate::debug_log;
use crate::decompress::{self, decompress};
#[cfg(feature = "stdio")]
use crate::libbz2_rs_sys_version;

#[cfg(feature = "stdio")]
pub use crate::high_level::*;

pub(crate) const BZ_MAX_ALPHA_SIZE: usize = 258;
pub(crate) const BZ_MAX_CODE_LEN: usize = 23;

pub(crate) const BZ_N_GROUPS: usize = 6;
pub(crate) const BZ_N_ITERS: usize = 4;

pub(crate) const BZ_G_SIZE: usize = 50;
pub(crate) const BZ_MAX_SELECTORS: u16 = {
    let tmp = 2 + (900000 / BZ_G_SIZE);
    assert!(tmp >> 16 == 0);
    tmp as u16
};

pub(crate) const BZ_RUNA: u16 = 0;
pub(crate) const BZ_RUNB: u16 = 1;

pub(crate) const BZ_MAX_UNUSED_U32: u32 = 5000;

#[cfg(doc)]
use crate::{
    BZ_CONFIG_ERROR, BZ_DATA_ERROR, BZ_DATA_ERROR_MAGIC, BZ_FINISH, BZ_FINISH_OK, BZ_FLUSH,
    BZ_FLUSH_OK, BZ_IO_ERROR, BZ_MEM_ERROR, BZ_OK, BZ_OUTBUFF_FULL, BZ_PARAM_ERROR, BZ_RUN,
    BZ_RUN_OK, BZ_SEQUENCE_ERROR, BZ_STREAM_END, BZ_UNEXPECTED_EOF,
};

#[cfg(feature = "custom-prefix")]
macro_rules! prefix {
    ($name:expr) => {
        concat!(env!("LIBBZ2_RS_SYS_PREFIX"), stringify!($name))
    };
}

// NOTE: once we reach 1.0.0, the macro used for the `semver-prefix` feature should no longer include the
// minor version in the name. The name is meant to be unique between semver-compatible versions!
const _PRE_ONE_DOT_O: () = assert!(env!("CARGO_PKG_VERSION_MAJOR").as_bytes()[0] == b'0');

#[cfg(feature = "semver-prefix")]
macro_rules! prefix {
    ($name:expr) => {
        concat!(
            "LIBBZ2_RS_SYS_v",
            env!("CARGO_PKG_VERSION_MAJOR"),
            ".",
            env!("CARGO_PKG_VERSION_MINOR"),
            ".x_",
            stringify!($name)
        )
    };
}

#[cfg(all(
    not(feature = "custom-prefix"),
    not(feature = "semver-prefix"),
    not(any(test, feature = "testing-prefix"))
))]
macro_rules! prefix {
    ($name:expr) => {
        stringify!($name)
    };
}

#[cfg(all(
    not(feature = "custom-prefix"),
    not(feature = "semver-prefix"),
    any(test, feature = "testing-prefix")
))]
macro_rules! prefix {
    ($name:expr) => {
        concat!("LIBBZ2_RS_SYS_TEST_", stringify!($name))
    };
}

pub(crate) use prefix;

/// The version of the zlib library.
///
/// Its value is a pointer to a NULL-terminated sequence of bytes.
///
/// The version string for this release is `
#[doc = libbz2_rs_sys_version!()]
/// `:
///
/// - The first component is the version of stock zlib that this release is compatible with
/// - The final component is the zlib-rs version used to build this release.
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzlibVersion))]
#[cfg(feature = "stdio")]
pub const extern "C" fn BZ2_bzlibVersion() -> *const core::ffi::c_char {
    const LIBBZ2_RS_SYS_VERSION: &str = concat!(libbz2_rs_sys_version!(), "\0");
    LIBBZ2_RS_SYS_VERSION.as_ptr().cast::<core::ffi::c_char>()
}

type AllocFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int) -> *mut c_void;
type FreeFunc = unsafe extern "C" fn(*mut c_void, *mut c_void) -> ();

/// The current stream state.
///
/// # Custom allocators
///
/// The low-level API supports passing in a custom allocator as part of the [`bz_stream`]:
///
/// ```no_check
/// struct bz_stream {
///     // ...
///     pub bzalloc: Option<unsafe extern "C" fn(_: *mut c_void, _: c_int, _: c_int) -> *mut c_void>,
///     pub bzfree: Option<unsafe extern "C" fn(_: *mut c_void, _: *mut c_void)>,
///     pub opaque: *mut c_void,
/// }
/// ```
///
/// When these fields are `None` (or `NULL` in C), the initialization functions will try to
/// put in a default allocator, based on feature flags:
///
/// - `"rust-allocator"` uses the rust global allocator
/// - `"c-allocator"` uses an allocator based on `malloc` and `free`
///
/// When both configured, `"rust-allocator"` is preferred. When no default allocator is configured,
/// the high-level interface will return a [`BZ_CONFIG_ERROR`]. The low-level interface (the
/// functions that take a [`bz_stream`] as their argument) return a [`BZ_PARAM_ERROR`], unless the
/// user set the `bzalloc` and `bzfree` fields.
///
/// When custom `bzalloc` and `bzfree` functions are given, they must adhere to the following contract
/// to be safe:
///
/// - a call `bzalloc(opaque, n, m)` must return a pointer `p` to `n * m` bytes of memory, or
///   `NULL` if out of memory
/// - a call `bzfree(opaque, p)` must free that memory
///
/// The `strm.opaque` value is passed to as the first argument to all calls to `bzalloc`
/// and `bzfree`, but is otherwise ignored by the library.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *const c_char,
    pub avail_in: c_uint,
    pub total_in_lo32: c_uint,
    pub total_in_hi32: c_uint,
    pub next_out: *mut c_char,
    pub avail_out: c_uint,
    pub total_out_lo32: c_uint,
    pub total_out_hi32: c_uint,
    pub state: *mut c_void,
    pub bzalloc: Option<AllocFunc>,
    pub bzfree: Option<FreeFunc>,
    pub opaque: *mut c_void,
}

pub(crate) use stream::*;
mod stream {
    use super::*;

    #[repr(C)]
    pub(crate) struct BzStream<S: StreamState> {
        pub next_in: *const c_char,
        pub avail_in: c_uint,
        pub total_in_lo32: c_uint,
        pub total_in_hi32: c_uint,
        pub next_out: *mut c_char,
        pub avail_out: c_uint,
        pub total_out_lo32: c_uint,
        pub total_out_hi32: c_uint,
        pub state: *mut S,
        pub bzalloc: Option<AllocFunc>,
        pub bzfree: Option<FreeFunc>,
        pub opaque: *mut c_void,
    }

    macro_rules! check_layout {
    ($($field:ident,)*) => {
        const _: () = {
            $(assert!(offset_of!(bz_stream, $field) == offset_of!(BzStream<DState>, $field));)*
            $(assert!(offset_of!(bz_stream, $field) == offset_of!(BzStream<EState>, $field));)*
        };
    };
}

    check_layout!(
        next_in,
        avail_in,
        total_in_lo32,
        total_in_hi32,
        next_out,
        avail_out,
        total_out_lo32,
        total_out_hi32,
        state,
        bzalloc,
        bzfree,
        opaque,
    );

    pub(crate) trait StreamState {}

    impl StreamState for EState {}
    impl StreamState for DState {}

    impl bz_stream {
        pub const fn zeroed() -> Self {
            Self {
                next_in: ptr::null_mut::<c_char>(),
                avail_in: 0,
                total_in_lo32: 0,
                total_in_hi32: 0,
                next_out: ptr::null_mut::<c_char>(),
                avail_out: 0,
                total_out_lo32: 0,
                total_out_hi32: 0,
                state: ptr::null_mut::<c_void>(),
                bzalloc: None,
                bzfree: None,
                opaque: ptr::null_mut::<c_void>(),
            }
        }
    }

    impl<S: StreamState> BzStream<S> {
        pub(crate) const fn zeroed() -> Self {
            Self {
                next_in: ptr::null_mut::<c_char>(),
                avail_in: 0,
                total_in_lo32: 0,
                total_in_hi32: 0,
                next_out: ptr::null_mut::<c_char>(),
                avail_out: 0,
                total_out_lo32: 0,
                total_out_hi32: 0,
                state: ptr::null_mut::<S>(),
                bzalloc: None,
                bzfree: None,
                opaque: ptr::null_mut::<c_void>(),
            }
        }

        /// # Safety
        ///
        /// The given [`bz_stream`] must either have a NULL state or be initialized with the state
        /// indicated by the generic param `S`. It must also have `bzalloc`/`bzfree`/`opaque` correctly
        /// configured.
        pub(crate) unsafe fn from_mut(s: &mut bz_stream) -> &mut Self {
            unsafe { mem::transmute(s) }
        }

        /// # Safety
        ///
        /// The given [`bz_stream`] must be initialized and either have a NULL state or be initialized
        /// with the state indicated by the generic param `S`. It must also have
        /// `bzalloc`/`bzfree`/`opaque` correctly configured.
        pub(crate) unsafe fn from_ptr<'a>(p: *mut bz_stream) -> Option<&'a mut Self> {
            unsafe { p.cast::<Self>().as_mut() }
        }

        pub(super) fn allocator(&self) -> Option<Allocator> {
            unsafe { Allocator::from_bz_stream(self) }
        }

        /// Read up to 7 bytes into the bit buffer.
        ///
        /// The caller is responsible for updating `self.total_in`!
        #[must_use]
        #[inline(always)]
        pub(crate) fn pull_u64(
            &mut self,
            mut bit_buffer: u64,
            bits_used: i32,
        ) -> Option<(u64, i32)> {
            // we should only ask for more input if there are at least 8 free bits
            debug_assert!(bits_used <= 56);

            if self.avail_in < 8 {
                return None;
            }

            // of course this uses big endian values
            let read = u64::from_be_bytes(unsafe { self.next_in.cast::<[u8; 8]>().read() });

            // because of the endianness, we can only shift in whole bytes.
            // this calculates the number of available bits, rounded down to the nearest multiple
            // of 8.
            let increment_bits = (63 - bits_used) & !7;

            // shift existing bits to the end, and or new bits in
            bit_buffer = (bit_buffer << increment_bits) | (read >> (64 - increment_bits));

            // we read 8 bytes above, but can only process `increment_bytes` worth of bits
            let increment_bytes = increment_bits / 8;
            self.next_in = unsafe { (self.next_in).add(increment_bytes as usize) };
            self.avail_in -= increment_bytes as u32;

            // skips updating `self.total_in`: the caller is responsible for keeping it updated

            Some((bit_buffer, bits_used + increment_bits))
        }

        /// Read exactly 1 byte into the buffer
        ///
        /// The caller is responsible for updating `self.total_in`!
        #[must_use]
        #[inline(always)]
        pub(crate) fn pull_u8(
            &mut self,
            mut bit_buffer: u64,
            bits_used: i32,
        ) -> Option<(u64, i32)> {
            // we should only ask for more input if there are at least 8 free bits
            debug_assert!(bits_used <= 56);

            if self.avail_in == 0 || bits_used > 56 {
                return None;
            }

            let read = unsafe { *(self.next_in as *mut u8) };
            bit_buffer <<= 8;
            bit_buffer |= u64::from(read);

            self.next_in = unsafe { (self.next_in).offset(1) };
            self.avail_in -= 1;

            // skips updating `self.total_in`: the caller is responsible for keeping it updated

            Some((bit_buffer, bits_used + 8))
        }

        #[must_use]
        pub(crate) fn read_byte(&mut self) -> Option<u8> {
            if self.avail_in == 0 {
                return None;
            }
            let b = unsafe { *(self.next_in as *mut u8) };
            self.next_in = unsafe { (self.next_in).offset(1) };
            self.avail_in -= 1;
            self.total_in_lo32 = (self.total_in_lo32).wrapping_add(1);
            if self.total_in_lo32 == 0 {
                self.total_in_hi32 = (self.total_in_hi32).wrapping_add(1);
            }
            Some(b)
        }

        #[must_use]
        pub(super) fn write_byte(&mut self, byte: u8) -> bool {
            if self.avail_out == 0 {
                return false;
            }
            unsafe {
                *self.next_out = byte as c_char;
            }
            self.avail_out -= 1;
            self.next_out = unsafe { (self.next_out).offset(1) };
            self.total_out_lo32 = (self.total_out_lo32).wrapping_add(1);
            if self.total_out_lo32 == 0 {
                self.total_out_hi32 = (self.total_out_hi32).wrapping_add(1);
            }
            true
        }
    }

    pub(super) fn configure_allocator<S: StreamState>(strm: &mut BzStream<S>) -> Option<Allocator> {
        match (strm.bzalloc, strm.bzfree) {
            (Some(allocate), Some(deallocate)) => {
                Some(Allocator::custom(allocate, deallocate, strm.opaque))
            }
            (None, None) => {
                let allocator = Allocator::DEFAULT?;
                let (bzalloc, bzfree) = Allocator::default_function_pointers()?;

                strm.bzalloc = Some(bzalloc);
                strm.bzfree = Some(bzfree);

                Some(allocator)
            }
            // Using a different allocator for alloc and free is UB. The user of libbzip2-rs can't get a
            // reference to the default alloc or free function, so hitting this path means that using
            // the default alloc or free function would cause two allocators to be mixed. As such return
            // an error to prevent UB.
            #[cfg(any(feature = "rust-allocator", not(feature = "c-allocator")))]
            _ => None,

            #[cfg(all(feature = "c-allocator", not(feature = "rust-allocator")))]
            _ => {
                // this is almost certainly a bug, but replicates the original C behavior.
                //
                // Note that this logic does not really work with the default rust allocator, because
                // it will panic at runtime when called directly. Usually the idea here is that
                // allocation is special, and free is just the default `libc::free` that we configure
                // by default with the default C allocator.
                let (default_bzalloc, default_bzfree) = crate::allocator::c_allocator::ALLOCATOR;

                let bzalloc = strm.bzalloc.get_or_insert(default_bzalloc);
                let bzfree = strm.bzfree.get_or_insert(default_bzfree);

                Some(Allocator::custom(*bzalloc, *bzfree, strm.opaque))
            }
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub(crate) enum ReturnCode {
    BZ_OK = 0,
    BZ_RUN_OK = 1,
    BZ_FLUSH_OK = 2,
    BZ_FINISH_OK = 3,
    BZ_STREAM_END = 4,
    BZ_SEQUENCE_ERROR = -1,
    BZ_PARAM_ERROR = -2,
    BZ_MEM_ERROR = -3,
    BZ_DATA_ERROR = -4,
    BZ_DATA_ERROR_MAGIC = -5,
    BZ_IO_ERROR = -6,
    BZ_UNEXPECTED_EOF = -7,
    BZ_OUTBUFF_FULL = -8,
    BZ_CONFIG_ERROR = -9,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum Mode {
    Idle,
    Running,
    Flushing,
    Finishing,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum State {
    Output,
    Input,
}

pub(crate) const BZ_N_RADIX: i32 = 2;
pub(crate) const BZ_N_QSORT: i32 = 12;
pub(crate) const BZ_N_SHELL: i32 = 18;
pub(crate) const BZ_N_OVERSHOOT: usize = (BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2) as usize;

pub(crate) const FTAB_LEN: usize = u16::MAX as usize + 2;

pub(crate) struct EState {
    pub strm_addr: usize, // Only for a consistency check
    pub mode: Mode,
    pub state: State,
    pub avail_in_expect: u32,
    pub arr1: Arr1,
    pub arr2: Arr2,
    pub ftab: Ftab,
    pub origPtr: i32,
    pub writer: crate::compress::EWriter,
    pub workFactor: i32,
    pub state_in_ch: u32,
    pub state_in_len: i32,
    pub nblock: i32,
    pub nblockMAX: i32,
    pub state_out_pos: i32,
    pub nInUse: i32,
    pub inUse: [bool; 256],
    pub unseqToSeq: [u8; 256],
    pub blockCRC: u32,
    pub combinedCRC: u32,
    pub verbosity: i32,
    pub blockNo: i32,
    pub blockSize100k: i32,
    pub nMTF: i32,
    pub mtfFreq: [i32; 258],
    pub selector: [u8; 18002],
    pub selectorMtf: [u8; 18002],
    pub len: [[u8; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    pub code: [[u32; 258]; 6],
    pub rfreq: [[i32; 258]; 6],
    pub len_pack: [[u32; 4]; 258],
}

/// Creates a new pointer that is dangling, but well-aligned.
pub(crate) fn dangling<T>() -> *mut T {
    ptr::null_mut::<T>().wrapping_add(mem::align_of::<T>())
}

pub(crate) struct Arr1 {
    ptr: *mut u32,
    len: usize,
}

impl Arr1 {
    fn alloc(allocator: &Allocator, len: usize) -> Option<Self> {
        let ptr = allocator.allocate_zeroed(len)?;
        Some(Self { ptr, len })
    }

    unsafe fn dealloc(&mut self, allocator: &Allocator) {
        let this = mem::replace(
            self,
            Self {
                ptr: dangling(),
                len: 0,
            },
        );
        if this.len != 0 {
            unsafe { allocator.deallocate(this.ptr, this.len) }
        }
    }

    pub(crate) fn mtfv(&mut self) -> &mut [u16] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.cast(), self.len * 2) }
    }

    pub(crate) fn ptr(&mut self) -> &mut [u32] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

pub(crate) struct Arr2 {
    ptr: *mut u32,
    len: usize,
}

impl Arr2 {
    fn alloc(allocator: &Allocator, len: usize) -> Option<Self> {
        let ptr = allocator.allocate_zeroed(len)?;
        Some(Self { ptr, len })
    }

    unsafe fn dealloc(&mut self, allocator: &Allocator) {
        let this = mem::replace(
            self,
            Self {
                ptr: dangling(),
                len: 0,
            },
        );
        if this.len != 0 {
            unsafe { allocator.deallocate(this.ptr, this.len) }
        }
    }

    pub(crate) fn eclass(&mut self) -> &mut [u32] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    pub(crate) fn zbits(&mut self, nblock: usize) -> &mut [u8] {
        assert!(nblock <= 4 * self.len);
        unsafe {
            core::slice::from_raw_parts_mut(
                self.ptr.cast::<u8>().add(nblock),
                self.len * 4 - nblock,
            )
        }
    }

    pub(crate) fn raw_block(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.cast(), self.len * 4) }
    }

    pub(crate) fn block(&mut self, nblock: usize) -> &mut [u8] {
        assert!(nblock <= 4 * self.len);
        unsafe { core::slice::from_raw_parts_mut(self.ptr.cast(), nblock) }
    }

    pub(crate) fn block_and_quadrant(&mut self, nblock: usize) -> (&mut [u8], &mut [u16]) {
        let len = nblock + BZ_N_OVERSHOOT;
        assert!(3 * len.next_multiple_of(2) <= 4 * self.len);

        let block = unsafe { core::slice::from_raw_parts_mut(self.ptr.cast(), len) };

        let start_byte = len.next_multiple_of(2);
        let quadrant: *mut u16 = unsafe { self.ptr.cast::<u16>().byte_add(start_byte) };
        let quadrant = unsafe { core::slice::from_raw_parts_mut(quadrant, len) };
        quadrant.fill(0);

        (block, quadrant)
    }
}

pub(crate) struct Ftab {
    ptr: *mut u32,
}

impl Ftab {
    fn alloc(allocator: &Allocator) -> Option<Self> {
        let ptr = allocator.allocate_zeroed(FTAB_LEN)?;
        Some(Self { ptr })
    }

    unsafe fn dealloc(&mut self, allocator: &Allocator) {
        let this = mem::replace(
            self,
            Self {
                ptr: ptr::null_mut(),
            },
        );
        if !this.ptr.is_null() {
            unsafe { allocator.deallocate(this.ptr, FTAB_LEN) }
        }
    }

    pub(crate) fn ftab(&mut self) -> &mut [u32; FTAB_LEN] {
        // NOTE: this panics if the pointer is NULL, that is important!
        unsafe { self.ptr.cast::<[u32; FTAB_LEN]>().as_mut().unwrap() }
    }
}

#[repr(C)]
pub(crate) struct DState {
    pub strm_addr: usize, // Only for a consistency check
    pub state: decompress::State,
    pub state_out_len: u32,
    pub state_out_ch: u8,
    pub blockRandomised: bool,
    pub blockSize100k: u8,
    pub k0: u8,
    pub bsBuff: u64,
    pub bsLive: i32,
    pub rNToGo: u16,
    pub rTPos: u16,
    pub smallDecompress: DecompressMode,
    pub currBlockNo: i32,
    pub verbosity: i32,
    pub origPtr: i32,
    pub tPos: u32,
    pub nblock_used: i32,
    pub unzftab: [u32; 256],
    pub cftab: [u32; 257],
    pub cftabCopy: [u32; 257],
    pub tt: DSlice<u32>,
    pub ll16: DSlice<u16>,
    pub ll4: DSlice<u8>,
    pub storedBlockCRC: u32,
    pub storedCombinedCRC: u32,
    pub calculatedBlockCRC: u32,
    pub calculatedCombinedCRC: u32,
    pub nInUse: u16,
    pub inUse: [bool; 256],
    pub inUse16: [bool; 16],
    pub seqToUnseq: [u8; 256],
    pub mtfa: [u8; 4096],
    pub mtfbase: [u16; 16],
    pub selector: [u8; 18002],
    pub selectorMtf: [u8; 18002],
    pub len: [[u8; 258]; 6],
    pub limit: [[i32; 258]; 6],
    pub base: [[i32; 258]; 6],
    pub perm: [[u16; 258]; 6],
    pub minLens: [u8; 6],
    pub save: SaveArea,
}

#[derive(Default)]
#[repr(C)]
pub(crate) struct SaveArea {
    pub i: i32,
    pub j: i32,
    pub alphaSize: u16,
    pub EOB: u16,
    pub groupNo: i32,
    pub nblock: u32,
    pub es: u32,
    pub zvec: i32,
    pub nextSym: u16,
    pub nSelectors: u16,
    pub groupPos: u8,
    pub zn: u8,
    pub nGroups: u8,
    pub t: u8,
    pub curr: u8,
    pub nblockMAX100k: u8,
    pub logN: u8, // the log_2 of N
    pub zj: bool,
    pub gMinlen: u8,
    pub gSel: u8,
}

pub(crate) struct DSlice<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> DSlice<T> {
    fn new() -> Self {
        Self {
            ptr: dangling(),
            len: 0,
        }
    }

    pub(crate) fn alloc(allocator: &Allocator, len: usize) -> Option<Self> {
        let ptr = allocator.allocate_zeroed::<T>(len)?;
        Some(Self { ptr, len })
    }

    pub(crate) unsafe fn dealloc(&mut self, allocator: &Allocator) {
        let this = mem::replace(self, Self::new());
        if this.len != 0 {
            unsafe { allocator.deallocate(this.ptr, this.len) }
        }
    }

    pub(crate) fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

const _C_INT_SIZE: () = assert!(core::mem::size_of::<core::ffi::c_int>() == 4);
const _C_SHORT_SIZE: () = assert!(core::mem::size_of::<core::ffi::c_short>() == 2);
const _C_CHAR_SIZE: () = assert!(core::mem::size_of::<core::ffi::c_char>() == 1);

fn prepare_new_block(s: &mut EState) {
    s.nblock = 0;
    s.writer.num_z = 0;
    s.state_out_pos = 0;
    s.blockCRC = 0xffffffff;
    s.inUse.fill(false);
    s.blockNo += 1;
}

fn init_rl(s: &mut EState) {
    s.state_in_ch = 256 as c_int as u32;
    s.state_in_len = 0 as c_int;
}

fn isempty_rl(s: &mut EState) -> bool {
    !(s.state_in_ch < 256 && s.state_in_len > 0)
}

/// Prepares the stream for compression.
///
/// # Returns
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `strm.is_null()`
///     - `!(1..=9).contains(&blockSize100k)`
///     - `!(0..=4).contains(&verbosity)`
///     - `!(0..=250).contains(&workFactor)`
///     - no [valid allocator](bz_stream#custom-allocators) could be configured
/// - [`BZ_MEM_ERROR`] if insufficient memory is available
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `strm` is `NULL`
///     - `strm` satisfies the requirements of `&mut *strm`
/// * The `bzalloc`, `bzfree` and `opaque` fields form a [valid allocator](bz_stream#custom-allocators).
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzCompressInit))]
pub unsafe extern "C" fn BZ2_bzCompressInit(
    strm: *mut bz_stream,
    blockSize100k: c_int,
    verbosity: c_int,
    workFactor: c_int,
) -> c_int {
    let Some(strm) = (unsafe { BzStream::from_ptr(strm) }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };
    BZ2_bzCompressInitHelp(strm, blockSize100k, verbosity, workFactor) as c_int
}

pub(crate) fn BZ2_bzCompressInitHelp(
    strm: &mut BzStream<EState>,
    blockSize100k: c_int,
    verbosity: c_int,
    mut workFactor: c_int,
) -> ReturnCode {
    if !(1..=9).contains(&blockSize100k) || !(0..=250).contains(&workFactor) {
        return ReturnCode::BZ_PARAM_ERROR;
    }

    if workFactor == 0 {
        workFactor = 30;
    }

    // return a param error when no [valid allocator](bz_stream#custom-allocators) could be configured
    let Some(allocator) = configure_allocator(strm) else {
        return ReturnCode::BZ_PARAM_ERROR;
    };

    let Some(s) = allocator.allocate_zeroed::<EState>(1) else {
        return ReturnCode::BZ_MEM_ERROR;
    };

    // this `s.strm` pointer should _NEVER_ be used! it exists just as a consistency check to ensure
    // that a given state belongs to a given strm.
    unsafe { (*s).strm_addr = strm as *const _ as usize }; // FIXME use .addr() once stable

    let n = 100000 * blockSize100k;

    let arr1_len = n as usize;
    let arr1 = Arr1::alloc(&allocator, arr1_len);

    let arr2_len = n as usize + (2 + 12 + 18 + 2);
    let arr2 = Arr2::alloc(&allocator, arr2_len);

    let ftab = Ftab::alloc(&allocator);

    match (arr1, arr2, ftab) {
        (Some(arr1), Some(arr2), Some(ftab)) => unsafe {
            (*s).arr1 = arr1;
            (*s).arr2 = arr2;
            (*s).ftab = ftab;
        },
        (arr1, arr2, ftab) => {
            if let Some(mut arr1) = arr1 {
                unsafe { arr1.dealloc(&allocator) };
            }

            if let Some(mut arr2) = arr2 {
                unsafe { arr2.dealloc(&allocator) };
            }

            if let Some(mut ftab) = ftab {
                unsafe { ftab.dealloc(&allocator) };
            }

            unsafe { allocator.deallocate(s, 1) };

            return ReturnCode::BZ_MEM_ERROR;
        }
    };

    strm.state = s;

    // safety: the EState has now been sufficiently initialized; the allocator zeroes the memory,
    // and the only fields where zero is not a valid value are the arrays that were just set
    //
    // note in particular that if the discriminant of the first variant of an enum is unspecified,
    // then it is set to zero.
    let s = unsafe { &mut *s };

    s.blockNo = 0;
    s.state = State::Output;
    s.mode = Mode::Running;
    s.combinedCRC = 0;
    s.blockSize100k = blockSize100k;
    s.nblockMAX = 100000 * blockSize100k - 19;
    s.verbosity = verbosity;
    s.workFactor = workFactor;

    strm.total_in_lo32 = 0;
    strm.total_in_hi32 = 0;
    strm.total_out_lo32 = 0;
    strm.total_out_hi32 = 0;

    init_rl(s);
    prepare_new_block(s);

    ReturnCode::BZ_OK
}

macro_rules! BZ_UPDATE_CRC {
    ($crcVar:expr, $cha:expr) => {
        let index = ($crcVar >> 24) ^ ($cha as core::ffi::c_uint);
        $crcVar = ($crcVar << 8) ^ BZ2_CRC32TABLE[index as usize];
    };
}

fn add_pair_to_block(s: &mut EState) {
    let ch: u8 = s.state_in_ch as u8;

    for _ in 0..s.state_in_len {
        BZ_UPDATE_CRC!(s.blockCRC, ch);
    }

    let block = s.arr2.raw_block();
    s.inUse[s.state_in_ch as usize] = true;
    match s.state_in_len {
        1 => {
            block[s.nblock as usize..][..1].fill(ch);
            s.nblock += 1;
        }
        2 => {
            block[s.nblock as usize..][..2].fill(ch);
            s.nblock += 2;
        }
        3 => {
            block[s.nblock as usize..][..3].fill(ch);
            s.nblock += 3;
        }
        _ => {
            s.inUse[(s.state_in_len - 4) as usize] = true;

            block[s.nblock as usize..][..4].fill(ch);
            s.nblock += 4;

            block[s.nblock as usize] = (s.state_in_len - 4) as u8;
            s.nblock += 1;
        }
    };
}

fn flush_rl(s: &mut EState) {
    if s.state_in_ch < 256 {
        add_pair_to_block(s);
    }
    init_rl(s);
}

macro_rules! ADD_CHAR_TO_BLOCK {
    ($zs:expr, $zchh0:expr) => {
        let zchh: u32 = $zchh0 as u32;

        if zchh != $zs.state_in_ch && $zs.state_in_len == 1 {
            /*-- fast track the common case --*/

            let ch: u8 = $zs.state_in_ch as u8;
            BZ_UPDATE_CRC!($zs.blockCRC, ch);
            $zs.inUse[$zs.state_in_ch as usize] = true;
            $zs.arr2.raw_block()[$zs.nblock as usize] = ch;
            $zs.nblock += 1;
            $zs.nblock;
            $zs.state_in_ch = zchh;
        } else if zchh != $zs.state_in_ch || $zs.state_in_len == 255 {
            /*-- general, uncommon cases --*/

            if $zs.state_in_ch < 256 {
                add_pair_to_block($zs);
            }
            $zs.state_in_ch = zchh;
            $zs.state_in_len = 1;
        } else {
            $zs.state_in_len += 1;
        }
    };
}

fn copy_input_until_stop(strm: &mut BzStream<EState>, s: &mut EState) -> bool {
    let mut progress_in = false;

    match s.mode {
        Mode::Running => loop {
            if s.nblock >= s.nblockMAX {
                break;
            }
            if let Some(b) = strm.read_byte() {
                progress_in = true;
                ADD_CHAR_TO_BLOCK!(s, b as u32);
            } else {
                break;
            }
        },
        Mode::Idle | Mode::Flushing | Mode::Finishing => loop {
            if s.nblock >= s.nblockMAX {
                break;
            }
            if s.avail_in_expect == 0 {
                break;
            }
            if let Some(b) = strm.read_byte() {
                progress_in = true;
                ADD_CHAR_TO_BLOCK!(s, b as u32);
            } else {
                break;
            }
            s.avail_in_expect -= 1;
        },
    }
    progress_in
}

fn copy_output_until_stop(strm: &mut BzStream<EState>, s: &mut EState) -> bool {
    let mut progress_out = false;

    let zbits = &mut s.arr2.raw_block()[s.nblock as usize..];

    loop {
        if s.state_out_pos >= s.writer.num_z as i32 {
            break;
        }
        if !strm.write_byte(zbits[s.state_out_pos as usize]) {
            break;
        }
        progress_out = true;
        s.state_out_pos += 1;
    }
    progress_out
}

fn handle_compress(strm: &mut BzStream<EState>, s: &mut EState) -> bool {
    let mut progress_in = false;
    let mut progress_out = false;

    loop {
        if let State::Input = s.state {
            progress_out |= copy_output_until_stop(strm, s);
            if s.state_out_pos < s.writer.num_z as i32 {
                break;
            }
            if matches!(s.mode, Mode::Finishing) && s.avail_in_expect == 0 && isempty_rl(s) {
                break;
            }
            prepare_new_block(s);
            s.state = State::Output;
            if matches!(s.mode, Mode::Flushing) && s.avail_in_expect == 0 && isempty_rl(s) {
                break;
            }
        }
        if let State::Input = s.state {
            continue;
        }
        progress_in |= copy_input_until_stop(strm, s);
        if !matches!(s.mode, Mode::Running) && s.avail_in_expect == 0 {
            flush_rl(s);
            let is_last_block = matches!(s.mode, Mode::Finishing);
            compress_block(s, is_last_block);
            s.state = State::Input;
        } else if s.nblock >= s.nblockMAX {
            compress_block(s, false);
            s.state = State::Input;
        } else if strm.avail_in == 0 {
            break;
        }
    }

    progress_in || progress_out
}

pub(crate) enum Action {
    Run = 0,
    Flush = 1,
    Finish = 2,
}

impl TryFrom<i32> for Action {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Run),
            1 => Ok(Self::Flush),
            2 => Ok(Self::Finish),
            _ => Err(()),
        }
    }
}

/// Compresses as much data as possible, and stops when the input buffer becomes empty or the output buffer becomes full.
///
/// # Returns
///
/// - [`BZ_SEQUENCE_ERROR`] if called on an invalid stream, e.g.
///     - before [`BZ2_bzCompressInit`]
///     - after [`BZ2_bzCompressEnd`]
/// - [`BZ_PARAM_ERROR`] if any of
///     - `strm.is_null()`
///     - `strm.s.is_null()`
///     - action is not one of [`BZ_RUN`], [`BZ_FLUSH`] or [`BZ_FINISH`]
/// - [`BZ_RUN_OK`] successfully compressed, but ran out of input or output space
/// - [`BZ_FLUSH_OK`] not all compressed data has been written to the output yet
/// - [`BZ_FINISH_OK`] if all input has been read but not all output has been written to the output
///   buffer yet
/// - [`BZ_STREAM_END`] if all input has been read all output has been written to the output buffer
///
/// # Safety
///
/// * Either
///     - `strm` is `NULL`
///     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`BZ2_bzCompressInit`]
/// * Either
///     - `strm.next_in` is `NULL` and `strm.avail_in` is 0
///     - `strm.next_in` is readable for `strm.avail_in` bytes
/// * Either
///     - `strm.next_out` is `NULL` and `strm.avail_out` is `0`
///     - `strm.next_out` is writable for `strm.avail_out` bytes
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzCompress))]
pub unsafe extern "C" fn BZ2_bzCompress(strm: *mut bz_stream, action: c_int) -> c_int {
    let Some(strm) = (unsafe { BzStream::from_ptr(strm) }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };

    BZ2_bzCompressHelp(strm, action) as c_int
}

pub(crate) fn BZ2_bzCompressHelp(strm: &mut BzStream<EState>, action: i32) -> ReturnCode {
    let Some(s) = (unsafe { strm.state.as_mut() }) else {
        return ReturnCode::BZ_PARAM_ERROR;
    };

    // FIXME use .addr() once stable
    if s.strm_addr != strm as *mut _ as usize {
        return ReturnCode::BZ_PARAM_ERROR;
    }

    compress_loop(strm, s, action)
}

fn compress_loop(strm: &mut BzStream<EState>, s: &mut EState, action: i32) -> ReturnCode {
    loop {
        match s.mode {
            Mode::Idle => return ReturnCode::BZ_SEQUENCE_ERROR,
            Mode::Running => match Action::try_from(action) {
                Ok(Action::Run) => {
                    let progress = handle_compress(strm, s);
                    return if progress {
                        ReturnCode::BZ_RUN_OK
                    } else {
                        ReturnCode::BZ_PARAM_ERROR
                    };
                }
                Ok(Action::Flush) => {
                    s.avail_in_expect = strm.avail_in;
                    s.mode = Mode::Flushing;
                }
                Ok(Action::Finish) => {
                    s.avail_in_expect = strm.avail_in;
                    s.mode = Mode::Finishing;
                }
                Err(()) => {
                    return ReturnCode::BZ_PARAM_ERROR;
                }
            },
            Mode::Flushing => {
                let Ok(Action::Flush) = Action::try_from(action) else {
                    return ReturnCode::BZ_SEQUENCE_ERROR;
                };
                if s.avail_in_expect != strm.avail_in {
                    return ReturnCode::BZ_SEQUENCE_ERROR;
                }
                handle_compress(strm, s);
                if s.avail_in_expect > 0
                    || !isempty_rl(s)
                    || s.state_out_pos < s.writer.num_z as i32
                {
                    return ReturnCode::BZ_FLUSH_OK;
                }
                s.mode = Mode::Running;
                return ReturnCode::BZ_RUN_OK;
            }
            Mode::Finishing => {
                let Ok(Action::Finish) = Action::try_from(action) else {
                    // unreachable in practice
                    return ReturnCode::BZ_SEQUENCE_ERROR;
                };
                if s.avail_in_expect != strm.avail_in {
                    // unreachable in practice
                    return ReturnCode::BZ_SEQUENCE_ERROR;
                }
                let progress = handle_compress(strm, s);
                if !progress {
                    return ReturnCode::BZ_SEQUENCE_ERROR;
                }
                if s.avail_in_expect > 0
                    || !isempty_rl(s)
                    || s.state_out_pos < s.writer.num_z as i32
                {
                    return ReturnCode::BZ_FINISH_OK;
                }
                s.mode = Mode::Idle;
                return ReturnCode::BZ_STREAM_END;
            }
        }
    }
}

/// Deallocates all dynamically allocated data structures for this stream.
///
/// # Returns
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `strm.is_null()`
///     - `strm.s.is_null()`
///     - no [valid allocator](bz_stream#custom-allocators) could be configured
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// * Either
///     - `strm` is `NULL`
///     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`BZ2_bzCompressInit`]
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzCompressEnd))]
pub unsafe extern "C" fn BZ2_bzCompressEnd(strm: *mut bz_stream) -> c_int {
    let Some(strm) = (unsafe { BzStream::from_ptr(strm) }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };
    BZ2_bzCompressEndHelp(strm)
}

fn BZ2_bzCompressEndHelp(strm: &mut BzStream<EState>) -> c_int {
    let Some(s) = (unsafe { strm.state.as_mut() }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };

    // FIXME use .addr() once stable
    if s.strm_addr != strm as *mut _ as usize {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    }

    let Some(allocator) = strm.allocator() else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };

    unsafe {
        s.arr1.dealloc(&allocator);
        s.arr2.dealloc(&allocator);
        s.ftab.dealloc(&allocator);
    }

    unsafe {
        allocator.deallocate(strm.state.cast::<EState>(), 1);
    }
    strm.state = ptr::null_mut::<EState>();

    ReturnCode::BZ_OK as c_int
}

pub(crate) enum DecompressMode {
    Small,
    Fast,
}

/// Prepares the stream for decompression.
///
/// # Returns
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `strm.is_null()`
///     - `!(0..=1).contains(&small)`
///     - `!(0..=4).contains(&verbosity)`
///     - no [valid allocator](bz_stream#custom-allocators) could be configured
/// - [`BZ_MEM_ERROR`] if insufficient memory is available
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `strm` is `NULL`
///     - `strm` satisfies the requirements of `&mut *strm`
/// * The `bzalloc`, `bzfree` and `opaque` fields form a [valid allocator](bz_stream#custom-allocators).
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzDecompressInit))]
pub unsafe extern "C" fn BZ2_bzDecompressInit(
    strm: *mut bz_stream,
    verbosity: c_int,
    small: c_int,
) -> c_int {
    let Some(strm) = (unsafe { BzStream::from_ptr(strm) }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };
    BZ2_bzDecompressInitHelp(strm, verbosity, small) as c_int
}

pub(crate) fn BZ2_bzDecompressInitHelp(
    strm: &mut BzStream<DState>,
    verbosity: c_int,
    small: c_int,
) -> ReturnCode {
    let decompress_mode = match small {
        0 => DecompressMode::Fast,
        1 => DecompressMode::Small,
        _ => return ReturnCode::BZ_PARAM_ERROR,
    };
    if !(0..=4).contains(&verbosity) {
        return ReturnCode::BZ_PARAM_ERROR;
    }

    // return a param error when no [valid allocator](bz_stream#custom-allocators) could be configured
    let Some(allocator) = configure_allocator(strm) else {
        return ReturnCode::BZ_PARAM_ERROR;
    };

    let Some(s) = allocator.allocate_zeroed::<DState>(1) else {
        return ReturnCode::BZ_MEM_ERROR;
    };

    // this `s.strm` pointer should _NEVER_ be used! it exists just as a consistency check to ensure
    // that a given state belongs to a given strm.
    unsafe { (*s).strm_addr = strm as *const _ as usize }; // FIXME use .addr() once stable

    unsafe {
        (*s).state = decompress::State::BZ_X_MAGIC_1;
        (*s).bsLive = 0;
        (*s).bsBuff = 0;
        (*s).calculatedCombinedCRC = 0;
    }

    unsafe {
        (*s).smallDecompress = decompress_mode;
        (*s).ll4 = DSlice::new();
        (*s).ll16 = DSlice::new();
        (*s).tt = DSlice::new();
        (*s).currBlockNo = 0;
        (*s).verbosity = verbosity;
    }

    strm.state = s;

    strm.total_in_lo32 = 0;
    strm.total_in_hi32 = 0;
    strm.total_out_lo32 = 0;
    strm.total_out_hi32 = 0;

    ReturnCode::BZ_OK
}

macro_rules! BZ_RAND_MASK {
    ($s:expr) => {
        ($s.rNToGo == 1) as u8
    };
}

macro_rules! BZ_RAND_UPD_MASK {
    ($s:expr) => {
        if ($s.rNToGo == 0) {
            $s.rNToGo = $crate::randtable::BZ2_RNUMS[$s.rTPos as usize];
            $s.rTPos += 1;
            if ($s.rTPos == 512) {
                $s.rTPos = 0
            };
        }
        $s.rNToGo -= 1;
    };
}

pub(crate) use BZ_RAND_UPD_MASK;

macro_rules! BZ_GET_FAST {
    ($s:expr) => {
        match $s.tt.as_slice().get($s.tPos as usize) {
            None => return true,
            Some(&bits) => {
                $s.tPos = bits;
                let tmp = ($s.tPos & 0xff) as u8;
                $s.tPos >>= 8;
                tmp
            }
        }
    };
}

fn un_rle_obuf_to_output_fast(strm: &mut BzStream<DState>, s: &mut DState) -> bool {
    let mut k1: u8;
    if s.blockRandomised {
        loop {
            /* try to finish existing run */
            loop {
                if s.state_out_len == 0 {
                    if strm.avail_out == 0 {
                        return false;
                    } else {
                        break;
                    }
                }
                if !strm.write_byte(s.state_out_ch) {
                    return false;
                }
                BZ_UPDATE_CRC!(s.calculatedBlockCRC, s.state_out_ch);
                s.state_out_len -= 1;
            }

            /* can a new run be started? */
            if s.nblock_used == s.save.nblock as i32 + 1 {
                return false;
            }

            /* Only caused by corrupt data stream? */
            if s.nblock_used > s.save.nblock as i32 + 1 {
                return true;
            }

            s.state_out_ch = s.k0;

            s.state_out_len = 1;
            k1 = BZ_GET_FAST!(s);
            BZ_RAND_UPD_MASK!(s);
            k1 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            };
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            s.state_out_len = 2;
            k1 = BZ_GET_FAST!(s);
            BZ_RAND_UPD_MASK!(s);
            k1 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            };
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            s.state_out_len = 3;
            k1 = BZ_GET_FAST!(s);
            BZ_RAND_UPD_MASK!(s);
            k1 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            };
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            k1 = BZ_GET_FAST!(s);
            BZ_RAND_UPD_MASK!(s);
            k1 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
            s.state_out_len = k1 as u32 + 4;
            s.k0 = BZ_GET_FAST!(s);
            BZ_RAND_UPD_MASK!(s);
            s.k0 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
        }
    } else {
        /* restore */
        let mut c_calculatedBlockCRC: u32 = s.calculatedBlockCRC;
        let mut c_state_out_ch: u8 = s.state_out_ch;
        let mut c_state_out_len: u32 = s.state_out_len;
        let mut c_nblock_used: i32 = s.nblock_used;
        let mut c_k0: u8 = s.k0;
        let mut c_tPos: u32 = s.tPos;
        let mut cs_next_out: *mut c_char = strm.next_out;
        let mut cs_avail_out: c_uint = strm.avail_out;
        let ro_blockSize100k: u8 = s.blockSize100k;
        /* end restore */

        let avail_out_INIT: u32 = cs_avail_out;
        let s_save_nblockPP: i32 = s.save.nblock as i32 + 1;

        let tt = &s.tt.as_slice()[..100000usize.wrapping_mul(usize::from(ro_blockSize100k))];

        macro_rules! BZ_GET_FAST_C {
            ($c_tPos:expr) => {
                match tt.get($c_tPos as usize) {
                    None => {
                        // return corrupt if we're past the length of the block
                        return true;
                    }
                    Some(&v) => (v >> 8, (v & 0xff) as u8),
                }
            };
        }

        'return_notr: loop {
            macro_rules! write_one_byte {
                ($byte:expr) => {
                    if cs_avail_out == 0 {
                        c_state_out_len = 1;
                        break 'return_notr;
                    } else {
                        unsafe { *(cs_next_out as *mut u8) = $byte };
                        BZ_UPDATE_CRC!(c_calculatedBlockCRC, $byte);
                        cs_next_out = unsafe { cs_next_out.add(1) };
                        cs_avail_out -= 1;
                    }
                };
            }

            if c_state_out_len > 0 {
                let bound = Ord::min(cs_avail_out, c_state_out_len);

                unsafe {
                    core::ptr::write_bytes(cs_next_out as *mut u8, c_state_out_ch, bound as usize);
                    cs_next_out = cs_next_out.add(bound as usize);
                };

                for _ in 0..bound {
                    BZ_UPDATE_CRC!(c_calculatedBlockCRC, c_state_out_ch);
                }

                cs_avail_out -= bound;
                c_state_out_len -= bound;

                if cs_avail_out == 0 {
                    break 'return_notr;
                }
            }

            loop {
                /* Only caused by corrupt data stream? */
                if c_nblock_used > s_save_nblockPP {
                    return true;
                }

                /* can a new run be started? */
                if c_nblock_used == s_save_nblockPP {
                    c_state_out_len = 0;
                    break 'return_notr;
                }

                c_state_out_ch = c_k0;
                (c_tPos, k1) = BZ_GET_FAST_C!(c_tPos);
                c_nblock_used += 1;

                if k1 != c_k0 {
                    c_k0 = k1;
                    write_one_byte!(c_state_out_ch);
                    continue;
                }

                if c_nblock_used == s_save_nblockPP {
                    write_one_byte!(c_state_out_ch);
                    continue;
                }

                c_state_out_len = 2;
                (c_tPos, k1) = BZ_GET_FAST_C!(c_tPos);
                c_nblock_used += 1;

                if c_nblock_used == s_save_nblockPP {
                    continue 'return_notr;
                }

                if k1 != c_k0 {
                    c_k0 = k1;
                    continue 'return_notr;
                }

                c_state_out_len = 3;
                (c_tPos, k1) = BZ_GET_FAST_C!(c_tPos);
                c_nblock_used += 1;

                if c_nblock_used == s_save_nblockPP {
                    continue 'return_notr;
                }

                if k1 != c_k0 {
                    c_k0 = k1;
                    continue 'return_notr;
                }

                (c_tPos, k1) = BZ_GET_FAST_C!(c_tPos);
                c_nblock_used += 1;
                c_state_out_len = k1 as u32 + 4;
                (c_tPos, c_k0) = BZ_GET_FAST_C!(c_tPos);
                c_nblock_used += 1;

                continue 'return_notr;
            }
        }

        /* save */
        let total_out_lo32_old: c_uint = strm.total_out_lo32;
        strm.total_out_lo32 =
            (strm.total_out_lo32).wrapping_add(avail_out_INIT.wrapping_sub(cs_avail_out));
        if strm.total_out_lo32 < total_out_lo32_old {
            strm.total_out_hi32 = (strm.total_out_hi32).wrapping_add(1);
        }
        s.calculatedBlockCRC = c_calculatedBlockCRC;
        s.state_out_ch = c_state_out_ch;
        s.state_out_len = c_state_out_len;
        s.nblock_used = c_nblock_used;
        s.k0 = c_k0;
        s.tPos = c_tPos;
        strm.next_out = cs_next_out;
        strm.avail_out = cs_avail_out;
        /* end save */
    }

    false
}

#[inline]
pub(crate) fn index_into_f(index: u32, cftab: &[u32; 257]) -> u8 {
    let mut nb = 0u16;
    let mut na = 256;
    loop {
        let mid = (nb + na) >> 1;
        if index >= cftab[mid as usize] {
            nb = mid;
        } else {
            na = mid;
        }
        if na - nb == 1 {
            break;
        }
    }

    // NOTE: nb < na, hence nb will fit in a u8
    debug_assert!(u8::try_from(nb).is_ok());
    nb as u8
}

macro_rules! GET_LL4 {
    ($s:expr, $i:expr) => {
        $s.ll4.as_slice()[($s.tPos >> 1) as usize] as u32 >> ($s.tPos << 2 & 0x4) & 0xf
    };
}

macro_rules! BZ_GET_SMALL {
    ($s:expr) => {
        match $s.ll16.as_slice().get($s.tPos as usize) {
            None => return true,
            Some(&low_bits) => {
                let high_bits = GET_LL4!($s, $s.tPos);
                let tmp = index_into_f($s.tPos, &$s.cftab);
                $s.tPos = u32::from(low_bits) | high_bits << 16;
                tmp
            }
        }
    };
}

fn un_rle_obuf_to_output_small(strm: &mut BzStream<DState>, s: &mut DState) -> bool {
    let mut k1: u8;
    if s.blockRandomised {
        loop {
            /* try to finish existing run */
            loop {
                if s.state_out_len == 0 {
                    match strm.avail_out {
                        0 => return false,
                        _ => break,
                    }
                }
                if !strm.write_byte(s.state_out_ch) {
                    return false;
                }
                BZ_UPDATE_CRC!(s.calculatedBlockCRC, s.state_out_ch);
                s.state_out_len -= 1;
            }

            /* can a new run be started? */
            if s.nblock_used == s.save.nblock as i32 + 1 {
                return false;
            }

            /* Only caused by corrupt data stream? */
            if s.nblock_used > s.save.nblock as i32 + 1 {
                return true;
            }

            s.state_out_ch = s.k0;

            s.state_out_len = 1;
            k1 = BZ_GET_SMALL!(s);
            BZ_RAND_UPD_MASK!(s);
            k1 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            };
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            s.state_out_len = 2;
            k1 = BZ_GET_SMALL!(s);
            BZ_RAND_UPD_MASK!(s);
            k1 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            }
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            s.state_out_len = 3;
            k1 = BZ_GET_SMALL!(s);
            BZ_RAND_UPD_MASK!(s);
            k1 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            }
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            k1 = BZ_GET_SMALL!(s);
            BZ_RAND_UPD_MASK!(s);
            k1 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
            s.state_out_len = k1 as u32 + 4;
            s.k0 = BZ_GET_SMALL!(s);
            BZ_RAND_UPD_MASK!(s);
            s.k0 ^= BZ_RAND_MASK!(s);
            s.nblock_used += 1;
        }
    } else {
        loop {
            loop {
                if s.state_out_len == 0 {
                    if strm.avail_out == 0 {
                        return false;
                    } else {
                        break;
                    }
                }
                if !strm.write_byte(s.state_out_ch) {
                    return false;
                }
                BZ_UPDATE_CRC!(s.calculatedBlockCRC, s.state_out_ch);
                s.state_out_len -= 1;
            }
            if s.nblock_used == s.save.nblock as i32 + 1 {
                return false;
            }
            if s.nblock_used > s.save.nblock as i32 + 1 {
                return true;
            }

            s.state_out_len = 1;
            s.state_out_ch = s.k0;
            k1 = BZ_GET_SMALL!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            }
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            s.state_out_len = 2;
            k1 = BZ_GET_SMALL!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            }
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            s.state_out_len = 3;
            k1 = BZ_GET_SMALL!(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save.nblock as i32 + 1 {
                continue;
            }
            if k1 != s.k0 {
                s.k0 = k1;
                continue;
            };

            k1 = BZ_GET_SMALL!(s);
            s.nblock_used += 1;
            s.state_out_len = k1 as u32 + 4;
            s.k0 = BZ_GET_SMALL!(s);
            s.nblock_used += 1;
        }
    }
}

/// Decompresses as much data as possible, and stops when the input buffer becomes empty or the output buffer becomes full.
///
/// # Returns
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `strm.is_null()`
///     - `strm.s.is_null()`
///     - `strm.avail_out < 1`
/// - [`BZ_DATA_ERROR`] if a data integrity error is detected in the compressed stream
/// - [`BZ_DATA_ERROR_MAGIC`] if the compressed stream doesn't begin with the right magic bytes
/// - [`BZ_MEM_ERROR`] if there wasn't enough memory available
/// - [`BZ_STREAM_END`] if the logical end of the data stream was detected and all output has been
///   written to the output buffer
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// * Either
///     - `strm` is `NULL`
///     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`BZ2_bzDecompressInit`]
/// * Either
///     - `strm.next_in` is `NULL` and `strm.avail_in` is 0
///     - `strm.next_in` is readable for `strm.avail_in` bytes
/// * Either
///     - `strm.next_out` is `NULL` and `strm.avail_out` is `0`
///     - `strm.next_out` is writable for `strm.avail_out` bytes
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzDecompress))]
pub unsafe extern "C" fn BZ2_bzDecompress(strm: *mut bz_stream) -> c_int {
    let Some(strm) = (unsafe { BzStream::from_ptr(strm) }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };

    BZ2_bzDecompressHelp(strm) as c_int
}

pub(crate) fn BZ2_bzDecompressHelp(strm: &mut BzStream<DState>) -> ReturnCode {
    let Some(s) = (unsafe { strm.state.as_mut() }) else {
        return ReturnCode::BZ_PARAM_ERROR;
    };

    // FIXME use .addr() once stable
    if s.strm_addr != strm as *mut _ as usize {
        return ReturnCode::BZ_PARAM_ERROR;
    }

    let Some(allocator) = strm.allocator() else {
        return ReturnCode::BZ_PARAM_ERROR;
    };

    loop {
        match s.state {
            decompress::State::BZ_X_IDLE => {
                return ReturnCode::BZ_SEQUENCE_ERROR;
            }
            decompress::State::BZ_X_OUTPUT => {
                let corrupt = match s.smallDecompress {
                    DecompressMode::Small => un_rle_obuf_to_output_small(strm, s),
                    DecompressMode::Fast => un_rle_obuf_to_output_fast(strm, s),
                };

                if corrupt {
                    return ReturnCode::BZ_DATA_ERROR;
                }

                if s.nblock_used == s.save.nblock as i32 + 1 && s.state_out_len == 0 {
                    s.calculatedBlockCRC = !s.calculatedBlockCRC;
                    if s.verbosity >= 3 {
                        debug_log!(
                            " {{{:#08x}, {:#08x}}}",
                            s.storedBlockCRC,
                            s.calculatedBlockCRC,
                        );
                    }
                    if s.verbosity >= 2 {
                        debug_log!("]");
                    }
                    #[cfg(not(feature = "__internal-fuzz-disable-checksum"))]
                    if s.calculatedBlockCRC != s.storedBlockCRC {
                        return ReturnCode::BZ_DATA_ERROR;
                    }
                    s.calculatedCombinedCRC = s.calculatedCombinedCRC.rotate_left(1);
                    s.calculatedCombinedCRC ^= s.calculatedBlockCRC;
                    s.state = decompress::State::BZ_X_BLKHDR_1;

                    continue;
                } else {
                    return ReturnCode::BZ_OK;
                }
            }
            _ => match decompress(strm, s, &allocator) {
                ReturnCode::BZ_STREAM_END => {
                    if s.verbosity >= 3 {
                        debug_log!(
                            "\n    combined CRCs: stored = {:#08x}, computed = {:#08x}",
                            s.storedCombinedCRC,
                            s.calculatedCombinedCRC,
                        );
                    }
                    #[cfg(not(feature = "__internal-fuzz-disable-checksum"))]
                    if s.calculatedCombinedCRC != s.storedCombinedCRC {
                        return ReturnCode::BZ_DATA_ERROR;
                    }
                    return ReturnCode::BZ_STREAM_END;
                }
                return_code => match s.state {
                    decompress::State::BZ_X_OUTPUT => continue,
                    _ => return return_code,
                },
            },
        }
    }
}

/// Deallocates all dynamically allocated data structures for this stream.
///
/// # Returns
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `strm.is_null()`
///     - `strm.s.is_null()`
///     - no [valid allocator](bz_stream#custom-allocators) could be configured
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// * Either
///     - `strm` is `NULL`
///     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`BZ2_bzDecompressInit`]
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzDecompressEnd))]
pub unsafe extern "C" fn BZ2_bzDecompressEnd(strm: *mut bz_stream) -> c_int {
    let Some(strm) = (unsafe { BzStream::from_ptr(strm) }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };
    BZ2_bzDecompressEndHelp(strm) as c_int
}

fn BZ2_bzDecompressEndHelp(strm: &mut BzStream<DState>) -> ReturnCode {
    let Some(s) = (unsafe { strm.state.as_mut() }) else {
        return ReturnCode::BZ_PARAM_ERROR;
    };

    // FIXME use .addr() once stable
    if s.strm_addr != strm as *mut _ as usize {
        return ReturnCode::BZ_PARAM_ERROR;
    }

    let Some(allocator) = strm.allocator() else {
        return ReturnCode::BZ_PARAM_ERROR;
    };

    unsafe {
        s.tt.dealloc(&allocator);
        s.ll16.dealloc(&allocator);
        s.ll4.dealloc(&allocator);
    }

    unsafe { allocator.deallocate(strm.state, 1) };
    strm.state = ptr::null_mut::<DState>();

    ReturnCode::BZ_OK
}

/// Compress the input data into the destination buffer.
///
/// This function attempts to compress the data in `source[0 .. sourceLen]` into `dest[0 .. *destLen]`.
/// If the destination buffer is big enough, `*destLen` is set to the size of the compressed data, and [`BZ_OK`] is returned.
/// If the compressed data won't fit, `*destLen` is unchanged, and [`BZ_OUTBUFF_FULL`] is returned.
///
/// For the meaning of parameters `blockSize100k`, `verbosity` and `workFactor`, see [`BZ2_bzCompressInit`].
///
/// A safe choice for the length of the output buffer is a size 1% larger than the input length,
/// plus 600 extra bytes.
///
/// # Returns
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `dest.is_null()`
///     - `destLen.is_null()`
///     - `source.is_null()`
///     - `!(1..=9).contains(&blockSize100k)`
///     - `!(0..=4).contains(&verbosity)`
///     - `!(0..=250).contains(&workFactor)`
/// - [`BZ_MEM_ERROR`] if insufficient memory is available
/// - [`BZ_OUTBUFF_FULL`] if the size of the compressed data exceeds `*destLen`
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `destLen` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `dest` is `NULL`
///     - `dest` is writable for `*destLen` bytes
/// * Either
///     - `source` is `NULL`
///     - `source` is readable for `sourceLen`
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzBuffToBuffCompress))]
pub unsafe extern "C" fn BZ2_bzBuffToBuffCompress(
    dest: *mut c_char,
    destLen: *mut c_uint,
    source: *mut c_char,
    sourceLen: c_uint,
    blockSize100k: c_int,
    verbosity: c_int,
    workFactor: c_int,
) -> c_int {
    if dest.is_null() || source.is_null() {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    }

    let Some(destLen) = (unsafe { destLen.as_mut() }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };

    match unsafe {
        BZ2_bzBuffToBuffCompressHelp(
            dest,
            *destLen,
            source,
            sourceLen,
            blockSize100k,
            verbosity,
            workFactor,
        )
    } {
        Ok(written) => {
            *destLen -= written;
            ReturnCode::BZ_OK as c_int
        }
        Err(err) => err as c_int,
    }
}

unsafe fn BZ2_bzBuffToBuffCompressHelp(
    dest: *mut c_char,
    destLen: c_uint,
    source: *mut c_char,
    sourceLen: c_uint,
    blockSize100k: c_int,
    verbosity: c_int,
    workFactor: c_int,
) -> Result<c_uint, ReturnCode> {
    let mut strm = BzStream::zeroed();

    match BZ2_bzCompressInitHelp(&mut strm, blockSize100k, verbosity, workFactor) {
        ReturnCode::BZ_OK => {}
        ret => return Err(ret),
    }

    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = destLen;

    match BZ2_bzCompressHelp(&mut strm, Action::Finish as i32) {
        ReturnCode::BZ_FINISH_OK => {
            BZ2_bzCompressEndHelp(&mut strm);
            Err(ReturnCode::BZ_OUTBUFF_FULL)
        }
        ReturnCode::BZ_STREAM_END => {
            BZ2_bzCompressEndHelp(&mut strm);
            Ok(strm.avail_out)
        }
        error => {
            BZ2_bzCompressEndHelp(&mut strm);
            Err(error)
        }
    }
}

/// Decompress the input data into the destination buffer.
///
/// This function attempts to decompress the data in `source[0 .. sourceLen]` into `dest[0 .. *destLen]`.
/// If the destination buffer is big enough, `*destLen` is set to the size of the decompressed data, and [`BZ_OK`] is returned.
/// If the decompressed data won't fit, `*destLen` is unchanged, and [`BZ_OUTBUFF_FULL`] is returned.
///
/// For the meaning of parameters `small`, `verbosity`, see [`BZ2_bzDecompressInit`].
///
/// Because the compression ratio of the compressed data cannot be known in advance,
/// there is no easy way to guarantee that the output buffer will be big enough.
/// You may of course make arrangements in your code to record the size of the uncompressed data,
/// but such a mechanism is beyond the scope of this library.
///
/// # Returns
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `dest.is_null()`
///     - `destLen.is_null()`
///     - `source.is_null()`
///     - `!(0..=1).contains(&small)`
///     - `!(0..=4).contains(&verbosity)`
/// - [`BZ_MEM_ERROR`] if insufficient memory is available
/// - [`BZ_OUTBUFF_FULL`] if the size of the compressed data exceeds `*destLen`
/// - [`BZ_DATA_ERROR`] if a data integrity error is detected in the compressed stream
/// - [`BZ_DATA_ERROR_MAGIC`] if the compressed stream doesn't begin with the right magic bytes
/// - [`BZ_UNEXPECTED_EOF`] if the compressed data ends before the logical end-of-stream was detected
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `destLen` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `dest` is `NULL`
///     - `dest` is writable for `*destLen` bytes
/// * Either
///     - `source` is `NULL`
///     - `source` is readable for `sourceLen`
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[cfg_attr(feature = "export-symbols", export_name = prefix!(BZ2_bzBuffToBuffDecompress))]
pub unsafe extern "C" fn BZ2_bzBuffToBuffDecompress(
    dest: *mut c_char,
    destLen: *mut c_uint,
    source: *mut c_char,
    sourceLen: c_uint,
    small: c_int,
    verbosity: c_int,
) -> c_int {
    if dest.is_null() || destLen.is_null() || source.is_null() {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    }

    let Some(destLen) = (unsafe { destLen.as_mut() }) else {
        return ReturnCode::BZ_PARAM_ERROR as c_int;
    };

    match unsafe {
        BZ2_bzBuffToBuffDecompressHelp(dest, *destLen, source, sourceLen, small, verbosity)
    } {
        Ok(written) => {
            *destLen -= written;
            ReturnCode::BZ_OK as c_int
        }
        Err(err) => err as c_int,
    }
}

unsafe fn BZ2_bzBuffToBuffDecompressHelp(
    dest: *mut c_char,
    destLen: c_uint,
    source: *mut c_char,
    sourceLen: c_uint,
    small: c_int,
    verbosity: c_int,
) -> Result<c_uint, ReturnCode> {
    let mut strm = BzStream::zeroed();

    match BZ2_bzDecompressInitHelp(&mut strm, verbosity, small) {
        ReturnCode::BZ_OK => {}
        ret => return Err(ret),
    }

    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = destLen;

    match BZ2_bzDecompressHelp(&mut strm) {
        ReturnCode::BZ_OK => {
            BZ2_bzDecompressEndHelp(&mut strm);
            match strm.avail_out {
                0 => Err(ReturnCode::BZ_OUTBUFF_FULL),
                _ => Err(ReturnCode::BZ_UNEXPECTED_EOF),
            }
        }
        ReturnCode::BZ_STREAM_END => {
            BZ2_bzDecompressEndHelp(&mut strm);
            Ok(strm.avail_out)
        }
        error => {
            BZ2_bzDecompressEndHelp(&mut strm);
            Err(error)
        }
    }
}
