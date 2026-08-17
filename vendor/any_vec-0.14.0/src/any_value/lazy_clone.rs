use core::any::TypeId;
use crate::any_value::{AnyValue, AnyValueCloneable, AnyValueTypeless, AnyValueSizeless};

/// Makes [`AnyValueCloneable`] actually [`Clone`]able.
/// Clone underlying value on consumption.
///
/// Source must outlive `LazyClone`. `LazyClone` let you
/// take element from one [`AnyVec`] and put it multiple times
/// into another, without intermediate copies and cast to concrete type.
/// 
/// Can be constructed by calling [AnyValueCloneable::lazy_clone].
///
/// [`AnyVec`]: crate::AnyVec
pub struct LazyClone<'a, T: AnyValueCloneable>{
    value: &'a T
}

impl<'a, T: AnyValueCloneable> LazyClone<'a, T>{
    #[inline]
    pub fn new(value: &'a T) -> Self{
        Self{value}
    }
}

impl<'a, T: AnyValueCloneable> AnyValueSizeless for LazyClone<'a, T> {
    type Type = T::Type;

    #[inline]
    fn as_bytes_ptr(&self) -> *const u8 {
        self.value.as_bytes_ptr()
    }

    #[inline]
    unsafe fn move_into<KnownType:'static /*= Unknown*/>(self, out: *mut u8, _bytes_size: usize) {
        self.value.clone_into(out);
    }
}

impl<'a, T: AnyValueCloneable + AnyValueTypeless> AnyValueTypeless for LazyClone<'a, T>{
    #[inline]
    fn size(&self) -> usize {
        self.value.size()
    }
}

impl<'a, T: AnyValueCloneable + AnyValue> AnyValue for LazyClone<'a, T>{
    #[inline]
    fn value_typeid(&self) -> TypeId {
        self.value.value_typeid()
    }
}

impl<'a, T: AnyValueCloneable> AnyValueCloneable for LazyClone<'a, T>{
    #[inline]
    unsafe fn clone_into(&self, out: *mut u8) {
        self.value.clone_into(out);
    }
}

impl<'a, T: AnyValueCloneable> Clone for LazyClone<'a, T>{
    #[inline]
    fn clone(&self) -> Self {
        Self{value: self.value}
    }
}