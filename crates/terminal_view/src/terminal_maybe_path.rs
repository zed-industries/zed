//! Heuristics which define variations of a [MaybePathLike] from [terminal]
//!
//! # TODOs
//! ## [Cmd+click to linkify file in terminal doesn't work when there are whitespace or certain separators in the filename](https://github.com/zed-industries/zed/issues/12338)
//!
//! - [ ] Move tests to a checked-in baseline file for expected, to prioritized maintaining fast build times over fast test execution
//! - [ ] Add many more tests
//!     - [ ] `file://` Urls
//!     - [ ] Lots of edge cases
//!
//! ### Issues found while testing this feature
//! - [ ] Navigation to line and column navigates to the wrong column when line
//! contains unicode. I suspect it is using char's instead of graphemes.
//! - [x] When sending NewNaviagationTarget(None), we were not also clearning last_hovered_word, but we should.
//! - [ ] When holding Cmd, and the terminal output is scrolling, the link is highlighted, but after scrolling
//! away, it is still hyperlinking whatever random text is where the original link was.
//! - [ ] When holding Cmd, and the terminal contents are not scrolling, but a command is running that is adding
//! output off screen, the hovered link moves down one line for each new line of content added off screen, hyperlinking
//! whatever random text is there.
//! - [ ] Zed's tooltips don't render markdown tables correctly
//! - [ ] On Windows, PS terminal doesn't hyperlink any paths
//! - [ ] Wiggling the mouse over the terminal window (with no keys pressed) consumues 1 full cpu core. Seems sub-optimal.
//! - [ ] After Cmd-click navigating, when the mouse in the terminal, but not over any word (over empty space), pressing
//! Cmd causes the previously linkified path to linkify again.

use fancy_regex::Regex;
use itertools::Itertools;
use std::{
    borrow::Cow,
    fmt::Display,
    iter,
    ops::{Deref, Range},
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};
use terminal::{
    terminal_maybe_path_like::{
        has_common_surrounding_symbols, longest_surrounding_symbols_match,
        path_with_position_regex_match, preapproved_path_hyperlink_regexes, MaybePathLike,
        RowColumn, MAIN_SEPARATORS,
    },
    terminal_settings::PathHyperlinkNavigation,
};
#[cfg(doc)]
use util::paths::PathWithPosition;

fn word_regex() -> &'static Regex {
    static WORD_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(terminal::WORD_REGEX).unwrap());
    &WORD_REGEX
}

/// The `line` and the `word_range` from hovered or Cmd-clicked [MaybePathLike] from [terminal]
#[derive(Clone, Debug)]
pub struct MaybePath {
    line: String,
    word_range: Range<usize>,
    path_hyperlink_regexes: Arc<Vec<Regex>>,
}

impl Display for MaybePath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.word_range.start != 0 || self.word_range.end != self.line.len() {
            formatter.write_fmt(format_args!(
                "{:?} «{}»",
                self,
                &self.line[self.word_range.clone()]
            ))
        } else {
            formatter.write_fmt(format_args!("{:?}", self))
        }
    }
}

pub trait VariantIterator<'a>: Iterator<Item = MaybePathVariant<'a>> + 'a {}
impl<'a, I: Iterator<Item = MaybePathVariant<'a>> + 'a> VariantIterator<'a> for I {}

impl MaybePath {
    pub(super) fn from_maybe_path_like(
        maybe_path_like: &MaybePathLike,
        path_hyperlink_regexes: Arc<Vec<Regex>>,
    ) -> Self {
        let (line, word_range) = maybe_path_like.to_line_and_word_range();
        Self {
            line,
            word_range,
            path_hyperlink_regexes,
        }
    }

    #[cfg(test)]
    fn new(line: &str, word_range: Range<usize>, path_regexes: Arc<Vec<Regex>>) -> Self {
        Self {
            line: line.to_string(),
            word_range,
            path_hyperlink_regexes: path_regexes,
        }
    }

    const MAX_MAIN_THREAD_PREFIX_WORDS: usize = 2;
    const MAX_BACKGROUND_THREAD_PREFIX_WORDS: usize = usize::MAX;

    /// Simple maybe path variants. These need to be kept to a small well-defined set of variants.
    ///
    /// On the main thread, these will be checked against worktrees only, using
    /// [PathHyperlinkNavigation::Default].
    ///
    /// *Local Only*--If no worktree match is found they will also be checked
    /// for existence in the workspace's real file system on the background thread,
    /// using [PathHyperlinkNavigation::Advanced]
    pub fn simple_variants(
        &self,
        path_hyperlink_navigation: PathHyperlinkNavigation,
    ) -> impl VariantIterator<'_> {
        [MaybePathVariant::new(
            &self.line,
            self.word_range.clone(),
            // TODO(davewa): We (currently) don't include self.path_hyperlink_regexes
            // here because we don't want to let user settings tank perforamce on the
            // main thread. But, the experience will be worse, (no hyperlink on remote
            // workspaces, delayed hyperlink on local workspaces. Also it is a pathological
            // case--in practice it would be unlikely that a user would add so many regexes
            // that it adversly affects performance. We perhaps could add a separate
            // `terminal.path_hyperlink_main_thread_timout` that defaults to a much smaller
            // number than `terminal.path_hyperlink_timout`?
            &preapproved_path_hyperlink_regexes().iter().collect_vec(),
            None,
        )]
        .into_iter()
        .chain(
            iter::once_with(move || {
                self.longest_surrounding_symbols_variant(path_hyperlink_navigation)
            })
            .flatten(),
        )
        .chain(
            iter::once_with(move || {
                // One prefix stripped is the most likely path, start there
                itertools::rev(
                    self.line_ends_in_a_path_maybe_path_variants(
                        path_hyperlink_navigation,
                        0,
                        Self::MAX_MAIN_THREAD_PREFIX_WORDS,
                    )
                    .collect_vec(),
                )
            })
            .flatten(),
        )
    }

    /// All [PathHyperlinkNavigation::Advanced] maybe path variants.
    pub fn advanced_variants(&self) -> impl VariantIterator<'_> {
        // TODO(davewa): Some way to assert we are not called on the main thread...
        self.line_ends_in_a_path_maybe_path_variants(
            PathHyperlinkNavigation::Advanced,
            Self::MAX_MAIN_THREAD_PREFIX_WORDS,
            Self::MAX_BACKGROUND_THREAD_PREFIX_WORDS,
        )
    }

    /// All [PathHyperlinkNavigation::Exhaustive] maybe path variants that start on the hovered word or a
    /// word before it and end the hovered word or a word after it.
    pub fn exhaustive_variants(&self) -> impl VariantIterator<'_> {
        // TODO(davewa): Some way to assert we are not called on the main thread...
        let starts = word_regex()
            .find_iter(&self.line[..self.word_range.end])
            .map(|match_| match_.ok())
            .flatten()
            .map(|match_| match_.start());

        starts.flat_map(move |start| {
            itertools::rev(
                word_regex()
                    .find_iter(&self.line[self.word_range.start..])
                    .collect_vec(),
            )
            .map(|match_| match_.ok())
            .flatten()
            .map(move |match_| {
                let range = start..self.word_range.start + match_.end();
                MaybePathVariant::new(
                    &self.line,
                    range.clone(),
                    &self.path_hyperlink_regexes(PathHyperlinkNavigation::Exhaustive),
                    None,
                )
            })
        })
    }

    fn path_hyperlink_regexes(
        &self,
        path_hyperlink_navigation: PathHyperlinkNavigation,
    ) -> Vec<&'_ Regex> {
        if path_hyperlink_navigation > PathHyperlinkNavigation::Default {
            self.path_hyperlink_regexes
                .iter()
                .chain(preapproved_path_hyperlink_regexes().iter())
                .collect_vec()
        } else {
            preapproved_path_hyperlink_regexes().iter().collect_vec()
        }
    }

    /// [PathHyperlinkNavigation::Default] variant for the longest surrounding symbols match, if any
    fn longest_surrounding_symbols_variant(
        &self,
        path_hyperlink_navigation: PathHyperlinkNavigation,
    ) -> Option<MaybePathVariant<'_>> {
        if let Some(surrounding_range) =
            longest_surrounding_symbols_match(&self.line, &self.word_range)
        {
            if surrounding_range != self.word_range {
                return Some(MaybePathVariant::new(
                    &self.line,
                    surrounding_range.start + 1..surrounding_range.end - 1,
                    &self.path_hyperlink_regexes(path_hyperlink_navigation),
                    Some(surrounding_range.start..self.line.len()),
                ));
            }
        }

        None
    }

    /// [PathHyperlinkNavigation::Advanced] maybe path variants that start on the hovered word or a
    /// word before it and end at the end of the line.
    fn line_ends_in_a_path_maybe_path_variants(
        &self,
        path_hyperlink_navigation: PathHyperlinkNavigation,
        start_prefix_words: usize,
        max_prefix_words: usize,
    ) -> impl VariantIterator<'_> {
        word_regex()
            .find_iter(&self.line[..self.word_range.end])
            .skip(start_prefix_words)
            .take(max_prefix_words)
            .map(|match_| match_.ok())
            .flatten()
            .map(move |match_| {
                MaybePathVariant::new(
                    &self.line,
                    match_.range().start..self.line.len(),
                    &self.path_hyperlink_regexes(path_hyperlink_navigation),
                    None,
                )
            })
    }
}

/// Like [PathWithPosition], with enhancements for [MaybePath] processing
///
/// Specifically, we:
/// - Don't require allocation
/// - Model row and column restrictions directly (cannot have a column without a row)
/// - Include our range within our source [MaybePath], and the length of the line and column suffix
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MaybePathWithPosition<'a> {
    pub path: Cow<'a, Path>,
    pub position: Option<RowColumn>,
    pub range: Range<usize>,
}

impl<'a> MaybePathWithPosition<'a> {
    fn new(range: &Range<usize>, path: Cow<'a, Path>, position: Option<RowColumn>) -> Self {
        Self {
            path,
            position,
            range: range.clone(),
        }
    }

    pub fn into_owned_with_path(self, path: Cow<'static, Path>) -> MaybePathWithPosition<'static> {
        MaybePathWithPosition { path, ..self }
    }

    pub fn into_owned(self) -> MaybePathWithPosition<'static> {
        MaybePathWithPosition {
            path: Cow::Owned(self.path.into_owned()),
            ..self
        }
    }
}

/// A contiguous sequence of words which includes the hovered word of a [MaybePath]
///
/// - Variations are ordered from most common to least common
/// - Surrounding symbols (if any) are stripped after processing the other variations
/// - Git diff prefixes are only processed if surrounding symbols are not present
/// - Row and column are never processed on a git diff variation
///
/// # Examples
///
/// | **original**         | **stripped**                              | **git diff**     | **row column**                            |
/// |----------------------|-------------------------------------------|------------------|-------------------------------------------|
/// | [a/some/path.rs]:4:2 | a/some/path.rs<br>*row = 4, column = 2*   |                  | [a/some/path.rs]<br>*row = 4, column = 2* |
/// | [a/some/path.rs:4:2] | a/some/path.rs:4:2                        |                  |                                           |
/// | a/some/path.rs:4:2   |                                           | some/path.rs:4:2 | a/some/path.rs<br>*row = 4, column = 2*   |
///
// Note: The above table renders perfectly in docs, but currenlty does not render correctly in the tooltip in Zed.
#[derive(Debug)]
pub struct MaybePathVariant<'a> {
    line: &'a str,
    variations: Vec<(Range<usize>, Option<RowColumn>)>,
    /// `a/~/foo.rs` is a valid path on it's own. If we parsed a git diff path like `+++ a/~/foo.rs` into a
    /// `~/foo.rs` variation, never absolutize it.
    #[cfg_attr(target_os = "windows", allow(dead_code))]
    absolutize_home_dir: bool,
}

impl<'a> MaybePathVariant<'a> {
    pub fn new(
        line: &'a str,
        mut path: Range<usize>,
        path_regexes: &Vec<&'a Regex>,
        stripped_common_symbols_regex_range: Option<Range<usize>>,
    ) -> Self {
        // We add variations from most common to least common
        let mut maybe_path = &line[path.clone()];
        let mut absolutize_home_dir = true;

        // Start with full range
        let mut variations = vec![(path.clone(), None)];

        // For all of these, path must be at least 2 characters
        if maybe_path.len() > 2 {
            // Git diff parsing--only if we did not strip common symbols
            if (maybe_path.starts_with('a') || maybe_path.starts_with('b'))
                && maybe_path[1..].starts_with(MAIN_SEPARATORS)
            {
                absolutize_home_dir = false;
                variations.push((path.start + 2..path.end, None));
                // Note: we do not update maybe_path here because row and column
                // should be processed with the git diff prefixes included, e.g.
                // `a/some/path:4:2` is never interpreted as `some/path`, row = 4, column = 2
                // because git diff never adds a position suffix
            }

            if let Some((range, position)) =
                path_with_position_regex_match(&maybe_path, path_regexes)
            {
                path = path.start + range.start..path.start + range.end;
                maybe_path = &line[path.clone()];
                if has_common_surrounding_symbols(&maybe_path) {
                    variations.insert(0, (path.start + 1..path.end - 1, Some(position)));
                };
                variations.insert(0, (path, Some(position)));
            } else if stripped_common_symbols_regex_range.is_none() {
                if has_common_surrounding_symbols(&maybe_path) {
                    variations.insert(0, (path.start + 1..path.end - 1, None));
                }
            }

            if let Some(stripped_common_symbols_regex_range) = stripped_common_symbols_regex_range {
                // In this case, surrounding symbols were stripped already by the caller.
                if let Some((range, position)) = path_with_position_regex_match(
                    &line[stripped_common_symbols_regex_range.clone()],
                    path_regexes,
                ) {
                    variations.insert(
                        0,
                        (
                            stripped_common_symbols_regex_range.start + range.start
                                ..stripped_common_symbols_regex_range.start + range.end,
                            Some(position),
                        ),
                    );
                }
            }
        }

        Self {
            line,
            variations,
            absolutize_home_dir,
        }
    }

    /// Returns all relative substring variations of the contained path:
    /// - With and without stripped common surrounding symbols: `"` `'` `(` `)` `[` `]`
    /// - With and without line and column suffix: `:4:2` or `(4,2)`
    /// - With and without git diff prefixes: `a/` or `b/`
    ///
    /// If `prefix_to_strip` is provided, each variation will additionally be stripped of that
    /// prefix (if it is present).
    pub fn relative_variations(
        &self,
        prefix_to_strip: Option<&Path>,
    ) -> Vec<MaybePathWithPosition<'a>> {
        self.variations
            .iter()
            .filter_map(|(range, position)| {
                let maybe_path = Path::new(&self.line[range.clone()]);
                if maybe_path.is_relative() {
                    Some(MaybePathWithPosition::new(
                        range,
                        Cow::Borrowed(prefix_to_strip.map_or(maybe_path, |prefix_to_strip| {
                            maybe_path
                                .strip_prefix(prefix_to_strip)
                                .unwrap_or(maybe_path)
                        })),
                        position.clone(),
                    ))
                } else {
                    None
                }
            })
            .collect_vec()
    }

    fn absolutize<P: Deref<Target = Path>>(
        &self,
        roots: &Vec<P>,
        home_dir: &PathBuf,
        range: &Range<usize>,
        position: Option<RowColumn>,
    ) -> Vec<MaybePathWithPosition<'a>> {
        let mut absolutized = Vec::new();

        let path = Path::new(&self.line[range.clone()]);
        if path.is_absolute() {
            absolutized.push(MaybePathWithPosition::new(
                range,
                Cow::Borrowed(path),
                position,
            ));
            return absolutized;
        }

        for root in roots {
            absolutized.push(MaybePathWithPosition::new(
                range,
                Cow::Owned(root.join(path)),
                position,
            ));
        }

        self.absolutize_home_dir(path, home_dir, range, position, &mut absolutized);

        absolutized
    }

    #[cfg(target_os = "windows")]
    fn absolutize_home_dir(
        &self,
        _path: &Path,
        _home_dir: &PathBuf,
        _range: &Range<usize>,
        _position: Option<RowColumn>,
        _absolutized: &mut Vec<MaybePathWithPosition<'a>>,
    ) -> () {
    }

    /// Yields all absolutized variations of all relative and absolute variations
    #[cfg(not(target_os = "windows"))]
    fn absolutize_home_dir(
        &self,
        path: &Path,
        home_dir: &PathBuf,
        range: &Range<usize>,
        position: Option<RowColumn>,
        absolutized: &mut Vec<MaybePathWithPosition<'a>>,
    ) -> () {
        if self.absolutize_home_dir {
            if let Ok(tildeless_path) = path.strip_prefix("~") {
                absolutized.push(MaybePathWithPosition::new(
                    range,
                    Cow::Owned(home_dir.join(tildeless_path)),
                    position,
                ));
            }
        }
    }

    pub fn absolutized_variations<P: Deref<Target = Path>>(
        &self,
        roots: &Vec<P>,
        home_dir: &PathBuf,
    ) -> Vec<MaybePathWithPosition<'a>> {
        self.variations
            .iter()
            .map(|(range, position)| self.absolutize(roots, home_dir, range, position.clone()))
            .flatten()
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collections::HashMap;
    use fs::{FakeFs, Fs};
    use gpui::TestAppContext;
    use itertools::Itertools;
    use serde_json::json;
    use std::{path::Path, sync::Arc};
    use terminal::terminal_settings::PathHyperlinkNavigation;
    use util::{path, separator};

    struct ExpectedMaybePathVariations<'a> {
        relative: Vec<MaybePathWithPosition<'a>>,
        absolutized: Vec<MaybePathWithPosition<'a>>,
        open_target: Option<MaybePathWithPosition<'static>>,
    }

    #[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
    enum Thread {
        Main,
        Background,
    }

    type ExpectedMap<'a> =
        HashMap<(PathHyperlinkNavigation, Thread), Vec<ExpectedMaybePathVariations<'a>>>;

    async fn test_maybe_paths<'a>(
        fs: Arc<FakeFs>,
        custom_path_regexes: Arc<Vec<Regex>>,
        worktree_root: &Path,
        line: &str,
        word_index: Option<usize>,
        expected: &ExpectedMap<'a>,
    ) {
        let maybe_paths = if let Some(word_index) = word_index {
            vec![word_regex()
                .find_iter(&line)
                .map(Result::<_, _>::ok)
                .flatten()
                .map(|match_| match_.range())
                .nth(word_index)
                .unwrap()]
        } else {
            word_regex()
                .find_iter(&line)
                .map(Result::<_, _>::ok)
                .flatten()
                .map(|match_| match_.range())
                .collect_vec()
        };

        let test_maybe_path = async |path_hyperlink_navigation, thread| {
            if !expected.contains_key(&(path_hyperlink_navigation, thread)) {
                return;
            }

            let word_expected = expected.get(&(path_hyperlink_navigation, thread)).unwrap();
            for (matched, expected) in maybe_paths.iter().zip(word_expected) {
                let maybe_path =
                    MaybePath::new(line, matched.clone(), Arc::clone(&custom_path_regexes));
                println!("\n\nTesting {path_hyperlink_navigation:?}: {maybe_path}");

                let variants = match (path_hyperlink_navigation, thread) {
                    (PathHyperlinkNavigation::Default, Thread::Main) => maybe_path
                        .simple_variants(PathHyperlinkNavigation::Default)
                        .collect_vec(),
                    (PathHyperlinkNavigation::Default, Thread::Background) => maybe_path
                        .simple_variants(PathHyperlinkNavigation::Advanced)
                        .collect_vec(),
                    (PathHyperlinkNavigation::Advanced, _) => {
                        maybe_path.advanced_variants().collect_vec()
                    }
                    (PathHyperlinkNavigation::Exhaustive, _) => {
                        maybe_path.exhaustive_variants().collect_vec()
                    }
                    _ => {
                        assert!(false, "Unexpected {path_hyperlink_navigation:?}");
                        return;
                    }
                };

                test_variants(Arc::clone(&fs), worktree_root, variants, &expected).await
            }
        };

        test_maybe_path(PathHyperlinkNavigation::Default, Thread::Main).await;
        test_maybe_path(PathHyperlinkNavigation::Default, Thread::Background).await;
        test_maybe_path(PathHyperlinkNavigation::Advanced, Thread::Background).await;
        test_maybe_path(PathHyperlinkNavigation::Exhaustive, Thread::Background).await;
    }

    fn format_maybe_path_with_position(
        maybe_path_with_position: &MaybePathWithPosition,
        rel_or_abs: Option<&str>,
    ) -> String {
        let MaybePathWithPosition {
            ref path, position, ..
        } = maybe_path_with_position;

        let path = if rel_or_abs.is_some() {
            format!("{:?}", path.to_string_lossy(),)
        } else {
            format!("{}", path.to_string_lossy(),)
        };
        let position = format!(
            "{}",
            position
                .map(|position| format!(
                    ", {}, {}{}",
                    position.suffix_length,
                    position.row,
                    position
                        .column
                        .map(|column| format!(", {column}"))
                        .unwrap_or_default()
                ))
                .unwrap_or_default()
        );

        if let Some(rel_or_abs) = rel_or_abs {
            format!("   [ {rel_or_abs}!({path}) ]{position};")
        } else {
            format!("{path}{position}")
        }
    }

    fn check_variations<'a>(
        actual: &Vec<MaybePathWithPosition<'a>>,
        expected: &Vec<MaybePathWithPosition<'a>>,
        rel_or_abs: &str,
    ) {
        let errors = actual
            .iter()
            .zip(expected.iter())
            .filter(|(actual, expected)| {
                actual.path != expected.path || actual.position != expected.position
            })
            .inspect(|(actual, expected)| {
                println!(
                    "  left: \"{}\", position = {:?}",
                    actual.path.to_string_lossy(),
                    actual.position
                );
                println!(
                    " right: \"{}\", position = {:?}",
                    expected.path.to_string_lossy(),
                    expected.position
                );
            })
            .collect_vec();

        if actual.len() != expected.len() || !errors.is_empty() {
            println!("\nActual:");
            actual.iter().for_each(|actual| {
                println!(
                    "{}",
                    format_maybe_path_with_position(actual, Some(rel_or_abs))
                )
            });
            println!("\nExpected:");
            expected.iter().for_each(|expected| {
                println!(
                    "{}",
                    format_maybe_path_with_position(expected, Some(rel_or_abs))
                )
            });
            assert!(false);
        }
    }

    async fn test_variants<'a>(
        fs: Arc<FakeFs>,
        worktree_root: &Path,
        variants: Vec<MaybePathVariant<'_>>,
        expected: &ExpectedMaybePathVariations<'a>,
    ) {
        //assert_eq!(variants().len(), 3);

        println!("\n\nVariants:");
        for variant in &variants {
            println!("[");
            for (range, position) in variant.variations.iter().cloned() {
                println!(
                    "\t{}",
                    format_maybe_path_with_position(
                        &MaybePathWithPosition {
                            path: Cow::Borrowed(Path::new(&variant.line[range.clone()])),
                            position,
                            range
                        },
                        None
                    )
                );
            }
            println!("],");
        }

        println!("\nTesting Relative: strip_prefix = {worktree_root:?}");

        let actual_relative = variants
            .iter()
            .flat_map(|maybe_path_variant| {
                maybe_path_variant.relative_variations(Some(worktree_root))
            })
            .collect_vec();

        check_variations(&actual_relative, &expected.relative, "rel");

        const HOME_DIR: &str = path!("/Usors/uzer");
        const CWD: &str = path!("/Some/cool/place");

        let home_dir = Path::new(HOME_DIR).to_path_buf();
        let roots = Vec::from_iter([worktree_root, Path::new(CWD)]);

        println!("\nTesting Absolutized: home_dir: {home_dir:?}, roots: {roots:?}",);

        let actual_absolutized = variants
            .iter()
            .flat_map(|maybe_path_variant| {
                maybe_path_variant.absolutized_variations(&roots, &home_dir)
            })
            .collect_vec();

        check_variations(&actual_absolutized, &expected.absolutized, "abs");

        let actual_open_target = async || {
            for maybe_path_with_position in &actual_absolutized {
                if let Ok(Some(_metadata)) = fs.metadata(&maybe_path_with_position.path).await {
                    // TODO(davewa): assert_eq!(metadata.is_dir, expected_open_target.is_dir)
                    return Some(MaybePathWithPosition {
                        path: Cow::Owned(maybe_path_with_position.path.to_path_buf()),
                        ..maybe_path_with_position.clone()
                    });
                }
            }

            None
        };

        if let Some(actual_open_target) = actual_open_target().await {
            if let Some(expected_open_target) = expected.open_target.as_ref() {
                assert_eq!(
                    *expected_open_target.path, actual_open_target.path,
                    "Mismatched open target paths"
                );
                assert_eq!(
                    expected_open_target.position, actual_open_target.position,
                    "Mismatched open target positions"
                );
            } else {
                assert!(
                    false,
                    "Expected no open target, but found: {:?}",
                    actual_open_target
                );
            }
        } else if let Some(expected_open_target) = expected.open_target.as_ref() {
            assert!(
                false,
                "No open target found, expected: {:?}",
                expected_open_target
            );
        }
    }

    #[cfg(target_os = "windows")]
    macro_rules! maybe_home_path_with_positions {
        ($variations:ident, [ $($path:expr),+ ], $row:literal, $column:literal; $($tail:tt)*) => {
            maybe_path_with_positions!($variations, $($tail)*);
        };

        ($variations:ident, [ $($path:expr),+ ], $row:literal; $($tail:tt)*) => {
            maybe_path_with_positions!($variations, $($tail)*);
        };

        ($variations:ident, [ $($path:expr),+ ]; $($tail:tt)*) => {
            maybe_path_with_positions!($variations, $($tail)*);
        };
    }

    #[cfg(not(target_os = "windows"))]
    macro_rules! maybe_home_path_with_positions {
        ($variations:ident, [ $($path:expr),+ ] $($tail:tt)*) => {
            maybe_path_with_positions!($variations, [ $($path),+ ] $($tail)*);
        };
    }

    macro_rules! maybe_path_with_positions {
        ($variations:ident, [ $($path:expr),+ ], $suffix_length:literal, $row:literal, $column:literal; $($tail:tt)*) => {
            $variations.push(MaybePathWithPosition::new(
                &(0..0),
                Cow::Borrowed(Path::new(concat!($($path),+))),
                Some(RowColumn{ row: $row, column: Some($column), suffix_length: $suffix_length })
            ));
            maybe_path_with_positions!($variations, $($tail)*);
        };

        ($variations:ident, [ $($path:expr),+ ], $suffix_length:literal, $row:literal; $($tail:tt)*) => {
            $variations.push(MaybePathWithPosition::new(
                &(0..0),
                Cow::Borrowed(Path::new(concat!($($path),+))),
                Some(RowColumn{ row: $row, column: None, suffix_length: $suffix_length })
            ));
            maybe_path_with_positions!($variations, $($tail)*);
        };

        ($variations:ident, [ $($path:expr),+ ]; $($tail:tt)*) => {
            $variations.push(MaybePathWithPosition::new(
                &(0..0),
                Cow::Borrowed(Path::new(concat!($($path),+))),
                None
            ));
            maybe_path_with_positions!($variations, $($tail)*);
        };

        ($variations:ident, [ @home $($path:expr),+ ] $($tail:tt)*) => {
            maybe_home_path_with_positions!($variations, [ $($path),+ ] $($tail)*);
        };

        ($variations:ident,) => {
        };

        ($($tail:tt)+) => { {
            let mut maybe_path_variations = Vec::new();
            maybe_path_with_positions!(maybe_path_variations, $($tail)+);
            maybe_path_variations
        } };

        () => { Vec::new() };
    }

    macro_rules! relative {
        ($($tail:tt)*) => { maybe_path_with_positions![ $($tail)* ] }
    }

    macro_rules! absolutized {
        ($($tail:tt)*) => { maybe_path_with_positions![ $($tail)* ] }
    }

    macro_rules! expected_open_target {
        ($path:literal, $suffix_length:literal, $row:literal, $column:literal) => {
            Some(MaybePathWithPosition::new(
                &(0..0),
                Cow::Borrowed(Path::new(path!($path))),
                Some(RowColumn {
                    row: $row,
                    column: Some($column),
                    suffix_length: $suffix_length,
                }),
            ))
        };
        ($path:literal, $suffix_length:literal, $row:literal) => {
            Some(MaybePathWithPosition::new(
                &(0..0),
                Cow::Borrowed(Path::new(path!($path))),
                Some(RowColumn {
                    row: $row,
                    column: None,
                    suffix_length: $suffix_length,
                }),
            ))
        };
        ($path:literal) => {
            Some(MaybePathWithPosition::new(
                &(0..0),
                Cow::Borrowed(Path::new(path!($path))),
                None,
            ))
        };
    }

    macro_rules! expected {
        ($relative:expr, $absolutized:expr) => {
            ExpectedMaybePathVariations {
                relative: $relative,
                absolutized: $absolutized,
                open_target: None,
            }
        };

        ($relative:expr, $absolutized:expr, $open_target:expr) => {
            ExpectedMaybePathVariations {
                relative: $relative,
                absolutized: $absolutized,
                open_target: $open_target,
            }
        };
    }

    macro_rules! abs {
        ($path:literal) => {
            path!($path)
        };
    }

    macro_rules! rel {
        ($path:literal) => {
            separator!($path)
        };
    }

    // <https://github.com/zed-industries/zed/issues/16004>
    #[gpui::test]
    async fn issue_16004(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/w"),
            json!({
                "src": {
                    "3rdparty": {
                        "zed": {
                            "bad_py.py": "",
                            "bad py.py": ""
                        },
                    },
                }
            }),
        )
        .await;

        let mut expected = ExpectedMap::from_iter([]);

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Main),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                    ],
                    absolutized![
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                    ]
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Background),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                    ],
                    absolutized![
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ abs!("/w/src/3rdparty/zed/bad_py.py") ], 8, 8;
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/w/src/3rdparty/zed/bad_py.py") ], 8, 8;
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                    ],
                    expected_open_target!("/w/src/3rdparty/zed/bad_py.py", 8, 8)
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Advanced, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                    ],
                    absolutized![
                    ]
                }]
                .into_iter(),
            ),
        );

        expected.insert(
            (PathHyperlinkNavigation::Exhaustive, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!("File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad_py.py\", line 8,") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad_py.py\", line") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\", line 8, in") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\", line 8,") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\", line") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad_py.py\",") ];
                    ],
                    absolutized![
                        [ abs!("/w/src/3rdparty/zed/bad_py.py") ], 8, 8;
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/w/src/3rdparty/zed/bad_py.py") ], 8, 8;
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in") ];
                        [ abs!("/w/src/3rdparty/zed/bad_py.py") ], 8, 8;
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad_py.py\", line 8,") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad_py.py\", line 8,") ];
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad_py.py\", line") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad_py.py\", line") ];
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ abs!("/w/src/3rdparty/zed/bad_py.py") ], 8, 8;
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>") ];
                        [ abs!("/w/src/3rdparty/zed/bad_py.py") ], 8, 8;
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\", line 8, in") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\", line 8, in") ];
                        [ abs!("/w/src/3rdparty/zed/bad_py.py") ], 8, 8;
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\", line 8,") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\", line 8,") ];
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\", line") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\", line") ];
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad_py.py\",") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad_py.py\",") ];
                    ],
                    expected_open_target!("/w/src/3rdparty/zed/bad_py.py", 8, 8)
                }]
                .into_iter(),
            ),
        );

        const PATH_LINE_COLUMN_REGEX_PYTHON: &str =
            "\"(?<path>[^,]+)\"((?<suffix>(, line (?<line>[0-9]+))))?";

        let path_regexes = Arc::new(Vec::from_iter([
            Regex::new(PATH_LINE_COLUMN_REGEX_PYTHON).unwrap()
        ]));

        test_maybe_paths(
            Arc::clone(&fs),
            Arc::clone(&path_regexes),
            &Path::new(abs!("/w")),
            "  File \"/w/src/3rdparty/zed/bad_py.py\", line 8, in <module>",
            Some(1),
            &expected,
        )
        .await;

        let mut expected = ExpectedMap::from_iter([]);

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Main),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("\"/w/src/3rdparty/zed/bad") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                    ],
                    absolutized![
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ];
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                    ],
                    expected_open_target!("/w/src/3rdparty/zed/bad py.py")
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Background),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("\"/w/src/3rdparty/zed/bad") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                    ],
                    absolutized![
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                    ],
                    expected_open_target!("/w/src/3rdparty/zed/bad py.py", 8, 8)
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Advanced, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                    ],
                    absolutized![
                    ]
                }]
                .into_iter(),
            ),
        );

        expected.insert(
            (PathHyperlinkNavigation::Exhaustive, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!("File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad py.py\", line 8, in") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad py.py\", line 8,") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad py.py\", line") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad py.py\",") ];
                        [ rel!("File \"/w/src/3rdparty/zed/bad") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad py.py\", line 8, in") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad py.py\", line 8,") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad py.py\", line") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad py.py\",") ];
                        [ rel!("\"/w/src/3rdparty/zed/bad") ];
                    ],
                    absolutized![
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad py.py\", line 8, in") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad py.py\", line 8, in") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad py.py\", line 8,") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad py.py\", line 8,") ];
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad py.py\", line") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad py.py\", line") ];
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad py.py\",") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad py.py\",") ];
                        [ abs!("/w/File \"/w/src/3rdparty/zed/bad") ];
                        [ abs!("/Some/cool/place/File \"/w/src/3rdparty/zed/bad") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad py.py\", line 8, in") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad py.py\", line 8, in") ];
                        [ abs!("/w/src/3rdparty/zed/bad py.py") ], 8, 8;
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad py.py\", line 8,") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad py.py\", line 8,") ];
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad py.py\", line") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad py.py\", line") ];
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad py.py\",") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad py.py\",") ];
                        [ abs!("/w/\"/w/src/3rdparty/zed/bad") ];
                        [ abs!("/Some/cool/place/\"/w/src/3rdparty/zed/bad") ];
                    ],
                    expected_open_target!("/w/src/3rdparty/zed/bad py.py", 8, 8)
                }]
                .into_iter(),
            ),
        );

        test_maybe_paths(
            Arc::clone(&fs),
            Arc::clone(&path_regexes),
            &Path::new(abs!("/w")),
            "  File \"/w/src/3rdparty/zed/bad py.py\", line 8, in <module>",
            Some(1),
            &expected,
        )
        .await;
    }

    #[gpui::test]
    async fn issue_25086(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "app": {
                    "services": {
                        "opensearch": {
                            "contacts" :{
                                "create_service.rb": ""
                            }
                        },
                        "open search": {
                            "contacts" :{
                                "create service.rb": ""
                            }
                        }
                    },
                }
            }),
        )
        .await;

        let mut expected = ExpectedMap::from_iter([]);

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Main),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ rel!("./app/services/opensearch/contacts/create_service.rb:9:in") ];
                        [ rel!("./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ rel!("./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ rel!("# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ rel!("# ./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                    ],
                    absolutized![
                        [ abs!("/root/./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/root/./app/services/opensearch/contacts/create_service.rb:9:in") ];
                        [ abs!("/Some/cool/place/./app/services/opensearch/contacts/create_service.rb:9:in") ];
                        [ abs!("/root/./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/root/./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/root/# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/root/# ./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/# ./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                    ],
                    expected_open_target!("/root/./app/services/opensearch/contacts/create_service.rb", 2, 9)
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Advanced, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                    ],
                    absolutized![
                    ]
                }]
                .into_iter(),
            ),
        );

        expected.insert(
            (PathHyperlinkNavigation::Exhaustive, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!("# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ rel!("# ./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ rel!("# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ rel!("# ./app/services/opensearch/contacts/create_service.rb:9:in") ];
                        [ rel!("./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ rel!("./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ rel!("./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ rel!("./app/services/opensearch/contacts/create_service.rb:9:in") ];
                    ],
                    absolutized![
                        [ abs!("/root/# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/root/# ./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/# ./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/root/# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/# ./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/root/# ./app/services/opensearch/contacts/create_service.rb:9:in") ];
                        [ abs!("/Some/cool/place/# ./app/services/opensearch/contacts/create_service.rb:9:in") ];
                        [ abs!("/root/./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/root/./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/root/./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/./app/services/opensearch/contacts/create_service.rb") ], 2, 9;
                        [ abs!("/root/./app/services/opensearch/contacts/create_service.rb:9:in") ];
                        [ abs!("/Some/cool/place/./app/services/opensearch/contacts/create_service.rb:9:in") ];
                    ],
                    expected_open_target!("/root/./app/services/opensearch/contacts/create_service.rb", 2, 9)
                }]
                .into_iter(),
            ),
        );

        test_maybe_paths(
            Arc::clone(&fs),
            Arc::new(Vec::new()),
            &Path::new(abs!("/root")),
            "# ./app/services/opensearch/contacts/create_service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'",
            Some(1),
            &expected,
        )
        .await;

        let mut expected = ExpectedMap::from_iter([]);

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Main),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("search/contacts/create") ];
                        [ rel!("./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ rel!("./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ rel!("# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ rel!("# ./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                    ],
                    absolutized![
                        [ abs!("/root/search/contacts/create") ];
                        [ abs!("/Some/cool/place/search/contacts/create") ];
                        [ abs!("/root/./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/root/# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/# ./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/# ./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                    ],
                    expected_open_target!("/root/./app/services/open search/contacts/create service.rb", 2, 9)

                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Advanced, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!("search/contacts/create service.rb") ], 2, 9;
                        [ rel!("search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                    ],
                    absolutized![
                        [ abs!("/root/search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                    ]
                }]
                .into_iter(),
            ),
        );

        expected.insert(
            (PathHyperlinkNavigation::Exhaustive, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!("# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ rel!("# ./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ rel!("# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ rel!("# ./app/services/open search/contacts/create service.rb:9:in") ];
                        [ rel!("# ./app/services/open search/contacts/create") ];
                        [ rel!("./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ rel!("./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ rel!("./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ rel!("./app/services/open search/contacts/create service.rb:9:in") ];
                        [ rel!("./app/services/open search/contacts/create") ];
                        [ rel!("search/contacts/create service.rb") ], 2, 9;
                        [ rel!("search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ rel!("search/contacts/create service.rb") ], 2, 9;
                        [ rel!("search/contacts/create service.rb:9:in") ];
                        [ rel!("search/contacts/create") ];
                    ],
                    absolutized![
                        [ abs!("/root/# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/# ./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/# ./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/root/# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/# ./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/# ./app/services/open search/contacts/create service.rb:9:in") ];
                        [ abs!("/Some/cool/place/# ./app/services/open search/contacts/create service.rb:9:in") ];
                        [ abs!("/root/# ./app/services/open search/contacts/create") ];
                        [ abs!("/Some/cool/place/# ./app/services/open search/contacts/create") ];
                        [ abs!("/root/./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/root/./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/./app/services/open search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/./app/services/open search/contacts/create service.rb:9:in") ];
                        [ abs!("/Some/cool/place/./app/services/open search/contacts/create service.rb:9:in") ];
                        [ abs!("/root/./app/services/open search/contacts/create") ];
                        [ abs!("/Some/cool/place/./app/services/open search/contacts/create") ];
                        [ abs!("/root/search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/Some/cool/place/search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'") ];
                        [ abs!("/root/search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/Some/cool/place/search/contacts/create service.rb") ], 2, 9;
                        [ abs!("/root/search/contacts/create service.rb:9:in") ];
                        [ abs!("/Some/cool/place/search/contacts/create service.rb:9:in") ];
                        [ abs!("/root/search/contacts/create") ];
                        [ abs!("/Some/cool/place/search/contacts/create") ];
                    ],
                    expected_open_target!("/root/./app/services/open search/contacts/create service.rb", 2, 9)

                }]
                .into_iter(),
            ),
        );

        test_maybe_paths(
            Arc::clone(&fs),
            Arc::new(Vec::new()),
            &Path::new(abs!("/root")),
            "# ./app/services/open search/contacts/create service.rb:9:in 'Opensearch::Contacts::CreateService#validate_field_keys'",
            Some(2),
            &expected,
        )
        .await;
    }

    // <https://github.com/zed-industries/zed/issues/12338>
    #[gpui::test]
    async fn issue_12338(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "'test file 1.txt'": "",
                "test、2.txt": "",
                "test。3.txt": "",
            }),
        )
        .await;

        let mut expected = ExpectedMap::from_iter([]);

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Main),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("'test") ];
                        [ rel!("test file 1.txt") ];
                        [ rel!("0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ rel!(".rw-r--r--     0     staff 05-27 13:56 'test file 1.txt'") ];
                    ],
                    absolutized![
                        [ abs!("/root/'test") ];
                        [ abs!("/Some/cool/place/'test") ];
                        [ abs!("/root/test file 1.txt") ];
                        [ abs!("/Some/cool/place/test file 1.txt") ];
                        [ abs!("/root/0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/root/.rw-r--r--     0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/.rw-r--r--     0     staff 05-27 13:56 'test file 1.txt'") ];
                    ]
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Advanced, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!("staff 05-27 13:56 'test file 1.txt'") ];
                        [ rel!("05-27 13:56 'test file 1.txt'") ];
                        [ rel!("13:56 'test file 1.txt'") ];
                        [ rel!("test file 1.txt") ];
                        [ rel!("'test file 1.txt'") ];
                    ],
                    absolutized![
                        [ abs!("/root/staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/root/05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/root/13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/13:56 'test file 1.txt'") ];
                        [ abs!("/root/test file 1.txt") ];
                        [ abs!("/Some/cool/place/test file 1.txt") ];
                        [ abs!("/root/'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/'test file 1.txt'") ];
                    ],
                    expected_open_target!("/root/'test file 1.txt'")
                }]
                .into_iter(),
            ),
        );

        expected.insert(
            (PathHyperlinkNavigation::Exhaustive, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!(".rw-r--r--     0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ rel!(".rw-r--r--     0     staff 05-27 13:56 'test file") ];
                        [ rel!(".rw-r--r--     0     staff 05-27 13:56 'test") ];
                        [ rel!("0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ rel!("0     staff 05-27 13:56 'test file") ];
                        [ rel!("0     staff 05-27 13:56 'test") ];
                        [ rel!("staff 05-27 13:56 'test file 1.txt'") ];
                        [ rel!("staff 05-27 13:56 'test file") ];
                        [ rel!("staff 05-27 13:56 'test") ];
                        [ rel!("05-27 13:56 'test file 1.txt'") ];
                        [ rel!("05-27 13:56 'test file") ];
                        [ rel!("05-27 13:56 'test") ];
                        [ rel!("13:56 'test file 1.txt'") ];
                        [ rel!("13:56 'test file") ];
                        [ rel!("13:56 'test") ];
                        [ rel!("test file 1.txt") ];
                        [ rel!("'test file 1.txt'") ];
                        [ rel!("'test file") ];
                        [ rel!("'test") ];
                    ],
                    absolutized![
                        [ abs!("/root/.rw-r--r--     0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/.rw-r--r--     0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/root/.rw-r--r--     0     staff 05-27 13:56 'test file") ];
                        [ abs!("/Some/cool/place/.rw-r--r--     0     staff 05-27 13:56 'test file") ];
                        [ abs!("/root/.rw-r--r--     0     staff 05-27 13:56 'test") ];
                        [ abs!("/Some/cool/place/.rw-r--r--     0     staff 05-27 13:56 'test") ];
                        [ abs!("/root/0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/0     staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/root/0     staff 05-27 13:56 'test file") ];
                        [ abs!("/Some/cool/place/0     staff 05-27 13:56 'test file") ];
                        [ abs!("/root/0     staff 05-27 13:56 'test") ];
                        [ abs!("/Some/cool/place/0     staff 05-27 13:56 'test") ];
                        [ abs!("/root/staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/staff 05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/root/staff 05-27 13:56 'test file") ];
                        [ abs!("/Some/cool/place/staff 05-27 13:56 'test file") ];
                        [ abs!("/root/staff 05-27 13:56 'test") ];
                        [ abs!("/Some/cool/place/staff 05-27 13:56 'test") ];
                        [ abs!("/root/05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/05-27 13:56 'test file 1.txt'") ];
                        [ abs!("/root/05-27 13:56 'test file") ];
                        [ abs!("/Some/cool/place/05-27 13:56 'test file") ];
                        [ abs!("/root/05-27 13:56 'test") ];
                        [ abs!("/Some/cool/place/05-27 13:56 'test") ];
                        [ abs!("/root/13:56 'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/13:56 'test file 1.txt'") ];
                        [ abs!("/root/13:56 'test file") ];
                        [ abs!("/Some/cool/place/13:56 'test file") ];
                        [ abs!("/root/13:56 'test") ];
                        [ abs!("/Some/cool/place/13:56 'test") ];
                        [ abs!("/root/test file 1.txt") ];
                        [ abs!("/Some/cool/place/test file 1.txt") ];
                        [ abs!("/root/'test file 1.txt'") ];
                        [ abs!("/Some/cool/place/'test file 1.txt'") ];
                        [ abs!("/root/'test file") ];
                        [ abs!("/Some/cool/place/'test file") ];
                        [ abs!("/root/'test") ];
                        [ abs!("/Some/cool/place/'test") ];
                    ],
                    expected_open_target!("/root/'test file 1.txt'")
                }]
                .into_iter(),
            ),
        );

        test_maybe_paths(
            Arc::clone(&fs),
            Arc::new(Vec::new()),
            &Path::new(abs!("/root")),
            ".rw-r--r--     0     staff 05-27 13:56 'test file 1.txt'",
            Some(5),
            &expected,
        )
        .await;

        let mut expected = ExpectedMap::from_iter([]);

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Main),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("test、2.txt") ];
                        [ rel!("0     staff 05-27 14:03 test、2.txt") ];
                        [ rel!(".rw-r--r--     0     staff 05-27 14:03 test、2.txt") ];
                    ],
                    absolutized![
                        [ abs!("/root/test、2.txt") ];
                        [ abs!("/Some/cool/place/test、2.txt") ];
                        [ abs!("/root/0     staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/0     staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/root/.rw-r--r--     0     staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/.rw-r--r--     0     staff 05-27 14:03 test、2.txt") ];
                    ],
                    expected_open_target!("/root/test、2.txt")
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Advanced, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!("staff 05-27 14:03 test、2.txt") ];
                        [ rel!("05-27 14:03 test、2.txt") ];
                        [ rel!("14:03 test、2.txt") ];
                        [ rel!("test、2.txt") ];
                    ],
                    absolutized![
                        [ abs!("/root/staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/root/05-27 14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/05-27 14:03 test、2.txt") ];
                        [ abs!("/root/14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/14:03 test、2.txt") ];
                        [ abs!("/root/test、2.txt") ];
                        [ abs!("/Some/cool/place/test、2.txt") ];
                    ],
                    expected_open_target!("/root/test、2.txt")
                }]
                .into_iter(),
            ),
        );

        expected.insert(
            (PathHyperlinkNavigation::Exhaustive, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!(".rw-r--r--     0     staff 05-27 14:03 test、2.txt") ];
                        [ rel!("0     staff 05-27 14:03 test、2.txt") ];
                        [ rel!("staff 05-27 14:03 test、2.txt") ];
                        [ rel!("05-27 14:03 test、2.txt") ];
                        [ rel!("14:03 test、2.txt") ];
                        [ rel!("test、2.txt") ];
                    ],
                    absolutized![
                        [ abs!("/root/.rw-r--r--     0     staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/.rw-r--r--     0     staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/root/0     staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/0     staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/root/staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/staff 05-27 14:03 test、2.txt") ];
                        [ abs!("/root/05-27 14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/05-27 14:03 test、2.txt") ];
                        [ abs!("/root/14:03 test、2.txt") ];
                        [ abs!("/Some/cool/place/14:03 test、2.txt") ];
                        [ abs!("/root/test、2.txt") ];
                        [ abs!("/Some/cool/place/test、2.txt") ];
                    ],
                    expected_open_target!("/root/test、2.txt")
                }]
                .into_iter(),
            ),
        );

        test_maybe_paths(
            Arc::clone(&fs),
            Arc::new(Vec::new()),
            &Path::new(abs!("/root")),
            ".rw-r--r--     0     staff 05-27 14:03 test、2.txt",
            // ".rw-r--r--     0     staff 05-27 14:03 test。3.txt"
            Some(5),
            &expected,
        )
        .await;

        let mut expected = ExpectedMap::from_iter([]);

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Main),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("test。3.txt") ];
                        [ rel!("0     staff 05-27 14:03 test。3.txt") ];
                        [ rel!(".rw-r--r--     0     staff 05-27 14:03 test。3.txt") ];
                    ],
                    absolutized![
                        [ abs!("/root/test。3.txt") ];
                        [ abs!("/Some/cool/place/test。3.txt") ];
                        [ abs!("/root/0     staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/0     staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/root/.rw-r--r--     0     staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/.rw-r--r--     0     staff 05-27 14:03 test。3.txt") ];
                    ],
                    expected_open_target!("/root/test。3.txt")
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Advanced, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!("staff 05-27 14:03 test。3.txt") ];
                        [ rel!("05-27 14:03 test。3.txt") ];
                        [ rel!("14:03 test。3.txt") ];
                        [ rel!("test。3.txt") ];
                    ],
                    absolutized![
                        [ abs!("/root/staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/root/05-27 14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/05-27 14:03 test。3.txt") ];
                        [ abs!("/root/14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/14:03 test。3.txt") ];
                        [ abs!("/root/test。3.txt") ];
                        [ abs!("/Some/cool/place/test。3.txt") ];
                    ],
                    expected_open_target!("/root/test。3.txt")
                }]
                .into_iter(),
            ),
        );

        expected.insert(
            (PathHyperlinkNavigation::Exhaustive, Thread::Background),
            Vec::from_iter(
                [expected! {
                    relative![
                        [ rel!(".rw-r--r--     0     staff 05-27 14:03 test。3.txt") ];
                        [ rel!("0     staff 05-27 14:03 test。3.txt") ];
                        [ rel!("staff 05-27 14:03 test。3.txt") ];
                        [ rel!("05-27 14:03 test。3.txt") ];
                        [ rel!("14:03 test。3.txt") ];
                        [ rel!("test。3.txt") ];
                    ],
                    absolutized![
                        [ abs!("/root/.rw-r--r--     0     staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/.rw-r--r--     0     staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/root/0     staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/0     staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/root/staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/staff 05-27 14:03 test。3.txt") ];
                        [ abs!("/root/05-27 14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/05-27 14:03 test。3.txt") ];
                        [ abs!("/root/14:03 test。3.txt") ];
                        [ abs!("/Some/cool/place/14:03 test。3.txt") ];
                        [ abs!("/root/test。3.txt") ];
                        [ abs!("/Some/cool/place/test。3.txt") ];
                    ],
                    expected_open_target!("/root/test。3.txt")
                }]
                .into_iter(),
            ),
        );

        test_maybe_paths(
            Arc::clone(&fs),
            Arc::new(Vec::new()),
            &Path::new(abs!("/root")),
            ".rw-r--r--     0     staff 05-27 14:03 test。3.txt",
            Some(5),
            &expected,
        )
        .await;
    }

    #[gpui::test]
    async fn simple_maybe_paths(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root1"),
            json!({
                "one.txt": "",
                "two.txt": "",
            }),
        )
        .await;
        fs.insert_tree(
            path!("/root 2"),
            json!({
                "שיתופית.rs": "",
            }),
        )
        .await;

        let mut expected = ExpectedMap::from_iter([]);

        expected.insert(
            (PathHyperlinkNavigation::Default, Thread::Main),
            Vec::from_iter([
                expected!{
                    relative![
                        [ rel!("+++") ];
                        [ rel!("+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ],
                    absolutized![
                        [ abs!("/root 2/+++") ];
                        [ abs!("/Some/cool/place/+++") ];
                        [ abs!("/root 2/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ]
                },
                expected!{
                    relative![
                        [ rel!("a/~/협동조합") ];
                        [ rel!("~/협동조합") ];
                        [ rel!("a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ],
                    absolutized![
                        [ abs!("/root 2/a/~/협동조합") ];
                        [ abs!("/Some/cool/place/a/~/협동조합") ];
                        [ abs!("/root 2/~/협동조합") ];
                        [ abs!("/Some/cool/place/~/협동조합") ];
                        [ abs!("/root 2/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ]
                },
                expected!{
                    relative![
                        [ rel!("~/super/cool") ];
                        [ rel!("a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ],
                    absolutized![
                        [ abs!("/root 2/~/super/cool") ];
                        [ abs!("/Some/cool/place/~/super/cool") ];
                        [ @home abs!("/Usors/uzer/super/cool") ];
                        [ abs!("/root 2/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ]
                },
                expected!{
                    relative![
                        [ rel!("b/path") ], 4, 4, 2;
                        [ rel!("b/path:4:2") ];
                        [ rel!("path:4:2") ];
                        [ rel!("a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        ],
                    absolutized![
                        [ abs!("/root 2/b/path") ], 4, 4, 2;
                        [ abs!("/Some/cool/place/b/path") ], 4, 4, 2;
                        [ abs!("/root 2/b/path:4:2") ];
                        [ abs!("/Some/cool/place/b/path:4:2") ];
                        [ abs!("/root 2/path:4:2") ];
                        [ abs!("/Some/cool/place/path:4:2") ];
                        [ abs!("/root 2/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ]
                },
                expected!{
                    relative![
                        [ "(", abs!("/root") ];
                        [ rel!("a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ],
                    absolutized![
                        [ abs!("/root 2/("), abs!("/root") ];
                        [ abs!("/Some/cool/place/("), abs!("/root") ];
                        [ abs!("/root 2/שיתופית.rs") ];
                        [ abs!("/root 2/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ],
                    expected_open_target!("/root 2/שיתופית.rs")
                },
                expected!{
                    relative![
                        [ rel!("2/שיתופית.rs)") ];
                        [ rel!("a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ rel!("+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ],
                    absolutized![
                        [ abs!("/root 2/2/שיתופית.rs)") ];
                        [ abs!("/Some/cool/place/2/שיתופית.rs)") ];
                        [ abs!("/root 2/שיתופית.rs") ];
                        [ abs!("/root 2/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/root 2/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        [ abs!("/Some/cool/place/+++ a/~/협동조합   ~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                    ],
                    expected_open_target!("/root 2/שיתופית.rs")
                }
            ].into_iter()),
        );

        expected.insert(
            (PathHyperlinkNavigation::Advanced, Thread::Background),
            Vec::from_iter(
                [
                    expected! {
                        relative![
                        ],
                        absolutized![
                        ]
                    },
                    expected! {
                        relative![
                        ],
                        absolutized![
                        ]
                    },
                    expected! {
                        relative![
                            [ rel!("~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        ],
                        absolutized![
                            [ abs!("/root 2/~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ @home abs!("/Usors/uzer/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        ]
                    },
                    expected! {
                        relative![
                            [ rel!("~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ rel!("b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ rel!("path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        ],
                        absolutized![
                            [ abs!("/root 2/~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ @home abs!("/Usors/uzer/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                        ]
                    },
                    expected! {
                        relative![
                            [ rel!("~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ rel!("b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ rel!("path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ "(", abs!("/root 2/שיתופית.rs)") ];
                        ],
                        absolutized![
                            [ abs!("/root 2/~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ @home abs!("/Usors/uzer/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/שיתופית.rs") ];
                            [ abs!("/root 2/("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/("), abs!("/root 2/שיתופית.rs"), ")" ];
                        ],
                        expected_open_target!("/root 2/שיתופית.rs")
                    },
                    expected! {
                        relative![
                            [ rel!("~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ rel!("b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ rel!("path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ "(", abs!("/root 2/שיתופית.rs)") ];
                            [ rel!("2/שיתופית.rs)") ];
                            ],
                        absolutized![
                            [ abs!("/root 2/~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/~/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ @home abs!("/Usors/uzer/super/cool b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/b/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/path:4:2 ("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/שיתופית.rs") ];
                            [ abs!("/root 2/("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/Some/cool/place/("), abs!("/root 2/שיתופית.rs"), ")" ];
                            [ abs!("/root 2/2/שיתופית.rs)") ];
                            [ abs!("/Some/cool/place/2/שיתופית.rs)") ];
                        ],
                        expected_open_target!("/root 2/שיתופית.rs")
                    },
                ]
                .into_iter(),
            ),
        );

        test_maybe_paths(
            fs,
            Arc::new(Vec::new()),
            &Path::new(abs!("/root 2")),
            concat!(
                rel!("+++ a/~/협동조합   ~/super/cool b/path:4:2 ("),
                abs!("/root 2/שיתופית.rs"),
                rel!(")")
            ),
            None,
            &expected,
        )
        .await
    }
}
