/// Parameter for [`ReflectEq`].
#[derive(Debug, Default)]
pub struct ReflectEqMode {
    /// When `true`, `NaN` values are considered equal to each other.
    pub nan_equal: bool,
    _non_exhausitve: (),
}

impl ReflectEqMode {
    /// Default equality, similar to `#[derive(PartialEq)]`.
    pub fn default() -> ReflectEqMode {
        Default::default()
    }

    /// Equality where float `NaN` values are considered equal to each other.
    ///
    /// Useful in tests.
    pub fn nan_equal() -> ReflectEqMode {
        ReflectEqMode {
            nan_equal: true,
            ..Default::default()
        }
    }
}

/// Special version of eq.
///
/// With `mode` [`ReflectEqMode::default()`], should be equivalent
/// to `#[derive(PartialEq)]`.
pub trait ReflectEq {
    /// Perform the equality comparison.
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool;
}
