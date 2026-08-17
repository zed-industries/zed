// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `IMPL` Boxed wrapper implementation.

use std::{
    cmp, fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr,
};

use crate::translate::*;

// rustdoc-stripper-ignore-next
/// Wrapper implementations for Boxed types. See `wrapper!`.
#[macro_export]
macro_rules! glib_boxed_wrapper {
    ([$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty,
     @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr
     $(, @type_ $get_type_expr:expr)?) => {
        $crate::glib_boxed_wrapper!(@generic_impl [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name);

        $crate::glib_boxed_wrapper!(
            @memory_manager_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name,
            @copy $copy_arg $copy_expr, @free $free_arg $free_expr
        );

        $crate::glib_boxed_wrapper!(@value_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name $(, @type_ $get_type_expr)?);
    };

    (@generic_impl [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty) => {
        $(#[$attr])*
        #[doc = "\n\nGLib type: Boxed type with copy-on-clone semantics."]
        #[repr(transparent)]
        $visibility struct $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? {
            inner: $crate::boxed::Boxed<$ffi_name, Self>,
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $name $(<$($generic),+>)? {
            #[doc = "Return the inner pointer to the underlying C value."]
            #[inline]
            pub fn as_ptr(&self) -> *mut $ffi_name {
                unsafe { *(self as *const Self as *const *const $ffi_name) as *mut $ffi_name }
            }

            #[doc = "Borrows the underlying C value."]
            #[inline]
            pub unsafe fn from_glib_ptr_borrow(ptr: &*mut $ffi_name) -> &Self {
                debug_assert_eq!(
                    std::mem::size_of::<Self>(),
                    std::mem::size_of::<$crate::ffi::gpointer>()
                );
                debug_assert!(!ptr.is_null());
                &*(ptr as *const *mut $ffi_name as *const Self)
            }

            #[doc = "Borrows the underlying C value mutably."]
            #[inline]
            pub unsafe fn from_glib_ptr_borrow_mut(ptr: &mut *mut $ffi_name) -> &mut Self {
                debug_assert_eq!(
                    std::mem::size_of::<Self>(),
                    std::mem::size_of::<$crate::ffi::gpointer>()
                );
                debug_assert!(!ptr.is_null());
                &mut *(ptr as *mut *mut $ffi_name as *mut Self)
            }
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::clone::Clone for $name $(<$($generic),+>)? {
            #[doc = "Copies the boxed type with the type-specific copy function."]
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    inner: std::clone::Clone::clone(&self.inner),
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::GlibPtrDefault for $name $(<$($generic),+>)? {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::TransparentPtrType for $name $(<$($generic),+>)? {}

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = std::marker::PhantomData<&'a $crate::boxed::Boxed<$ffi_name, Self>>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.inner);
                $crate::translate::Stash(stash.0, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.inner)
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtr<'a, *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = std::marker::PhantomData<&'a $crate::boxed::Boxed<$ffi_name, Self>>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.inner);
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.inner) as *mut _
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtrMut<'a, *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = std::marker::PhantomData<&'a mut $crate::boxed::Boxed<$ffi_name, Self>>;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> $crate::translate::StashMut<'a, *mut $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut self.inner);
                $crate::translate::StashMut(stash.0, stash.1)
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *mut *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = (std::marker::PhantomData<&'a [Self]>, Option<Vec<*const $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [Self]) -> (*mut *const $ffi_name, Self::Storage) {
                let mut v_ptr = Vec::with_capacity(t.len() + 1);
                unsafe {
                    let ptr = v_ptr.as_mut_ptr();
                    std::ptr::copy_nonoverlapping(t.as_ptr() as *mut *const $ffi_name, ptr, t.len());
                    std::ptr::write(ptr.add(t.len()), std::ptr::null_mut());
                    v_ptr.set_len(t.len() + 1);
                }

                (v_ptr.as_ptr() as *mut *const $ffi_name, (std::marker::PhantomData, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [Self]) -> (*mut *const $ffi_name, Self::Storage) {
                let v_ptr = unsafe {
                    let v_ptr = $crate::ffi::g_malloc(std::mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    std::ptr::copy_nonoverlapping(t.as_ptr() as *mut *const $ffi_name, v_ptr, t.len());
                    std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());

                    v_ptr
                };

                (v_ptr, (std::marker::PhantomData, None))
            }

            fn to_glib_full_from_slice(t: &[Self]) -> *mut *const $ffi_name {
                unsafe {
                    let v_ptr = $crate::ffi::g_malloc(std::mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), $crate::translate::ToGlibPtr::to_glib_full(s));
                    }
                    std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *const *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = (std::marker::PhantomData<&'a [Self]>, Option<Vec<*const $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [Self]) -> (*const *const $ffi_name, Self::Storage) {
                let (ptr, stash) = $crate::translate::ToGlibContainerFromSlice::<'a, *mut *const $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *const $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [Self]) -> (*const *const $ffi_name, Self::Storage) {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[Self]) -> *const *const $ffi_name {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrNone<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                Self {
                    inner: $crate::translate::from_glib_none(ptr),
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrNone<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                Self {
                    inner: $crate::translate::from_glib_none(ptr),
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrFull<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                Self {
                    inner: $crate::translate::from_glib_full(ptr),
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrFull<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_full(ptr: *const $ffi_name) -> Self {
                Self {
                    inner: $crate::translate::from_glib_full(ptr),
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::Borrowed::new(
                    Self {
                        inner: $crate::translate::from_glib_borrow::<_, $crate::boxed::Boxed<_, _>>(ptr).into_inner(),
                    }
                )
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::from_glib_borrow::<_, Self>(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::<Self>::with_capacity(num);
                let res_ptr = res.as_mut_ptr();
                for i in 0..num {
                    ::std::ptr::write(res_ptr.add(i), $crate::translate::from_glib_none(std::ptr::read(ptr.add(i))));
                }
                res.set_len(num);
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    $crate::ffi::g_free(ptr as *mut _);
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                let res_ptr = res.as_mut_ptr();
                ::std::ptr::copy_nonoverlapping(ptr as *mut Self, res_ptr, num);
                res.set_len(num);
                $crate::ffi::g_free(ptr as *mut _);
                res
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }
        }
        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::IntoGlibPtr<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            fn into_glib_ptr(self) -> *mut $ffi_name {
                let s = std::mem::ManuallyDrop::new(self);
                $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&*s).0 as *mut _
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::IntoGlibPtr<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            fn into_glib_ptr(self) -> *const $ffi_name {
                let s = std::mem::ManuallyDrop::new(self);
                $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&*s).0 as *const _
            }
        }

    };

    (@value_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty) => {
     };

    (@value_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty, @type_ $get_type_expr:expr) => {
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::prelude::StaticType for $name $(<$($generic),+>)? {
            #[inline]
            fn static_type() -> $crate::types::Type {
                #[allow(unused_unsafe)]
                #[allow(clippy::macro_metavars_in_unsafe)]
                unsafe { $crate::translate::from_glib($get_type_expr) }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ValueType for $name $(<$($generic),+>)? {
            type Type = Self;
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ValueTypeOptional for $name $(<$($generic),+>)? { }

        #[doc(hidden)]
        unsafe impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::value::FromValue<'a> for $name $(<$($generic),+>)? {
            type Checker = $crate::value::GenericValueTypeOrNoneChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a $crate::Value) -> Self {
                let ptr = $crate::gobject_ffi::g_value_dup_boxed($crate::translate::ToGlibPtr::to_glib_none(value).0);
                debug_assert!(!ptr.is_null());
                <Self as $crate::translate::FromGlibPtrFull<*mut $ffi_name>>::from_glib_full(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        unsafe impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::value::FromValue<'a> for &'a $name $(<$($generic),+>)? {
            type Checker = $crate::value::GenericValueTypeOrNoneChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a $crate::Value) -> Self {
                let value = &*(value as *const $crate::Value as *const $crate::gobject_ffi::GValue);
                <$name $(<$($generic),+>)?>::from_glib_ptr_borrow(&*(&value.data[0].v_pointer as *const $crate::ffi::gpointer as *const *mut $ffi_name))
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ToValue for $name $(<$($generic),+>)? {
            #[inline]
            fn to_value(&self) -> $crate::Value {
                unsafe {
                    let mut value = $crate::Value::from_type_unchecked(<Self as $crate::prelude::StaticType>::static_type());
                    $crate::gobject_ffi::g_value_take_boxed(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_full(self) as *mut _,
                    );
                    value
                }
            }

            #[inline]
            fn value_type(&self) -> $crate::Type {
                <Self as $crate::prelude::StaticType>::static_type()
            }
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::convert::From<$name $(<$($generic),+>)?> for $crate::Value {
            #[inline]
            fn from(o: $name $(<$($generic),+>)?) -> Self {
                unsafe {
                    let mut value = $crate::Value::from_type_unchecked(<$name $(<$($generic),+>)? as $crate::prelude::StaticType>::static_type());
                    $crate::gobject_ffi::g_value_take_boxed(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::IntoGlibPtr::<*mut $ffi_name>::into_glib_ptr(o) as *mut _,
                    );
                    value
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ToValueOptional for $name $(<$($generic),+>)? {
            #[inline]
            fn to_value_optional(s: Option<&Self>) -> $crate::Value {
                let mut value = $crate::Value::for_value_type::<Self>();
                unsafe {
                    $crate::gobject_ffi::g_value_take_boxed(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_full(&s) as *mut _,
                    );
                }

                value
            }
        }


        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::prelude::HasParamSpec for $name $(<$($generic),+>)? {
            type ParamSpec = $crate::ParamSpecBoxed;
            type SetValue = Self;
            type BuilderFn = fn(&str) -> $crate::ParamSpecBoxedBuilder<Self>;

            fn param_spec_builder() -> Self::BuilderFn {
                |name| Self::ParamSpec::builder(name)
            }
        }
    };

    (@memory_manager_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty, @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr) => {
        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::boxed::BoxedMemoryManager for $name $(<$($generic),+>)? {
            type Target = $ffi_name;

            #[inline]
            unsafe fn copy($copy_arg: *const Self::Target) -> *mut Self::Target {
                $copy_expr
            }

            #[inline]
            #[allow(clippy::no_effect)]
            unsafe fn free($free_arg: *mut Self::Target) {
                $free_expr;
            }
        }
    };
}

// The safety docs really belong in the wrapper!() macro for Boxed<T>
/// Memory management functions for a boxed type.
pub trait BoxedMemoryManager: 'static {
    type Target;

    /// Makes a copy.
    unsafe fn copy(ptr: *const Self::Target) -> *mut Self::Target;
    /// Frees the object.
    unsafe fn free(ptr: *mut Self::Target);
}

/// Encapsulates memory management logic for boxed types.
#[repr(transparent)]
pub struct Boxed<T: 'static, MM: BoxedMemoryManager<Target = T>> {
    inner: ptr::NonNull<T>,
    _dummy: PhantomData<*mut MM>,
}

impl<'a, T: 'static, MM: BoxedMemoryManager<Target = T>> ToGlibPtr<'a, *const T> for Boxed<T, MM> {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const T, Self> {
        let ptr = self.inner.as_ptr();
        Stash(ptr, PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *const T {
        let ptr = self.inner.as_ptr();
        unsafe { MM::copy(ptr) }
    }
}

impl<'a, T: 'static, MM: BoxedMemoryManager<Target = T>> ToGlibPtrMut<'a, *mut T> for Boxed<T, MM> {
    type Storage = PhantomData<&'a mut Self>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut T, Self> {
        let ptr = self.inner.as_ptr();
        StashMut(ptr, PhantomData)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> FromGlibPtrNone<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        debug_assert!(!ptr.is_null());
        let ptr = MM::copy(ptr);
        from_glib_full(ptr)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> FromGlibPtrNone<*const T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const T) -> Self {
        debug_assert!(!ptr.is_null());
        let ptr = MM::copy(ptr);
        from_glib_full(ptr)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> FromGlibPtrFull<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut T) -> Self {
        debug_assert!(!ptr.is_null());
        Self {
            inner: ptr::NonNull::new_unchecked(ptr),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> FromGlibPtrFull<*const T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_full(ptr: *const T) -> Self {
        debug_assert!(!ptr.is_null());
        Self {
            inner: ptr::NonNull::new_unchecked(ptr as *mut T),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> FromGlibPtrBorrow<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut T) -> Borrowed<Self> {
        debug_assert!(!ptr.is_null());
        Borrowed::new(Self {
            inner: ptr::NonNull::new_unchecked(ptr),
            _dummy: PhantomData,
        })
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> Drop for Boxed<T, MM> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            MM::free(self.inner.as_ptr());
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> fmt::Debug for Boxed<T, MM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Boxed").field("inner", &self.inner).finish()
    }
}

impl<T, MM: BoxedMemoryManager<Target = T>> PartialOrd for Boxed<T, MM> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, MM: BoxedMemoryManager<Target = T>> Ord for Boxed<T, MM> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.to_glib_none().0.cmp(&other.to_glib_none().0)
    }
}

impl<T, MM: BoxedMemoryManager<Target = T>> PartialEq for Boxed<T, MM> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.to_glib_none().0 == other.to_glib_none().0
    }
}

impl<T, MM: BoxedMemoryManager<Target = T>> Eq for Boxed<T, MM> {}

impl<T, MM: BoxedMemoryManager<Target = T>> Hash for Boxed<T, MM> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.to_glib_none().0.hash(state)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> Clone for Boxed<T, MM> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { from_glib_none(self.to_glib_none().0 as *mut T) }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> Deref for Boxed<T, MM> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe {
            // This is safe because the pointer will remain valid while self is borrowed
            &*self.to_glib_none().0
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<Target = T>> DerefMut for Boxed<T, MM> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            // This is safe because the pointer will remain valid while self is borrowed
            &mut *self.to_glib_none_mut().0
        }
    }
}
