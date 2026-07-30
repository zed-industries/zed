use std::{fmt, sync::Arc};

use fluent_bundle::{FluentArgs, FluentValue};
use gpui_shared_string::SharedString;

use crate::catalog;

/// A value substituted into a localized message's placeable.
///
/// Values are kept as numbers and strings rather than pre-rendered text so that
/// the catalog can format them according to the active locale.
#[derive(Clone, Debug, PartialEq)]
pub enum ArgValue {
    /// A string value.
    Str(SharedString),
    /// A numeric value. Catalogs can branch on it for plural selection.
    Number(f64),
}

impl ArgValue {
    fn to_fluent(&self) -> FluentValue<'static> {
        match self {
            ArgValue::Str(value) => FluentValue::from(value.to_string()),
            ArgValue::Number(value) => FluentValue::from(*value),
        }
    }
}

impl fmt::Display for ArgValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgValue::Str(value) => write!(f, "{value}"),
            ArgValue::Number(value) => {
                if value.fract() == 0.0 && value.is_finite() {
                    write!(f, "{}", *value as i64)
                } else {
                    write!(f, "{value}")
                }
            }
        }
    }
}

impl From<SharedString> for ArgValue {
    fn from(value: SharedString) -> Self {
        ArgValue::Str(value)
    }
}

impl From<&'static str> for ArgValue {
    fn from(value: &'static str) -> Self {
        ArgValue::Str(SharedString::new_static(value))
    }
}

impl From<String> for ArgValue {
    fn from(value: String) -> Self {
        ArgValue::Str(SharedString::new(value))
    }
}

macro_rules! impl_from_number {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl From<$ty> for ArgValue {
                fn from(value: $ty) -> Self {
                    ArgValue::Number(value as f64)
                }
            }
        )+
    };
}

impl_from_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

/// A piece of user-facing text that resolves against the active locale when it
/// is read, rather than when it is constructed.
///
/// Resolving late is what lets the locale change at runtime: a view that holds a
/// `LocalizedString` produces the new text on its next render, with no need to
/// rebuild the view tree by hand.
///
/// Construct these with [`crate::t!`] rather than by hand, so that the key stays
/// derived from the English literal.
#[derive(Clone, Debug, PartialEq)]
pub struct LocalizedString {
    key: &'static str,
    fallback: &'static str,
    args: Option<Arc<Vec<(&'static str, ArgValue)>>>,
}

impl LocalizedString {
    /// Creates a localized string with no placeables.
    pub fn from_parts(key: &'static str, fallback: &'static str) -> Self {
        Self {
            key,
            fallback,
            args: None,
        }
    }

    /// Creates a localized string whose placeables are filled from `args`.
    pub fn with_args(
        key: &'static str,
        fallback: &'static str,
        args: Vec<(&'static str, ArgValue)>,
    ) -> Self {
        Self {
            key,
            fallback,
            args: Some(Arc::new(args)),
        }
    }

    /// The catalog key this string resolves against.
    pub fn key(&self) -> &'static str {
        self.key
    }

    /// The English literal from the call site, used when no catalog defines
    /// [`Self::key`].
    pub fn fallback(&self) -> &'static str {
        self.fallback
    }

    /// Resolves against the active locale.
    ///
    /// Falls back to the English literal from the call site when the active
    /// locale has no translation for this key, so a partially translated catalog
    /// yields a partially translated UI rather than blank or missing text.
    pub fn resolve(&self) -> SharedString {
        let fluent_args = self.args.as_ref().map(|args| {
            let mut fluent_args = FluentArgs::with_capacity(args.len());
            for (name, value) in args.iter() {
                fluent_args.set(*name, value.to_fluent());
            }
            fluent_args
        });

        if let Some(translated) = catalog::lookup(self.key, fluent_args.as_ref()) {
            return SharedString::new(translated);
        }

        match &self.args {
            // The overwhelming majority of UI strings have no placeables, so the
            // untranslated path hands back the literal without copying it.
            None => SharedString::new_static(self.fallback),
            Some(args) => SharedString::new(interpolate(self.fallback, args)),
        }
    }
}

impl fmt::Display for LocalizedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.resolve())
    }
}

impl From<LocalizedString> for SharedString {
    fn from(value: LocalizedString) -> Self {
        value.resolve()
    }
}

impl From<LocalizedString> for String {
    fn from(value: LocalizedString) -> Self {
        value.resolve().to_string()
    }
}

/// Substitutes `{$name}` placeables in an untranslated English literal.
///
/// This deliberately does not run the literal through Fluent: source literals are
/// written for humans reading the code, not as Fluent patterns, so text like
/// `"Use {} to expand"` must survive unchanged rather than fail to parse.
fn interpolate(source: &str, args: &[(&'static str, ArgValue)]) -> String {
    let mut out = String::with_capacity(source.len());
    let mut rest = source;

    while let Some(open) = rest.find('{') {
        let (before, from_brace) = rest.split_at(open);
        out.push_str(before);

        let Some(close) = from_brace.find('}') else {
            // Unbalanced brace: the remainder is literal text.
            out.push_str(from_brace);
            return out;
        };

        let placeable = &from_brace[1..close];
        let name = placeable.trim().strip_prefix('$').map(str::trim);

        match name.and_then(|name| args.iter().find(|(arg, _)| *arg == name)) {
            Some((_, value)) => out.push_str(&value.to_string()),
            // Not a placeable we can fill, so it is literal text.
            None => out.push_str(&from_brace[..=close]),
        }

        rest = &from_brace[close + 1..];
    }

    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolates_named_placeables() {
        let args = vec![
            ("count", ArgValue::Number(3.0)),
            ("name", ArgValue::Str(SharedString::new_static("main.rs"))),
        ];

        assert_eq!(
            interpolate("Renaming {$count} files", &args),
            "Renaming 3 files"
        );
        assert_eq!(
            interpolate("{ $name } has {$count} changes", &args),
            "main.rs has 3 changes"
        );
    }

    #[test]
    fn leaves_non_placeable_braces_alone() {
        let args = vec![("count", ArgValue::Number(1.0))];

        // Not a placeable, so it must survive verbatim.
        assert_eq!(interpolate("Use {} to expand", &args), "Use {} to expand");
        assert_eq!(interpolate("{unknown}", &args), "{unknown}");
        assert_eq!(interpolate("{$missing}", &args), "{$missing}");
        // Unbalanced braces are literal text, not a panic.
        assert_eq!(interpolate("Unclosed {$count", &args), "Unclosed {$count");
    }

    #[test]
    fn renders_whole_numbers_without_a_decimal_point() {
        assert_eq!(ArgValue::Number(3.0).to_string(), "3");
        assert_eq!(ArgValue::Number(3.5).to_string(), "3.5");
        assert_eq!(ArgValue::from(7usize).to_string(), "7");
    }

    #[test]
    fn falls_back_to_the_english_literal() {
        let _guard = catalog::lock_for_test();
        catalog::reset();

        let zoom_in = LocalizedString::from_parts("zoom-in", "Zoom In");
        assert_eq!(zoom_in.resolve().as_ref(), "Zoom In");
        assert_eq!(zoom_in.key(), "zoom-in");

        let renaming = LocalizedString::with_args(
            "renaming-count-files",
            "Renaming {$count} files",
            vec![("count", ArgValue::Number(2.0))],
        );
        assert_eq!(renaming.resolve().as_ref(), "Renaming 2 files");
    }

    #[test]
    fn converts_into_a_shared_string() {
        let _guard = catalog::lock_for_test();
        catalog::reset();

        let shared: SharedString = LocalizedString::from_parts("zoom-in", "Zoom In").into();
        assert_eq!(shared.as_ref(), "Zoom In");
    }
}
