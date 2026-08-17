// Take a look at the license at the top of the repository in the LICENSE file.

use std::{cmp, ffi::CStr, fmt, ops::Deref, ptr};

use crate::{
    ffi, gobject_ffi, prelude::*, translate::*, ParamSpecEnum, ParamSpecFlags, Type, TypeInfo,
    Value,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UserDirectory {
    #[doc(alias = "G_USER_DIRECTORY_DESKTOP")]
    Desktop,
    #[doc(alias = "G_USER_DIRECTORY_DOCUMENTS")]
    Documents,
    #[doc(alias = "G_USER_DIRECTORY_DOWNLOAD")]
    Downloads,
    #[doc(alias = "G_USER_DIRECTORY_MUSIC")]
    Music,
    #[doc(alias = "G_USER_DIRECTORY_PICTURES")]
    Pictures,
    #[doc(alias = "G_USER_DIRECTORY_PUBLIC_SHARE")]
    PublicShare,
    #[doc(alias = "G_USER_DIRECTORY_TEMPLATES")]
    Templates,
    #[doc(alias = "G_USER_DIRECTORY_VIDEOS")]
    Videos,
}

#[doc(hidden)]
impl IntoGlib for UserDirectory {
    type GlibType = ffi::GUserDirectory;

    #[inline]
    fn into_glib(self) -> ffi::GUserDirectory {
        match self {
            Self::Desktop => ffi::G_USER_DIRECTORY_DESKTOP,
            Self::Documents => ffi::G_USER_DIRECTORY_DOCUMENTS,
            Self::Downloads => ffi::G_USER_DIRECTORY_DOWNLOAD,
            Self::Music => ffi::G_USER_DIRECTORY_MUSIC,
            Self::Pictures => ffi::G_USER_DIRECTORY_PICTURES,
            Self::PublicShare => ffi::G_USER_DIRECTORY_PUBLIC_SHARE,
            Self::Templates => ffi::G_USER_DIRECTORY_TEMPLATES,
            Self::Videos => ffi::G_USER_DIRECTORY_VIDEOS,
        }
    }
}

// rustdoc-stripper-ignore-next
/// Representation of an `enum` for dynamically, at runtime, querying the values of the enum and
/// using them.
#[doc(alias = "GEnumClass")]
#[repr(transparent)]
pub struct EnumClass(ptr::NonNull<gobject_ffi::GEnumClass>);

unsafe impl Send for EnumClass {}
unsafe impl Sync for EnumClass {}

impl fmt::Debug for EnumClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnumClass")
            .field("type", &self.type_())
            .field("values", &self.values())
            .finish()
    }
}

impl EnumClass {
    // rustdoc-stripper-ignore-next
    /// Create a new `EnumClass` from a static type `T`.
    ///
    /// Panics if `T` is not representing an enum.
    pub fn new<T: StaticType + HasParamSpec<ParamSpec = ParamSpecEnum>>() -> Self {
        Self::with_type(T::static_type()).expect("invalid enum class")
    }
    // rustdoc-stripper-ignore-next
    /// Create a new `EnumClass` from a `Type`.
    ///
    /// Returns `None` if `type_` is not representing an enum.
    pub fn with_type(type_: Type) -> Option<Self> {
        unsafe {
            let is_enum: bool = from_glib(gobject_ffi::g_type_is_a(
                type_.into_glib(),
                gobject_ffi::G_TYPE_ENUM,
            ));
            if !is_enum {
                return None;
            }

            Some(EnumClass(
                ptr::NonNull::new(gobject_ffi::g_type_class_ref(type_.into_glib()) as *mut _)
                    .unwrap(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// `Type` of the enum.
    pub fn type_(&self) -> Type {
        unsafe { from_glib(self.0.as_ref().g_type_class.g_type) }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `EnumValue` by integer `value`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with `value`.
    #[doc(alias = "g_enum_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self, value: i32) -> Option<&EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value(self.0.as_ptr(), value);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const EnumValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `EnumValue` by string name `name`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with name `name`.
    #[doc(alias = "g_enum_get_value_by_name")]
    #[doc(alias = "get_value_by_name")]
    pub fn value_by_name(&self, name: &str) -> Option<&EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value_by_name(self.0.as_ptr(), name.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const EnumValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `EnumValue` by string nick `nick`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with nick `nick`.
    #[doc(alias = "g_enum_get_value_by_nick")]
    #[doc(alias = "get_value_by_nick")]
    pub fn value_by_nick(&self, nick: &str) -> Option<&EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value_by_nick(self.0.as_ptr(), nick.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const EnumValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets all `EnumValue` of this `EnumClass`.
    #[doc(alias = "get_values")]
    pub fn values(&self) -> &[EnumValue] {
        unsafe {
            if self.0.as_ref().n_values == 0 {
                return &[];
            }
            std::slice::from_raw_parts(
                self.0.as_ref().values as *const EnumValue,
                self.0.as_ref().n_values as usize,
            )
        }
    }

    // rustdoc-stripper-ignore-next
    /// Converts integer `value` to a `Value`, if part of the enum.
    pub fn to_value(&self, value: i32) -> Option<Value> {
        self.value(value).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Converts string name `name` to a `Value`, if part of the enum.
    pub fn to_value_by_name(&self, name: &str) -> Option<Value> {
        self.value_by_name(name).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Converts string nick `nick` to a `Value`, if part of the enum.
    pub fn to_value_by_nick(&self, nick: &str) -> Option<Value> {
        self.value_by_nick(nick).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Complete `TypeInfo` for an enum with values.
    /// This is an associated function. A method would result in a stack overflow due to a recurvice call:
    /// callers should first create an `EnumClass` instance by calling `EnumClass::with_type()` which indirectly
    /// calls `TypePluginRegisterImpl::register_dynamic_enum()` and `TypePluginImpl::complete_type_info()`
    /// and one of them should call `EnumClass::with_type()` before calling this method.
    /// `const_static_values` is a reference on a wrapper of a slice of `EnumValue`.
    /// It must be static to ensure enumeration values are never dropped, and ensures that slice is terminated
    ///  by an `EnumValue` with all members being 0, as expected by GLib.
    #[doc(alias = "g_enum_complete_type_info")]
    pub fn complete_type_info(
        type_: Type,
        const_static_values: &'static EnumValues,
    ) -> Option<TypeInfo> {
        unsafe {
            let is_enum: bool = from_glib(gobject_ffi::g_type_is_a(
                type_.into_glib(),
                gobject_ffi::G_TYPE_ENUM,
            ));
            if !is_enum {
                return None;
            }

            let info = TypeInfo::default();
            gobject_ffi::g_enum_complete_type_info(
                type_.into_glib(),
                info.as_ptr(),
                const_static_values.to_glib_none().0,
            );
            Some(info)
        }
    }
}

impl Drop for EnumClass {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_type_class_unref(self.0.as_ptr() as *mut _);
        }
    }
}

impl Clone for EnumClass {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            Self(ptr::NonNull::new(gobject_ffi::g_type_class_ref(self.type_().into_glib()) as *mut _).unwrap())
        }
    }
}

// rustdoc-stripper-ignore-next
/// Representation of a single enum value of an `EnumClass`.
#[doc(alias = "GEnumValue")]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct EnumValue(gobject_ffi::GEnumValue);

unsafe impl Send for EnumValue {}
unsafe impl Sync for EnumValue {}

impl fmt::Debug for EnumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnumValue")
            .field("value", &self.value())
            .field("name", &self.name())
            .field("nick", &self.nick())
            .finish()
    }
}

impl EnumValue {
    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// It is the responsibility of the caller to ensure `GEnumValue` is
    /// valid.
    pub const unsafe fn unsafe_from(g_value: gobject_ffi::GEnumValue) -> Self {
        Self(g_value)
    }

    // rustdoc-stripper-ignore-next
    /// Get integer value corresponding to the value.
    #[doc(alias = "get_value")]
    pub fn value(&self) -> i32 {
        self.0.value
    }

    // rustdoc-stripper-ignore-next
    /// Get name corresponding to the value.
    #[doc(alias = "get_name")]
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.value_name).to_str().unwrap() }
    }

    // rustdoc-stripper-ignore-next
    /// Get nick corresponding to the value.
    #[doc(alias = "get_nick")]
    pub fn nick(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.value_nick).to_str().unwrap() }
    }

    // rustdoc-stripper-ignore-next
    /// Convert enum value to a `Value`.
    pub fn to_value(&self, enum_: &EnumClass) -> Value {
        unsafe {
            let mut v = Value::from_type_unchecked(enum_.type_());
            gobject_ffi::g_value_set_enum(v.to_glib_none_mut().0, self.0.value);
            v
        }
    }

    // rustdoc-stripper-ignore-next
    /// Convert enum value from a `Value`.
    pub fn from_value(value: &Value) -> Option<(EnumClass, &EnumValue)> {
        unsafe {
            let enum_class = EnumClass::with_type(value.type_())?;
            let v = enum_class.value(gobject_ffi::g_value_get_enum(value.to_glib_none().0))?;
            let v = &*(v as *const EnumValue);
            Some((enum_class, v))
        }
    }
}

impl PartialEq for EnumValue {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(&other.value())
    }
}

impl Eq for EnumValue {}

impl PartialOrd for EnumValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EnumValue {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl UnsafeFrom<gobject_ffi::GEnumValue> for EnumValue {
    unsafe fn unsafe_from(g_value: gobject_ffi::GEnumValue) -> Self {
        Self::unsafe_from(g_value)
    }
}

unsafe impl<'a> crate::value::FromValue<'a> for &EnumValue {
    type Checker = EnumTypeChecker;

    unsafe fn from_value(value: &'a Value) -> Self {
        let (_, v) = EnumValue::from_value(value).unwrap();
        // SAFETY: The enum class and its values live forever
        std::mem::transmute(v)
    }
}

// rustdoc-stripper-ignore-next
/// Define the zero value and the associated GLib type.
impl EnumerationValue<EnumValue> for EnumValue {
    type GlibType = gobject_ffi::GEnumValue;
    const ZERO: EnumValue = unsafe {
        EnumValue::unsafe_from(gobject_ffi::GEnumValue {
            value: 0,
            value_name: ptr::null(),
            value_nick: ptr::null(),
        })
    };
}

// rustdoc-stripper-ignore-next
/// Storage of enum values.
pub type EnumValuesStorage<const N: usize> = EnumerationValuesStorage<EnumValue, N>;

// rustdoc-stripper-ignore-next
/// Representation of enum values wrapped by `EnumValuesStorage`
pub type EnumValues = EnumerationValues<EnumValue>;

pub struct EnumTypeChecker();
unsafe impl crate::value::ValueTypeChecker for EnumTypeChecker {
    type Error = InvalidEnumError;

    fn check(value: &Value) -> Result<(), Self::Error> {
        let t = value.type_();
        if t.is_a(Type::ENUM) {
            Ok(())
        } else {
            Err(InvalidEnumError)
        }
    }
}

// rustdoc-stripper-ignore-next
/// An error returned from the [`get`](struct.Value.html#method.get) function
/// on a [`Value`](struct.Value.html) for enum types.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InvalidEnumError;

impl fmt::Display for InvalidEnumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value is not an enum")
    }
}

impl std::error::Error for InvalidEnumError {}

// rustdoc-stripper-ignore-next
/// Representation of a `flags` for dynamically, at runtime, querying the values of the enum and
/// using them
#[doc(alias = "GFlagsClass")]
#[repr(transparent)]
pub struct FlagsClass(ptr::NonNull<gobject_ffi::GFlagsClass>);

unsafe impl Send for FlagsClass {}
unsafe impl Sync for FlagsClass {}

impl fmt::Debug for FlagsClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlagsClass")
            .field("type", &self.type_())
            .field("values", &self.values())
            .finish()
    }
}

impl FlagsClass {
    // rustdoc-stripper-ignore-next
    /// Create a new `FlagsClass` from a static type `T`.
    ///
    /// Panics if `T` is not representing an flags type.
    pub fn new<T: StaticType + HasParamSpec<ParamSpec = ParamSpecFlags>>() -> Self {
        Self::with_type(T::static_type()).expect("invalid flags class")
    }
    // rustdoc-stripper-ignore-next
    /// Create a new `FlagsClass` from a `Type`
    ///
    /// Returns `None` if `type_` is not representing a flags type.
    pub fn with_type(type_: Type) -> Option<Self> {
        unsafe {
            let is_flags: bool = from_glib(gobject_ffi::g_type_is_a(
                type_.into_glib(),
                gobject_ffi::G_TYPE_FLAGS,
            ));
            if !is_flags {
                return None;
            }

            Some(FlagsClass(
                ptr::NonNull::new(gobject_ffi::g_type_class_ref(type_.into_glib()) as *mut _)
                    .unwrap(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// `Type` of the flags.
    pub fn type_(&self) -> Type {
        unsafe { from_glib(self.0.as_ref().g_type_class.g_type) }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `FlagsValue` by integer `value`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with `value`.
    #[doc(alias = "g_flags_get_first_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self, value: u32) -> Option<&FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_first_value(self.0.as_ptr(), value);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const FlagsValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `FlagsValue` by string name `name`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with name `name`.
    #[doc(alias = "g_flags_get_value_by_name")]
    #[doc(alias = "get_value_by_name")]
    pub fn value_by_name(&self, name: &str) -> Option<&FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_value_by_name(self.0.as_ptr(), name.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const FlagsValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `FlagsValue` by string nick `nick`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with nick `nick`.
    #[doc(alias = "g_flags_get_value_by_nick")]
    #[doc(alias = "get_value_by_nick")]
    pub fn value_by_nick(&self, nick: &str) -> Option<&FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_value_by_nick(self.0.as_ptr(), nick.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const FlagsValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets all `FlagsValue` of this `FlagsClass`.
    #[doc(alias = "get_values")]
    pub fn values(&self) -> &[FlagsValue] {
        unsafe {
            if self.0.as_ref().n_values == 0 {
                return &[];
            }
            std::slice::from_raw_parts(
                self.0.as_ref().values as *const FlagsValue,
                self.0.as_ref().n_values as usize,
            )
        }
    }

    // rustdoc-stripper-ignore-next
    /// Converts integer `value` to a `Value`, if part of the flags.
    pub fn to_value(&self, value: u32) -> Option<Value> {
        self.value(value).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Converts string name `name` to a `Value`, if part of the flags.
    pub fn to_value_by_name(&self, name: &str) -> Option<Value> {
        self.value_by_name(name).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Converts string nick `nick` to a `Value`, if part of the flags.
    pub fn to_value_by_nick(&self, nick: &str) -> Option<Value> {
        self.value_by_nick(nick).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the flags corresponding to integer `f` is set in `value`.
    pub fn is_set(&self, value: &Value, f: u32) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
            flags & f != 0
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the flags corresponding to string name `name` is set in `value`.
    pub fn is_set_by_name(&self, value: &Value, name: &str) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            if let Some(f) = self.value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                flags & f.value() != 0
            } else {
                false
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the flags corresponding to string nick `nick` is set in `value`.
    pub fn is_set_by_nick(&self, value: &Value, nick: &str) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            if let Some(f) = self.value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                flags & f.value() != 0
            } else {
                false
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Set flags value corresponding to integer `f` in `value`, if part of that flags. If the
    /// flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    #[doc(alias = "g_value_set_flags")]
    pub fn set(&self, mut value: Value, f: u32) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value(f) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Set flags value corresponding to string name `name` in `value`, if part of that flags.
    /// If the flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn set_by_name(&self, mut value: Value, name: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Set flags value corresponding to string nick `nick` in `value`, if part of that flags.
    /// If the flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn set_by_nick(&self, mut value: Value, nick: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags value corresponding to integer `f` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset(&self, mut value: Value, f: u32) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value(f) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags value corresponding to string name `name` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset_by_name(&self, mut value: Value, name: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags value corresponding to string nick `nick` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset_by_nick(&self, mut value: Value, nick: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Converts an integer `value` to a string of nicks separated by `|`.
    pub fn to_nick_string(&self, mut value: u32) -> String {
        let mut s = String::new();
        for val in self.values() {
            let v = val.value();
            if v != 0 && (value & v) == v {
                value &= !v;
                if !s.is_empty() {
                    s.push('|');
                }
                s.push_str(val.nick());
            }
        }
        s
    }

    // rustdoc-stripper-ignore-next
    /// Converts a string of nicks `s` separated by `|` to an integer value.
    pub fn from_nick_string(&self, s: &str) -> Result<u32, ParseFlagsError> {
        s.split('|').try_fold(0u32, |acc, flag| {
            self.value_by_nick(flag.trim())
                .map(|v| acc + v.value())
                .ok_or_else(|| ParseFlagsError(flag.to_owned()))
        })
    }

    // rustdoc-stripper-ignore-next
    /// Returns a new `FlagsBuilder` for conveniently setting/unsetting flags
    /// and building a `Value`.
    pub fn builder(&self) -> FlagsBuilder<'_> {
        FlagsBuilder::new(self)
    }

    // rustdoc-stripper-ignore-next
    /// Returns a new `FlagsBuilder` for conveniently setting/unsetting flags
    /// and building a `Value`. The `Value` is initialized with `value`.
    pub fn builder_with_value(&self, value: Value) -> Option<FlagsBuilder<'_>> {
        if self.type_() != value.type_() {
            return None;
        }

        Some(FlagsBuilder::with_value(self, value))
    }

    // rustdoc-stripper-ignore-next
    /// Complete `TypeInfo` for the flags with values.
    /// This is an associated function. A method would result in a stack overflow due to a recurvice call:
    /// callers should first create an `FlagsClass` instance by calling `FlagsClass::with_type()` which indirectly
    /// calls `TypePluginRegisterImpl::register_dynamic_flags()` and `TypePluginImpl::complete_type_info()`
    /// and one of them should call `FlagsClass::with_type()` before calling this method.
    /// `const_static_values` is a reference on a wrapper of a slice of `FlagsValue`.
    /// It must be static to ensure flags values are never dropped, and ensures that slice is terminated
    ///  by an `FlagsValue` with all members being 0, as expected by GLib.
    #[doc(alias = "g_flags_complete_type_info")]
    pub fn complete_type_info(
        type_: Type,
        const_static_values: &'static FlagsValues,
    ) -> Option<TypeInfo> {
        unsafe {
            let is_flags: bool = from_glib(gobject_ffi::g_type_is_a(
                type_.into_glib(),
                gobject_ffi::G_TYPE_FLAGS,
            ));
            if !is_flags {
                return None;
            }

            let info = TypeInfo::default();
            gobject_ffi::g_flags_complete_type_info(
                type_.into_glib(),
                info.as_ptr(),
                const_static_values.to_glib_none().0,
            );
            Some(info)
        }
    }
}

impl Drop for FlagsClass {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_type_class_unref(self.0.as_ptr() as *mut _);
        }
    }
}

impl Clone for FlagsClass {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            Self(ptr::NonNull::new(gobject_ffi::g_type_class_ref(self.type_().into_glib()) as *mut _).unwrap())
        }
    }
}

#[derive(Debug)]
pub struct ParseFlagsError(String);

impl std::error::Error for ParseFlagsError {}

impl fmt::Display for ParseFlagsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown flag: '{}'", self.0)
    }
}

impl ParseFlagsError {
    pub fn flag(&self) -> &str {
        &self.0
    }
}

// rustdoc-stripper-ignore-next
/// Representation of a single flags value of a `FlagsClass`.
#[doc(alias = "GFlagsValue")]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FlagsValue(gobject_ffi::GFlagsValue);

unsafe impl Send for FlagsValue {}
unsafe impl Sync for FlagsValue {}

impl fmt::Debug for FlagsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlagsValue")
            .field("value", &self.value())
            .field("name", &self.name())
            .field("nick", &self.nick())
            .finish()
    }
}

impl FlagsValue {
    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// It is the responsibility of the caller to ensure `GFlagsValue` is
    /// valid.
    pub const unsafe fn unsafe_from(g_value: gobject_ffi::GFlagsValue) -> Self {
        Self(g_value)
    }

    // rustdoc-stripper-ignore-next
    /// Get integer value corresponding to the value.
    #[doc(alias = "get_value")]
    pub fn value(&self) -> u32 {
        self.0.value
    }

    // rustdoc-stripper-ignore-next
    /// Get name corresponding to the value.
    #[doc(alias = "get_name")]
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.value_name).to_str().unwrap() }
    }

    // rustdoc-stripper-ignore-next
    /// Get nick corresponding to the value.
    #[doc(alias = "get_nick")]
    pub fn nick(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.value_nick).to_str().unwrap() }
    }

    // rustdoc-stripper-ignore-next
    /// Convert flags value to a `Value`.
    pub fn to_value(&self, flags: &FlagsClass) -> Value {
        unsafe {
            let mut v = Value::from_type_unchecked(flags.type_());
            gobject_ffi::g_value_set_flags(v.to_glib_none_mut().0, self.0.value);
            v
        }
    }

    // rustdoc-stripper-ignore-next
    /// Convert flags values from a `Value`. This returns all flags that are set.
    pub fn from_value(value: &Value) -> Option<(FlagsClass, Vec<&FlagsValue>)> {
        unsafe {
            let flags_class = FlagsClass::with_type(value.type_())?;
            let mut res = Vec::new();
            let f = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
            for v in flags_class.values() {
                if v.value() & f != 0 {
                    res.push(&*(v as *const FlagsValue));
                }
            }
            Some((flags_class, res))
        }
    }
}

impl PartialEq for FlagsValue {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(&other.value())
    }
}

impl Eq for FlagsValue {}

impl UnsafeFrom<gobject_ffi::GFlagsValue> for FlagsValue {
    unsafe fn unsafe_from(g_value: gobject_ffi::GFlagsValue) -> Self {
        Self::unsafe_from(g_value)
    }
}

// rustdoc-stripper-ignore-next
/// Define the zero value and the associated GLib type.
impl EnumerationValue<FlagsValue> for FlagsValue {
    type GlibType = gobject_ffi::GFlagsValue;
    const ZERO: FlagsValue = unsafe {
        FlagsValue::unsafe_from(gobject_ffi::GFlagsValue {
            value: 0,
            value_name: ptr::null(),
            value_nick: ptr::null(),
        })
    };
}

// rustdoc-stripper-ignore-next
/// Storage of flags values.
pub type FlagsValuesStorage<const N: usize> = EnumerationValuesStorage<FlagsValue, N>;

// rustdoc-stripper-ignore-next
/// Representation of flags values wrapped by `FlagsValuesStorage`
pub type FlagsValues = EnumerationValues<FlagsValue>;

// rustdoc-stripper-ignore-next
/// Builder for conveniently setting/unsetting flags and returning a `Value`.
///
/// Example for getting a flags property, unsetting some flags and setting the updated flags on the
/// object again:
///
/// ```ignore
/// let flags = obj.property("flags").unwrap();
/// let flags_class = FlagsClass::new(flags.type_()).unwrap();
/// let flags = flags_class.builder_with_value(flags).unwrap()
///     .unset_by_nick("some-flag")
///     .unset_by_nick("some-other-flag")
///     .build()
///     .unwrap();
/// obj.set_property("flags", &flags).unwrap();
/// ```
///
/// If setting/unsetting any value fails, `build()` returns `None`.
#[must_use = "The builder must be built to be used"]
pub struct FlagsBuilder<'a>(&'a FlagsClass, Option<Value>);
impl FlagsBuilder<'_> {
    fn new(flags_class: &FlagsClass) -> FlagsBuilder<'_> {
        let value = unsafe { Value::from_type_unchecked(flags_class.type_()) };
        FlagsBuilder(flags_class, Some(value))
    }

    fn with_value(flags_class: &FlagsClass, value: Value) -> FlagsBuilder<'_> {
        FlagsBuilder(flags_class, Some(value))
    }

    // rustdoc-stripper-ignore-next
    /// Set flags corresponding to integer value `f`.
    pub fn set(mut self, f: u32) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set(value, f).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Set flags corresponding to string name `name`.
    pub fn set_by_name(mut self, name: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set_by_name(value, name).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Set flags corresponding to string nick `nick`.
    pub fn set_by_nick(mut self, nick: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set_by_nick(value, nick).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Unsets flags corresponding to integer value `f`.
    pub fn unset(mut self, f: u32) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset(value, f).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags corresponding to string name `name`.
    pub fn unset_by_name(mut self, name: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset_by_name(value, name).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags corresponding to string nick `nick`.
    pub fn unset_by_nick(mut self, nick: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset_by_nick(value, nick).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Converts to the final `Value`, unless any previous setting/unsetting of flags failed.
    #[must_use = "Value returned from the builder should probably be used"]
    pub fn build(self) -> Option<Value> {
        self.1
    }
}

unsafe impl<'a> crate::value::FromValue<'a> for Vec<&FlagsValue> {
    type Checker = FlagsTypeChecker;

    unsafe fn from_value(value: &'a Value) -> Self {
        let (_, v) = FlagsValue::from_value(value).unwrap();
        // SAFETY: The enum class and its values live forever
        std::mem::transmute(v)
    }
}

pub struct FlagsTypeChecker();
unsafe impl crate::value::ValueTypeChecker for FlagsTypeChecker {
    type Error = InvalidFlagsError;

    fn check(value: &Value) -> Result<(), Self::Error> {
        let t = value.type_();
        if t.is_a(Type::FLAGS) {
            Ok(())
        } else {
            Err(InvalidFlagsError)
        }
    }
}

// rustdoc-stripper-ignore-next
/// An error returned from the [`get`](struct.Value.html#method.get) function
/// on a [`Value`](struct.Value.html) for flags types.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InvalidFlagsError;

impl fmt::Display for InvalidFlagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value is not a flags")
    }
}

impl std::error::Error for InvalidFlagsError {}

// rustdoc-stripper-ignore-next
/// helper trait to define the zero value and the associated GLib type.
pub trait EnumerationValue<E>: Copy {
    type GlibType;
    const ZERO: E;
}

// rustdoc-stripper-ignore-next
/// Storage of enumeration values terminated by a zero value. Should be used
/// only as a storage location for `EnumValue` or `FlagsValue` when registering
/// an enum or flags as a dynamic type.
/// see `TypePluginRegisterImpl::register_dynamic_enum()`, `TypePluginRegisterImpl::register_dynamic_flags()`
/// and `TypePluginImpl::complete_type_info()`.
/// Inner is intentionally private to ensure other modules will not access the
/// enum (or flags) values by this way.
/// Use `EnumClass::values()` or `EnumClass::value()` to get the enum values.
/// Use `FlagsClass::values()` or `FlagsClass::value()` to get the flags values.
#[repr(C)]
pub struct EnumerationValuesStorage<E: EnumerationValue<E>, const S: usize>([E; S]);

impl<E: EnumerationValue<E>, const S: usize> EnumerationValuesStorage<E, S> {
    // rustdoc-stripper-ignore-next
    /// creates a new `EnumerationValuesStorage` with the given values and a final zero value.
    pub const fn new<const N: usize>(values: [E; N]) -> Self {
        #[repr(C)]
        #[derive(Copy, Clone)]
        struct Both<E: Copy, const N: usize>([E; N], [E; 1]);

        #[repr(C)]
        union Transmute<E: Copy, const N: usize, const S: usize> {
            from: Both<E, N>,
            to: [E; S],
        }

        // SAFETY: Transmute is repr(C) and union fields are compatible in terms of size and alignment, so the access to union fields is safe.
        unsafe {
            // create an array with the values and terminated by a zero value.
            let all = Transmute {
                from: Both(values, [E::ZERO; 1]),
            }
            .to;
            Self(all)
        }
    }
}

impl<E: EnumerationValue<E>, const S: usize> AsRef<EnumerationValues<E>>
    for EnumerationValuesStorage<E, S>
{
    fn as_ref(&self) -> &EnumerationValues<E> {
        // SAFETY: EnumerationStorage and EnumerationValues are repr(C) and their unique field are compatible (array and slice of the same type), so the cast is safe.
        unsafe { &*(&self.0 as *const [E] as *const EnumerationValues<E>) }
    }
}

// rustdoc-stripper-ignore-next
/// Representation of enumeration values wrapped by `EnumerationValuesStorage`.
/// Easier to use because don't have a size parameter to be specify. Should be
/// used only to register an enum or flags as a dynamic type.
/// see `TypePluginRegisterImpl::register_dynamic_enum()`, `TypePluginRegisterImpl::register_dynamic_flags()`
/// and `TypePluginImpl::complete_type_info()`.
/// Field is intentionally private to ensure other modules will not access the
/// enum (or flags) values by this way.
/// Use `EnumClass::values()` or `EnumClass::value()` to get the enum values.
/// Use `FlagsClass::values()` or `FlagsClass::value()` to get the flags values.
#[repr(C)]
pub struct EnumerationValues<E: EnumerationValue<E>>([E]);

impl<E: EnumerationValue<E>> Deref for EnumerationValues<E> {
    type Target = [E];

    // rustdoc-stripper-ignore-next
    /// Dereferences the enumeration values as a slice, but excluding the last value which is zero.
    fn deref(&self) -> &Self::Target {
        // SAFETY: EnumerationValues contains at least the zero value which terminates the array.
        unsafe { std::slice::from_raw_parts(self.0.as_ptr(), self.0.len() - 1) }
    }
}

#[doc(hidden)]
impl<'a, E: 'a + EnumerationValue<E>> ToGlibPtr<'a, *const E::GlibType> for EnumerationValues<E> {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const E::GlibType, Self> {
        Stash(self.0.as_ptr() as *const E::GlibType, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags() {
        let flags = FlagsClass::new::<crate::BindingFlags>();
        let values = flags.values();
        let def1 = values
            .iter()
            .find(|v| v.name() == "G_BINDING_DEFAULT")
            .unwrap();
        let def2 = flags.value_by_name("G_BINDING_DEFAULT").unwrap();
        assert!(ptr::eq(def1, def2));

        let value = flags.to_value(0).unwrap();
        let values = value.get::<Vec<&FlagsValue>>().unwrap();
        assert_eq!(values.len(), 0);

        assert_eq!(def1.value(), crate::BindingFlags::DEFAULT.bits());
    }
}
