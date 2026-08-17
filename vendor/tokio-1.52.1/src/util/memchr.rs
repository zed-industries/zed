//! Search for a byte in a byte array using libc.
//!
//! When nothing pulls in libc, then just use a trivial implementation. Note
//! that we only depend on libc on unix.

#[cfg(not(all(unix, feature = "libc")))]
fn memchr_inner(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|val| needle == *val)
}

#[cfg(all(unix, feature = "libc"))]
fn memchr_inner(needle: u8, haystack: &[u8]) -> Option<usize> {
    let start = haystack.as_ptr();

    // SAFETY: `start` is valid for `haystack.len()` bytes.
    let ptr = (unsafe { libc::memchr(start.cast(), needle as _, haystack.len()) })
        .cast::<u8>()
        .cast_const();

    if ptr.is_null() {
        None
    } else {
        // SAFETY: `ptr` will always be in bounds, since libc guarantees that the ptr will either
        //          be to an element inside the array or the ptr will be null
        //          since the ptr is in bounds the offset must also always be non null
        //          and there can't be more than isize::MAX elements inside an array
        //          as rust guarantees that the maximum number of bytes a allocation
        //          may occupy is isize::MAX
        unsafe {
            // TODO(MSRV 1.87): When bumping MSRV, switch to `ptr.byte_offset_from_unsigned(start)`.
            Some(usize::try_from(ptr.offset_from(start)).unwrap_unchecked())
        }
    }
}

pub(crate) fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    let index = memchr_inner(needle, haystack)?;

    // SAFETY: `memchr_inner` returns Some(index) and in that case index must point to an element in haystack
    //         or `memchr_inner` None which is guarded by the `?` operator above
    //         therefore the index must **always** point to an element in the array
    //         and so this indexing operation is safe
    // TODO(MSRV 1.81): When bumping MSRV, switch to `std::hint::assert_unchecked(haystack.get(..=index).is_some());`
    unsafe {
        if haystack.get(..=index).is_none() {
            std::hint::unreachable_unchecked()
        }
    }

    Some(index)
}

#[cfg(test)]
mod tests {
    use super::memchr;

    #[test]
    fn memchr_test() {
        let haystack = b"123abc456\0\xffabc\n";

        assert_eq!(memchr(b'1', haystack), Some(0));
        assert_eq!(memchr(b'2', haystack), Some(1));
        assert_eq!(memchr(b'3', haystack), Some(2));
        assert_eq!(memchr(b'4', haystack), Some(6));
        assert_eq!(memchr(b'5', haystack), Some(7));
        assert_eq!(memchr(b'6', haystack), Some(8));
        assert_eq!(memchr(b'7', haystack), None);
        assert_eq!(memchr(b'a', haystack), Some(3));
        assert_eq!(memchr(b'b', haystack), Some(4));
        assert_eq!(memchr(b'c', haystack), Some(5));
        assert_eq!(memchr(b'd', haystack), None);
        assert_eq!(memchr(b'A', haystack), None);
        assert_eq!(memchr(0, haystack), Some(9));
        assert_eq!(memchr(0xff, haystack), Some(10));
        assert_eq!(memchr(0xfe, haystack), None);
        assert_eq!(memchr(1, haystack), None);
        assert_eq!(memchr(b'\n', haystack), Some(14));
        assert_eq!(memchr(b'\r', haystack), None);
    }

    #[test]
    fn memchr_all() {
        let mut arr = Vec::new();
        for b in 0..=255 {
            arr.push(b);
        }
        for b in 0..=255 {
            assert_eq!(memchr(b, &arr), Some(b as usize));
        }
        arr.reverse();
        for b in 0..=255 {
            assert_eq!(memchr(b, &arr), Some(255 - b as usize));
        }
    }

    #[test]
    fn memchr_empty() {
        for b in 0..=255 {
            assert_eq!(memchr(b, b""), None);
        }
    }
}
