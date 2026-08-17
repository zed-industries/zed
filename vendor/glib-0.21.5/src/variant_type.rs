// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::{Borrow, Cow},
    fmt,
    hash::{Hash, Hasher},
    iter,
    marker::PhantomData,
    ops::Deref,
    ptr, slice,
    str::FromStr,
};

use crate::{ffi, gobject_ffi, prelude::*, translate::*, BoolError, Type};

// rustdoc-stripper-ignore-next
/// Describes `Variant` types.
///
/// The `Variant` type system (based on the D-Bus one) describes types with
/// "type strings". `VariantType` is an owned immutable type string (you can
/// think of it as a `Box<str>` statically guaranteed to be a valid type
/// string), `&VariantTy` is a borrowed one (like `&str`).
#[doc(alias = "GVariantType")]
pub struct VariantType {
    // GVariantType* essentially is a char*, that always is valid UTF-8 but
    // isn't NUL-terminated.
    ptr: ptr::NonNull<ffi::GVariantType>,
    // We query the length on creation assuming it's cheap (because type strings
    // are short) and likely to happen anyway.
    len: usize,
}

impl VariantType {
    // rustdoc-stripper-ignore-next
    /// Tries to create a `VariantType` from a string slice.
    ///
    /// Returns `Ok` if the string is a valid type string, `Err` otherwise.
    pub fn new(type_string: &str) -> Result<VariantType, BoolError> {
        VariantTy::new(type_string).map(ToOwned::to_owned)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a `VariantType` from a key and value type.
    #[doc(alias = "g_variant_type_new_dict_entry")]
    pub fn new_dict_entry(key_type: &VariantTy, value_type: &VariantTy) -> VariantType {
        unsafe {
            from_glib_full(ffi::g_variant_type_new_dict_entry(
                key_type.to_glib_none().0,
                value_type.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a `VariantType` from an array element type.
    #[doc(alias = "g_variant_type_new_array")]
    pub fn new_array(elem_type: &VariantTy) -> VariantType {
        unsafe { from_glib_full(ffi::g_variant_type_new_array(elem_type.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a `VariantType` from a maybe element type.
    #[doc(alias = "g_variant_type_new_maybe")]
    pub fn new_maybe(child_type: &VariantTy) -> VariantType {
        unsafe { from_glib_full(ffi::g_variant_type_new_maybe(child_type.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a `VariantType` from a maybe element type.
    #[doc(alias = "g_variant_type_new_tuple")]
    pub fn new_tuple(items: impl IntoIterator<Item = impl AsRef<VariantTy>>) -> VariantType {
        let mut builder = crate::GStringBuilder::new("(");

        for ty in items {
            builder.append(ty.as_ref().as_str());
        }

        builder.append_c(')');

        VariantType::from_string(builder.into_string()).unwrap()
    }

    // rustdoc-stripper-ignore-next
    /// Tries to create a `VariantType` from an owned string.
    ///
    /// Returns `Ok` if the string is a valid type string, `Err` otherwise.
    pub fn from_string(type_string: impl Into<crate::GString>) -> Result<VariantType, BoolError> {
        let type_string = type_string.into();
        VariantTy::new(&type_string)?;

        let len = type_string.len();
        unsafe {
            let ptr = type_string.into_glib_ptr();

            Ok(VariantType {
                ptr: ptr::NonNull::new_unchecked(ptr as *mut ffi::GVariantType),
                len,
            })
        }
    }
}

unsafe impl Send for VariantType {}
unsafe impl Sync for VariantType {}

impl Drop for VariantType {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::g_variant_type_free(self.ptr.as_ptr()) }
    }
}

impl AsRef<VariantTy> for VariantType {
    #[inline]
    fn as_ref(&self) -> &VariantTy {
        self
    }
}

impl Borrow<VariantTy> for VariantType {
    #[inline]
    fn borrow(&self) -> &VariantTy {
        self
    }
}

impl Clone for VariantType {
    #[inline]
    fn clone(&self) -> VariantType {
        unsafe {
            VariantType {
                ptr: ptr::NonNull::new_unchecked(ffi::g_variant_type_copy(self.ptr.as_ptr())),
                len: self.len,
            }
        }
    }
}

impl Deref for VariantType {
    type Target = VariantTy;

    #[allow(clippy::cast_slice_from_raw_parts)]
    #[inline]
    fn deref(&self) -> &VariantTy {
        unsafe {
            &*(slice::from_raw_parts(self.ptr.as_ptr() as *const u8, self.len) as *const [u8]
                as *const VariantTy)
        }
    }
}

impl fmt::Debug for VariantType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <VariantTy as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for VariantType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for VariantType {
    type Err = BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Hash for VariantType {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        <VariantTy as Hash>::hash(self, state)
    }
}

impl<'a> From<VariantType> for Cow<'a, VariantTy> {
    #[inline]
    fn from(ty: VariantType) -> Cow<'a, VariantTy> {
        Cow::Owned(ty)
    }
}

#[doc(hidden)]
impl IntoGlibPtr<*mut ffi::GVariantType> for VariantType {
    #[inline]
    fn into_glib_ptr(self) -> *mut ffi::GVariantType {
        std::mem::ManuallyDrop::new(self).to_glib_none().0
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GVariantType> for VariantType {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GVariantType, Self> {
        Stash(self.ptr.as_ptr(), PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *const ffi::GVariantType {
        unsafe { ffi::g_variant_type_copy(self.ptr.as_ptr()) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::GVariantType> for VariantType {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GVariantType, Self> {
        Stash(self.ptr.as_ptr(), PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::GVariantType {
        unsafe { ffi::g_variant_type_copy(self.ptr.as_ptr()) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GVariantType> for VariantType {
    type Storage = PhantomData<&'a mut Self>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GVariantType, Self> {
        StashMut(self.ptr.as_ptr(), PhantomData)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GVariantType> for VariantType {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GVariantType) -> VariantType {
        VariantTy::from_ptr(ptr).to_owned()
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::GVariantType> for VariantType {
    #[inline]
    unsafe fn from_glib_full(ptr: *const ffi::GVariantType) -> VariantType {
        // Don't assume ownership of a const pointer.
        // A transfer: full annotation on a `const GVariantType*` is likely a bug.
        VariantTy::from_ptr(ptr).to_owned()
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GVariantType> for VariantType {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GVariantType) -> VariantType {
        debug_assert!(!ptr.is_null());
        let len: usize = ffi::g_variant_type_get_string_length(ptr) as _;
        VariantType {
            ptr: ptr::NonNull::new_unchecked(ptr),
            len,
        }
    }
}

// rustdoc-stripper-ignore-next
/// Describes `Variant` types.
///
/// This is a borrowed counterpart of [`VariantType`](struct.VariantType.html).
/// Essentially it's a `str` statically guaranteed to be a valid type string.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VariantTy {
    inner: str,
}

impl VariantTy {
    // rustdoc-stripper-ignore-next
    /// `bool`.
    #[doc(alias = "G_VARIANT_TYPE_BOOLEAN")]
    pub const BOOLEAN: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_BOOLEAN) };

    // rustdoc-stripper-ignore-next
    /// `u8`.
    #[doc(alias = "G_VARIANT_TYPE_BYTE")]
    pub const BYTE: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_BYTE) };

    // rustdoc-stripper-ignore-next
    /// `i16`.
    #[doc(alias = "G_VARIANT_TYPE_INT16")]
    pub const INT16: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_INT16) };

    // rustdoc-stripper-ignore-next
    /// `u16`.
    #[doc(alias = "G_VARIANT_TYPE_UINT16")]
    pub const UINT16: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_UINT16) };

    // rustdoc-stripper-ignore-next
    /// `i32`.
    #[doc(alias = "G_VARIANT_TYPE_INT32")]
    pub const INT32: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_INT32) };

    // rustdoc-stripper-ignore-next
    /// `u32`.
    #[doc(alias = "G_VARIANT_TYPE_UINT32")]
    pub const UINT32: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_UINT32) };

    // rustdoc-stripper-ignore-next
    /// `i64`.
    #[doc(alias = "G_VARIANT_TYPE_INT64")]
    pub const INT64: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_INT64) };

    // rustdoc-stripper-ignore-next
    /// `u64`.
    #[doc(alias = "G_VARIANT_TYPE_UINT64")]
    pub const UINT64: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_UINT64) };

    // rustdoc-stripper-ignore-next
    /// `f64`.
    #[doc(alias = "G_VARIANT_TYPE_DOUBLE")]
    pub const DOUBLE: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_DOUBLE) };

    // rustdoc-stripper-ignore-next
    /// `&str`.
    #[doc(alias = "G_VARIANT_TYPE_STRING")]
    pub const STRING: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_STRING) };

    // rustdoc-stripper-ignore-next
    /// DBus object path.
    #[doc(alias = "G_VARIANT_TYPE_OBJECT_PATH")]
    pub const OBJECT_PATH: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_OBJECT_PATH) };

    // rustdoc-stripper-ignore-next
    /// Type signature.
    #[doc(alias = "G_VARIANT_TYPE_SIGNATURE")]
    pub const SIGNATURE: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_SIGNATURE) };

    // rustdoc-stripper-ignore-next
    /// Variant.
    #[doc(alias = "G_VARIANT_TYPE_VARIANT")]
    pub const VARIANT: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_VARIANT) };

    // rustdoc-stripper-ignore-next
    /// Handle.
    #[doc(alias = "G_VARIANT_TYPE_HANDLE")]
    pub const HANDLE: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_HANDLE) };

    // rustdoc-stripper-ignore-next
    /// Unit, i.e. `()`.
    #[doc(alias = "G_VARIANT_TYPE_UNIT")]
    pub const UNIT: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_UNIT) };

    // rustdoc-stripper-ignore-next
    /// An indefinite type that is a supertype of every type (including itself).
    #[doc(alias = "G_VARIANT_TYPE_ANY")]
    pub const ANY: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_ANY) };

    // rustdoc-stripper-ignore-next
    /// Any basic type.
    #[doc(alias = "G_VARIANT_TYPE_BASIC")]
    pub const BASIC: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_BASIC) };

    // rustdoc-stripper-ignore-next
    /// Any maybe type, i.e. `Option<T>`.
    #[doc(alias = "G_VARIANT_TYPE_MAYBE")]
    pub const MAYBE: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_MAYBE) };

    // rustdoc-stripper-ignore-next
    /// Any array type, i.e. `[T]`.
    #[doc(alias = "G_VARIANT_TYPE_ARRAY")]
    pub const ARRAY: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_ARRAY) };

    // rustdoc-stripper-ignore-next
    /// Any tuple type, i.e. `(T)`, `(T, T)`, etc.
    #[doc(alias = "G_VARIANT_TYPE_TUPLE")]
    pub const TUPLE: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_TUPLE) };

    // rustdoc-stripper-ignore-next
    /// Any dict entry type, i.e. `DictEntry<K, V>`.
    #[doc(alias = "G_VARIANT_TYPE_DICT_ENTRY")]
    pub const DICT_ENTRY: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_DICT_ENTRY) };

    // rustdoc-stripper-ignore-next
    /// Any dictionary type, i.e. `HashMap<K, V>`, `BTreeMap<K, V>`.
    #[doc(alias = "G_VARIANT_TYPE_DICTIONARY")]
    pub const DICTIONARY: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_DICTIONARY) };

    // rustdoc-stripper-ignore-next
    /// String array, i.e. `[&str]`.
    #[doc(alias = "G_VARIANT_TYPE_STRING_ARRAY")]
    pub const STRING_ARRAY: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_STRING_ARRAY) };

    // rustdoc-stripper-ignore-next
    /// Object path array, i.e. `[&str]`.
    #[doc(alias = "G_VARIANT_TYPE_OBJECT_PATH_ARRAY")]
    pub const OBJECT_PATH_ARRAY: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_OBJECT_PATH_ARRAY) };

    // rustdoc-stripper-ignore-next
    /// Byte string, i.e. `[u8]`.
    #[doc(alias = "G_VARIANT_TYPE_BYTE_STRING")]
    pub const BYTE_STRING: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_BYTE_STRING) };

    // rustdoc-stripper-ignore-next
    /// Byte string array, i.e. `[[u8]]`.
    #[doc(alias = "G_VARIANT_TYPE_BYTE_STRING_ARRAY")]
    pub const BYTE_STRING_ARRAY: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_BYTE_STRING_ARRAY) };

    // rustdoc-stripper-ignore-next
    /// Variant dictionary, i.e. `HashMap<String, Variant>`, `BTreeMap<String, Variant>`, etc.
    #[doc(alias = "G_VARIANT_TYPE_VARDICT")]
    pub const VARDICT: &'static VariantTy =
        unsafe { VariantTy::from_str_unchecked(ffi::G_VARIANT_TYPE_VARDICT) };

    // rustdoc-stripper-ignore-next
    /// Tries to create a `&VariantTy` from a string slice.
    ///
    /// Returns `Ok` if the string is a valid type string, `Err` otherwise.
    pub fn new(type_string: &str) -> Result<&VariantTy, BoolError> {
        unsafe {
            let ptr = type_string.as_ptr();
            let limit = ptr.add(type_string.len());
            let mut end = ptr::null();

            let ok = from_glib(ffi::g_variant_type_string_scan(
                ptr as *const _,
                limit as *const _,
                &mut end,
            ));
            if ok && end as *const _ == limit {
                Ok(&*(type_string.as_bytes() as *const [u8] as *const VariantTy))
            } else {
                Err(bool_error!("Invalid type string: '{}'", type_string))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Converts a type string into `&VariantTy` without any checks.
    ///
    /// # Safety
    ///
    /// The caller is responsible for passing in only a valid variant type string.
    #[inline]
    pub const unsafe fn from_str_unchecked(type_string: &str) -> &VariantTy {
        std::mem::transmute::<&str, &VariantTy>(type_string)
    }

    // rustdoc-stripper-ignore-next
    /// Creates `&VariantTy` with a wildcard lifetime from a `GVariantType`
    /// pointer.
    #[doc(hidden)]
    #[allow(clippy::cast_slice_from_raw_parts)]
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const ffi::GVariantType) -> &'a VariantTy {
        debug_assert!(!ptr.is_null());
        let len: usize = ffi::g_variant_type_get_string_length(ptr) as _;
        debug_assert!(len > 0);
        &*(slice::from_raw_parts(ptr as *const u8, len) as *const [u8] as *const VariantTy)
    }

    // rustdoc-stripper-ignore-next
    /// Returns a `GVariantType` pointer.
    #[doc(hidden)]
    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GVariantType {
        self.inner.as_ptr() as *const _
    }

    // rustdoc-stripper-ignore-next
    /// Converts to a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is a definite type.
    #[doc(alias = "g_variant_type_is_definite")]
    pub fn is_definite(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_type_is_definite(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is a container type.
    #[doc(alias = "g_variant_type_is_container")]
    pub fn is_container(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_type_is_container(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is a basic type.
    #[doc(alias = "g_variant_type_is_basic")]
    pub fn is_basic(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_type_is_basic(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is a maybe type.
    #[doc(alias = "g_variant_type_is_maybe")]
    pub fn is_maybe(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_type_is_maybe(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is an array type.
    #[doc(alias = "g_variant_type_is_array")]
    pub fn is_array(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_type_is_array(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is a tuple type.
    #[doc(alias = "g_variant_type_is_tuple")]
    pub fn is_tuple(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_type_is_tuple(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is a dict entry type.
    #[doc(alias = "g_variant_type_is_dict_entry")]
    pub fn is_dict_entry(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_type_is_dict_entry(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is a variant.
    #[doc(alias = "g_variant_type_is_variant")]
    pub fn is_variant(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_type_is_variant(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if this variant type is a subtype of another.
    #[doc(alias = "g_variant_type_is_subtype_of")]
    pub fn is_subtype_of(&self, supertype: &Self) -> bool {
        unsafe {
            from_glib(ffi::g_variant_type_is_subtype_of(
                self.to_glib_none().0,
                supertype.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return the element type of this variant type.
    ///
    /// # Panics
    ///
    /// This function panics if not called with an array or maybe type.
    #[doc(alias = "g_variant_type_element")]
    pub fn element(&self) -> &VariantTy {
        assert!(self.is_array() || self.is_maybe());

        unsafe {
            let element = ffi::g_variant_type_element(self.to_glib_none().0);
            Self::from_ptr(element)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Iterate over the types of this variant type.
    ///
    /// # Panics
    ///
    /// This function panics if not called with a tuple or dictionary entry type.
    pub fn tuple_types(&self) -> VariantTyIterator<'_> {
        VariantTyIterator::new(self).expect("VariantTy does not represent a tuple")
    }

    // rustdoc-stripper-ignore-next
    /// Return the first type of this variant type.
    ///
    /// # Panics
    ///
    /// This function panics if not called with a tuple or dictionary entry type.
    #[doc(alias = "g_variant_type_first")]
    pub fn first(&self) -> Option<&VariantTy> {
        assert!(self.as_str().starts_with('(') || self.as_str().starts_with('{'));

        unsafe {
            let first = ffi::g_variant_type_first(self.to_glib_none().0);
            if first.is_null() {
                None
            } else {
                Some(Self::from_ptr(first))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return the next type of this variant type.
    #[doc(alias = "g_variant_type_next")]
    pub fn next(&self) -> Option<&VariantTy> {
        unsafe {
            let next = ffi::g_variant_type_next(self.to_glib_none().0);
            if next.is_null() {
                None
            } else {
                Some(Self::from_ptr(next))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return the number of items in this variant type.
    #[doc(alias = "g_variant_type_n_items")]
    pub fn n_items(&self) -> usize {
        unsafe { ffi::g_variant_type_n_items(self.to_glib_none().0) }
    }

    // rustdoc-stripper-ignore-next
    /// Return the key type of this variant type.
    ///
    /// # Panics
    ///
    /// This function panics if not called with a dictionary entry type.
    #[doc(alias = "g_variant_type_key")]
    pub fn key(&self) -> &VariantTy {
        assert!(self.as_str().starts_with('{'));

        unsafe {
            let key = ffi::g_variant_type_key(self.to_glib_none().0);
            Self::from_ptr(key)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return the value type of this variant type.
    ///
    /// # Panics
    ///
    /// This function panics if not called with a dictionary entry type.
    #[doc(alias = "g_variant_type_value")]
    pub fn value(&self) -> &VariantTy {
        assert!(self.as_str().starts_with('{'));

        unsafe {
            let value = ffi::g_variant_type_value(self.to_glib_none().0);
            Self::from_ptr(value)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return this type as an array.
    pub(crate) fn as_array<'a>(&self) -> Cow<'a, VariantTy> {
        if self == VariantTy::STRING {
            Cow::Borrowed(VariantTy::STRING_ARRAY)
        } else if self == VariantTy::BYTE {
            Cow::Borrowed(VariantTy::BYTE_STRING)
        } else if self == VariantTy::BYTE_STRING {
            Cow::Borrowed(VariantTy::BYTE_STRING_ARRAY)
        } else if self == VariantTy::OBJECT_PATH {
            Cow::Borrowed(VariantTy::OBJECT_PATH_ARRAY)
        } else if self == VariantTy::DICT_ENTRY {
            Cow::Borrowed(VariantTy::DICTIONARY)
        } else {
            Cow::Owned(VariantType::new_array(self))
        }
    }
}

unsafe impl Sync for VariantTy {}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GVariantType> for VariantTy {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GVariantType, Self> {
        Stash(self.as_ptr(), PhantomData)
    }
}

impl fmt::Display for VariantTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> From<&'a VariantTy> for Cow<'a, VariantTy> {
    #[inline]
    fn from(ty: &'a VariantTy) -> Cow<'a, VariantTy> {
        Cow::Borrowed(ty)
    }
}

impl AsRef<VariantTy> for VariantTy {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ToOwned for VariantTy {
    type Owned = VariantType;

    #[inline]
    fn to_owned(&self) -> VariantType {
        unsafe {
            VariantType {
                ptr: ptr::NonNull::new_unchecked(ffi::g_variant_type_copy(self.as_ptr())),
                len: self.inner.len(),
            }
        }
    }
}

impl StaticType for VariantTy {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_variant_type_get_gtype()) }
    }
}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for &'a VariantTy {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0);
        debug_assert!(!ptr.is_null());
        VariantTy::from_ptr(ptr as *const ffi::GVariantType)
    }
}

#[doc(hidden)]
impl crate::value::ToValue for VariantTy {
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(VariantTy::static_type());
            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            );
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        VariantTy::static_type()
    }
}

#[doc(hidden)]
impl crate::value::ToValue for &VariantTy {
    fn to_value(&self) -> crate::Value {
        (*self).to_value()
    }

    #[inline]
    fn value_type(&self) -> crate::Type {
        VariantTy::static_type()
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for &VariantTy {
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        let mut value = crate::Value::for_value_type::<VariantType>();
        unsafe {
            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            );
        }

        value
    }
}

impl StaticType for VariantType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_variant_type_get_gtype()) }
    }
}

#[doc(hidden)]
impl crate::value::ValueType for VariantType {
    type Type = VariantType;
}

#[doc(hidden)]
impl crate::value::ValueTypeOptional for VariantType {}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for VariantType {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0);
        debug_assert!(!ptr.is_null());
        from_glib_none(ptr as *const ffi::GVariantType)
    }
}

#[doc(hidden)]
impl crate::value::ToValue for VariantType {
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(VariantType::static_type());
            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                ToGlibPtr::<*mut _>::to_glib_none(&self).0 as *mut _,
            );
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        VariantType::static_type()
    }
}

#[doc(hidden)]
impl From<VariantType> for crate::Value {
    fn from(t: VariantType) -> Self {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(VariantType::static_type());
            gobject_ffi::g_value_take_boxed(
                value.to_glib_none_mut().0,
                IntoGlibPtr::<*mut _>::into_glib_ptr(t) as *mut _,
            );
            value
        }
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for VariantType {
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                ToGlibPtr::<*mut _>::to_glib_none(&s).0 as *mut _,
            );
        }

        value
    }
}

impl PartialEq for VariantType {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        <VariantTy as PartialEq>::eq(self, other)
    }
}

macro_rules! impl_eq {
    ($lhs:ty, $rhs: ty) => {
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <VariantTy as PartialEq>::eq(self, other)
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <VariantTy as PartialEq>::eq(self, other)
            }
        }
    };
}

impl_eq!(VariantType, VariantTy);
impl_eq!(VariantType, &'a VariantTy);
impl_eq!(VariantType, Cow<'a, VariantTy>);
impl_eq!(&'a VariantTy, Cow<'b, VariantTy>);

macro_rules! impl_str_eq {
    ($lhs:ty, $rhs: ty) => {
        #[allow(clippy::redundant_slicing)]
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                self.as_str().eq(&other[..])
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                self[..].eq(other.as_str())
            }
        }
    };
}

impl_str_eq!(VariantTy, str);
impl_str_eq!(VariantTy, &'a str);
impl_str_eq!(&'a VariantTy, str);
impl_str_eq!(VariantTy, String);
impl_str_eq!(&'a VariantTy, String);
impl_str_eq!(VariantType, str);
impl_str_eq!(VariantType, &'a str);
impl_str_eq!(VariantType, String);

impl Eq for VariantType {}

// rustdoc-stripper-ignore-next
/// An iterator over the individual components of a tuple [VariantTy].
///
/// This can be conveniently constructed using [VariantTy::tuple_types].
#[derive(Debug, Copy, Clone)]
pub struct VariantTyIterator<'a> {
    elem: Option<&'a VariantTy>,
}

impl<'a> VariantTyIterator<'a> {
    // rustdoc-stripper-ignore-next
    /// Creates a new iterator over the types of the specified [VariantTy].
    ///
    /// Returns `Ok` if the type is a definite tuple or dictionary entry type,
    /// `Err` otherwise.
    pub fn new(ty: &'a VariantTy) -> Result<Self, BoolError> {
        if (ty.is_tuple() && ty != VariantTy::TUPLE) || ty.is_dict_entry() {
            Ok(Self { elem: ty.first() })
        } else {
            Err(bool_error!(
                "Expected a definite tuple or dictionary entry type"
            ))
        }
    }
}

impl<'a> Iterator for VariantTyIterator<'a> {
    type Item = &'a VariantTy;

    #[doc(alias = "g_variant_type_next")]
    fn next(&mut self) -> Option<Self::Item> {
        let elem = self.elem?;
        self.elem = elem.next();
        Some(elem)
    }
}

impl iter::FusedIterator for VariantTyIterator<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn equal<T, U>(ptr1: *const T, ptr2: *const U) -> bool {
        from_glib(ffi::g_variant_type_equal(
            ptr1 as *const _,
            ptr2 as *const _,
        ))
    }

    #[test]
    fn new() {
        let ty = VariantTy::new("((iii)s)").unwrap();
        unsafe {
            assert!(equal(ty.as_ptr(), b"((iii)s)\0" as *const u8));
        }
    }

    #[test]
    fn new_empty() {
        assert!(VariantTy::new("").is_err());
    }

    #[test]
    fn new_with_nul() {
        assert!(VariantTy::new("((iii\0)s)").is_err());
    }

    #[test]
    fn new_too_short() {
        assert!(VariantTy::new("((iii").is_err());
    }

    #[test]
    fn new_too_long() {
        assert!(VariantTy::new("(iii)s").is_err());
    }

    #[test]
    fn eq() {
        let ty1 = VariantTy::new("((iii)s)").unwrap();
        let ty2 = VariantTy::new("((iii)s)").unwrap();
        assert_eq!(ty1, ty2);
        assert_eq!(ty1, "((iii)s)");
        unsafe {
            assert!(equal(ty1.as_ptr(), ty2.as_ptr()));
        }
    }

    #[test]
    fn ne() {
        let ty1 = VariantTy::new("((iii)s)").unwrap();
        let ty2 = VariantTy::new("((iii)o)").unwrap();
        assert_ne!(ty1, ty2);
        assert_ne!(ty1, "((iii)o)");
        unsafe {
            assert!(!equal(ty1.as_ptr(), ty2.as_ptr()));
        }
    }

    #[test]
    fn from_bytes() {
        unsafe {
            let ty = VariantTy::from_ptr(b"((iii)s)" as *const u8 as *const _);
            assert_eq!(ty, "((iii)s)");
            assert!(equal(ty.as_ptr(), "((iii)s)".as_ptr()));
        }
    }

    #[test]
    fn to_owned() {
        let ty1 = VariantTy::new("((iii)s)").unwrap();
        let ty2 = ty1.to_owned();
        assert_eq!(ty1, ty2);
        assert_eq!(ty2, "((iii)s)");
        unsafe {
            assert!(equal(ty1.as_ptr(), ty2.as_ptr()));
        }
    }

    #[test]
    fn value() {
        let ty1 = VariantType::new("*").unwrap();
        let tyv = ty1.to_value();
        let ty2 = tyv.get::<VariantType>().unwrap();
        assert_eq!(ty1, ty2);

        let ty3 = VariantTy::new("*").unwrap();
        let tyv2 = ty3.to_value();
        let ty4 = tyv2.get::<VariantType>().unwrap();
        assert_eq!(ty3, ty4);

        let ty5 = VariantTy::ANY;
        let tyv3 = ty5.to_value();
        let ty6 = tyv3.get::<VariantType>().unwrap();
        assert_eq!(ty5, ty6);
    }

    #[test]
    fn type_() {
        assert_eq!(VariantTy::static_type(), VariantType::static_type())
    }

    #[test]
    fn tuple_iter() {
        let ty = VariantTy::new("((iii)s)").unwrap();
        let types: Vec<_> = ty.tuple_types().map(|t| t.as_str()).collect();
        assert_eq!(&types, &["(iii)", "s"]);
    }
}
