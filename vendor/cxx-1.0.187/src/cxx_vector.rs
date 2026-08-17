//! Less used details of `CxxVector` are exposed in this module. `CxxVector`
//! itself is exposed at the crate root.

use crate::extern_type::ExternType;
use crate::kind::Trivial;
use crate::string::CxxString;
use crate::unique_ptr::UniquePtr;
use core::ffi::c_void;
use core::fmt::{self, Debug};
use core::iter::FusedIterator;
use core::marker::{PhantomData, PhantomPinned};
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::pin::Pin;
use core::slice;

/// Binding to C++ `std::vector<T, std::allocator<T>>`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `CxxVector` by value. Instead in
/// Rust code we will only ever look at a vector behind a reference or smart
/// pointer, as in `&CxxVector<T>` or `UniquePtr<CxxVector<T>>`.
#[repr(C, packed)]
pub struct CxxVector<T> {
    // A thing, because repr(C) structs are not allowed to consist exclusively
    // of PhantomData fields.
    _void: [c_void; 0],
    // The conceptual vector elements to ensure that autotraits are propagated
    // correctly, e.g. CxxVector is UnwindSafe iff T is.
    _elements: PhantomData<[T]>,
    // Prevent unpin operation from Pin<&mut CxxVector<T>> to &mut CxxVector<T>.
    _pinned: PhantomData<PhantomPinned>,
}

impl<T> CxxVector<T>
where
    T: VectorElement,
{
    /// Constructs a new heap allocated vector, wrapped by UniquePtr.
    ///
    /// The C++ vector is default constructed.
    pub fn new() -> UniquePtr<Self> {
        unsafe { UniquePtr::from_raw(T::__vector_new()) }
    }

    /// Returns the number of elements in the vector.
    ///
    /// Matches the behavior of C++ [std::vector\<T\>::size][size].
    ///
    /// [size]: https://en.cppreference.com/w/cpp/container/vector/size
    pub fn len(&self) -> usize {
        T::__vector_size(self)
    }

    /// Returns the capacity of the vector.
    ///
    /// Matches the behavior of C++ [std::vector\<T\>::capacity][capacity].
    ///
    /// [capacity]: https://en.cppreference.com/w/cpp/container/vector/capacity
    pub fn capacity(&self) -> usize {
        T::__vector_capacity(self)
    }

    /// Returns true if the vector contains no elements.
    ///
    /// Matches the behavior of C++ [std::vector\<T\>::empty][empty].
    ///
    /// [empty]: https://en.cppreference.com/w/cpp/container/vector/empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to an element at the given position, or `None` if
    /// out of bounds.
    pub fn get(&self, pos: usize) -> Option<&T> {
        if pos < self.len() {
            Some(unsafe { self.get_unchecked(pos) })
        } else {
            None
        }
    }

    /// Returns a pinned mutable reference to an element at the given position,
    /// or `None` if out of bounds.
    ///
    /// This method cannot be named "get\_mut" due to a conflict with
    /// `Pin::get_mut`.
    #[doc(alias = "get_mut")]
    pub fn index_mut(self: Pin<&mut Self>, pos: usize) -> Option<Pin<&mut T>> {
        if pos < self.len() {
            Some(unsafe { self.index_unchecked_mut(pos) })
        } else {
            None
        }
    }

    /// Returns a reference to an element without doing bounds checking.
    ///
    /// This is generally not recommended, use with caution! Calling this method
    /// with an out-of-bounds index is undefined behavior even if the resulting
    /// reference is not used.
    ///
    /// Matches the behavior of C++
    /// [std::vector\<T\>::operator\[\] const][operator_at].
    ///
    /// [operator_at]: https://en.cppreference.com/w/cpp/container/vector/operator_at
    pub unsafe fn get_unchecked(&self, pos: usize) -> &T {
        let this = self as *const CxxVector<T> as *mut CxxVector<T>;
        unsafe {
            let ptr = T::__get_unchecked(this, pos) as *const T;
            &*ptr
        }
    }

    /// Returns a pinned mutable reference to an element without doing bounds
    /// checking.
    ///
    /// This is generally not recommended, use with caution! Calling this method
    /// with an out-of-bounds index is undefined behavior even if the resulting
    /// reference is not used.
    ///
    /// Matches the behavior of C++
    /// [std::vector\<T\>::operator\[\]][operator_at].
    ///
    /// [operator_at]: https://en.cppreference.com/w/cpp/container/vector/operator_at
    ///
    /// This method cannot be named "get\_unchecked\_mut" due to a conflict with
    /// `Pin::get_unchecked_mut`.
    #[doc(alias = "get_unchecked_mut")]
    pub unsafe fn index_unchecked_mut(self: Pin<&mut Self>, pos: usize) -> Pin<&mut T> {
        unsafe {
            let ptr = T::__get_unchecked(self.get_unchecked_mut(), pos);
            Pin::new_unchecked(&mut *ptr)
        }
    }

    /// Returns a slice to the underlying contiguous array of elements.
    pub fn as_slice(&self) -> &[T]
    where
        T: ExternType<Kind = Trivial>,
    {
        let len = self.len();
        if len == 0 {
            // The slice::from_raw_parts in the other branch requires a nonnull
            // and properly aligned data ptr. C++ standard does not guarantee
            // that data() on a vector with size 0 would return a nonnull
            // pointer or sufficiently aligned pointer, so using it would be
            // undefined behavior. Create our own empty slice in Rust instead
            // which upholds the invariants.
            &[]
        } else {
            let this = self as *const CxxVector<T> as *mut CxxVector<T>;
            let ptr = unsafe { T::__get_unchecked(this, 0) };
            unsafe { slice::from_raw_parts(ptr, len) }
        }
    }

    /// Returns a slice to the underlying contiguous array of elements by
    /// mutable reference.
    pub fn as_mut_slice(self: Pin<&mut Self>) -> &mut [T]
    where
        T: ExternType<Kind = Trivial>,
    {
        let len = self.len();
        if len == 0 {
            &mut []
        } else {
            let ptr = unsafe { T::__get_unchecked(self.get_unchecked_mut(), 0) };
            unsafe { slice::from_raw_parts_mut(ptr, len) }
        }
    }

    /// Returns an iterator over elements of type `&T`.
    pub fn iter(&self) -> Iter<T> {
        Iter { v: self, index: 0 }
    }

    /// Returns an iterator over elements of type `Pin<&mut T>`.
    pub fn iter_mut(self: Pin<&mut Self>) -> IterMut<T> {
        IterMut { v: self, index: 0 }
    }

    /// Appends an element to the back of the vector.
    ///
    /// Matches the behavior of C++ [std::vector\<T\>::push_back][push_back].
    ///
    /// [push_back]: https://en.cppreference.com/w/cpp/container/vector/push_back
    pub fn push(self: Pin<&mut Self>, value: T)
    where
        T: ExternType<Kind = Trivial>,
    {
        let mut value = ManuallyDrop::new(value);
        unsafe {
            // C++ calls move constructor followed by destructor on `value`.
            T::__push_back(self, &mut value);
        }
    }

    /// Removes the last element from a vector and returns it, or `None` if the
    /// vector is empty.
    pub fn pop(self: Pin<&mut Self>) -> Option<T>
    where
        T: ExternType<Kind = Trivial>,
    {
        if self.is_empty() {
            None
        } else {
            let mut value = MaybeUninit::uninit();
            Some(unsafe {
                T::__pop_back(self, &mut value);
                value.assume_init()
            })
        }
    }

    /// Ensures that this vector's capacity is at least `additional` elements
    /// larger than its length.
    ///
    /// The capacity may be increased by more than `additional` elements if the
    /// implementation chooses, to amortize the cost of frequent reallocations.
    ///
    /// **The meaning of the argument is not the same as
    /// [std::vector\<T\>::reserve][reserve] in C++.** The C++ standard library
    /// and Rust standard library both have a `reserve` method on vectors, but
    /// in C++ code the argument always refers to total capacity, whereas in
    /// Rust code it always refers to additional capacity. This API on
    /// `CxxVector` follows the Rust convention, the same way that for the
    /// length accessor we use the Rust conventional `len()` naming and not C++
    /// `size()`.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows usize, or if `T` is not
    /// move-constructible in C++.
    ///
    /// [reserve]: https://en.cppreference.com/w/cpp/container/vector/reserve.html
    pub fn reserve(self: Pin<&mut Self>, additional: usize) {
        let new_cap = self
            .len()
            .checked_add(additional)
            .expect("CxxVector capacity overflow");
        unsafe { T::__reserve(self, new_cap) }
    }
}

impl<T> Extend<T> for Pin<&mut CxxVector<T>>
where
    T: ExternType<Kind = Trivial> + VectorElement,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        self.as_mut().reserve(iter.size_hint().0);
        for element in iter {
            self.as_mut().push(element);
        }
    }
}

/// Iterator over elements of a `CxxVector` by shared reference.
///
/// The iterator element type is `&'a T`.
pub struct Iter<'a, T> {
    v: &'a CxxVector<T>,
    index: usize,
}

impl<'a, T> IntoIterator for &'a CxxVector<T>
where
    T: VectorElement,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: VectorElement,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.v.get(self.index)?;
        self.index += 1;
        Some(next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T>
where
    T: VectorElement,
{
    fn len(&self) -> usize {
        self.v.len() - self.index
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> where T: VectorElement {}

/// Iterator over elements of a `CxxVector` by pinned mutable reference.
///
/// The iterator element type is `Pin<&'a mut T>`.
pub struct IterMut<'a, T> {
    v: Pin<&'a mut CxxVector<T>>,
    index: usize,
}

impl<'a, T> IntoIterator for Pin<&'a mut CxxVector<T>>
where
    T: VectorElement,
{
    type Item = Pin<&'a mut T>;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: VectorElement,
{
    type Item = Pin<&'a mut T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.v.as_mut().index_mut(self.index)?;
        self.index += 1;
        // Extend lifetime to allow simultaneous holding of nonoverlapping
        // elements, analogous to slice::split_first_mut.
        unsafe {
            let ptr = Pin::into_inner_unchecked(next) as *mut T;
            Some(Pin::new_unchecked(&mut *ptr))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T>
where
    T: VectorElement,
{
    fn len(&self) -> usize {
        self.v.len() - self.index
    }
}

impl<'a, T> FusedIterator for IterMut<'a, T> where T: VectorElement {}

impl<T> Debug for CxxVector<T>
where
    T: VectorElement + Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_list().entries(self).finish()
    }
}

/// Trait bound for types which may be used as the `T` inside of a
/// `CxxVector<T>` in generic code.
///
/// This trait has no publicly callable or implementable methods. Implementing
/// it outside of the CXX codebase requires using [explicit shim trait impls],
/// adding the line `impl CxxVector<MyType> {}` in the same `cxx::bridge` that
/// defines `MyType`.
///
/// # Example
///
/// A bound `T: VectorElement` may be necessary when manipulating [`CxxVector`]
/// in generic code.
///
/// ```
/// use cxx::vector::{CxxVector, VectorElement};
/// use std::fmt::Display;
///
/// pub fn take_generic_vector<T>(vector: &CxxVector<T>)
/// where
///     T: VectorElement + Display,
/// {
///     println!("the vector elements are:");
///     for element in vector {
///         println!("  • {}", element);
///     }
/// }
/// ```
///
/// Writing the same generic function without a `VectorElement` trait bound
/// would not compile.
///
/// [explicit shim trait impls]: https://cxx.rs/extern-c++.html#explicit-shim-trait-impls
pub unsafe trait VectorElement: Sized {
    #[doc(hidden)]
    fn __typename(f: &mut fmt::Formatter) -> fmt::Result;
    #[doc(hidden)]
    fn __vector_new() -> *mut CxxVector<Self>;
    #[doc(hidden)]
    fn __vector_size(v: &CxxVector<Self>) -> usize;
    #[doc(hidden)]
    fn __vector_capacity(v: &CxxVector<Self>) -> usize;
    #[doc(hidden)]
    unsafe fn __get_unchecked(v: *mut CxxVector<Self>, pos: usize) -> *mut Self;
    #[doc(hidden)]
    unsafe fn __reserve(v: Pin<&mut CxxVector<Self>>, new_cap: usize);
    #[doc(hidden)]
    unsafe fn __push_back(v: Pin<&mut CxxVector<Self>>, value: &mut ManuallyDrop<Self>) {
        // Opaque C type vector elements do not get this method because they can
        // never exist by value on the Rust side of the bridge.
        let _ = v;
        let _ = value;
        unreachable!()
    }
    #[doc(hidden)]
    unsafe fn __pop_back(v: Pin<&mut CxxVector<Self>>, out: &mut MaybeUninit<Self>) {
        // Opaque C type vector elements do not get this method because they can
        // never exist by value on the Rust side of the bridge.
        let _ = v;
        let _ = out;
        unreachable!()
    }
    #[doc(hidden)]
    fn __unique_ptr_null() -> MaybeUninit<*mut c_void>;
    #[doc(hidden)]
    unsafe fn __unique_ptr_raw(raw: *mut CxxVector<Self>) -> MaybeUninit<*mut c_void>;
    #[doc(hidden)]
    unsafe fn __unique_ptr_get(repr: MaybeUninit<*mut c_void>) -> *const CxxVector<Self>;
    #[doc(hidden)]
    unsafe fn __unique_ptr_release(repr: MaybeUninit<*mut c_void>) -> *mut CxxVector<Self>;
    #[doc(hidden)]
    unsafe fn __unique_ptr_drop(repr: MaybeUninit<*mut c_void>);
}

macro_rules! vector_element_by_value_methods {
    (opaque, $segment:expr, $ty:ty) => {};
    (trivial, $segment:expr, $ty:ty) => {
        unsafe fn __push_back(v: Pin<&mut CxxVector<$ty>>, value: &mut ManuallyDrop<$ty>) {
            extern "C" {
                #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$push_back")]
                fn __push_back(_: Pin<&mut CxxVector<$ty>>, _: &mut ManuallyDrop<$ty>);
            }
            unsafe { __push_back(v, value) }
        }
        unsafe fn __pop_back(v: Pin<&mut CxxVector<$ty>>, out: &mut MaybeUninit<$ty>) {
            extern "C" {
                #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$pop_back")]
                fn __pop_back(_: Pin<&mut CxxVector<$ty>>, _: &mut MaybeUninit<$ty>);
            }
            unsafe { __pop_back(v, out) }
        }
    };
}

macro_rules! impl_vector_element {
    ($kind:ident, $segment:expr, $name:expr, $ty:ty) => {
        const_assert_eq!(0, mem::size_of::<CxxVector<$ty>>());
        const_assert_eq!(1, mem::align_of::<CxxVector<$ty>>());

        unsafe impl VectorElement for $ty {
            fn __typename(f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str($name)
            }
            fn __vector_new() -> *mut CxxVector<Self> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$new")]
                    fn __vector_new() -> *mut CxxVector<$ty>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &CxxVector<$ty>) -> usize {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$size")]
                    fn __vector_size(_: &CxxVector<$ty>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            fn __vector_capacity(v: &CxxVector<$ty>) -> usize {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$capacity")]
                    fn __vector_capacity(_: &CxxVector<$ty>) -> usize;
                }
                unsafe { __vector_capacity(v) }
            }
            unsafe fn __get_unchecked(v: *mut CxxVector<$ty>, pos: usize) -> *mut $ty {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$get_unchecked")]
                    fn __get_unchecked(_: *mut CxxVector<$ty>, _: usize) -> *mut $ty;
                }
                unsafe { __get_unchecked(v, pos) }
            }
            unsafe fn __reserve(v: Pin<&mut CxxVector<$ty>>, new_cap: usize) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$vector$", $segment, "$reserve")]
                    fn __reserve(_: Pin<&mut CxxVector<$ty>>, _: usize);
                }
                unsafe { __reserve(v, new_cap) }
            }
            vector_element_by_value_methods!($kind, $segment, $ty);
            fn __unique_ptr_null() -> MaybeUninit<*mut c_void> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$null")]
                    fn __unique_ptr_null(this: *mut MaybeUninit<*mut c_void>);
                }
                let mut repr = MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(raw: *mut CxxVector<Self>) -> MaybeUninit<*mut c_void> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$raw")]
                    fn __unique_ptr_raw(this: *mut MaybeUninit<*mut c_void>, raw: *mut CxxVector<$ty>);
                }
                let mut repr = MaybeUninit::uninit();
                unsafe { __unique_ptr_raw(&mut repr, raw) }
                repr
            }
            unsafe fn __unique_ptr_get(repr: MaybeUninit<*mut c_void>) -> *const CxxVector<Self> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$get")]
                    fn __unique_ptr_get(this: *const MaybeUninit<*mut c_void>) -> *const CxxVector<$ty>;
                }
                unsafe { __unique_ptr_get(&repr) }
            }
            unsafe fn __unique_ptr_release(mut repr: MaybeUninit<*mut c_void>) -> *mut CxxVector<Self> {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$release")]
                    fn __unique_ptr_release(this: *mut MaybeUninit<*mut c_void>) -> *mut CxxVector<$ty>;
                }
                unsafe { __unique_ptr_release(&mut repr) }
            }
            unsafe fn __unique_ptr_drop(mut repr: MaybeUninit<*mut c_void>) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$unique_ptr$std$vector$", $segment, "$drop")]
                    fn __unique_ptr_drop(this: *mut MaybeUninit<*mut c_void>);
                }
                unsafe { __unique_ptr_drop(&mut repr) }
            }
        }
    };
}

macro_rules! impl_vector_element_for_primitive {
    ($ty:ident) => {
        impl_vector_element!(trivial, stringify!($ty), stringify!($ty), $ty);
    };
}

impl_vector_element_for_primitive!(u8);
impl_vector_element_for_primitive!(u16);
impl_vector_element_for_primitive!(u32);
impl_vector_element_for_primitive!(u64);
impl_vector_element_for_primitive!(usize);
impl_vector_element_for_primitive!(i8);
impl_vector_element_for_primitive!(i16);
impl_vector_element_for_primitive!(i32);
impl_vector_element_for_primitive!(i64);
impl_vector_element_for_primitive!(isize);
impl_vector_element_for_primitive!(f32);
impl_vector_element_for_primitive!(f64);

impl_vector_element!(opaque, "string", "CxxString", CxxString);
