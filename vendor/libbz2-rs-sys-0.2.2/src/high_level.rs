#![allow(unsafe_op_in_unsafe_fn)]

use core::ffi::{c_char, c_int, c_uint, c_void, CStr};
use core::{mem, ptr};

use libc::FILE;
use libc::{fclose, fdopen, ferror, fflush, fgetc, fopen, fread, fwrite, ungetc};

use crate::allocator::Allocator;
use crate::bzlib::prefix;
use crate::bzlib::BZ_MAX_UNUSED_U32;
use crate::bzlib::{bz_stream, BZ2_bzCompressEnd, BZ2_bzDecompressEnd};
use crate::bzlib::{Action, BzStream, ReturnCode};
use crate::bzlib::{
    BZ2_bzCompressHelp, BZ2_bzCompressInitHelp, BZ2_bzDecompressHelp, BZ2_bzDecompressInitHelp,
};
use crate::BZ_MAX_UNUSED;

#[cfg(doc)]
use crate::{
    BZ2_bzCompressInit, BZ2_bzDecompressInit, BZ_CONFIG_ERROR, BZ_DATA_ERROR, BZ_DATA_ERROR_MAGIC,
    BZ_FINISH, BZ_FINISH_OK, BZ_FLUSH, BZ_FLUSH_OK, BZ_IO_ERROR, BZ_MEM_ERROR, BZ_OK,
    BZ_OUTBUFF_FULL, BZ_PARAM_ERROR, BZ_RUN, BZ_RUN_OK, BZ_SEQUENCE_ERROR, BZ_STREAM_END,
    BZ_UNEXPECTED_EOF,
};

// FIXME remove this
#[cfg(not(target_os = "windows"))]
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stdinp")]
    static mut stdin: *mut FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stdoutp")]
    static mut stdout: *mut FILE;
}

#[cfg(target_os = "windows")]
extern "C" {
    fn __acrt_iob_func(idx: libc::c_uint) -> *mut FILE;
}

#[cfg(not(target_os = "windows"))]
macro_rules! STDIN {
    () => {
        stdin
    };
}

#[cfg(target_os = "windows")]
macro_rules! STDIN {
    () => {
        __acrt_iob_func(0)
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! STDOUT {
    () => {
        stdout
    };
}

#[cfg(target_os = "windows")]
macro_rules! STDOUT {
    () => {
        __acrt_iob_func(1)
    };
}

/// Abstract handle to a `.bz2` file.
///
/// This type is created by:
///
/// - [`BZ2_bzReadOpen`]
/// - [`BZ2_bzWriteOpen`]
/// - [`BZ2_bzopen`]
///
/// And destructed by:
///
/// - [`BZ2_bzReadClose`]
/// - [`BZ2_bzWriteClose`]
/// - [`BZ2_bzclose`]
#[allow(non_camel_case_types)]
pub struct BZFILE {
    handle: *mut FILE,
    buf: [i8; BZ_MAX_UNUSED as usize],
    bufN: i32,
    strm: bz_stream,
    lastErr: ReturnCode,
    operation: Operation,
    initialisedOk: bool,
}

unsafe fn myfeof(f: *mut FILE) -> bool {
    let c = fgetc(f);
    if c == -1 {
        return true;
    }

    ungetc(c, f);

    false
}

macro_rules! BZ_SETERR_RAW {
    ($bzerror:expr, $bzf:expr, $return_code:expr) => {
        if let Some(bzerror) = $bzerror.as_deref_mut() {
            *bzerror = $return_code as c_int;
        }

        if let Some(bzf) = $bzf.as_deref_mut() {
            bzf.lastErr = $return_code;
        }
    };
}

macro_rules! BZ_SETERR {
    ($bzerror:expr, $bzf:expr, $return_code:expr) => {
        if let Some(bzerror) = $bzerror.as_deref_mut() {
            *bzerror = $return_code as c_int;
        }

        $bzf.lastErr = $return_code;
    };
}

/// Prepare to write compressed data to a file handle.
///
/// The file handle `f` should refer to a file which has been opened for writing, and for which the error indicator `libc::ferror(f)` is not set.
///
/// For the meaning of parameters `blockSize100k`, `verbosity` and `workFactor`, see [`BZ2_bzCompressInit`].
///
/// # Returns
///
/// - if `*bzerror` is [`BZ_OK`], a valid pointer to an abstract `BZFILE`
/// - otherwise `NULL`
///
/// # Possible assignments to `bzerror`
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `f.is_null`
///     - `!(1..=9).contains(&blockSize100k)`
///     - `!(0..=4).contains(&verbosity)`
///     - `!(0..=250).contains(&workFactor)`
/// - [`BZ_CONFIG_ERROR`] if no default allocator is configured
/// - [`BZ_IO_ERROR`] if `libc::ferror(f)` is nonzero
/// - [`BZ_MEM_ERROR`] if insufficient memory is available
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `bzerror` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `f` is `NULL`
///     - `f` a valid pointer to a `FILE`
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzWriteOpen)]
pub unsafe extern "C" fn BZ2_bzWriteOpen(
    bzerror: *mut c_int,
    f: *mut FILE,
    blockSize100k: c_int,
    verbosity: c_int,
    workFactor: c_int,
) -> *mut BZFILE {
    BZ2_bzWriteOpenHelp(bzerror.as_mut(), f, blockSize100k, verbosity, workFactor)
}

unsafe fn BZ2_bzWriteOpenHelp(
    mut bzerror: Option<&mut c_int>,
    f: *mut FILE,
    blockSize100k: c_int,
    verbosity: c_int,
    mut workFactor: c_int,
) -> *mut BZFILE {
    let mut bzf: Option<&mut BZFILE> = None;

    BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_OK);

    if f.is_null()
        || !(1..=9).contains(&blockSize100k)
        || !(0..=250).contains(&workFactor)
        || !(0..=4).contains(&verbosity)
    {
        BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_PARAM_ERROR);
        return ptr::null_mut();
    }

    if ferror(f) != 0 {
        BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
        return ptr::null_mut();
    }

    let Some(allocator) = Allocator::DEFAULT else {
        BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_CONFIG_ERROR);
        return ptr::null_mut();
    };

    let Some(bzf) = allocator.allocate_zeroed::<BZFILE>(1) else {
        BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_MEM_ERROR);
        return ptr::null_mut();
    };

    // SAFETY: bzf is non-null and correctly initalized
    let bzf = unsafe { &mut *bzf };

    BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_OK);

    bzf.initialisedOk = false;
    bzf.bufN = 0;
    bzf.handle = f;
    bzf.operation = Operation::Writing;
    bzf.strm.bzalloc = None;
    bzf.strm.bzfree = None;
    bzf.strm.opaque = ptr::null_mut();

    if workFactor == 0 {
        workFactor = 30;
    }

    match BZ2_bzCompressInitHelp(
        BzStream::from_mut(&mut bzf.strm),
        blockSize100k,
        verbosity,
        workFactor,
    ) {
        ReturnCode::BZ_OK => {
            bzf.strm.avail_in = 0;
            bzf.initialisedOk = true;

            bzf as *mut BZFILE
        }
        error => {
            BZ_SETERR!(bzerror, bzf, error);
            allocator.deallocate(bzf, 1);

            ptr::null_mut()
        }
    }
}

/// Absorbs `len` bytes from the buffer `buf`, eventually to be compressed and written to the file.
///
/// # Returns
///
/// # Possible assignments to `bzerror`
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `b.is_null()`
///     - `buf.is_null()`
///     - `len < 0`
/// - [`BZ_SEQUENCE_ERROR`] if b was opened with [`BZ2_bzReadOpen`]
/// - [`BZ_IO_ERROR`] if there is an error writing to the compressed file
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `bzerror` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzWriteOpen`] or [`BZ2_bzReadOpen`]
/// * Either
///     - `buf` is `NULL`
///     - `buf` is writable for `len` bytes
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzWrite)]
pub unsafe extern "C" fn BZ2_bzWrite(
    bzerror: *mut c_int,
    b: *mut BZFILE,
    buf: *const c_void,
    len: c_int,
) {
    BZ2_bzWriteHelp(bzerror.as_mut(), b.as_mut(), buf, len)
}

unsafe fn BZ2_bzWriteHelp(
    mut bzerror: Option<&mut c_int>,
    mut b: Option<&mut BZFILE>,
    buf: *const c_void,
    len: c_int,
) {
    BZ_SETERR_RAW!(bzerror, b, ReturnCode::BZ_OK);

    let Some(bzf) = b.as_mut() else {
        BZ_SETERR_RAW!(bzerror, b, ReturnCode::BZ_PARAM_ERROR);
        return;
    };

    if buf.is_null() || len < 0 as c_int {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_PARAM_ERROR);
        return;
    }

    if !matches!(bzf.operation, Operation::Writing) {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_SEQUENCE_ERROR);
        return;
    }

    if ferror(bzf.handle) != 0 {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
        return;
    }

    if len == 0 {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_OK);
        return;
    }

    bzf.strm.avail_in = len as c_uint;
    bzf.strm.next_in = buf.cast::<c_char>();

    loop {
        bzf.strm.avail_out = BZ_MAX_UNUSED_U32;
        bzf.strm.next_out = bzf.buf.as_mut_ptr().cast::<c_char>();
        match BZ2_bzCompressHelp(
            unsafe { BzStream::from_mut(&mut bzf.strm) },
            Action::Run as c_int,
        ) {
            ReturnCode::BZ_RUN_OK => {
                if bzf.strm.avail_out < BZ_MAX_UNUSED_U32 {
                    let n1 = (BZ_MAX_UNUSED_U32 - bzf.strm.avail_out) as usize;
                    let n2 = fwrite(
                        bzf.buf.as_mut_ptr().cast::<c_void>(),
                        mem::size_of::<u8>(),
                        n1,
                        bzf.handle,
                    );
                    if n1 != n2 || ferror(bzf.handle) != 0 {
                        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
                        return;
                    }
                }
                if bzf.strm.avail_in == 0 {
                    BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_OK);
                    return;
                }
            }
            error => {
                BZ_SETERR!(bzerror, bzf, error);
                return;
            }
        }
    }
}

/// Compresses and flushes to the compressed file all data so far supplied by [`BZ2_bzWrite`].
///
/// The logical end-of-stream markers are also written, so subsequent calls to [`BZ2_bzWrite`] are illegal.
/// All memory associated with the compressed file `b` is released. [`libc::fflush`] is called on the compressed file,
/// but it is not [`libc::fclose`]'d.
///
/// If [`BZ2_bzWriteClose`] is called to clean up after an error, the only action is to release the memory.
/// The library records the error codes issued by previous calls, so this situation will be detected automatically.
/// There is no attempt to complete the compression operation, nor to [`libc::fflush`] the compressed file.
/// You can force this behaviour to happen even in the case of no error, by passing a nonzero value to `abandon`.
///
/// # Possible assignments to `bzerror`
///
/// - [`BZ_CONFIG_ERROR`] if no default allocator is configured
/// - [`BZ_SEQUENCE_ERROR`] if b was opened with [`BZ2_bzWriteOpen`]
/// - [`BZ_IO_ERROR`] if there is an error writing to the compressed file
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `bzerror` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzReadOpen`] or [`BZ2_bzWriteOpen`]
/// * `nbytes_in` satisfies the requirements of [`pointer::as_mut`]
/// * `nbytes_out` satisfies the requirements of [`pointer::as_mut`]
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzWriteClose)]
pub unsafe extern "C" fn BZ2_bzWriteClose(
    bzerror: *mut c_int,
    b: *mut BZFILE,
    abandon: c_int,
    nbytes_in: *mut c_uint,
    nbytes_out: *mut c_uint,
) {
    BZ2_bzWriteCloseHelp(
        bzerror.as_mut(),
        b.as_mut(),
        abandon,
        nbytes_in.as_mut(),
        nbytes_out.as_mut(),
    )
}

unsafe fn BZ2_bzWriteCloseHelp(
    bzerror: Option<&mut c_int>,
    b: Option<&mut BZFILE>,
    abandon: c_int,
    nbytes_in: Option<&mut c_uint>,
    nbytes_out: Option<&mut c_uint>,
) {
    BZ2_bzWriteClose64Help(bzerror, b, abandon, nbytes_in, None, nbytes_out, None);
}

/// Compresses and flushes to the compressed file all data so far supplied by [`BZ2_bzWrite`].
///
/// The logical end-of-stream markers are also written, so subsequent calls to [`BZ2_bzWrite`] are illegal.
/// All memory associated with the compressed file `b` is released. [`libc::fflush`] is called on the compressed file,
/// but it is not [`libc::fclose`]'d.
///
/// If [`BZ2_bzWriteClose64`] is called to clean up after an error, the only action is to release the memory.
/// The library records the error codes issued by previous calls, so this situation will be detected automatically.
/// There is no attempt to complete the compression operation, nor to [`libc::fflush`] the compressed file.
/// You can force this behaviour to happen even in the case of no error, by passing a nonzero value to `abandon`.
///
/// # Possible assignments to `bzerror`
///
/// - [`BZ_CONFIG_ERROR`] if no default allocator is configured
/// - [`BZ_SEQUENCE_ERROR`] if b was opened with [`BZ2_bzWriteOpen`]
/// - [`BZ_IO_ERROR`] if there is an error writing to the compressed file
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `bzerror` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzReadOpen`] or [`BZ2_bzWriteOpen`]
/// * `nbytes_in_lo32: satisfies the requirements of [`pointer::as_mut`]
/// * `nbytes_in_hi32: satisfies the requirements of [`pointer::as_mut`]
/// * `nbytes_out_lo32: satisfies the requirements of [`pointer::as_mut`]
/// * `nbytes_out_hi32: satisfies the requirements of [`pointer::as_mut`]
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzWriteClose64)]
pub unsafe extern "C" fn BZ2_bzWriteClose64(
    bzerror: *mut c_int,
    b: *mut BZFILE,
    abandon: c_int,
    nbytes_in_lo32: *mut c_uint,
    nbytes_in_hi32: *mut c_uint,
    nbytes_out_lo32: *mut c_uint,
    nbytes_out_hi32: *mut c_uint,
) {
    BZ2_bzWriteClose64Help(
        bzerror.as_mut(),
        b.as_mut(),
        abandon,
        nbytes_in_lo32.as_mut(),
        nbytes_in_hi32.as_mut(),
        nbytes_out_lo32.as_mut(),
        nbytes_out_hi32.as_mut(),
    )
}

unsafe fn BZ2_bzWriteClose64Help(
    mut bzerror: Option<&mut c_int>,
    mut b: Option<&mut BZFILE>,
    abandon: c_int,
    mut nbytes_in_lo32: Option<&mut c_uint>,
    mut nbytes_in_hi32: Option<&mut c_uint>,
    mut nbytes_out_lo32: Option<&mut c_uint>,
    mut nbytes_out_hi32: Option<&mut c_uint>,
) {
    let Some(bzf) = b else {
        BZ_SETERR_RAW!(bzerror, b, ReturnCode::BZ_PARAM_ERROR);
        return;
    };

    if !matches!(bzf.operation, Operation::Writing) {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_SEQUENCE_ERROR);
        return;
    }

    if ferror(bzf.handle) != 0 {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
        return;
    }

    if let Some(nbytes_in_lo32) = nbytes_in_lo32.as_deref_mut() {
        *nbytes_in_lo32 = 0
    }
    if let Some(nbytes_in_hi32) = nbytes_in_hi32.as_deref_mut() {
        *nbytes_in_hi32 = 0;
    }
    if let Some(nbytes_out_lo32) = nbytes_out_lo32.as_deref_mut() {
        *nbytes_out_lo32 = 0;
    }
    if let Some(nbytes_out_hi32) = nbytes_out_hi32.as_deref_mut() {
        *nbytes_out_hi32 = 0;
    }

    if abandon == 0 && bzf.lastErr == ReturnCode::BZ_OK {
        loop {
            bzf.strm.avail_out = BZ_MAX_UNUSED_U32;
            bzf.strm.next_out = (bzf.buf).as_mut_ptr().cast::<c_char>();
            match BZ2_bzCompressHelp(BzStream::from_mut(&mut bzf.strm), 2 as c_int) {
                ret @ (ReturnCode::BZ_FINISH_OK | ReturnCode::BZ_STREAM_END) => {
                    if bzf.strm.avail_out < BZ_MAX_UNUSED_U32 {
                        let n1 = (BZ_MAX_UNUSED_U32 - bzf.strm.avail_out) as usize;
                        let n2 = fwrite(
                            bzf.buf.as_mut_ptr().cast::<c_void>(),
                            mem::size_of::<u8>(),
                            n1,
                            bzf.handle,
                        );
                        if n1 != n2 || ferror(bzf.handle) != 0 {
                            BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
                        }
                    }

                    if let ReturnCode::BZ_STREAM_END = ret {
                        break;
                    }
                }
                ret => {
                    BZ_SETERR!(bzerror, bzf, ret);
                    return;
                }
            }
        }
    }

    if abandon == 0 && ferror(bzf.handle) == 0 {
        fflush(bzf.handle);
        if ferror(bzf.handle) != 0 {
            BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
            return;
        }
    }

    if let Some(nbytes_in_lo32) = nbytes_in_lo32 {
        *nbytes_in_lo32 = bzf.strm.total_in_lo32;
    }
    if let Some(nbytes_in_hi32) = nbytes_in_hi32 {
        *nbytes_in_hi32 = bzf.strm.total_in_hi32;
    }
    if let Some(nbytes_out_lo32) = nbytes_out_lo32 {
        *nbytes_out_lo32 = bzf.strm.total_out_lo32;
    }
    if let Some(nbytes_out_hi32) = nbytes_out_hi32 {
        *nbytes_out_hi32 = bzf.strm.total_out_hi32;
    }

    BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_OK);

    BZ2_bzCompressEnd(&mut bzf.strm);

    let Some(allocator) = Allocator::DEFAULT else {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_CONFIG_ERROR);
        return;
    };

    allocator.deallocate(bzf, 1);
}

/// Prepare to read compressed data from a file handle.
///
/// The file handle `f` should refer to a file which has been opened for reading, and for which the error indicator `libc::ferror(f)` is not set.
///
/// If small is 1, the library will try to decompress using less memory, at the expense of speed.
///
/// For reasons explained below, [`BZ2_bzRead`] will decompress the nUnused bytes starting at unused, before starting to read from the file `f`.
/// At most [`BZ_MAX_UNUSED`] bytes may be supplied like this. If this facility is not required, you should pass NULL and 0 for unused and nUnused respectively.
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
/// - if `*bzerror` is [`BZ_OK`], a valid pointer to an abstract `BZFILE`
/// - otherwise `NULL`
///
/// # Possible assignments to `bzerror`
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `(unused.is_null() && nUnused != 0)`
///     - `(!unused.is_null() && !(0..=BZ_MAX_UNUSED).contains(&nUnused))`
///     - `!(0..=1).contains(&small)`
///     - `!(0..=4).contains(&verbosity)`
/// - [`BZ_CONFIG_ERROR`] if no default allocator is configured
/// - [`BZ_IO_ERROR`] if `libc::ferror(f)` is nonzero
/// - [`BZ_MEM_ERROR`] if insufficient memory is available
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `bzerror` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `unused` is `NULL`
///     - `unused` is readable for `nUnused` bytes
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzReadOpen)]
pub unsafe extern "C" fn BZ2_bzReadOpen(
    bzerror: *mut c_int,
    f: *mut FILE,
    verbosity: c_int,
    small: c_int,
    unused: *mut c_void,
    nUnused: c_int,
) -> *mut BZFILE {
    BZ2_bzReadOpenHelp(bzerror.as_mut(), f, verbosity, small, unused, nUnused)
}

unsafe fn BZ2_bzReadOpenHelp(
    mut bzerror: Option<&mut c_int>,
    f: *mut FILE,
    verbosity: c_int,
    small: c_int,
    unused: *mut c_void,
    nUnused: c_int,
) -> *mut BZFILE {
    let mut bzf: Option<&mut BZFILE> = None;

    BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_OK);

    if f.is_null()
        || !(0..=1).contains(&small)
        || !(0..=4).contains(&verbosity)
        || (unused.is_null() && nUnused != 0)
        || (!unused.is_null() && !(0..=BZ_MAX_UNUSED_U32 as c_int).contains(&nUnused))
    {
        BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_PARAM_ERROR);
        return ptr::null_mut::<BZFILE>();
    }

    if ferror(f) != 0 {
        BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
        return ptr::null_mut::<BZFILE>();
    }

    let Some(allocator) = Allocator::DEFAULT else {
        BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_CONFIG_ERROR);
        return ptr::null_mut();
    };

    let Some(bzf) = allocator.allocate_zeroed::<BZFILE>(1) else {
        BZ_SETERR_RAW!(bzerror, bzf, ReturnCode::BZ_MEM_ERROR);
        return ptr::null_mut();
    };

    // SAFETY: bzf is non-null and correctly initalized
    let bzf = unsafe { &mut *bzf };

    BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_OK);

    bzf.initialisedOk = false;
    bzf.handle = f;
    bzf.bufN = 0;
    bzf.operation = Operation::Reading;
    bzf.strm.bzalloc = None;
    bzf.strm.bzfree = None;
    bzf.strm.opaque = ptr::null_mut();

    if nUnused > 0 {
        ptr::copy(
            unused as *mut i8,
            bzf.buf[bzf.bufN as usize..].as_mut_ptr(),
            nUnused as usize,
        );
        bzf.bufN += nUnused;
    }

    match BZ2_bzDecompressInitHelp(BzStream::from_mut(&mut bzf.strm), verbosity, small) {
        ReturnCode::BZ_OK => {
            bzf.strm.avail_in = bzf.bufN as c_uint;
            bzf.strm.next_in = bzf.buf.as_mut_ptr().cast::<c_char>();
            bzf.initialisedOk = true;
        }
        ret => {
            BZ_SETERR!(bzerror, bzf, ret);

            allocator.deallocate(bzf, 1);

            return ptr::null_mut();
        }
    }

    bzf as *mut BZFILE
}

/// Releases all memory associated with a [`BZFILE`] opened with [`BZ2_bzReadOpen`].
///
/// This function does not call `fclose` on the underlying file handle, the caller should close the
/// file if appropriate.
///
/// This function should be called to clean up after all error situations on `BZFILE`s opened with
/// [`BZ2_bzReadOpen`].
///
/// # Possible assignments to `bzerror`
///
/// - [`BZ_CONFIG_ERROR`] if no default allocator is configured
/// - [`BZ_SEQUENCE_ERROR`] if b was opened with [`BZ2_bzWriteOpen`]
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `bzerror` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzReadOpen`] or [`BZ2_bzWriteOpen`]
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzReadClose)]
pub unsafe extern "C" fn BZ2_bzReadClose(bzerror: *mut c_int, b: *mut BZFILE) {
    BZ2_bzReadCloseHelp(bzerror.as_mut(), b.as_mut())
}

unsafe fn BZ2_bzReadCloseHelp(mut bzerror: Option<&mut c_int>, mut b: Option<&mut BZFILE>) {
    BZ_SETERR_RAW!(bzerror, b, ReturnCode::BZ_OK);

    let Some(bzf) = b else {
        BZ_SETERR_RAW!(bzerror, b, ReturnCode::BZ_OK);
        return;
    };

    if !matches!(bzf.operation, Operation::Reading) {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_SEQUENCE_ERROR);
        return;
    }

    if bzf.initialisedOk {
        BZ2_bzDecompressEnd(&mut bzf.strm);
    }

    let Some(allocator) = Allocator::DEFAULT else {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_CONFIG_ERROR);
        return;
    };

    allocator.deallocate(bzf, 1)
}

/// Reads up to `len` (uncompressed) bytes from the compressed file `b` into the buffer `buf`.
///
/// # Returns
///
/// The number of bytes read
///
/// # Possible assignments to `bzerror`
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `b.is_null()`
///     - `buf.is_null()`
///     - `len < 0`
/// - [`BZ_SEQUENCE_ERROR`] if b was opened with [`BZ2_bzWriteOpen`]
/// - [`BZ_IO_ERROR`] if there is an error reading from the compressed file
/// - [`BZ_UNEXPECTED_EOF`] if the compressed data ends before the logical end-of-stream was detected
/// - [`BZ_DATA_ERROR`] if a data integrity error is detected in the compressed stream
/// - [`BZ_DATA_ERROR_MAGIC`] if the compressed stream doesn't begin with the right magic bytes
/// - [`BZ_MEM_ERROR`] if insufficient memory is available
/// - [`BZ_STREAM_END`] if the logical end-of-stream was detected
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `bzerror` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzReadOpen`] or [`BZ2_bzWriteOpen`]
/// * Either
///     - `buf` is `NULL`
///     - `buf` is writable for `len` bytes
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzRead)]
pub unsafe extern "C" fn BZ2_bzRead(
    bzerror: *mut c_int,
    b: *mut BZFILE,
    buf: *mut c_void,
    len: c_int,
) -> c_int {
    BZ2_bzReadHelp(bzerror.as_mut(), b.as_mut(), buf, len)
}

unsafe fn BZ2_bzReadHelp(
    mut bzerror: Option<&mut c_int>,
    mut b: Option<&mut BZFILE>,
    buf: *mut c_void,
    len: c_int,
) -> c_int {
    BZ_SETERR_RAW!(bzerror, b, ReturnCode::BZ_OK);

    let Some(bzf) = b.as_mut() else {
        BZ_SETERR_RAW!(bzerror, b, ReturnCode::BZ_PARAM_ERROR);
        return 0;
    };

    if buf.is_null() || len < 0 {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_PARAM_ERROR);
        return 0;
    }

    if !matches!(bzf.operation, Operation::Reading) {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_SEQUENCE_ERROR);
        return 0;
    }

    if len == 0 as c_int {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_OK);
        return 0;
    }

    bzf.strm.avail_out = len as c_uint;
    bzf.strm.next_out = buf as *mut c_char;
    loop {
        if ferror(bzf.handle) != 0 {
            BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
            return 0;
        }

        if bzf.strm.avail_in == 0 && !myfeof(bzf.handle) {
            let n = fread(
                (bzf.buf).as_mut_ptr() as *mut c_void,
                ::core::mem::size_of::<u8>(),
                5000,
                bzf.handle,
            ) as i32;

            if ferror(bzf.handle) != 0 {
                BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_IO_ERROR);
                return 0;
            }

            bzf.bufN = n;
            bzf.strm.avail_in = bzf.bufN as c_uint;
            bzf.strm.next_in = (bzf.buf).as_mut_ptr().cast::<c_char>();
        }

        match BZ2_bzDecompressHelp(unsafe { BzStream::from_mut(&mut bzf.strm) }) {
            ReturnCode::BZ_OK => {
                if myfeof(bzf.handle) && bzf.strm.avail_in == 0 && bzf.strm.avail_out > 0 {
                    BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_UNEXPECTED_EOF);
                    return 0;
                } else if bzf.strm.avail_out == 0 {
                    BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_OK);
                    return len;
                } else {
                    continue;
                }
            }
            ReturnCode::BZ_STREAM_END => {
                BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_STREAM_END);
                return (len as c_uint - bzf.strm.avail_out) as c_int;
            }
            error => {
                BZ_SETERR!(bzerror, bzf, error);
                return 0;
            }
        }
    }
}

/// Returns data which was read from the compressed file but was not needed to get to the logical end-of-stream.
///
/// # Returns
///
/// - `*unused` is set to the address of the data
/// - `*nUnused` is set to the number of bytes.
///
/// `*nUnused` will be set to a value contained in `0..=BZ_MAX_UNUSED`.
///
/// # Possible assignments to `bzerror`
///
/// - [`BZ_PARAM_ERROR`] if any of
///     - `b.is_null()`
///     - `unused.is_null()`
///     - `nUnused.is_null()`
/// - [`BZ_SEQUENCE_ERROR`] if any of
///     - [`BZ_STREAM_END`] has not been signaled
///     - b was opened with [`BZ2_bzWriteOpen`]
/// - [`BZ_OK`] otherwise
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `bzerror` satisfies the requirements of [`pointer::as_mut`]
/// * `unused` satisfies the requirements of [`pointer::as_mut`]
/// * `nUnused` satisfies the requirements of [`pointer::as_mut`]
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzReadOpen`] or [`BZ2_bzWriteOpen`]
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzReadGetUnused)]
pub unsafe extern "C" fn BZ2_bzReadGetUnused(
    bzerror: *mut c_int,
    b: *mut BZFILE,
    unused: *mut *mut c_void,
    nUnused: *mut c_int,
) {
    BZ2_bzReadGetUnusedHelp(
        bzerror.as_mut(),
        b.as_mut(),
        unused.as_mut(),
        nUnused.as_mut(),
    )
}

unsafe fn BZ2_bzReadGetUnusedHelp(
    mut bzerror: Option<&mut c_int>,
    mut b: Option<&mut BZFILE>,
    unused: Option<&mut *mut c_void>,
    nUnused: Option<&mut c_int>,
) {
    let Some(bzf) = b.as_mut() else {
        BZ_SETERR_RAW!(bzerror, b, ReturnCode::BZ_PARAM_ERROR);
        return;
    };

    if bzf.lastErr != ReturnCode::BZ_STREAM_END {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_SEQUENCE_ERROR);
        return;
    }

    let (Some(unused), Some(nUnused)) = (unused, nUnused) else {
        BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_PARAM_ERROR);
        return;
    };

    BZ_SETERR!(bzerror, bzf, ReturnCode::BZ_OK);

    *nUnused = bzf.strm.avail_in as c_int;
    *unused = bzf.strm.next_in as *mut c_void;
}

#[derive(Copy, Clone)]
pub(crate) enum Operation {
    Reading,
    Writing,
}

enum OpenMode {
    Pointer,
    FileDescriptor(i32),
}

unsafe fn bzopen_or_bzdopen(path: Option<&CStr>, open_mode: OpenMode, mode: &CStr) -> *mut BZFILE {
    let mut bzerr = 0;
    let mut unused: [c_char; BZ_MAX_UNUSED as usize] = [0; BZ_MAX_UNUSED as usize];

    let mut blockSize100k = 9;
    let verbosity = 0;
    let workFactor = 30;
    let nUnused = 0;

    let mut smallMode = false;
    let mut operation = Operation::Reading;

    for c in mode.to_bytes() {
        match c {
            b'r' => operation = Operation::Reading,
            b'w' => operation = Operation::Writing,
            b's' => smallMode = true,
            b'0'..=b'9' => blockSize100k = (*c - b'0') as i32,
            _ => {}
        }
    }

    let mode = match open_mode {
        OpenMode::Pointer => match operation {
            Operation::Reading => b"rbe\0".as_slice(),
            Operation::Writing => b"rbe\0".as_slice(),
        },
        OpenMode::FileDescriptor(_) => match operation {
            Operation::Reading => b"rb\0".as_slice(),
            Operation::Writing => b"rb\0".as_slice(),
        },
    };

    let mode2 = mode.as_ptr().cast_mut().cast::<c_char>();

    let default_file = match operation {
        Operation::Reading => STDIN!(),
        Operation::Writing => STDOUT!(),
    };

    let fp = match open_mode {
        OpenMode::Pointer => match path {
            None => default_file,
            Some(path) if path.is_empty() => default_file,
            Some(path) => fopen(path.as_ptr(), mode2),
        },
        OpenMode::FileDescriptor(fd) => fdopen(fd, mode2),
    };

    if fp.is_null() {
        return ptr::null_mut();
    }

    let bzfp = match operation {
        Operation::Reading => BZ2_bzReadOpen(
            &mut bzerr,
            fp,
            verbosity,
            smallMode as i32,
            unused.as_mut_ptr() as *mut c_void,
            nUnused,
        ),
        Operation::Writing => BZ2_bzWriteOpen(
            &mut bzerr,
            fp,
            blockSize100k.clamp(1, 9),
            verbosity,
            workFactor,
        ),
    };

    if bzfp.is_null() {
        if fp != STDIN!() && fp != STDOUT!() {
            fclose(fp);
        }
        return ptr::null_mut();
    }

    bzfp
}

/// Opens a `.bz2` file for reading or writing using its name. Analogous to [`libc::fopen`].
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `path` is `NULL`
///     - `path` is a null-terminated sequence of bytes
/// * Either
///     - `mode` is `NULL`
///     - `mode` is a null-terminated sequence of bytes
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzopen)]
pub unsafe extern "C" fn BZ2_bzopen(path: *const c_char, mode: *const c_char) -> *mut BZFILE {
    let mode = if mode.is_null() {
        return ptr::null_mut();
    } else {
        CStr::from_ptr(mode)
    };

    let path = if path.is_null() {
        None
    } else {
        Some(CStr::from_ptr(path))
    };

    bzopen_or_bzdopen(path, OpenMode::Pointer, mode)
}

/// Opens a `.bz2` file for reading or writing using a pre-existing file descriptor. Analogous to [`libc::fdopen`].
///
/// # Safety
///
/// The caller must guarantee that
///
/// * `fd` must be a valid file descriptor for the duration of [`BZ2_bzdopen`]
/// * Either
///     - `mode` is `NULL`
///     - `mode` is a null-terminated sequence of bytes
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzdopen)]
pub unsafe extern "C" fn BZ2_bzdopen(fd: c_int, mode: *const c_char) -> *mut BZFILE {
    let mode = if mode.is_null() {
        return ptr::null_mut();
    } else {
        CStr::from_ptr(mode)
    };

    bzopen_or_bzdopen(None, OpenMode::FileDescriptor(fd), mode)
}

/// Reads up to `len` (uncompressed) bytes from the compressed file `b` into the buffer `buf`.
///
/// Analogous to [`libc::fread`].
///
/// # Returns
///
/// Number of bytes read on success, or `-1` on failure.
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzWriteOpen`] or [`BZ2_bzReadOpen`]
/// * Either
///     - `buf` is `NULL`
///     - `buf` is writable for `len` bytes
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzread)]
pub unsafe extern "C" fn BZ2_bzread(b: *mut BZFILE, buf: *mut c_void, len: c_int) -> c_int {
    BZ2_bzreadHelp(b.as_mut(), buf, len)
}

unsafe fn BZ2_bzreadHelp(mut b: Option<&mut BZFILE>, buf: *mut c_void, len: c_int) -> c_int {
    let mut bzerr = 0;

    if let Some(b) = b.as_deref_mut() {
        if b.lastErr == ReturnCode::BZ_STREAM_END {
            return 0;
        }
    }

    let nread = BZ2_bzReadHelp(Some(&mut bzerr), b, buf, len);
    if bzerr == 0 || bzerr == ReturnCode::BZ_STREAM_END as i32 {
        nread
    } else {
        -1
    }
}

/// Absorbs `len` bytes from the buffer `buf`, eventually to be compressed and written to the file.
///
/// Analogous to [`libc::fwrite`].
///
/// # Returns
///
/// The value `len` on success, or `-1` on failure.
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzWriteOpen`] or [`BZ2_bzReadOpen`]
/// * Either
///     - `buf` is `NULL`
///     - `buf` is readable for `len` bytes
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzwrite)]
pub unsafe extern "C" fn BZ2_bzwrite(b: *mut BZFILE, buf: *const c_void, len: c_int) -> c_int {
    BZ2_bzwriteHelp(b.as_mut(), buf, len)
}

unsafe fn BZ2_bzwriteHelp(b: Option<&mut BZFILE>, buf: *const c_void, len: c_int) -> c_int {
    let mut bzerr = 0;
    BZ2_bzWriteHelp(Some(&mut bzerr), b, buf, len);

    match bzerr {
        0 => len,
        _ => -1,
    }
}

/// Flushes a [`BZFILE`].
///
/// Analogous to [`libc::fflush`].
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzReadOpen`] or [`BZ2_bzWriteOpen`]
#[export_name = prefix!(BZ2_bzflush)]
pub unsafe extern "C" fn BZ2_bzflush(mut _b: *mut BZFILE) -> c_int {
    /* do nothing now... */
    0
}

/// Closes a [`BZFILE`].
///
/// Analogous to [`libc::fclose`].
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzReadOpen`] or [`BZ2_bzWriteOpen`]
#[export_name = prefix!(BZ2_bzclose)]
pub unsafe extern "C" fn BZ2_bzclose(b: *mut BZFILE) {
    BZ2_bzcloseHelp(b.as_mut())
}

unsafe fn BZ2_bzcloseHelp(mut b: Option<&mut BZFILE>) {
    let mut bzerr: c_int = 0;

    let operation = if let Some(bzf) = &mut b {
        bzf.operation
    } else {
        return;
    };

    match operation {
        Operation::Reading => {
            BZ2_bzReadCloseHelp(Some(&mut bzerr), b.as_deref_mut());
        }
        Operation::Writing => {
            BZ2_bzWriteCloseHelp(Some(&mut bzerr), b.as_deref_mut(), false as i32, None, None);
            if bzerr != 0 {
                BZ2_bzWriteCloseHelp(None, b.as_deref_mut(), true as i32, None, None);
            }
        }
    }

    if let Some(bzf) = b {
        if bzf.handle != STDIN!() && bzf.handle != STDOUT!() {
            fclose(bzf.handle);
        }
    }
}

const BZERRORSTRINGS: [&str; 16] = [
    "OK\0",
    "SEQUENCE_ERROR\0",
    "PARAM_ERROR\0",
    "MEM_ERROR\0",
    "DATA_ERROR\0",
    "DATA_ERROR_MAGIC\0",
    "IO_ERROR\0",
    "UNEXPECTED_EOF\0",
    "OUTBUFF_FULL\0",
    "CONFIG_ERROR\0",
    "???\0",
    "???\0",
    "???\0",
    "???\0",
    "???\0",
    "???\0",
];

/// Describes the most recent error.
///
/// # Returns
///
/// A null-terminated string describing the most recent error status of `b`, and also sets `*errnum` to its numerical value.
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `b` is `NULL`
///     - `b` is initialized with [`BZ2_bzReadOpen`] or [`BZ2_bzWriteOpen`]
/// * `errnum` satisfies the requirements of [`pointer::as_mut`]
///
/// [`pointer::as_mut`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut
#[export_name = prefix!(BZ2_bzerror)]
pub unsafe extern "C" fn BZ2_bzerror(b: *const BZFILE, errnum: *mut c_int) -> *const c_char {
    BZ2_bzerrorHelp(
        b.as_ref().expect("Passed null pointer to BZ2_bzerror"),
        errnum.as_mut(),
    )
}

fn BZ2_bzerrorHelp(b: &BZFILE, errnum: Option<&mut c_int>) -> *const c_char {
    let err = Ord::min(0, b.lastErr as c_int);
    if let Some(errnum) = errnum {
        *errnum = err;
    };
    let msg = match BZERRORSTRINGS.get(-err as usize) {
        Some(msg) => msg,
        None => "???\0",
    };
    msg.as_ptr().cast::<c_char>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_messages() {
        let mut bz_file = BZFILE {
            handle: core::ptr::null_mut(),
            buf: [0; 5000],
            bufN: 0,
            strm: bz_stream::zeroed(),
            lastErr: ReturnCode::BZ_OK,
            operation: Operation::Reading,
            initialisedOk: false,
        };

        let return_codes = [
            ReturnCode::BZ_OK,
            ReturnCode::BZ_RUN_OK,
            ReturnCode::BZ_FLUSH_OK,
            ReturnCode::BZ_FINISH_OK,
            ReturnCode::BZ_STREAM_END,
            ReturnCode::BZ_SEQUENCE_ERROR,
            ReturnCode::BZ_PARAM_ERROR,
            ReturnCode::BZ_MEM_ERROR,
            ReturnCode::BZ_DATA_ERROR,
            ReturnCode::BZ_DATA_ERROR_MAGIC,
            ReturnCode::BZ_IO_ERROR,
            ReturnCode::BZ_UNEXPECTED_EOF,
            ReturnCode::BZ_OUTBUFF_FULL,
            ReturnCode::BZ_CONFIG_ERROR,
        ];

        for return_code in return_codes {
            bz_file.lastErr = return_code;

            let mut errnum = 0;
            let ptr = unsafe { BZ2_bzerror(&bz_file as *const BZFILE, &mut errnum) };
            assert!(!ptr.is_null());
            let cstr = unsafe { CStr::from_ptr(ptr) };

            let msg = cstr.to_str().unwrap();

            let expected = match return_code {
                ReturnCode::BZ_OK => "OK",
                ReturnCode::BZ_RUN_OK => "OK",
                ReturnCode::BZ_FLUSH_OK => "OK",
                ReturnCode::BZ_FINISH_OK => "OK",
                ReturnCode::BZ_STREAM_END => "OK",
                ReturnCode::BZ_SEQUENCE_ERROR => "SEQUENCE_ERROR",
                ReturnCode::BZ_PARAM_ERROR => "PARAM_ERROR",
                ReturnCode::BZ_MEM_ERROR => "MEM_ERROR",
                ReturnCode::BZ_DATA_ERROR => "DATA_ERROR",
                ReturnCode::BZ_DATA_ERROR_MAGIC => "DATA_ERROR_MAGIC",
                ReturnCode::BZ_IO_ERROR => "IO_ERROR",
                ReturnCode::BZ_UNEXPECTED_EOF => "UNEXPECTED_EOF",
                ReturnCode::BZ_OUTBUFF_FULL => "OUTBUFF_FULL",
                ReturnCode::BZ_CONFIG_ERROR => "CONFIG_ERROR",
            };

            assert_eq!(msg, expected);

            if (return_code as i32) < 0 {
                assert_eq!(return_code as i32, errnum);
            } else {
                assert_eq!(0, errnum);
            }
        }
    }
}
