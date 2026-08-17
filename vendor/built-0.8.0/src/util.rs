//! Various convenience functions for `built` at runtime.

use std::fmt;
use std::fmt::Write;

#[cfg(feature = "git2")]
pub use crate::git::{get_repo_description, get_repo_head};

#[cfg(feature = "chrono")]
pub use crate::krono::strptime;

/// Parses version-strings with `semver::Version::parse()`.
///
/// This function is only available if `built` was compiled with the
/// `semver` feature.
///
/// The function takes a reference to an array of names and version numbers as
/// serialized by `built` and returns an iterator over the unchanged names
/// and parsed version numbers.
///
/// ```
/// pub mod build_info {
///     pub static DEPENDENCIES: [(&'static str, &'static str); 1] = [("built", "0.1.0")];
/// }
///
/// let deps = build_info::DEPENDENCIES;
/// assert!(built::util::parse_versions(&deps)
///                      .any(|(name, ver)| name == "built" &&
///                                         ver >= semver::Version::parse("0.1.0").unwrap()));
/// ```
///
/// # Panics
/// If a version can't be parsed by `semver::Version::parse()`. This should never
/// happen with version strings provided by Cargo and `built`.
#[cfg(feature = "semver")]
pub fn parse_versions<'a, T>(
    name_and_versions: T,
) -> impl Iterator<Item = (&'a str, semver::Version)>
where
    T: IntoIterator<Item = &'a (&'a str, &'a str)>,
{
    fn parse_version<'a>(t: &'a (&'a str, &'a str)) -> (&'a str, semver::Version) {
        (t.0, t.1.parse().unwrap())
    }
    name_and_versions.into_iter().map(parse_version)
}

/// Detect execution on various Continuous Integration platforms.
///
/// CI-platforms are detected by the presence of known environment variables.
/// This allows to detect specific CI-platform (like `GitLab`); various
/// generic environment variables are also checked, which may result in
/// `CIPlatform::Generic`.
///
/// Since some platforms have fairly generic environment variables to begin with
/// (e.g. `TASK_ID`), this function may have false positives.
#[must_use]
pub fn detect_ci() -> Option<super::CIPlatform> {
    crate::environment::EnvironmentMap::new().detect_ci()
}

pub(crate) trait ParseFromEnv<'a>
where
    Self: Sized,
{
    type Err: std::fmt::Debug;

    fn parse_from_env(s: &'a str) -> Result<Self, Self::Err>;
}

impl<'a> ParseFromEnv<'a> for &'a str {
    type Err = std::convert::Infallible;

    fn parse_from_env(s: &'a str) -> Result<Self, Self::Err> {
        Ok(s)
    }
}

macro_rules! parsefromenv_impl {
    ($tie:ty) => {
        impl ParseFromEnv<'_> for $tie {
            type Err = <Self as std::str::FromStr>::Err;

            fn parse_from_env(s: &str) -> Result<Self, Self::Err> {
                s.parse()
            }
        }
    };
    ($($tie:ty),+) => {
        $(
            parsefromenv_impl!($tie);
        )+
    };
}
parsefromenv_impl!(i64, i32, i16, i8, u64, u32, u16, u8, bool, String);

impl<'a, T> ParseFromEnv<'a> for Vec<T>
where
    T: ParseFromEnv<'a>,
{
    type Err = <T as ParseFromEnv<'a>>::Err;

    fn parse_from_env(s: &'a str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|chunk| T::parse_from_env(chunk.trim()))
            .collect()
    }
}

impl<'a, T> ParseFromEnv<'a> for Option<T>
where
    T: ParseFromEnv<'a>,
{
    type Err = <T as ParseFromEnv<'a>>::Err;

    fn parse_from_env(s: &'a str) -> Result<Self, Self::Err> {
        if s == "BUILT_OVERRIDE_NONE" {
            Ok(None)
        } else {
            Ok(Some(T::parse_from_env(s)?))
        }
    }
}

pub(crate) struct ArrayDisplay<'a, T, F>(pub &'a [T], pub F)
where
    F: Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result;

impl<T, F> fmt::Display for ArrayDisplay<'_, T, F>
where
    F: Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;
        for (i, v) in self.0.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            (self.1)(v, f)?;
        }
        f.write_char(']')
    }
}

#[cfg(feature = "cargo-lock")]
pub(crate) struct TupleArrayDisplay<'a, T>(pub &'a [(T, T)]);

#[cfg(feature = "cargo-lock")]
impl<T> fmt::Display for TupleArrayDisplay<'_, T>
where
    T: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            ArrayDisplay(self.0, |(a, b), fmt| write!(
                fmt,
                r#"("{}", "{}")"#,
                a.as_ref().escape_default(),
                b.as_ref().escape_default()
            ))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env() {
        assert_eq!(String::parse_from_env("foo"), Ok("foo".to_owned()));
        assert_eq!(<&str>::parse_from_env("foo"), Ok("foo"));

        assert_eq!(i64::parse_from_env("123"), Ok(123));
        assert_eq!(i32::parse_from_env("123"), Ok(123));
        assert_eq!(i16::parse_from_env("123"), Ok(123));
        assert_eq!(i8::parse_from_env("123"), Ok(123));
        assert_eq!(u64::parse_from_env("123"), Ok(123));
        assert_eq!(u32::parse_from_env("123"), Ok(123));
        assert_eq!(u16::parse_from_env("123"), Ok(123));
        assert_eq!(u8::parse_from_env("123"), Ok(123));

        assert_eq!(i8::parse_from_env("-123"), Ok(-123));
        assert!(u8::parse_from_env("-123").is_err());
        assert!(i32::parse_from_env("foo").is_err());
        assert!(i32::parse_from_env("").is_err());

        assert_eq!(bool::parse_from_env("true"), Ok(true));
        assert_eq!(bool::parse_from_env("false"), Ok(false));
        assert!(bool::parse_from_env("").is_err());
        assert!(bool::parse_from_env("foo").is_err());

        assert!(Option::<i32>::parse_from_env("").is_err());
        assert_eq!(Option::<&str>::parse_from_env(""), Ok(Some("")));
        assert_eq!(
            Option::parse_from_env("BUILT_OVERRIDE_NONE"),
            Ok(Option::<i32>::None)
        );
        assert_eq!(Option::parse_from_env("123"), Ok(Some(123u8)));
        assert_eq!(Option::<bool>::parse_from_env("true"), Ok(Some(true)));
        assert!(Option::<bool>::parse_from_env("123").is_err());

        assert_eq!(
            Vec::<&str>::parse_from_env("foo, b a r , foo"),
            Ok(vec!["foo", "b a r", "foo"])
        );
        assert_eq!(
            Vec::<i32>::parse_from_env("123,456,789"),
            Ok(vec![123, 456, 789])
        );

        assert_eq!(
            Option::parse_from_env("123,456"),
            Ok(Some(vec![123u32, 456u32]))
        );
    }
}
