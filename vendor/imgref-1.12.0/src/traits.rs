use crate::{ImgRef, ImgRefMut, ImgVec};
use core::hash::{Hash, Hasher};

impl<T: Hash> Hash for ImgRef<'_, T> {
    #[allow(deprecated)]
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        for row in self.rows() {
            Hash::hash_slice(row, state);
        }
    }
}

impl<T: Hash> Hash for ImgRefMut<'_, T> {
    #[allow(deprecated)]
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<T: Hash> Hash for ImgVec<T> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<'b, T, U> PartialEq<ImgRef<'b, U>> for ImgRef<'_, T> where T: PartialEq<U> {
    #[allow(deprecated)]
    #[inline]
    fn eq(&self, other: &ImgRef<'b, U>) -> bool {
        self.width == other.width &&
        self.height == other.height &&
        self.rows().zip(other.rows()).all(|(a,b)| a == b)
    }
}

impl<'b, T, U> PartialEq<ImgRefMut<'b, U>> for ImgRefMut<'_, T> where T: PartialEq<U> {
    #[allow(deprecated)]
    #[inline]
    fn eq(&self, other: &ImgRefMut<'b, U>) -> bool {
        self.as_ref().eq(&other.as_ref())
    }
}


impl<T, U> PartialEq<ImgVec<U>> for ImgVec<T> where T: PartialEq<U> {
    #[allow(deprecated)]
    #[inline(always)]
    fn eq(&self, other: &ImgVec<U>) -> bool {
        self.as_ref().eq(&other.as_ref())
    }
}

impl<'a, T, U> PartialEq<ImgRef<'a, U>> for ImgVec<T> where T: PartialEq<U> {
    #[allow(deprecated)]
    #[inline(always)]
    fn eq(&self, other: &ImgRef<'a, U>) -> bool {
        self.as_ref().eq(other)
    }
}

impl<T, U> PartialEq<ImgVec<U>> for ImgRef<'_, T> where T: PartialEq<U> {
    #[allow(deprecated)]
    #[inline(always)]
    fn eq(&self, other: &ImgVec<U>) -> bool {
        self.eq(&other.as_ref())
    }
}

impl<'b, T, U> PartialEq<ImgRef<'b, U>> for ImgRefMut<'_, T> where T: PartialEq<U> {
    #[allow(deprecated)]
    #[inline(always)]
    fn eq(&self, other: &ImgRef<'b, U>) -> bool {
        self.as_ref().eq(other)
    }
}

impl<'b, T, U> PartialEq<ImgRefMut<'b, U>> for ImgRef<'_, T> where T: PartialEq<U> {
    #[allow(deprecated)]
    #[inline(always)]
    fn eq(&self, other: &ImgRefMut<'b, U>) -> bool {
        self.eq(&other.as_ref())
    }
}

impl<T: Eq> Eq for ImgRefMut<'_, T> {
}

impl<T: Eq> Eq for ImgRef<'_, T> {
}

impl<T: Eq> Eq for ImgVec<T> {
}

#[test]
fn test_eq_hash() {
    use alloc::vec;

    #[derive(Debug)]
    struct Comparable(u16);
    impl PartialEq<u8> for Comparable {
        fn eq(&self, other: &u8) -> bool { self.0 == u16::from(*other) }
    }

    let newtype = ImgVec::new(vec![Comparable(0), Comparable(1), Comparable(2), Comparable(3)], 2, 2);
    let mut img1 = ImgVec::new(vec![0u8, 1, 2, 3], 2, 2);
    let img_ne = ImgVec::new(vec![0u8, 1, 2, 3], 4, 1);
    let img2 = ImgVec::new_stride(vec![0u8, 1, 255, 2, 3, 255], 2, 2, 3);
    let mut img3 = ImgVec::new_stride(vec![0u8, 1, 255, 2, 3], 2, 2, 3);

    assert_eq!(newtype, img1);
    equiv(&img1, &img2);
    equiv(&img2, &img3);
    equiv(&img1, &img3);

    assert_ne!(img1, img_ne);
    assert_eq!(img1.as_ref(), img2);
    assert_eq!(img2, img3.as_ref());
    equiv(&img1.as_ref(), &img3.as_ref());
    equiv(&img1.as_mut(), &img3.as_mut());
    assert_eq!(img2.as_ref(), img3.as_mut());

    let mut map = HashSet::new();
    img3[(0usize, 0usize)] = 100;
    assert_ne!(img1, img3);
    assert!(map.insert(img1));
    assert!(map.insert(img3));
    assert!(map.insert(img_ne));
    assert!(!map.insert(img2));
}

#[cfg(test)]
use std::collections::HashSet;
#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
fn equiv<A>(a: &A, b: &A) where A: Eq + PartialEq + Hash + Debug {
    assert_eq!(a, b);
    let mut map = HashSet::new();
    assert!(map.insert(a));
    assert!(!map.insert(b));
    assert!(!map.insert(a));
    assert!(map.remove(b));
    assert!(map.is_empty());
}
