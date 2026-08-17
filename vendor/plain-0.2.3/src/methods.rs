
use core::{mem, slice};

use {Error, Plain};

/// Check if a byte slice is aligned suitably for type T.
#[inline]
pub fn is_aligned<T>(bytes: &[u8]) -> bool {
    ((bytes.as_ptr() as usize) % mem::align_of::<T>()) == 0
}

#[inline(always)]
fn check_alignment<T>(bytes: &[u8]) -> Result<(), Error> {
    if is_aligned::<T>(bytes) {
        Ok(())
    } else {
        Err(Error::BadAlignment)
    }
}

#[inline(always)]
fn check_length<T>(bytes: &[u8], len: usize) -> Result<(), Error> {
    if mem::size_of::<T>() > 0 && (bytes.len() / mem::size_of::<T>()) < len {
        Err(Error::TooShort)
    } else {
        Ok(())
    }
}

/// Interpret data as bytes. Not safe for data with padding.
#[inline(always)]
pub unsafe fn as_bytes<S>(s: &S) -> &[u8]
where
    S: ?Sized,
{
    let bptr = s as *const S as *const u8;
    let bsize = mem::size_of_val(s);
    slice::from_raw_parts(bptr, bsize)
}

/// Interpret data as mutable bytes.
/// Reading is not safe for data with padding. Writing is ok.
#[inline(always)]
pub unsafe fn as_mut_bytes<S>(s: &mut S) -> &mut [u8]
where
    S: Plain + ?Sized,
{
    let bptr = s as *mut S as *mut u8;
    let bsize = mem::size_of_val(s);
    slice::from_raw_parts_mut(bptr, bsize)
}

/// Safely converts a byte slice to a reference.
///
/// However, if the byte slice is not long enough
/// to contain target type, or if it doesn't
/// satisfy the type's alignment requirements,
/// the function returns an error.
///
/// The function will not fail when the
/// byte slice is longer than necessary, since it is
/// a common practice to interpret the beginning of
/// a slice as a fixed-size header.
///
/// In many cases it is preferrable to allocate
/// a value/slice of the target type and use
/// [`copy_from_bytes()`](fn.copy_from_bytes.html) to copy
/// data instead. That way, any issues with alignment
/// are implicitly avoided.
///
#[inline]
pub fn from_bytes<T>(bytes: &[u8]) -> Result<&T, Error>
where
    T: Plain,
{
    try!(check_alignment::<T>(bytes));
    try!(check_length::<T>(bytes, 1));
    Ok(unsafe { &*(bytes.as_ptr() as *const T) })
}

/// Similar to [`from_bytes()`](fn.from_bytes.html),
/// except that the output is a slice of T, instead
/// of a reference to a single T. All concerns about
/// alignment also apply here, but size is handled
/// differently.
///
/// The result slice's length is set to be
/// `bytes.len() / size_of::<T>()`, and there
/// are no requirements for input size. I.e.
/// the result may be empty slice, and the input
/// slice doesn't necessarily have to end on `T`'s
/// boundary. The latter has pragmatic reasons: If the
/// length of the array is not known in advance,
/// e.g. if it's terminated by a special element,
/// it's perfectly legal to turn the whole rest
/// of data into `&[T]` and set the proper length
/// after inspecting the array.
///
/// In many cases it is preferrable to allocate
/// a value/slice of the target type and use
/// [`copy_from_bytes()`](fn.copy_from_bytes.html) to copy
/// data instead. That way, any issues with alignment
/// are implicitly avoided.
///
#[inline]
pub fn slice_from_bytes<T>(bytes: &[u8]) -> Result<&[T], Error>
where
    T: Plain,
{
    let len = bytes.len() / mem::size_of::<T>();
    slice_from_bytes_len(bytes, len)
}


/// Same as [`slice_from_bytes()`](fn.slice_from_bytes.html),
/// except that it takes explicit length of the result slice.
///
/// If the input slice cannot satisfy the length, returns error.
/// The input slice is allowed to be longer than necessary.
///
#[inline]
pub fn slice_from_bytes_len<T>(bytes: &[u8], len: usize) -> Result<&[T], Error>
where
    T: Plain,
{
    try!(check_alignment::<T>(bytes));
    try!(check_length::<T>(bytes, len));
    Ok(unsafe {
        slice::from_raw_parts(bytes.as_ptr() as *const T, len)
    })
}

/// See [`from_bytes()`](fn.from_bytes.html).
///
/// Does the same, except with mutable references.
///
#[inline]
pub fn from_mut_bytes<T>(bytes: &mut [u8]) -> Result<&mut T, Error>
where
    T: Plain,
{
    try!(check_alignment::<T>(bytes));
    try!(check_length::<T>(bytes, 1));
    Ok(unsafe { &mut *(bytes.as_mut_ptr() as *mut T) })
}

/// See [`slice_from_bytes()`](fn.slice_from_bytes.html).
///
/// Does the same, except with mutable references.
///
#[inline]
pub fn slice_from_mut_bytes<T>(bytes: &mut [u8]) -> Result<&mut [T], Error>
where
    T: Plain,
{
    let len = bytes.len() / mem::size_of::<T>();
    slice_from_mut_bytes_len(bytes, len)
}

/// See [`slice_from_bytes_len()`](fn.slice_from_bytes_len.html).
///
/// Does the same, except with mutable references.
///
#[inline]
pub fn slice_from_mut_bytes_len<T>(bytes: &mut [u8], len: usize) -> Result<&mut [T], Error>
where
    T: Plain,
{
    try!(check_alignment::<T>(bytes));
    try!(check_length::<T>(bytes, len));
    Ok(unsafe {
        slice::from_raw_parts_mut(bytes.as_ptr() as *mut T, len)
    })
}

/// Copies data from a byte slice into existing memory.
/// Suitable when [`from_bytes()`](fn.from_bytes.html) would normally
/// be used, but the data is not aligned properly in memory.
///
/// For an example how to use it, see crate-level documentation.
///
#[inline]
pub fn copy_from_bytes<T>(into: &mut T, bytes: &[u8]) -> Result<(), Error>
where
    T: Plain + ?Sized,
{
    let sz = mem::size_of_val(into);

    if bytes.len() < sz {
        return Err(Error::TooShort);
    }

    unsafe {
        as_mut_bytes(into).copy_from_slice(&bytes[..sz]);
    }

    Ok(())
}
