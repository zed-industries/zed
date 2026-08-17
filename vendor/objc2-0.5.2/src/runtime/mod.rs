//! # Direct runtime bindings.
//!
//! This module contains safe(r) bindings to common parts of the Objective-C
//! runtime. See the [`ffi`][crate::ffi] module for details on the raw
//! bindings.
//!
//!
//! # Example
//!
//! Using features of the runtime to query information about `NSObject`.
//!
//! ```
#![doc = include_str!("../../examples/introspection.rs")]
//! ```
#![allow(clippy::missing_panics_doc)]

use alloc::vec::Vec;
use core::fmt;
use core::hash;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};
use core::str;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::os::raw::c_uint;

// Note: While this is not public, it is still a breaking change to remove,
// since `objc2-foundation` relies on it.
#[doc(hidden)]
pub mod __nsstring;
mod bool;
mod declare;
#[cfg(not(feature = "malloc"))]
mod malloc;
mod message_receiver;
mod method_encoding_iter;
mod method_implementation;
mod nsobject;
mod nsproxy;
mod nszone;
mod protocol_object;
mod retain_release_fast;

pub(crate) use self::method_encoding_iter::{EncodingParseError, MethodEncodingIter};
pub(crate) use self::retain_release_fast::{objc_release_fast, objc_retain_fast};
use crate::encode::{Encode, EncodeArguments, EncodeReturn, Encoding, OptionEncode, RefEncode};
use crate::verify::{verify_method_signature, Inner};
use crate::{ffi, Message};

// Note: While this is not public, it is still a breaking change to remove,
// since `objc2-foundation` relies on it.
#[doc(hidden)]
pub use self::nsproxy::NSProxy as __NSProxy;

pub use self::bool::Bool;
pub use self::declare::{ClassBuilder, ProtocolBuilder};
pub use self::message_receiver::MessageReceiver;
pub use self::method_implementation::MethodImplementation;
pub use self::nsobject::{NSObject, NSObjectProtocol};
pub use self::nszone::NSZone;
pub use self::protocol_object::{ImplementedBy, ProtocolObject};
pub use crate::verify::VerificationError;

#[cfg(not(feature = "malloc"))]
use self::malloc::{MallocSlice, MallocStr};
#[cfg(feature = "malloc")]
use malloc_buf::{Malloc as MallocSlice, Malloc as MallocStr};

/// We do not want to expose `MallocSlice` to end users, because in the
/// future, we want to be able to change it to `Box<[T], MallocAllocator>`.
///
/// So instead we use an unnameable type.
#[cfg(not(feature = "malloc"))]
macro_rules! MallocSlice {
    ($t:ty) => {
        impl std::ops::Deref<Target = [$t]> + AsRef<[$t]> + std::fmt::Debug
    };
}

#[cfg(feature = "malloc")]
macro_rules! MallocSlice {
    ($t:ty) => {
        malloc_buf::Malloc<[$t]>
    };
}

/// Same as `MallocSlice!`.
#[cfg(not(feature = "malloc"))]
macro_rules! MallocStr {
    () => {
        impl std::ops::Deref<Target = str> + AsRef<str> + std::fmt::Debug + std::fmt::Display
    };
}

#[cfg(feature = "malloc")]
macro_rules! MallocStr {
    () => {
        malloc_buf::Malloc<str>
    };
}

/// Implement PartialEq, Eq and Hash using pointer semantics; there's not
/// really a better way to do it for this type
macro_rules! standard_pointer_impls {
    ($name:ident) => {
        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.as_ptr() == other.as_ptr()
            }
        }
        impl Eq for $name {}
        impl hash::Hash for $name {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.as_ptr().hash(state)
            }
        }
    };
}

/// Use [`Bool`] or [`ffi::BOOL`] instead.
#[deprecated = "Use `Bool` or `ffi::BOOL` instead"]
#[allow(non_upper_case_globals)]
pub type BOOL = ffi::BOOL;

/// Use [`Bool::YES`] or [`ffi::YES`] instead.
#[deprecated = "Use `Bool::YES` or `ffi::YES` instead"]
pub const YES: ffi::BOOL = ffi::YES;

/// Use [`Bool::NO`] or [`ffi::NO`] instead.
#[deprecated = "Use `Bool::NO` or `ffi::NO` instead"]
pub const NO: ffi::BOOL = ffi::NO;

#[cfg(not(feature = "unstable-c-unwind"))]
type InnerImp = unsafe extern "C" fn();
#[cfg(feature = "unstable-c-unwind")]
type InnerImp = unsafe extern "C-unwind" fn();

/// A pointer to the start of a method implementation.
///
/// The first argument is a pointer to the receiver, the second argument is
/// the selector, and the rest of the arguments follow.
///
///
/// # Safety
///
/// This is a "catch all" type; it must be transmuted to the correct type
/// before being called!
///
/// Also note that this is non-null! If you require an Imp that can be null,
/// use `Option<Imp>`.
pub type Imp = InnerImp;

/// A method selector.
///
/// The Rust equivalent of Objective-C's `SEL` type. You can create this
/// statically using the [`sel!`] macro.
///
/// The main reason the Objective-C runtime uses a custom type for selectors,
/// as opposed to a plain c-string, is to support efficient comparison - a
/// a selector is effectively an [interned string], so this makes equiality
/// comparisons very cheap.
///
/// This struct guarantees the null-pointer optimization, namely that
/// `Option<Sel>` is the same size as `Sel`.
///
/// [`sel!`]: crate::sel
/// [interned string]: https://en.wikipedia.org/wiki/String_interning
#[repr(transparent)]
#[derive(Copy, Clone)]
#[doc(alias = "SEL")]
#[doc(alias = "objc_selector")]
pub struct Sel {
    ptr: NonNull<ffi::objc_selector>,
}

// SAFETY: Sel is immutable (and can be retrieved from any thread using the
// `sel!` macro).
unsafe impl Sync for Sel {}
unsafe impl Send for Sel {}
impl UnwindSafe for Sel {}
impl RefUnwindSafe for Sel {}

impl Sel {
    #[inline]
    #[doc(hidden)]
    pub const unsafe fn __internal_from_ptr(ptr: *const ffi::objc_selector) -> Self {
        // Used in static selectors.
        // SAFETY: Upheld by caller.
        let ptr = unsafe { NonNull::new_unchecked(ptr as *mut ffi::objc_selector) };
        Self { ptr }
    }

    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *const ffi::objc_selector) -> Option<Self> {
        // SAFETY: Caller verifies that the pointer is valid.
        NonNull::new(ptr as *mut ffi::objc_selector).map(|ptr| Self { ptr })
    }

    /// Get a pointer to the raw selector.
    ///
    /// Useful when working with raw FFI methods.
    #[inline]
    pub const fn as_ptr(&self) -> *const ffi::objc_selector {
        self.ptr.as_ptr()
    }

    // We explicitly don't do #[track_caller] here, since we expect the error
    // to never actually happen.
    pub(crate) unsafe fn register_unchecked(name: *const c_char) -> Self {
        let ptr = unsafe { ffi::sel_registerName(name) };
        // SAFETY: `sel_registerName` declares return type as `SEL _Nonnull`,
        // at least when input is also `_Nonnull` (which it is in our case).
        //
        // Looking at the source code, it can fail and will return NULL if
        // allocating space for the selector failed (which then subsequently
        // invokes UB by calling `memcpy` with a NULL argument):
        // <https://github.com/apple-oss-distributions/objc4/blob/objc4-841.13/runtime/objc-os.h#L1002-L1004>
        //
        // I suspect this will be really uncommon in practice, since the
        // called selector is almost always going to be present in the binary
        // already; but alas, we'll handle it!
        unsafe { Self::from_ptr(ptr).expect("failed allocating selector") }
    }

    /// Registers a selector with the Objective-C runtime.
    ///
    /// This is the dynamic version of the [`sel!`] macro, prefer to use that
    /// when your selector is static.
    ///
    /// [`sel!`]: crate::sel
    ///
    ///
    /// # Panics
    ///
    /// Panics if `name` contains an internal NUL byte, or if the runtime
    /// failed allocating space for the selector.
    #[doc(alias = "sel_registerName")]
    pub fn register(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        // SAFETY: Input is a non-null, NUL-terminated C-string pointer.
        unsafe { Self::register_unchecked(name.as_ptr()) }
    }

    /// Returns the string representation of the selector.
    ///
    /// # Panics
    ///
    /// Panics if the selector is not valid UTF-8 (however unlikely!)
    #[doc(alias = "sel_getName")]
    pub fn name(&self) -> &str {
        // SAFETY: Input is non-null selector. Declares return type as
        // `const char * _Nonnull`, source code agrees.
        let ptr = unsafe { ffi::sel_getName(self.as_ptr()) };
        // SAFETY: The string is a valid C-style NUL-terminated string, and
        // likely has static lifetime since the selector has static lifetime
        // (though we bind it to `&self` to be safe).
        let name = unsafe { CStr::from_ptr(ptr) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    pub(crate) fn number_of_arguments(self) -> usize {
        self.name()
            .as_bytes()
            .iter()
            .filter(|&&b| b == b':')
            .count()
    }
}

impl PartialEq for Sel {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if cfg!(feature = "gnustep-1-7") {
            // GNUStep implements "typed" selectors, which means their pointer
            // values sometimes differ; so let's use the runtime-provided
            // `sel_isEqual`.
            unsafe { Bool::from_raw(ffi::sel_isEqual(self.as_ptr(), other.as_ptr())).as_bool() }
        } else {
            // `ffi::sel_isEqual` uses pointer comparison on Apple (the
            // documentation explicitly notes this); so as an optimization,
            // let's do that as well!
            self.as_ptr() == other.as_ptr()
        }
    }
}

impl Eq for Sel {}

impl hash::Hash for Sel {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        if cfg!(feature = "gnustep-1-7") {
            // Note: We hash the name instead of the pointer on GNUStep, since
            // they're typed.
            self.name().hash(state);
        } else {
            self.as_ptr().hash(state);
        }
    }
}

// SAFETY: `Sel` is FFI compatible, and the encoding is `Sel`.
unsafe impl Encode for Sel {
    const ENCODING: Encoding = Encoding::Sel;
}

unsafe impl OptionEncode for Sel {}

// RefEncode is not implemented for Sel, because there is literally no API
// that takes &Sel, while the user could get confused and accidentally attempt
// that.

impl fmt::Display for Sel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.name(), f)
    }
}

impl fmt::Debug for Sel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Sel").field(&self.name()).finish()
    }
}

impl fmt::Pointer for Sel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

/// A type that represents an instance variable.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/ivar?language=objc).
#[repr(C)]
#[doc(alias = "objc_ivar")]
pub struct Ivar(ffi::objc_ivar);

// SAFETY: Ivar is immutable (and can be retrieved from AnyClass anyhow).
unsafe impl Sync for Ivar {}
unsafe impl Send for Ivar {}
impl UnwindSafe for Ivar {}
impl RefUnwindSafe for Ivar {}

impl Ivar {
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_ivar {
        let ptr: *const Self = self;
        ptr.cast()
    }

    /// Returns the instance variable's name.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418922-ivar_getname?language=objc).
    #[doc(alias = "ivar_getName")]
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(ffi::ivar_getName(self.as_ptr())) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Returns the instance variable's offset from the object base.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418976-ivar_getoffset?language=objc).
    #[inline]
    #[doc(alias = "ivar_getOffset")]
    pub fn offset(&self) -> isize {
        unsafe { ffi::ivar_getOffset(self.as_ptr()) }
    }

    /// Returns the instance variable's `@encode(type)` string.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418569-ivar_gettypeencoding?language=objc).
    #[doc(alias = "ivar_getTypeEncoding")]
    pub fn type_encoding(&self) -> &str {
        let encoding = unsafe { CStr::from_ptr(ffi::ivar_getTypeEncoding(self.as_ptr())) };
        str::from_utf8(encoding.to_bytes()).unwrap()
    }

    #[inline]
    pub(crate) fn debug_assert_encoding(&self, _expected: &Encoding) {
        #[cfg(debug_assertions)]
        {
            let encoding = self.type_encoding();
            assert!(
                _expected.equivalent_to_str(encoding),
                "wrong encoding. Tried to retrieve ivar with encoding {encoding}, but the encoding of the given type was {_expected}",
            );
        }
    }

    /// Returns a pointer to the instance variable / ivar on the given object.
    ///
    /// This is similar to [`UnsafeCell::get`], see that for more information
    /// on what is and isn't safe to do.
    ///
    /// Usually you will have defined the instance variable yourself with
    /// [`ClassBuilder::add_ivar`], the type of the ivar `T` must match the
    /// type used in that.
    ///
    /// Library implementors are strongly encouraged to expose a safe
    /// interface to the ivar.
    ///
    /// [`UnsafeCell::get`]: core::cell::UnsafeCell::get
    /// [`ClassBuilder::add_ivar`]: crate::declare::ClassBuilder::add_ivar
    ///
    ///
    /// # Panics
    ///
    /// Panics when `debug_assertions` are enabled if the type encoding of the
    /// ivar differs from the type encoding of `T`.
    ///
    ///
    /// # Safety
    ///
    /// The object must have the given instance variable on it, and it must be
    /// of type `T`. Any invariants that the object have assumed about the
    /// value of the instance variable must not be violated.
    ///
    /// Note that an object can have multiple instance variables with the same
    /// name; you must ensure that when the instance variable was retrieved,
    /// was retrieved from the class that it was defined on. In particular,
    /// getting a class dynamically using e.g. [`AnyObject::class`], and using
    /// an instance variable from that here is _not_ sound in general.
    ///
    /// No thread syncronization is done on accesses to the variable, so you
    /// must ensure that any access to the returned pointer do not cause data
    /// races, and that Rust's mutability rules are not otherwise violated.
    #[inline]
    pub unsafe fn load_ptr<T: Encode>(&self, obj: &AnyObject) -> *mut T {
        self.debug_assert_encoding(&T::ENCODING);

        let ptr = NonNull::from(obj);
        // SAFETY: That the ivar is valid is ensured by the caller
        let ptr = unsafe { AnyObject::ivar_at_offset::<T>(ptr, self.offset()) };

        // Safe as *mut T because `self` is `UnsafeCell`
        ptr.as_ptr()
    }

    /// Returns a reference to the instance variable with the given name.
    ///
    /// See [`Ivar::load_ptr`] for more information.
    ///
    ///
    /// # Panics
    ///
    /// Panics when `debug_assertions` are enabled if the type encoding of the
    /// ivar differs from the type encoding of `T`.
    ///
    ///
    /// # Safety
    ///
    /// The object must have the given instance variable on it, and it must be
    /// of type `T`.
    ///
    /// No thread syncronization is done, so you must ensure that no other
    /// thread is concurrently mutating the variable. This requirement can be
    /// considered upheld if all mutation happens through [`Ivar::load_mut`]
    /// (since that takes the object mutably).
    #[inline]
    pub unsafe fn load<'obj, T: Encode>(&self, obj: &'obj AnyObject) -> &'obj T {
        // SAFETY: That the ivar is valid as `&T` is ensured by the caller,
        // and the reference is properly bound to the object.
        unsafe { self.load_ptr::<T>(obj).as_ref().unwrap_unchecked() }
    }

    /// Returns a mutable reference to the ivar with the given name.
    ///
    /// See [`Ivar::load_ptr`] for more information.
    ///
    ///
    /// # Panics
    ///
    /// Panics when `debug_assertions` are enabled if the type encoding of the
    /// ivar differs from the type encoding of `T`.
    ///
    ///
    /// # Safety
    ///
    /// The object must have an instance variable with the given name, and it
    /// must be of type `T`.
    ///
    /// This access happens through `&mut`, which means we know it to be the
    /// only reference, hence you do not need to do any work to ensure that
    /// data races do not happen.
    #[inline]
    pub unsafe fn load_mut<'obj, T: Encode>(&self, obj: &'obj mut AnyObject) -> &'obj mut T {
        self.debug_assert_encoding(&T::ENCODING);

        let ptr = NonNull::from(obj);
        // SAFETY: That the ivar is valid is ensured by the caller
        let mut ptr = unsafe { AnyObject::ivar_at_offset::<T>(ptr, self.offset()) };

        // SAFETY: That the ivar is valid as `&mut T` is ensured by taking an
        // `&mut` object
        unsafe { ptr.as_mut() }
    }
}

standard_pointer_impls!(Ivar);

impl fmt::Debug for Ivar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ivar")
            .field("name", &self.name())
            .field("offset", &self.offset())
            .field("type_encoding", &self.type_encoding())
            .finish_non_exhaustive()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct MethodDescription {
    pub(crate) sel: Sel,
    pub(crate) types: &'static str,
}

impl MethodDescription {
    pub(crate) unsafe fn from_raw(raw: ffi::objc_method_description) -> Option<Self> {
        // SAFETY: Sel::from_ptr checks for NULL, rest is checked by caller.
        let sel = unsafe { Sel::from_ptr(raw.name) }?;
        if raw.types.is_null() {
            return None;
        }
        // SAFETY: We've checked that the pointer is not NULL, rest is checked
        // by caller.
        let types = unsafe { CStr::from_ptr(raw.types) }.to_str().unwrap();
        Some(Self { sel, types })
    }
}

/// A type that represents a method in a class definition.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/method?language=objc).
#[repr(C)]
#[doc(alias = "objc_method")]
pub struct Method(ffi::objc_method);

// SAFETY: Method is immutable (and can be retrieved from AnyClass anyhow).
unsafe impl Sync for Method {}
unsafe impl Send for Method {}
impl UnwindSafe for Method {}
impl RefUnwindSafe for Method {}

impl Method {
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_method {
        let ptr: *const Self = self;
        ptr.cast()
    }

    // Note: We don't take `&mut` here, since the operations on methods work
    // atomically.
    #[inline]
    pub(crate) fn as_mut_ptr(&self) -> *mut ffi::objc_method {
        self.as_ptr() as _
    }

    /// Returns the name of self.
    #[inline]
    #[doc(alias = "method_getName")]
    pub fn name(&self) -> Sel {
        unsafe { Sel::from_ptr(ffi::method_getName(self.as_ptr())).unwrap() }
    }

    /// Returns the `Encoding` of self's return type.
    #[doc(alias = "method_copyReturnType")]
    pub fn return_type(&self) -> MallocStr!() {
        unsafe {
            let encoding = ffi::method_copyReturnType(self.as_ptr());
            MallocStr::from_c_str(encoding).unwrap()
        }
    }

    /// Returns the `Encoding` of a single parameter type of self, or
    /// [`None`] if self has no parameter at the given index.
    #[doc(alias = "method_copyArgumentType")]
    pub fn argument_type(&self, index: usize) -> Option<MallocStr!()> {
        unsafe {
            let encoding = ffi::method_copyArgumentType(self.as_ptr(), index as c_uint);
            NonNull::new(encoding).map(|encoding| MallocStr::from_c_str(encoding.as_ptr()).unwrap())
        }
    }

    /// An iterator over the method's types.
    ///
    /// It is approximately equivalent to:
    ///
    /// ```ignore
    /// let types = method.types();
    /// assert_eq!(types.next()?, method.return_type());
    /// for i in 0..method.arguments_count() {
    ///    assert_eq!(types.next()?, method.argument_type(i)?);
    /// }
    /// assert!(types.next().is_none());
    /// ```
    #[doc(alias = "method_getTypeEncoding")]
    pub(crate) fn types(&self) -> MethodEncodingIter<'_> {
        // SAFETY: The method pointer is valid and non-null
        let cstr = unsafe { ffi::method_getTypeEncoding(self.as_ptr()) };
        if cstr.is_null() {
            panic!("method type encoding was NULL");
        }
        // SAFETY: `method_getTypeEncoding` returns a C-string, and we just
        // checked that it is non-null.
        let encoding = unsafe { CStr::from_ptr(cstr) };
        let s = str::from_utf8(encoding.to_bytes()).expect("method type encoding to be UTF-8");
        MethodEncodingIter::new(s)
    }

    /// Returns the number of arguments accepted by self.
    #[inline]
    #[doc(alias = "method_getNumberOfArguments")]
    pub fn arguments_count(&self) -> usize {
        unsafe { ffi::method_getNumberOfArguments(self.as_ptr()) as usize }
    }

    /// Returns the implementation of this method.
    #[doc(alias = "method_getImplementation")]
    pub fn implementation(&self) -> Imp {
        unsafe { ffi::method_getImplementation(self.as_ptr()).expect("null IMP") }
    }

    /// Set the implementation of this method.
    ///
    /// Note that any thread may at any point be changing method
    /// implementations, so if you intend to call the previous method as
    /// returned by e.g. [`Self::implementation`], beware that that may now be
    /// stale.
    ///
    /// The previous implementation is returned from this function though, so
    /// you can call that instead.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418707-method_setimplementation?language=objc).
    ///
    ///
    /// # Safety
    ///
    /// The given implementation function pointer must:
    ///
    /// 1. Have the signature expected by the Objective-C runtime and callers
    ///    of this method.
    ///
    /// 2. Be at least as safe as the existing method, i.e. by overriding the
    ///    previous method, it should not be possible for the program to cause
    ///    UB.
    ///
    ///    A common mistake would be expecting e.g. a pointer to not be null,
    ///    where the null case was handled before.
    #[doc(alias = "method_setImplementation")]
    pub unsafe fn set_implementation(&self, imp: Imp) -> Imp {
        // SAFETY: The new impl is not NULL, and the rest is upheld by the
        // caller.
        unsafe { ffi::method_setImplementation(self.as_mut_ptr(), Some(imp)).expect("null IMP") }
    }

    /// Exchange the implementation of two methods.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/1418769-method_exchangeimplementations?language=objc).
    ///
    ///
    /// # Safety
    ///
    /// The two methods must be perfectly compatible, both in signature, and
    /// in expected (in terms of safety, not necessarily behaviour) input and
    /// output.
    ///
    ///
    /// # Example
    ///
    /// This is an atomic version of the following:
    ///
    /// ```
    /// use objc2::runtime::Method;
    /// # use objc2::runtime::NSObject;
    /// # use objc2::sel;
    /// # use crate::objc2::ClassType;
    ///
    /// let m1: &Method;
    /// let m2: &Method;
    /// #
    /// # // Use the same method twice, to avoid actually changing anything
    /// # m1 = NSObject::class().instance_method(sel!(hash)).unwrap();
    /// # m2 = NSObject::class().instance_method(sel!(hash)).unwrap();
    ///
    /// unsafe {
    ///     let imp = m2.set_implementation(m1.implementation());
    ///     m1.set_implementation(imp);
    /// }
    /// ```
    #[inline]
    #[doc(alias = "method_exchangeImplementations")]
    pub unsafe fn exchange_implementation(&self, other: &Self) {
        // TODO: Consider checking that `self.types()` and `other.types()`
        // match when debug assertions are enabled?

        // SAFETY: Verified by caller
        unsafe { ffi::method_exchangeImplementations(self.as_mut_ptr(), other.as_mut_ptr()) }
    }
}

standard_pointer_impls!(Method);

impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.name())
            .field("types", &self.types())
            .field("implementation", &self.implementation())
            .finish_non_exhaustive()
    }
}

/// A type that represents an Objective-C class.
///
/// This is an opaque type meant to be used behind a shared reference
/// `&AnyClass`, which is semantically equivalent to `Class _Nonnull`.
///
/// A nullable class can be used as `Option<&AnyClass>`.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/objectivec/class?language=objc).
#[repr(C)]
#[doc(alias = "Class")]
#[doc(alias = "objc_class")]
pub struct AnyClass(ffi::objc_class);

/// Use [`AnyClass`] instead.
#[deprecated = "renamed to `runtime::AnyClass`"]
pub type Class = AnyClass;

// SAFETY: AnyClass is immutable (and can be retrieved from any thread using
// the `class!` macro).
unsafe impl Sync for AnyClass {}
unsafe impl Send for AnyClass {}
impl UnwindSafe for AnyClass {}
impl RefUnwindSafe for AnyClass {}
// Note that Unpin is not applicable.

impl AnyClass {
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_class {
        let ptr: *const Self = self;
        ptr.cast()
    }

    /// Returns the class definition of a specified class, or [`None`] if the
    /// class is not registered with the Objective-C runtime.
    #[doc(alias = "objc_getClass")]
    pub fn get(name: &str) -> Option<&'static Self> {
        let name = CString::new(name).unwrap();
        let cls = unsafe { ffi::objc_getClass(name.as_ptr()) };
        unsafe { cls.cast::<Self>().as_ref() }
    }

    // Same as `get`, but ...
    // fn lookup(name: &str) -> Option<&'static Self>;

    /// Obtains the list of registered class definitions.
    #[doc(alias = "objc_copyClassList")]
    pub fn classes() -> MallocSlice!(&'static Self) {
        unsafe {
            let mut count: c_uint = 0;
            let classes: *mut &Self = ffi::objc_copyClassList(&mut count).cast();
            MallocSlice::from_array(classes, count as usize)
        }
    }

    /// Returns the total number of registered classes.
    #[inline]
    #[doc(alias = "objc_getClassList")]
    pub fn classes_count() -> usize {
        unsafe { ffi::objc_getClassList(ptr::null_mut(), 0) as usize }
    }

    /// # Safety
    ///
    /// 1. The class pointer must be valid.
    /// 2. The string is unbounded, so the caller must bound it.
    pub(crate) unsafe fn name_raw<'a>(ptr: *const ffi::objc_class) -> &'a str {
        // SAFETY: Caller ensures that the pointer is valid
        let name = unsafe { ffi::class_getName(ptr) };
        if name.is_null() {
            panic!("class name was NULL");
        }
        // SAFETY: We've checked that the pointer is not NULL, and
        // `class_getName` is guaranteed to return a valid C-string.
        //
        // That the result is properly bounded is checked by the caller.
        let name = unsafe { CStr::from_ptr(name) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Returns the name of the class.
    #[doc(alias = "class_getName")]
    pub fn name(&self) -> &str {
        // SAFETY: The pointer is valid, and the return is properly bounded
        unsafe { Self::name_raw(self.as_ptr()) }
    }

    /// # Safety
    ///
    /// 1. The class pointer must be valid.
    /// 2. The caller must bound the lifetime of the returned class.
    #[inline]
    pub(crate) unsafe fn superclass_raw<'a>(ptr: *const ffi::objc_class) -> Option<&'a AnyClass> {
        // SAFETY: Caller ensures that the pointer is valid
        let superclass = unsafe { ffi::class_getSuperclass(ptr) };
        let superclass: *const AnyClass = superclass.cast();
        // SAFETY: The result is properly bounded by the caller.
        unsafe { superclass.as_ref() }
    }

    /// Returns the superclass of self, or [`None`] if self is a root class.
    #[inline]
    #[doc(alias = "class_getSuperclass")]
    pub fn superclass(&self) -> Option<&AnyClass> {
        // SAFETY: The pointer is valid, and the return is properly bounded
        unsafe { Self::superclass_raw(self.as_ptr()) }
    }

    /// Returns the metaclass of self.
    #[inline]
    #[doc(alias = "object_getClass")]
    pub fn metaclass(&self) -> &Self {
        let ptr: *const Self = unsafe { ffi::object_getClass(self.as_ptr().cast()) }.cast();
        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    // objc_getMetaClass -> Same as `AnyClass::get(name).metaclass()`

    #[allow(unused)]
    pub(crate) fn is_metaclass(&self) -> bool {
        unsafe { Bool::from_raw(ffi::class_isMetaClass(self.as_ptr())).as_bool() }
    }

    /// Returns the size of instances of self.
    #[inline]
    #[doc(alias = "class_getInstanceSize")]
    pub fn instance_size(&self) -> usize {
        unsafe { ffi::class_getInstanceSize(self.as_ptr()) }
    }

    /// Returns a specified instance method for self, or [`None`] if self and
    /// its superclasses do not contain an instance method with the specified
    /// selector.
    #[inline]
    #[doc(alias = "class_getInstanceMethod")]
    pub fn instance_method(&self, sel: Sel) -> Option<&Method> {
        unsafe {
            let method = ffi::class_getInstanceMethod(self.as_ptr(), sel.as_ptr());
            method.cast::<Method>().as_ref()
        }
    }

    /// Returns a specified class method for self, or [`None`] if self and
    /// its superclasses do not contain a class method with the specified
    /// selector.
    ///
    /// Same as `cls.metaclass().class_method()`.
    #[inline]
    #[doc(alias = "class_getClassMethod")]
    pub fn class_method(&self, sel: Sel) -> Option<&Method> {
        unsafe {
            let method = ffi::class_getClassMethod(self.as_ptr(), sel.as_ptr());
            method.cast::<Method>().as_ref()
        }
    }

    /// Returns the ivar for a specified instance variable of self, or
    /// [`None`] if self has no ivar with the given name.
    ///
    /// If the instance variable was not found on the specified class, the
    /// superclasses are searched.
    ///
    /// Attempting to access or modify instance variables of a class that you
    /// do no control may invoke undefined behaviour.
    #[doc(alias = "class_getInstanceVariable")]
    pub fn instance_variable(&self, name: &str) -> Option<&Ivar> {
        let name = CString::new(name).unwrap();
        unsafe {
            let ivar = ffi::class_getInstanceVariable(self.as_ptr(), name.as_ptr());
            ivar.cast::<Ivar>().as_ref()
        }
    }

    #[allow(unused)]
    #[doc(alias = "class_getClassVariable")]
    fn class_variable(&self, name: &str) -> Option<&Ivar> {
        let name = CString::new(name).unwrap();
        let ivar = unsafe { ffi::class_getClassVariable(self.as_ptr(), name.as_ptr()) };
        // SAFETY: TODO
        unsafe { ivar.cast::<Ivar>().as_ref() }
    }

    /// Describes the instance methods implemented by self.
    #[doc(alias = "class_copyMethodList")]
    pub fn instance_methods(&self) -> MallocSlice!(&Method) {
        unsafe {
            let mut count: c_uint = 0;
            let methods: *mut &Method = ffi::class_copyMethodList(self.as_ptr(), &mut count).cast();
            MallocSlice::from_array(methods, count as usize)
        }
    }

    /// Checks whether this class conforms to the specified protocol.
    #[inline]
    #[doc(alias = "class_conformsToProtocol")]
    pub fn conforms_to(&self, proto: &AnyProtocol) -> bool {
        unsafe {
            Bool::from_raw(ffi::class_conformsToProtocol(self.as_ptr(), proto.as_ptr())).as_bool()
        }
    }

    /// Get a list of the protocols to which this class conforms.
    #[doc(alias = "class_copyProtocolList")]
    pub fn adopted_protocols(&self) -> MallocSlice!(&AnyProtocol) {
        unsafe {
            let mut count: c_uint = 0;
            let protos: *mut &AnyProtocol =
                ffi::class_copyProtocolList(self.as_ptr(), &mut count).cast();
            MallocSlice::from_array(protos, count as usize)
        }
    }

    /// Describes the instance variables declared by self.
    #[doc(alias = "class_copyIvarList")]
    pub fn instance_variables(&self) -> MallocSlice!(&Ivar) {
        unsafe {
            let mut count: c_uint = 0;
            let ivars: *mut &Ivar = ffi::class_copyIvarList(self.as_ptr(), &mut count).cast();
            MallocSlice::from_array(ivars, count as usize)
        }
    }

    /// Check whether instances of this class respond to the given selector.
    ///
    /// This doesn't call `respondsToSelector:`, but works entirely within the
    /// runtime, which means it'll always be safe to call, but may not return
    /// exactly what you'd expect if `respondsToSelector:` has been
    /// overwritten.
    ///
    /// That said, it will always return `true` if an instance of the class
    /// responds to the selector, but may return `false` if they don't
    /// directly (e.g. does so by using forwarding instead).
    #[inline]
    #[doc(alias = "class_respondsToSelector")]
    pub fn responds_to(&self, sel: Sel) -> bool {
        // This may call `resolveInstanceMethod:` and `resolveClassMethod:`
        // SAFETY: The selector is guaranteed non-null.
        let res = unsafe { ffi::class_respondsToSelector(self.as_ptr(), sel.as_ptr()) };
        Bool::from_raw(res).as_bool()
    }

    // <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtPropertyIntrospection.html>
    // fn property(&self, name: &str) -> Option<&Property>;
    // fn properties(&self) -> MallocSlice!(&Property);
    // unsafe fn replace_method(&self, name: Sel, imp: Imp, types: &str) -> Imp;
    // unsafe fn replace_property(&self, name: &str, attributes: &[ffi::objc_property_attribute_t]);
    // fn method_imp(&self, name: Sel) -> Imp; // + _stret

    // fn get_version(&self) -> u32;
    // unsafe fn set_version(&mut self, version: u32);

    /// Verify argument and return types for a given selector.
    ///
    /// This will look up the encoding of the method for the given selector
    /// and return a [`VerificationError`] if any encodings differ for the
    /// arguments `A` and return type `R`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use objc2::{class, sel};
    /// use objc2::runtime::{AnyClass, Bool};
    /// let cls = class!(NSObject);
    /// let sel = sel!(isKindOfClass:);
    /// // Verify that `isKindOfClass:`:
    /// // - Exists on the class
    /// // - Takes a class as a parameter
    /// // - Returns a BOOL
    /// let result = cls.verify_sel::<(&AnyClass,), Bool>(sel);
    /// assert!(result.is_ok());
    /// ```
    #[allow(clippy::missing_errors_doc)] // Written differently in the docs
    pub fn verify_sel<A, R>(&self, sel: Sel) -> Result<(), VerificationError>
    where
        A: EncodeArguments,
        R: EncodeReturn,
    {
        let method = self.instance_method(sel).ok_or(Inner::MethodNotFound)?;
        verify_method_signature(method, A::ENCODINGS, &R::ENCODING_RETURN)
    }
}

standard_pointer_impls!(AnyClass);

unsafe impl RefEncode for AnyClass {
    const ENCODING_REF: Encoding = Encoding::Class;
}

impl fmt::Debug for AnyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyClass")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl fmt::Display for AnyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.name(), f)
    }
}

/// A type that represents an Objective-C protocol.
#[repr(C)]
#[doc(alias = "objc_protocol")]
pub struct AnyProtocol(ffi::objc_protocol);

/// Use [`AnyProtocol`] instead.
#[deprecated = "renamed to `runtime::AnyProtocol`"]
pub type Protocol = AnyProtocol;

// SAFETY: AnyProtocol is immutable (and can be retrieved from AnyClass anyhow).
unsafe impl Sync for AnyProtocol {}
unsafe impl Send for AnyProtocol {}
impl UnwindSafe for AnyProtocol {}
impl RefUnwindSafe for AnyProtocol {}
// Note that Unpin is not applicable.

impl AnyProtocol {
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_protocol {
        let ptr: *const Self = self;
        ptr.cast()
    }

    /// Returns the protocol definition of a specified protocol, or [`None`]
    /// if the protocol is not registered with the Objective-C runtime.
    #[doc(alias = "objc_getProtocol")]
    pub fn get(name: &str) -> Option<&'static Self> {
        let name = CString::new(name).unwrap();
        unsafe {
            let proto = ffi::objc_getProtocol(name.as_ptr());
            proto.cast::<Self>().as_ref()
        }
    }

    /// Obtains the list of registered protocol definitions.
    #[doc(alias = "objc_copyProtocolList")]
    pub fn protocols() -> MallocSlice!(&'static Self) {
        unsafe {
            let mut count: c_uint = 0;
            let protocols: *mut &Self = ffi::objc_copyProtocolList(&mut count).cast();
            MallocSlice::from_array(protocols, count as usize)
        }
    }

    /// Get a list of the protocols to which this protocol conforms.
    #[doc(alias = "protocol_copyProtocolList")]
    pub fn adopted_protocols(&self) -> MallocSlice!(&AnyProtocol) {
        unsafe {
            let mut count: c_uint = 0;
            let protocols: *mut &AnyProtocol =
                ffi::protocol_copyProtocolList(self.as_ptr(), &mut count).cast();
            MallocSlice::from_array(protocols, count as usize)
        }
    }

    /// Checks whether this protocol conforms to the specified protocol.
    #[inline]
    #[doc(alias = "protocol_conformsToProtocol")]
    pub fn conforms_to(&self, proto: &AnyProtocol) -> bool {
        unsafe {
            Bool::from_raw(ffi::protocol_conformsToProtocol(
                self.as_ptr(),
                proto.as_ptr(),
            ))
            .as_bool()
        }
    }

    /// Returns the name of self.
    #[doc(alias = "protocol_getName")]
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(ffi::protocol_getName(self.as_ptr())) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    fn method_descriptions_inner(&self, required: bool, instance: bool) -> Vec<MethodDescription> {
        let mut count: c_uint = 0;
        let descriptions = unsafe {
            ffi::protocol_copyMethodDescriptionList(
                self.as_ptr(),
                Bool::new(required).as_raw(),
                Bool::new(instance).as_raw(),
                &mut count,
            )
        };
        if descriptions.is_null() {
            return Vec::new();
        }
        let descriptions = unsafe { MallocSlice::from_array(descriptions, count as usize) };
        descriptions
            .iter()
            .map(|desc| {
                unsafe { MethodDescription::from_raw(*desc) }.expect("invalid method description")
            })
            .collect()
    }

    #[allow(dead_code)]
    #[doc(alias = "protocol_copyMethodDescriptionList")]
    pub(crate) fn method_descriptions(&self, required: bool) -> Vec<MethodDescription> {
        self.method_descriptions_inner(required, true)
    }

    #[allow(dead_code)]
    #[doc(alias = "protocol_copyMethodDescriptionList")]
    pub(crate) fn class_method_descriptions(&self, required: bool) -> Vec<MethodDescription> {
        self.method_descriptions_inner(required, false)
    }
}

impl PartialEq for AnyProtocol {
    /// Check whether the protocols are equal, or conform to each other.
    #[inline]
    #[doc(alias = "protocol_isEqual")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { Bool::from_raw(ffi::protocol_isEqual(self.as_ptr(), other.as_ptr())).as_bool() }
    }
}

impl Eq for AnyProtocol {}

// Don't implement `Hash` for protocol, it is unclear how that would work

unsafe impl RefEncode for AnyProtocol {
    // Protocols are objects internally.
    const ENCODING_REF: Encoding = Encoding::Object;
}

impl fmt::Debug for AnyProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyProtocol")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl fmt::Display for AnyProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.name(), f)
    }
}

/// An Objective-C object.
///
/// This is slightly different from [`NSObject`] in that it may represent an
/// instance of an _arbitary_ Objective-C class (e.g. it does not have to be
/// a subclass of `NSObject`, so it can represent other root classes like
/// `NSProxy`).
///
/// `Retained<AnyObject>` is equivalent to Objective-C's `id _Nonnull`.
///
/// This contains [`UnsafeCell`], and is similar to that in that one can
/// safely access and perform interior mutability on this (both via.
/// [`msg_send!`] and through ivars), so long as Rust's mutability rules are
/// upheld, and that data races are avoided.
///
/// Note: This is intentionally neither [`Sync`], [`Send`], [`UnwindSafe`],
/// [`RefUnwindSafe`] nor [`Unpin`], since that is something that may change
/// depending on the specific subclass. For example, `NSAutoreleasePool` is
/// not `Send`, it has to be deallocated on the same thread that it was
/// created. `NSLock` is not `Send` either.
///
/// This is somewhat similar to [`ffi::objc_object`].
///
/// [`UnsafeCell`]: core::cell::UnsafeCell
/// [`msg_send!`]: crate::msg_send
#[doc(alias = "id")]
#[repr(C)]
pub struct AnyObject(ffi::objc_object);

/// Use [`AnyObject`] instead.
#[deprecated = "renamed to `runtime::AnyObject`. Consider using the correct type from the autogenerated `objc2-*` framework crates instead though"]
pub type Object = AnyObject;

unsafe impl RefEncode for AnyObject {
    const ENCODING_REF: Encoding = Encoding::Object;
}

// SAFETY: This is technically slightly wrong, not all objects implement the
// standard memory management methods. But not having this impl would be too
// restrictive, so we'll live with it.
unsafe impl Message for AnyObject {}

impl AnyObject {
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::objc_object {
        let ptr: *const Self = self;
        ptr.cast()
    }

    /// Dynamically find the class of this object.
    ///
    ///
    /// # Example
    ///
    /// Check that an instance of `NSObject` has the precise class `NSObject`.
    ///
    /// ```
    /// use objc2::ClassType;
    /// use objc2::runtime::NSObject;
    ///
    /// let obj = NSObject::new();
    /// assert_eq!(obj.class(), NSObject::class());
    /// ```
    #[inline]
    #[doc(alias = "object_getClass")]
    pub fn class(&self) -> &'static AnyClass {
        let ptr: *const AnyClass = unsafe { ffi::object_getClass(self.as_ptr()) }.cast();
        // SAFETY: The class is not NULL because the object is not NULL, and
        // it is safe as `'static` since classes are static, and it could be
        // retrieved via. `AnyClass::get(self.class().name())` anyhow.
        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    /// Change the class of the object at runtime.
    ///
    /// Returns the object's previous class.
    ///
    ///
    /// # Safety
    ///
    /// The new class must:
    ///
    /// 1. Be a subclass of the object's current class.
    ///
    /// 2. The subclass must not add any instance variables - importantly, the
    ///    instance size of old and the new classes must be the same.
    ///
    /// 3. Any overridden methods on the new class must be fully compatible
    ///    with the old ones.
    ///
    /// Note that in the general case, where arbitary parts of the program
    /// may be trying to modify the class of the object concurrently, these
    /// requirements are not actually possible to uphold.
    ///
    /// Since usage of this function is expected to be extremely rare, and
    /// even more so trying to do it concurrently, it is recommended that you
    /// verify that the returned class is what you would expect, and if not,
    /// panic.
    #[inline]
    #[doc(alias = "object_setClass")]
    pub unsafe fn set_class<'s>(this: &Self, cls: &AnyClass) -> &'s AnyClass {
        let ptr =
            unsafe { ffi::object_setClass(this.as_ptr() as *mut ffi::objc_object, cls.as_ptr()) };
        let ptr: *const AnyClass = ptr.cast();
        // SAFETY: The class is not NULL because the object is not NULL.
        let old_cls = unsafe { ptr.as_ref().unwrap_unchecked() };
        // TODO: Check the superclass requirement too?
        debug_assert_eq!(
            old_cls.instance_size(),
            cls.instance_size(),
            "old and new class sizes were not equal; this is UB!"
        );
        old_cls
    }

    /// Offset an object pointer to get a pointer to an ivar.
    ///
    ///
    /// # Safety
    ///
    /// The offset must be valid for the given type.
    #[inline]
    pub(crate) unsafe fn ivar_at_offset<T>(ptr: NonNull<Self>, offset: isize) -> NonNull<T> {
        // `offset` is given in bytes, so we convert to `u8` and back to `T`
        let ptr: NonNull<u8> = ptr.cast();
        let ptr: *mut u8 = ptr.as_ptr();
        // SAFETY: The offset is valid
        let ptr: *mut u8 = unsafe { ptr.offset(offset) };
        // SAFETY: The offset operation is guaranteed to not end up computing
        // a NULL pointer.
        let ptr: NonNull<u8> = unsafe { NonNull::new_unchecked(ptr) };
        let ptr: NonNull<T> = ptr.cast();
        ptr
    }

    pub(crate) fn lookup_instance_variable_dynamically(&self, name: &str) -> &'static Ivar {
        let cls = self.class();
        cls.instance_variable(name)
            .unwrap_or_else(|| panic!("ivar {name} not found on class {cls}"))
    }

    /// Use [`Ivar::load`] instead.
    ///
    ///
    /// # Safety
    ///
    /// The object must have an instance variable with the given name, and it
    /// must be of type `T`.
    ///
    /// See [`Ivar::load_ptr`] for details surrounding this.
    #[deprecated = "this is difficult to use correctly, use `Ivar::load` instead."]
    pub unsafe fn get_ivar<T: Encode>(&self, name: &str) -> &T {
        let ivar = self.lookup_instance_variable_dynamically(name);
        // SAFETY: Upheld by caller
        unsafe { ivar.load::<T>(self) }
    }

    /// Use [`Ivar::load_mut`] instead.
    ///
    ///
    /// # Safety
    ///
    /// The object must have an instance variable with the given name, and it
    /// must be of type `T`.
    ///
    /// See [`Ivar::load_ptr`] for details surrounding this.
    #[deprecated = "this is difficult to use correctly, use `Ivar::load_mut` instead."]
    pub unsafe fn get_mut_ivar<T: Encode>(&mut self, name: &str) -> &mut T {
        let ivar = self.lookup_instance_variable_dynamically(name);
        // SAFETY: Upheld by caller
        unsafe { ivar.load_mut::<T>(self) }
    }

    // objc_setAssociatedObject
    // objc_getAssociatedObject
    // objc_removeAssociatedObjects
}

impl fmt::Debug for AnyObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}: {:p}>", self.class().name(), self.as_ptr())
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::string::ToString;
    use core::mem::size_of;

    use super::*;
    use crate::test_utils;
    use crate::{class, msg_send, sel, ClassType};

    #[test]
    fn test_selector() {
        macro_rules! test_sel {
            ($s:literal, $($tt:tt)+) => {{
                let sel = sel!($($tt)*);
                let expected = Sel::register($s);
                assert_eq!(sel, expected);
                assert_eq!(sel.name(), $s);
            }}
        }
        test_sel!("abc", abc);
        test_sel!("abc:", abc:);
        test_sel!("abc:def:", abc:def:);
        test_sel!("abc:def:ghi:", abc:def:ghi:);
        test_sel!("functionWithControlPoints::::", functionWithControlPoints::::);
        test_sel!("initWithControlPoints::::", initWithControlPoints::::);
        test_sel!("setFloatValue::", setFloatValue::);
        test_sel!("isSupported::", isSupported::);
        test_sel!("addEventListener:::", addEventListener:::);
        test_sel!("test::arg::", test::arg::);
        test_sel!("test::::with::spaces::", test : :: : with : : spaces : :);
        test_sel!("a::b:", a::b:);
    }

    #[test]
    fn test_empty_selector() {
        let sel = Sel::register("");
        assert_eq!(sel.name(), "");
        let sel = Sel::register(":");
        assert_eq!(sel.name(), ":");
        let sel = Sel::register("::");
        assert_eq!(sel.name(), "::");
    }

    #[test]
    #[should_panic = "NulError"]
    fn test_sel_register_null() {
        let _ = Sel::register("\0");
    }

    #[test]
    fn test_ivar() {
        let cls = test_utils::custom_class();
        let ivar = cls.instance_variable("_foo").unwrap();
        assert_eq!(ivar.name(), "_foo");
        assert!(<u32>::ENCODING.equivalent_to_str(ivar.type_encoding()));
        assert!(ivar.offset() > 0);
        assert!(cls.instance_variables().len() > 0);
    }

    #[test]
    fn test_instance_method() {
        let cls = test_utils::custom_class();
        let sel = Sel::register("foo");
        let method = cls.instance_method(sel).unwrap();
        assert_eq!(method.name().name(), "foo");
        assert_eq!(method.arguments_count(), 2);

        assert!(<u32>::ENCODING.equivalent_to_str(&method.return_type()));
        assert!(Sel::ENCODING.equivalent_to_str(&method.argument_type(1).unwrap()));

        assert!(cls.instance_methods().iter().any(|m| *m == method));
    }

    #[test]
    fn test_class_method() {
        let cls = test_utils::custom_class();
        let method = cls.class_method(sel!(classFoo)).unwrap();
        assert_eq!(method.name().name(), "classFoo");
        assert_eq!(method.arguments_count(), 2);

        assert!(<u32>::ENCODING.equivalent_to_str(&method.return_type()));
        assert!(Sel::ENCODING.equivalent_to_str(&method.argument_type(1).unwrap()));

        assert!(cls
            .metaclass()
            .instance_methods()
            .iter()
            .any(|m| *m == method));
    }

    #[test]
    fn test_class() {
        let cls = test_utils::custom_class();
        assert_eq!(cls.name(), "CustomObject");
        assert!(cls.instance_size() > 0);
        assert!(cls.superclass().is_none());

        assert!(cls.responds_to(sel!(foo)));
        assert!(cls.responds_to(sel!(setBar:)));
        assert!(cls.responds_to(sel!(test::test::)));
        assert!(!cls.responds_to(sel!(abc)));
        assert!(!cls.responds_to(sel!(addNumber:toNumber:)));

        assert_eq!(AnyClass::get(cls.name()), Some(cls));

        let metaclass = cls.metaclass();
        // The metaclass of a root class is a subclass of the root class
        assert_eq!(metaclass.superclass().unwrap(), cls);
        assert!(metaclass.responds_to(sel!(addNumber:toNumber:)));
        assert!(metaclass.responds_to(sel!(test::test::)));
        // TODO: This is unexpected!
        assert!(metaclass.responds_to(sel!(foo)));

        let subclass = test_utils::custom_subclass();
        assert_eq!(subclass.superclass().unwrap(), cls);
    }

    #[test]
    fn test_classes_count() {
        assert!(AnyClass::classes_count() > 0);
    }

    #[test]
    fn test_classes() {
        let classes = AnyClass::classes();
        assert!(classes.len() > 0);
    }

    #[test]
    fn test_protocol() {
        let proto = test_utils::custom_protocol();
        assert_eq!(proto.name(), "CustomProtocol");
        let class = test_utils::custom_class();
        assert!(class.conforms_to(proto));

        // The selectors are broken somehow on GNUStep < 2.0
        if cfg!(any(not(feature = "gnustep-1-7"), feature = "gnustep-2-0")) {
            let desc = MethodDescription {
                sel: sel!(setBar:),
                types: "v@:i",
            };
            assert_eq!(&proto.method_descriptions(true), &[desc]);
            let desc = MethodDescription {
                sel: sel!(getName),
                types: "*@:",
            };
            assert_eq!(&proto.method_descriptions(false), &[desc]);
            let desc = MethodDescription {
                sel: sel!(addNumber:toNumber:),
                types: "i@:ii",
            };
            assert_eq!(&proto.class_method_descriptions(true), &[desc]);
        }
        assert_eq!(&proto.class_method_descriptions(false), &[]);

        assert!(class.adopted_protocols().iter().any(|p| *p == proto));
    }

    #[test]
    fn test_protocol_method() {
        let class = test_utils::custom_class();
        let result: i32 = unsafe { msg_send![class, addNumber: 1, toNumber: 2] };
        assert_eq!(result, 3);
    }

    #[test]
    fn test_subprotocols() {
        let sub_proto = test_utils::custom_subprotocol();
        let super_proto = test_utils::custom_protocol();
        assert!(sub_proto.conforms_to(super_proto));
        assert_eq!(sub_proto.adopted_protocols()[0], super_proto);
    }

    #[test]
    fn test_protocols() {
        // Ensure that a protocol has been registered on linux
        let _ = test_utils::custom_protocol();

        assert!(AnyProtocol::protocols().len() > 0);
    }

    #[test]
    fn test_object() {
        let mut obj = test_utils::custom_object();
        let cls = test_utils::custom_class();
        assert_eq!(obj.class(), cls);

        let ivar = cls.instance_variable("_foo").unwrap();

        unsafe { *ivar.load_mut::<u32>(&mut obj) = 4 };
        let result = unsafe { *ivar.load::<u32>(&obj) };
        assert_eq!(result, 4);
    }

    #[test]
    fn test_object_ivar_unknown() {
        let cls = test_utils::custom_class();
        assert_eq!(cls.instance_variable("unknown"), None);
    }

    #[test]
    #[cfg_attr(
        feature = "malloc",
        ignore = "The `malloc_buf` crate calls `from_raw_parts` unsoundly"
    )]
    fn test_no_ivars() {
        let cls = ClassBuilder::new("NoIvarObject", NSObject::class())
            .unwrap()
            .register();
        assert_eq!(cls.instance_variables().len(), 0);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic = "wrong encoding. Tried to retrieve ivar with encoding I, but the encoding of the given type was C"
    )]
    fn test_object_ivar_wrong_type() {
        let obj = test_utils::custom_object();
        let cls = test_utils::custom_class();
        let ivar = cls.instance_variable("_foo").unwrap();
        let _ = unsafe { *ivar.load::<u8>(&obj) };
    }

    #[test]
    fn test_encode() {
        fn assert_enc<T: Encode>(expected: &str) {
            assert_eq!(&T::ENCODING.to_string(), expected);
        }
        assert_enc::<&AnyObject>("@");
        assert_enc::<*mut AnyObject>("@");
        assert_enc::<&AnyClass>("#");
        assert_enc::<Sel>(":");
        assert_enc::<Option<Sel>>(":");
        assert_enc::<Imp>("^?");
        assert_enc::<Option<Imp>>("^?");
        assert_enc::<&AnyProtocol>("@");
    }

    #[test]
    fn test_send_sync() {
        fn assert_send_sync<T: Send + Sync + ?Sized>() {}
        assert_send_sync::<Bool>();
        assert_send_sync::<AnyClass>();
        assert_send_sync::<Ivar>();
        assert_send_sync::<Method>();
        assert_send_sync::<AnyProtocol>();
        assert_send_sync::<Sel>();
    }

    #[test]
    fn test_debug_display() {
        let sel = sel!(abc:);
        assert_eq!(format!("{sel}"), "abc:");
        assert_eq!(format!("{sel:?}"), "Sel(\"abc:\")");
        let cls = test_utils::custom_class();
        assert_eq!(format!("{cls}"), "CustomObject");
        assert_eq!(
            format!("{cls:?}"),
            "AnyClass { name: \"CustomObject\", .. }"
        );
        let protocol = test_utils::custom_protocol();
        assert_eq!(format!("{protocol}"), "CustomProtocol");
        assert_eq!(
            format!("{protocol:?}"),
            "AnyProtocol { name: \"CustomProtocol\", .. }"
        );

        let object = test_utils::custom_object();
        assert_eq!(
            format!("{:?}", &*object),
            format!("CustomObject(<CustomObject: {:p}>)", &*object)
        );
    }

    #[test]
    fn test_multiple_colon() {
        let class = test_utils::custom_class();
        let res: i32 = unsafe {
            MessageReceiver::send_message(class, sel!(test::test::), (1i32, 2i32, 3i32, 4i32))
        };
        assert_eq!(res, 10);

        let obj = test_utils::custom_object();
        let res: i32 = unsafe {
            MessageReceiver::send_message(&*obj, sel!(test::test::), (1i32, 2i32, 3i32, 4i32))
        };
        assert_eq!(res, 24);
    }

    #[test]
    fn test_sizes() {
        assert_eq!(size_of::<Sel>(), size_of::<*const ()>());
        assert_eq!(size_of::<Sel>(), size_of::<Option<Sel>>());

        // These must be zero-sized until we get extern types, otherwise the
        // optimizer may invalidly assume something about their layout.
        assert_eq!(size_of::<AnyClass>(), 0);
        assert_eq!(size_of::<AnyObject>(), 0);
        assert_eq!(size_of::<AnyProtocol>(), 0);
        assert_eq!(size_of::<Ivar>(), 0);
        assert_eq!(size_of::<Method>(), 0);
    }

    fn get_ivar_layout(cls: &AnyClass) -> *const u8 {
        let cls: *const AnyClass = cls;
        unsafe { ffi::class_getIvarLayout(cls.cast()) }
    }

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "ivar layout is still used on GNUStep"
    )]
    fn test_layout_does_not_matter_any_longer() {
        assert!(get_ivar_layout(class!(NSObject)).is_null());
        assert!(get_ivar_layout(class!(NSArray)).is_null());
        assert!(get_ivar_layout(class!(NSException)).is_null());
        assert!(get_ivar_layout(class!(NSNumber)).is_null());
        assert!(get_ivar_layout(class!(NSString)).is_null());
    }

    // Required for backwards compat
    #[test]
    #[cfg(feature = "malloc")]
    fn test_still_has_malloc_buf_type() {
        let _: malloc_buf::Malloc<[&AnyClass]> = AnyClass::classes();
    }

    #[cfg(feature = "malloc")]
    #[allow(dead_code)]
    fn assert_malloc_buf_compatible_with_anonymous_type(
    ) -> (MallocSlice!(&'static AnyClass), MallocStr!()) {
        (
            AnyClass::classes(),
            NSObject::class()
                .instance_method(sel!(description))
                .unwrap()
                .return_type(),
        )
    }
}
