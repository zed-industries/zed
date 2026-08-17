// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, prelude::*, translate::*, GStr, Regex};
use std::{marker::PhantomData, mem, ptr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MatchInfo<'input> {
    inner: std::ptr::NonNull<ffi::GMatchInfo>,
    _phantom: PhantomData<&'input GStr>,
}

impl Clone for MatchInfo<'_> {
    fn clone(&self) -> Self {
        unsafe {
            ffi::g_match_info_ref(self.inner.as_ptr());
        }
        Self {
            inner: self.inner,
            _phantom: PhantomData,
        }
    }
}

impl Drop for MatchInfo<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::g_match_info_unref(self.inner.as_ptr());
        }
    }
}

impl MatchInfo<'_> {
    #[doc = "Return the inner pointer to the underlying C value."]
    #[inline]
    pub fn as_ptr(&self) -> *mut ffi::GMatchInfo {
        self.inner.as_ptr()
    }
    #[doc = "Borrows the underlying C value."]
    #[inline]
    pub unsafe fn from_glib_ptr_borrow(ptr: &*mut ffi::GMatchInfo) -> &Self {
        debug_assert_eq!(
            std::mem::size_of::<Self>(),
            std::mem::size_of::<crate::ffi::gpointer>()
        );
        debug_assert!(!ptr.is_null());
        &*(ptr as *const *mut ffi::GMatchInfo as *const Self)
    }
}

#[doc(hidden)]
impl GlibPtrDefault for MatchInfo<'_> {
    type GlibType = *mut ffi::GMatchInfo;
}
#[doc(hidden)]
unsafe impl TransparentPtrType for MatchInfo<'_> {}

#[doc(hidden)]
impl<'a, 'input> ToGlibPtr<'a, *mut ffi::GMatchInfo> for MatchInfo<'input>
where
    'input: 'a,
{
    type Storage = PhantomData<&'a Self>;
    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GMatchInfo, Self> {
        Stash(self.inner.as_ptr(), PhantomData)
    }
    #[inline]
    fn to_glib_full(&self) -> *mut ffi::GMatchInfo {
        let ptr = self.inner.as_ptr();
        unsafe {
            ffi::g_match_info_ref(ptr);
        }
        ptr
    }
}
#[doc(hidden)]
impl<'a, 'input> ToGlibPtr<'a, *const ffi::GMatchInfo> for MatchInfo<'input>
where
    'input: 'a,
{
    type Storage = PhantomData<&'a Self>;
    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GMatchInfo, Self> {
        Stash(self.inner.as_ptr(), PhantomData)
    }
    #[inline]
    fn to_glib_full(&self) -> *const ffi::GMatchInfo {
        let ptr = self.inner.as_ptr();
        unsafe {
            ffi::g_match_info_ref(ptr);
        }
        ptr
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GMatchInfo> for MatchInfo<'_> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GMatchInfo) -> Self {
        debug_assert!(!ptr.is_null());
        unsafe {
            ffi::g_match_info_ref(ptr);
            Self {
                inner: ptr::NonNull::new_unchecked(ptr),
                _phantom: PhantomData,
            }
        }
    }
}
#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GMatchInfo> for MatchInfo<'_> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GMatchInfo) -> Self {
        Self::from_glib_none(ptr.cast_mut())
    }
}
#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GMatchInfo> for MatchInfo<'_> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GMatchInfo) -> Self {
        debug_assert!(!ptr.is_null());
        unsafe {
            Self {
                inner: ptr::NonNull::new_unchecked(ptr),
                _phantom: PhantomData,
            }
        }
    }
}
#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GMatchInfo> for MatchInfo<'_> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GMatchInfo) -> Borrowed<Self> {
        debug_assert!(!ptr.is_null());
        unsafe {
            Borrowed::new(Self {
                inner: ptr::NonNull::new_unchecked(ptr),
                _phantom: PhantomData,
            })
        }
    }
}
#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::GMatchInfo> for MatchInfo<'_> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const ffi::GMatchInfo) -> Borrowed<Self> {
        from_glib_borrow::<_, Self>(ptr.cast_mut())
    }
}

#[doc(hidden)]
impl IntoGlibPtr<*mut ffi::GMatchInfo> for MatchInfo<'_> {
    #[inline]
    fn into_glib_ptr(self) -> *mut ffi::GMatchInfo {
        let s = std::mem::ManuallyDrop::new(self);
        ToGlibPtr::<*const ffi::GMatchInfo>::to_glib_none(&*s).0 as *mut _
    }
}
#[doc(hidden)]
impl IntoGlibPtr<*const ffi::GMatchInfo> for MatchInfo<'_> {
    #[inline]
    fn into_glib_ptr(self) -> *const ffi::GMatchInfo {
        let s = std::mem::ManuallyDrop::new(self);
        ToGlibPtr::<*const ffi::GMatchInfo>::to_glib_none(&*s).0 as *const _
    }
}
impl StaticType for MatchInfo<'_> {
    #[inline]
    fn static_type() -> crate::types::Type {
        unsafe { from_glib(ffi::g_match_info_get_type()) }
    }
}

#[doc(hidden)]
impl ValueType for MatchInfo<'static> {
    type Type = Self;
}

#[doc(hidden)]
impl crate::value::ValueTypeOptional for MatchInfo<'static> {}

unsafe impl<'a, 'input: 'a> crate::value::FromValue<'a> for MatchInfo<'input> {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        let ptr = crate::gobject_ffi::g_value_dup_boxed(
            crate::translate::ToGlibPtr::to_glib_none(value).0,
        );
        debug_assert!(!ptr.is_null());
        <Self as crate::translate::FromGlibPtrFull<*mut ffi::GMatchInfo>>::from_glib_full(
            ptr as *mut ffi::GMatchInfo,
        )
    }
}
#[doc(hidden)]
unsafe impl<'a, 'input: 'a> crate::value::FromValue<'a> for &'a MatchInfo<'input> {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a crate::Value) -> Self {
        let value = &*(value as *const crate::Value as *const crate::gobject_ffi::GValue);
        <MatchInfo<'input>>::from_glib_ptr_borrow(
            &*(&value.data[0].v_pointer as *const crate::ffi::gpointer
                as *const *mut ffi::GMatchInfo),
        )
    }
}
impl ToValue for MatchInfo<'static> {
    #[inline]
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(<Self as StaticType>::static_type());
            crate::gobject_ffi::g_value_take_boxed(
                crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                crate::translate::ToGlibPtr::<*mut ffi::GMatchInfo>::to_glib_full(self) as *mut _,
            );
            value
        }
    }

    #[inline]
    fn value_type(&self) -> crate::Type {
        <Self as StaticType>::static_type()
    }
}

impl From<MatchInfo<'static>> for crate::Value {
    #[inline]
    fn from(s: MatchInfo<'static>) -> Self {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(
                <MatchInfo<'static> as StaticType>::static_type(),
            );
            crate::gobject_ffi::g_value_take_boxed(
                crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                crate::translate::IntoGlibPtr::<*mut ffi::GMatchInfo>::into_glib_ptr(s) as *mut _,
            );
            value
        }
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for MatchInfo<'static> {
    #[inline]
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            crate::gobject_ffi::g_value_take_boxed(
                crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                crate::translate::ToGlibPtr::<*mut ffi::GMatchInfo>::to_glib_full(&s) as *mut _,
            );
        }

        value
    }
}

impl HasParamSpec for MatchInfo<'static> {
    type ParamSpec = crate::ParamSpecBoxed;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> crate::ParamSpecBoxedBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl<'input> MatchInfo<'input> {
    #[doc(alias = "g_match_info_fetch")]
    pub fn fetch(&self, match_num: i32) -> Option<crate::GString> {
        unsafe { from_glib_full(ffi::g_match_info_fetch(self.to_glib_none().0, match_num)) }
    }

    #[doc(alias = "g_match_info_fetch_all")]
    pub fn fetch_all(&self) -> Vec<crate::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_match_info_fetch_all(self.to_glib_none().0))
        }
    }

    #[doc(alias = "g_match_info_fetch_pos")]
    pub fn fetch_pos(&self, match_num: i32) -> Option<(i32, i32)> {
        unsafe {
            let mut start_pos = std::mem::MaybeUninit::uninit();
            let mut end_pos = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::g_match_info_fetch_pos(
                self.to_glib_none().0,
                match_num,
                start_pos.as_mut_ptr(),
                end_pos.as_mut_ptr(),
            ));
            if ret {
                Some((start_pos.assume_init(), end_pos.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "g_match_info_get_match_count")]
    #[doc(alias = "get_match_count")]
    pub fn match_count(&self) -> i32 {
        unsafe { ffi::g_match_info_get_match_count(self.to_glib_none().0) }
    }

    #[doc(alias = "g_match_info_get_regex")]
    #[doc(alias = "get_regex")]
    pub fn regex(&self) -> Regex {
        unsafe { from_glib_none(ffi::g_match_info_get_regex(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_match_info_get_string")]
    #[doc(alias = "get_string")]
    pub fn string(&self) -> &'input crate::GStr {
        unsafe { from_glib_none(ffi::g_match_info_get_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_match_info_is_partial_match")]
    pub fn is_partial_match(&self) -> bool {
        unsafe { from_glib(ffi::g_match_info_is_partial_match(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_match_info_matches")]
    pub fn matches(&self) -> bool {
        unsafe { from_glib(ffi::g_match_info_matches(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_match_info_next")]
    pub fn next(&self) -> Result<bool, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_match_info_next(self.to_glib_none().0, &mut error);
            if !error.is_null() {
                Err(from_glib_full(error))
            } else {
                Ok(from_glib(is_ok))
            }
        }
    }

    #[doc(alias = "g_match_info_expand_references")]
    pub fn expand_references(
        &self,
        string_to_expand: impl IntoGStr,
    ) -> Result<Option<crate::GString>, crate::Error> {
        string_to_expand.run_with_gstr(|string_to_expand| unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_match_info_expand_references(
                self.to_glib_none().0,
                string_to_expand.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        })
    }

    #[doc(alias = "g_match_info_fetch_named")]
    pub fn fetch_named(&self, name: impl IntoGStr) -> Option<crate::GString> {
        name.run_with_gstr(|name| unsafe {
            from_glib_full(ffi::g_match_info_fetch_named(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        })
    }

    #[doc(alias = "g_match_info_fetch_named_pos")]
    pub fn fetch_named_pos(&self, name: impl IntoGStr) -> Option<(i32, i32)> {
        name.run_with_gstr(|name| unsafe {
            let mut start_pos = mem::MaybeUninit::uninit();
            let mut end_pos = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::g_match_info_fetch_named_pos(
                self.to_glib_none().0,
                name.to_glib_none().0,
                start_pos.as_mut_ptr(),
                end_pos.as_mut_ptr(),
            ));
            if ret {
                Some((start_pos.assume_init(), end_pos.assume_init()))
            } else {
                None
            }
        })
    }
}
