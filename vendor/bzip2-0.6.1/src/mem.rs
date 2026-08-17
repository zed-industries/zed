//! Raw low-level manipulations of bz streams.

use std::error;
use std::fmt;
use std::marker;
use std::mem;

use core::ffi::{c_int, c_uint};

use crate::{ffi, Compression};

/// Representation of an in-memory compression stream.
///
/// An instance of [`Compress`] can be used to compress a stream of bz2 data.
pub struct Compress {
    inner: Stream<DirCompress>,
}

/// Representation of an in-memory decompression stream.
///
/// An instance of [`Decompress`] can be used to decompress a stream of bz2-encoded
/// data.
pub struct Decompress {
    inner: Stream<DirDecompress>,
}

struct Stream<D: Direction> {
    // libbz2 requires a stable address for this stream.
    raw: Box<ffi::bz_stream>,
    _marker: marker::PhantomData<D>,
}

unsafe impl<D: Direction> Send for Stream<D> {}
unsafe impl<D: Direction> Sync for Stream<D> {}

trait Direction {
    unsafe fn destroy(stream: *mut ffi::bz_stream) -> c_int;
}

enum DirCompress {}
enum DirDecompress {}

/// Possible actions to take on compression.
#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub enum Action {
    /// Normal compression.
    Run = ffi::BZ_RUN as isize,
    /// Flush any existing output, but do not read any more input
    Flush = ffi::BZ_FLUSH as isize,
    /// Request that the compression stream be finalized.
    Finish = ffi::BZ_FINISH as isize,
}

/// Result of compression or decompression
#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub enum Status {
    /// Decompression went fine, nothing much to report.
    Ok,

    /// The Flush action on a compression went ok.
    FlushOk,

    /// The Run action on compression went ok.
    RunOk,

    /// The Finish action on compression went ok.
    FinishOk,

    /// The stream's end has been met, meaning that no more data can be input.
    StreamEnd,

    /// There was insufficient memory in the input or output buffer to complete
    /// the request, but otherwise everything went normally.
    MemNeeded,
}

/// Fatal errors encountered when compressing/decompressing bytes.
///
/// These errors indicate that progress could not be made in any form due to
/// input or output parameters.
#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub enum Error {
    /// The sequence of operations called on a decompression/compression stream
    /// were invalid. See methods for details.
    Sequence,

    /// The data being decompressed was invalid, or it was not a valid bz2
    /// stream.
    Data,

    /// The magic bz2 header wasn't present when decompressing.
    DataMagic,

    /// The parameters to this function were invalid.
    Param,
}

impl Compress {
    /// Creates a new stream prepared for compression.
    ///
    /// The `work_factor` parameter controls how the compression phase behaves
    /// when presented with worst case, highly repetitive, input data. If
    /// compression runs into difficulties caused by repetitive data, the
    /// library switches from the standard sorting algorithm to a fallback
    /// algorithm. The fallback is slower than the standard algorithm by perhaps
    /// a factor of three, but always behaves reasonably, no matter how bad the
    /// input.
    ///
    /// Lower values of `work_factor` reduce the amount of effort the standard
    /// algorithm will expend before resorting to the fallback. You should set
    /// this parameter carefully; too low, and many inputs will be handled by
    /// the fallback algorithm and so compress rather slowly, too high, and your
    /// average-to-worst case compression times can become very large. The
    /// default value of 30 gives reasonable behaviour over a wide range of
    /// circumstances.
    ///
    /// Allowable values range from 0 to 250 inclusive. 0 is a special case,
    /// equivalent to using the default value of 30.
    pub fn new(lvl: Compression, work_factor: u32) -> Self {
        unsafe {
            let mut raw = Box::new(mem::zeroed());
            assert_eq!(
                ffi::BZ2_bzCompressInit(&mut *raw, lvl.level() as c_int, 0, work_factor as c_int),
                0
            );
            Self {
                inner: Stream {
                    raw,
                    _marker: marker::PhantomData,
                },
            }
        }
    }

    unsafe fn compress_inner(
        &mut self,
        input: &[u8],
        output_ptr: *mut u8,
        output_len: usize,
        action: Action,
    ) -> Result<Status, Error> {
        // apparently 0-length compression requests which don't actually make
        // any progress are returned as BZ_PARAM_ERROR, which we don't want, to
        // just translate to a success here.
        if input.is_empty() && action == Action::Run {
            return Ok(Status::RunOk);
        }
        self.inner.raw.next_in = input.as_ptr() as *mut _;
        self.inner.raw.avail_in = input.len().min(c_uint::MAX as usize) as c_uint;
        self.inner.raw.next_out = output_ptr as *mut _;
        self.inner.raw.avail_out = output_len.min(c_uint::MAX as usize) as c_uint;
        unsafe {
            match ffi::BZ2_bzCompress(&mut *self.inner.raw, action as c_int) {
                ffi::BZ_RUN_OK => Ok(Status::RunOk),
                ffi::BZ_FLUSH_OK => Ok(Status::FlushOk),
                ffi::BZ_FINISH_OK => Ok(Status::FinishOk),
                ffi::BZ_STREAM_END => Ok(Status::StreamEnd),
                ffi::BZ_SEQUENCE_ERROR => Err(Error::Sequence),
                c => panic!("unknown return status: {c}"),
            }
        }
    }

    /// Compress a block of input into a block of output.
    ///
    /// If anything other than [`BZ_OK`] is seen, `Err` is returned.
    ///
    /// The action given must be one of [`Action::Run`], [`Action::Flush`] or [`Action::Finish`].
    ///
    /// [`BZ_OK`]: ffi::BZ_OK
    pub fn compress(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        action: Action,
    ) -> Result<Status, Error> {
        unsafe { self.compress_inner(input, output.as_mut_ptr(), output.len(), action) }
    }

    /// Same as [`Self::compress`] but accepts an uninitialised `output` buffer.
    pub fn compress_uninit(
        &mut self,
        input: &[u8],
        output: &mut [mem::MaybeUninit<u8>],
        action: Action,
    ) -> Result<Status, Error> {
        unsafe { self.compress_inner(input, output.as_mut_ptr() as *mut _, output.len(), action) }
    }

    /// Compress a block of input into an output vector.
    ///
    /// This function will not grow `output`, but it will fill the space after
    /// its current length up to its capacity. The length of the vector will be
    /// adjusted appropriately.
    pub fn compress_vec(
        &mut self,
        input: &[u8],
        output: &mut Vec<u8>,
        action: Action,
    ) -> Result<Status, Error> {
        let len = output.len();

        unsafe {
            let before = self.total_out();
            let ret = self.compress_uninit(input, output.spare_capacity_mut(), action);
            output.set_len((self.total_out() - before) as usize + len);

            ret
        }
    }

    /// Total number of bytes processed as input
    pub fn total_in(&self) -> u64 {
        self.inner.total_in()
    }

    /// Total number of bytes processed as output
    pub fn total_out(&self) -> u64 {
        self.inner.total_out()
    }
}

impl Decompress {
    /// Creates a new stream prepared for decompression.
    ///
    /// If `small` is true, then the library will use an alternative
    /// decompression algorithm which uses less memory but at the cost of
    /// decompressing more slowly (roughly speaking, half the speed, but the
    /// maximum memory requirement drops to around 2300k).
    pub fn new(small: bool) -> Self {
        unsafe {
            let mut raw = Box::new(mem::zeroed());
            assert_eq!(ffi::BZ2_bzDecompressInit(&mut *raw, 0, small as c_int), 0);
            Self {
                inner: Stream {
                    raw,
                    _marker: marker::PhantomData,
                },
            }
        }
    }

    unsafe fn decompress_inner(
        &mut self,
        input: &[u8],
        output_ptr: *mut u8,
        output_len: usize,
    ) -> Result<Status, Error> {
        self.inner.raw.next_in = input.as_ptr() as *mut _;
        self.inner.raw.avail_in = input.len().min(c_uint::MAX as usize) as c_uint;
        self.inner.raw.next_out = output_ptr as *mut _;
        self.inner.raw.avail_out = output_len.min(c_uint::MAX as usize) as c_uint;
        unsafe {
            match ffi::BZ2_bzDecompress(&mut *self.inner.raw) {
                ffi::BZ_OK => Ok(Status::Ok),
                ffi::BZ_MEM_ERROR => Ok(Status::MemNeeded),
                ffi::BZ_STREAM_END => Ok(Status::StreamEnd),
                ffi::BZ_PARAM_ERROR => Err(Error::Param),
                ffi::BZ_DATA_ERROR => Err(Error::Data),
                ffi::BZ_DATA_ERROR_MAGIC => Err(Error::DataMagic),
                ffi::BZ_SEQUENCE_ERROR => Err(Error::Sequence),
                c => panic!("wut: {c}"),
            }
        }
    }

    /// Decompress a block of input into a block of output.
    pub fn decompress(&mut self, input: &[u8], output: &mut [u8]) -> Result<Status, Error> {
        unsafe { self.decompress_inner(input, output.as_mut_ptr(), output.len()) }
    }

    /// Same as [`Self::decompress`] but accepts an uninitialized buffer.
    pub fn decompress_uninit(
        &mut self,
        input: &[u8],
        output: &mut [mem::MaybeUninit<u8>],
    ) -> Result<Status, Error> {
        unsafe { self.decompress_inner(input, output.as_mut_ptr() as *mut _, output.len()) }
    }

    /// Decompress a block of input into an output vector.
    ///
    /// This function will not grow `output`, but it will fill the space after
    /// its current length up to its capacity. The length of the vector will be
    /// adjusted appropriately.
    pub fn decompress_vec(&mut self, input: &[u8], output: &mut Vec<u8>) -> Result<Status, Error> {
        let len = output.len();

        unsafe {
            let before = self.total_out();
            let ret = self.decompress_uninit(input, output.spare_capacity_mut());
            output.set_len((self.total_out() - before) as usize + len);

            ret
        }
    }

    /// Total number of bytes processed as input
    pub fn total_in(&self) -> u64 {
        self.inner.total_in()
    }

    /// Total number of bytes processed as output
    pub fn total_out(&self) -> u64 {
        self.inner.total_out()
    }
}

impl<D: Direction> Stream<D> {
    fn total_in(&self) -> u64 {
        (self.raw.total_in_lo32 as u64) | ((self.raw.total_in_hi32 as u64) << 32)
    }

    fn total_out(&self) -> u64 {
        (self.raw.total_out_lo32 as u64) | ((self.raw.total_out_hi32 as u64) << 32)
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            Self::Sequence => "bzip2: sequence of operations invalid",
            Self::Data => "bzip2: invalid data",
            Self::DataMagic => "bzip2: bz2 header missing",
            Self::Param => "bzip2: invalid parameters",
        };
        f.write_str(description)
    }
}

impl From<Error> for std::io::Error {
    fn from(data: Error) -> Self {
        Self::other(data)
    }
}

impl Direction for DirCompress {
    unsafe fn destroy(stream: *mut ffi::bz_stream) -> c_int {
        ffi::BZ2_bzCompressEnd(stream)
    }
}
impl Direction for DirDecompress {
    unsafe fn destroy(stream: *mut ffi::bz_stream) -> c_int {
        ffi::BZ2_bzDecompressEnd(stream)
    }
}

impl<D: Direction> Drop for Stream<D> {
    fn drop(&mut self) {
        unsafe {
            let _ = D::destroy(&mut *self.raw);
        }
    }
}
