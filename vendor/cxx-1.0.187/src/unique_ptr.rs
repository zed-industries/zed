use crate::cxx_vector::{CxxVector, VectorElement};
use crate::extern_type::ExternType;
use crate::fmt::display;
use crate::kind::Trivial;
use crate::string::CxxString;
#[cfg(feature = "std")]
use alloc::string::String;
#[cfg(feature = "std")]
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ffi::c_void;
use core::fmt::{self, Debug, Display};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
#[cfg(feature = "std")]
use std::io::{self, IoSlice, Read, Seek, SeekFrom, Write};

/// Binding to C++ `std::unique_ptr<T, std::default_delete<T>>`.
#[repr(C)]
pub struct UniquePtr<T>
where
    T: UniquePtrTarget,
{
    repr: MaybeUninit<*mut c_void>,
    ty: PhantomData<T>,
}

impl<T> UniquePtr<T>
where
    T: UniquePtrTarget,
{
    /// Makes a new UniquePtr wrapping a null pointer.
    ///
    /// Matches the behavior of default-constructing a std::unique\_ptr.
    pub fn null() -> Self {
        UniquePtr {
            repr: T::__null(),
            ty: PhantomData,
        }
    }

    /// Allocates memory on the heap and makes a UniquePtr pointing to it.
    pub fn new(value: T) -> Self
    where
        T: ExternType<Kind = Trivial>,
    {
        UniquePtr {
            repr: T::__new(value),
            ty: PhantomData,
        }
    }

    /// Checks whether the UniquePtr does not own an object.
    ///
    /// This is the opposite of [std::unique_ptr\<T\>::operator bool](https://en.cppreference.com/w/cpp/memory/unique_ptr/operator_bool).
    pub fn is_null(&self) -> bool {
        self.as_ptr().is_null()
    }

    /// Returns a reference to the object owned by this UniquePtr if any,
    /// otherwise None.
    pub fn as_ref(&self) -> Option<&T> {
        let ptr = self.as_ptr();
        unsafe { ptr.as_ref() }
    }

    /// Returns a mutable pinned reference to the object owned by this UniquePtr
    /// if any, otherwise None.
    pub fn as_mut(&mut self) -> Option<Pin<&mut T>> {
        let ptr = self.as_mut_ptr();
        unsafe {
            let mut_reference = ptr.as_mut()?;
            Some(Pin::new_unchecked(mut_reference))
        }
    }

    /// Returns a mutable pinned reference to the object owned by this
    /// UniquePtr.
    ///
    /// # Panics
    ///
    /// Panics if the UniquePtr holds a null pointer.
    pub fn pin_mut(&mut self) -> Pin<&mut T> {
        match self.as_mut() {
            Some(target) => target,
            None => panic!(
                "called pin_mut on a null UniquePtr<{}>",
                display(T::__typename),
            ),
        }
    }

    /// Returns a raw const pointer to the object owned by this UniquePtr if
    /// any, otherwise the null pointer.
    pub fn as_ptr(&self) -> *const T {
        unsafe { T::__get(self.repr) }
    }

    /// Returns a raw mutable pointer to the object owned by this UniquePtr if
    /// any, otherwise the null pointer.
    ///
    /// As with [std::unique_ptr\<T\>::get](https://en.cppreference.com/w/cpp/memory/unique_ptr/get),
    /// this doesn't require that you hold an exclusive reference to the
    /// UniquePtr. This differs from Rust norms, so extra care should be taken
    /// in the way the pointer is used.
    pub fn as_mut_ptr(&self) -> *mut T {
        self.as_ptr() as *mut T
    }

    /// Consumes the UniquePtr, releasing its ownership of the heap-allocated T.
    ///
    /// Matches the behavior of [std::unique_ptr\<T\>::release](https://en.cppreference.com/w/cpp/memory/unique_ptr/release).
    pub fn into_raw(self) -> *mut T {
        let ptr = unsafe { T::__release(self.repr) };
        mem::forget(self);
        ptr
    }

    /// Constructs a UniquePtr retaking ownership of a pointer previously
    /// obtained from `into_raw`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory
    /// problems. For example a double-free may occur if the function is called
    /// twice on the same raw pointer.
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        UniquePtr {
            repr: unsafe { T::__raw(raw) },
            ty: PhantomData,
        }
    }
}

unsafe impl<T> Send for UniquePtr<T> where T: Send + UniquePtrTarget {}
unsafe impl<T> Sync for UniquePtr<T> where T: Sync + UniquePtrTarget {}

// UniquePtr is not a self-referential type and is safe to move out of a Pin,
// regardless whether the pointer's target is Unpin.
impl<T> Unpin for UniquePtr<T> where T: UniquePtrTarget {}

impl<T> Drop for UniquePtr<T>
where
    T: UniquePtrTarget,
{
    fn drop(&mut self) {
        unsafe { T::__drop(self.repr) }
    }
}

impl<T> Deref for UniquePtr<T>
where
    T: UniquePtrTarget,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null UniquePtr<{}>",
                display(T::__typename),
            ),
        }
    }
}

impl<T> DerefMut for UniquePtr<T>
where
    T: UniquePtrTarget + Unpin,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.as_mut() {
            Some(target) => Pin::into_inner(target),
            None => panic!(
                "called deref_mut on a null UniquePtr<{}>",
                display(T::__typename),
            ),
        }
    }
}

impl<T> Debug for UniquePtr<T>
where
    T: Debug + UniquePtrTarget,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Debug::fmt(value, formatter),
        }
    }
}

impl<T> Display for UniquePtr<T>
where
    T: Display + UniquePtrTarget,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Display::fmt(value, formatter),
        }
    }
}

impl<T> PartialEq for UniquePtr<T>
where
    T: PartialEq + UniquePtrTarget,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T> Eq for UniquePtr<T> where T: Eq + UniquePtrTarget {}

impl<T> PartialOrd for UniquePtr<T>
where
    T: PartialOrd + UniquePtrTarget,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.as_ref(), &other.as_ref())
    }
}

impl<T> Ord for UniquePtr<T>
where
    T: Ord + UniquePtrTarget,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.as_ref(), &other.as_ref())
    }
}

impl<T> Hash for UniquePtr<T>
where
    T: Hash + UniquePtrTarget,
{
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.as_ref().hash(hasher);
    }
}

/// Forwarding `Read` trait implementation in a manner similar to `Box<T>`.
///
/// Note that the implementation will panic for null `UniquePtr<T>`.
#[cfg(feature = "std")]
impl<T> Read for UniquePtr<T>
where
    for<'a> Pin<&'a mut T>: Read,
    T: UniquePtrTarget,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.pin_mut().read(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.pin_mut().read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.pin_mut().read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.pin_mut().read_exact(buf)
    }

    // TODO: Foward other `Read` trait methods when they get stabilized (e.g.
    // `read_buf` and/or `is_read_vectored`).
}

/// Forwarding `Seek` trait implementation in a manner similar to `Box<T>`.
///
/// Note that the implementation will panic for null `UniquePtr<T>`.
#[cfg(feature = "std")]
impl<T> Seek for UniquePtr<T>
where
    for<'a> Pin<&'a mut T>: Seek,
    T: UniquePtrTarget,
{
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pin_mut().seek(pos)
    }

    #[inline]
    fn rewind(&mut self) -> io::Result<()> {
        self.pin_mut().rewind()
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        self.pin_mut().stream_position()
    }

    #[inline]
    fn seek_relative(&mut self, offset: i64) -> io::Result<()> {
        self.pin_mut().seek_relative(offset)
    }

    // TODO: Foward other `Seek` trait methods if/when possible:
    // * `stream_len`: If/when stabilized
}

/// Forwarding `Write` trait implementation in a manner similar to `Box<T>`.
///
/// Note that the implementation will panic for null `UniquePtr<T>`.
#[cfg(feature = "std")]
impl<T> Write for UniquePtr<T>
where
    for<'a> Pin<&'a mut T>: Write,
    T: UniquePtrTarget,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.pin_mut().write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        self.pin_mut().write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.pin_mut().flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.pin_mut().write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
        self.pin_mut().write_fmt(fmt)
    }

    // TODO: Foward other `Write` trait methods when they get stabilized (e.g.
    // `write_all_vectored` and/or `is_write_vectored`).
}

/// Trait bound for types which may be used as the `T` inside of a
/// `UniquePtr<T>` in generic code.
///
/// This trait has no publicly callable or implementable methods. Implementing
/// it outside of the CXX codebase is not supported.
///
/// # Example
///
/// A bound `T: UniquePtrTarget` may be necessary when manipulating
/// [`UniquePtr`] in generic code.
///
/// ```
/// use cxx::memory::{UniquePtr, UniquePtrTarget};
/// use std::fmt::Display;
///
/// pub fn take_generic_ptr<T>(ptr: UniquePtr<T>)
/// where
///     T: UniquePtrTarget + Display,
/// {
///     println!("the unique_ptr points to: {}", *ptr);
/// }
/// ```
///
/// Writing the same generic function without a `UniquePtrTarget` trait bound
/// would not compile.
pub unsafe trait UniquePtrTarget {
    #[doc(hidden)]
    fn __typename(f: &mut fmt::Formatter) -> fmt::Result;
    #[doc(hidden)]
    fn __null() -> MaybeUninit<*mut c_void>;
    #[doc(hidden)]
    fn __new(value: Self) -> MaybeUninit<*mut c_void>
    where
        Self: Sized,
    {
        // Opaque C types do not get this method because they can never exist by
        // value on the Rust side of the bridge.
        let _ = value;
        unreachable!()
    }
    #[doc(hidden)]
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void>;
    #[doc(hidden)]
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self;
    #[doc(hidden)]
    unsafe fn __release(repr: MaybeUninit<*mut c_void>) -> *mut Self;
    #[doc(hidden)]
    unsafe fn __drop(repr: MaybeUninit<*mut c_void>);
}

extern "C" {
    #[link_name = "cxxbridge1$unique_ptr$std$string$null"]
    fn unique_ptr_std_string_null(this: *mut MaybeUninit<*mut c_void>);
    #[link_name = "cxxbridge1$unique_ptr$std$string$raw"]
    fn unique_ptr_std_string_raw(this: *mut MaybeUninit<*mut c_void>, raw: *mut CxxString);
    #[link_name = "cxxbridge1$unique_ptr$std$string$get"]
    fn unique_ptr_std_string_get(this: *const MaybeUninit<*mut c_void>) -> *const CxxString;
    #[link_name = "cxxbridge1$unique_ptr$std$string$release"]
    fn unique_ptr_std_string_release(this: *mut MaybeUninit<*mut c_void>) -> *mut CxxString;
    #[link_name = "cxxbridge1$unique_ptr$std$string$drop"]
    fn unique_ptr_std_string_drop(this: *mut MaybeUninit<*mut c_void>);
}

unsafe impl UniquePtrTarget for CxxString {
    fn __typename(f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CxxString")
    }
    fn __null() -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unsafe {
            unique_ptr_std_string_null(&raw mut repr);
        }
        repr
    }
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void> {
        let mut repr = MaybeUninit::uninit();
        unsafe { unique_ptr_std_string_raw(&raw mut repr, raw) }
        repr
    }
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self {
        unsafe { unique_ptr_std_string_get(&raw const repr) }
    }
    unsafe fn __release(mut repr: MaybeUninit<*mut c_void>) -> *mut Self {
        unsafe { unique_ptr_std_string_release(&raw mut repr) }
    }
    unsafe fn __drop(mut repr: MaybeUninit<*mut c_void>) {
        unsafe { unique_ptr_std_string_drop(&raw mut repr) }
    }
}

unsafe impl<T> UniquePtrTarget for CxxVector<T>
where
    T: VectorElement,
{
    fn __typename(f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CxxVector<{}>", display(T::__typename))
    }
    fn __null() -> MaybeUninit<*mut c_void> {
        T::__unique_ptr_null()
    }
    unsafe fn __raw(raw: *mut Self) -> MaybeUninit<*mut c_void> {
        unsafe { T::__unique_ptr_raw(raw) }
    }
    unsafe fn __get(repr: MaybeUninit<*mut c_void>) -> *const Self {
        unsafe { T::__unique_ptr_get(repr) }
    }
    unsafe fn __release(repr: MaybeUninit<*mut c_void>) -> *mut Self {
        unsafe { T::__unique_ptr_release(repr) }
    }
    unsafe fn __drop(repr: MaybeUninit<*mut c_void>) {
        unsafe { T::__unique_ptr_drop(repr) }
    }
}
