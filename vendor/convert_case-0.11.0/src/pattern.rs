use alloc::string::{String, ToString};
use alloc::vec::Vec;

fn lowercase_word(word: &str) -> String {
    word.to_lowercase()
}

fn uppercase_word(word: &str) -> String {
    word.to_uppercase()
}

/// Applies capital pattern to a single word using graphemes.
fn capital_word(word: &str) -> String {
    let mut chars = word.chars();

    if let Some(c) = chars.next() {
        [c.to_uppercase().collect(), chars.as_str().to_lowercase()].concat()
    } else {
        String::new()
    }
}

/// Transformations on a list of words.
///
/// A pattern is a function that maps a list of words into another list
/// after changing the casing of each letter.  How a patterns mutates
/// each letter can be dependent on the word the letters are present in.
///
/// ## Custom Pattern
///
/// A pattern is a function that maps from a borrowed list of words `&[&str]` to
/// an owned list of owned words `Vec<String>` by applying a transformation.
/// One example of custom behavior might be a pattern
/// that detects a fixed list of two-letter acronyms, and capitalizes them
/// appropriately on output.
/// ```
/// use convert_case::{Converter, Pattern};
///
/// fn pascal_upper_acronyms(words: &[&str]) -> Vec<String> {
///     Pattern::Capital.mutate(words).into_iter()
///         .map(|word| match word.as_ref() {
///             "Io" | "Xml" => word.to_uppercase(),
///             _ => word,
///         })
///         .collect()
/// }
///
/// let acronym_converter = Converter::new()
///     .set_patterns(&[Pattern::Custom(pascal_upper_acronyms)]);
///
/// assert_eq!(acronym_converter.convert("io_stream"), "IOStream");
/// assert_eq!(acronym_converter.convert("xml request"), "XMLRequest");
/// ```
///
/// Another example might be a one that explicitly adds leading
/// and trailing double underscores.  We do this by modifying the words directly,
/// which will get passed as-is to the join function.
/// ```
/// use convert_case::{Converter, Pattern};
///
/// fn snake_dunder(mut words: &[&str]) -> Vec<String> {
///     words
///         .into_iter()
///         .map(|word| word.to_lowercase())
///         .enumerate()
///         .map(|(i, word)| {
///             if words.len() == 1 {
///                 format!("__{}__", word)
///             } else if i == 0 {
///                 format!("__{}", word)
///             } else if i == words.len() - 1 {
///                 format!("{}__", word)
///             } else {
///                 word
///             }
///         })
///         .collect()
/// }
///
/// let dunder_converter = Converter::new()
///     .set_patterns(&[Pattern::Custom(snake_dunder)])
///     .set_delimiter("_");
///
/// assert_eq!(dunder_converter.convert("getAttr"), "__get_attr__");
/// assert_eq!(dunder_converter.convert("ITER"), "__iter__");
/// ```
#[derive(Debug, Copy, Clone)]
pub enum Pattern {
    /// Makes all words lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Lowercase.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["case", "conversion", "library"],
    /// );
    /// ```
    Lowercase,

    /// Makes all words uppercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Uppercase.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["CASE", "CONVERSION", "LIBRARY"],
    /// );
    /// ```
    Uppercase,

    /// Makes the first letter of each word uppercase
    /// and the remaining letters of each word lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Capital.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["Case", "Conversion", "Library"],
    /// );
    /// ```
    Capital,

    /// Makes the first non-empty word lowercase and the
    /// remaining capitalized.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Camel.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["case", "Conversion", "Library"],
    /// );
    /// ```
    Camel,

    /// Makes the first non-empty word capitalized and the
    /// remaining lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Sentence.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["Case", "conversion", "library"],
    /// );
    /// ```
    Sentence,

    /// Filters out empty words from the list.
    /// Useful when splitting produces empty words from leading/trailing/duplicate delimiters.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::RemoveEmpty.mutate(&["", "first", "", "second", ""]),
    ///     vec!["first", "second"],
    /// );
    /// ```
    RemoveEmpty,

    /// Define custom behavior to transform a set of words.
    ///
    /// See the [`Pattern`] documentation for examples.
    Custom(fn(&[&str]) -> Vec<String>),
}

impl Pattern {
    /// Applies the pattern transformation to a list of words.
    pub fn mutate<S: AsRef<str>>(&self, words: &[S]) -> Vec<String> {
        use Pattern::*;
        match self {
            Custom(transformation) => {
                let borrowed: Vec<&str> = words.iter().map(|s| s.as_ref()).collect();
                (transformation)(&borrowed)
            }
            Lowercase => words
                .iter()
                .map(|word| lowercase_word(word.as_ref()))
                .collect(),
            Uppercase => words
                .iter()
                .map(|word| uppercase_word(word.as_ref()))
                .collect(),
            Capital => words
                .iter()
                .map(|word| capital_word(word.as_ref()))
                .collect(),
            Camel => words
                .iter()
                .enumerate()
                .map(|(i, word)| {
                    if i == 0 {
                        lowercase_word(word.as_ref())
                    } else {
                        capital_word(word.as_ref())
                    }
                })
                .collect(),
            Sentence => words
                .iter()
                .enumerate()
                .map(|(i, word)| {
                    if i == 0 {
                        capital_word(word.as_ref())
                    } else {
                        lowercase_word(word.as_ref())
                    }
                })
                .collect(),
            RemoveEmpty => words
                .iter()
                .filter(|word| !word.as_ref().is_empty())
                .map(|word| word.as_ref().to_string())
                .collect(),
        }
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Lowercase, Self::Lowercase) => true,
            (Self::Uppercase, Self::Uppercase) => true,
            (Self::Capital, Self::Capital) => true,
            (Self::Camel, Self::Camel) => true,
            (Self::Sentence, Self::Sentence) => true,
            (Self::RemoveEmpty, Self::RemoveEmpty) => true,
            // Custom patterns are never equal because they contain function pointers,
            // which cannot be reliably compared.
            (Self::Custom(_), Self::Custom(_)) => false,
            _ => false,
        }
    }
}

impl Eq for Pattern {}

impl core::hash::Hash for Pattern {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        // Hash the discriminant for all variants
        core::mem::discriminant(self).hash(state);
        // Custom variants only hash the discriminant since they can't be meaningfully compared
    }
}

#[cfg(test)]
mod test {
    use crate::Case;
    use crate::Converter;

    use super::*;

    #[test]
    fn mutate_empty_strings() {
        for word_pattern in [lowercase_word, uppercase_word, capital_word] {
            assert_eq!(String::new(), word_pattern(&String::new()))
        }
    }

    #[test]
    fn filtering_with_remove_empty() {
        let conv = Converter::new()
            .from_case(Case::Kebab)
            .set_patterns(&[Pattern::RemoveEmpty, Pattern::Camel]);

        assert_eq!(conv.convert("--leading-delims"), "leadingDelims");
    }

    #[test]
    fn remove_empty_pattern() {
        assert_eq!(
            Pattern::RemoveEmpty.mutate(&["", "first", "", "second", ""]),
            vec!["first", "second"]
        );
        assert_eq!(Pattern::RemoveEmpty.mutate(&["only"]), vec!["only"]);
        assert_eq!(
            Pattern::RemoveEmpty.mutate(&["", "", ""]),
            Vec::<String>::new()
        );
    }
}
