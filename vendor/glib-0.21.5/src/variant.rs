// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `Variant` binding and helper traits.
//!
//! [`Variant`](struct.Variant.html) is an immutable dynamically-typed generic
//! container. Its type and value are defined at construction and never change.
//!
//! `Variant` types are described by [`VariantType`](../struct.VariantType.html)
//! "type strings".
//!
//! `GVariant` supports arbitrarily complex types built from primitives like integers, floating point
//! numbers, strings, arrays, tuples and dictionaries. See [`ToVariant#foreign-impls`] for
//! a full list of supported types. You may also implement [`ToVariant`] and [`FromVariant`]
//! manually, or derive them using the [`Variant`](derive@crate::Variant) derive macro.
//!
//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::variant::{Variant, FromVariant};
//! use std::collections::HashMap;
//!
//! // Using the `ToVariant` trait.
//! let num = 10.to_variant();
//!
//! // `is` tests the type of the value.
//! assert!(num.is::<i32>());
//!
//! // `get` tries to extract the value.
//! assert_eq!(num.get::<i32>(), Some(10));
//! assert_eq!(num.get::<u32>(), None);
//!
//! // `get_str` tries to borrow a string slice.
//! let hello = "Hello!".to_variant();
//! assert_eq!(hello.str(), Some("Hello!"));
//! assert_eq!(num.str(), None);
//!
//! // `fixed_array` tries to borrow a fixed size array (u8, bool, i16, etc.),
//! // rather than creating a deep copy which would be expensive for
//! // nontrivially sized arrays of fixed size elements.
//! // The test data here is the zstd compression header, which
//! // stands in for arbitrary binary data (e.g. not UTF-8).
//! let bufdata = b"\xFD\x2F\xB5\x28";
//! let bufv = glib::Variant::array_from_fixed_array(&bufdata[..]);
//! assert_eq!(bufv.fixed_array::<u8>().unwrap(), bufdata);
//! assert!(num.fixed_array::<u8>().is_err());
//!
//! // Variant carrying a Variant
//! let variant = Variant::from_variant(&hello);
//! let variant = variant.as_variant().unwrap();
//! assert_eq!(variant.str(), Some("Hello!"));
//!
//! // Variant carrying an array
//! let array = ["Hello", "there!"];
//! let variant = array.into_iter().collect::<Variant>();
//! assert_eq!(variant.n_children(), 2);
//! assert_eq!(variant.child_value(0).str(), Some("Hello"));
//! assert_eq!(variant.child_value(1).str(), Some("there!"));
//!
//! // You can also convert from and to a Vec
//! let variant = vec!["Hello", "there!"].to_variant();
//! assert_eq!(variant.n_children(), 2);
//! let vec = <Vec<String>>::from_variant(&variant).unwrap();
//! assert_eq!(vec[0], "Hello");
//!
//! // Conversion to and from HashMap and BTreeMap is also possible
//! let mut map: HashMap<u16, &str> = HashMap::new();
//! map.insert(1, "hi");
//! map.insert(2, "there");
//! let variant = map.to_variant();
//! assert_eq!(variant.n_children(), 2);
//! let map: HashMap<u16, String> = HashMap::from_variant(&variant).unwrap();
//! assert_eq!(map[&1], "hi");
//! assert_eq!(map[&2], "there");
//!
//! // And conversion to and from tuples.
//! let variant = ("hello", 42u16, vec![ "there", "you" ],).to_variant();
//! assert_eq!(variant.n_children(), 3);
//! assert_eq!(variant.type_().as_str(), "(sqas)");
//! let tuple = <(String, u16, Vec<String>)>::from_variant(&variant).unwrap();
//! assert_eq!(tuple.0, "hello");
//! assert_eq!(tuple.1, 42);
//! assert_eq!(tuple.2, &[ "there", "you"]);
//!
//! // `Option` is supported as well, through maybe types
//! let variant = Some("hello").to_variant();
//! assert_eq!(variant.n_children(), 1);
//! let mut s = <Option<String>>::from_variant(&variant).unwrap();
//! assert_eq!(s.unwrap(), "hello");
//! s = None;
//! let variant = s.to_variant();
//! assert_eq!(variant.n_children(), 0);
//! let s = <Option<String>>::from_variant(&variant).unwrap();
//! assert!(s.is_none());
//!
//! // Paths may be converted, too. Please note the portability warning above!
//! use std::path::{Path, PathBuf};
//! let path = Path::new("foo/bar");
//! let path_variant = path.to_variant();
//! assert_eq!(PathBuf::from_variant(&path_variant).as_deref(), Some(path));
//! ```

use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fmt,
    fmt::Display,
    hash::{BuildHasher, Hash, Hasher},
    mem, ptr, slice, str,
};

use crate::{
    ffi, gobject_ffi, prelude::*, translate::*, Bytes, Type, VariantIter, VariantStrIter,
    VariantTy, VariantType,
};

wrapper! {
    // rustdoc-stripper-ignore-next
    /// A generic immutable value capable of carrying various types.
    ///
    /// See the [module documentation](index.html) for more details.
    #[doc(alias = "GVariant")]
    pub struct Variant(Shared<ffi::GVariant>);

    match fn {
        ref => |ptr| ffi::g_variant_ref_sink(ptr),
        unref => |ptr| ffi::g_variant_unref(ptr),
    }
}

impl StaticType for Variant {
    #[inline]
    fn static_type() -> Type {
        Type::VARIANT
    }
}

#[doc(hidden)]
impl crate::value::ValueType for Variant {
    type Type = Variant;
}

#[doc(hidden)]
impl crate::value::ValueTypeOptional for Variant {}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for Variant {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        let ptr = gobject_ffi::g_value_dup_variant(value.to_glib_none().0);
        debug_assert!(!ptr.is_null());
        from_glib_full(ptr)
    }
}

#[doc(hidden)]
impl crate::value::ToValue for Variant {
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(Variant::static_type());
            gobject_ffi::g_value_take_variant(value.to_glib_none_mut().0, self.to_glib_full());
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        Variant::static_type()
    }
}

#[doc(hidden)]
impl From<Variant> for crate::Value {
    #[inline]
    fn from(v: Variant) -> Self {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(Variant::static_type());
            gobject_ffi::g_value_take_variant(value.to_glib_none_mut().0, v.into_glib_ptr());
            value
        }
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for Variant {
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_take_variant(value.to_glib_none_mut().0, s.to_glib_full());
        }

        value
    }
}

// rustdoc-stripper-ignore-next
/// An error returned from the [`try_get`](struct.Variant.html#method.try_get) function
/// on a [`Variant`](struct.Variant.html) when the expected type does not match the actual type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VariantTypeMismatchError {
    pub actual: VariantType,
    pub expected: VariantType,
}

impl VariantTypeMismatchError {
    pub fn new(actual: VariantType, expected: VariantType) -> Self {
        Self { actual, expected }
    }
}

impl fmt::Display for VariantTypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type mismatch: Expected '{}' got '{}'",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for VariantTypeMismatchError {}

impl Variant {
    // rustdoc-stripper-ignore-next
    /// Returns the type of the value.
    #[doc(alias = "g_variant_get_type")]
    pub fn type_(&self) -> &VariantTy {
        unsafe { VariantTy::from_ptr(ffi::g_variant_get_type(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if the type of the value corresponds to `T`.
    #[inline]
    #[doc(alias = "g_variant_is_of_type")]
    pub fn is<T: StaticVariantType>(&self) -> bool {
        self.is_type(&T::static_variant_type())
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if the type of the value corresponds to `type_`.
    ///
    /// This is equivalent to [`self.type_().is_subtype_of(type_)`](VariantTy::is_subtype_of).
    #[inline]
    #[doc(alias = "g_variant_is_of_type")]
    pub fn is_type(&self, type_: &VariantTy) -> bool {
        unsafe {
            from_glib(ffi::g_variant_is_of_type(
                self.to_glib_none().0,
                type_.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the classification of the variant.
    #[doc(alias = "g_variant_classify")]
    pub fn classify(&self) -> crate::VariantClass {
        unsafe { from_glib(ffi::g_variant_classify(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Tries to extract a value of type `T`.
    ///
    /// Returns `Some` if `T` matches the variant's type.
    #[inline]
    pub fn get<T: FromVariant>(&self) -> Option<T> {
        T::from_variant(self)
    }

    // rustdoc-stripper-ignore-next
    /// Tries to extract a value of type `T`.
    pub fn try_get<T: FromVariant>(&self) -> Result<T, VariantTypeMismatchError> {
        self.get().ok_or_else(|| {
            VariantTypeMismatchError::new(
                self.type_().to_owned(),
                T::static_variant_type().into_owned(),
            )
        })
    }

    // rustdoc-stripper-ignore-next
    /// Boxes value.
    #[inline]
    pub fn from_variant(value: &Variant) -> Self {
        unsafe { from_glib_none(ffi::g_variant_new_variant(value.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Unboxes self.
    ///
    /// Returns `Some` if self contains a `Variant`.
    #[inline]
    #[doc(alias = "get_variant")]
    pub fn as_variant(&self) -> Option<Variant> {
        unsafe { from_glib_full(ffi::g_variant_get_variant(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Reads a child item out of a container `Variant` instance.
    ///
    /// # Panics
    ///
    /// * if `self` is not a container type.
    /// * if given `index` is larger than number of children.
    #[doc(alias = "get_child_value")]
    #[doc(alias = "g_variant_get_child_value")]
    #[must_use]
    pub fn child_value(&self, index: usize) -> Variant {
        assert!(self.is_container());
        assert!(index < self.n_children());

        unsafe { from_glib_full(ffi::g_variant_get_child_value(self.to_glib_none().0, index)) }
    }

    // rustdoc-stripper-ignore-next
    /// Try to read a child item out of a container `Variant` instance.
    ///
    /// It returns `None` if `self` is not a container type or if the given
    /// `index` is larger than number of children.
    pub fn try_child_value(&self, index: usize) -> Option<Variant> {
        if !(self.is_container() && index < self.n_children()) {
            return None;
        }

        let v =
            unsafe { from_glib_full(ffi::g_variant_get_child_value(self.to_glib_none().0, index)) };
        Some(v)
    }

    // rustdoc-stripper-ignore-next
    /// Try to read a child item out of a container `Variant` instance.
    ///
    /// It returns `Ok(None)` if `self` is not a container type or if the given
    /// `index` is larger than number of children.  An error is thrown if the
    /// type does not match.
    pub fn try_child_get<T: StaticVariantType + FromVariant>(
        &self,
        index: usize,
    ) -> Result<Option<T>, VariantTypeMismatchError> {
        // TODO: In the future optimize this by using g_variant_get_child()
        // directly to avoid allocating a GVariant.
        self.try_child_value(index).map(|v| v.try_get()).transpose()
    }

    // rustdoc-stripper-ignore-next
    /// Read a child item out of a container `Variant` instance.
    ///
    /// # Panics
    ///
    /// * if `self` is not a container type.
    /// * if given `index` is larger than number of children.
    /// * if the expected variant type does not match
    pub fn child_get<T: StaticVariantType + FromVariant>(&self, index: usize) -> T {
        // TODO: In the future optimize this by using g_variant_get_child()
        // directly to avoid allocating a GVariant.
        self.child_value(index).get().unwrap()
    }

    // rustdoc-stripper-ignore-next
    /// Tries to extract a `&str`.
    ///
    /// Returns `Some` if the variant has a string type (`s`, `o` or `g` type
    /// strings).
    #[doc(alias = "get_str")]
    #[doc(alias = "g_variant_get_string")]
    pub fn str(&self) -> Option<&str> {
        unsafe {
            match self.type_().as_str() {
                "s" | "o" | "g" => {
                    let mut len = 0;
                    let ptr = ffi::g_variant_get_string(self.to_glib_none().0, &mut len);
                    if len == 0 {
                        Some("")
                    } else {
                        let ret = str::from_utf8_unchecked(slice::from_raw_parts(
                            ptr as *const u8,
                            len as _,
                        ));
                        Some(ret)
                    }
                }
                _ => None,
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Tries to extract a `&[T]` from a variant of array type with a suitable element type.
    ///
    /// Returns an error if the type is wrong.
    #[doc(alias = "g_variant_get_fixed_array")]
    pub fn fixed_array<T: FixedSizeVariantType>(&self) -> Result<&[T], VariantTypeMismatchError> {
        unsafe {
            let expected_ty = T::static_variant_type().as_array();
            if self.type_() != expected_ty {
                return Err(VariantTypeMismatchError {
                    actual: self.type_().to_owned(),
                    expected: expected_ty.into_owned(),
                });
            }

            let mut n_elements = mem::MaybeUninit::uninit();
            let ptr = ffi::g_variant_get_fixed_array(
                self.to_glib_none().0,
                n_elements.as_mut_ptr(),
                mem::size_of::<T>(),
            );

            let n_elements = n_elements.assume_init();
            if n_elements == 0 {
                Ok(&[])
            } else {
                debug_assert!(!ptr.is_null());
                Ok(slice::from_raw_parts(ptr as *const T, n_elements))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new Variant array from children.
    ///
    /// # Panics
    ///
    /// This function panics if not all variants are of type `T`.
    #[doc(alias = "g_variant_new_array")]
    pub fn array_from_iter<T: StaticVariantType>(
        children: impl IntoIterator<Item = Variant>,
    ) -> Self {
        Self::array_from_iter_with_type(&T::static_variant_type(), children)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new Variant array from children with the specified type.
    ///
    /// # Panics
    ///
    /// This function panics if not all variants are of type `type_`.
    #[doc(alias = "g_variant_new_array")]
    pub fn array_from_iter_with_type(
        type_: &VariantTy,
        children: impl IntoIterator<Item = impl AsRef<Variant>>,
    ) -> Self {
        unsafe {
            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), type_.as_array().to_glib_none().0);
            let mut builder = builder.assume_init();
            for value in children.into_iter() {
                let value = value.as_ref();
                if ffi::g_variant_is_of_type(value.to_glib_none().0, type_.to_glib_none().0)
                    == ffi::GFALSE
                {
                    ffi::g_variant_builder_clear(&mut builder);
                    assert!(value.is_type(type_));
                }

                ffi::g_variant_builder_add_value(&mut builder, value.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new Variant array from a fixed array.
    #[doc(alias = "g_variant_new_fixed_array")]
    pub fn array_from_fixed_array<T: FixedSizeVariantType>(array: &[T]) -> Self {
        let type_ = T::static_variant_type();

        unsafe {
            from_glib_none(ffi::g_variant_new_fixed_array(
                type_.as_ptr(),
                array.as_ptr() as ffi::gconstpointer,
                array.len(),
                mem::size_of::<T>(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new Variant tuple from children.
    #[doc(alias = "g_variant_new_tuple")]
    pub fn tuple_from_iter(children: impl IntoIterator<Item = impl AsRef<Variant>>) -> Self {
        unsafe {
            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::TUPLE.to_glib_none().0);
            let mut builder = builder.assume_init();
            for value in children.into_iter() {
                ffi::g_variant_builder_add_value(&mut builder, value.as_ref().to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new dictionary entry Variant.
    ///
    /// [DictEntry] should be preferred over this when the types are known statically.
    #[doc(alias = "g_variant_new_dict_entry")]
    pub fn from_dict_entry(key: &Variant, value: &Variant) -> Self {
        unsafe {
            from_glib_none(ffi::g_variant_new_dict_entry(
                key.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new maybe Variant.
    #[doc(alias = "g_variant_new_maybe")]
    pub fn from_maybe<T: StaticVariantType>(child: Option<&Variant>) -> Self {
        let type_ = T::static_variant_type();
        match child {
            Some(child) => {
                assert_eq!(type_, child.type_());

                Self::from_some(child)
            }
            None => Self::from_none(&type_),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new maybe Variant from a child.
    #[doc(alias = "g_variant_new_maybe")]
    pub fn from_some(child: &Variant) -> Self {
        unsafe {
            from_glib_none(ffi::g_variant_new_maybe(
                ptr::null(),
                child.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new maybe Variant with Nothing.
    #[doc(alias = "g_variant_new_maybe")]
    pub fn from_none(type_: &VariantTy) -> Self {
        unsafe {
            from_glib_none(ffi::g_variant_new_maybe(
                type_.to_glib_none().0,
                ptr::null_mut(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Extract the value of a maybe Variant.
    ///
    /// Returns the child value, or `None` if the value is Nothing.
    ///
    /// # Panics
    ///
    /// Panics if the variant is not maybe-typed.
    #[inline]
    pub fn as_maybe(&self) -> Option<Variant> {
        assert!(self.type_().is_maybe());

        unsafe { from_glib_full(ffi::g_variant_get_maybe(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Pretty-print the contents of this variant in a human-readable form.
    ///
    /// A variant can be recreated from this output via [`Variant::parse`].
    #[doc(alias = "g_variant_print")]
    pub fn print(&self, type_annotate: bool) -> crate::GString {
        unsafe {
            from_glib_full(ffi::g_variant_print(
                self.to_glib_none().0,
                type_annotate.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Parses a GVariant from the text representation produced by [`print()`](Self::print).
    #[doc(alias = "g_variant_parse")]
    pub fn parse(type_: Option<&VariantTy>, text: &str) -> Result<Self, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let text = text.as_bytes().as_ptr_range();
            let variant = ffi::g_variant_parse(
                type_.to_glib_none().0,
                text.start as *const _,
                text.end as *const _,
                ptr::null_mut(),
                &mut error,
            );
            if variant.is_null() {
                debug_assert!(!error.is_null());
                Err(from_glib_full(error))
            } else {
                debug_assert!(error.is_null());
                Ok(from_glib_full(variant))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Constructs a new serialized-mode GVariant instance.
    #[doc(alias = "g_variant_new_from_bytes")]
    pub fn from_bytes<T: StaticVariantType>(bytes: &Bytes) -> Self {
        Variant::from_bytes_with_type(bytes, &T::static_variant_type())
    }

    // rustdoc-stripper-ignore-next
    /// Constructs a new serialized-mode GVariant instance.
    ///
    /// This is the same as `from_bytes`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_bytes_trusted<T: StaticVariantType>(bytes: &Bytes) -> Self {
        Variant::from_bytes_with_type_trusted(bytes, &T::static_variant_type())
    }

    // rustdoc-stripper-ignore-next
    /// Constructs a new serialized-mode GVariant instance.
    #[doc(alias = "g_variant_new_from_data")]
    pub fn from_data<T: StaticVariantType, A: AsRef<[u8]>>(data: A) -> Self {
        Variant::from_data_with_type(data, &T::static_variant_type())
    }

    // rustdoc-stripper-ignore-next
    /// Constructs a new serialized-mode GVariant instance.
    ///
    /// This is the same as `from_data`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_data_trusted<T: StaticVariantType, A: AsRef<[u8]>>(data: A) -> Self {
        Variant::from_data_with_type_trusted(data, &T::static_variant_type())
    }

    // rustdoc-stripper-ignore-next
    /// Constructs a new serialized-mode GVariant instance with a given type.
    #[doc(alias = "g_variant_new_from_bytes")]
    pub fn from_bytes_with_type(bytes: &Bytes, type_: &VariantTy) -> Self {
        unsafe {
            from_glib_none(ffi::g_variant_new_from_bytes(
                type_.as_ptr() as *const _,
                bytes.to_glib_none().0,
                false.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Constructs a new serialized-mode GVariant instance with a given type.
    ///
    /// This is the same as `from_bytes`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_bytes_with_type_trusted(bytes: &Bytes, type_: &VariantTy) -> Self {
        from_glib_none(ffi::g_variant_new_from_bytes(
            type_.as_ptr() as *const _,
            bytes.to_glib_none().0,
            true.into_glib(),
        ))
    }

    // rustdoc-stripper-ignore-next
    /// Constructs a new serialized-mode GVariant instance with a given type.
    #[doc(alias = "g_variant_new_from_data")]
    pub fn from_data_with_type<A: AsRef<[u8]>>(data: A, type_: &VariantTy) -> Self {
        unsafe {
            let data = Box::new(data);
            let (data_ptr, len) = {
                let data = (*data).as_ref();
                (data.as_ptr(), data.len())
            };

            unsafe extern "C" fn free_data<A: AsRef<[u8]>>(ptr: ffi::gpointer) {
                let _ = Box::from_raw(ptr as *mut A);
            }

            from_glib_none(ffi::g_variant_new_from_data(
                type_.as_ptr() as *const _,
                data_ptr as ffi::gconstpointer,
                len,
                false.into_glib(),
                Some(free_data::<A>),
                Box::into_raw(data) as ffi::gpointer,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Constructs a new serialized-mode GVariant instance with a given type.
    ///
    /// This is the same as `from_data`, except that checks on the passed
    /// data are skipped.
    ///
    /// You should not use this function on data from external sources.
    ///
    /// # Safety
    ///
    /// Since the data is not validated, this is potentially dangerous if called
    /// on bytes which are not guaranteed to have come from serialising another
    /// Variant.  The caller is responsible for ensuring bad data is not passed in.
    pub unsafe fn from_data_with_type_trusted<A: AsRef<[u8]>>(data: A, type_: &VariantTy) -> Self {
        let data = Box::new(data);
        let (data_ptr, len) = {
            let data = (*data).as_ref();
            (data.as_ptr(), data.len())
        };

        unsafe extern "C" fn free_data<A: AsRef<[u8]>>(ptr: ffi::gpointer) {
            let _ = Box::from_raw(ptr as *mut A);
        }

        from_glib_none(ffi::g_variant_new_from_data(
            type_.as_ptr() as *const _,
            data_ptr as ffi::gconstpointer,
            len,
            true.into_glib(),
            Some(free_data::<A>),
            Box::into_raw(data) as ffi::gpointer,
        ))
    }

    // rustdoc-stripper-ignore-next
    /// Returns the serialized form of a GVariant instance.
    #[doc(alias = "get_data_as_bytes")]
    #[doc(alias = "g_variant_get_data_as_bytes")]
    pub fn data_as_bytes(&self) -> Bytes {
        unsafe { from_glib_full(ffi::g_variant_get_data_as_bytes(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the serialized form of a GVariant instance.
    #[doc(alias = "g_variant_get_data")]
    pub fn data(&self) -> &[u8] {
        unsafe {
            let selfv = self.to_glib_none();
            let len = ffi::g_variant_get_size(selfv.0);
            if len == 0 {
                return &[];
            }
            let ptr = ffi::g_variant_get_data(selfv.0);
            slice::from_raw_parts(ptr as *const _, len as _)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the size of serialized form of a GVariant instance.
    #[doc(alias = "g_variant_get_size")]
    pub fn size(&self) -> usize {
        unsafe { ffi::g_variant_get_size(self.to_glib_none().0) }
    }

    // rustdoc-stripper-ignore-next
    /// Stores the serialized form of a GVariant instance into the given slice.
    ///
    /// The slice needs to be big enough.
    #[doc(alias = "g_variant_store")]
    pub fn store(&self, data: &mut [u8]) -> Result<usize, crate::BoolError> {
        unsafe {
            let size = ffi::g_variant_get_size(self.to_glib_none().0);
            if data.len() < size {
                return Err(bool_error!("Provided slice is too small"));
            }

            ffi::g_variant_store(self.to_glib_none().0, data.as_mut_ptr() as ffi::gpointer);

            Ok(size)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns a copy of the variant in normal form.
    #[doc(alias = "g_variant_get_normal_form")]
    #[must_use]
    pub fn normal_form(&self) -> Self {
        unsafe { from_glib_full(ffi::g_variant_get_normal_form(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Returns a copy of the variant in the opposite endianness.
    #[doc(alias = "g_variant_byteswap")]
    #[must_use]
    pub fn byteswap(&self) -> Self {
        unsafe { from_glib_full(ffi::g_variant_byteswap(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Determines the number of children in a container GVariant instance.
    #[doc(alias = "g_variant_n_children")]
    pub fn n_children(&self) -> usize {
        assert!(self.is_container());

        unsafe { ffi::g_variant_n_children(self.to_glib_none().0) }
    }

    // rustdoc-stripper-ignore-next
    /// Create an iterator over items in the variant.
    ///
    /// Note that this heap allocates a variant for each element,
    /// which can be particularly expensive for large arrays.
    pub fn iter(&self) -> VariantIter {
        assert!(self.is_container());

        VariantIter::new(self.clone())
    }

    // rustdoc-stripper-ignore-next
    /// Create an iterator over borrowed strings from a GVariant of type `as` (array of string).
    ///
    /// This will fail if the variant is not an array of with
    /// the expected child type.
    ///
    /// A benefit of this API over [`Self::iter()`] is that it
    /// minimizes allocation, and provides strongly typed access.
    ///
    /// ```
    /// # use glib::prelude::*;
    /// let strs = &["foo", "bar"];
    /// let strs_variant: glib::Variant = strs.to_variant();
    /// for s in strs_variant.array_iter_str()? {
    ///     println!("{}", s);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn array_iter_str(&self) -> Result<VariantStrIter<'_>, VariantTypeMismatchError> {
        let child_ty = String::static_variant_type();
        let actual_ty = self.type_();
        let expected_ty = child_ty.as_array();
        if actual_ty != expected_ty {
            return Err(VariantTypeMismatchError {
                actual: actual_ty.to_owned(),
                expected: expected_ty.into_owned(),
            });
        }

        Ok(VariantStrIter::new(self))
    }

    // rustdoc-stripper-ignore-next
    /// Return whether this Variant is a container type.
    #[doc(alias = "g_variant_is_container")]
    pub fn is_container(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_is_container(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Return whether this Variant is in normal form.
    #[doc(alias = "g_variant_is_normal_form")]
    pub fn is_normal_form(&self) -> bool {
        unsafe { from_glib(ffi::g_variant_is_normal_form(self.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Return whether input string is a valid `VariantClass::ObjectPath`.
    #[doc(alias = "g_variant_is_object_path")]
    pub fn is_object_path(string: &str) -> bool {
        unsafe { from_glib(ffi::g_variant_is_object_path(string.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Return whether input string is a valid `VariantClass::Signature`.
    #[doc(alias = "g_variant_is_signature")]
    pub fn is_signature(string: &str) -> bool {
        unsafe { from_glib(ffi::g_variant_is_signature(string.to_glib_none().0)) }
    }
}

unsafe impl Send for Variant {}
unsafe impl Sync for Variant {}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Variant")
            .field("ptr", &ToGlibPtr::<*const _>::to_glib_none(self).0)
            .field("type", &self.type_())
            .field("value", &self.to_string())
            .finish()
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.print(true))
    }
}

impl str::FromStr for Variant {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(None, s)
    }
}

impl PartialEq for Variant {
    #[doc(alias = "g_variant_equal")]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::g_variant_equal(
                ToGlibPtr::<*const _>::to_glib_none(self).0 as *const _,
                ToGlibPtr::<*const _>::to_glib_none(other).0 as *const _,
            ))
        }
    }
}

impl Eq for Variant {}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            if ffi::g_variant_classify(self.to_glib_none().0)
                != ffi::g_variant_classify(other.to_glib_none().0)
            {
                return None;
            }

            if self.is_container() {
                return None;
            }

            let res = ffi::g_variant_compare(
                ToGlibPtr::<*const _>::to_glib_none(self).0 as *const _,
                ToGlibPtr::<*const _>::to_glib_none(other).0 as *const _,
            );

            Some(res.cmp(&0))
        }
    }
}

impl Hash for Variant {
    #[doc(alias = "g_variant_hash")]
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            state.write_u32(ffi::g_variant_hash(
                ToGlibPtr::<*const _>::to_glib_none(self).0 as *const _,
            ))
        }
    }
}

impl AsRef<Variant> for Variant {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

// rustdoc-stripper-ignore-next
/// Converts to `Variant`.
pub trait ToVariant {
    // rustdoc-stripper-ignore-next
    /// Returns a `Variant` clone of `self`.
    fn to_variant(&self) -> Variant;
}

// rustdoc-stripper-ignore-next
/// Extracts a value.
pub trait FromVariant: Sized + StaticVariantType {
    // rustdoc-stripper-ignore-next
    /// Tries to extract a value.
    ///
    /// Returns `Some` if the variant's type matches `Self`.
    fn from_variant(variant: &Variant) -> Option<Self>;
}

// rustdoc-stripper-ignore-next
/// Returns `VariantType` of `Self`.
pub trait StaticVariantType {
    // rustdoc-stripper-ignore-next
    /// Returns the `VariantType` corresponding to `Self`.
    fn static_variant_type() -> Cow<'static, VariantTy>;
}

impl StaticVariantType for Variant {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::VARIANT)
    }
}

impl<T: ?Sized + ToVariant> ToVariant for &T {
    fn to_variant(&self) -> Variant {
        <T as ToVariant>::to_variant(self)
    }
}

impl<'a, T: Into<Variant> + Clone> From<&'a T> for Variant {
    #[inline]
    fn from(v: &'a T) -> Self {
        v.clone().into()
    }
}

impl<T: ?Sized + StaticVariantType> StaticVariantType for &T {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <T as StaticVariantType>::static_variant_type()
    }
}

macro_rules! impl_numeric {
    ($name:ty, $typ:expr, $new_fn:ident, $get_fn:ident) => {
        impl StaticVariantType for $name {
            fn static_variant_type() -> Cow<'static, VariantTy> {
                Cow::Borrowed($typ)
            }
        }

        impl ToVariant for $name {
            fn to_variant(&self) -> Variant {
                unsafe { from_glib_none(ffi::$new_fn(*self)) }
            }
        }

        impl From<$name> for Variant {
            #[inline]
            fn from(v: $name) -> Self {
                v.to_variant()
            }
        }

        impl FromVariant for $name {
            fn from_variant(variant: &Variant) -> Option<Self> {
                unsafe {
                    if variant.is::<Self>() {
                        Some(ffi::$get_fn(variant.to_glib_none().0))
                    } else {
                        None
                    }
                }
            }
        }
    };
}

impl_numeric!(u8, VariantTy::BYTE, g_variant_new_byte, g_variant_get_byte);
impl_numeric!(
    i16,
    VariantTy::INT16,
    g_variant_new_int16,
    g_variant_get_int16
);
impl_numeric!(
    u16,
    VariantTy::UINT16,
    g_variant_new_uint16,
    g_variant_get_uint16
);
impl_numeric!(
    i32,
    VariantTy::INT32,
    g_variant_new_int32,
    g_variant_get_int32
);
impl_numeric!(
    u32,
    VariantTy::UINT32,
    g_variant_new_uint32,
    g_variant_get_uint32
);
impl_numeric!(
    i64,
    VariantTy::INT64,
    g_variant_new_int64,
    g_variant_get_int64
);
impl_numeric!(
    u64,
    VariantTy::UINT64,
    g_variant_new_uint64,
    g_variant_get_uint64
);
impl_numeric!(
    f64,
    VariantTy::DOUBLE,
    g_variant_new_double,
    g_variant_get_double
);

impl StaticVariantType for () {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::UNIT)
    }
}

impl ToVariant for () {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_tuple(ptr::null(), 0)) }
    }
}

impl From<()> for Variant {
    #[inline]
    fn from(_: ()) -> Self {
        ().to_variant()
    }
}

impl FromVariant for () {
    fn from_variant(variant: &Variant) -> Option<Self> {
        if variant.is::<Self>() {
            Some(())
        } else {
            None
        }
    }
}

impl StaticVariantType for bool {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::BOOLEAN)
    }
}

impl ToVariant for bool {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_boolean(self.into_glib())) }
    }
}

impl From<bool> for Variant {
    #[inline]
    fn from(v: bool) -> Self {
        v.to_variant()
    }
}

impl FromVariant for bool {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                Some(from_glib(ffi::g_variant_get_boolean(
                    variant.to_glib_none().0,
                )))
            } else {
                None
            }
        }
    }
}

impl StaticVariantType for String {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::STRING)
    }
}

impl ToVariant for String {
    fn to_variant(&self) -> Variant {
        self[..].to_variant()
    }
}

impl From<String> for Variant {
    #[inline]
    fn from(s: String) -> Self {
        s.to_variant()
    }
}

impl FromVariant for String {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.str().map(String::from)
    }
}

impl StaticVariantType for str {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        String::static_variant_type()
    }
}

impl ToVariant for str {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_take_string(self.to_glib_full())) }
    }
}

impl From<&str> for Variant {
    #[inline]
    fn from(s: &str) -> Self {
        s.to_variant()
    }
}

impl StaticVariantType for std::path::PathBuf {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        std::path::Path::static_variant_type()
    }
}

impl ToVariant for std::path::PathBuf {
    fn to_variant(&self) -> Variant {
        self.as_path().to_variant()
    }
}

impl From<std::path::PathBuf> for Variant {
    #[inline]
    fn from(p: std::path::PathBuf) -> Self {
        p.to_variant()
    }
}

impl FromVariant for std::path::PathBuf {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            let ptr = ffi::g_variant_get_bytestring(variant.to_glib_none().0);
            Some(crate::translate::c_to_path_buf(ptr as *const _))
        }
    }
}

impl StaticVariantType for std::path::Path {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <&[u8]>::static_variant_type()
    }
}

impl ToVariant for std::path::Path {
    fn to_variant(&self) -> Variant {
        let tmp = crate::translate::path_to_c(self);
        unsafe { from_glib_none(ffi::g_variant_new_bytestring(tmp.as_ptr() as *const u8)) }
    }
}

impl From<&std::path::Path> for Variant {
    #[inline]
    fn from(p: &std::path::Path) -> Self {
        p.to_variant()
    }
}

impl StaticVariantType for std::ffi::OsString {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        std::ffi::OsStr::static_variant_type()
    }
}

impl ToVariant for std::ffi::OsString {
    fn to_variant(&self) -> Variant {
        self.as_os_str().to_variant()
    }
}

impl From<std::ffi::OsString> for Variant {
    #[inline]
    fn from(s: std::ffi::OsString) -> Self {
        s.to_variant()
    }
}

impl FromVariant for std::ffi::OsString {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            let ptr = ffi::g_variant_get_bytestring(variant.to_glib_none().0);
            Some(crate::translate::c_to_os_string(ptr as *const _))
        }
    }
}

impl StaticVariantType for std::ffi::OsStr {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <&[u8]>::static_variant_type()
    }
}

impl ToVariant for std::ffi::OsStr {
    fn to_variant(&self) -> Variant {
        let tmp = crate::translate::os_str_to_c(self);
        unsafe { from_glib_none(ffi::g_variant_new_bytestring(tmp.as_ptr() as *const u8)) }
    }
}

impl From<&std::ffi::OsStr> for Variant {
    #[inline]
    fn from(s: &std::ffi::OsStr) -> Self {
        s.to_variant()
    }
}

impl<T: StaticVariantType> StaticVariantType for Option<T> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Owned(VariantType::new_maybe(&T::static_variant_type()))
    }
}

impl<T: StaticVariantType + ToVariant> ToVariant for Option<T> {
    fn to_variant(&self) -> Variant {
        Variant::from_maybe::<T>(self.as_ref().map(|m| m.to_variant()).as_ref())
    }
}

impl<T: StaticVariantType + Into<Variant>> From<Option<T>> for Variant {
    #[inline]
    fn from(v: Option<T>) -> Self {
        Variant::from_maybe::<T>(v.map(|v| v.into()).as_ref())
    }
}

impl<T: StaticVariantType + FromVariant> FromVariant for Option<T> {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                let c_child = ffi::g_variant_get_maybe(variant.to_glib_none().0);
                if !c_child.is_null() {
                    let child: Variant = from_glib_full(c_child);

                    Some(T::from_variant(&child))
                } else {
                    Some(None)
                }
            } else {
                None
            }
        }
    }
}

impl<T: StaticVariantType> StaticVariantType for [T] {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        T::static_variant_type().as_array()
    }
}

impl<T: StaticVariantType + ToVariant> ToVariant for [T] {
    fn to_variant(&self) -> Variant {
        unsafe {
            if self.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    T::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for value in self {
                let value = value.to_variant();
                ffi::g_variant_builder_add_value(&mut builder, value.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

impl<T: StaticVariantType + ToVariant> From<&[T]> for Variant {
    #[inline]
    fn from(s: &[T]) -> Self {
        s.to_variant()
    }
}

impl<T: FromVariant> FromVariant for Vec<T> {
    fn from_variant(variant: &Variant) -> Option<Self> {
        if !variant.is_container() {
            return None;
        }

        let mut vec = Vec::with_capacity(variant.n_children());

        for i in 0..variant.n_children() {
            match variant.child_value(i).get() {
                Some(child) => vec.push(child),
                None => return None,
            }
        }

        Some(vec)
    }
}

impl<T: StaticVariantType + ToVariant> ToVariant for Vec<T> {
    fn to_variant(&self) -> Variant {
        self.as_slice().to_variant()
    }
}

impl<T: StaticVariantType + Into<Variant>> From<Vec<T>> for Variant {
    fn from(v: Vec<T>) -> Self {
        unsafe {
            if v.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    T::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for value in v {
                let value = value.into();
                ffi::g_variant_builder_add_value(&mut builder, value.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

impl<T: StaticVariantType> StaticVariantType for Vec<T> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <[T]>::static_variant_type()
    }
}

impl<K, V, H> FromVariant for HashMap<K, V, H>
where
    K: FromVariant + Eq + Hash,
    V: FromVariant,
    H: BuildHasher + Default,
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        if !variant.is_container() {
            return None;
        }

        let mut map = HashMap::default();

        for i in 0..variant.n_children() {
            let entry = variant.child_value(i);
            let key = entry.child_value(0).get()?;
            let val = entry.child_value(1).get()?;

            map.insert(key, val);
        }

        Some(map)
    }
}

impl<K, V> FromVariant for BTreeMap<K, V>
where
    K: FromVariant + Eq + Ord,
    V: FromVariant,
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        if !variant.is_container() {
            return None;
        }

        let mut map = BTreeMap::default();

        for i in 0..variant.n_children() {
            let entry = variant.child_value(i);
            let key = entry.child_value(0).get()?;
            let val = entry.child_value(1).get()?;

            map.insert(key, val);
        }

        Some(map)
    }
}

impl<K, V> ToVariant for HashMap<K, V>
where
    K: StaticVariantType + ToVariant + Eq + Hash,
    V: StaticVariantType + ToVariant,
{
    fn to_variant(&self) -> Variant {
        unsafe {
            if self.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    DictEntry::<K, V>::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for (key, value) in self {
                let entry = DictEntry::new(key, value).to_variant();
                ffi::g_variant_builder_add_value(&mut builder, entry.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

impl<K, V> From<HashMap<K, V>> for Variant
where
    K: StaticVariantType + Into<Variant> + Eq + Hash,
    V: StaticVariantType + Into<Variant>,
{
    fn from(m: HashMap<K, V>) -> Self {
        unsafe {
            if m.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    DictEntry::<K, V>::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for (key, value) in m {
                let entry = Variant::from(DictEntry::new(key, value));
                ffi::g_variant_builder_add_value(&mut builder, entry.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

impl<K, V> ToVariant for BTreeMap<K, V>
where
    K: StaticVariantType + ToVariant + Eq + Hash,
    V: StaticVariantType + ToVariant,
{
    fn to_variant(&self) -> Variant {
        unsafe {
            if self.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    DictEntry::<K, V>::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for (key, value) in self {
                let entry = DictEntry::new(key, value).to_variant();
                ffi::g_variant_builder_add_value(&mut builder, entry.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

impl<K, V> From<BTreeMap<K, V>> for Variant
where
    K: StaticVariantType + Into<Variant> + Eq + Hash,
    V: StaticVariantType + Into<Variant>,
{
    fn from(m: BTreeMap<K, V>) -> Self {
        unsafe {
            if m.is_empty() {
                return from_glib_none(ffi::g_variant_new_array(
                    DictEntry::<K, V>::static_variant_type().to_glib_none().0,
                    ptr::null(),
                    0,
                ));
            }

            let mut builder = mem::MaybeUninit::uninit();
            ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::ARRAY.to_glib_none().0);
            let mut builder = builder.assume_init();
            for (key, value) in m {
                let entry = Variant::from(DictEntry::new(key, value));
                ffi::g_variant_builder_add_value(&mut builder, entry.to_glib_none().0);
            }
            from_glib_none(ffi::g_variant_builder_end(&mut builder))
        }
    }
}

/// A Dictionary entry.
///
/// While GVariant format allows a dictionary entry to be an independent type, typically you'll need
/// to use this in a dictionary, which is simply an array of dictionary entries. The following code
/// creates a dictionary:
///
/// ```
///# use glib::prelude::*; // or `use gtk::prelude::*;`
/// use glib::variant::{Variant, FromVariant, DictEntry};
///
/// let entries = [
///     DictEntry::new("uuid", 1000u32),
///     DictEntry::new("guid", 1001u32),
/// ];
/// let dict = entries.into_iter().collect::<Variant>();
/// assert_eq!(dict.n_children(), 2);
/// assert_eq!(dict.type_().as_str(), "a{su}");
/// ```
#[derive(Debug, Clone)]
pub struct DictEntry<K, V> {
    key: K,
    value: V,
}

impl<K, V> DictEntry<K, V>
where
    K: StaticVariantType,
    V: StaticVariantType,
{
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<K, V> FromVariant for DictEntry<K, V>
where
    K: FromVariant,
    V: FromVariant,
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        if !variant.type_().is_subtype_of(VariantTy::DICT_ENTRY) {
            return None;
        }

        let key = variant.child_value(0).get()?;
        let value = variant.child_value(1).get()?;

        Some(Self { key, value })
    }
}

impl<K, V> ToVariant for DictEntry<K, V>
where
    K: StaticVariantType + ToVariant,
    V: StaticVariantType + ToVariant,
{
    fn to_variant(&self) -> Variant {
        Variant::from_dict_entry(&self.key.to_variant(), &self.value.to_variant())
    }
}

impl<K, V> From<DictEntry<K, V>> for Variant
where
    K: StaticVariantType + Into<Variant>,
    V: StaticVariantType + Into<Variant>,
{
    fn from(e: DictEntry<K, V>) -> Self {
        Variant::from_dict_entry(&e.key.into(), &e.value.into())
    }
}

impl ToVariant for Variant {
    fn to_variant(&self) -> Variant {
        Variant::from_variant(self)
    }
}

impl FromVariant for Variant {
    fn from_variant(variant: &Variant) -> Option<Self> {
        variant.as_variant()
    }
}

impl<K: StaticVariantType, V: StaticVariantType> StaticVariantType for DictEntry<K, V> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Owned(VariantType::new_dict_entry(
            &K::static_variant_type(),
            &V::static_variant_type(),
        ))
    }
}

fn static_variant_mapping<K, V>() -> Cow<'static, VariantTy>
where
    K: StaticVariantType,
    V: StaticVariantType,
{
    use std::fmt::Write;

    let key_type = K::static_variant_type();
    let value_type = V::static_variant_type();

    if key_type == VariantTy::STRING && value_type == VariantTy::VARIANT {
        return Cow::Borrowed(VariantTy::VARDICT);
    }

    let mut builder = crate::GStringBuilder::default();
    write!(builder, "a{{{}{}}}", key_type.as_str(), value_type.as_str()).unwrap();

    Cow::Owned(VariantType::from_string(builder.into_string()).unwrap())
}

impl<K, V, H> StaticVariantType for HashMap<K, V, H>
where
    K: StaticVariantType,
    V: StaticVariantType,
    H: BuildHasher + Default,
{
    fn static_variant_type() -> Cow<'static, VariantTy> {
        static_variant_mapping::<K, V>()
    }
}

impl<K, V> StaticVariantType for BTreeMap<K, V>
where
    K: StaticVariantType,
    V: StaticVariantType,
{
    fn static_variant_type() -> Cow<'static, VariantTy> {
        static_variant_mapping::<K, V>()
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> StaticVariantType for ($($name,)+)
            where
                $($name: StaticVariantType,)+
            {
                fn static_variant_type() -> Cow<'static, VariantTy> {
                    Cow::Owned(VariantType::new_tuple(&[
                        $(
                            $name::static_variant_type(),
                        )+
                    ]))
                }
            }

            impl<$($name),+> FromVariant for ($($name,)+)
            where
                $($name: FromVariant,)+
            {
                fn from_variant(variant: &Variant) -> Option<Self> {
                    if !variant.type_().is_subtype_of(VariantTy::TUPLE) {
                        return None;
                    }

                    Some((
                        $(
                            match variant.try_child_get::<$name>($n) {
                                Ok(Some(field)) => field,
                                _ => return None,
                            },
                        )+
                    ))
                }
            }

            impl<$($name),+> ToVariant for ($($name,)+)
            where
                $($name: ToVariant,)+
            {
                fn to_variant(&self) -> Variant {
                    unsafe {
                        let mut builder = mem::MaybeUninit::uninit();
                        ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::TUPLE.to_glib_none().0);
                        let mut builder = builder.assume_init();

                        $(
                            let field = self.$n.to_variant();
                            ffi::g_variant_builder_add_value(&mut builder, field.to_glib_none().0);
                        )+

                        from_glib_none(ffi::g_variant_builder_end(&mut builder))
                    }
                }
            }

            impl<$($name),+> From<($($name,)+)> for Variant
            where
                $($name: Into<Variant>,)+
            {
                fn from(t: ($($name,)+)) -> Self {
                    unsafe {
                        let mut builder = mem::MaybeUninit::uninit();
                        ffi::g_variant_builder_init(builder.as_mut_ptr(), VariantTy::TUPLE.to_glib_none().0);
                        let mut builder = builder.assume_init();

                        $(
                            let field = t.$n.into();
                            ffi::g_variant_builder_add_value(&mut builder, field.to_glib_none().0);
                        )+

                        from_glib_none(ffi::g_variant_builder_end(&mut builder))
                    }
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl<T: Into<Variant> + StaticVariantType> FromIterator<T> for Variant {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Variant::array_from_iter::<T>(iter.into_iter().map(|v| v.into()))
    }
}

/// Trait for fixed size variant types.
pub unsafe trait FixedSizeVariantType: StaticVariantType + Sized + Copy {}
unsafe impl FixedSizeVariantType for u8 {}
unsafe impl FixedSizeVariantType for i16 {}
unsafe impl FixedSizeVariantType for u16 {}
unsafe impl FixedSizeVariantType for i32 {}
unsafe impl FixedSizeVariantType for u32 {}
unsafe impl FixedSizeVariantType for i64 {}
unsafe impl FixedSizeVariantType for u64 {}
unsafe impl FixedSizeVariantType for f64 {}
unsafe impl FixedSizeVariantType for bool {}

/// Wrapper type for fixed size type arrays.
///
/// Converting this from/to a `Variant` is generally more efficient than working on the type
/// directly. This is especially important when deriving `Variant` trait implementations on custom
/// types.
///
/// This wrapper type can hold for example `Vec<u8>`, `Box<[u8]>` and similar types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedSizeVariantArray<A, T>(A, std::marker::PhantomData<T>)
where
    A: AsRef<[T]>,
    T: FixedSizeVariantType;

impl<A: AsRef<[T]>, T: FixedSizeVariantType> From<A> for FixedSizeVariantArray<A, T> {
    fn from(array: A) -> Self {
        FixedSizeVariantArray(array, std::marker::PhantomData)
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> FixedSizeVariantArray<A, T> {
    pub fn into_inner(self) -> A {
        self.0
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> std::ops::Deref for FixedSizeVariantArray<A, T> {
    type Target = A;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> std::ops::DerefMut for FixedSizeVariantArray<A, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> AsRef<A> for FixedSizeVariantArray<A, T> {
    #[inline]
    fn as_ref(&self) -> &A {
        &self.0
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> AsMut<A> for FixedSizeVariantArray<A, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut A {
        &mut self.0
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> AsRef<[T]> for FixedSizeVariantArray<A, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<A: AsRef<[T]> + AsMut<[T]>, T: FixedSizeVariantType> AsMut<[T]>
    for FixedSizeVariantArray<A, T>
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> StaticVariantType for FixedSizeVariantArray<A, T> {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <[T]>::static_variant_type()
    }
}

impl<A: AsRef<[T]> + for<'a> From<&'a [T]>, T: FixedSizeVariantType> FromVariant
    for FixedSizeVariantArray<A, T>
{
    fn from_variant(variant: &Variant) -> Option<Self> {
        Some(FixedSizeVariantArray(
            A::from(variant.fixed_array::<T>().ok()?),
            std::marker::PhantomData,
        ))
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> ToVariant for FixedSizeVariantArray<A, T> {
    fn to_variant(&self) -> Variant {
        Variant::array_from_fixed_array(self.0.as_ref())
    }
}

impl<A: AsRef<[T]>, T: FixedSizeVariantType> From<FixedSizeVariantArray<A, T>> for Variant {
    #[doc(alias = "g_variant_new_from_data")]
    fn from(a: FixedSizeVariantArray<A, T>) -> Self {
        unsafe {
            let data = Box::new(a.0);
            let (data_ptr, len) = {
                let data = (*data).as_ref();
                (data.as_ptr(), mem::size_of_val(data))
            };

            unsafe extern "C" fn free_data<A: AsRef<[T]>, T: FixedSizeVariantType>(
                ptr: ffi::gpointer,
            ) {
                let _ = Box::from_raw(ptr as *mut A);
            }

            from_glib_none(ffi::g_variant_new_from_data(
                T::static_variant_type().to_glib_none().0,
                data_ptr as ffi::gconstpointer,
                len,
                false.into_glib(),
                Some(free_data::<A, T>),
                Box::into_raw(data) as ffi::gpointer,
            ))
        }
    }
}

/// A wrapper type around `Variant` handles.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle(pub i32);

impl From<i32> for Handle {
    fn from(v: i32) -> Self {
        Handle(v)
    }
}

impl From<Handle> for i32 {
    fn from(v: Handle) -> Self {
        v.0
    }
}

impl StaticVariantType for Handle {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::HANDLE)
    }
}

impl ToVariant for Handle {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_handle(self.0)) }
    }
}

impl From<Handle> for Variant {
    #[inline]
    fn from(h: Handle) -> Self {
        h.to_variant()
    }
}

impl FromVariant for Handle {
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                Some(Handle(ffi::g_variant_get_handle(variant.to_glib_none().0)))
            } else {
                None
            }
        }
    }
}

/// A wrapper type around `Variant` object paths.
///
/// Values of these type are guaranteed to be valid object paths.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectPath(String);

impl ObjectPath {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ObjectPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::ops::Deref for ObjectPath {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for ObjectPath {
    type Error = crate::BoolError;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        if !Variant::is_object_path(&v) {
            return Err(bool_error!("Invalid object path"));
        }

        Ok(ObjectPath(v))
    }
}

impl<'a> TryFrom<&'a str> for ObjectPath {
    type Error = crate::BoolError;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        ObjectPath::try_from(String::from(v))
    }
}

impl From<ObjectPath> for String {
    fn from(v: ObjectPath) -> Self {
        v.0
    }
}

impl StaticVariantType for ObjectPath {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::OBJECT_PATH)
    }
}

impl ToVariant for ObjectPath {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_object_path(self.0.to_glib_none().0)) }
    }
}

impl From<ObjectPath> for Variant {
    #[inline]
    fn from(p: ObjectPath) -> Self {
        let mut s = p.0;
        s.push('\0');
        unsafe { Self::from_data_trusted::<ObjectPath, _>(s) }
    }
}

impl FromVariant for ObjectPath {
    #[allow(unused_unsafe)]
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                Some(ObjectPath(String::from(variant.str().unwrap())))
            } else {
                None
            }
        }
    }
}

/// A wrapper type around `Variant` signatures.
///
/// Values of these type are guaranteed to be valid signatures.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature(String);

impl Signature {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for Signature {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for Signature {
    type Error = crate::BoolError;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        if !Variant::is_signature(&v) {
            return Err(bool_error!("Invalid signature"));
        }

        Ok(Signature(v))
    }
}

impl<'a> TryFrom<&'a str> for Signature {
    type Error = crate::BoolError;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        Signature::try_from(String::from(v))
    }
}

impl From<Signature> for String {
    fn from(v: Signature) -> Self {
        v.0
    }
}

impl StaticVariantType for Signature {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::SIGNATURE)
    }
}

impl ToVariant for Signature {
    fn to_variant(&self) -> Variant {
        unsafe { from_glib_none(ffi::g_variant_new_signature(self.0.to_glib_none().0)) }
    }
}

impl From<Signature> for Variant {
    #[inline]
    fn from(s: Signature) -> Self {
        let mut s = s.0;
        s.push('\0');
        unsafe { Self::from_data_trusted::<Signature, _>(s) }
    }
}

impl FromVariant for Signature {
    #[allow(unused_unsafe)]
    fn from_variant(variant: &Variant) -> Option<Self> {
        unsafe {
            if variant.is::<Self>() {
                Some(Signature(String::from(variant.str().unwrap())))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    macro_rules! unsigned {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::MAX;
                while n > 0 {
                    let v = n.to_variant();
                    assert_eq!(v.get(), Some(n));
                    n /= 2;
                }
            }
        };
    }

    macro_rules! signed {
        ($name:ident, $ty:ident) => {
            #[test]
            fn $name() {
                let mut n = $ty::MAX;
                while n > 0 {
                    let v = n.to_variant();
                    assert_eq!(v.get(), Some(n));
                    let v = (-n).to_variant();
                    assert_eq!(v.get(), Some(-n));
                    n /= 2;
                }
            }
        };
    }

    unsigned!(test_u8, u8);
    unsigned!(test_u16, u16);
    unsigned!(test_u32, u32);
    unsigned!(test_u64, u64);
    signed!(test_i16, i16);
    signed!(test_i32, i32);
    signed!(test_i64, i64);

    #[test]
    fn test_str() {
        let s = "this is a test";
        let v = s.to_variant();
        assert_eq!(v.str(), Some(s));
        assert_eq!(42u32.to_variant().str(), None);
    }

    #[test]
    fn test_fixed_array() {
        let b = b"this is a test";
        let v = Variant::array_from_fixed_array(&b[..]);
        assert_eq!(v.type_().as_str(), "ay");
        assert_eq!(v.fixed_array::<u8>().unwrap(), b);
        assert!(42u32.to_variant().fixed_array::<u8>().is_err());

        let b = [1u32, 10u32, 100u32];
        let v = Variant::array_from_fixed_array(&b);
        assert_eq!(v.type_().as_str(), "au");
        assert_eq!(v.fixed_array::<u32>().unwrap(), b);
        assert!(v.fixed_array::<u8>().is_err());

        let b = [true, false, true];
        let v = Variant::array_from_fixed_array(&b);
        assert_eq!(v.type_().as_str(), "ab");
        assert_eq!(v.fixed_array::<bool>().unwrap(), b);
        assert!(v.fixed_array::<u8>().is_err());

        let b = [1.0f64, 2.0f64, 3.0f64];
        let v = Variant::array_from_fixed_array(&b);
        assert_eq!(v.type_().as_str(), "ad");
        #[allow(clippy::float_cmp)]
        {
            assert_eq!(v.fixed_array::<f64>().unwrap(), b);
        }
        assert!(v.fixed_array::<u64>().is_err());
    }

    #[test]
    fn test_fixed_variant_array() {
        let b = FixedSizeVariantArray::from(&b"this is a test"[..]);
        let v = b.to_variant();
        assert_eq!(v.type_().as_str(), "ay");
        assert_eq!(
            &*v.get::<FixedSizeVariantArray<Vec<u8>, u8>>().unwrap(),
            &*b
        );

        let b = FixedSizeVariantArray::from(vec![1i32, 2, 3]);
        let v = b.to_variant();
        assert_eq!(v.type_().as_str(), "ai");
        assert_eq!(v.get::<FixedSizeVariantArray<Vec<i32>, i32>>().unwrap(), b);
    }

    #[test]
    fn test_string() {
        let s = String::from("this is a test");
        let v = s.to_variant();
        assert_eq!(v.get(), Some(s));
        assert_eq!(v.normal_form(), v);
    }

    #[test]
    fn test_eq() {
        let v1 = "this is a test".to_variant();
        let v2 = "this is a test".to_variant();
        let v3 = "test".to_variant();
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_hash() {
        let v1 = "this is a test".to_variant();
        let v2 = "this is a test".to_variant();
        let v3 = "test".to_variant();
        let mut set = HashSet::new();
        set.insert(v1);
        assert!(set.contains(&v2));
        assert!(!set.contains(&v3));

        assert_eq!(
            <HashMap<&str, (&str, u8, u32)>>::static_variant_type().as_str(),
            "a{s(syu)}"
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(<Vec<&str>>::static_variant_type().as_str(), "as");
        assert_eq!(
            <Vec<(&str, u8, u32)>>::static_variant_type().as_str(),
            "a(syu)"
        );
        let a = ["foo", "bar", "baz"].to_variant();
        assert_eq!(a.normal_form(), a);
        assert_eq!(a.array_iter_str().unwrap().len(), 3);
        let o = 0u32.to_variant();
        assert!(o.array_iter_str().is_err());
    }

    #[test]
    fn test_array_from_iter() {
        let a = Variant::array_from_iter::<String>(
            ["foo", "bar", "baz"].into_iter().map(|s| s.to_variant()),
        );
        assert_eq!(a.type_().as_str(), "as");
        assert_eq!(a.n_children(), 3);

        assert_eq!(a.try_child_get::<String>(0), Ok(Some(String::from("foo"))));
        assert_eq!(a.try_child_get::<String>(1), Ok(Some(String::from("bar"))));
        assert_eq!(a.try_child_get::<String>(2), Ok(Some(String::from("baz"))));
    }

    #[test]
    fn test_array_collect() {
        let a = ["foo", "bar", "baz"].into_iter().collect::<Variant>();
        assert_eq!(a.type_().as_str(), "as");
        assert_eq!(a.n_children(), 3);

        assert_eq!(a.try_child_get::<String>(0), Ok(Some(String::from("foo"))));
        assert_eq!(a.try_child_get::<String>(1), Ok(Some(String::from("bar"))));
        assert_eq!(a.try_child_get::<String>(2), Ok(Some(String::from("baz"))));
    }

    #[test]
    fn test_tuple() {
        assert_eq!(<(&str, u32)>::static_variant_type().as_str(), "(su)");
        assert_eq!(<(&str, u8, u32)>::static_variant_type().as_str(), "(syu)");
        let a = ("test", 1u8, 2u32).to_variant();
        assert_eq!(a.normal_form(), a);
        assert_eq!(a.try_child_get::<String>(0), Ok(Some(String::from("test"))));
        assert_eq!(a.try_child_get::<u8>(1), Ok(Some(1u8)));
        assert_eq!(a.try_child_get::<u32>(2), Ok(Some(2u32)));
        assert_eq!(
            a.try_get::<(String, u8, u32)>(),
            Ok((String::from("test"), 1u8, 2u32))
        );
    }

    #[test]
    fn test_tuple_from_iter() {
        let a = Variant::tuple_from_iter(["foo".to_variant(), 1u8.to_variant(), 2i32.to_variant()]);
        assert_eq!(a.type_().as_str(), "(syi)");
        assert_eq!(a.n_children(), 3);

        assert_eq!(a.try_child_get::<String>(0), Ok(Some(String::from("foo"))));
        assert_eq!(a.try_child_get::<u8>(1), Ok(Some(1u8)));
        assert_eq!(a.try_child_get::<i32>(2), Ok(Some(2i32)));
    }

    #[test]
    fn test_empty() {
        assert_eq!(<()>::static_variant_type().as_str(), "()");
        let a = ().to_variant();
        assert_eq!(a.type_().as_str(), "()");
        assert_eq!(a.get::<()>(), Some(()));
    }

    #[test]
    fn test_maybe() {
        assert!(<Option<()>>::static_variant_type().is_maybe());
        let m1 = Some(()).to_variant();
        assert_eq!(m1.type_().as_str(), "m()");

        assert_eq!(m1.get::<Option<()>>(), Some(Some(())));
        assert!(m1.as_maybe().is_some());

        let m2 = None::<()>.to_variant();
        assert!(m2.as_maybe().is_none());
    }

    #[test]
    fn test_btreemap() {
        assert_eq!(
            <BTreeMap<String, u32>>::static_variant_type().as_str(),
            "a{su}"
        );
        // Validate that BTreeMap adds entries to dict in sorted order
        let mut m = BTreeMap::new();
        let total = 20;
        for n in 0..total {
            let k = format!("v{n:04}");
            m.insert(k, n as u32);
        }
        let v = m.to_variant();
        let n = v.n_children();
        assert_eq!(total, n);
        for n in 0..total {
            let child = v
                .try_child_get::<DictEntry<String, u32>>(n)
                .unwrap()
                .unwrap();
            assert_eq!(*child.value(), n as u32);
        }

        assert_eq!(BTreeMap::from_variant(&v).unwrap(), m);
    }

    #[test]
    fn test_get() -> Result<(), Box<dyn std::error::Error>> {
        let u = 42u32.to_variant();
        assert!(u.get::<i32>().is_none());
        assert_eq!(u.get::<u32>().unwrap(), 42);
        assert!(u.try_get::<i32>().is_err());
        // Test ? conversion
        assert_eq!(u.try_get::<u32>()?, 42);
        Ok(())
    }

    #[test]
    fn test_byteswap() {
        let u = 42u32.to_variant();
        assert_eq!(u.byteswap().get::<u32>().unwrap(), 704643072u32);
        assert_eq!(u.byteswap().byteswap().get::<u32>().unwrap(), 42u32);
    }

    #[test]
    fn test_try_child() {
        let a = ["foo"].to_variant();
        assert!(a.try_child_value(0).is_some());
        assert_eq!(a.try_child_get::<String>(0).unwrap().unwrap(), "foo");
        assert_eq!(a.child_get::<String>(0), "foo");
        assert!(a.try_child_get::<u32>(0).is_err());
        assert!(a.try_child_value(1).is_none());
        assert!(a.try_child_get::<String>(1).unwrap().is_none());
        let u = 42u32.to_variant();
        assert!(u.try_child_value(0).is_none());
        assert!(u.try_child_get::<String>(0).unwrap().is_none());
    }

    #[test]
    fn test_serialize() {
        let a = ("test", 1u8, 2u32).to_variant();

        let bytes = a.data_as_bytes();
        let data = a.data();
        let len = a.size();
        assert_eq!(bytes.len(), len);
        assert_eq!(data.len(), len);

        let mut store_data = vec![0u8; len];
        assert_eq!(a.store(&mut store_data).unwrap(), len);

        assert_eq!(&bytes, data);
        assert_eq!(&store_data, data);

        let b = Variant::from_data::<(String, u8, u32), _>(store_data);
        assert_eq!(a, b);

        let c = Variant::from_bytes::<(String, u8, u32)>(&bytes);
        assert_eq!(a, c);
    }

    #[test]
    fn test_print_parse() {
        let a = ("test", 1u8, 2u32).to_variant();

        let a2 = Variant::parse(Some(a.type_()), &a.print(false)).unwrap();
        assert_eq!(a, a2);

        let a3: Variant = a.to_string().parse().unwrap();
        assert_eq!(a, a3);
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn test_paths() {
        use std::path::PathBuf;

        let path = PathBuf::from("foo");
        let v = path.to_variant();
        assert_eq!(PathBuf::from_variant(&v), Some(path));
    }

    #[test]
    fn test_regression_from_variant_panics() {
        let variant = "text".to_variant();
        let hashmap: Option<HashMap<u64, u64>> = FromVariant::from_variant(&variant);
        assert!(hashmap.is_none());

        let variant = HashMap::<u64, u64>::new().to_variant();
        let hashmap: Option<HashMap<u64, u64>> = FromVariant::from_variant(&variant);
        assert!(hashmap.is_some());
    }
}
