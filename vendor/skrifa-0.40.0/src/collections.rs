//! Internal "small" style collection types.

use alloc::vec::Vec;
use core::hash::{Hash, Hasher};

/// A growable vector type with inline storage optimization.
///
/// Note that unlike the real `SmallVec`, this only works with types that
/// are `Copy + Default` to simplify our implementation.
#[derive(Clone)]
pub(crate) struct SmallVec<T, const N: usize>(Storage<T, N>);

impl<T, const N: usize> SmallVec<T, N>
where
    T: Copy + Default,
{
    /// Creates a new, empty `SmallVec<T>`.
    pub fn new() -> Self {
        Self(Storage::Inline([T::default(); N], 0))
    }

    /// Creates a new `SmallVec<T>` of the given length with each element
    /// containing a copy of `value`.
    pub fn with_len(len: usize, value: T) -> Self {
        if len <= N {
            Self(Storage::Inline([value; N], len))
        } else {
            let mut vec = Vec::new();
            vec.resize(len, value);
            Self(Storage::Heap(vec))
        }
    }

    /// Clears the vector, removing all values.
    pub fn clear(&mut self) {
        match &mut self.0 {
            Storage::Inline(_buf, len) => *len = 0,
            Storage::Heap(vec) => vec.clear(),
        }
    }

    /// Tries to reserve capacity for at least `additional` more elements.
    pub fn try_reserve(&mut self, additional: usize) -> bool {
        match &mut self.0 {
            Storage::Inline(buf, len) => {
                let new_cap = *len + additional;
                if new_cap > N {
                    let mut vec = Vec::new();
                    if vec.try_reserve(new_cap).is_err() {
                        return false;
                    }
                    vec.extend_from_slice(&buf[..*len]);
                    self.0 = Storage::Heap(vec);
                }
            }
            Storage::Heap(vec) => {
                if vec.try_reserve(additional).is_err() {
                    return false;
                }
            }
        }
        true
    }

    /// Appends an element to the back of the collection.
    pub fn push(&mut self, value: T) {
        match &mut self.0 {
            Storage::Inline(buf, len) => {
                if *len + 1 > N {
                    let mut vec = Vec::with_capacity(*len + 1);
                    vec.extend_from_slice(&buf[..*len]);
                    vec.push(value);
                    self.0 = Storage::Heap(vec);
                } else {
                    buf[*len] = value;
                    *len += 1;
                }
            }
            Storage::Heap(vec) => vec.push(value),
        }
    }

    /// Removes and returns the value at the back of the collection.
    pub fn pop(&mut self) -> Option<T> {
        match &mut self.0 {
            Storage::Inline(buf, len) => {
                if *len > 0 {
                    *len -= 1;
                    Some(buf[*len])
                } else {
                    None
                }
            }
            Storage::Heap(vec) => vec.pop(),
        }
    }

    /// Shortens the vector, keeping the first `len` elements.
    pub fn truncate(&mut self, len: usize) {
        match &mut self.0 {
            Storage::Inline(_buf, inline_len) => {
                *inline_len = len.min(*inline_len);
            }
            Storage::Heap(vec) => vec.truncate(len),
        }
    }
}

impl<T, const N: usize> SmallVec<T, N> {
    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        match &self.0 {
            Storage::Inline(buf, len) => &buf[..*len],
            Storage::Heap(vec) => vec.as_slice(),
        }
    }

    /// Extracts a mutable slice containing the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        match &mut self.0 {
            Storage::Inline(buf, len) => &mut buf[..*len],
            Storage::Heap(vec) => vec.as_mut_slice(),
        }
    }
}

impl<T, const N: usize> Default for SmallVec<T, N>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> core::ops::Deref for SmallVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> core::ops::DerefMut for SmallVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> Hash for SmallVec<T, N>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T, const N: usize> PartialEq for SmallVec<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T, const N: usize> PartialEq<[T]> for SmallVec<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T, const N: usize> Eq for SmallVec<T, N> where T: Eq {}

impl<T, const N: usize> core::fmt::Debug for SmallVec<T, N>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.as_slice().iter()).finish()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a SmallVec<T, N> {
    type IntoIter = core::slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut SmallVec<T, N> {
    type IntoIter = core::slice::IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<T, const N: usize> IntoIterator for SmallVec<T, N>
where
    T: Copy,
{
    type IntoIter = IntoIter<T, N>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { vec: self, pos: 0 }
    }
}

#[derive(Clone)]
pub(crate) struct IntoIter<T, const N: usize> {
    vec: SmallVec<T, N>,
    pos: usize,
}

impl<T, const N: usize> Iterator for IntoIter<T, N>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.vec.get(self.pos)?;
        self.pos += 1;
        Some(*value)
    }
}

#[derive(Clone)]
enum Storage<T, const N: usize> {
    Inline([T; N], usize),
    Heap(Vec<T>),
}

#[cfg(test)]
mod test {
    use super::{SmallVec, Storage};

    #[test]
    fn choose_inline() {
        let vec = SmallVec::<_, 4>::with_len(4, 0);
        assert!(matches!(vec.0, Storage::Inline(..)));
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn choose_heap() {
        let vec = SmallVec::<_, 4>::with_len(5, 0);
        assert!(matches!(vec.0, Storage::Heap(..)));
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn store_and_read_inline() {
        let mut vec = SmallVec::<_, 8>::with_len(8, 0);
        for (i, value) in vec.iter_mut().enumerate() {
            *value = i * 2;
        }
        let expected = [0, 2, 4, 6, 8, 10, 12, 14];
        assert_eq!(vec.as_slice(), &expected);
        assert_eq!(format!("{vec:?}"), format!("{expected:?}"));
    }

    #[test]
    fn store_and_read_heap() {
        let mut vec = SmallVec::<_, 4>::with_len(8, 0);
        for (i, value) in vec.iter_mut().enumerate() {
            *value = i * 2;
        }
        let expected = [0, 2, 4, 6, 8, 10, 12, 14];
        assert_eq!(vec.as_slice(), &expected);
        assert_eq!(format!("{vec:?}"), format!("{expected:?}"));
    }

    #[test]
    fn spill_to_heap() {
        let mut vec = SmallVec::<_, 4>::new();
        for i in 0..4 {
            vec.push(i);
        }
        assert!(matches!(vec.0, Storage::Inline(..)));
        vec.push(4);
        assert!(matches!(vec.0, Storage::Heap(..)));
        let expected = [0, 1, 2, 3, 4];
        assert_eq!(vec.as_slice(), &expected);
    }

    #[test]
    fn clear_inline() {
        let mut vec = SmallVec::<_, 4>::new();
        for i in 0..4 {
            vec.push(i);
        }
        assert!(matches!(vec.0, Storage::Inline(..)));
        assert_eq!(vec.len(), 4);
        vec.clear();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn clear_heap() {
        let mut vec = SmallVec::<_, 3>::new();
        for i in 0..4 {
            vec.push(i);
        }
        assert!(matches!(vec.0, Storage::Heap(..)));
        assert_eq!(vec.len(), 4);
        vec.clear();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn reserve() {
        let mut vec = SmallVec::<_, 3>::new();
        for i in 0..2 {
            vec.push(i);
        }
        assert!(matches!(vec.0, Storage::Inline(..)));
        assert!(vec.try_reserve(1));
        // still inline after reserving 1
        assert!(matches!(vec.0, Storage::Inline(..)));
        assert!(vec.try_reserve(2));
        // reserving 2 spills to heap
        assert!(matches!(vec.0, Storage::Heap(..)));
    }

    #[test]
    fn iter() {
        let mut vec = SmallVec::<_, 3>::new();
        for i in 0..3 {
            vec.push(i);
        }
        assert!(&[0, 1, 2].iter().eq(vec.iter()));
    }

    #[test]
    fn into_iter() {
        let mut vec = SmallVec::<_, 3>::new();
        for i in 0..3 {
            vec.push(i);
        }
        assert!([0, 1, 2].into_iter().eq(vec.into_iter()));
    }
}
