use std;
use std::io::{self, Cursor, ErrorKind, Read};
use std::str;

use rmp::decode::{read_marker, RmpRead};
use rmp::Marker;

use super::Error;
use crate::{Utf8StringRef, ValueRef};

fn read_str_data<'a, R>(rd: &mut R, len: usize, depth: u16) -> Result<Utf8StringRef<'a>, Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;
    let buf = read_bin_data(rd, len, depth)?;
    match str::from_utf8(buf) {
        Ok(s) => Ok(Utf8StringRef::from(s)),
        Err(err) => {
            let s = Utf8StringRef {
                s: Err((buf, err)),
            };
            Ok(s)
        }
    }
}

fn read_bin_data<'a, R>(rd: &mut R, len: usize, depth: u16) -> Result<&'a [u8], Error>
    where R: BorrowRead<'a>
{
    let _depth = super::decrement_depth(depth)?;
    let buf = rd.fill_buf();

    if len > buf.len() {
        return Err(Error::InvalidDataRead(io::Error::new(ErrorKind::UnexpectedEof, "unexpected EOF")));
    }

    // Take a slice.
    let buf = &buf[..len];
    rd.consume(len);

    Ok(buf)
}

fn read_ext_body<'a, R>(rd: &mut R, len: usize, depth: u16) -> Result<(i8, &'a [u8]), Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;
    let ty = rd.read_data_i8()?;
    let buf = read_bin_data(rd, len, depth)?;

    Ok((ty, buf))
}

fn read_array_data<'a, R>(rd: &mut R, mut len: usize, depth: u16) -> Result<Vec<ValueRef<'a>>, Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;
    // Note: Do not preallocate a Vec of size `len`.
    // See https://github.com/3Hren/msgpack-rust/issues/151
    let mut vec = Vec::new();

    while len > 0 {
        vec.push(read_value_ref_inner(rd, depth)?);
        len -= 1;
    }

    Ok(vec)
}

fn read_map_data<'a, R>(rd: &mut R, mut len: usize, depth: u16) -> Result<Vec<(ValueRef<'a>, ValueRef<'a>)>, Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;
    // Note: Do not preallocate a Vec of size `len`.
    // See https://github.com/3Hren/msgpack-rust/issues/151
    let mut vec = Vec::new();

    while len > 0 {
        vec.push((read_value_ref_inner(rd, depth)?, read_value_ref_inner(rd, depth)?));
        len -= 1;
    }

    Ok(vec)
}

/// A `BorrowRead` is a type of Reader which has an internal buffer.
///
/// This magic trait acts like a standard `BufRead` but unlike the standard this has an explicit
/// internal buffer lifetime, which allows to borrow from underlying buffer while consuming bytes.
pub trait BorrowRead<'a>: Read {
    /// Returns the buffer contents.
    ///
    /// This function is a lower-level call. It needs to be paired with the consume method to
    /// function properly. When calling this method, none of the contents will be "read" in the
    /// sense that later calling read may return the same contents. As such, consume must be called
    /// with the number of bytes that are consumed from this buffer to ensure that the bytes are
    /// never returned twice.
    ///
    /// An empty buffer returned indicates that the stream has reached EOF.
    fn fill_buf(&self) -> &'a [u8];

    /// Tells this buffer that len bytes have been consumed from the buffer, so they should no
    /// longer be returned in calls to read.
    fn consume(&mut self, len: usize);
}

impl<'a> BorrowRead<'a> for &'a [u8] {
    fn fill_buf(&self) -> &'a [u8] {
        self
    }

    fn consume(&mut self, len: usize) {
        *self = &(*self)[len..];
    }
}

/// Useful when you want to know how much bytes has been consumed during `ValueRef` decoding.
impl<'a> BorrowRead<'a> for Cursor<&'a [u8]> {
    fn fill_buf(&self) -> &'a [u8] {
        let len = std::cmp::min(self.position(), self.get_ref().len() as u64);
        &self.get_ref()[len as usize..]
    }

    fn consume(&mut self, len: usize) {
        let pos = self.position();
        self.set_position(pos + len as u64);
    }
}

fn read_value_ref_inner<'a, R>(rd: &mut R, depth: u16) -> Result<ValueRef<'a>, Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;

    // Reading the marker involves either 1 byte read or nothing. On success consumes strictly
    // 1 byte from the `rd`.
    let val = match read_marker(rd)? {
        Marker::Null => ValueRef::Nil,
        Marker::True => ValueRef::Boolean(true),
        Marker::False => ValueRef::Boolean(false),
        Marker::FixPos(val) => ValueRef::from(val),
        Marker::FixNeg(val) => ValueRef::from(val),
        Marker::U8 => ValueRef::from(rd.read_data_u8()?),
        Marker::U16 => ValueRef::from(rd.read_data_u16()?),
        Marker::U32 => ValueRef::from(rd.read_data_u32()?),
        Marker::U64 => ValueRef::from(rd.read_data_u64()?),
        Marker::I8 => ValueRef::from(rd.read_data_i8()?),
        Marker::I16 => ValueRef::from(rd.read_data_i16()?),
        Marker::I32 => ValueRef::from(rd.read_data_i32()?),
        Marker::I64 => ValueRef::from(rd.read_data_i64()?),
        Marker::F32 => ValueRef::F32(rd.read_data_f32()?),
        Marker::F64 => ValueRef::F64(rd.read_data_f64()?),
        Marker::FixStr(len) => {
            let res = read_str_data(rd, len as usize, depth)?;
            ValueRef::String(res)
        }
        Marker::Str8 => {
            let len = rd.read_data_u8()?;
            let res = read_str_data(rd, len as usize, depth)?;
            ValueRef::String(res)
        }
        Marker::Str16 => {
            let len = rd.read_data_u16()?;
            let res = read_str_data(rd, len as usize, depth)?;
            ValueRef::String(res)
        }
        Marker::Str32 => {
            let len = rd.read_data_u32()?;
            let res = read_str_data(rd, len as usize, depth)?;
            ValueRef::String(res)
        }
        Marker::Bin8 => {
            let len = rd.read_data_u8()?;
            let res = read_bin_data(rd, len as usize, depth)?;
            ValueRef::Binary(res)
        }
        Marker::Bin16 => {
            let len = rd.read_data_u16()?;
            let res = read_bin_data(rd, len as usize, depth)?;
            ValueRef::Binary(res)
        }
        Marker::Bin32 => {
            let len = rd.read_data_u32()?;
            let res = read_bin_data(rd, len as usize, depth)?;
            ValueRef::Binary(res)
        }
        Marker::FixArray(len) => {
            let vec = read_array_data(rd, len as usize, depth)?;
            ValueRef::Array(vec)
        }
        Marker::Array16 => {
            let len = rd.read_data_u16()?;
            let vec = read_array_data(rd, len as usize, depth)?;
            ValueRef::Array(vec)
        }
        Marker::Array32 => {
            let len = rd.read_data_u32()?;
            let vec = read_array_data(rd, len as usize, depth)?;
            ValueRef::Array(vec)
        }
        Marker::FixMap(len) => {
            let map = read_map_data(rd, len as usize, depth)?;
            ValueRef::Map(map)
        }
        Marker::Map16 => {
            let len = rd.read_data_u16()?;
            let map = read_map_data(rd, len as usize, depth)?;
            ValueRef::Map(map)
        }
        Marker::Map32 => {
            let len = rd.read_data_u32()?;
            let map = read_map_data(rd, len as usize, depth)?;
            ValueRef::Map(map)
        }
        Marker::FixExt1 => {
            let len = 1;
            let (ty, vec) = read_ext_body(rd, len as usize, depth)?;
            ValueRef::Ext(ty, vec)
        }
        Marker::FixExt2 => {
            let len = 2;
            let (ty, vec) = read_ext_body(rd, len as usize, depth)?;
            ValueRef::Ext(ty, vec)
        }
        Marker::FixExt4 => {
            let len = 4;
            let (ty, vec) = read_ext_body(rd, len as usize, depth)?;
            ValueRef::Ext(ty, vec)
        }
        Marker::FixExt8 => {
            let len = 8;
            let (ty, vec) = read_ext_body(rd, len as usize, depth)?;
            ValueRef::Ext(ty, vec)
        }
        Marker::FixExt16 => {
            let len = 16;
            let (ty, vec) = read_ext_body(rd, len as usize, depth)?;
            ValueRef::Ext(ty, vec)
        }
        Marker::Ext8 => {
            let len = rd.read_data_u8()?;
            let (ty, vec) = read_ext_body(rd, len as usize, depth)?;
            ValueRef::Ext(ty, vec)
        }
        Marker::Ext16 => {
            let len = rd.read_data_u16()?;
            let (ty, vec) = read_ext_body(rd, len as usize, depth)?;
            ValueRef::Ext(ty, vec)
        }
        Marker::Ext32 => {
            let len = rd.read_data_u32()?;
            let (ty, vec) = read_ext_body(rd, len as usize, depth)?;
            ValueRef::Ext(ty, vec)
        }
        Marker::Reserved => ValueRef::Nil,
    };

    Ok(val)
}

/// Attempts to read the data from the given reader until either a complete MessagePack value
/// decoded or an error detected.
///
/// Returns either a non-owning `ValueRef`, which borrows the buffer from the given reader or an
/// error.
///
/// The reader should meet the requirement of a special `BorrowRead` trait, which allows to mutate
/// itself but permits to mutate the buffer it contains. It allows to perform a completely
/// zero-copy reading without a data loss fear in case of an error.
///
/// Currently only two types fit in this requirement: `&[u8]` and `Cursor<&[u8]>`. Using Cursor is
/// helpful, when you need to know how exactly many bytes the decoded `ValueRef` consumes. A `Vec<u8>`
/// type doesn't fit in the `BorrowRead` requirement, because its mut reference can mutate the
/// underlying buffer - use `Vec::as_slice()` if you need to decode a value from the vector.
///
/// # Errors
///
/// Returns an `Error` value if unable to continue the decoding operation either because of read
/// failure or any other circumstances. See `Error` documentation for more information.
///
/// This function enforces a maximum recursion depth of [`MAX_DEPTH`](super::MAX_DEPTH) and returns
/// [`Error::DepthLimitExceeded`] if the maximum is hit. If you run into stack overflows despite
/// this, use [`read_value_ref_with_max_depth`] with a custom maximum depth.
///
/// # Examples
/// ```
/// use rmpv::ValueRef;
/// use rmpv::decode::read_value_ref;
///
/// let buf = [0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];
/// let mut rd = &buf[..];
///
/// assert_eq!(ValueRef::from("le message"), read_value_ref(&mut rd).unwrap());
/// ```
#[inline(never)]
pub fn read_value_ref<'a, R>(rd: &mut R) -> Result<ValueRef<'a>, Error>
    where R: BorrowRead<'a>
{
    read_value_ref_inner(rd, super::MAX_DEPTH as _)
}

/// Attempts to read the data from the given reader until either a complete MessagePack value
/// decoded or an error detected.
///
/// Returns either a non-owning `ValueRef`, which borrows the buffer from the given reader or an
/// error.
///
/// See [`read_value_ref`] for more information on how to use this function. This variant allows
/// you to specify the maximum recursion depth, if [`MAX_DEPTH`](super::MAX_DEPTH) is too high.
///
/// # Errors
///
/// Same as [`read_value_ref`], using the `max_depth` parameter in place of
/// [`MAX_DEPTH`](super::MAX_DEPTH).
#[inline(never)]
pub fn read_value_ref_with_max_depth<'a, R>(rd: &mut R, max_depth: usize) -> Result<ValueRef<'a>, Error>
    where R: BorrowRead<'a>
{
    read_value_ref_inner(rd, max_depth.min(u16::MAX as _) as u16)
}
