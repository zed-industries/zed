//! Module for the version manifest.
//!
//! A version manifest can be used to configure and specify how versions are parsed and compared.
//! For example, you can configure the maximum depth of a version number, and set whether text
//! parts are ignored in a version string.

/// Version manifest (configuration).
///
/// A manifest (configuration) that is used respectively when parsing and comparing version strings.
///
/// # Examples
///
/// ```rust
/// use version_compare::{Manifest, Version};
///
/// // Create manifest with max depth of 2
/// let mut manifest = Manifest::default();
/// manifest.max_depth = Some(2);
///
/// // Version strings equal with manifest because we compare up-to 2 parts deep
/// let a = Version::from_manifest("1.0.1", &manifest).unwrap();
/// let b = Version::from_manifest("1.0.2", &manifest).unwrap();
/// assert!(a == b);
/// ```

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct Manifest {
    /// The maximum depth of a version number.
    ///
    /// This specifies the maximum number of parts. There is no limit if `None` is set.
    pub max_depth: Option<usize>,

    /// Whether to ignore text parts in version strings.
    pub ignore_text: bool,

    /// Use GNU sort based ordering.
    ///
    /// Enabling this modifies the ordering of numbers with a leading zero to mimick GNUs sort.
    ///
    /// Issue: https://github.com/timvisee/version-compare/issues/27
    pub gnu_ordering: bool,
}

/// Version manifest implementation.
impl Manifest {
    /// Check whether there's a maximum configured depth.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Manifest;
    ///
    /// let mut manifest = Manifest::default();
    ///
    /// assert!(!manifest.has_max_depth());
    ///
    /// manifest.max_depth = Some(3);
    /// assert!(manifest.has_max_depth());
    /// ```
    pub fn has_max_depth(&self) -> bool {
        self.max_depth.is_some() && self.max_depth.unwrap() > 0
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::Manifest;

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn has_max_depth() {
        let mut manifest = Manifest::default();

        manifest.max_depth = Some(1);
        assert!(manifest.has_max_depth());

        manifest.max_depth = Some(3);
        assert!(manifest.has_max_depth());

        manifest.max_depth = None;
        assert!(!manifest.has_max_depth());
    }
}
