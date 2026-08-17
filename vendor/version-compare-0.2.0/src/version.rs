//! Version module, which provides the `Version` struct as parsed version representation.
//!
//! Version numbers in the form of a string are parsed to a `Version` first, before any comparison
//! is made. This struct provides many methods and features for easy comparison, probing and other
//! things.

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::iter::Peekable;
use std::slice::Iter;

use crate::{Cmp, Manifest, Part};

/// Version struct, wrapping a string, providing useful comparison functions.
///
/// A version in string format can be parsed using methods like `Version::from("1.2.3");`,
/// returning a `Result` with the parse result.
///
/// The original version string can be accessed using `version.as_str()`. A `Version` that isn't
/// derrived from a version string returns a generated string.
///
/// The struct provides many methods for easy comparison and probing.
///
/// # Examples
///
/// ```
/// use version_compare::{Version};
///
/// let ver = Version::from("1.2.3").unwrap();
/// ```
#[derive(Clone, Eq)]
pub struct Version<'a> {
    version: &'a str,
    parts: Vec<Part<'a>>,
    manifest: Option<&'a Manifest>,
}

impl<'a> Version<'a> {
    /// Create a `Version` instance from a version string.
    ///
    /// The version string should be passed to the `version` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Cmp, Version};
    ///
    /// let a = Version::from("1.2.3").unwrap();
    /// let b = Version::from("1.3.0").unwrap();
    ///
    /// assert_eq!(a.compare(b), Cmp::Lt);
    /// ```
    pub fn from(version: &'a str) -> Option<Self> {
        Some(Version {
            version,
            parts: split_version_str(version, None)?,
            manifest: None,
        })
    }

    /// Create a `Version` instance from already existing parts
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Cmp, Version, Part};
    ///
    /// let ver = Version::from_parts("1.0", vec![Part::Number(1), Part::Number(0)]);
    /// ```
    pub fn from_parts(version: &'a str, parts: Vec<Part<'a>>) -> Self {
        Version {
            version,
            parts,
            manifest: None,
        }
    }

    /// Create a `Version` instance from a version string with the given `manifest`.
    ///
    /// The version string should be passed to the `version` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Cmp, Version, Manifest};
    ///
    /// let manifest = Manifest::default();
    /// let ver = Version::from_manifest("1.2.3", &manifest).unwrap();
    ///
    /// assert_eq!(ver.compare(Version::from("1.2.3").unwrap()), Cmp::Eq);
    /// ```
    pub fn from_manifest(version: &'a str, manifest: &'a Manifest) -> Option<Self> {
        Some(Version {
            version,
            parts: split_version_str(version, Some(manifest))?,
            manifest: Some(manifest),
        })
    }

    /// Get the version manifest, if available.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Version;
    ///
    /// let version = Version::from("1.2.3").unwrap();
    ///
    /// if version.has_manifest() {
    ///     println!(
    ///         "Maximum version part depth is {} for this version",
    ///         version.manifest().unwrap().max_depth.unwrap_or(0),
    ///     );
    /// } else {
    ///     println!("Version has no manifest");
    /// }
    /// ```
    pub fn manifest(&self) -> Option<&Manifest> {
        self.manifest
    }

    /// Check whether this version has a manifest.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Version;
    ///
    /// let version = Version::from("1.2.3").unwrap();
    ///
    /// if version.has_manifest() {
    ///     println!("This version does have a manifest");
    /// } else {
    ///     println!("This version does not have a manifest");
    /// }
    /// ```
    pub fn has_manifest(&self) -> bool {
        self.manifest().is_some()
    }

    /// Set the version manifest.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Version, Manifest};
    ///
    /// let manifest = Manifest::default();
    /// let mut version = Version::from("1.2.3").unwrap();
    ///
    /// version.set_manifest(Some(&manifest));
    /// ```
    pub fn set_manifest(&mut self, manifest: Option<&'a Manifest>) {
        self.manifest = manifest;

        // TODO: Re-parse the version string, because the manifest might have changed.
    }

    /// Get the original version string.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Version;
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.as_str(), "1.2.3");
    /// ```
    pub fn as_str(&self) -> &str {
        self.version
    }

    /// Get a specific version part by it's `index`.
    /// An error is returned if the given index is out of bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Version, Part};
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.part(0), Ok(Part::Number(1)));
    /// assert_eq!(ver.part(1), Ok(Part::Number(2)));
    /// assert_eq!(ver.part(2), Ok(Part::Number(3)));
    /// ```
    #[allow(clippy::result_unit_err)]
    pub fn part(&self, index: usize) -> Result<Part<'a>, ()> {
        // Make sure the index is in-bound
        if index >= self.parts.len() {
            return Err(());
        }

        Ok(self.parts[index])
    }

    /// Get a vector of all version parts.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::{Version, Part};
    ///
    /// let ver = Version::from("1.2.3").unwrap();
    ///
    /// assert_eq!(ver.parts(), [
    ///     Part::Number(1),
    ///     Part::Number(2),
    ///     Part::Number(3)
    /// ]);
    /// ```
    pub fn parts(&self) -> &[Part<'a>] {
        self.parts.as_slice()
    }

    /// Compare this version to the given `other` version using the default `Manifest`.
    ///
    /// This method returns one of the following comparison operators:
    ///
    /// * `Lt`
    /// * `Eq`
    /// * `Gt`
    ///
    /// Other comparison operators can be used when comparing, but aren't returned by this method.
    ///
    /// # Examples:
    ///
    /// ```
    /// use version_compare::{Cmp, Version};
    ///
    /// let a = Version::from("1.2").unwrap();
    /// let b = Version::from("1.3.2").unwrap();
    ///
    /// assert_eq!(a.compare(&b), Cmp::Lt);
    /// assert_eq!(b.compare(&a), Cmp::Gt);
    /// assert_eq!(a.compare(&a), Cmp::Eq);
    /// ```
    pub fn compare<V>(&self, other: V) -> Cmp
    where
        V: Borrow<Version<'a>>,
    {
        compare_iter(
            self.parts.iter().peekable(),
            other.borrow().parts.iter().peekable(),
            self.manifest,
        )
    }

    /// Compare this version to the given `other` version,
    /// and check whether the given comparison operator is valid using the default `Manifest`.
    ///
    /// All comparison operators can be used.
    ///
    /// # Examples:
    ///
    /// ```
    /// use version_compare::{Cmp, Version};
    ///
    /// let a = Version::from("1.2").unwrap();
    /// let b = Version::from("1.3.2").unwrap();
    ///
    /// assert!(a.compare_to(&b, Cmp::Lt));
    /// assert!(a.compare_to(&b, Cmp::Le));
    /// assert!(a.compare_to(&a, Cmp::Eq));
    /// assert!(a.compare_to(&a, Cmp::Le));
    /// ```
    pub fn compare_to<V>(&self, other: V, operator: Cmp) -> bool
    where
        V: Borrow<Version<'a>>,
    {
        match self.compare(other) {
            Cmp::Eq => matches!(operator, Cmp::Eq | Cmp::Le | Cmp::Ge),
            Cmp::Lt => matches!(operator, Cmp::Ne | Cmp::Lt | Cmp::Le),
            Cmp::Gt => matches!(operator, Cmp::Ne | Cmp::Gt | Cmp::Ge),
            _ => unreachable!(),
        }
    }
}

impl<'a> fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

// Show just the version component parts as debug output
impl<'a> fmt::Debug for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.parts)
        } else {
            write!(f, "{:?}", self.parts)
        }
    }
}

/// Implement the partial ordering trait for the version struct, to easily allow version comparison.
impl<'a> PartialOrd for Version<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other).ord().unwrap())
    }
}

/// Implement the partial equality trait for the version struct, to easily allow version comparison.
impl<'a> PartialEq for Version<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.compare_to(other, Cmp::Eq)
    }
}

/// Split the given version string, in it's version parts.
fn split_version_str<'a>(
    version: &'a str,
    manifest: Option<&'a Manifest>,
) -> Option<Vec<Part<'a>>> {
    // Split the version string, and create a vector to put the parts in
    let split = version.split(|c| !char::is_alphanumeric(c));
    let mut parts = Vec::new();

    // Get the manifest to follow
    let mut used_manifest = &Manifest::default();
    if let Some(m) = manifest {
        used_manifest = m;
    }

    // Loop over the parts, and parse them
    for part in split {
        // We may not go over the maximum depth
        if used_manifest.max_depth.is_some() && parts.len() >= used_manifest.max_depth.unwrap_or(0)
        {
            break;
        }

        // Skip empty parts
        if part.is_empty() {
            continue;
        }

        // Try to parse the value as an number
        match part.parse::<i32>() {
            Ok(number) => {
                // For GNU ordering we parse numbers with leading zero as string
                if number > 0
                    && part.starts_with('0')
                    && manifest.map(|m| m.gnu_ordering).unwrap_or(false)
                {
                    parts.push(Part::Text(part));
                    continue;
                }

                // Push the number part to the vector
                parts.push(Part::Number(number));
            }
            Err(_) => {
                // Ignore text parts if specified
                if used_manifest.ignore_text {
                    continue;
                }

                // Numbers suffixed by text should be split into a number and text as well,
                // if the number overflows, handle it as text
                let split_at = part
                    .char_indices()
                    .take(part.len() - 1)
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .map(|(i, c)| (i, c, part.chars().nth(i + 1).unwrap()))
                    .filter(|(_, _, b)| b.is_alphabetic())
                    .map(|(i, _, _)| i)
                    .next();
                if let Some(at) = split_at {
                    if let Ok(n) = part[..=at].parse() {
                        parts.push(Part::Number(n));
                        parts.push(Part::Text(&part[at + 1..]));
                    } else {
                        parts.push(Part::Text(part));
                    }
                    continue;
                }

                // Push the text part to the vector
                parts.push(Part::Text(part))
            }
        }
    }

    // The version must contain a number part if any part was parsed
    if !parts.is_empty() && !parts.iter().any(|p| matches!(p, Part::Number(_))) {
        return None;
    }

    // Return the list of parts
    Some(parts)
}

/// Compare two version numbers based on the iterators of their version parts.
///
/// This method returns one of the following comparison operators:
///
/// * `Lt`
/// * `Eq`
/// * `Gt`
///
/// Other comparison operators can be used when comparing, but aren't returned by this method.
fn compare_iter<'a>(
    mut iter: Peekable<Iter<Part<'a>>>,
    mut other_iter: Peekable<Iter<Part<'a>>>,
    manifest: Option<&Manifest>,
) -> Cmp {
    // Iterate over the iterator, without consuming it
    for part in &mut iter {
        match (part, other_iter.next()) {
            // If we only have a zero on the lhs, continue
            (Part::Number(lhs), None) if lhs == &0 => {
                continue;
            }

            // If we only have text on the lhs, it is less
            (Part::Text(_), None) => return Cmp::Lt,

            // If we have anything else on the lhs, it is greater
            (_, None) => return Cmp::Gt,

            // Compare numbers
            (Part::Number(lhs), Some(Part::Number(rhs))) => match Cmp::from(lhs.cmp(rhs)) {
                Cmp::Eq => {}
                cmp => return cmp,
            },

            // Compare text
            (Part::Text(lhs), Some(Part::Text(rhs))) => {
                // Normalize case and compare text: "RC1" will be less than "RC2"
                match Cmp::from(lhs.to_lowercase().cmp(&rhs.to_lowercase())) {
                    Cmp::Eq => {}
                    cmp => return cmp,
                }
            }

            // For GNU ordering we have a special number/text comparison
            (lhs @ Part::Number(_), Some(rhs @ Part::Text(_)))
            | (lhs @ Part::Text(_), Some(rhs @ Part::Number(_)))
                if manifest.map(|m| m.gnu_ordering).unwrap_or(false) =>
            {
                match compare_gnu_number_text(lhs, rhs) {
                    Some(Cmp::Eq) | None => {}
                    Some(cmp) => return cmp,
                }
            }

            // TODO: decide what to do for other type combinations
            _ => {}
        }
    }

    // Check whether we should iterate over the other iterator, if it has any items left
    match other_iter.peek() {
        // Compare based on the other iterator
        Some(_) => compare_iter(other_iter, iter, manifest).flip(),

        // Nothing more to iterate over, the versions should be equal
        None => Cmp::Eq,
    }
}

/// Special logic for comparing a number and text with GNU ordering.
///
/// Numbers should be ordered like this:
///
/// - 3
/// - 04
/// - 4
// TODO: this is not efficient, find a better method
fn compare_gnu_number_text(lhs: &Part, rhs: &Part) -> Option<Cmp> {
    // Both values must be parsable as numbers
    let lhs_num = match lhs {
        Part::Number(n) => *n,
        Part::Text(n) => n.parse().ok()?,
    };
    let rhs_num = match rhs {
        Part::Number(n) => *n,
        Part::Text(n) => n.parse().ok()?,
    };

    // Return ordering if numeric values are different
    match lhs_num.cmp(&rhs_num).into() {
        Cmp::Eq => {}
        cmp => return Some(cmp),
    }

    // Either value must have a leading zero
    if !matches!(lhs, Part::Text(t) if t.starts_with('0'))
        && !matches!(rhs, Part::Text(t) if t.starts_with('0'))
    {
        return None;
    }

    let lhs = match lhs {
        Part::Number(n) => format!("{}", n),
        Part::Text(n) => n.to_string(),
    };
    let rhs = match rhs {
        Part::Number(n) => format!("{}", n),
        Part::Text(n) => n.to_string(),
    };

    Some(lhs.cmp(&rhs).into())
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use std::cmp;

    use crate::test::{COMBIS, VERSIONS, VERSIONS_ERROR};
    use crate::{Cmp, Manifest, Part};

    use super::Version;

    #[test]
    // TODO: This doesn't really test whether this method fully works
    fn from() {
        // Test whether parsing works for each test version
        for version in VERSIONS {
            assert!(Version::from(version.0).is_some());
        }

        // Test whether parsing works for each test invalid version
        for version in VERSIONS_ERROR {
            assert!(Version::from(version.0).is_none());
        }
    }

    #[test]
    // TODO: This doesn't really test whether this method fully works
    fn from_manifest() {
        // Create a manifest
        let manifest = Manifest::default();

        // Test whether parsing works for each test version
        for version in VERSIONS {
            assert_eq!(
                Version::from_manifest(version.0, &manifest)
                    .unwrap()
                    .manifest,
                Some(&manifest)
            );
        }

        // Test whether parsing works for each test invalid version
        for version in VERSIONS_ERROR {
            assert!(Version::from_manifest(version.0, &manifest).is_none());
        }
    }

    #[test]
    fn manifest() {
        let manifest = Manifest::default();
        let mut version = Version::from("1.2.3").unwrap();

        version.manifest = Some(&manifest);
        assert_eq!(version.manifest(), Some(&manifest));

        version.manifest = None;
        assert_eq!(version.manifest(), None);
    }

    #[test]
    fn has_manifest() {
        let manifest = Manifest::default();
        let mut version = Version::from("1.2.3").unwrap();

        version.manifest = Some(&manifest);
        assert!(version.has_manifest());

        version.manifest = None;
        assert!(!version.has_manifest());
    }

    #[test]
    fn set_manifest() {
        let manifest = Manifest::default();
        let mut version = Version::from("1.2.3").unwrap();

        version.set_manifest(Some(&manifest));
        assert_eq!(version.manifest, Some(&manifest));

        version.set_manifest(None);
        assert_eq!(version.manifest, None);
    }

    #[test]
    fn as_str() {
        // Test for each test version
        for version in VERSIONS {
            // The input version string must be the same as the returned string
            assert_eq!(Version::from(version.0).unwrap().as_str(), version.0);
        }
    }

    #[test]
    fn part() {
        // Test for each test version
        for version in VERSIONS {
            // Create a version object
            let ver = Version::from(version.0).unwrap();

            // Loop through each part
            for i in 0..version.1 {
                assert_eq!(ver.part(i), Ok(ver.parts[i]));
            }

            // A value outside the range must return an error
            assert!(ver.part(version.1).is_err());
        }
    }

    #[test]
    fn parts() {
        // Test for each test version
        for version in VERSIONS {
            // The number of parts must match
            assert_eq!(Version::from(version.0).unwrap().parts().len(), version.1);
        }
    }

    #[test]
    fn parts_max_depth() {
        // Create a manifest
        let mut manifest = Manifest::default();

        // Loop through a range of numbers
        for depth in 0..5 {
            // Set the maximum depth
            manifest.max_depth = if depth > 0 { Some(depth) } else { None };

            // Test for each test version with the manifest
            for version in VERSIONS {
                // Create a version object, and count it's parts
                let ver = Version::from_manifest(version.0, &manifest);

                // Some versions might be none, because not all of the start with a number when the
                // maximum depth is 1. A version string with only text isn't allowed,
                // resulting in none.
                if ver.is_none() {
                    continue;
                }

                // Get the part count
                let count = ver.unwrap().parts().len();

                // The number of parts must match
                if depth == 0 {
                    assert_eq!(count, version.1);
                } else {
                    assert_eq!(count, cmp::min(version.1, depth));
                }
            }
        }
    }

    #[test]
    fn parts_ignore_text() {
        // Create a manifest
        let mut manifest = Manifest::default();

        // Try this for true and false
        for ignore in &[true, false] {
            // Set to ignore text
            manifest.ignore_text = *ignore;

            // Keep track whether any version passed with text
            let mut had_text = false;

            // Test each test version
            for version in VERSIONS {
                // Create a version instance, and get it's parts
                let ver = Version::from_manifest(version.0, &manifest).unwrap();

                // Loop through all version parts
                for part in ver.parts() {
                    if let Part::Text(_) = part {
                        // Set the flag
                        had_text = true;

                        // Break the loop if we already reached text when not ignored
                        if !ignore {
                            break;
                        }
                    }
                }
            }

            // Assert had text
            assert_eq!(had_text, !ignore);
        }
    }

    #[test]
    fn compare() {
        // Compare each version in the version set
        for entry in COMBIS {
            // Get both versions
            let (a, b) = entry.versions();

            // Compare them
            assert_eq!(
                a.compare(b),
                entry.2.clone(),
                "Testing that {} is {} {}",
                entry.0,
                entry.2.sign(),
                entry.1,
            );
        }
    }

    #[test]
    fn compare_to() {
        // Compare each version in the version set
        for entry in COMBIS.iter().filter(|c| c.3.is_none()) {
            // Get both versions
            let (a, b) = entry.versions();

            // Test normally and inverse
            assert!(a.compare_to(&b, entry.2));
            assert!(!a.compare_to(b, entry.2.invert()));
        }

        // Assert an exceptional case, compare to not equal
        assert!(Version::from("1.2")
            .unwrap()
            .compare_to(Version::from("1.2.3").unwrap(), Cmp::Ne,));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Version::from("1.2.3").unwrap()), "1.2.3");
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Version::from("1.2.3").unwrap()),
            "[Number(1), Number(2), Number(3)]",
        );
        assert_eq!(
            format!("{:#?}", Version::from("1.2.3").unwrap()),
            "[\n    Number(\n        1,\n    ),\n    Number(\n        2,\n    ),\n    Number(\n        3,\n    ),\n]",
        );
    }

    #[test]
    fn partial_cmp() {
        // Compare each version in the version set
        for entry in COMBIS {
            // Get both versions
            let (a, b) = entry.versions();

            // Compare and assert
            match entry.2 {
                Cmp::Eq => assert!(a == b),
                Cmp::Lt => assert!(a < b),
                Cmp::Gt => assert!(a > b),
                _ => {}
            }
        }
    }

    #[test]
    fn partial_eq() {
        // Compare each version in the version set
        for entry in COMBIS {
            // Skip entries that are less or equal, or greater or equal
            match entry.2 {
                Cmp::Le | Cmp::Ge => continue,
                _ => {}
            }

            // Get both versions
            let (a, b) = entry.versions();

            // Determine what the result should be
            let result = matches!(entry.2, Cmp::Eq);

            // Test
            assert_eq!(a == b, result);
        }

        // Assert an exceptional case, compare to not equal
        assert!(Version::from("1.2").unwrap() != Version::from("1.2.3").unwrap());
    }
}
