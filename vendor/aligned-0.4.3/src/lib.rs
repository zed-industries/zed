//! A newtype with alignment of at least `A` bytes
//!
//! # Examples
//!
//! ```
//! use std::mem;
//!
//! use aligned::{Aligned, A2, A4, A16};
//!
//! // Array aligned to a 2 byte boundary
//! static X: Aligned<A2, [u8; 3]> = Aligned([0; 3]);
//!
//! // Array aligned to a 4 byte boundary
//! static Y: Aligned<A4, [u8; 3]> = Aligned([0; 3]);
//!
//! // Unaligned array
//! static Z: [u8; 3] = [0; 3];
//!
//! // You can allocate the aligned arrays on the stack too
//! let w: Aligned<A16, _> = Aligned([0u8; 3]);
//!
//! assert_eq!(mem::align_of_val(&X), 2);
//! assert_eq!(mem::align_of_val(&Y), 4);
//! assert_eq!(mem::align_of_val(&Z), 1);
//! assert_eq!(mem::align_of_val(&w), 16);
//! ```

#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(not(test), no_std)]

use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::{self},
};

use as_slice::{AsMutSlice, AsSlice};

/// A marker trait for an alignment value.
pub trait Alignment: Copy + sealed::Sealed {
    /// The alignment in bytes.
    const ALIGN: usize;
}

impl Alignment for A1 {
    const ALIGN: usize = 1;
}
impl Alignment for A2 {
    const ALIGN: usize = 2;
}
impl Alignment for A4 {
    const ALIGN: usize = 4;
}
impl Alignment for A8 {
    const ALIGN: usize = 8;
}
impl Alignment for A16 {
    const ALIGN: usize = 16;
}
impl Alignment for A32 {
    const ALIGN: usize = 32;
}
impl Alignment for A64 {
    const ALIGN: usize = 64;
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::A1 {}
    impl Sealed for super::A2 {}
    impl Sealed for super::A4 {}
    impl Sealed for super::A8 {}
    impl Sealed for super::A16 {}
    impl Sealed for super::A32 {}
    impl Sealed for super::A64 {}
}

/// 1-byte alignment
#[derive(Clone, Copy)]
#[repr(align(1))]
pub struct A1;

/// 2-byte alignment
#[derive(Clone, Copy)]
#[repr(align(2))]
pub struct A2;

/// 4-byte alignment
#[derive(Clone, Copy)]
#[repr(align(4))]
pub struct A4;

/// 8-byte alignment
#[derive(Clone, Copy)]
#[repr(align(8))]
pub struct A8;

/// 16-byte alignment
#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct A16;

/// 32-byte alignment
#[derive(Clone, Copy)]
#[repr(align(32))]
pub struct A32;

/// 64-byte alignment
#[derive(Clone, Copy)]
#[repr(align(64))]
pub struct A64;

/// A newtype with alignment of at least `A` bytes
#[repr(C)]
pub struct Aligned<A, T>
where
    T: ?Sized,
{
    _alignment: [A; 0],
    value: T,
}

/// Changes the alignment of `value` to be at least `A` bytes
#[allow(non_snake_case)]
pub const fn Aligned<A, T>(value: T) -> Aligned<A, T> {
    Aligned {
        _alignment: [],
        value,
    }
}

impl<A, T> ops::Deref for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<A, T> ops::DerefMut for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<A, T> Aligned<A, [T]>
where
    A: Alignment,
{
    fn is_index_aligned(index: usize) -> bool {
        use core::mem::size_of;

        (index * size_of::<T>()) % A::ALIGN == 0
    }
    fn check_start_index(index: usize) {
        if !Self::is_index_aligned(index) {
            panic!("Unaligned start index");
        }
    }
}

impl<A, T> ops::Index<ops::RangeFrom<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    type Output = Aligned<A, [T]>;

    fn index(&self, range: ops::RangeFrom<usize>) -> &Aligned<A, [T]> {
        Self::check_start_index(range.start);
        unsafe { &*(&self.value[range] as *const [T] as *const Aligned<A, [T]>) }
    }
}

impl<A, T> ops::Index<ops::RangeTo<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    type Output = Aligned<A, [T]>;

    fn index(&self, range: ops::RangeTo<usize>) -> &Aligned<A, [T]> {
        unsafe { &*(&self.value[range] as *const [T] as *const Aligned<A, [T]>) }
    }
}

impl<A, T> ops::Index<ops::RangeToInclusive<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    type Output = Aligned<A, [T]>;

    fn index(&self, range: ops::RangeToInclusive<usize>) -> &Aligned<A, [T]> {
        unsafe { &*(&self.value[range] as *const [T] as *const Aligned<A, [T]>) }
    }
}

impl<A, T> ops::Index<ops::RangeInclusive<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    type Output = Aligned<A, [T]>;

    fn index(&self, range: ops::RangeInclusive<usize>) -> &Aligned<A, [T]> {
        Self::check_start_index(*range.start());
        unsafe { &*(&self.value[range] as *const [T] as *const Aligned<A, [T]>) }
    }
}

impl<A, T> ops::Index<ops::Range<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    type Output = Aligned<A, [T]>;

    fn index(&self, range: ops::Range<usize>) -> &Aligned<A, [T]> {
        Self::check_start_index(range.start);
        unsafe { &*(&self.value[range] as *const [T] as *const Aligned<A, [T]>) }
    }
}

impl<A, T> ops::Index<ops::RangeFull> for Aligned<A, [T]>
where
    A: Alignment,
{
    type Output = Aligned<A, [T]>;

    fn index(&self, range: ops::RangeFull) -> &Aligned<A, [T]> {
        unsafe { &*(&self.value[range] as *const [T] as *const Aligned<A, [T]>) }
    }
}

impl<A, T> ops::IndexMut<ops::RangeFrom<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    fn index_mut(&mut self, range: ops::RangeFrom<usize>) -> &mut Aligned<A, [T]> {
        Self::check_start_index(range.start);
        unsafe { &mut *(&mut self.value[range] as *mut [T] as *mut Aligned<A, [T]>) }
    }
}

impl<A, T> ops::IndexMut<ops::RangeTo<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    fn index_mut(&mut self, range: ops::RangeTo<usize>) -> &mut Aligned<A, [T]> {
        unsafe { &mut *(&mut self.value[range] as *mut [T] as *mut Aligned<A, [T]>) }
    }
}

impl<A, T> ops::IndexMut<ops::RangeToInclusive<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    fn index_mut(&mut self, range: ops::RangeToInclusive<usize>) -> &mut Aligned<A, [T]> {
        unsafe { &mut *(&mut self.value[range] as *mut [T] as *mut Aligned<A, [T]>) }
    }
}

impl<A, T> ops::IndexMut<ops::RangeInclusive<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    fn index_mut(&mut self, range: ops::RangeInclusive<usize>) -> &mut Aligned<A, [T]> {
        Self::check_start_index(*range.start());
        unsafe { &mut *(&mut self.value[range] as *mut [T] as *mut Aligned<A, [T]>) }
    }
}

impl<A, T> ops::IndexMut<ops::Range<usize>> for Aligned<A, [T]>
where
    A: Alignment,
{
    fn index_mut(&mut self, range: ops::Range<usize>) -> &mut Aligned<A, [T]> {
        Self::check_start_index(range.start);
        unsafe { &mut *(&mut self.value[range] as *mut [T] as *mut Aligned<A, [T]>) }
    }
}

impl<A, T> ops::IndexMut<ops::RangeFull> for Aligned<A, [T]>
where
    A: Alignment,
{
    fn index_mut(&mut self, range: ops::RangeFull) -> &mut Aligned<A, [T]> {
        unsafe { &mut *(&mut self.value[range] as *mut [T] as *mut Aligned<A, [T]>) }
    }
}

impl<A, T> AsSlice for Aligned<A, T>
where
    A: Alignment,
    T: AsSlice,
{
    type Element = T::Element;

    fn as_slice(&self) -> &[T::Element] {
        T::as_slice(&**self)
    }
}

impl<A, T> AsMutSlice for Aligned<A, T>
where
    A: Alignment,
    T: AsMutSlice,
{
    fn as_mut_slice(&mut self) -> &mut [T::Element] {
        T::as_mut_slice(&mut **self)
    }
}

impl<A, T> Borrow<T> for Aligned<A, T>
where
    A: Alignment,
{
    fn borrow(&self) -> &T {
        &self.value
    }
}

impl<A, T> BorrowMut<T> for Aligned<A, T>
where
    A: Alignment,
{
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<A, T> Borrow<[<Aligned<A, T> as AsSlice>::Element]> for Aligned<A, T>
where
    A: Alignment,
    Aligned<A, T>: AsSlice,
{
    fn borrow(&self) -> &[<Aligned<A, T> as AsSlice>::Element] {
        self.as_slice()
    }
}

impl<A, T> BorrowMut<[<Aligned<A, T> as AsSlice>::Element]> for Aligned<A, T>
where
    A: Alignment,
    Aligned<A, T>: AsMutSlice,
{
    fn borrow_mut(&mut self) -> &mut [<Aligned<A, T> as AsSlice>::Element] {
        self.as_mut_slice()
    }
}

impl<A, T> Clone for Aligned<A, T>
where
    A: Alignment,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            _alignment: [],
            value: self.value.clone(),
        }
    }
}

impl<A, T> Copy for Aligned<A, T>
where
    A: Alignment,
    T: Copy,
{
}

impl<A, T> Default for Aligned<A, T>
where
    A: Alignment,
    T: Default,
{
    fn default() -> Self {
        Self {
            _alignment: [],
            value: Default::default(),
        }
    }
}

impl<A, T> Debug for Aligned<A, T>
where
    A: Alignment,
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<A, T> Display for Aligned<A, T>
where
    A: Alignment,
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<A, T> PartialEq for Aligned<A, T>
where
    A: Alignment,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<A, T> Eq for Aligned<A, T>
where
    A: Alignment,
    T: Eq,
{
}

impl<A, T> Hash for Aligned<A, T>
where
    A: Alignment,
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<A, T> Ord for Aligned<A, T>
where
    A: Alignment,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<A, T> PartialOrd for Aligned<A, T>
where
    A: Alignment,
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

#[test]
fn sanity() {
    use core::mem;

    let a: Aligned<A1, _> = Aligned([0u8; 3]);
    let x: Aligned<A2, _> = Aligned([0u8; 3]);
    let y: Aligned<A4, _> = Aligned([0u8; 3]);
    let z: Aligned<A8, _> = Aligned([0u8; 3]);
    let w: Aligned<A16, _> = Aligned([0u8; 3]);

    // check alignment
    assert_eq!(mem::align_of_val(&a), 1);
    assert_eq!(mem::align_of_val(&x), 2);
    assert_eq!(mem::align_of_val(&y), 4);
    assert_eq!(mem::align_of_val(&z), 8);
    assert_eq!(mem::align_of_val(&w), 16);

    assert!(a.as_ptr() as usize % 1 == 0);
    assert!(x.as_ptr() as usize % 2 == 0);
    assert!(y.as_ptr() as usize % 4 == 0);
    assert!(z.as_ptr() as usize % 8 == 0);
    assert!(w.as_ptr() as usize % 16 == 0);

    // test `deref`
    assert_eq!(a.len(), 3);
    assert_eq!(x.len(), 3);
    assert_eq!(y.len(), 3);
    assert_eq!(z.len(), 3);
    assert_eq!(w.len(), 3);

    // alignment should be preserved after slicing
    let a: &Aligned<_, [_]> = &a;
    let x: &Aligned<_, [_]> = &x;
    let y: &Aligned<_, [_]> = &y;
    let z: &Aligned<_, [_]> = &z;
    let w: &Aligned<_, [_]> = &w;

    let a: &Aligned<_, _> = &a[..2];
    let x: &Aligned<_, _> = &x[..2];
    let y: &Aligned<_, _> = &y[..2];
    let z: &Aligned<_, _> = &z[..2];
    let w: &Aligned<_, _> = &w[..2];

    assert!(a.as_ptr() as usize % 1 == 0);
    assert!(x.as_ptr() as usize % 2 == 0);
    assert!(y.as_ptr() as usize % 4 == 0);
    assert!(z.as_ptr() as usize % 8 == 0);
    assert!(w.as_ptr() as usize % 16 == 0);

    // alignment should be preserved after boxing
    let a: Box<Aligned<A1, [u8]>> = Box::new(Aligned([0u8; 3]));
    let x: Box<Aligned<A2, [u8]>> = Box::new(Aligned([0u8; 3]));
    let y: Box<Aligned<A4, [u8]>> = Box::new(Aligned([0u8; 3]));
    let z: Box<Aligned<A8, [u8]>> = Box::new(Aligned([0u8; 3]));
    let w: Box<Aligned<A16, [u8]>> = Box::new(Aligned([0u8; 3]));

    assert_eq!(mem::align_of_val(&*a), 1);
    assert_eq!(mem::align_of_val(&*x), 2);
    assert_eq!(mem::align_of_val(&*y), 4);
    assert_eq!(mem::align_of_val(&*z), 8);
    assert_eq!(mem::align_of_val(&*w), 16);

    // test coercions
    let x: Aligned<A2, _> = Aligned([0u8; 3]);
    let y: &Aligned<A2, [u8]> = &x;
    let _: &[u8] = y;
}

#[test]
fn test_range_to() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&a[..0]).as_slice(), &[],);
    assert_eq!((&a[..1]).as_slice(), &[0],);
    assert_eq!((&a[..2]).as_slice(), &[0, 1],);
    assert_eq!((&a[..3]).as_slice(), &[0, 1, 2],);
    assert_eq!((&a[..4]).as_slice(), &[0, 1, 2, 3],);
}

#[test]
fn test_range_to_mut() {
    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&mut a[..0]).as_slice(), &[],);
    assert_eq!((&mut a[..1]).as_slice(), &[0],);
    assert_eq!((&mut a[..2]).as_slice(), &[0, 1],);
    assert_eq!((&mut a[..3]).as_slice(), &[0, 1, 2],);
    assert_eq!((&mut a[..4]).as_slice(), &[0, 1, 2, 3],);
}

#[test]
fn test_range_to_inclusive() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&a[..=0]).as_slice(), &[0],);
    assert_eq!((&a[..=1]).as_slice(), &[0, 1],);
    assert_eq!((&a[..=2]).as_slice(), &[0, 1, 2],);
    assert_eq!((&a[..=3]).as_slice(), &[0, 1, 2, 3],);
}

#[test]
fn test_range_to_inclusive_mut() {
    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&mut a[..=0]).as_slice(), &[0],);
    assert_eq!((&mut a[..=1]).as_slice(), &[0, 1],);
    assert_eq!((&mut a[..=2]).as_slice(), &[0, 1, 2],);
    assert_eq!((&mut a[..=3]).as_slice(), &[0, 1, 2, 3],);
}

#[test]
fn test_range_full() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&a[..]).as_slice(), &[0, 1, 2, 3],);
}

#[test]
fn test_range_full_mut() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&a[..]).as_slice(), &[0, 1, 2, 3],);
}

#[test]
fn test_range_from() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&a[0..]).as_slice(), &[0, 1, 2, 3],);
    assert_eq!((&a[2..]).as_slice(), &[2, 3],);
    assert_eq!((&a[4..]).as_slice(), &[],);

    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0, 1, 2]);
    assert_eq!((&a[0..]).as_slice(), &[0, 1, 2],);
    assert_eq!((&a[2..]).as_slice(), &[2],);

    let a: &Aligned<A4, [u8]> = &Aligned::<A4, _>([0, 1, 2, 3, 4, 5, 6, 7]);
    assert_eq!((&a[0..]).as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7],);
    assert_eq!((&a[4..]).as_slice(), &[4, 5, 6, 7],);
    assert_eq!((&a[8..]).as_slice(), &[],);

    let a: &Aligned<A4, [u8]> = &Aligned::<A4, _>([0, 1, 2, 3, 4, 5, 6]);
    assert_eq!((&a[0..]).as_slice(), &[0, 1, 2, 3, 4, 5, 6],);
    assert_eq!((&a[4..]).as_slice(), &[4, 5, 6],);

    let a: &Aligned<A8, [u8]> =
        &Aligned::<A8, _>([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(
        (&a[0..]).as_slice(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    );
    assert_eq!((&a[8..]).as_slice(), &[8, 9, 10, 11, 12, 13, 14, 15],);
    assert_eq!((&a[16..]).as_slice(), &[],);

    let a: &Aligned<A8, [u8]> =
        &Aligned::<A8, _>([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    assert_eq!(
        (&a[0..]).as_slice(),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    );
    assert_eq!((&a[8..]).as_slice(), &[8, 9, 10, 11, 12, 13, 14],);

    let a: &Aligned<A16, [u8]> = &Aligned::<A16, _>([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ]);
    assert_eq!(
        (&a[0..]).as_slice(),
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31
        ],
    );
    assert_eq!(
        (&a[16..]).as_slice(),
        &[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    );
    assert_eq!((&a[32..]).as_slice(), &[],);

    let a: &Aligned<A16, [u8]> = &Aligned::<A16, _>([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30,
    ]);
    assert_eq!(
        (&a[0..]).as_slice(),
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30
        ],
    );
    assert_eq!(
        (&a[16..]).as_slice(),
        &[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
    );

    let a: &Aligned<A32, [u8]> = &Aligned::<A32, _>([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    ]);
    assert_eq!(
        (&a[0..]).as_slice(),
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
        ]
    );
    assert_eq!(
        (&a[32..]).as_slice(),
        &[
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
            54, 55, 56, 57, 58, 59, 60, 61, 62, 63
        ]
    );
    assert_eq!((&a[64..]).as_slice(), &[],);

    let a: &Aligned<A32, [u8]> = &Aligned::<A32, _>([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62,
    ]);
    assert_eq!(
        (&a[0..]).as_slice(),
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
        ]
    );
    assert_eq!(
        (&a[32..]).as_slice(),
        &[
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
            54, 55, 56, 57, 58, 59, 60, 61, 62
        ]
    );

    let a: &Aligned<A64, [u8]> = &Aligned::<A64, _>([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    ]);
    assert_eq!(
        (&a[0..]).as_slice(),
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
            109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
            126, 127
        ]
    );
    assert_eq!(
        (&a[64..]).as_slice(),
        &[
            64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
            86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105,
            106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
            123, 124, 125, 126, 127
        ]
    );
    assert_eq!((&a[128..]).as_slice(), &[]);

    let a: &Aligned<A64, [u8]> = &Aligned::<A64, _>([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
    ]);
    assert_eq!(
        (&a[0..]).as_slice(),
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
            109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
            126
        ]
    );
    assert_eq!(
        (&a[64..]).as_slice(),
        &[
            64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
            86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105,
            106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
            123, 124, 125, 126
        ]
    );
}

#[test]
#[should_panic(expected = "Unaligned start index")]
fn test_range_from_a2_invalid_alignment() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0u8; 4]);
    let _ = &a[1..];
}

#[test]
#[should_panic(expected = "Unaligned start index")]
fn test_range_from_a4_invalid_alignment() {
    let a: &Aligned<A4, [u8]> = &Aligned::<A4, _>([0u8; 8]);
    let _ = &a[6..];
}

#[test]
fn test_range_from_mut() {
    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&mut a[0..]).as_slice(), &[0, 1, 2, 3],);
    assert_eq!((&mut a[2..]).as_slice(), &[2, 3],);
    assert_eq!((&mut a[4..]).as_slice(), &[],);

    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0, 1, 2]);
    assert_eq!((&mut a[0..]).as_slice(), &[0, 1, 2],);
    assert_eq!((&mut a[2..]).as_slice(), &[2],);
}

#[test]
#[should_panic(expected = "Unaligned start index")]
fn test_range_from_mut_invalid_argument() {
    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0u8; 4]);
    let _ = &mut a[1..];
}

#[test]
fn test_range() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&a[0..0]).as_slice(), &[],);
    assert_eq!((&a[0..1]).as_slice(), &[0],);
    assert_eq!((&a[0..2]).as_slice(), &[0, 1],);
    assert_eq!((&a[0..3]).as_slice(), &[0, 1, 2],);
    assert_eq!((&a[0..4]).as_slice(), &[0, 1, 2, 3],);
    assert_eq!((&a[2..2]).as_slice(), &[],);
    assert_eq!((&a[2..3]).as_slice(), &[2],);
    assert_eq!((&a[2..4]).as_slice(), &[2, 3],);
    assert_eq!((&a[4..4]).as_slice(), &[],);
}

#[test]
#[should_panic(expected = "Unaligned start index")]
fn test_range_invalid_alignment() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0u8; 4]);
    let _ = &a[1..2];
}

#[test]
fn test_range_mut() {
    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&mut a[0..0]).as_slice(), &[],);
    assert_eq!((&mut a[0..1]).as_slice(), &[0],);
    assert_eq!((&mut a[0..2]).as_slice(), &[0, 1],);
    assert_eq!((&mut a[0..3]).as_slice(), &[0, 1, 2],);
    assert_eq!((&mut a[0..4]).as_slice(), &[0, 1, 2, 3],);
    assert_eq!((&mut a[2..2]).as_slice(), &[],);
    assert_eq!((&mut a[2..3]).as_slice(), &[2],);
    assert_eq!((&mut a[2..4]).as_slice(), &[2, 3],);
    assert_eq!((&mut a[4..4]).as_slice(), &[],);
}

#[test]
#[should_panic(expected = "Unaligned start index")]
fn test_range_mut_invalid_alignment() {
    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0u8; 4]);
    let _ = &mut a[1..2];
}

#[test]
fn test_range_inclusive() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&a[0..=0]).as_slice(), &[0],);
    assert_eq!((&a[0..=1]).as_slice(), &[0, 1],);
    assert_eq!((&a[0..=2]).as_slice(), &[0, 1, 2],);
    assert_eq!((&a[0..=3]).as_slice(), &[0, 1, 2, 3],);
    assert_eq!((&a[2..=2]).as_slice(), &[2],);
    assert_eq!((&a[2..=3]).as_slice(), &[2, 3],);
}

#[test]
#[should_panic(expected = "Unaligned start index")]
fn test_range_inclusive_invalid_alignment() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0u8; 4]);
    let _ = &a[1..=2];
}

#[test]
fn test_range_inclusive_mut() {
    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0, 1, 2, 3]);
    assert_eq!((&mut a[0..=0]).as_slice(), &[0],);
    assert_eq!((&mut a[0..=1]).as_slice(), &[0, 1],);
    assert_eq!((&mut a[0..=2]).as_slice(), &[0, 1, 2],);
    assert_eq!((&mut a[0..=3]).as_slice(), &[0, 1, 2, 3],);
    assert_eq!((&mut a[2..=2]).as_slice(), &[2],);
    assert_eq!((&mut a[2..=3]).as_slice(), &[2, 3],);
}

#[test]
#[should_panic(expected = "Unaligned start index")]
fn test_range_inclusive_mut_invalid_alignment() {
    let a: &mut Aligned<A2, [u8]> = &mut Aligned::<A2, _>([0u8; 4]);
    let _ = &mut a[1..=2];
}

#[test]
#[should_panic(expected = "out of range")]
fn test_range_from_out_of_bounds() {
    let a: &Aligned<A2, [u8]> = &Aligned::<A2, _>([0u8; 4]);
    let _ = &a[6..];
}
