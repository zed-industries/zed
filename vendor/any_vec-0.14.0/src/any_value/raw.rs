use core::any::TypeId;
use core::ptr::NonNull;
use crate::any_value::{AnyValue, AnyValueMut, AnyValueTypelessMut, AnyValueTypeless, AnyValueSizeless, AnyValueSizelessMut};
use crate::any_value::Unknown;

/// [AnyValueSizeless] non-owning byte ptr wrapper, that knows nothing about it's type.
/// 
/// Source should be forgotten, before pushing to [AnyVec].
/// Contained value **WILL NOT** be dropped on wrapper drop.
///
/// # Example
/// ```rust
/// # use std::any::TypeId;
/// # use std::mem;
/// # use std::mem::size_of;
/// # use std::ptr::NonNull;
/// # use any_vec::AnyVec;
/// # use any_vec::any_value::AnyValueSizelessRaw;
/// let s = String::from("Hello!");
/// let raw_value = unsafe{AnyValueSizelessRaw::new(
///     NonNull::from(&s).cast::<u8>()
/// )};
/// mem::forget(s);
///
/// let mut any_vec: AnyVec = AnyVec::new::<String>();
/// unsafe{
///     any_vec.push_unchecked(raw_value);
/// }
/// ```
///
/// [AnyVec]: crate::AnyVec
pub struct AnyValueSizelessRaw {
    ptr: NonNull<u8>
}

impl AnyValueSizelessRaw {
    #[inline]
    pub unsafe fn new(ptr: NonNull<u8>) -> Self{
        Self{ptr}
    }
}
impl AnyValueSizeless for AnyValueSizelessRaw {
    type Type = Unknown;

    #[inline]
    fn as_bytes_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}
impl AnyValueSizelessMut for AnyValueSizelessRaw {
    #[inline]
    fn as_bytes_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }
}

/// [AnyValueTypeless] byte ptr wrapper, that know it's type size.
/// 
/// Source should be forgotten, before pushing to [AnyVec].
/// Contained value **WILL NOT** be dropped on wrapper drop.
///
/// # Example
/// ```rust
/// # use std::any::TypeId;
/// # use std::mem;
/// # use std::mem::size_of;
/// # use std::ptr::NonNull;
/// # use any_vec::AnyVec;
/// # use any_vec::any_value::AnyValueTypelessRaw;
/// let s = String::from("Hello!");
/// let raw_value = unsafe{AnyValueTypelessRaw::new(
///     NonNull::from(&s).cast::<u8>(),
///     size_of::<String>()
/// )};
/// mem::forget(s);
///
/// let mut any_vec: AnyVec = AnyVec::new::<String>();
/// unsafe{
///     any_vec.push_unchecked(raw_value);
/// }
/// ```
///
/// [AnyVec]: crate::AnyVec
pub struct AnyValueTypelessRaw {
    ptr: NonNull<u8>,
    size: usize,
}

impl AnyValueTypelessRaw {
    #[inline]
    pub unsafe fn new(ptr: NonNull<u8>, size: usize) -> Self{
        Self{ptr, size}
    }
}

impl AnyValueSizeless for AnyValueTypelessRaw {
    type Type = Unknown;

    #[inline]
    fn as_bytes_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}
impl AnyValueSizelessMut for AnyValueTypelessRaw {
    #[inline]
    fn as_bytes_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }
}
impl AnyValueTypeless for AnyValueTypelessRaw {
    #[inline]
    fn size(&self) -> usize {
        self.size
    }
}
impl AnyValueTypelessMut for AnyValueTypelessRaw {}


/// [AnyValue] byte ptr wrapper, that know it's type.
/// 
/// Source should be forgotten, before pushing to [AnyVec].
/// Contained value **WILL NOT** be dropped on wrapper drop.
///
/// # Example
/// ```rust
/// # use std::any::TypeId;
/// # use std::mem;
/// # use std::mem::size_of;
/// # use std::ptr::NonNull;
/// # use any_vec::AnyVec;
/// # use any_vec::any_value::AnyValueRaw;
/// let s = String::from("Hello!");
/// let raw_value = unsafe{AnyValueRaw::new(
///     NonNull::from(&s).cast::<u8>(),
///     size_of::<String>(),
///     TypeId::of::<String>()
/// )};
/// mem::forget(s);
///
/// let mut any_vec: AnyVec = AnyVec::new::<String>();
/// unsafe{
///     any_vec.push_unchecked(raw_value);
/// }
/// ```
///
/// [AnyVec]: crate::AnyVec
pub struct AnyValueRaw {
    raw_unsafe: AnyValueTypelessRaw,
    typeid: TypeId
}

impl AnyValueRaw {
    #[inline]
    pub unsafe fn new(ptr: NonNull<u8>, size: usize, typeid: TypeId) -> Self{
        Self{
            raw_unsafe: AnyValueTypelessRaw::new(ptr, size),
            typeid
        }
    }
}

impl AnyValueSizeless for AnyValueRaw {
    type Type = Unknown;

    #[inline]
    fn as_bytes_ptr(&self) -> *const u8 {
        self.raw_unsafe.ptr.as_ptr()
    }
}
impl AnyValueSizelessMut   for AnyValueRaw {
    #[inline]
    fn as_bytes_mut_ptr(&mut self) -> *mut u8 {
        self.raw_unsafe.ptr.as_ptr()
    }
}
impl AnyValueTypeless for AnyValueRaw {
    #[inline]
    fn size(&self) -> usize {
        self.raw_unsafe.size
    }
}
impl AnyValue for AnyValueRaw {
    #[inline]
    fn value_typeid(&self) -> TypeId {
        self.typeid
    }
}

impl AnyValueTypelessMut for AnyValueRaw {}
impl AnyValueMut for AnyValueRaw {}