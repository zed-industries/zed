use core::any::TypeId;
use core::mem::size_of;
use crate::any_value::{AnyValue, AnyValueMut, AnyValueTypelessMut, AnyValueTypeless, AnyValueSizeless, AnyValueSizelessMut};

/// Helper struct to convert concrete type to [`AnyValueMut`].
/// 
/// Unlike [AnyValueRaw] this one owns underlying value. So, its not
/// special in any way.
/// 
/// [AnyValueRaw]: super::AnyValueRaw
pub struct AnyValueWrapper<T: 'static>{
    value: T
}
impl<T: 'static> AnyValueWrapper<T> {
    #[inline]
    pub fn new(value: T) -> Self{
        Self{ value }
    }
}

impl<T: 'static> AnyValueSizeless for AnyValueWrapper<T> {
    type Type = T;

    #[inline]
    fn as_bytes_ptr(&self) -> *const u8 {
        &self.value as *const _ as *const u8
    }
}
impl<T: 'static> AnyValueSizelessMut for AnyValueWrapper<T> {
    #[inline]
    fn as_bytes_mut_ptr(&mut self) -> *mut u8 {
        &mut self.value as *mut _ as *mut u8
    }
}
impl<T: 'static> AnyValueTypeless for AnyValueWrapper<T> {
    #[inline]
    fn size(&self) -> usize {
        size_of::<T>()
    }
}
impl<T: 'static> AnyValue for AnyValueWrapper<T> {
    #[inline]
    fn value_typeid(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
impl<T: 'static> AnyValueTypelessMut for AnyValueWrapper<T> {}
impl<T: 'static> AnyValueMut for AnyValueWrapper<T> {}