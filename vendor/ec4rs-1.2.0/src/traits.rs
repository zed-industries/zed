use crate::rawvalue::RawValue;

/// Trait for types that be parsed out of [`RawValue`]s.
///
/// Types that implement this trait should also implement `Into<RawValue>`.
pub trait PropertyValue: Sized {
    /// Indicates whether a value that is case-insensitively equal to "unset"
    /// should NOT be treated as if the value is unset.
    ///
    /// This will typically be false for non-string properties.
    const MAYBE_UNSET: bool;

    /// The type of value returned on a failed parse.
    type Err;

    /// Parses a value from a not-unset [`RawValue`].
    ///
    /// This usually shouldn't be called directly.
    /// See [`crate::Properties`] or [`RawValue::parse`].
    fn parse(value: &RawValue) -> Result<Self, Self::Err>;
}

/// Trait for types that are associated with property names.
///
/// Types that implement this trait will usually also implement [`PropertyValue`].
pub trait PropertyKey {
    /// The lowercase string key for this property.
    ///
    /// Used to look up the value in a [`crate::Properties`] map.
    fn key() -> &'static str;
}

/// Tests if the result of parsing the result of an `Into<RawValue>` conversion
/// is *not unequal* to the original value.
#[cfg(test)]
pub fn test_reparse<T, E: std::fmt::Debug>(initial: T)
where
    T: Clone + PropertyValue<Err = E> + Into<RawValue> + std::fmt::Debug + PartialEq,
{
    let written: RawValue = initial.clone().into();
    let result = T::parse(&written).expect("reparse errored");
    assert!(
        !result.ne(&initial),
        "reparsed value is unequal to original; expected `{:?}`, got `{:?}`",
        initial,
        result
    )
}
