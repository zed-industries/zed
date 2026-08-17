// Take a look at the license at the top of the repository in the LICENSE file.

use std::{cmp::Ordering, ops, slice};

use crate::{
    ffi, gobject_ffi, prelude::*, translate::*, ParamSpecValueArray, ParamSpecValueArrayBuilder,
    Type, Value,
};

wrapper! {
    #[derive(Debug)]
    #[doc(alias = "GValueArray")]
    pub struct ValueArray(Boxed<gobject_ffi::GValueArray>);

    match fn {
        copy => |ptr| gobject_ffi::g_value_array_copy(mut_override(ptr)),
        free => |ptr| gobject_ffi::g_value_array_free(ptr),
    }
}

impl ValueArray {
    #[inline]
    pub fn new(values: impl IntoIterator<Item = impl ToValue>) -> Self {
        let iter = values.into_iter();
        let mut array = Self::with_capacity(iter.size_hint().0);
        for v in iter {
            array.append(v.to_value());
        }

        array
    }

    #[inline]
    pub fn from_values(values: impl IntoIterator<Item = Value>) -> Self {
        Self::new(values)
    }

    #[doc(alias = "g_value_array_new")]
    #[inline]
    pub fn with_capacity(capacity: usize) -> ValueArray {
        assert!(capacity <= u32::MAX as usize);

        unsafe { from_glib_full(gobject_ffi::g_value_array_new(capacity as u32)) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.n_values as usize
    }

    #[doc(alias = "g_value_array_append")]
    #[inline]
    pub fn append(&mut self, value: impl ToValue) {
        self.append_value(&value.to_value());
    }

    #[doc(alias = "g_value_array_append")]
    #[inline]
    pub fn append_value(&mut self, value: &Value) {
        unsafe {
            gobject_ffi::g_value_array_append(self.to_glib_none_mut().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "g_value_array_insert")]
    #[inline]
    pub fn insert(&mut self, index_: usize, value: impl ToValue) {
        self.insert_value(index_, &value.to_value());
    }

    #[doc(alias = "g_value_array_insert")]
    #[inline]
    pub fn insert_value(&mut self, index_: usize, value: &Value) {
        assert!(index_ <= self.len());

        unsafe {
            gobject_ffi::g_value_array_insert(
                self.to_glib_none_mut().0,
                index_ as u32,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_value_array_prepend")]
    #[inline]
    pub fn prepend(&mut self, value: impl ToValue) {
        self.prepend_value(&value.to_value());
    }

    #[doc(alias = "g_value_array_prepend")]
    #[inline]
    pub fn prepend_value(&mut self, value: &Value) {
        unsafe {
            gobject_ffi::g_value_array_prepend(self.to_glib_none_mut().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "g_value_array_remove")]
    #[inline]
    pub fn remove(&mut self, index_: usize) {
        assert!(index_ < self.len());

        unsafe {
            gobject_ffi::g_value_array_remove(self.to_glib_none_mut().0, index_ as u32);
        }
    }

    #[doc(alias = "g_value_array_sort_with_data")]
    pub fn sort_with_data<F: FnMut(&Value, &Value) -> Ordering>(&mut self, compare_func: F) {
        unsafe extern "C" fn compare_func_trampoline(
            a: ffi::gconstpointer,
            b: ffi::gconstpointer,
            func: ffi::gpointer,
        ) -> i32 {
            let func = func as *mut &mut dyn FnMut(&Value, &Value) -> Ordering;

            let a = &*(a as *const Value);
            let b = &*(b as *const Value);

            (*func)(a, b).into_glib()
        }
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut dyn FnMut(&Value, &Value) -> Ordering = &mut func;
            let func_ptr =
                &func_obj as *const &mut dyn FnMut(&Value, &Value) -> Ordering as ffi::gpointer;

            gobject_ffi::g_value_array_sort_with_data(
                self.to_glib_none_mut().0,
                Some(compare_func_trampoline),
                func_ptr,
            );
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[Value] {
        if self.is_empty() {
            return &[];
        }

        unsafe {
            slice::from_raw_parts(
                (*self.as_ptr()).values as *const Value,
                (*self.as_ptr()).n_values as usize,
            )
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Value] {
        if self.is_empty() {
            return &mut [];
        }

        unsafe {
            slice::from_raw_parts_mut(
                (*self.as_ptr()).values as *mut Value,
                (*self.as_ptr()).n_values as usize,
            )
        }
    }
}

impl ops::Deref for ValueArray {
    type Target = [Value];

    #[inline]
    fn deref(&self) -> &[Value] {
        self.as_slice()
    }
}

impl ops::DerefMut for ValueArray {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Value] {
        self.as_mut_slice()
    }
}

impl Default for ValueArray {
    fn default() -> Self {
        Self::with_capacity(8)
    }
}

impl std::iter::FromIterator<Value> for ValueArray {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self::from_values(iter)
    }
}

impl std::iter::Extend<Value> for ValueArray {
    fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
        for v in iter.into_iter() {
            self.append_value(&v);
        }
    }
}

// Implementing `Value` traits manually because of a custom ParamSpec
impl StaticType for ValueArray {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::g_value_array_get_type()) }
    }
}

#[doc(hidden)]
impl ValueType for ValueArray {
    type Type = Self;
}

#[doc(hidden)]
impl crate::value::ValueTypeOptional for ValueArray {}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for ValueArray {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_dup_boxed(value.to_glib_none().0);
        debug_assert!(!ptr.is_null());
        from_glib_full(ptr as *mut gobject_ffi::GValueArray)
    }
}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for &'a ValueArray {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a Value) -> Self {
        debug_assert_eq!(
            std::mem::size_of::<Self>(),
            std::mem::size_of::<ffi::gpointer>()
        );
        let value = &*(value as *const Value as *const gobject_ffi::GValue);
        debug_assert!(!value.data[0].v_pointer.is_null());
        <ValueArray>::from_glib_ptr_borrow(
            &*(&value.data[0].v_pointer as *const ffi::gpointer
                as *const *mut gobject_ffi::GValueArray),
        )
    }
}

#[doc(hidden)]
impl ToValue for ValueArray {
    #[inline]
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::from_type_unchecked(<Self as StaticType>::static_type());
            gobject_ffi::g_value_take_boxed(
                value.to_glib_none_mut().0,
                ToGlibPtr::<*mut gobject_ffi::GValueArray>::to_glib_full(self) as *mut _,
            );
            value
        }
    }

    #[inline]
    fn value_type(&self) -> Type {
        <Self as StaticType>::static_type()
    }
}

impl std::convert::From<ValueArray> for Value {
    #[inline]
    fn from(o: ValueArray) -> Self {
        unsafe {
            let mut value = Value::from_type_unchecked(<ValueArray as StaticType>::static_type());
            gobject_ffi::g_value_take_boxed(
                value.to_glib_none_mut().0,
                IntoGlibPtr::<*mut gobject_ffi::GValueArray>::into_glib_ptr(o) as *mut _,
            );
            value
        }
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for ValueArray {
    #[inline]
    fn to_value_optional(s: Option<&Self>) -> Value {
        let mut value = Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_take_boxed(
                value.to_glib_none_mut().0,
                ToGlibPtr::<*mut gobject_ffi::GValueArray>::to_glib_full(&s) as *mut _,
            );
        }

        value
    }
}

impl HasParamSpec for ValueArray {
    type ParamSpec = ParamSpecValueArray;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecValueArrayBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr = ValueArray::new(["123", "456"]);
        assert_eq!(
            arr.first().and_then(|v| v.get::<String>().ok()),
            Some(String::from("123"))
        );
        assert_eq!(
            arr.get(1).and_then(|v| v.get::<String>().ok()),
            Some(String::from("456"))
        );
    }

    #[test]
    fn test_append() {
        let mut arr = ValueArray::default();
        arr.append("123");
        arr.append(123u32);
        arr.append_value(&Value::from(456u64));

        assert_eq!(
            arr.first().and_then(|v| v.get::<String>().ok()),
            Some(String::from("123"))
        );
        assert_eq!(arr.get(1).and_then(|v| v.get::<u32>().ok()), Some(123));
        assert_eq!(arr.get(2).and_then(|v| v.get::<u64>().ok()), Some(456));
    }
}
