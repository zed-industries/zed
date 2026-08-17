//! Version compare module, with useful static comparison methods.

use crate::version::Version;
use crate::Cmp;

/// Compare two version number strings to each other.
///
/// This compares version `a` to version `b`, and returns whether version `a` is greater, less
/// or equal to version `b`.
///
/// If either version number string is invalid an error is returned.
///
/// One of the following operators is returned:
///
/// * `Cmp::Eq`
/// * `Cmp::Lt`
/// * `Cmp::Gt`
///
/// # Examples
///
/// ```
/// use version_compare::{Cmp, compare};
///
/// assert_eq!(compare("1.2.3", "1.2.3"), Ok(Cmp::Eq));
/// assert_eq!(compare("1.2.3", "1.2.4"), Ok(Cmp::Lt));
/// assert_eq!(compare("1", "0.1"), Ok(Cmp::Gt));
/// ```
#[allow(clippy::result_unit_err)]
pub fn compare<A, B>(a: A, b: B) -> Result<Cmp, ()>
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    let a = Version::from(a.as_ref()).ok_or(())?;
    let b = Version::from(b.as_ref()).ok_or(())?;
    Ok(a.compare(b))
}

/// Compare two version number strings to each other and test against the given comparison
/// `operator`.
///
/// If either version number string is invalid an error is returned.
///
/// # Examples
///
/// ```
/// use version_compare::{Cmp, compare_to};
///
/// assert!(compare_to("1.2.3", "1.2.3", Cmp::Eq).unwrap());
/// assert!(compare_to("1.2.3", "1.2.3", Cmp::Le).unwrap());
/// assert!(compare_to("1.2.3", "1.2.4", Cmp::Lt).unwrap());
/// assert!(compare_to("1", "0.1", Cmp::Gt).unwrap());
/// assert!(compare_to("1", "0.1", Cmp::Ge).unwrap());
/// ```
#[allow(clippy::result_unit_err)]
pub fn compare_to<A, B>(a: A, b: B, operator: Cmp) -> Result<bool, ()>
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    let a = Version::from(a.as_ref()).ok_or(())?;
    let b = Version::from(b.as_ref()).ok_or(())?;
    Ok(a.compare_to(b, operator))
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use crate::test::{COMBIS, COMBIS_ERROR};
    use crate::Cmp;

    #[test]
    fn compare() {
        // Compare each version in the version set having the default manifest
        for entry in COMBIS.iter().filter(|c| c.3.is_none()) {
            assert_eq!(
                super::compare(entry.0, entry.1),
                Ok(entry.2),
                "Testing that {} is {} {}",
                entry.0,
                entry.2.sign(),
                entry.1,
            );
        }

        // Compare each error version in the version set
        for entry in COMBIS_ERROR {
            let result = super::compare(entry.0, entry.1);

            if result.is_ok() {
                assert!(result != Ok(entry.2));
            }
        }
    }

    #[test]
    fn compare_to() {
        // Compare each version in the version set having the default manifest
        for entry in COMBIS.iter().filter(|c| c.3.is_none()) {
            // Test
            assert!(super::compare_to(entry.0, entry.1, entry.2).unwrap());

            // Make sure the inverse operator is not correct
            assert!(!super::compare_to(entry.0, entry.1, entry.2.invert()).unwrap());
        }

        // Compare each error version in the version set
        for entry in COMBIS_ERROR {
            let result = super::compare_to(entry.0, entry.1, entry.2);

            if result.is_ok() {
                assert!(!result.unwrap())
            }
        }

        // Assert an exceptional case, compare to not equal
        assert!(super::compare_to("1.2.3", "1.2", Cmp::Ne).unwrap());
    }
}
