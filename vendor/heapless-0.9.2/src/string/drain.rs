use core::{fmt, iter::FusedIterator, str::Chars};

use crate::LenType;

use super::StringView;

/// A draining iterator for `String`.
///
/// This struct is created by the [`drain`] method on [`crate::String`]. See its
/// documentation for more.
///
/// [`drain`]: crate::String::drain
pub struct Drain<'a, LenT: LenType> {
    /// Will be used as &'a mut String in the destructor
    pub(super) string: *mut StringView<LenT>,
    /// Stast of part to remove
    pub(super) start: LenT,
    /// End of part to remove
    pub(super) end: LenT,
    /// Current remaining range to remove
    pub(super) iter: Chars<'a>,
}

impl<LenT: LenType> fmt::Debug for Drain<'_, LenT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Drain").field(&self.as_str()).finish()
    }
}

unsafe impl<LenT: LenType> Sync for Drain<'_, LenT> {}
unsafe impl<LenT: LenType> Send for Drain<'_, LenT> {}

impl<LenT: LenType> Drop for Drain<'_, LenT> {
    fn drop(&mut self) {
        unsafe {
            // Use `Vec::drain`. “Reaffirm” the bounds checks to avoid
            // panic code being inserted again.
            let self_vec = (*self.string).as_mut_vec();
            let start = self.start.into_usize();
            let end = self.end.into_usize();
            if start <= end && end <= self_vec.len() {
                self_vec.drain(start..end);
            }
        }
    }
}

impl<LenT: LenType> Drain<'_, LenT> {
    /// Returns the remaining (sub)string of this iterator as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::String;
    ///
    /// let mut s = String::<8>::try_from("abc").unwrap();
    /// let mut drain = s.drain(..);
    /// assert_eq!(drain.as_str(), "abc");
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_str(), "bc");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.iter.as_str()
    }
}

impl<LenT: LenType> AsRef<str> for Drain<'_, LenT> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<LenT: LenType> AsRef<[u8]> for Drain<'_, LenT> {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl<LenT: LenType> Iterator for Drain<'_, LenT> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<char> {
        self.next_back()
    }
}

impl<LenT: LenType> DoubleEndedIterator for Drain<'_, LenT> {
    #[inline]
    fn next_back(&mut self) -> Option<char> {
        self.iter.next_back()
    }
}

impl<LenT: LenType> FusedIterator for Drain<'_, LenT> {}

#[cfg(test)]
mod tests {
    use crate::String;

    #[test]
    fn drain_front() {
        let mut s = String::<8>::try_from("abcd").unwrap();
        let mut it = s.drain(..1);
        assert_eq!(it.next(), Some('a'));
        drop(it);
        assert_eq!(s, "bcd");
    }

    #[test]
    fn drain_middle() {
        let mut s = String::<8>::try_from("abcd").unwrap();
        let mut it = s.drain(1..3);
        assert_eq!(it.next(), Some('b'));
        assert_eq!(it.next(), Some('c'));
        drop(it);
        assert_eq!(s, "ad");
    }

    #[test]
    fn drain_end() {
        let mut s = String::<8>::try_from("abcd").unwrap();
        let mut it = s.drain(3..);
        assert_eq!(it.next(), Some('d'));
        drop(it);
        assert_eq!(s, "abc");
    }
}
