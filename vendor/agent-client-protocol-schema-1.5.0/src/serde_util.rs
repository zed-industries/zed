//! Custom option-like field wrappers and builder helpers for serde.
//!
//! ## Types
//!
//! - [`MaybeUndefined<T>`] — three-state: undefined (key absent), null, or value.
//! - [`SkipListener`] — [`serde_with::InspectError`] hook used by every
//!   `VecSkipError` call site in the protocol types.
//!
//! ## Builder traits
//!
//! - [`IntoOption<T>`] — ergonomic conversion into `Option<T>` for builder methods.
//! - [`IntoMaybeUndefined<T>`] — ergonomic conversion into `MaybeUndefined<T>` for builder methods.
//!
//! `MaybeUndefined` based on: <https://docs.rs/async-graphql/latest/src/async_graphql/types/maybe_undefined.rs.html>
use std::{
    borrow::Cow,
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, de::DeserializeAsWrap};

// ---- SkipListener ----

/// Inspector passed to every `VecSkipError<_, SkipListener>` in the protocol
/// types so that malformed list entries dropped during deserialization are
/// surfaced to observability tooling rather than vanishing silently.
///
/// - With the `tracing` feature enabled, this is a zero-sized type whose
///   [`InspectError`](serde_with::InspectError) implementation emits a
///   [`tracing::warn!`] event on every skipped entry.
/// - With the feature disabled (the default), it resolves to `()` — which
///   `serde_with` ships with a no-op `InspectError` implementation — so call
///   sites incur zero runtime cost.
#[cfg(feature = "tracing")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) struct SkipListener;

#[cfg(feature = "tracing")]
impl serde_with::InspectError for SkipListener {
    fn inspect_error(error: impl serde::de::Error) {
        tracing::warn!(
            %error,
            "skipped malformed list entry during deserialization",
        );
    }
}

/// Zero-cost stand-in for [`SkipListener`] when the `tracing` feature is
/// disabled. Resolves to `()`, which `serde_with` already ships with a no-op
/// `InspectError` implementation.
#[cfg(not(feature = "tracing"))]
pub(crate) type SkipListener = ();

// ---- DefaultTrueOnError ----

#[cfg(any(feature = "unstable_auth_methods", test))]
#[derive(Deserialize)]
#[serde(transparent)]
struct BoolDefaultTrue(bool);

#[cfg(any(feature = "unstable_auth_methods", test))]
impl Default for BoolDefaultTrue {
    fn default() -> Self {
        Self(true)
    }
}

/// Deserializes a boolean, falling back to `true` when the input is malformed.
#[cfg(any(feature = "unstable_auth_methods", test))]
pub(crate) struct DefaultTrueOnError;

#[cfg(any(feature = "unstable_auth_methods", test))]
impl<'de> DeserializeAs<'de, bool> for DefaultTrueOnError {
    fn deserialize_as<D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        <serde_with::DefaultOnError as DeserializeAs<'de, BoolDefaultTrue>>::deserialize_as(
            deserializer,
        )
        .map(|value| value.0)
    }
}

#[cfg(test)]
mod skip_listener_tests {
    use std::cell::Cell;

    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use serde_with::{DefaultOnError, VecSkipError, serde_as};

    thread_local! {
        static SKIP_COUNT: Cell<u32> = const { Cell::new(0) };
    }

    /// Test-only inspector that counts skipped entries.
    struct CountingListener;

    impl serde_with::InspectError for CountingListener {
        fn inspect_error(_error: impl serde::de::Error) {
            SKIP_COUNT.with(|c| c.set(c.get() + 1));
        }
    }

    #[serde_as]
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Wrapper {
        #[serde_as(deserialize_as = "VecSkipError<_, CountingListener>")]
        values: Vec<u32>,
    }

    #[test]
    fn inspector_runs_for_each_skipped_entry() {
        SKIP_COUNT.with(|c| c.set(0));

        let input = json!({"values": [1, "oops", 2, {}, 3]});
        let wrapper: Wrapper = serde_json::from_value(input).unwrap();

        assert_eq!(wrapper.values, vec![1, 2, 3]);
        assert_eq!(SKIP_COUNT.with(Cell::get), 2);
    }

    /// Mirrors the pattern applied to every required `Vec<T>` field in the
    /// protocol: `DefaultOnError<VecSkipError<_, ...>>` + `#[serde(default)]`.
    /// Element-level failures are skipped; any outer shape error (`null`, a
    /// string, a map, etc.) collapses to `Default::default()` (i.e. `vec![]`).
    #[serde_as]
    #[derive(Deserialize, Debug, PartialEq)]
    struct ResilientVec {
        #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, CountingListener>>")]
        #[serde(default)]
        values: Vec<u32>,
    }

    #[test]
    fn resilient_vec_tolerates_missing_null_and_wrong_type() {
        // Missing field -> `#[serde(default)]` supplies `vec![]`.
        let r: ResilientVec = serde_json::from_value(json!({})).unwrap();
        assert_eq!(r.values, Vec::<u32>::new());

        // Explicit null -> `DefaultOnError` swallows the type error.
        let r: ResilientVec = serde_json::from_value(json!({"values": null})).unwrap();
        assert_eq!(r.values, Vec::<u32>::new());

        // Wrong outer type (string) -> `DefaultOnError` swallows.
        let r: ResilientVec = serde_json::from_value(json!({"values": "oops"})).unwrap();
        assert_eq!(r.values, Vec::<u32>::new());

        // Wrong outer type (object) -> `DefaultOnError` swallows.
        let r: ResilientVec = serde_json::from_value(json!({"values": {"k": 1}})).unwrap();
        assert_eq!(r.values, Vec::<u32>::new());

        // Valid array with element errors -> `VecSkipError` skips per-element.
        SKIP_COUNT.with(|c| c.set(0));
        let r: ResilientVec =
            serde_json::from_value(json!({"values": [1, "oops", 2, {}, 3]})).unwrap();
        assert_eq!(r.values, vec![1, 2, 3]);
        assert_eq!(SKIP_COUNT.with(Cell::get), 2);
    }

    #[test]
    fn resilient_vec_does_not_invoke_inspector_on_outer_failure() {
        SKIP_COUNT.with(|c| c.set(0));

        // Outer failures are swallowed silently by `DefaultOnError`; the
        // inspector only sees per-element failures inside a valid array.
        let _r: ResilientVec = serde_json::from_value(json!({"values": null})).unwrap();
        let _r: ResilientVec = serde_json::from_value(json!({"values": "oops"})).unwrap();
        let _r: ResilientVec = serde_json::from_value(json!({"values": {}})).unwrap();

        assert_eq!(SKIP_COUNT.with(Cell::get), 0);
    }

    /// Mirrors the pattern applied to every optional `Option<Vec<T>>` field:
    /// `DefaultOnError<Option<VecSkipError<_, ...>>>` + `#[serde(default)]`.
    /// `null` becomes `None`; outer shape errors also collapse to `None`;
    /// element-level failures are skipped inside the array.
    #[serde_as]
    #[derive(Deserialize, Debug, PartialEq)]
    struct ResilientOptionVec {
        #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, CountingListener>>>")]
        #[serde(default)]
        values: Option<Vec<u32>>,
    }

    #[test]
    fn resilient_option_vec_tolerates_missing_null_and_wrong_type() {
        // Missing field -> `None`.
        let r: ResilientOptionVec = serde_json::from_value(json!({})).unwrap();
        assert_eq!(r.values, None);

        // Explicit null -> `None`.
        let r: ResilientOptionVec = serde_json::from_value(json!({"values": null})).unwrap();
        assert_eq!(r.values, None);

        // Empty array -> `Some(vec![])`.
        let r: ResilientOptionVec = serde_json::from_value(json!({"values": []})).unwrap();
        assert_eq!(r.values, Some(Vec::<u32>::new()));

        // Valid array -> `Some(vec)`.
        let r: ResilientOptionVec = serde_json::from_value(json!({"values": [1, 2, 3]})).unwrap();
        assert_eq!(r.values, Some(vec![1, 2, 3]));

        // Wrong outer type (string) -> `DefaultOnError` collapses to `None`.
        let r: ResilientOptionVec = serde_json::from_value(json!({"values": "oops"})).unwrap();
        assert_eq!(r.values, None);

        // Wrong outer type (object) -> `DefaultOnError` collapses to `None`.
        let r: ResilientOptionVec = serde_json::from_value(json!({"values": {"k": 1}})).unwrap();
        assert_eq!(r.values, None);

        // Valid array with element errors -> `VecSkipError` skips per-element.
        SKIP_COUNT.with(|c| c.set(0));
        let r: ResilientOptionVec =
            serde_json::from_value(json!({"values": [1, "oops", 2, {}, 3]})).unwrap();
        assert_eq!(r.values, Some(vec![1, 2, 3]));
        assert_eq!(SKIP_COUNT.with(Cell::get), 2);
    }

    fn default_true() -> bool {
        true
    }

    #[serde_as]
    #[derive(Deserialize, Debug, PartialEq)]
    struct DefaultTrueWrapper {
        #[serde_as(deserialize_as = "super::DefaultTrueOnError")]
        #[serde(default = "default_true")]
        value: bool,
    }

    #[test]
    fn default_true_on_error_uses_true_for_missing_and_malformed_values() {
        let wrapper: DefaultTrueWrapper = serde_json::from_value(json!({})).unwrap();
        assert!(wrapper.value);

        let wrapper: DefaultTrueWrapper = serde_json::from_value(json!({"value": false})).unwrap();
        assert!(!wrapper.value);

        let wrapper: DefaultTrueWrapper = serde_json::from_value(json!({"value": "oops"})).unwrap();
        assert!(wrapper.value);
    }
}

// ---- IntoOption ----

/// Utility trait for builder methods for optional values.
/// This allows the caller to either pass in the value itself without wrapping it in `Some`,
/// or to just pass in an Option if that is what they have.
pub trait IntoOption<T> {
    /// Converts this value into an optional builder argument.
    fn into_option(self) -> Option<T>;
}

impl<T> IntoOption<T> for Option<T> {
    fn into_option(self) -> Option<T> {
        self
    }
}

impl<T> IntoOption<T> for T {
    fn into_option(self) -> Option<T> {
        Some(self)
    }
}

impl IntoOption<String> for &str {
    fn into_option(self) -> Option<String> {
        Some(self.into())
    }
}

impl IntoOption<String> for &mut str {
    fn into_option(self) -> Option<String> {
        Some(self.into())
    }
}

impl IntoOption<String> for &String {
    fn into_option(self) -> Option<String> {
        Some(self.into())
    }
}

impl IntoOption<String> for Box<str> {
    fn into_option(self) -> Option<String> {
        Some(self.into())
    }
}

impl IntoOption<String> for Cow<'_, str> {
    fn into_option(self) -> Option<String> {
        Some(self.into())
    }
}

impl IntoOption<String> for Arc<str> {
    fn into_option(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<T: ?Sized + AsRef<OsStr>> IntoOption<PathBuf> for &T {
    fn into_option(self) -> Option<PathBuf> {
        Some(self.into())
    }
}

impl IntoOption<PathBuf> for Box<Path> {
    fn into_option(self) -> Option<PathBuf> {
        Some(self.into())
    }
}

impl IntoOption<PathBuf> for Cow<'_, Path> {
    fn into_option(self) -> Option<PathBuf> {
        Some(self.into())
    }
}

impl IntoOption<serde_json::Value> for &str {
    fn into_option(self) -> Option<serde_json::Value> {
        Some(self.into())
    }
}

impl IntoOption<serde_json::Value> for String {
    fn into_option(self) -> Option<serde_json::Value> {
        Some(self.into())
    }
}

impl IntoOption<serde_json::Value> for Cow<'_, str> {
    fn into_option(self) -> Option<serde_json::Value> {
        Some(self.into())
    }
}

// ---- MaybeUndefined ----

/// Similar to `Option`, but it has three states, `undefined`, `null` and `x`.
///
/// When using with Serde, you will likely want to skip serialization of `undefined`
/// and add a `default` for deserialization.
///
/// # Example
///
/// ```rust
/// use agent_client_protocol_schema::MaybeUndefined;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
/// struct A {
///     #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
///     a: MaybeUndefined<i32>,
/// }
/// ```
#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, JsonSchema)]
#[schemars(with = "Option<Option<T>>", inline)]
#[expect(clippy::exhaustive_enums)]
pub enum MaybeUndefined<T> {
    /// The field was not present.
    #[default]
    Undefined,
    /// The field was present with a JSON `null` value.
    Null,
    /// The field was present with a non-null value.
    Value(T),
}

impl<T> MaybeUndefined<T> {
    /// Returns true if the `MaybeUndefined<T>` is undefined.
    #[inline]
    pub const fn is_undefined(&self) -> bool {
        matches!(self, MaybeUndefined::Undefined)
    }

    /// Returns true if the `MaybeUndefined<T>` is null.
    #[inline]
    pub const fn is_null(&self) -> bool {
        matches!(self, MaybeUndefined::Null)
    }

    /// Returns true if the `MaybeUndefined<T>` contains value.
    #[inline]
    pub const fn is_value(&self) -> bool {
        matches!(self, MaybeUndefined::Value(_))
    }

    /// Borrow the value, returns `None` if the `MaybeUndefined<T>` is
    /// `undefined` or `null`, otherwise returns `Some(T)`.
    #[inline]
    pub const fn value(&self) -> Option<&T> {
        match self {
            MaybeUndefined::Value(value) => Some(value),
            _ => None,
        }
    }

    /// Converts the `MaybeUndefined<T>` to `Option<T>`.
    #[inline]
    pub fn take(self) -> Option<T> {
        match self {
            MaybeUndefined::Value(value) => Some(value),
            _ => None,
        }
    }

    /// Converts the `MaybeUndefined<T>` to `Option<Option<T>>`.
    #[inline]
    pub const fn as_opt_ref(&self) -> Option<Option<&T>> {
        match self {
            MaybeUndefined::Undefined => None,
            MaybeUndefined::Null => Some(None),
            MaybeUndefined::Value(value) => Some(Some(value)),
        }
    }

    /// Converts the `MaybeUndefined<T>` to `Option<Option<&U>>`.
    #[inline]
    pub fn as_opt_deref<U>(&self) -> Option<Option<&U>>
    where
        U: ?Sized,
        T: Deref<Target = U>,
    {
        match self {
            MaybeUndefined::Undefined => None,
            MaybeUndefined::Null => Some(None),
            MaybeUndefined::Value(value) => Some(Some(&**value)),
        }
    }

    /// Returns `true` if the `MaybeUndefined<T>` contains the given value.
    #[inline]
    pub fn contains_value<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            MaybeUndefined::Value(y) => x == y,
            _ => false,
        }
    }

    /// Returns `true` if the `MaybeUndefined<T>` contains the given nullable
    /// value.
    #[inline]
    pub fn contains<U>(&self, x: Option<&U>) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            MaybeUndefined::Value(y) => matches!(x, Some(v) if v == y),
            MaybeUndefined::Null => x.is_none(),
            MaybeUndefined::Undefined => false,
        }
    }

    /// Maps a `MaybeUndefined<T>` to `MaybeUndefined<U>` by applying a function
    /// to the contained nullable value
    #[inline]
    pub fn map<U, F: FnOnce(Option<T>) -> Option<U>>(self, f: F) -> MaybeUndefined<U> {
        match self {
            MaybeUndefined::Value(v) => match f(Some(v)) {
                Some(v) => MaybeUndefined::Value(v),
                None => MaybeUndefined::Null,
            },
            MaybeUndefined::Null => match f(None) {
                Some(v) => MaybeUndefined::Value(v),
                None => MaybeUndefined::Null,
            },
            MaybeUndefined::Undefined => MaybeUndefined::Undefined,
        }
    }

    /// Maps a `MaybeUndefined<T>` to `MaybeUndefined<U>` by applying a function
    /// to the contained value
    #[inline]
    pub fn map_value<U, F: FnOnce(T) -> U>(self, f: F) -> MaybeUndefined<U> {
        match self {
            MaybeUndefined::Value(v) => MaybeUndefined::Value(f(v)),
            MaybeUndefined::Null => MaybeUndefined::Null,
            MaybeUndefined::Undefined => MaybeUndefined::Undefined,
        }
    }

    /// Update `value` if the `MaybeUndefined<T>` is not undefined.
    ///
    /// # Example
    ///
    /// ```rust
    /// use agent_client_protocol_schema::MaybeUndefined;
    ///
    /// let mut value = None;
    ///
    /// MaybeUndefined::Value(10i32).update_to(&mut value);
    /// assert_eq!(value, Some(10));
    ///
    /// MaybeUndefined::Undefined.update_to(&mut value);
    /// assert_eq!(value, Some(10));
    ///
    /// MaybeUndefined::Null.update_to(&mut value);
    /// assert_eq!(value, None);
    /// ```
    pub fn update_to(self, value: &mut Option<T>) {
        match self {
            MaybeUndefined::Value(new) => *value = Some(new),
            MaybeUndefined::Null => *value = None,
            MaybeUndefined::Undefined => {}
        }
    }
}

impl<T, E> MaybeUndefined<Result<T, E>> {
    /// Transposes a `MaybeUndefined` of a [`Result`] into a [`Result`] of a
    /// `MaybeUndefined`.
    ///
    /// [`MaybeUndefined::Undefined`] will be mapped to
    /// [`Ok`]`(`[`MaybeUndefined::Undefined`]`)`. [`MaybeUndefined::Null`]
    /// will be mapped to [`Ok`]`(`[`MaybeUndefined::Null`]`)`.
    /// [`MaybeUndefined::Value`]`(`[`Ok`]`(_))` and
    /// [`MaybeUndefined::Value`]`(`[`Err`]`(_))` will be mapped to
    /// [`Ok`]`(`[`MaybeUndefined::Value`]`(_))` and [`Err`]`(_)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is [`MaybeUndefined::Value`]`(`[`Err`]`(_))`.
    #[inline]
    pub fn transpose(self) -> Result<MaybeUndefined<T>, E> {
        match self {
            MaybeUndefined::Undefined => Ok(MaybeUndefined::Undefined),
            MaybeUndefined::Null => Ok(MaybeUndefined::Null),
            MaybeUndefined::Value(Ok(v)) => Ok(MaybeUndefined::Value(v)),
            MaybeUndefined::Value(Err(e)) => Err(e),
        }
    }
}

impl<T: Serialize> Serialize for MaybeUndefined<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MaybeUndefined::Value(value) => value.serialize(serializer),
            MaybeUndefined::Null => serializer.serialize_none(),
            MaybeUndefined::Undefined => serializer.serialize_unit(),
        }
    }
}

impl<'de, T> Deserialize<'de> for MaybeUndefined<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<MaybeUndefined<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => MaybeUndefined::Value(value),
            None => MaybeUndefined::Null,
        })
    }
}

impl<T> From<MaybeUndefined<T>> for Option<Option<T>> {
    fn from(maybe_undefined: MaybeUndefined<T>) -> Self {
        match maybe_undefined {
            MaybeUndefined::Undefined => None,
            MaybeUndefined::Null => Some(None),
            MaybeUndefined::Value(value) => Some(Some(value)),
        }
    }
}

impl<T> From<Option<Option<T>>> for MaybeUndefined<T> {
    fn from(value: Option<Option<T>>) -> Self {
        match value {
            Some(Some(value)) => Self::Value(value),
            Some(None) => Self::Null,
            None => Self::Undefined,
        }
    }
}

impl<'de, T, TAs> DeserializeAs<'de, MaybeUndefined<T>> for MaybeUndefined<TAs>
where
    TAs: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<MaybeUndefined<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<DeserializeAsWrap<T, TAs>>::deserialize(deserializer).map(|value| match value {
            Some(value) => MaybeUndefined::Value(value.into_inner()),
            None => MaybeUndefined::Null,
        })
    }
}

/// Utility trait for builder methods for optional values.
/// This allows the caller to either pass in the value itself without wrapping it in `Some`,
/// or to just pass in an Option if that is what they have, or set it back to undefined.
pub trait IntoMaybeUndefined<T> {
    /// Converts this value into a three-state builder argument.
    fn into_maybe_undefined(self) -> MaybeUndefined<T>;
}

impl<T> IntoMaybeUndefined<T> for T {
    fn into_maybe_undefined(self) -> MaybeUndefined<T> {
        MaybeUndefined::Value(self)
    }
}

impl<T> IntoMaybeUndefined<T> for Option<T> {
    fn into_maybe_undefined(self) -> MaybeUndefined<T> {
        match self {
            Some(value) => MaybeUndefined::Value(value),
            None => MaybeUndefined::Null,
        }
    }
}

impl<T> IntoMaybeUndefined<T> for MaybeUndefined<T> {
    fn into_maybe_undefined(self) -> MaybeUndefined<T> {
        self
    }
}

impl IntoMaybeUndefined<String> for &str {
    fn into_maybe_undefined(self) -> MaybeUndefined<String> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<String> for &mut str {
    fn into_maybe_undefined(self) -> MaybeUndefined<String> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<String> for &String {
    fn into_maybe_undefined(self) -> MaybeUndefined<String> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<String> for Box<str> {
    fn into_maybe_undefined(self) -> MaybeUndefined<String> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<String> for Cow<'_, str> {
    fn into_maybe_undefined(self) -> MaybeUndefined<String> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<String> for Arc<str> {
    fn into_maybe_undefined(self) -> MaybeUndefined<String> {
        MaybeUndefined::Value(self.to_string())
    }
}

impl<T: ?Sized + AsRef<OsStr>> IntoMaybeUndefined<PathBuf> for &T {
    fn into_maybe_undefined(self) -> MaybeUndefined<PathBuf> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<PathBuf> for Box<Path> {
    fn into_maybe_undefined(self) -> MaybeUndefined<PathBuf> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<PathBuf> for Cow<'_, Path> {
    fn into_maybe_undefined(self) -> MaybeUndefined<PathBuf> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<serde_json::Value> for &str {
    fn into_maybe_undefined(self) -> MaybeUndefined<serde_json::Value> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<serde_json::Value> for String {
    fn into_maybe_undefined(self) -> MaybeUndefined<serde_json::Value> {
        MaybeUndefined::Value(self.into())
    }
}

impl IntoMaybeUndefined<serde_json::Value> for Cow<'_, str> {
    fn into_maybe_undefined(self) -> MaybeUndefined<serde_json::Value> {
        MaybeUndefined::Value(self.into())
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::{from_value, json, to_value};

    use super::*;

    #[test]
    fn test_maybe_undefined_serde() {
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
        struct A {
            #[serde(default, skip_serializing_if = "MaybeUndefined::is_undefined")]
            a: MaybeUndefined<i32>,
        }

        assert_eq!(to_value(MaybeUndefined::Value(100i32)).unwrap(), json!(100));

        assert_eq!(
            from_value::<MaybeUndefined<i32>>(json!(100)).unwrap(),
            MaybeUndefined::Value(100)
        );
        assert_eq!(
            from_value::<MaybeUndefined<i32>>(json!(null)).unwrap(),
            MaybeUndefined::Null
        );

        assert_eq!(
            to_value(&A {
                a: MaybeUndefined::Value(100i32)
            })
            .unwrap(),
            json!({"a": 100})
        );

        assert_eq!(
            to_value(&A {
                a: MaybeUndefined::Null,
            })
            .unwrap(),
            json!({ "a": null })
        );

        assert_eq!(
            to_value(&A {
                a: MaybeUndefined::Undefined,
            })
            .unwrap(),
            json!({})
        );

        assert_eq!(
            from_value::<A>(json!({"a": 100})).unwrap(),
            A {
                a: MaybeUndefined::Value(100i32)
            }
        );

        assert_eq!(
            from_value::<A>(json!({ "a": null })).unwrap(),
            A {
                a: MaybeUndefined::Null
            }
        );

        assert_eq!(
            from_value::<A>(json!({})).unwrap(),
            A {
                a: MaybeUndefined::Undefined
            }
        );
    }

    #[test]
    fn test_maybe_undefined_to_nested_option() {
        assert_eq!(Option::<Option<i32>>::from(MaybeUndefined::Undefined), None);

        assert_eq!(
            Option::<Option<i32>>::from(MaybeUndefined::Null),
            Some(None)
        );

        assert_eq!(
            Option::<Option<i32>>::from(MaybeUndefined::Value(42)),
            Some(Some(42))
        );
    }

    #[test]
    fn test_as_opt_ref() {
        let value = MaybeUndefined::<String>::Undefined;
        let r = value.as_opt_ref();
        assert_eq!(r, None);

        let value = MaybeUndefined::<String>::Null;
        let r = value.as_opt_ref();
        assert_eq!(r, Some(None));

        let value = MaybeUndefined::<String>::Value("abc".to_string());
        let r = value.as_opt_ref();
        assert_eq!(r, Some(Some(&"abc".to_string())));
    }

    #[test]
    fn test_as_opt_deref() {
        let value = MaybeUndefined::<String>::Undefined;
        let r = value.as_opt_deref();
        assert_eq!(r, None);

        let value = MaybeUndefined::<String>::Null;
        let r = value.as_opt_deref();
        assert_eq!(r, Some(None));

        let value = MaybeUndefined::<String>::Value("abc".to_string());
        let r = value.as_opt_deref();
        assert_eq!(r, Some(Some("abc")));
    }

    #[test]
    fn test_contains_value() {
        let test = "abc";

        let mut value: MaybeUndefined<String> = MaybeUndefined::Undefined;
        assert!(!value.contains_value(&test));

        value = MaybeUndefined::Null;
        assert!(!value.contains_value(&test));

        value = MaybeUndefined::Value("abc".to_string());
        assert!(value.contains_value(&test));
    }

    #[test]
    fn test_contains() {
        let test = Some("abc");
        let none: Option<&str> = None;

        let mut value: MaybeUndefined<String> = MaybeUndefined::Undefined;
        assert!(!value.contains(test.as_ref()));
        assert!(!value.contains(none.as_ref()));

        value = MaybeUndefined::Null;
        assert!(!value.contains(test.as_ref()));
        assert!(value.contains(none.as_ref()));

        value = MaybeUndefined::Value("abc".to_string());
        assert!(value.contains(test.as_ref()));
        assert!(!value.contains(none.as_ref()));
    }

    #[test]
    fn test_map_value() {
        let mut value: MaybeUndefined<i32> = MaybeUndefined::Undefined;
        assert_eq!(value.map_value(|v| v > 2), MaybeUndefined::Undefined);

        value = MaybeUndefined::Null;
        assert_eq!(value.map_value(|v| v > 2), MaybeUndefined::Null);

        value = MaybeUndefined::Value(5);
        assert_eq!(value.map_value(|v| v > 2), MaybeUndefined::Value(true));
    }

    #[test]
    fn test_map() {
        let mut value: MaybeUndefined<i32> = MaybeUndefined::Undefined;
        assert_eq!(value.map(|v| Some(v.is_some())), MaybeUndefined::Undefined);

        value = MaybeUndefined::Null;
        assert_eq!(
            value.map(|v| Some(v.is_some())),
            MaybeUndefined::Value(false)
        );

        value = MaybeUndefined::Value(5);
        assert_eq!(
            value.map(|v| Some(v.is_some())),
            MaybeUndefined::Value(true)
        );
    }

    #[test]
    fn test_transpose() {
        let mut value: MaybeUndefined<Result<i32, &'static str>> = MaybeUndefined::Undefined;
        assert_eq!(value.transpose(), Ok(MaybeUndefined::Undefined));

        value = MaybeUndefined::Null;
        assert_eq!(value.transpose(), Ok(MaybeUndefined::Null));

        value = MaybeUndefined::Value(Ok(5));
        assert_eq!(value.transpose(), Ok(MaybeUndefined::Value(5)));

        value = MaybeUndefined::Value(Err("error"));
        assert_eq!(value.transpose(), Err("error"));
    }
}
