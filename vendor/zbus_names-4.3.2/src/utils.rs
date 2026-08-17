macro_rules! impl_str_basic {
    ($type:ty) => {
        impl zvariant::Basic for $type {
            const SIGNATURE_CHAR: char = <zvariant::Str<'_>>::SIGNATURE_CHAR;
            const SIGNATURE_STR: &'static str = <zvariant::Str<'_>>::SIGNATURE_STR;
        }
    };
}

/// Generates all boilerplate code for a D-Bus name type and its owned variant.
///
/// This macro generates all the boilerplate code for a D-Bus name type and its owned variant.
///
/// # Parameters
/// - `$name`: The name of the borrowed type (e.g., `InterfaceName`).
/// - `$owned_name`: The name of the owned type (e.g., `OwnedInterfaceName`).
/// - `$validate_fn`: The validation function to use.
macro_rules! define_name_type_impls {
    (
        name: $name:ident,
        owned: $owned_name:ident,
        validate: $validate_fn:ident $(,)?
    ) => {
        // === impl_str_basic for borrowed type ===
        impl zvariant::Basic for $name<'_> {
            const SIGNATURE_CHAR: char = <zvariant::Str<'_>>::SIGNATURE_CHAR;
            const SIGNATURE_STR: &'static str = <zvariant::Str<'_>>::SIGNATURE_STR;
        }

        impl<'name> $name<'name> {
            /// This is faster than `Clone::clone` when `self` contains owned data.
            pub fn as_ref(&self) -> $name<'_> {
                $name(self.0.as_ref())
            }

            /// The name as string.
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            /// Create a new name from the given string.
            ///
            /// Since the passed string is not checked for correctness, prefer using the
            /// `TryFrom<&str>` implementation.
            pub fn from_str_unchecked(name: &'name str) -> Self {
                Self(zvariant::Str::from(name))
            }

            /// Same as `try_from`, except it takes a `&'static str`.
            pub fn from_static_str(name: &'static str) -> crate::Result<Self> {
                $validate_fn(name)?;
                Ok(Self(zvariant::Str::from_static(name)))
            }

            /// Same as `from_str_unchecked`, except it takes a `&'static str`.
            pub const fn from_static_str_unchecked(name: &'static str) -> Self {
                Self(zvariant::Str::from_static(name))
            }

            /// Same as `from_str_unchecked`, except it takes an owned `String`.
            ///
            /// Since the passed string is not checked for correctness, prefer using the
            /// `TryFrom<String>` implementation.
            pub fn from_string_unchecked(name: String) -> Self {
                Self(zvariant::Str::from(name))
            }

            /// Creates an owned clone of `self`.
            pub fn to_owned(&self) -> $name<'static> {
                $name(self.0.to_owned())
            }

            /// Creates an owned clone of `self`.
            pub fn into_owned(self) -> $name<'static> {
                $name(self.0.into_owned())
            }
        }

        impl std::ops::Deref for $name<'_> {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl std::borrow::Borrow<str> for $name<'_> {
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.as_str(), f)
            }
        }

        impl PartialEq<str> for $name<'_> {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $name<'_> {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<$owned_name> for $name<'_> {
            fn eq(&self, other: &$owned_name) -> bool {
                *self == other.0
            }
        }

        impl<'de: 'name, 'name> serde::Deserialize<'de> for $name<'name> {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let name = <std::borrow::Cow<'name, str>>::deserialize(deserializer)?;

                Self::try_from(name).map_err(|e| serde::de::Error::custom(e.to_string()))
            }
        }

        /// This never succeeds but is provided so it's easier to pass `Option::None` values for API
        /// requiring `Option<TryInto<impl BusName>>`, since type inference won't work here.
        impl TryFrom<()> for $name<'_> {
            type Error = crate::Error;

            fn try_from(_value: ()) -> crate::Result<Self> {
                unreachable!("Conversion from `()` is not meant to actually work");
            }
        }

        impl<'name> From<&$name<'name>> for $name<'name> {
            fn from(name: &$name<'name>) -> Self {
                name.clone()
            }
        }

        impl<'name> From<$name<'name>> for zvariant::Str<'name> {
            fn from(value: $name<'name>) -> Self {
                value.0
            }
        }

        impl<'name> zvariant::NoneValue for $name<'name> {
            type NoneType = &'name str;

            fn null_value() -> Self::NoneType {
                <&str>::default()
            }
        }

        // === TryFrom impls for borrowed type ===
        impl<'s> TryFrom<&'s str> for $name<'s> {
            type Error = crate::Error;

            fn try_from(value: &'s str) -> crate::Result<Self> {
                let value = zvariant::Str::from(value);
                $validate_fn(value.as_str())?;
                Ok(Self(value))
            }
        }

        impl<'s> TryFrom<&'s str> for $owned_name {
            type Error = crate::Error;

            fn try_from(value: &'s str) -> crate::Result<Self> {
                Ok(Self::from(<$name<'s>>::try_from(value)?))
            }
        }

        impl TryFrom<String> for $name<'_> {
            type Error = crate::Error;

            fn try_from(value: String) -> crate::Result<Self> {
                let value = zvariant::Str::from(value);
                $validate_fn(value.as_str())?;
                Ok(Self(value))
            }
        }

        impl TryFrom<String> for $owned_name {
            type Error = crate::Error;

            fn try_from(value: String) -> crate::Result<Self> {
                Ok(Self::from(<$name<'_>>::try_from(value)?))
            }
        }

        impl TryFrom<std::sync::Arc<str>> for $name<'_> {
            type Error = crate::Error;

            fn try_from(value: std::sync::Arc<str>) -> crate::Result<Self> {
                let value = zvariant::Str::from(value);
                $validate_fn(value.as_str())?;
                Ok(Self(value))
            }
        }

        impl TryFrom<std::sync::Arc<str>> for $owned_name {
            type Error = crate::Error;

            fn try_from(value: std::sync::Arc<str>) -> crate::Result<Self> {
                Ok(Self::from(<$name<'_>>::try_from(value)?))
            }
        }

        impl<'s> TryFrom<std::borrow::Cow<'s, str>> for $name<'s> {
            type Error = crate::Error;

            fn try_from(value: std::borrow::Cow<'s, str>) -> crate::Result<Self> {
                let value = zvariant::Str::from(value);
                $validate_fn(value.as_str())?;
                Ok(Self(value))
            }
        }

        impl<'s> TryFrom<std::borrow::Cow<'s, str>> for $owned_name {
            type Error = crate::Error;

            fn try_from(value: std::borrow::Cow<'s, str>) -> crate::Result<Self> {
                Ok(Self::from(<$name<'s>>::try_from(value)?))
            }
        }

        impl<'s> TryFrom<zvariant::Str<'s>> for $name<'s> {
            type Error = crate::Error;

            fn try_from(value: zvariant::Str<'s>) -> crate::Result<Self> {
                $validate_fn(value.as_str())?;
                Ok(Self(value))
            }
        }

        impl<'s> TryFrom<zvariant::Str<'s>> for $owned_name {
            type Error = crate::Error;

            fn try_from(value: zvariant::Str<'s>) -> crate::Result<Self> {
                Ok(Self::from(<$name<'s>>::try_from(value)?))
            }
        }

        // === Owned type impls ===

        // impl_str_basic for owned type
        impl zvariant::Basic for $owned_name {
            const SIGNATURE_CHAR: char = <zvariant::Str<'_>>::SIGNATURE_CHAR;
            const SIGNATURE_STR: &'static str = <zvariant::Str<'_>>::SIGNATURE_STR;
        }

        impl $owned_name {
            /// Convert to the inner type, consuming `self`.
            pub fn into_inner(self) -> $name<'static> {
                self.0
            }

            /// Get a reference to the inner type.
            pub fn inner(&self) -> &$name<'static> {
                &self.0
            }

            /// This is faster than `Clone::clone` when `self` contains owned data.
            pub fn as_ref(&self) -> $name<'_> {
                self.0.as_ref()
            }
        }

        impl std::ops::Deref for $owned_name {
            type Target = $name<'static>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'a> std::borrow::Borrow<$name<'a>> for $owned_name {
            fn borrow(&self) -> &$name<'a> {
                &self.0
            }
        }

        impl std::borrow::Borrow<str> for $owned_name {
            fn borrow(&self) -> &str {
                self.0.as_str()
            }
        }

        impl AsRef<str> for $owned_name {
            fn as_ref(&self) -> &str {
                self.0.as_str()
            }
        }

        impl std::fmt::Debug for $owned_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($owned_name))
                    .field(&self.as_str())
                    .finish()
            }
        }

        impl std::fmt::Display for $owned_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&$name::from(self), f)
            }
        }

        impl From<$owned_name> for $name<'_> {
            fn from(name: $owned_name) -> Self {
                name.into_inner()
            }
        }

        impl<'unowned, 'owned: 'unowned> From<&'owned $owned_name> for $name<'unowned> {
            fn from(name: &'owned $owned_name) -> Self {
                $name::from_str_unchecked(name.as_str())
            }
        }

        impl From<$name<'_>> for $owned_name {
            fn from(name: $name<'_>) -> Self {
                $owned_name(name.into_owned())
            }
        }

        impl From<$owned_name> for zvariant::Str<'_> {
            fn from(value: $owned_name) -> Self {
                value.into_inner().0
            }
        }

        impl<'de> serde::Deserialize<'de> for $owned_name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                String::deserialize(deserializer)
                    .and_then(|n| {
                        $name::try_from(n).map_err(|e| serde::de::Error::custom(e.to_string()))
                    })
                    .map(Self)
            }
        }

        impl PartialEq<&str> for $owned_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<$name<'_>> for $owned_name {
            fn eq(&self, other: &$name<'_>) -> bool {
                self.0 == *other
            }
        }

        impl zvariant::NoneValue for $owned_name {
            type NoneType = <$name<'static> as zvariant::NoneValue>::NoneType;

            fn null_value() -> Self::NoneType {
                $name::null_value()
            }
        }
    };
}

pub(crate) use define_name_type_impls;
pub(crate) use impl_str_basic;
