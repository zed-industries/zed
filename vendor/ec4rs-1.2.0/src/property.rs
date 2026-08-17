//! Enums for common EditorConfig properties.
//!
//! This crate contains every current universal property specified by standard,
//! plus others that are common enough to be worth supporting.

use super::{PropertyKey, PropertyValue};
use crate::rawvalue::RawValue;

use std::fmt::Display;

/// Error for common property parse failures.
#[derive(Clone, Copy, Debug)]
pub struct UnknownValueError;

impl Display for UnknownValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown value")
    }
}

impl std::error::Error for UnknownValueError {}

//TODO: Deduplicate these macros a bit?

macro_rules! property_choice {
    ($prop_id:ident, $name:literal; $(($variant:ident, $string:literal)),+) => {
        // MISTAKE: These need to be #[non_exhaustive],
        // but adding it would be a breaking change.
        // Hold off on adding it until the breakage would need to happen anyway.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[repr(u8)]
        #[doc = concat!("The [`",$name,"`](https://github.com/editorconfig/editorconfig/wiki/EditorConfig-Properties#",$name,") property.")]
        #[allow(missing_docs)]
        pub enum $prop_id {$($variant),+}

        impl PropertyValue for $prop_id {
            const MAYBE_UNSET: bool = false;
            type Err = UnknownValueError;
            fn parse(raw: &RawValue) -> Result<Self, Self::Err> {
                match raw.into_str().to_lowercase().as_str() {
                    $($string => Ok($prop_id::$variant),)+
                    _ => Err(UnknownValueError)
                }
            }
        }

        impl From<$prop_id> for RawValue {
            fn from(val: $prop_id) -> RawValue {
                match val {
                    $($prop_id::$variant => RawValue::from($string)),*
                }
            }
        }

        impl PropertyKey for $prop_id {
            fn key() -> &'static str {$name}
        }

        impl Display for $prop_id {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $($prop_id::$variant => $string),*
                })
            }
        }
    }
}

macro_rules! property_valued {
    (
        $prop_id:ident, $name:literal, $value_type:ty;
        $(($variant:ident, $string:literal)),*
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        #[doc = concat!("The [`",$name,"`](https://github.com/editorconfig/editorconfig/wiki/EditorConfig-Properties#",$name,") property.")]
        #[allow(missing_docs)]
        pub enum $prop_id {
            Value($value_type)
            $(,$variant)*
        }

        impl PropertyValue for $prop_id {
            const MAYBE_UNSET: bool = false;
            type Err = UnknownValueError;
            fn parse(raw: &RawValue) -> Result<Self, Self::Err> {
                match raw.into_str().to_lowercase().as_str() {
                    $($string => Ok($prop_id::$variant),)*
                    v => v.parse::<$value_type>().map(Self::Value).or(Err(UnknownValueError))
                }
            }
        }

        impl From<$prop_id> for RawValue {
            fn from(val: $prop_id) -> RawValue {
                match val {
                    $prop_id::Value(v) => RawValue::from(v.to_string()),
                    $($prop_id::$variant => RawValue::from($string)),*
                }
            }
        }

        impl PropertyKey for $prop_id {
            fn key() -> &'static str {$name}
        }

        impl Display for $prop_id {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $prop_id::Value(v) => write!(f, "{}", v),
                    $($prop_id::$variant => write!(f, "{}", $string)),*
                }
            }
        }
    }
}

property_choice! {
    IndentStyle, "indent_style";
    (Tabs, "tab"),
    (Spaces, "space")
}

// NOTE:
// The spec and the wiki disagree on the valid range of indent/tab sizes.
// The spec says "whole numbers" for both,
// whereas the wiki says "an integer"/"a positive integer" respectively.
// This implementation follows the spec strictly here.
// Notably, it will happily consider sizes of 0 valid.

property_valued! {IndentSize, "indent_size", usize; (UseTabWidth, "tab")}
property_valued! {TabWidth, "tab_width", usize;}

property_choice! {
    EndOfLine, "end_of_line";
    (Lf,   "lf"),
    (CrLf, "crlf"),
    (Cr,   "cr")
}

property_choice! {
    Charset, "charset";
    (Utf8,    "utf-8"),
    (Latin1,  "latin1"),
    (Utf16Le, "utf-16le"),
    (Utf16Be, "utf-16be"),
    (Utf8Bom, "utf-8-bom")
}

property_valued! {TrimTrailingWs, "trim_trailing_whitespace", bool;}
property_valued! {FinalNewline, "insert_final_newline", bool;}
property_valued! {MaxLineLen, "max_line_length", usize; (Off, "off")}

// As of the authorship of this comment, spelling_language isn't on the wiki.
// Ooop.

#[cfg(feature = "language-tags")]
/// The `spelling_language` property added by EditorConfig 0.16.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[allow(missing_docs)]
pub enum SpellingLanguage {
    Value(crate::language_tags::LanguageTag),
}

#[cfg(feature = "language-tags")]
impl PropertyValue for SpellingLanguage {
    const MAYBE_UNSET: bool = false;
    type Err = crate::language_tags::ParseError;
    fn parse(raw: &RawValue) -> Result<Self, Self::Err> {
        if let Some(string) = raw.into_option() {
            string.parse().map(SpellingLanguage::Value)
        } else {
            Err(crate::language_tags::ParseError::EmptySubtag)
        }
    }
}

#[cfg(feature = "language-tags")]
impl From<SpellingLanguage> for RawValue {
    fn from(val: SpellingLanguage) -> RawValue {
        match val {
            SpellingLanguage::Value(v) => v.to_string().into(),
        }
    }
}

#[cfg(feature = "language-tags")]
impl PropertyKey for SpellingLanguage {
    fn key() -> &'static str {
        "spelling_language"
    }
}

#[cfg(feature = "language-tags")]
impl Display for SpellingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpellingLanguage::Value(v) => v.fmt(f),
        }
    }
}

/// All the keys of the standard properties.
///
/// Can be used to determine if a property is defined in the specification or not.
pub static STANDARD_KEYS: &[&str] = &[
    "indent_size",
    "indent_style",
    "tab_width",
    "end_of_line",
    "charset",
    "trim_trailing_whitespace",
    "insert_final_newline",
    "spelling_language",
    // NOT "max_line_length".
];
