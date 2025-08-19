use crate::SearchOptions;
use anyhow;
use fancy_regex::Regex;
use std::sync::LazyLock;

/// A `PatternItem` is a character, preceded by a backslash, that can be used to
/// modify the search options.
/// For example, using `\c` in a search query will make the search
/// case-insensitive, while `\C` will make it case-sensitive.
#[derive(Clone, Debug, PartialEq)]
enum PatternItem {
    CaseSensitiveFalse,
    CaseSensitiveTrue,
}

/// Regex for matching pattern items in a search query.
pub static PATTERN_ITEMS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?<!\\)(\\[{}])",
        PatternItem::all_variants()
            .iter()
            .map(|item| item.character())
            .collect::<String>()
    ))
    .expect("Failed to compile pattern items regex")
});

impl TryFrom<&str> for PatternItem {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str {
            "\\c" => Ok(Self::CaseSensitiveFalse),
            "\\C" => Ok(Self::CaseSensitiveTrue),
            _ => anyhow::bail!("Invalid pattern item: {}", str),
        }
    }
}

impl PatternItem {
    /// Representation of the pattern item as a single character, without the
    /// backslash.
    pub fn character(&self) -> char {
        match self {
            Self::CaseSensitiveFalse => 'c',
            Self::CaseSensitiveTrue => 'C',
        }
    }

    pub fn search_option(&self) -> (SearchOptions, bool) {
        match self {
            Self::CaseSensitiveFalse => (SearchOptions::CASE_SENSITIVE, false),
            Self::CaseSensitiveTrue => (SearchOptions::CASE_SENSITIVE, true),
        }
    }

    fn all_variants() -> &'static [Self] {
        &[Self::CaseSensitiveFalse, Self::CaseSensitiveTrue]
    }
}

#[derive(Default)]
pub struct PatternItems {
    items: Vec<PatternItem>,
}

impl PatternItems {
    /// Builds the list of pattern items that, from the provided search options
    /// and query, do actually affect the search options.
    /// For example, if search options is `SearchOptions::CASE_SENSITIVE`, and
    /// the query only contains `\C`, then the pattern item will not have an
    /// effect on the sarch options.
    pub fn from_search_options(search_options: SearchOptions, query: &str) -> Self {
        let mut search_options = search_options;
        let mut pattern_items: Vec<PatternItem> = Vec::new();

        Self::extract_from_query(query)
            .iter()
            .for_each(|pattern_item| {
                let (search_option, value) = pattern_item.search_option();

                if search_options.contains(search_option) != value {
                    search_options.toggle(search_option);
                    pattern_items.push(pattern_item.clone());
                }
            });

        Self {
            items: pattern_items,
        }
    }

    /// Replaces all pattern items in the provided string with an empty string.
    pub fn clean_query(str: &str) -> String {
        PATTERN_ITEMS_REGEX.replace_all(str, "").into_owned()
    }

    /// Calculates what the provided search options looked liked before the
    /// pattern items were applied.
    pub fn revert(&self, search_options: SearchOptions) -> SearchOptions {
        let mut result = search_options;

        self.items
            .iter()
            .rev()
            .map(PatternItem::search_option)
            .for_each(|(search_option, value)| result.set(search_option, !value));

        result
    }

    /// Returns the search options after applying the pattern items.
    pub fn apply(&self, search_options: SearchOptions) -> SearchOptions {
        let mut search_options = search_options;

        self.items
            .iter()
            .map(PatternItem::search_option)
            .for_each(|(search_option, value)| search_options.set(search_option, value));

        search_options
    }

    /// Extracts all pattern items from the provided string.
    fn extract_from_query(str: &str) -> Vec<PatternItem> {
        PATTERN_ITEMS_REGEX
            .captures_iter(str)
            .filter_map(|capture| capture.ok()?.get(1))
            .filter_map(|capture| PatternItem::try_from(capture.as_str()).ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_query() {
        let query = "Main\\c\\C";
        let cleaned_query = PatternItems::clean_query(query);
        assert_eq!(cleaned_query, "Main");
    }

    #[test]
    fn test_apply() {
        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\C";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        assert_eq!(pattern_items.apply(search_options), search_options);

        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\c";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        assert_eq!(pattern_items.apply(search_options), SearchOptions::NONE);

        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\c\\C";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        assert_eq!(pattern_items.apply(search_options), search_options);

        let search_options = SearchOptions::NONE;
        let query = "Main\\c\\C";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        assert_eq!(
            pattern_items.apply(search_options),
            SearchOptions::CASE_SENSITIVE
        );

        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\c\\C\\c";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        assert_eq!(pattern_items.apply(search_options), SearchOptions::NONE);
    }

    #[test]
    fn test_revert() {
        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\c";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        let updated_search_options = pattern_items.apply(search_options);
        assert_eq!(updated_search_options, SearchOptions::NONE);
        assert_eq!(pattern_items.revert(updated_search_options), search_options);

        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\c";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        let updated_search_options = pattern_items.apply(search_options);
        assert_eq!(updated_search_options, SearchOptions::NONE);
        assert_eq!(pattern_items.revert(updated_search_options), search_options);

        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\c\\C";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        let updated_search_options = pattern_items.apply(search_options);
        assert_eq!(updated_search_options, search_options);
        assert_eq!(pattern_items.revert(updated_search_options), search_options);

        let search_options = SearchOptions::NONE;
        let query = "Main\\c\\C";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        let updated_search_options = pattern_items.apply(search_options);
        assert_eq!(updated_search_options, SearchOptions::CASE_SENSITIVE);
        assert_eq!(pattern_items.revert(updated_search_options), search_options);

        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\c\\C\\c";
        let pattern_items = PatternItems::from_search_options(search_options, query);
        let updated_search_options = pattern_items.apply(search_options);
        assert_eq!(updated_search_options, SearchOptions::NONE);
        assert_eq!(pattern_items.revert(updated_search_options), search_options);
    }

    #[test]
    fn test_extract_from_query() {
        let query = "Main\\c\\C\\c";
        let pattern_items = PatternItems::extract_from_query(query);
        assert_eq!(
            pattern_items,
            vec![
                PatternItem::CaseSensitiveFalse,
                PatternItem::CaseSensitiveTrue,
                PatternItem::CaseSensitiveFalse
            ]
        );
    }

    #[test]
    fn test_from_search_options() {
        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\C";
        let pattern_items = PatternItems::from_search_options(search_options, query);

        assert_eq!(pattern_items.items, vec![]);

        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\c";
        let pattern_items = PatternItems::from_search_options(search_options, query);

        assert_eq!(pattern_items.items, vec![PatternItem::CaseSensitiveFalse]);

        let search_options = SearchOptions::CASE_SENSITIVE;
        let query = "Main\\C\\c\\C";
        let pattern_items = PatternItems::from_search_options(search_options, query);

        assert_eq!(
            pattern_items.items,
            vec![
                PatternItem::CaseSensitiveFalse,
                PatternItem::CaseSensitiveTrue
            ]
        );
    }
}
