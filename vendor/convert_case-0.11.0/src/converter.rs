use crate::boundary;
use crate::boundary::Boundary;
use crate::pattern::Pattern;
use crate::Case;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// The parameters for performing a case conversion.
///
/// A `Converter` stores three fields needed for case conversion.
/// 1) `boundaries`: how a string is split into _words_.
/// 2) `patterns`: how words are mutated, or how each character's case will change.
/// 3) `delimiter`: how the mutated words are joined into the final string.
///
/// Then calling [`convert`](Converter::convert) on a `Converter` will apply a case conversion
/// defined by those fields.  The `Converter` struct is what is used underneath those functions
/// available in the `Casing` struct.
///
/// You can use `Converter` when you need more specificity on conversion
/// than those provided in `Casing`, or if it is simply more convenient or explicit.
///
/// ```
/// use convert_case::{Boundary, Case, Casing, Converter, Pattern};
///
/// let s = "DialogueBox-border-shadow";
///
/// // Convert using Casing trait
/// assert_eq!(
///     s.from_case(Case::Kebab).to_case(Case::Snake),
///     "dialoguebox_border_shadow",
/// );
///
/// // Convert using similar methods on Converter
/// let conv = Converter::new()
///     .from_case(Case::Kebab)
///     .to_case(Case::Snake);
/// assert_eq!(conv.convert(s), "dialoguebox_border_shadow");
///
/// // Convert by setting each field explicitly
/// let conv = Converter::new()
///     .set_boundaries(&[Boundary::Hyphen])
///     .set_patterns(&[Pattern::Lowercase])
///     .set_delimiter("_");
/// assert_eq!(conv.convert(s), "dialoguebox_border_shadow");
/// ```
///
/// Or you can use `Converter` when you are performing a transformation
/// not provided as a variant of `Case`.
///
/// ```
/// # use convert_case::{Boundary, Case, Casing, Converter, Pattern};
/// let dot_camel = Converter::new()
///     .set_boundaries(&[Boundary::LowerUpper, Boundary::LowerDigit])
///     .set_patterns(&[Pattern::Camel])
///     .set_delimiter(".");
/// assert_eq!(dot_camel.convert("CollisionShape2D"), "collision.Shape.2d");
/// ```
pub struct Converter {
    /// How a string is split into words.
    pub boundaries: Vec<Boundary>,

    /// How each word is mutated before joining.
    pub patterns: Vec<Pattern>,

    /// The string used to join mutated words together.
    pub delimiter: String,
}

impl Default for Converter {
    fn default() -> Self {
        Converter {
            boundaries: Boundary::defaults().to_vec(),
            patterns: Vec::new(),
            delimiter: String::new(),
        }
    }
}

impl Converter {
    /// Creates a new `Converter` with default fields.  This is the same as `Default::default()`.
    /// The `Converter` will use [`Boundary::defaults()`] for boundaries, no patterns, and an empty
    /// string as a delimiter.
    /// ```
    /// # use convert_case::Converter;
    /// let conv = Converter::new();
    /// assert_eq!(conv.convert("Ice-cream TRUCK"), "IcecreamTRUCK")
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts a string.
    /// ```
    /// # use convert_case::{Case, Converter};
    /// let conv = Converter::new()
    ///     .to_case(Case::Camel);
    /// assert_eq!(conv.convert("XML_HTTP_Request"), "xmlHttpRequest")
    /// ```
    pub fn convert<T>(&self, s: T) -> String
    where
        T: AsRef<str>,
    {
        let words = boundary::split(&s, &self.boundaries);

        let mut result: Vec<String> = words.into_iter().map(|s| s.to_string()).collect();
        for pattern in &self.patterns {
            result = pattern.mutate(&result);
        }

        result.join(&self.delimiter)
    }

    /// Set the pattern and delimiter to those associated with the given case.
    /// ```
    /// # use convert_case::{Case, Converter};
    /// let conv = Converter::new()
    ///     .to_case(Case::Pascal);
    /// assert_eq!(conv.convert("variable name"), "VariableName")
    /// ```
    pub fn to_case(mut self, case: Case) -> Self {
        self.patterns.push(case.pattern());
        self.delimiter = case.delimiter().to_string();
        self
    }

    /// Sets the boundaries to those associated with the provided case.  This is used
    /// by the `from_case` function in the `Casing` trait.
    /// ```
    /// # use convert_case::{Case, Converter};
    /// let conv = Converter::new()
    ///     .from_case(Case::Snake)
    ///     .to_case(Case::Title);
    /// assert_eq!(conv.convert("dot_productValue"), "Dot Productvalue")
    /// ```
    pub fn from_case(mut self, case: Case) -> Self {
        self.boundaries = case.boundaries().to_vec();
        self
    }

    /// Sets the boundaries to those provided.
    /// ```
    /// # use convert_case::{Boundary, Case, Converter};
    /// let conv = Converter::new()
    ///     .set_boundaries(&[Boundary::Underscore, Boundary::LowerUpper])
    ///     .to_case(Case::Lower);
    /// assert_eq!(conv.convert("firstName_lastName"), "first name last name");
    /// ```
    pub fn set_boundaries(mut self, bs: &[Boundary]) -> Self {
        self.boundaries = bs.to_vec();
        self
    }

    /// Adds a boundary to the list of boundaries.
    /// ```
    /// # use convert_case::{Boundary, Case, Converter};
    /// let conv = Converter::new()
    ///     .from_case(Case::Title)
    ///     .add_boundary(Boundary::Hyphen)
    ///     .to_case(Case::Snake);
    /// assert_eq!(conv.convert("My Biography - Video 1"), "my_biography___video_1")
    /// ```
    pub fn add_boundary(mut self, b: Boundary) -> Self {
        self.boundaries.push(b);
        self
    }

    /// Adds a vector of boundaries to the list of boundaries.
    /// ```
    /// # use convert_case::{Boundary, Case, Converter};
    /// let conv = Converter::new()
    ///     .from_case(Case::Kebab)
    ///     .to_case(Case::Title)
    ///     .add_boundaries(&[Boundary::Underscore, Boundary::LowerUpper]);
    /// assert_eq!(conv.convert("2020-10_firstDay"), "2020 10 First Day");
    /// ```
    pub fn add_boundaries(mut self, bs: &[Boundary]) -> Self {
        self.boundaries.extend(bs);
        self
    }

    /// Removes a boundary from the list of boundaries if it exists.
    ///
    /// Note: [`Boundary::Custom`] variants are never considered equal due to
    /// function pointer comparison limitations, so they cannot be removed using this method.
    /// Recall that the default boundaries include no custom enumerations.
    /// ```
    /// # use convert_case::{Boundary, Case, Converter};
    /// let conv = Converter::new()
    ///     .remove_boundary(Boundary::Acronym)
    ///     .to_case(Case::Kebab);
    /// assert_eq!(conv.convert("HTTPRequest_parser"), "httprequest-parser");
    /// ```
    pub fn remove_boundary(mut self, b: Boundary) -> Self {
        self.boundaries.retain(|&x| x != b);
        self
    }

    /// Removes all the provided boundaries from the list of boundaries if it exists.
    ///
    /// Note: [`Boundary::Custom`] variants are never considered equal due to
    /// function pointer comparison limitations, so they cannot be removed using this method.
    /// Recall that the default boundaries include no custom enumerations.
    /// ```
    /// # use convert_case::{Boundary, Case, Converter};
    /// let conv = Converter::new()
    ///     .remove_boundaries(&Boundary::digits())
    ///     .to_case(Case::Snake);
    /// assert_eq!(conv.convert("C04 S03 Path Finding.pdf"), "c04_s03_path_finding.pdf");
    /// ```
    pub fn remove_boundaries(mut self, bs: &[Boundary]) -> Self {
        for b in bs {
            self.boundaries.retain(|&x| x != *b);
        }
        self
    }

    /// Sets a single pattern, replacing any existing patterns.
    /// ```
    /// # use convert_case::{Converter, Pattern};
    /// let conv = Converter::new()
    ///     .set_delimiter("_")
    ///     .set_pattern(Pattern::Sentence);
    /// assert_eq!(conv.convert("BJARNE CASE"), "Bjarne_case");
    /// ```
    pub fn set_pattern(mut self, p: Pattern) -> Self {
        self.patterns = vec![p];
        self
    }

    /// Sets the patterns to those provided, replacing any existing patterns.
    /// An empty slice means no mutation (words pass through unchanged).
    /// ```
    /// # use convert_case::{Converter, Pattern};
    /// let conv = Converter::new()
    ///     .set_delimiter("_")
    ///     .set_patterns(&[Pattern::Sentence]);
    /// assert_eq!(conv.convert("BJARNE CASE"), "Bjarne_case");
    /// ```
    pub fn set_patterns(mut self, ps: &[Pattern]) -> Self {
        self.patterns = ps.to_vec();
        self
    }

    /// Adds a pattern to the end of the pattern list.
    /// Patterns are applied in order, so this pattern will be applied last.
    /// ```
    /// # use convert_case::{Case, Converter, Pattern};
    /// let conv = Converter::new()
    ///     .from_case(Case::Kebab)
    ///     .add_pattern(Pattern::RemoveEmpty)
    ///     .add_pattern(Pattern::Camel);
    /// assert_eq!(conv.convert("--leading-delims"), "leadingDelims");
    /// ```
    pub fn add_pattern(mut self, p: Pattern) -> Self {
        self.patterns.push(p);
        self
    }

    /// Adds multiple patterns to the end of the pattern list.
    /// ```
    /// # use convert_case::{Converter, Pattern};
    /// let conv = Converter::new()
    ///     .add_patterns(&[Pattern::RemoveEmpty, Pattern::Lowercase]);
    /// ```
    pub fn add_patterns(mut self, ps: &[Pattern]) -> Self {
        self.patterns.extend(ps);
        self
    }

    /// Removes a pattern from the list if it exists.
    ///
    /// Note: [`Pattern::Custom`] variants are never considered equal due to
    /// function pointer comparison limitations, so they cannot be removed using this method.
    /// ```
    /// # use convert_case::{Boundary, Case, Converter, Pattern};
    /// let conv = Converter::new()
    ///     .set_boundaries(&[Boundary::Space])
    ///     .to_case(Case::Snake)
    ///     .remove_pattern(Pattern::Lowercase);
    /// assert_eq!(conv.convert("HeLLo WoRLD"), "HeLLo_WoRLD");
    /// ```
    pub fn remove_pattern(mut self, p: Pattern) -> Self {
        self.patterns.retain(|&x| x != p);
        self
    }

    /// Removes all specified patterns from the list.
    ///
    /// Note: [`Pattern::Custom`] variants are never considered equal due to
    /// function pointer comparison limitations, so they cannot be removed using this method.
    /// ```
    /// # use convert_case::{Converter, Pattern};
    /// let conv = Converter::new()
    ///     .set_patterns(&[Pattern::RemoveEmpty, Pattern::Lowercase, Pattern::Capital])
    ///     .remove_patterns(&[Pattern::Lowercase, Pattern::Capital]);
    /// // Only RemoveEmpty remains
    /// ```
    pub fn remove_patterns(mut self, ps: &[Pattern]) -> Self {
        for p in ps {
            self.patterns.retain(|&x| x != *p);
        }
        self
    }

    /// Sets the delimiter.
    /// ```
    /// # use convert_case::{Case, Converter};
    /// let conv = Converter::new()
    ///     .to_case(Case::Snake)
    ///     .set_delimiter(".");
    /// assert_eq!(conv.convert("LowerWithDots"), "lower.with.dots");
    /// ```
    pub fn set_delimiter<T>(mut self, d: T) -> Self
    where
        T: ToString,
    {
        self.delimiter = d.to_string();
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Casing;

    #[test]
    fn snake_converter_from_case() {
        let conv = Converter::new().to_case(Case::Snake);
        let s = String::from("my var name");
        assert_eq!(s.to_case(Case::Snake), conv.convert(s));
    }

    #[test]
    fn snake_converter_from_scratch() {
        let conv = Converter::new()
            .set_delimiter("_")
            .set_patterns(&[Pattern::Lowercase]);
        let s = String::from("my var name");
        assert_eq!(s.to_case(Case::Snake), conv.convert(s));
    }

    #[test]
    fn custom_pattern() {
        let conv = Converter::new()
            .to_case(Case::Snake)
            .set_patterns(&[Pattern::Sentence]);
        assert_eq!(conv.convert("bjarne case"), "Bjarne_case");
    }

    #[test]
    fn custom_delim() {
        let conv = Converter::new().set_delimiter("..");
        assert_eq!(conv.convert("ohMy"), "oh..My");
    }

    #[test]
    fn no_delim() {
        let conv = Converter::new()
            .from_case(Case::Title)
            .to_case(Case::Kebab)
            .set_delimiter("");
        assert_eq!(conv.convert("Just Flat"), "justflat");
    }

    #[test]
    fn no_digit_boundaries() {
        let conv = Converter::new()
            .remove_boundaries(&Boundary::digits())
            .to_case(Case::Snake);
        assert_eq!(conv.convert("Test 08Bound"), "test_08bound");
        assert_eq!(conv.convert("a8aA8A"), "a8a_a8a");
    }

    #[test]
    fn remove_boundary() {
        let conv = Converter::new()
            .remove_boundary(Boundary::DigitUpper)
            .to_case(Case::Snake);
        assert_eq!(conv.convert("Test 08Bound"), "test_08bound");
        assert_eq!(conv.convert("a8aA8A"), "a_8_a_a_8a");
    }

    #[test]
    fn add_boundary() {
        let conv = Converter::new()
            .from_case(Case::Snake)
            .to_case(Case::Kebab)
            .add_boundary(Boundary::LowerUpper);
        assert_eq!(conv.convert("word_wordWord"), "word-word-word");
    }

    #[test]
    fn add_boundaries() {
        let conv = Converter::new()
            .from_case(Case::Snake)
            .to_case(Case::Kebab)
            .add_boundaries(&[Boundary::LowerUpper, Boundary::UpperLower]);
        assert_eq!(conv.convert("word_wordWord"), "word-word-w-ord");
    }

    #[test]
    fn twice() {
        let s = "myVarName".to_string();
        let conv = Converter::new().to_case(Case::Snake);
        let snake = conv.convert(&s);
        let kebab = s.to_case(Case::Kebab);
        assert_eq!(snake.to_case(Case::Camel), kebab.to_case(Case::Camel));
    }

    #[test]
    fn reuse_after_change() {
        let conv = Converter::new().from_case(Case::Snake).to_case(Case::Kebab);
        assert_eq!(conv.convert("word_wordWord"), "word-wordword");

        let conv = conv.add_boundary(Boundary::LowerUpper);
        assert_eq!(conv.convert("word_wordWord"), "word-word-word");
    }

    #[test]
    fn explicit_boundaries() {
        let conv = Converter::new()
            .set_boundaries(&[
                Boundary::DigitLower,
                Boundary::DigitUpper,
                Boundary::Acronym,
            ])
            .to_case(Case::Snake);
        assert_eq!(
            conv.convert("section8lesson2HTTPRequests"),
            "section8_lesson2_http_requests"
        );
    }
}
