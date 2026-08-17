use crate::extern_type::ExternType;
use crate::fmt::display;
use crate::kind::Trivial;
use crate::string::CxxString;
use crate::unique_ptr::{UniquePtr, UniquePtrTarget};
use crate::weak_ptr::{WeakPtr, WeakPtrTarget};
use core::cmp::Ordering;
use core::ffi::c_void;
use core::fmt::{self, Debug, Display};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::pin::Pin;

/// Binding to C++ `std::shared_ptr<T>`.
///
/// <div class="warning">
///
/// **WARNING:** Unlike Rust's `Arc<T>`, a C++ shared pointer manipulates
/// pointers to 2 separate objects in general.
///
/// 1. One is the **managed** pointer, and its identity is associated with
///    shared ownership of a strong and weak count shared by other SharedPtr and
///    WeakPtr instances having the same managed pointer.
///
/// 2. The other is the **stored** pointer, which is commonly either the same as
///    the managed pointer, or is a pointer into some member of the managed
///    object, but can be any unrelated pointer in general.
///
/// The managed pointer is the one passed to a deleter upon the strong count
/// reaching zero, but the stored pointer is the one accessed by deref
/// operations and methods such as `is_null`.
///
/// A shared pointer is considered **empty** if the strong count is zero,
/// meaning the managed pointer has been deleted or is about to be deleted. A
/// shared pointer is considered **null** if the stored pointer is the null
/// pointer. All combinations are possible. To be explicit, a shared pointer can
/// be nonempty and nonnull, or nonempty and null, or empty and nonnull, or
/// empty and null. In general all of these cases need to be considered when
/// handling a SharedPtr.
///
/// </div>
#[repr(C)]
pub struct SharedPtr<T>
where
    T: SharedPtrTarget,
{
    repr: [MaybeUninit<*mut c_void>; 2],
    ty: PhantomData<T>,
}

impl<T> SharedPtr<T>
where
    T: SharedPtrTarget,
{
    /// Makes a new SharedPtr that is both **empty** and **null**.
    ///
    /// Matches the behavior of default-constructing a std::shared\_ptr.
    pub fn null() -> Self {
        let mut shared_ptr = MaybeUninit::<SharedPtr<T>>::uninit();
        let new = shared_ptr.as_mut_ptr().cast();
        unsafe {
            T::__null(new);
            shared_ptr.assume_init()
        }
    }

    /// Allocates memory on the heap and makes a SharedPtr owner for it.
    ///
    /// The shared pointer will be **nonempty** and **nonnull**.
    pub fn new(value: T) -> Self
    where
        T: ExternType<Kind = Trivial>,
    {
        let mut shared_ptr = MaybeUninit::<SharedPtr<T>>::uninit();
        let new = shared_ptr.as_mut_ptr().cast();
        unsafe {
            T::__new(value, new);
            shared_ptr.assume_init()
        }
    }

    /// Creates a shared pointer from a C++ heap-allocated pointer.
    ///
    /// Matches the behavior of std::shared\_ptr's constructor `explicit shared_ptr(T*)`.
    ///
    /// The SharedPtr gains ownership of the pointer and will call
    /// `std::default_delete` on it when the refcount goes to zero.
    ///
    /// The object pointed to by the input pointer is not relocated by this
    /// operation, so any pointers into this data structure elsewhere in the
    /// program continue to be valid.
    ///
    /// The resulting shared pointer is **nonempty** regardless of whether the
    /// input pointer is null, but may be either **null** or **nonnull**.
    ///
    /// # Panics
    ///
    /// Panics if `T` is an incomplete type (including `void`) or is not
    /// destructible.
    ///
    /// # Safety
    ///
    /// Pointer must either be null or point to a valid instance of T
    /// heap-allocated in C++ by `new`.
    #[track_caller]
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        let mut shared_ptr = MaybeUninit::<SharedPtr<T>>::uninit();
        let new = shared_ptr.as_mut_ptr().cast();
        unsafe {
            T::__raw(new, raw);
            shared_ptr.assume_init()
        }
    }

    /// Checks whether the SharedPtr holds a null stored pointer.
    ///
    /// This is the opposite of [std::shared_ptr\<T\>::operator bool](https://en.cppreference.com/w/cpp/memory/shared_ptr/operator_bool).
    ///
    /// <div class="warning">
    ///
    /// This method is unrelated to the state of the reference count. It is
    /// possible to have a SharedPtr that is nonnull but empty (has a refcount
    /// of 0), typically from having been constructed using the alias
    /// constructors in C++. Inversely, it is also possible to be null and
    /// nonempty.
    ///
    /// </div>
    pub fn is_null(&self) -> bool {
        let this = self as *const Self as *const c_void;
        let ptr = unsafe { T::__get(this) };
        ptr.is_null()
    }

    /// Returns a reference to the object pointed to by the stored pointer if
    /// nonnull, otherwise None.
    ///
    /// <div class="warning">
    ///
    /// The shared pointer's managed object may or may not already have been
    /// destroyed.
    ///
    /// </div>
    pub fn as_ref(&self) -> Option<&T> {
        let ptr = self.as_ptr();
        unsafe { ptr.as_ref() }
    }

    /// Returns a mutable pinned reference to the object pointed to by the
    /// stored pointer.
    ///
    /// <div class="warning">
    ///
    /// The shared pointer's managed object may or may not already have been
    /// destroyed.
    ///
    /// </div>
    ///
    /// # Panics
    ///
    /// Panics if the SharedPtr holds a null stored pointer.
    ///
    /// # Safety
    ///
    /// This method makes no attempt to ascertain the state of the reference
    /// count. In particular, unlike `Arc::get_mut`, we do not enforce absence
    /// of other SharedPtr and WeakPtr referring to the same data as this one.
    /// As always, it is Undefined Behavior to have simultaneous references to
    /// the same value while a Rust exclusive reference to it exists anywhere in
    /// the program.
    ///
    /// For the special case of CXX [opaque C++ types], this method can be used
    /// to safely call thread-safe non-const member functions on a C++ object
    /// without regard for whether the reference is exclusive. This capability
    /// applies only to opaque types `extern "C++" { type T; }`. It does not
    /// apply to extern types defined with a non-opaque Rust representation
    /// `extern "C++" { type T = ...; }`.
    ///
    /// [opaque C++ types]: https://cxx.rs/extern-c++.html#opaque-c-types
    pub unsafe fn pin_mut_unchecked(&mut self) -> Pin<&mut T> {
        let ptr = self.as_mut_ptr();
        match unsafe { ptr.as_mut() } {
            Some(target) => unsafe { Pin::new_unchecked(target) },
            None => panic!(
                "called pin_mut_unchecked on a null SharedPtr<{}>",
                display(T::__typename),
            ),
        }
    }

    /// Returns the SharedPtr's stored pointer as a raw const pointer.
    pub fn as_ptr(&self) -> *const T {
        let this = self as *const Self as *const c_void;
        unsafe { T::__get(this) }
    }

    /// Returns the SharedPtr's stored pointer as a raw mutable pointer.
    ///
    /// As with [std::shared_ptr\<T\>::get](https://en.cppreference.com/w/cpp/memory/shared_ptr/get),
    /// this doesn't require that you hold an exclusive reference to the
    /// SharedPtr. This differs from Rust norms, so extra care should be taken
    /// in the way the pointer is used.
    pub fn as_mut_ptr(&self) -> *mut T {
        self.as_ptr() as *mut T
    }

    /// Constructs new WeakPtr as a non-owning reference to the object managed
    /// by `self`. If `self` manages no object, the WeakPtr manages no object
    /// too.
    ///
    /// Matches the behavior of [std::weak_ptr\<T\>::weak_ptr(const std::shared_ptr\<T\> \&)](https://en.cppreference.com/w/cpp/memory/weak_ptr/weak_ptr).
    pub fn downgrade(&self) -> WeakPtr<T>
    where
        T: WeakPtrTarget,
    {
        let this = self as *const Self as *const c_void;
        let mut weak_ptr = MaybeUninit::<WeakPtr<T>>::uninit();
        let new = weak_ptr.as_mut_ptr().cast();
        unsafe {
            T::__downgrade(this, new);
            weak_ptr.assume_init()
        }
    }
}

unsafe impl<T> Send for SharedPtr<T> where T: Send + Sync + SharedPtrTarget {}
unsafe impl<T> Sync for SharedPtr<T> where T: Send + Sync + SharedPtrTarget {}

impl<T> Clone for SharedPtr<T>
where
    T: SharedPtrTarget,
{
    fn clone(&self) -> Self {
        let mut shared_ptr = MaybeUninit::<SharedPtr<T>>::uninit();
        let new = shared_ptr.as_mut_ptr().cast();
        let this = self as *const Self as *mut c_void;
        unsafe {
            T::__clone(this, new);
            shared_ptr.assume_init()
        }
    }
}

// SharedPtr is not a self-referential type and is safe to move out of a Pin,
// regardless whether the pointer's target is Unpin.
impl<T> Unpin for SharedPtr<T> where T: SharedPtrTarget {}

impl<T> Drop for SharedPtr<T>
where
    T: SharedPtrTarget,
{
    fn drop(&mut self) {
        let this = self as *mut Self as *mut c_void;
        unsafe { T::__drop(this) }
    }
}

impl<T> Deref for SharedPtr<T>
where
    T: SharedPtrTarget,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null SharedPtr<{}>",
                display(T::__typename),
            ),
        }
    }
}

impl<T> Debug for SharedPtr<T>
where
    T: Debug + SharedPtrTarget,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Debug::fmt(value, formatter),
        }
    }
}

impl<T> Display for SharedPtr<T>
where
    T: Display + SharedPtrTarget,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Display::fmt(value, formatter),
        }
    }
}

impl<T> PartialEq for SharedPtr<T>
where
    T: PartialEq + SharedPtrTarget,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T> Eq for SharedPtr<T> where T: Eq + SharedPtrTarget {}

impl<T> PartialOrd for SharedPtr<T>
where
    T: PartialOrd + SharedPtrTarget,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.as_ref(), &other.as_ref())
    }
}

impl<T> Ord for SharedPtr<T>
where
    T: Ord + SharedPtrTarget,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.as_ref(), &other.as_ref())
    }
}

impl<T> Hash for SharedPtr<T>
where
    T: Hash + SharedPtrTarget,
{
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.as_ref().hash(hasher);
    }
}

impl<T> From<UniquePtr<T>> for SharedPtr<T>
where
    T: UniquePtrTarget + SharedPtrTarget,
{
    fn from(unique: UniquePtr<T>) -> Self {
        unsafe { SharedPtr::from_raw(UniquePtr::into_raw(unique)) }
    }
}

/// Trait bound for types which may be used as the `T` inside of a
/// `SharedPtr<T>` in generic code.
///
/// This trait has no publicly callable or implementable methods. Implementing
/// it outside of the CXX codebase is not supported.
///
/// # Example
///
/// A bound `T: SharedPtrTarget` may be necessary when manipulating
/// [`SharedPtr`] in generic code.
///
/// ```
/// use cxx::memory::{SharedPtr, SharedPtrTarget};
/// use std::fmt::Display;
///
/// pub fn take_generic_ptr<T>(ptr: SharedPtr<T>)
/// where
///     T: SharedPtrTarget + Display,
/// {
///     println!("the shared_ptr points to: {}", *ptr);
/// }
/// ```
///
/// Writing the same generic function without a `SharedPtrTarget` trait bound
/// would not compile.
pub unsafe trait SharedPtrTarget {
    #[doc(hidden)]
    fn __typename(f: &mut fmt::Formatter) -> fmt::Result;
    #[doc(hidden)]
    unsafe fn __null(new: *mut c_void);
    #[doc(hidden)]
    unsafe fn __new(value: Self, new: *mut c_void)
    where
        Self: Sized,
    {
        // Opaque C types do not get this method because they can never exist by
        // value on the Rust side of the bridge.
        let _ = value;
        let _ = new;
        unreachable!()
    }
    #[doc(hidden)]
    unsafe fn __raw(new: *mut c_void, raw: *mut Self);
    #[doc(hidden)]
    unsafe fn __clone(this: *const c_void, new: *mut c_void);
    #[doc(hidden)]
    unsafe fn __get(this: *const c_void) -> *const Self;
    #[doc(hidden)]
    unsafe fn __drop(this: *mut c_void);
}

macro_rules! impl_shared_ptr_target {
    ($segment:expr, $name:expr, $ty:ty) => {
        unsafe impl SharedPtrTarget for $ty {
            fn __typename(f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str($name)
            }
            unsafe fn __null(new: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$shared_ptr$", $segment, "$null")]
                    fn __null(new: *mut c_void);
                }
                unsafe { __null(new) }
            }
            unsafe fn __new(value: Self, new: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$shared_ptr$", $segment, "$uninit")]
                    fn __uninit(new: *mut c_void) -> *mut c_void;
                }
                unsafe { __uninit(new).cast::<$ty>().write(value) }
            }
            unsafe fn __raw(new: *mut c_void, raw: *mut Self) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$shared_ptr$", $segment, "$raw")]
                    fn __raw(new: *mut c_void, raw: *mut c_void);
                }
                unsafe { __raw(new, raw as *mut c_void) }
            }
            unsafe fn __clone(this: *const c_void, new: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$shared_ptr$", $segment, "$clone")]
                    fn __clone(this: *const c_void, new: *mut c_void);
                }
                unsafe { __clone(this, new) }
            }
            unsafe fn __get(this: *const c_void) -> *const Self {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$shared_ptr$", $segment, "$get")]
                    fn __get(this: *const c_void) -> *const c_void;
                }
                unsafe { __get(this) }.cast()
            }
            unsafe fn __drop(this: *mut c_void) {
                extern "C" {
                    #[link_name = concat!("cxxbridge1$std$shared_ptr$", $segment, "$drop")]
                    fn __drop(this: *mut c_void);
                }
                unsafe { __drop(this) }
            }
        }
    };
}

macro_rules! impl_shared_ptr_target_for_primitive {
    ($ty:ident) => {
        impl_shared_ptr_target!(stringify!($ty), stringify!($ty), $ty);
    };
}

impl_shared_ptr_target_for_primitive!(bool);
impl_shared_ptr_target_for_primitive!(u8);
impl_shared_ptr_target_for_primitive!(u16);
impl_shared_ptr_target_for_primitive!(u32);
impl_shared_ptr_target_for_primitive!(u64);
impl_shared_ptr_target_for_primitive!(usize);
impl_shared_ptr_target_for_primitive!(i8);
impl_shared_ptr_target_for_primitive!(i16);
impl_shared_ptr_target_for_primitive!(i32);
impl_shared_ptr_target_for_primitive!(i64);
impl_shared_ptr_target_for_primitive!(isize);
impl_shared_ptr_target_for_primitive!(f32);
impl_shared_ptr_target_for_primitive!(f64);

impl_shared_ptr_target!("string", "CxxString", CxxString);
