/// Same as [`std::vec!`] but converts each element with [`Into`].
///
/// **WARNING:** it's not recommended to import this macro into scope. Reference it
/// using the full path (`bon::vec![]`) to avoid confusion with the [`std::vec!`] macro.
///
/// A good example of the use case for this macro is when you want to create a
/// [`Vec<String>`] where part of the items are hard-coded string literals of type
/// `&str` and the other part is made of dynamic [`String`] values.
///
/// ```
/// fn convert_media(input_extension: &str, output_extension: &str) -> std::io::Result<()> {
///     let ffmpeg_args: Vec<String> = bon::vec![
///         "-i",
///         format!("input.{input_extension}"),
///         "-y",
///         format!("output.{output_extension}"),
///     ];
///
///     std::process::Command::new("ffmpeg").args(ffmpeg_args).output()?;
///
///     Ok(())
/// }
/// ```
///
/// This macro doesn't support `vec![expr; N]` syntax, since it's simpler to
/// just write `vec![expr.into(); N]` using [`std::vec!`] instead.
#[macro_export]
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
#[allow(edition_2024_expr_fragment_specifier)]
macro_rules! vec {
    () => ($crate::__::alloc::vec::Vec::new());
    ($($item:expr),+ $(,)?) => ($crate::__::alloc::vec![$(::core::convert::Into::into($item)),+ ]);
}

/// Creates a fixed-size array literal with each element converted with [`Into`].
///
/// You'll probably need a hint for the target type of items in the array if the
/// compiler can't infer it from its usage.
///
/// This is similar in spirit to the [`bon::vec!`] macro, but it's for arrays.
/// See [`bon::vec!`] docs for details.
///
/// Same example as in [`bon::vec!`], but using this macro. It works with array
/// as well because [`Command::args`] accepts any value that implements [`IntoIterator`]:
///
/// ```
/// fn convert_media(input_extension: &str, output_extension: &str) -> std::io::Result<()> {
///     let ffmpeg_args: [String; 4] = bon::arr![
///         "-i",
///         format!("input.{input_extension}"),
///         "-y",
///         format!("output.{output_extension}"),
///     ];
///
///     std::process::Command::new("ffmpeg").args(ffmpeg_args).output()?;
///
///     Ok(())
/// }
/// ```
///
/// This macro doesn't support `[expr; N]` syntax, since it's simpler to
/// just write `[expr.into(); N]` instead.
///
/// [`Command::args`]: std::process::Command::args
/// [`bon::vec!`]: crate::vec
#[macro_export]
#[allow(edition_2024_expr_fragment_specifier)]
macro_rules! arr {
    () => ([]);
    ($($item:expr),+ $(,)?) => ([$(::core::convert::Into::into($item)),+]);
}

#[cfg(test)]
mod tests {
    // It's okay to use indexing in tests for conciseness.
    #![allow(clippy::indexing_slicing)]

    #[cfg(feature = "alloc")]
    use crate::__::alloc::{string::String, vec::Vec};
    use core::num::NonZeroU8;

    #[cfg(feature = "alloc")]
    #[test]
    fn arr_of_strings() {
        let actual: [String; 3] = crate::arr!["foo", "bar", "baz"];
        assert_eq!(actual, ["foo", "bar", "baz"]);

        let actual: [String; 0] = crate::arr![];
        assert!(actual.is_empty());
    }

    #[test]
    fn arr_of_numbers() {
        let actual: [u8; 2] = crate::arr![NonZeroU8::new(1).unwrap(), NonZeroU8::new(2).unwrap()];
        assert_eq!(actual, [1, 2]);

        let actual: [u8; 0] = crate::arr![];
        assert!(actual.is_empty());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn vec_smoke() {
        let actual: Vec<String> = crate::vec!["foo", "bar", "baz"];
        assert_eq!(actual, ["foo", "bar", "baz"]);

        let actual: Vec<String> = crate::vec![];
        assert!(actual.is_empty());
    }

    #[cfg(feature = "std")]
    #[test]
    fn map_smoke() {
        use std::collections::{BTreeMap, HashMap};

        let hash_strings: HashMap<String, String> = crate::map! {
            "Hello": "World",
            "Goodbye": "Mars",
        };

        assert_eq!(hash_strings["Hello"], "World");
        assert_eq!(hash_strings["Goodbye"], "Mars");

        let tree_strings: BTreeMap<String, String> = crate::map! {
            "Hello": "World",
            "Goodbye": "Mars",
        };

        assert_eq!(tree_strings["Hello"], "World");
        assert_eq!(tree_strings["Goodbye"], "Mars");
    }

    #[cfg(feature = "std")]
    #[test]
    fn set_smoke() {
        use std::collections::BTreeSet;
        use std::collections::HashSet;

        let hash_strings: HashSet<String> = crate::set!["Hello", "World", "Goodbye", "Mars"];

        assert!(hash_strings.contains("Hello"));
        assert!(hash_strings.contains("World"));
        assert!(hash_strings.contains("Goodbye"));
        assert!(hash_strings.contains("Mars"));

        let tree_strings: BTreeSet<String> = crate::set!["Hello", "World", "Goodbye", "Mars"];

        assert!(tree_strings.contains("Hello"));
        assert!(tree_strings.contains("World"));
        assert!(tree_strings.contains("Goodbye"));
        assert!(tree_strings.contains("Mars"));
    }
}
