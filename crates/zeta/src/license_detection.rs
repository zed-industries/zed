use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
    ops::Range,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use anyhow::{Result, anyhow};
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, Entity, Subscription, Task};
use itertools::Itertools;
use postage::watch;
use project::Worktree;
use strum::VariantArray;
use util::{ResultExt as _, maybe, rel_path::RelPath};
use worktree::ChildEntriesOptions;

/// Matches the most common license locations, with US and UK English spelling.
static LICENSE_FILE_NAME_REGEX: LazyLock<regex::bytes::Regex> = LazyLock::new(|| {
    regex::bytes::RegexBuilder::new(
        "^ \
        (?: license | licence)? \
        (?: [\\-._]? \
            (?: apache (?: [\\-._] (?: 2.0 | 2 ))? | \
                0? bsd (?: [\\-._] [0123])? (?: [\\-._] clause)? | \
                isc | \
                mit | \
                upl | \
                zlib))? \
        (?: [\\-._]? (?: license | licence))? \
        (?: \\.txt | \\.md)? \
        $",
    )
    .ignore_whitespace(true)
    .case_insensitive(true)
    .build()
    .unwrap()
});

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, VariantArray)]
pub enum OpenSourceLicense {
    Apache2_0,
    BSDZero,
    BSD,
    ISC,
    MIT,
    UPL1_0,
    Zlib,
}

impl Display for OpenSourceLicense {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spdx_identifier())
    }
}

impl OpenSourceLicense {
    /// These are SPDX identifiers for the licenses, except for BSD, where the variants are not
    /// distinguished.
    pub fn spdx_identifier(&self) -> &'static str {
        match self {
            OpenSourceLicense::Apache2_0 => "apache-2.0",
            OpenSourceLicense::BSDZero => "0bsd",
            OpenSourceLicense::BSD => "bsd",
            OpenSourceLicense::ISC => "isc",
            OpenSourceLicense::MIT => "mit",
            OpenSourceLicense::UPL1_0 => "upl-1.0",
            OpenSourceLicense::Zlib => "zlib",
        }
    }

    pub fn patterns(&self) -> &'static [&'static str] {
        match self {
            OpenSourceLicense::Apache2_0 => &[
                include_str!("../license_patterns/apache-2.0-pattern"),
                include_str!("../license_patterns/apache-2.0-reference-pattern"),
            ],
            OpenSourceLicense::BSDZero => &[include_str!("../license_patterns/0bsd-pattern")],
            OpenSourceLicense::BSD => &[include_str!("../license_patterns/bsd-pattern")],
            OpenSourceLicense::ISC => &[include_str!("../license_patterns/isc-pattern")],
            OpenSourceLicense::MIT => &[include_str!("../license_patterns/mit-pattern")],
            OpenSourceLicense::UPL1_0 => &[include_str!("../license_patterns/upl-1.0-pattern")],
            OpenSourceLicense::Zlib => &[include_str!("../license_patterns/zlib-pattern")],
        }
    }
}

// TODO: Consider using databake or similar to not parse at runtime.
static LICENSE_PATTERNS: LazyLock<LicensePatterns> = LazyLock::new(|| {
    let mut approximate_max_length = 0;
    let mut patterns = Vec::new();
    for license in OpenSourceLicense::VARIANTS {
        for pattern in license.patterns() {
            let (pattern, length) = parse_pattern(pattern).unwrap();
            patterns.push((*license, pattern));
            approximate_max_length = approximate_max_length.max(length);
        }
    }
    LicensePatterns {
        patterns,
        approximate_max_length,
    }
});

fn detect_license(text: &str) -> Option<OpenSourceLicense> {
    let text = canonicalize_license_text(text);
    for (license, pattern) in LICENSE_PATTERNS.patterns.iter() {
        log::trace!("Checking if license is {}", license);
        if check_pattern(&pattern, &text) {
            return Some(*license);
        }
    }

    None
}

struct LicensePatterns {
    patterns: Vec<(OpenSourceLicense, Vec<PatternPart>)>,
    approximate_max_length: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct PatternPart {
    /// Indicates that matching `text` is optional. Skipping `match_any_chars` is conditional on
    /// matching `text`.
    optional: bool,
    /// Indicates the number of characters that can be skipped before matching `text`.
    match_any_chars: Range<usize>,
    /// The text to match, may be empty.
    text: String,
}

/// Lines that start with "-- " begin a `PatternPart`. `-- 1..10` specifies `match_any_chars:
/// 1..10`. `-- 1..10 optional:` additionally specifies `optional: true`. It's a parse error for a
/// line to start with `--` without matching this format.
///
/// Text that does not have `--` prefixes participate in the `text` field and are canonicalized by
/// lowercasing, replacing all runs of whitespace with a single space, and otherwise only keeping
/// ascii alphanumeric characters.
fn parse_pattern(pattern_source: &str) -> Result<(Vec<PatternPart>, usize)> {
    let mut pattern = Vec::new();
    let mut part = PatternPart::default();
    let mut approximate_max_length = 0;
    for line in pattern_source.lines() {
        if let Some(directive) = line.trim().strip_prefix("--") {
            if part != PatternPart::default() {
                pattern.push(part);
                part = PatternPart::default();
            }
            let valid = maybe!({
                let directive_chunks = directive.split_whitespace().collect::<Vec<_>>();
                if !(1..=2).contains(&directive_chunks.len()) {
                    return None;
                }
                if directive_chunks.len() == 2 {
                    part.optional = true;
                }
                let range_chunks = directive_chunks[0].split("..").collect::<Vec<_>>();
                if range_chunks.len() != 2 {
                    return None;
                }
                part.match_any_chars.start = range_chunks[0].parse::<usize>().ok()?;
                part.match_any_chars.end = range_chunks[1].parse::<usize>().ok()?;
                if part.match_any_chars.start > part.match_any_chars.end {
                    return None;
                }
                approximate_max_length += part.match_any_chars.end;
                Some(())
            });
            if valid.is_none() {
                return Err(anyhow!("Invalid pattern directive: {}", line));
            }
            continue;
        }
        approximate_max_length += line.len() + 1;
        let line = canonicalize_license_text(line);
        if line.is_empty() {
            continue;
        }
        if !part.text.is_empty() {
            part.text.push(' ');
        }
        part.text.push_str(&line);
    }
    if part != PatternPart::default() {
        pattern.push(part);
    }
    Ok((pattern, approximate_max_length))
}

/// Checks a pattern against text by iterating over the pattern parts in reverse order, and checking
/// matches with the end of a prefix of the input. Assumes that `canonicalize_license_text` has
/// already been applied to the input.
fn check_pattern(pattern: &[PatternPart], input: &str) -> bool {
    let mut input_ix = input.len();
    let mut match_any_chars = 0..0;
    for part in pattern.iter().rev() {
        if part.text.is_empty() {
            match_any_chars.start += part.match_any_chars.start;
            match_any_chars.end += part.match_any_chars.end;
            continue;
        }

        let search_range_end = n_chars_before_offset(match_any_chars.start, input_ix, input);
        let search_range_start = n_chars_before_offset(
            match_any_chars.len() + part.text.len(),
            search_range_end,
            input,
        );
        let found_ix = input[search_range_start..search_range_end].rfind(&part.text);

        if let Some(found_ix) = found_ix {
            input_ix = search_range_start + found_ix;
            match_any_chars = part.match_any_chars.clone();
        } else if !part.optional {
            log::trace!(
                "Failed to match pattern\n`...{}`\nagainst input\n`...{}`",
                &part.text[n_chars_before_offset(128, part.text.len(), &part.text)..],
                &input[n_chars_before_offset(128, search_range_end, input)..search_range_end],
            );
            return false;
        }
    }
    is_char_count_within_range(&input[..input_ix], match_any_chars)
}

fn n_chars_before_offset(char_count: usize, offset: usize, string: &str) -> usize {
    if char_count == 0 {
        return offset;
    }
    string[..offset]
        .char_indices()
        .nth_back(char_count.saturating_sub(1))
        .map_or(0, |(byte_ix, _)| byte_ix)
}

fn is_char_count_within_range(string: &str, char_count_range: Range<usize>) -> bool {
    if string.len() >= char_count_range.start * 4 && string.len() < char_count_range.end {
        return true;
    }
    if string.len() < char_count_range.start || string.len() >= char_count_range.end * 4 {
        return false;
    }
    char_count_range.contains(&string.chars().count())
}

/// Canonicalizes license text by removing all non-alphanumeric characters, lowercasing, and turning
/// runs of whitespace into a single space. Unicode alphanumeric characters are intentionally
/// preserved since these should cause license mismatch when not within a portion of the license
/// where arbitrary text is allowed.
fn canonicalize_license_text(license: &str) -> String {
    license
        .chars()
        .filter(|c| c.is_ascii_whitespace() || c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
        .split_ascii_whitespace()
        .join(" ")
}

pub enum LicenseDetectionWatcher {
    Local {
        is_open_source_rx: watch::Receiver<bool>,
        _is_open_source_task: Task<()>,
        _worktree_subscription: Subscription,
    },
    SingleFile,
    Remote,
}

impl LicenseDetectionWatcher {
    pub fn new(worktree: &Entity<Worktree>, cx: &mut App) -> Self {
        let worktree_ref = worktree.read(cx);
        if worktree_ref.is_single_file() {
            return Self::SingleFile;
        }

        let (files_to_check_tx, mut files_to_check_rx) = futures::channel::mpsc::unbounded();

        let Worktree::Local(local_worktree) = worktree_ref else {
            return Self::Remote;
        };
        let fs = local_worktree.fs().clone();

        let options = ChildEntriesOptions {
            include_files: true,
            include_dirs: false,
            include_ignored: true,
        };
        for top_file in local_worktree.child_entries_with_options(RelPath::empty(), options) {
            let path_bytes = top_file.path.as_unix_str().as_bytes();
            if top_file.is_created() && LICENSE_FILE_NAME_REGEX.is_match(path_bytes) {
                let rel_path = top_file.path.clone();
                files_to_check_tx.unbounded_send(rel_path).ok();
            }
        }

        let _worktree_subscription =
            cx.subscribe(worktree, move |_worktree, event, _cx| match event {
                worktree::Event::UpdatedEntries(updated_entries) => {
                    for updated_entry in updated_entries.iter() {
                        let rel_path = &updated_entry.0;
                        let path_bytes = rel_path.as_unix_str().as_bytes();
                        if LICENSE_FILE_NAME_REGEX.is_match(path_bytes) {
                            files_to_check_tx.unbounded_send(rel_path.clone()).ok();
                        }
                    }
                }
                worktree::Event::DeletedEntry(_) | worktree::Event::UpdatedGitRepositories(_) => {}
            });

        let worktree_snapshot = worktree.read(cx).snapshot();
        let (mut is_open_source_tx, is_open_source_rx) = watch::channel_with::<bool>(false);

        let _is_open_source_task = cx.background_spawn(async move {
            let mut eligible_licenses = BTreeSet::new();
            while let Some(rel_path) = files_to_check_rx.next().await {
                let abs_path = worktree_snapshot.absolutize(&rel_path);
                let was_open_source = !eligible_licenses.is_empty();
                if Self::is_path_eligible(&fs, abs_path).await.unwrap_or(false) {
                    eligible_licenses.insert(rel_path);
                } else {
                    eligible_licenses.remove(&rel_path);
                }
                let is_open_source = !eligible_licenses.is_empty();
                if is_open_source != was_open_source {
                    *is_open_source_tx.borrow_mut() = is_open_source;
                }
            }
        });

        Self::Local {
            is_open_source_rx,
            _is_open_source_task,
            _worktree_subscription,
        }
    }

    async fn is_path_eligible(fs: &Arc<dyn Fs>, abs_path: PathBuf) -> Option<bool> {
        log::debug!("checking if `{abs_path:?}` is an open source license");
        // resolve symlinks so that the file size from metadata is correct
        let Some(abs_path) = fs.canonicalize(&abs_path).await.ok() else {
            log::debug!(
                "`{abs_path:?}` license file probably deleted (error canonicalizing the path)"
            );
            return None;
        };
        let metadata = fs.metadata(&abs_path).await.log_err()??;
        if metadata.len > LICENSE_PATTERNS.approximate_max_length as u64 {
            log::debug!(
                "`{abs_path:?}` license file was skipped \
                because its size of {} bytes was larger than the max size of {} bytes",
                metadata.len,
                LICENSE_PATTERNS.approximate_max_length
            );
            return None;
        }
        let text = fs.load(&abs_path).await.log_err()?;
        let is_eligible = detect_license(&text).is_some();
        if is_eligible {
            log::debug!(
                "`{abs_path:?}` matches a license that is eligible for data collection (if enabled)"
            );
        } else {
            log::debug!(
                "`{abs_path:?}` does not match a license that is eligible for data collection"
            );
        }
        Some(is_eligible)
    }

    /// Answers false until we find out it's open source
    pub fn is_project_open_source(&self) -> bool {
        match self {
            Self::Local {
                is_open_source_rx, ..
            } => *is_open_source_rx.borrow(),
            Self::SingleFile | Self::Remote => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use fs::FakeFs;
    use gpui::TestAppContext;
    use rand::Rng as _;
    use serde_json::json;
    use settings::SettingsStore;

    use super::*;

    const APACHE_2_0_TXT: &str = include_str!("../license_examples/apache-2.0-ex0.txt");
    const ISC_TXT: &str = include_str!("../license_examples/isc.txt");
    const MIT_TXT: &str = include_str!("../license_examples/mit-ex0.txt");
    const UPL_1_0_TXT: &str = include_str!("../license_examples/upl-1.0.txt");
    const BSD_0_TXT: &str = include_str!("../license_examples/0bsd.txt");

    #[track_caller]
    fn assert_matches_license(text: &str, license: OpenSourceLicense) {
        assert_eq!(detect_license(text), Some(license));
        assert!(text.len() < LICENSE_PATTERNS.approximate_max_length);
    }

    /*
    // Uncomment this and run with `cargo test -p zeta -- --no-capture &> licenses-output` to
    // traverse your entire home directory and run license detection on every file that has a
    // license-like name.
    #[test]
    fn test_check_all_licenses_in_home_dir() {
        let mut detected = Vec::new();
        let mut unrecognized = Vec::new();
        let mut walked_entries = 0;
        let homedir = std::env::home_dir().unwrap();
        for entry in walkdir::WalkDir::new(&homedir) {
            walked_entries += 1;
            if walked_entries % 10000 == 0 {
                println!(
                    "So far visited {} files in {}",
                    walked_entries,
                    homedir.display()
                );
            }
            let Ok(entry) = entry else {
                continue;
            };
            if !LICENSE_FILE_NAME_REGEX.is_match(entry.file_name().as_encoded_bytes()) {
                continue;
            }
            let Ok(contents) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            let path_string = entry.path().to_string_lossy().into_owned();
            let license = detect_license(&contents);
            match license {
                Some(license) => detected.push((license, path_string)),
                None => unrecognized.push(path_string),
            }
        }
        println!("\nDetected licenses:\n");
        detected.sort();
        for (license, path) in &detected {
            println!("{}: {}", license.spdx_identifier(), path);
        }
        println!("\nUnrecognized licenses:\n");
        for path in &unrecognized {
            println!("{}", path);
        }
        panic!(
            "{} licenses detected, {} unrecognized",
            detected.len(),
            unrecognized.len()
        );
        println!("This line has a warning to make sure this test is always commented out");
    }
    */

    #[test]
    fn test_apache_positive_detection() {
        assert_matches_license(APACHE_2_0_TXT, OpenSourceLicense::Apache2_0);
        assert_matches_license(
            include_str!("../license_examples/apache-2.0-ex1.txt"),
            OpenSourceLicense::Apache2_0,
        );
        assert_matches_license(
            include_str!("../license_examples/apache-2.0-ex2.txt"),
            OpenSourceLicense::Apache2_0,
        );
        assert_matches_license(
            include_str!("../license_examples/apache-2.0-ex3.txt"),
            OpenSourceLicense::Apache2_0,
        );
        assert_matches_license(
            include_str!("../license_examples/apache-2.0-ex4.txt"),
            OpenSourceLicense::Apache2_0,
        );
        assert_matches_license(
            include_str!("../../../LICENSE-APACHE"),
            OpenSourceLicense::Apache2_0,
        );
    }

    #[test]
    fn test_apache_negative_detection() {
        assert_eq!(
            detect_license(&format!(
                "{APACHE_2_0_TXT}\n\nThe terms in this license are void if P=NP."
            )),
            None
        );
    }

    #[test]
    fn test_bsd_1_clause_positive_detection() {
        assert_matches_license(
            include_str!("../license_examples/bsd-1-clause.txt"),
            OpenSourceLicense::BSD,
        );
    }

    #[test]
    fn test_bsd_2_clause_positive_detection() {
        assert_matches_license(
            include_str!("../license_examples/bsd-2-clause-ex0.txt"),
            OpenSourceLicense::BSD,
        );
    }

    #[test]
    fn test_bsd_3_clause_positive_detection() {
        assert_matches_license(
            include_str!("../license_examples/bsd-3-clause-ex0.txt"),
            OpenSourceLicense::BSD,
        );
        assert_matches_license(
            include_str!("../license_examples/bsd-3-clause-ex1.txt"),
            OpenSourceLicense::BSD,
        );
        assert_matches_license(
            include_str!("../license_examples/bsd-3-clause-ex2.txt"),
            OpenSourceLicense::BSD,
        );
        assert_matches_license(
            include_str!("../license_examples/bsd-3-clause-ex3.txt"),
            OpenSourceLicense::BSD,
        );
        assert_matches_license(
            include_str!("../license_examples/bsd-3-clause-ex4.txt"),
            OpenSourceLicense::BSD,
        );
    }

    #[test]
    fn test_bsd_0_positive_detection() {
        assert_matches_license(BSD_0_TXT, OpenSourceLicense::BSDZero);
    }

    #[test]
    fn test_isc_positive_detection() {
        assert_matches_license(ISC_TXT, OpenSourceLicense::ISC);
    }

    #[test]
    fn test_isc_negative_detection() {
        let license_text = format!(
            r#"{ISC_TXT}

            This project is dual licensed under the ISC License and the MIT License."#
        );

        assert_eq!(detect_license(&license_text), None);
    }

    #[test]
    fn test_mit_positive_detection() {
        assert_matches_license(MIT_TXT, OpenSourceLicense::MIT);
        assert_matches_license(
            include_str!("../license_examples/mit-ex1.txt"),
            OpenSourceLicense::MIT,
        );
        assert_matches_license(
            include_str!("../license_examples/mit-ex2.txt"),
            OpenSourceLicense::MIT,
        );
        assert_matches_license(
            include_str!("../license_examples/mit-ex3.txt"),
            OpenSourceLicense::MIT,
        );
    }

    #[test]
    fn test_mit_negative_detection() {
        let license_text = format!(
            r#"{MIT_TXT}

            This project is dual licensed under the MIT License and the Apache License, Version 2.0."#
        );
        assert_eq!(detect_license(&license_text), None);
    }

    #[test]
    fn test_upl_positive_detection() {
        assert_matches_license(UPL_1_0_TXT, OpenSourceLicense::UPL1_0);
    }

    #[test]
    fn test_upl_negative_detection() {
        let license_text = format!(
            r#"{UPL_1_0_TXT}

            This project is dual licensed under the UPL License and the MIT License."#
        );

        assert_eq!(detect_license(&license_text), None);
    }

    #[test]
    fn test_zlib_positive_detection() {
        assert_matches_license(
            include_str!("../license_examples/zlib-ex0.txt"),
            OpenSourceLicense::Zlib,
        );
    }

    #[test]
    fn random_strings_negative_detection() {
        for _i in 0..20 {
            let random_string = rand::rng()
                .sample_iter::<char, _>(rand::distr::StandardUniform)
                .take(512)
                .collect::<String>();
            assert_eq!(detect_license(&random_string), None);
        }
    }

    #[test]
    fn test_n_chars_before_offset() {
        assert_eq!(n_chars_before_offset(2, 4, "hello"), 2);

        let input = "ㄒ乇丂ㄒ";
        assert_eq!(n_chars_before_offset(2, input.len(), input), "ㄒ乇".len());
    }

    #[test]
    fn test_is_char_count_within_range() {
        // TODO: make this into a proper property test.
        for _i in 0..20 {
            let mut rng = rand::rng();
            let random_char_count = rng.random_range(0..64);
            let random_string = rand::rng()
                .sample_iter::<char, _>(rand::distr::StandardUniform)
                .take(random_char_count)
                .collect::<String>();
            let min_chars = rng.random_range(0..10);
            let max_chars = rng.random_range(min_chars..32);
            let char_count_range = min_chars..max_chars;
            assert_eq!(
                is_char_count_within_range(&random_string, char_count_range.clone()),
                char_count_range.contains(&random_char_count),
            );
        }
    }

    #[test]
    fn test_license_file_name_regex() {
        // Test basic license file names
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"licence"));

        // Test with extensions
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.txt"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.md"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE.txt"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE.md"));

        // Test with specific license types
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-APACHE"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE_MIT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-ISC"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-UPL"));

        // Test with "license" coming after
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"APACHE-LICENSE"));

        // Test version numbers
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"APACHE-2"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"APACHE-2.0"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"BSD-1"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"BSD-2"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"BSD-3"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"BSD-3-CLAUSE"));

        // Test combinations
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-MIT.txt"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE.ISC.md"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license_upl"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.APACHE.2.0"));

        // Test case insensitive
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"License"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license-mit.TXT"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"LICENCE_isc.MD"));

        // Test edge cases that should match
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"license.mit"));
        assert!(LICENSE_FILE_NAME_REGEX.is_match(b"licence-upl.txt"));

        // Test non-matching patterns
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"COPYING"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.html"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"MYLICENSE"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"src/LICENSE"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE.old"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSE-GPL"));
        assert!(!LICENSE_FILE_NAME_REGEX.is_match(b"LICENSEABC"));
    }

    #[test]
    fn test_canonicalize_license_text() {
        let input = "  Paragraph 1\nwith multiple lines\n\n\n\nParagraph 2\nwith more lines\n  ";
        let expected = "paragraph 1 with multiple lines paragraph 2 with more lines";
        assert_eq!(canonicalize_license_text(input), expected);

        // Test tabs and mixed whitespace
        let input = "Word1\t\tWord2\n\n   Word3\r\n\r\n\r\nWord4   ";
        let expected = "word1 word2 word3 word4";
        assert_eq!(canonicalize_license_text(input), expected);
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    #[gpui::test]
    async fn test_watcher_single_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "main.rs": "fn main() {}" }))
            .await;

        let worktree = Worktree::local(
            Path::new("/root/main.rs"),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let watcher = cx.update(|cx| LicenseDetectionWatcher::new(&worktree, cx));
        assert!(matches!(watcher, LicenseDetectionWatcher::SingleFile));
        assert!(!watcher.is_project_open_source());
    }

    #[gpui::test]
    async fn test_watcher_updates_on_changes(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "main.rs": "fn main() {}" }))
            .await;

        let worktree = Worktree::local(
            Path::new("/root"),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let watcher = cx.update(|cx| LicenseDetectionWatcher::new(&worktree, cx));
        assert!(matches!(watcher, LicenseDetectionWatcher::Local { .. }));
        assert!(!watcher.is_project_open_source());

        fs.write(Path::new("/root/LICENSE-MIT"), MIT_TXT.as_bytes())
            .await
            .unwrap();

        cx.background_executor.run_until_parked();
        assert!(watcher.is_project_open_source());

        fs.write(Path::new("/root/LICENSE-APACHE"), APACHE_2_0_TXT.as_bytes())
            .await
            .unwrap();

        cx.background_executor.run_until_parked();
        assert!(watcher.is_project_open_source());

        fs.write(Path::new("/root/LICENSE-MIT"), "Nevermind".as_bytes())
            .await
            .unwrap();

        // Still considered open source as LICENSE-APACHE is present
        cx.background_executor.run_until_parked();
        assert!(watcher.is_project_open_source());

        fs.write(
            Path::new("/root/LICENSE-APACHE"),
            "Also nevermind".as_bytes(),
        )
        .await
        .unwrap();

        cx.background_executor.run_until_parked();
        assert!(!watcher.is_project_open_source());
    }

    #[gpui::test]
    async fn test_watcher_initially_opensource_and_then_deleted(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({ "main.rs": "fn main() {}", "LICENSE-MIT": MIT_TXT }),
        )
        .await;

        let worktree = Worktree::local(
            Path::new("/root"),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let watcher = cx.update(|cx| LicenseDetectionWatcher::new(&worktree, cx));
        assert!(matches!(watcher, LicenseDetectionWatcher::Local { .. }));

        cx.background_executor.run_until_parked();
        assert!(watcher.is_project_open_source());

        fs.remove_file(
            Path::new("/root/LICENSE-MIT"),
            fs::RemoveOptions {
                recursive: false,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();

        cx.background_executor.run_until_parked();
        assert!(!watcher.is_project_open_source());
    }
}
