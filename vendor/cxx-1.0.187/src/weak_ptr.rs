use crate::shared_ptr::{SharedPtr, SharedPtrTarget};
use crate::string::CxxString;
use core::ffi::c_void;
use core::fmt::{self, Debug};
use core::marker::PhantomData;
use core::mem::MaybeUninit;

/// Binding to C++ `std::weak_ptr<T>`.
///
/// The typical way to construct a WeakPtr from Rust is by [downgrading] from a
/// SharedPtr.
///
/// [downgrading]: crate::SharedPtr::downgrade
#[repr(C)]
pub struct WeakPtr<T>
where
    T: WeakPtrTarget,
{
    repr: [MaybeUninit<*mut c_void>; 2],
    ty: PhantomData<T>,
}

impl<T> WeakPtr<T>
where
    T: WeakPtrTarget,
{
    /// Makes a new WeakPtr wrapping a null pointer.
    ///
    /// Matches the behavior of default-constructing a std::weak\_ptr.
    pub fn null() -> Self {
        let mut weak_ptr = MaybeUninit::<WeakPtr<T>>::uninit();
        let new = weak_ptr.as_mut_ptr().cast();
        unsafe {
            T::__null(new);
            weak_ptr.assume_init()
        }
    }

    /// Upgrades a non-owning reference into an owning reference if possible,
    /// otherwise to a null reference.
    ///
    /// Matches the behavior of [std::weak_ptr\<T\>::lock](https://en.cppreference.com/w/cpp/memory/weak_ptr/lock).
    pub fn upgrade(&self) -> SharedPtr<T>
    where
        T: SharedPtrTarget,
    {
        let this = self as *const Self as *const c_void;
        let mut shared_ptr = MaybeUninit::<SharedPtr<T>>::uninit();
        let new = shared_ptr.as_mut_ptr().cast();
        unsafe {
            T::__upgrade(this, new);
            shared_ptr.assume_init()
        }
    }
}

unsafe impl<T> Send for WeakPtr<T> where T: Send + Sync + WeakPtrTarget {}
unsafe impl<T> Sync for WeakPtr<T> where T: Send + Sync + WeakPtrTarget {}

impl<T> Clone for WeakPtr<T>
where
    T: WeakPtrTarget,
{
    fn clone(&self) -> Self {
        let mut weak_ptr = MaybeUninit::<WeakPtr<T>>::uninit();
        let new = weak_ptr.as_mut_ptr().cast();
        let this = self as *const Self as *mut c_void;
        unsafe {
            T::__clone(this, new);
            weak_ptr.assume_init()
        }
    }
}

impl<T> Drop for WeakPtr<T>
where
    T: WeakPtrTarget,
{
    fn drop(&mut self) {
        let this = self as *mut Self as *mut c_void;
        unsafe { T::__drop(this) }
    }
}

impl<T> Debug for WeakPtr<T>
where
    T: Debug + WeakPtrTarget + SharedPtrTarget,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.upgrade(), formatter)
    }
}

/// Trait bound for types which may be used as the `T` inside of a `WeakPtr<T>`
/// in generic code.
///
/// This trait has no publicly callable or implementable methods. Implementing
/// it outside of the CXX codebase is not supported.
pub unsafe trait WeakPtrTarget {
    #[doc(hidden)]
    fn __typename(f: &mut fmt::Formatter) -> fmt::Result;
    #[doc(hidden)]
    unsafe fn __null(new: *mut c_void);
    #[doc(hidden)]
    unsafe fn __clone(this: *const c_void, new: *mut c_void);
    #[doc(hidden)]
    unsafe fn __downgrade(shared: *const c_void, new: *mut c_void);
    #[doc(hidden)]
    unsafe fn __upgrade(weak: *const c_void, shared: *mut c_void);
    #[doc(hidden)]
    unsafe fn __drop(this: *mut c_void);
}

macro_rules! impl_weak_ptr_target {
    ($segment:expr, $name:expr, $ty:ty) => {
        unsafe impl WeakPtrTarget for $ty {
            fn __typename(f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str($name)
            }
            unsafe fn __null(new: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$weak_ptr$", $segment, "$null")]
                    fn __null(new: *mut c_void);
                }
                unsafe { __null(new) }
            }
            unsafe fn __clone(this: *const c_void, new: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$weak_ptr$", $segment, "$clone")]
                    fn __clone(this: *const c_void, new: *mut c_void);
                }
                unsafe { __clone(this, new) }
            }
            unsafe fn __downgrade(shared: *const c_void, weak: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$weak_ptr$", $segment, "$downgrade")]
                    fn __downgrade(shared: *const c_void, weak: *mut c_void);
                }
                unsafe { __downgrade(shared, weak) }
            }
            unsafe fn __upgrade(weak: *const c_void, shared: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$weak_ptr$", $segment, "$upgrade")]
                    fn __upgrade(weak: *const c_void, shared: *mut c_void);
                }
                unsafe { __upgrade(weak, shared) }
            }
            unsafe fn __drop(this: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$weak_ptr$", $segment, "$drop")]
                    fn __drop(this: *mut c_void);
                }
                unsafe { __drop(this) }
            }
        }
    };
}

macro_rules! impl_weak_ptr_target_for_primitive {
    ($ty:ident) => {
        impl_weak_ptr_target!(stringify!($ty), stringify!($ty), $ty);
    };
}

impl_weak_ptr_target_for_primitive!(bool);
impl_weak_ptr_target_for_primitive!(u8);
impl_weak_ptr_target_for_primitive!(u16);
impl_weak_ptr_target_for_primitive!(u32);
impl_weak_ptr_target_for_primitive!(u64);
impl_weak_ptr_target_for_primitive!(usize);
impl_weak_ptr_target_for_primitive!(i8);
impl_weak_ptr_target_for_primitive!(i16);
impl_weak_ptr_target_for_primitive!(i32);
impl_weak_ptr_target_for_primitive!(i64);
impl_weak_ptr_target_for_primitive!(isize);
impl_weak_ptr_target_for_primitive!(f32);
impl_weak_ptr_target_for_primitive!(f64);

impl_weak_ptr_target!("string", "CxxString", CxxString);
