// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `IMPL` Shared (reference counted) wrapper implementation.

use std::{
    cmp, fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ptr,
};

use crate::translate::*;

// rustdoc-stripper-ignore-next
/// Wrapper implementations for shared types. See `wrapper!`.
#[macro_export]
macro_rules! glib_shared_wrapper {
    ([$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty,
     @ref $ref_arg:ident $ref_expr:expr, @unref $unref_arg:ident $unref_expr:expr
     $(, @type_ $get_type_expr:expr)?) => {
        $crate::glib_shared_wrapper!(
            @generic_impl [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name,
            @ref $ref_arg $ref_expr, @unref $unref_arg $unref_expr
        );

        $crate::glib_shared_wrapper!(@value_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name $(, @type_ $get_type_expr)?);
    };

    (@generic_impl [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty,
     @ref $ref_arg:ident $ref_expr:expr, @unref $unref_arg:ident $unref_expr:expr) => {
        $(#[$attr])*
        #[doc = "\n\nGLib type: Shared boxed type with reference counted clone semantics."]
        #[repr(transparent)]
        $visibility struct $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? {
            inner: $crate::shared::Shared<$ffi_name, Self>,
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
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::clone::Clone for $name $(<$($generic),+>)? {
            #[doc = "Makes a clone of this shared reference.\n\nThis increments the strong reference count of the reference. Dropping the reference will decrement it again."]
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    inner: std::clone::Clone::clone(&self.inner),
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::shared::SharedMemoryManager for $name $(<$($generic),+>)? {
            type Target = $ffi_name;

            #[inline]
            unsafe fn ref_($ref_arg: *mut Self::Target) {
                $ref_expr;
            }

            #[inline]
            #[allow(clippy::no_effect)]
            unsafe fn unref($unref_arg: *mut Self::Target) {
                $unref_expr;
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::GlibPtrDefault for $name $(<$($generic),+>)? {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::TransparentPtrType for $name $(<$($generic),+>)? {}

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtr<'a, *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = std::marker::PhantomData<&'a $crate::shared::Shared<$ffi_name, Self>>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.inner);
                $crate::translate::Stash(stash.0, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.inner)
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = std::marker::PhantomData<&'a $crate::shared::Shared<$ffi_name, Self>>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.inner);
                $crate::translate::Stash(stash.0 as *const _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.inner) as *const _
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *mut *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = (std::marker::PhantomData<&'a [Self]>, Option<Vec<*mut $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [Self]) -> (*mut *mut $ffi_name, Self::Storage) {
                let mut v_ptr = Vec::with_capacity(t.len() + 1);
                unsafe {
                    let ptr = v_ptr.as_mut_ptr();
                    std::ptr::copy_nonoverlapping(t.as_ptr() as *mut *mut $ffi_name, ptr, t.len());
                    std::ptr::write(ptr.add(t.len()), std::ptr::null_mut());
                    v_ptr.set_len(t.len() + 1);
                }

                (v_ptr.as_ptr() as *mut *mut $ffi_name, (std::marker::PhantomData, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [Self]) -> (*mut *mut $ffi_name, Self::Storage) {
                let v_ptr = unsafe {
                    let v_ptr = $crate::ffi::g_malloc(std::mem::size_of::<*mut $ffi_name>() * (t.len() + 1)) as *mut *mut $ffi_name;

                    std::ptr::copy_nonoverlapping(t.as_ptr() as *mut *mut $ffi_name, v_ptr, t.len());
                    std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());

                    v_ptr
                };

                (v_ptr, (std::marker::PhantomData, None))
            }

            fn to_glib_full_from_slice(t: &[Self]) -> *mut *mut $ffi_name {
                unsafe {
                    let v_ptr = $crate::ffi::g_malloc(std::mem::size_of::<*mut $ffi_name>() * (t.len() + 1)) as *mut *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), $crate::translate::ToGlibPtr::to_glib_full(s));
                    }
                    std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *const *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = (std::marker::PhantomData<&'a [Self]>, Option<Vec<*mut $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [Self]) -> (*const *mut $ffi_name, Self::Storage) {
                let (ptr, stash) = $crate::translate::ToGlibContainerFromSlice::<'a, *mut *mut $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *mut $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [Self]) -> (*const *mut $ffi_name, Self::Storage) {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[Self]) -> *const *mut $ffi_name {
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
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::Borrowed::new(
                    Self {
                        inner: $crate::translate::from_glib_borrow::<_, $crate::shared::Shared<_, _>>(ptr).into_inner(),
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
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_num_as_vec(ptr: *const *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            unsafe fn from_glib_container_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_as_vec(ptr: *const *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            unsafe fn from_glib_container_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
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

    (@value_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty) => { };

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
                        $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_full(self) as *mut _,
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
            fn from(s: $name $(<$($generic),+>)?) -> Self {
                unsafe {
                    let mut value = $crate::Value::from_type_unchecked(<$name $(<$($generic),+>)? as $crate::prelude::StaticType>::static_type());
                    $crate::gobject_ffi::g_value_take_boxed(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::IntoGlibPtr::<*mut $ffi_name>::into_glib_ptr(s) as *mut _,
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
                        $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_full(&s) as *mut _,
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
}

pub trait SharedMemoryManager {
    type Target;

    /// # Safety
    ///
    /// Callers are responsible for ensuring that a matching call to `unref`
    /// is made at an appropriate time.
    unsafe fn ref_(ptr: *mut Self::Target);

    /// # Safety
    ///
    /// Callers are responsible for ensuring that a matching call to `ref` was
    /// made before this is called, and that the pointer is not used after the
    /// `unref` call.
    unsafe fn unref(ptr: *mut Self::Target);
}

/// Encapsulates memory management logic for shared types.
#[repr(transparent)]
pub struct Shared<T, MM: SharedMemoryManager<Target = T>> {
    inner: ptr::NonNull<T>,
    mm: PhantomData<*const MM>,
}

impl<T, MM: SharedMemoryManager<Target = T>> Drop for Shared<T, MM> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            MM::unref(self.inner.as_ptr());
        }
    }
}

impl<T, MM: SharedMemoryManager<Target = T>> Clone for Shared<T, MM> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            MM::ref_(self.inner.as_ptr());
        }
        Self {
            inner: self.inner,
            mm: PhantomData,
        }
    }
}

impl<T, MM: SharedMemoryManager<Target = T>> fmt::Debug for Shared<T, MM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Shared")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T, MM: SharedMemoryManager<Target = T>> PartialOrd for Shared<T, MM> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, MM: SharedMemoryManager<Target = T>> Ord for Shared<T, MM> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T, MM: SharedMemoryManager<Target = T>> PartialEq for Shared<T, MM> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T, MM: SharedMemoryManager<Target = T>> Eq for Shared<T, MM> {}

impl<T, MM: SharedMemoryManager<Target = T>> Hash for Shared<T, MM> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.inner.hash(state)
    }
}

impl<'a, T: 'static, MM> ToGlibPtr<'a, *mut T> for Shared<T, MM>
where
    MM: SharedMemoryManager<Target = T> + 'static,
{
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut T, Self> {
        Stash(self.inner.as_ptr(), PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut T {
        unsafe {
            MM::ref_(self.inner.as_ptr());
        }
        self.inner.as_ptr()
    }
}

impl<T: 'static, MM: SharedMemoryManager<Target = T>> FromGlibPtrNone<*mut T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        debug_assert!(!ptr.is_null());
        MM::ref_(ptr);
        Self {
            inner: ptr::NonNull::new_unchecked(ptr),
            mm: PhantomData,
        }
    }
}

impl<T: 'static, MM: SharedMemoryManager<Target = T>> FromGlibPtrNone<*const T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const T) -> Self {
        debug_assert!(!ptr.is_null());
        MM::ref_(ptr as *mut _);
        Self {
            inner: ptr::NonNull::new_unchecked(ptr as *mut _),
            mm: PhantomData,
        }
    }
}

impl<T: 'static, MM: SharedMemoryManager<Target = T>> FromGlibPtrFull<*mut T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut T) -> Self {
        debug_assert!(!ptr.is_null());
        Self {
            inner: ptr::NonNull::new_unchecked(ptr),
            mm: PhantomData,
        }
    }
}

impl<T: 'static, MM: SharedMemoryManager<Target = T>> FromGlibPtrBorrow<*mut T> for Shared<T, MM> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut T) -> Borrowed<Self> {
        debug_assert!(!ptr.is_null());
        Borrowed::new(Self {
            inner: ptr::NonNull::new_unchecked(ptr),
            mm: PhantomData,
        })
    }
}
