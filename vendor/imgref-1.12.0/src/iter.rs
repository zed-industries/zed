use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::slice;

#[cfg(test)]
use alloc::vec;

/// Rows of the image. Call `Img.rows()` to create it.
///
/// Each element is a slice `width` pixels wide. Ignores padding, if there's any.
#[derive(Debug)]
#[must_use]
pub struct RowsIter<'a, T> {
    pub(crate) inner: slice::Chunks<'a, T>,
    pub(crate) width: usize,
}

impl<'a, T: 'a> Iterator for RowsIter<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(s) => {
                // guaranteed during creation of chunks iterator
                debug_assert!(s.len() >= self.width);
                unsafe {
                    Some(s.get_unchecked(0..self.width))
                }
            },
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.inner.nth(n) {
            Some(s) => {
                // guaranteed during creation of chunks iterator
                debug_assert!(s.len() >= self.width);
                unsafe {
                    Some(s.get_unchecked(0..self.width))
                }
            },
            None => None,
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.inner.count()
    }
}

impl<T> ExactSizeIterator for RowsIter<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> FusedIterator for RowsIter<'_, T> {}

impl<'a, T: 'a> DoubleEndedIterator for RowsIter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.inner.next_back() {
            Some(s) => {
                // guaranteed during creation of chunks iterator
                debug_assert!(s.len() >= self.width);
                unsafe {
                    Some(s.get_unchecked(0..self.width))
                }
            },
            None => None,
        }
    }
}

/// Rows of the image. Call `Img.rows_mut()` to create it.
///
/// Each element is a slice `width` pixels wide. Ignores padding, if there's any.
#[derive(Debug)]
#[must_use]
pub struct RowsIterMut<'a, T> {
    pub(crate) width: usize,
    pub(crate) inner: slice::ChunksMut<'a, T>,
}

impl<'a, T: 'a> Iterator for RowsIterMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(s) => Some(&mut s[0..self.width]),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.inner.nth(n) {
            Some(s) => Some(&mut s[0..self.width]),
            None => None,
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.inner.count()
    }
}

impl<T> ExactSizeIterator for RowsIterMut<'_, T> {}
impl<T> FusedIterator for RowsIterMut<'_, T> {}

impl<'a, T: 'a> DoubleEndedIterator for RowsIterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.inner.next_back() {
            Some(s) => Some(&mut s[0..self.width]),
            None => None,
        }
    }
}

/// Iterates over pixels in the (sub)image. Call `Img.pixels()` to create it.
///
/// Ignores padding, if there's any.
#[must_use]
pub struct PixelsIter<'a, T: Copy> {
    inner: PixelsRefIter<'a, T>,
}

impl<'a, T: Copy + 'a> PixelsIter<'a, T> {
    #[inline(always)]
    #[track_caller]
    pub(crate) fn new(img: super::ImgRef<'a, T>) -> Self {
        Self {
            inner: PixelsRefIter::new(img)
        }
    }
}

impl<'a, T: Copy + 'a> Iterator for PixelsIter<'a, T> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }
}

impl<T: Copy> ExactSizeIterator for PixelsIter<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Iterates over pixels in the (sub)image. Call `Img.pixels_ref()` to create it.
///
/// Ignores padding, if there's any.
#[derive(Debug)]
#[must_use]
pub struct PixelsRefIter<'a, T> {
    current: *const T,
    current_line_end: *const T,
    rows_left: usize,
    width: NonZeroUsize,
    pad: usize,
    _dat: PhantomData<&'a [T]>,
}

unsafe impl<T> Send for PixelsRefIter<'_, T> where T: Send {}
unsafe impl<T> Sync for PixelsRefIter<'_, T> where T: Sync {}

impl<'a, T: 'a> PixelsRefIter<'a, T> {
    #[inline]
    #[track_caller]
    pub(crate) fn new(img: super::ImgRef<'a, T>) -> Self {
        let width = NonZeroUsize::new(img.width()).expect("width > 0");
        let height = img.height();
        let stride = img.stride();
        assert!(stride >= width.get());
        let pad = stride - width.get();
        debug_assert!(img.buf().len() + stride >= stride * height + width.get(),
            "buffer len {} is less than {} (({}+{})x{})", img.buf().len(),
            stride * height - pad, width, pad, height);
        Self {
            current: img.buf().as_ptr(),
            current_line_end: img.buf()[width.get()..].as_ptr(),
            width,
            rows_left: height,
            pad,
            _dat: PhantomData,
        }
    }
}

impl<'a, T: 'a> Iterator for PixelsRefIter<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current >= self.current_line_end {
                if self.rows_left <= 1 {
                    return None;
                }
                self.rows_left -= 1;
                self.current = self.current_line_end.add(self.pad);
                self.current_line_end = self.current.add(self.width.get());
            }
            let px = &*self.current;
            self.current = self.current.add(1);
            Some(px)
        }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let this_line = unsafe {
            self.current_line_end.offset_from(self.current)
        };
        debug_assert!(this_line >= 0);
        let len = this_line as usize + (self.rows_left - 1) * self.width.get();
        (len, Some(len))
    }
}

impl<T: Copy> ExactSizeIterator for PixelsRefIter<'_, T> {
}

/// Iterates over pixels in the (sub)image. Call `Img.pixels_mut()` to create it.
///
/// Ignores padding, if there's any.
#[derive(Debug)]
#[must_use]
pub struct PixelsIterMut<'a, T> {
    current: *mut T,
    current_line_end: *mut T,
    y: usize,
    width: NonZeroUsize,
    pad: usize,
    _dat: PhantomData<&'a mut [T]>,
}

unsafe impl<T> Send for PixelsIterMut<'_, T> where T: Send {}
unsafe impl<T> Sync for PixelsIterMut<'_, T> where T: Sync {}

impl<'a, T: 'a> PixelsIterMut<'a, T> {
    #[inline]
    #[track_caller]
    pub(crate) fn new(img: &mut super::ImgRefMut<'a, T>) -> Self {
        let width = NonZeroUsize::new(img.width()).expect("width > 0");
        let stride = img.stride();
        debug_assert!(!img.buf().is_empty() && img.buf().len() + stride >= stride * img.height() + width.get());
        Self {
            current: img.buf_mut().as_mut_ptr(),
            current_line_end: img.buf_mut()[width.get()..].as_mut_ptr(),
            width,
            y: img.height(),
            pad: stride - width.get(),
            _dat: PhantomData,
        }
    }
}

impl<'a, T: 'a> Iterator for PixelsIterMut<'a, T> {
    type Item = &'a mut T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current >= self.current_line_end {
                self.y -= 1;
                if self.y == 0 {
                    return None;
                }
                self.current = self.current_line_end.add(self.pad);
                self.current_line_end = self.current.add(self.width.get());
            }
            let px = &mut *self.current;
            self.current = self.current.add(1);
            Some(px)
        }
    }
}

#[test]
fn iter() {
    let img = super::Img::new(vec![1u8, 2], 1, 2);
    let mut it = img.pixels();
    assert_eq!(Some(1), it.next());
    assert_eq!(Some(2), it.next());
    assert_eq!(None, it.next());

    let buf = [1u8; (16 + 3) * (8 + 1)];
    for width in 1..16 {
        for height in 1..8 {
            for pad in 0..3 {
                let stride = width + pad;
                let img = super::Img::new_stride(&buf[..stride * height + stride - width], width, height, stride);
                assert_eq!(width * height, img.pixels().map(|a| a as usize).sum(), "{width}x{height}");
                assert_eq!(width * height, img.pixels().count(), "{width}x{height}");
                assert_eq!(height, img.rows().count());

                let mut iter1 = img.pixels();
                let mut left = width * height;
                while let Some(_px) = iter1.next() {
                    left -= 1;
                    assert_eq!(left, iter1.len());
                }
                assert_eq!(0, iter1.len());
                iter1.next();
                assert_eq!(0, iter1.len());

                let mut iter2 = img.rows();
                match iter2.next() {
                    Some(_) => {
                        assert_eq!(height - 1, iter2.size_hint().0);
                        assert_eq!(height - 1, iter2.filter(|_| true).count());
                    },
                    None => {
                        assert_eq!(height, 0);
                    },
                };
            }
        }
    }
}
