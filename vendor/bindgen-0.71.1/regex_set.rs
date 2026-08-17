//! A type that represents the union of a set of regular expressions.
#![deny(clippy::missing_docs_in_private_items)]

use regex::RegexSet as RxSet;
use std::cell::Cell;

/// A dynamic set of regular expressions.
#[derive(Clone, Debug, Default)]
pub(crate) struct RegexSet {
    items: Vec<Box<str>>,
    /// Whether any of the items in the set was ever matched. The length of this
    /// vector is exactly the length of `items`.
    matched: Vec<Cell<bool>>,
    set: Option<RxSet>,
    /// Whether we should record matching items in the `matched` vector or not.
    record_matches: bool,
}

impl RegexSet {
    /// Is this set empty?
    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Insert a new regex into this set.
    pub(crate) fn insert<S>(&mut self, string: S)
    where
        S: AsRef<str>,
    {
        self.items.push(string.as_ref().to_owned().into_boxed_str());
        self.matched.push(Cell::new(false));
        self.set = None;
    }

    /// Returns slice of String from its field 'items'
    pub(crate) fn get_items(&self) -> &[Box<str>] {
        &self.items
    }

    /// Returns an iterator over regexes in the set which didn't match any
    /// strings yet.
    pub(crate) fn unmatched_items(&self) -> impl Iterator<Item = &str> {
        self.items.iter().enumerate().filter_map(move |(i, item)| {
            if !self.record_matches || self.matched[i].get() {
                return None;
            }

            Some(item.as_ref())
        })
    }

    /// Construct a `RegexSet` from the set of entries we've accumulated.
    ///
    /// Must be called before calling `matches()`, or it will always return
    /// false.
    #[inline]
    #[allow(unused)]
    pub(crate) fn build(&mut self, record_matches: bool) {
        self.build_inner(record_matches, None);
    }

    #[cfg(all(feature = "__cli", feature = "experimental"))]
    /// Construct a `RegexSet` from the set of entries we've accumulated and emit diagnostics if the
    /// name of the regex set is passed to it.
    ///
    /// Must be called before calling `matches()`, or it will always return
    /// false.
    #[inline]
    pub(crate) fn build_with_diagnostics(
        &mut self,
        record_matches: bool,
        name: Option<&'static str>,
    ) {
        self.build_inner(record_matches, name);
    }

    #[cfg(all(not(feature = "__cli"), feature = "experimental"))]
    /// Construct a RegexSet from the set of entries we've accumulated and emit diagnostics if the
    /// name of the regex set is passed to it.
    ///
    /// Must be called before calling `matches()`, or it will always return
    /// false.
    #[inline]
    pub(crate) fn build_with_diagnostics(
        &mut self,
        record_matches: bool,
        name: Option<&'static str>,
    ) {
        self.build_inner(record_matches, name);
    }

    fn build_inner(
        &mut self,
        record_matches: bool,
        _name: Option<&'static str>,
    ) {
        let items = self.items.iter().map(|item| format!("^({item})$"));
        self.record_matches = record_matches;
        self.set = match RxSet::new(items) {
            Ok(x) => Some(x),
            Err(e) => {
                warn!("Invalid regex in {:?}: {e:?}", self.items);
                #[cfg(feature = "experimental")]
                if let Some(name) = _name {
                    invalid_regex_warning(self, e, name);
                }
                None
            }
        }
    }

    /// Does the given `string` match any of the regexes in this set?
    pub(crate) fn matches<S>(&self, string: S) -> bool
    where
        S: AsRef<str>,
    {
        let s = string.as_ref();
        let Some(ref set) = self.set else {
            return false;
        };

        if !self.record_matches {
            return set.is_match(s);
        }

        let matches = set.matches(s);
        if !matches.matched_any() {
            return false;
        }
        for i in &matches {
            self.matched[i].set(true);
        }

        true
    }
}

#[cfg(feature = "experimental")]
fn invalid_regex_warning(
    set: &RegexSet,
    err: regex::Error,
    name: &'static str,
) {
    use crate::diagnostics::{Diagnostic, Level, Slice};

    let mut diagnostic = Diagnostic::default();

    match err {
        regex::Error::Syntax(string) => {
            if string.starts_with("regex parse error:\n") {
                let mut source = String::new();

                let mut parsing_source = true;

                for line in string.lines().skip(1) {
                    if parsing_source {
                        if line.starts_with(' ') {
                            source.push_str(line);
                            source.push('\n');
                            continue;
                        }
                        parsing_source = false;
                    }
                    let error = "error: ";
                    if line.starts_with(error) {
                        let (_, msg) = line.split_at(error.len());
                        diagnostic.add_annotation(msg.to_owned(), Level::Error);
                    } else {
                        diagnostic.add_annotation(line.to_owned(), Level::Info);
                    }
                }
                let mut slice = Slice::default();
                slice.with_source(source);
                diagnostic.add_slice(slice);

                diagnostic.with_title(
                    "Error while parsing a regular expression.",
                    Level::Warning,
                );
            } else {
                diagnostic.with_title(string, Level::Warning);
            }
        }
        err => {
            let err = err.to_string();
            diagnostic.with_title(err, Level::Warning);
        }
    }

    diagnostic.add_annotation(
        format!("This regular expression was passed via `{name}`."),
        Level::Note,
    );

    if set.items.iter().any(|item| item.as_ref() == "*") {
        diagnostic.add_annotation("Wildcard patterns \"*\" are no longer considered valid. Use \".*\" instead.", Level::Help);
    }
    diagnostic.display();
}
