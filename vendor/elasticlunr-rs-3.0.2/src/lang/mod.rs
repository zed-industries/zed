//! Intended to be compatible with <https://github.com/MihaiValentin/lunr-languages>. Each supported
//! language has a trimmer, a stop word filter, and a stemmer. Most users will not need to use
//! these modules directly.

pub mod common;

use crate::Pipeline;

pub trait Language {
    /// The name of the language in English
    fn name(&self) -> String;

    /// The ISO 639-1 language code of the language
    fn code(&self) -> String;

    /// Separates the input text into individual tokens. In most languages a token is a word, separated by whitespace.
    fn tokenize(&self, text: &str) -> Vec<String>;

    /// Returns the [`Pipeline`] to process the tokens with
    fn make_pipeline(&self) -> Pipeline;
}

/// Splits a text string into a vector of individual tokens.
pub fn tokenize_whitespace(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_lowercase())
        .collect()
}

macro_rules! impl_language {
    ($( ( $name:ident, $code:ident $(, #[$cfgs:meta] )? ), )+) => {
        /// Returns a list of all the [`Language`] implementations in the crate
        pub fn languages() -> Vec<Box<dyn Language>> {
            vec![
                $(
                    $(#[$cfgs])?
                    Box::new($code::$name::new()),
                )+
            ]
        }

        /// Returns the [`Language`] for the given two-character [ISO 639-1][iso] language code if the
        /// language is supported. Returns `None` if not supported.
        ///
        /// *Note:*
        ///
        /// The ISO 639-1 code for Dutch is "nl". However "du" is used for the module name
        /// and pipeline suffix in order to match lunr-languages.
        ///
        /// [iso]: https://en.wikipedia.org/wiki/ISO_639-1
        pub fn from_code(code: &str) -> Option<Box<dyn Language>> {
            match code.to_ascii_lowercase().as_str() {
                $(
                    $(#[$cfgs])?
                    stringify!($code) => Some(Box::new($code::$name::new())),
                )+
                _ => None,
            }
        }

        /// Returns the [`Language`] for the given English language name if the
        /// language is supported. Returns `None` if not supported. The first letter must
        /// be capitalized.
        pub fn from_name(name: &str) -> Option<Box<dyn Language>> {
            match name {
                $(
                    $(#[$cfgs])?
                    stringify!($name) => Some(Box::new($code::$name::new())),
                )+
                _ => None,
            }
        }

        $(
            $(#[$cfgs])?
            mod $code;

            $(#[$cfgs])?
            pub use $code::$name;
        )+
    };
}

impl_language! {
    (English, en),
    (Arabic, ar, #[cfg(feature = "ar")]),
    (Chinese, zh, #[cfg(feature = "zh")]),
    (Danish, da, #[cfg(feature = "da")]),
    (Dutch, du, #[cfg(feature = "du")]),
    (Finnish, fi, #[cfg(feature = "fi")]),
    (French, fr, #[cfg(feature = "fr")]),
    (German, de, #[cfg(feature = "de")]),
    (Hungarian, hu, #[cfg(feature = "hu")]),
    (Italian, it, #[cfg(feature = "it")]),
    (Japanese, ja, #[cfg(feature = "ja")]),
    (Korean, ko, #[cfg(feature = "ko")]),
    (Norwegian, no, #[cfg(feature = "no")]),
    (Portuguese, pt, #[cfg(feature = "pt")]),
    (Romanian, ro, #[cfg(feature = "ro")]),
    (Russian, ru, #[cfg(feature = "ru")]),
    (Spanish, es, #[cfg(feature = "es")]),
    (Swedish, sv, #[cfg(feature = "sv")]),
    (Turkish, tr, #[cfg(feature = "tr")]),
}

#[cfg(test)]
mod tests {
    use super::tokenize_whitespace;

    #[test]
    fn split_simple_strings() {
        let string = "this is a simple string";
        assert_eq!(
            &tokenize_whitespace(string),
            &["this", "is", "a", "simple", "string"]
        );
    }

    #[test]
    fn multiple_white_space() {
        let string = "  foo    bar  ";
        assert_eq!(&tokenize_whitespace(string), &["foo", "bar"]);
    }

    #[test]
    fn hyphens() {
        let string = "take the New York-San Francisco flight";
        assert_eq!(
            &tokenize_whitespace(string),
            &["take", "the", "new", "york", "san", "francisco", "flight"]
        );
    }

    #[test]
    fn splitting_strings_with_hyphens() {
        let string = "Solve for A - B";
        assert_eq!(&tokenize_whitespace(string), &["solve", "for", "a", "b"]);
    }
}
