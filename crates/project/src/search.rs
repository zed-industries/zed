use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::{Context as _, Ok, Result, bail};
use client::proto;
use fancy_regex::{Captures, Regex, RegexBuilder};
use fs::MTime;
use globset::{GlobBuilder, GlobMatcher};
use gpui::Entity;
use itertools::Itertools as _;
use language::{Buffer, BufferSnapshot, CharKind};
use smol::future::yield_now;
use std::{
    borrow::Cow,
    collections::BTreeSet,
    io::{BufRead, BufReader, Read},
    ops::Range,
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime},
};
use text::Anchor;
use util::{
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};

#[derive(Debug)]
pub enum SearchResult {
    Buffer {
        buffer: Entity<Buffer>,
        ranges: Vec<Range<Anchor>>,
    },
    LimitReached,
    WaitingForScan,
    Searching,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SearchInputKind {
    Query,
    Include,
    Exclude,
}

#[derive(Clone, Debug)]
pub struct SearchInputs {
    query: Arc<str>,
    files_to_include: PathMatcher,
    files_to_exclude: PathMatcher,
    metadata_filters: MetadataFilters,
    match_full_paths: bool,
    buffers: Option<Vec<Entity<Buffer>>>,
}

/// A `find(1)`-style numeric comparison: `+N` matches values greater than `N`
/// and `-N` values less than `N`. What a bare `N` means is left to each
/// predicate: `-mtime`/`-mmin` read it as "equal to", `-size` as "greater than".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindComparison {
    GreaterThan(u64),
    LessThan(u64),
    Equal(u64),
}

impl FindComparison {
    fn matches(self, value: u64) -> bool {
        match self {
            Self::GreaterThan(threshold) => value > threshold,
            Self::LessThan(threshold) => value < threshold,
            Self::Equal(threshold) => value == threshold,
        }
    }

    /// Splits the leading `+`/`-` off an operand. The constructor is `None` when
    /// the operand carries no sign, so each predicate can supply its own default.
    fn split(operand: &str) -> (Option<fn(u64) -> Self>, &str) {
        if let Some(rest) = operand.strip_prefix('+') {
            (Some(Self::GreaterThan), rest)
        } else if let Some(rest) = operand.strip_prefix('-') {
            (Some(Self::LessThan), rest)
        } else {
            (None, operand)
        }
    }
}

const SECONDS_PER_DAY: u64 = 60 * 60 * 24;
const SECONDS_PER_MINUTE: u64 = 60;

/// File metadata filters modelled on the `find(1)` predicates of the same name.
///
/// These are matched against the worktree's own entry metadata, so they cost no
/// extra syscalls and prune candidates before any file is read.
///
/// The syntax is `find`-inspired rather than `find`-compatible. `+N`/`-N` are
/// plain greater-than/less-than comparisons and round down rather than up, and
/// `-size` in particular defaults to KiB and to "greater than" -- see
/// [`MetadataFilters::parse_size`]. Predicates are whitespace-separated, so a
/// `-name` glob cannot contain spaces.
#[derive(Clone, Debug)]
pub struct MetadataFilters {
    /// Verbatim user input, retained so the filters can round-trip over the
    /// wire without having to re-render the parsed form.
    source: String,
    /// Glob on the base name, as `find -name` (or `-iname`, case-insensitive).
    name: Option<GlobMatcher>,
    /// File size, in bytes.
    size: Option<FindComparison>,
    /// Age in whole days, as `find -mtime`.
    mtime: Option<FindComparison>,
    /// Age in whole minutes, as `find -mmin`.
    mmin: Option<FindComparison>,
    /// Captured once when the query is built, so every entry in a single search
    /// is aged against the same clock.
    reference_time: SystemTime,
}

impl Default for MetadataFilters {
    fn default() -> Self {
        Self {
            source: String::new(),
            name: None,
            size: None,
            mtime: None,
            mmin: None,
            reference_time: SystemTime::UNIX_EPOCH,
        }
    }
}

impl MetadataFilters {
    pub fn new(source: &str) -> Result<Self> {
        Self::new_at(source, SystemTime::now())
    }

    fn new_at(source: &str, reference_time: SystemTime) -> Result<Self> {
        let mut this = Self {
            source: source.to_owned(),
            reference_time,
            ..Default::default()
        };

        let mut tokens = source.split_whitespace();
        while let Some(predicate) = tokens.next() {
            let operand = tokens
                .next()
                .with_context(|| format!("`{predicate}` is missing a value"))?;
            match predicate {
                // `-name` and `-iname` populate the same slot and differ only in
                // case folding, so the last one written wins -- the same way a
                // repeated `-size` does.
                "-name" => this.name = Some(Self::parse_name(predicate, operand, false)?),
                "-iname" => this.name = Some(Self::parse_name(predicate, operand, true)?),
                "-size" => this.size = Some(Self::parse_size(operand)?),
                "-mtime" => this.mtime = Some(Self::parse_count(predicate, operand)?),
                "-mmin" => this.mmin = Some(Self::parse_count(predicate, operand)?),
                _ => bail!(
                    "unknown filter `{predicate}`, expected -name, -iname, -size, -mtime or -mmin"
                ),
            }
        }

        std::result::Result::Ok(this)
    }

    /// Parses a `-name`/`-iname` operand: a glob matched against the base name
    /// only, as in `find`. `literal_separator` is left off because the value
    /// never contains a separator to begin with.
    fn parse_name(predicate: &str, operand: &str, case_insensitive: bool) -> Result<GlobMatcher> {
        let glob = GlobBuilder::new(operand)
            .case_insensitive(case_insensitive)
            .build()
            .with_context(|| format!("`{predicate} {operand}` is not a valid glob"))?;
        std::result::Result::Ok(glob.compile_matcher())
    }

    /// Parses a `-size` operand.
    ///
    /// Two deliberate departures from `find`, both aimed at the common case of
    /// "show me the big files":
    /// - the default unit is KiB, not 512-byte blocks (`-size 9` is 9 KiB);
    /// - an unsigned value means "greater than", not "equal to" (`-size 9` is
    ///   `+9k`, and `-size +1` is "over 1024 bytes").
    ///
    /// An explicit unit suffix still wins: `c` bytes, `b` 512-byte blocks, `k`,
    /// `M`, `G`.
    fn parse_size(operand: &str) -> Result<FindComparison> {
        let (comparison, digits) = FindComparison::split(operand);
        const KIB: u64 = 1 << 10;
        let (digits, unit) = match digits.as_bytes().last() {
            Some(b'c') => (&digits[..digits.len() - 1], 1),
            Some(b'b') => (&digits[..digits.len() - 1], 512),
            Some(b'k') => (&digits[..digits.len() - 1], KIB),
            Some(b'M') => (&digits[..digits.len() - 1], 1 << 20),
            Some(b'G') => (&digits[..digits.len() - 1], 1 << 30),
            _ => (digits, KIB),
        };
        let count: u64 = digits.parse().with_context(|| {
            format!("`-size {operand}` is not a number with an optional c/b/k/M/G suffix")
        })?;
        let bytes = count
            .checked_mul(unit)
            .with_context(|| format!("`-size {operand}` overflows"))?;
        std::result::Result::Ok(comparison.unwrap_or(FindComparison::GreaterThan)(bytes))
    }

    /// Parses a `-mtime`/`-mmin` operand. Unlike `-size`, an unsigned value here
    /// keeps `find`'s "equal to" meaning.
    fn parse_count(predicate: &str, operand: &str) -> Result<FindComparison> {
        let (comparison, digits) = FindComparison::split(operand);
        let count: u64 = digits
            .parse()
            .with_context(|| format!("`{predicate} {operand}` is not a number"))?;
        std::result::Result::Ok(comparison.unwrap_or(FindComparison::Equal)(count))
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.size.is_none() && self.mtime.is_none() && self.mmin.is_none()
    }

    /// The text these filters were parsed from, for round-tripping over RPC.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Tests a worktree entry against every configured predicate. `file_name` is
    /// the entry's base name, matched by `-name`.
    ///
    /// `mtime` is `None` for entries whose modification time the worktree scan
    /// could not read. Such an entry cannot be shown to satisfy an age filter,
    /// so it is rejected rather than silently passed through. An mtime in the
    /// future (clock skew, or a file written mid-scan) is treated as age zero.
    pub fn matches(&self, file_name: &str, size: u64, mtime: Option<MTime>) -> bool {
        if let Some(name) = &self.name
            && !name.is_match(file_name)
        {
            return false;
        }

        if let Some(size_filter) = self.size
            && !size_filter.matches(size)
        {
            return false;
        }

        if self.mtime.is_none() && self.mmin.is_none() {
            return true;
        }
        let Some(mtime) = mtime else {
            return false;
        };
        let age = self
            .reference_time
            .duration_since(mtime.timestamp_for_user())
            .unwrap_or(Duration::ZERO)
            .as_secs();

        if let Some(mtime_filter) = self.mtime
            && !mtime_filter.matches(age / SECONDS_PER_DAY)
        {
            return false;
        }
        if let Some(mmin_filter) = self.mmin
            && !mmin_filter.matches(age / SECONDS_PER_MINUTE)
        {
            return false;
        }
        true
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MatchPositionHint {
    Line(u32),
    ByteOffset(usize),
}

impl Default for MatchPositionHint {
    fn default() -> Self {
        Self::Line(0)
    }
}

impl SearchInputs {
    pub fn as_str(&self) -> &str {
        self.query.as_ref()
    }
    pub fn files_to_include(&self) -> &PathMatcher {
        &self.files_to_include
    }
    pub fn files_to_exclude(&self) -> &PathMatcher {
        &self.files_to_exclude
    }
    pub fn metadata_filters(&self) -> &MetadataFilters {
        &self.metadata_filters
    }
    pub fn buffers(&self) -> &Option<Vec<Entity<Buffer>>> {
        &self.buffers
    }
}
#[derive(Clone, Debug)]
pub enum SearchQuery {
    Text {
        search: AhoCorasick,
        replacement: Option<String>,
        whole_word: bool,
        case_sensitive: bool,
        include_ignored: bool,
        inner: SearchInputs,
    },
    Regex {
        regex: Regex,
        replacement: Option<String>,
        whole_word: bool,
        case_sensitive: bool,
        include_ignored: bool,
        one_match_per_line: bool,
        inner: SearchInputs,
        escaped: bool,
    },
}

static WORD_MATCH_TEST: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r"\B")
        .build()
        .expect("Failed to create WORD_MATCH_TEST")
});

impl SearchQuery {
    /// Create a text query
    ///
    /// If `match_full_paths` is true, include/exclude patterns will always be matched against fully qualified project paths beginning with a project root.
    /// If `match_full_paths` is false, patterns will be matched against worktree-relative paths.
    pub fn text(
        query: impl ToString,
        whole_word: bool,
        case_sensitive: bool,
        include_ignored: bool,
        files_to_include: PathMatcher,
        files_to_exclude: PathMatcher,
        match_full_paths: bool,
        buffers: Option<Vec<Entity<Buffer>>>,
    ) -> Result<Self> {
        let mut query = query.to_string();
        text::LineEnding::normalize(&mut query);
        if !case_sensitive && !query.is_ascii() {
            // AhoCorasickBuilder doesn't support case-insensitive search with unicode characters
            // Fallback to regex search as recommended by
            // https://docs.rs/aho-corasick/1.1/aho_corasick/struct.AhoCorasickBuilder.html#method.ascii_case_insensitive
            return Self::escaped_regex(
                query,
                whole_word,
                case_sensitive,
                include_ignored,
                files_to_include,
                files_to_exclude,
                match_full_paths,
                buffers,
            );
        }
        let search = AhoCorasickBuilder::new()
            .ascii_case_insensitive(!case_sensitive)
            .build([&query])?;
        let inner = SearchInputs {
            query: query.into(),
            files_to_exclude,
            files_to_include,
            metadata_filters: MetadataFilters::default(),
            match_full_paths,
            buffers,
        };
        Ok(Self::Text {
            search,
            replacement: None,
            whole_word,
            case_sensitive,
            include_ignored,
            inner,
        })
    }

    /// Create a regex query
    ///
    /// If `match_full_paths` is true, include/exclude patterns will be matched against fully qualified project paths
    /// beginning with a project root name. If false, they will be matched against project-relative paths (which don't start
    /// with their respective project root).
    pub fn regex(
        query: impl ToString,
        whole_word: bool,
        case_sensitive: bool,
        include_ignored: bool,
        one_match_per_line: bool,
        files_to_include: PathMatcher,
        files_to_exclude: PathMatcher,
        match_full_paths: bool,
        buffers: Option<Vec<Entity<Buffer>>>,
    ) -> Result<Self> {
        let query = query.to_string();
        let inner = SearchInputs {
            query: Arc::from(query.as_str()),
            files_to_include,
            files_to_exclude,
            metadata_filters: MetadataFilters::default(),
            match_full_paths,
            buffers,
        };
        Self::build_regex(
            query,
            whole_word,
            case_sensitive,
            include_ignored,
            one_match_per_line,
            inner,
            false,
        )
    }

    /// Create a regex query from a literal string, escaping any regex
    /// metacharacters so that the resulting query matches the literal text.
    ///
    /// Unlike `regex`, the query stored on the resulting `SearchQuery` is the
    /// original unescaped text, so `as_str` returns what the user typed.
    pub fn escaped_regex(
        query: impl ToString,
        whole_word: bool,
        case_sensitive: bool,
        include_ignored: bool,
        files_to_include: PathMatcher,
        files_to_exclude: PathMatcher,
        match_full_paths: bool,
        buffers: Option<Vec<Entity<Buffer>>>,
    ) -> Result<Self> {
        let mut query = query.to_string();
        text::LineEnding::normalize(&mut query);
        let inner = SearchInputs {
            query: Arc::from(query.as_str()),
            files_to_include,
            files_to_exclude,
            metadata_filters: MetadataFilters::default(),
            match_full_paths,
            buffers,
        };
        Self::build_regex(
            regex::escape(&query),
            whole_word,
            case_sensitive,
            include_ignored,
            false,
            inner,
            true,
        )
    }

    fn build_regex(
        mut pattern: String,
        whole_word: bool,
        mut case_sensitive: bool,
        include_ignored: bool,
        one_match_per_line: bool,
        inner: SearchInputs,
        escaped: bool,
    ) -> Result<Self> {
        if let Some((case_sensitive_from_pattern, new_pattern)) =
            Self::case_sensitive_from_pattern(&pattern)
        {
            case_sensitive = case_sensitive_from_pattern;
            pattern = new_pattern
        }

        if whole_word {
            let mut word_pattern = String::new();
            if let Some(first) = pattern.get(0..1)
                && WORD_MATCH_TEST.is_match(first).is_ok_and(|x| !x)
            {
                word_pattern.push_str("\\b");
            }
            word_pattern.push_str(&pattern);
            if let Some(last) = pattern.get(pattern.len() - 1..)
                && WORD_MATCH_TEST.is_match(last).is_ok_and(|x| !x)
            {
                word_pattern.push_str("\\b");
            }
            pattern = word_pattern
        }

        let regex = RegexBuilder::new(&pattern)
            .case_insensitive(!case_sensitive)
            .multi_line(true)
            .crlf(true)
            .build()?;
        Ok(Self::Regex {
            regex,
            replacement: None,
            whole_word,
            case_sensitive,
            include_ignored,
            inner,
            one_match_per_line,
            escaped,
        })
    }

    /// Extracts case sensitivity settings from pattern items in the provided
    /// query and returns the same query, with the pattern items removed.
    ///
    /// The following pattern modifiers are supported:
    ///
    /// - `\c` (case_sensitive: false)
    /// - `\C` (case_sensitive: true)
    ///
    /// If no pattern item were found, `None` will be returned.
    fn case_sensitive_from_pattern(query: &str) -> Option<(bool, String)> {
        if !(query.contains("\\c") || query.contains("\\C")) {
            return None;
        }

        let mut was_escaped = false;
        let mut new_query = String::new();
        let mut is_case_sensitive = None;

        for c in query.chars() {
            if was_escaped {
                if c == 'c' {
                    is_case_sensitive = Some(false);
                } else if c == 'C' {
                    is_case_sensitive = Some(true);
                } else {
                    new_query.push('\\');
                    new_query.push(c);
                }
                was_escaped = false
            } else if c == '\\' {
                was_escaped = true
            } else {
                new_query.push(c);
            }
        }

        is_case_sensitive.map(|c| (c, new_query))
    }

    pub fn from_proto(message: proto::SearchQuery, path_style: PathStyle) -> Result<Self> {
        let files_to_include = if message.files_to_include.is_empty() {
            message
                .files_to_include_legacy
                .split(',')
                .map(str::trim)
                .filter(|&glob_str| !glob_str.is_empty())
                .map(|s| s.to_string())
                .collect()
        } else {
            message.files_to_include
        };

        let files_to_exclude = if message.files_to_exclude.is_empty() {
            message
                .files_to_exclude_legacy
                .split(',')
                .map(str::trim)
                .filter(|&glob_str| !glob_str.is_empty())
                .map(|s| s.to_string())
                .collect()
        } else {
            message.files_to_exclude
        };

        // Re-parsed here rather than sent pre-parsed, so `-mtime`/`-mmin` are
        // aged against the host's clock -- the same clock the mtimes come from.
        let metadata_filters = MetadataFilters::new(&message.metadata_filters)?;

        let query = if message.regex {
            Self::regex(
                message.query,
                message.whole_word,
                message.case_sensitive,
                message.include_ignored,
                false,
                PathMatcher::new(files_to_include, path_style)?,
                PathMatcher::new(files_to_exclude, path_style)?,
                message.match_full_paths,
                None, // search opened only don't need search remote
            )?
        } else {
            Self::text(
                message.query,
                message.whole_word,
                message.case_sensitive,
                message.include_ignored,
                PathMatcher::new(files_to_include, path_style)?,
                PathMatcher::new(files_to_exclude, path_style)?,
                message.match_full_paths,
                None, // search opened only don't need search remote
            )?
        };
        Ok(query.with_metadata_filters(metadata_filters))
    }

    pub fn with_replacement(mut self, new_replacement: String) -> Self {
        match self {
            Self::Text {
                ref mut replacement,
                ..
            }
            | Self::Regex {
                ref mut replacement,
                ..
            } => {
                *replacement = Some(new_replacement);
                self
            }
        }
    }

    pub fn to_proto(&self) -> proto::SearchQuery {
        let mut files_to_include = self.files_to_include().sources();
        let mut files_to_exclude = self.files_to_exclude().sources();
        proto::SearchQuery {
            query: self.as_str().to_string(),
            regex: self.is_regex(),
            whole_word: self.whole_word(),
            case_sensitive: self.case_sensitive(),
            include_ignored: self.include_ignored(),
            files_to_include: files_to_include.clone().map(ToOwned::to_owned).collect(),
            files_to_exclude: files_to_exclude.clone().map(ToOwned::to_owned).collect(),
            match_full_paths: self.match_full_paths(),
            metadata_filters: self.metadata_filters().source().to_string(),
            // Populate legacy fields for backwards compatibility
            files_to_include_legacy: files_to_include.join(","),
            files_to_exclude_legacy: files_to_exclude.join(","),
        }
    }

    pub async fn detect(
        &self,
        mut reader: BufReader<Box<dyn Read + Send + Sync>>,
    ) -> Result<Option<MatchPositionHint>> {
        let query_str = self.as_str();
        if query_str.is_empty() {
            return Ok(None);
        }

        // Yield from this function every 20KB scanned.
        const YIELD_THRESHOLD: usize = 20 * 1024;

        match self {
            Self::Text { search, .. } => {
                let mut text = String::new();
                if query_str.contains('\n') {
                    reader.read_to_string(&mut text)?;
                    text::LineEnding::normalize(&mut text);
                    if search.is_match(&text) {
                        Ok(Some(MatchPositionHint::default()))
                    } else {
                        Ok(None)
                    }
                } else {
                    let mut bytes_read = 0;
                    let mut line_number = u32::default();
                    while reader.read_line(&mut text)? > 0 {
                        if search.is_match(&text) {
                            return Ok(Some(MatchPositionHint::Line(line_number)));
                        }
                        bytes_read += text.len();
                        if bytes_read >= YIELD_THRESHOLD {
                            bytes_read = 0;
                            smol::future::yield_now().await;
                        }
                        text.clear();
                        line_number += 1;
                    }
                    Ok(None)
                }
            }
            Self::Regex { regex, .. } => {
                let mut text = String::new();

                reader.read_to_string(&mut text)?;
                text::LineEnding::normalize(&mut text);
                if let Some(m) = regex.find(&text)? {
                    Ok(Some(MatchPositionHint::ByteOffset(m.start())))
                } else {
                    Ok(None)
                }
            }
        }
    }
    /// Returns the replacement text for this `SearchQuery`.
    pub fn replacement(&self) -> Option<&str> {
        match self {
            SearchQuery::Text { replacement, .. } | SearchQuery::Regex { replacement, .. } => {
                replacement.as_deref()
            }
        }
    }
    /// Replaces search hits if replacement is set. `text` is assumed to be a string that matches this `SearchQuery` exactly, without any leftovers on either side.
    pub fn replacement_for<'a>(&self, text: &'a str) -> Option<Cow<'a, str>> {
        match self {
            SearchQuery::Text { replacement, .. }
            | SearchQuery::Regex {
                replacement,
                escaped: true,
                ..
            } => replacement.clone().map(Cow::from),

            SearchQuery::Regex {
                regex,
                replacement: Some(replacement),
                escaped: false,
                ..
            } => {
                static TEXT_REPLACEMENT_SPECIAL_CHARACTERS_REGEX: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r"\\\\|\\n|\\t").unwrap());
                let replacement = TEXT_REPLACEMENT_SPECIAL_CHARACTERS_REGEX.replace_all(
                    replacement,
                    |c: &Captures| match c.get(0).unwrap().as_str() {
                        r"\\" => "\\",
                        r"\n" => "\n",
                        r"\t" => "\t",
                        x => unreachable!("Unexpected escape sequence: {}", x),
                    },
                );
                Some(regex.replace(text, replacement))
            }

            SearchQuery::Regex {
                replacement: None, ..
            } => None,
        }
    }

    pub async fn search(
        &self,
        buffer: &BufferSnapshot,
        subrange: Option<Range<usize>>,
    ) -> Vec<Range<usize>> {
        const YIELD_INTERVAL: usize = 20000;

        if self.as_str().is_empty() {
            return Default::default();
        }

        let range_offset = subrange.as_ref().map(|r| r.start).unwrap_or(0);
        let rope = if let Some(range) = subrange {
            buffer.as_rope().slice(range)
        } else {
            buffer.as_rope().clone()
        };

        let mut matches = Vec::new();
        match self {
            Self::Text {
                search, whole_word, ..
            } => {
                for (ix, mat) in search
                    .stream_find_iter(rope.bytes_in_range(0..rope.len()))
                    .enumerate()
                {
                    if (ix + 1) % YIELD_INTERVAL == 0 {
                        yield_now().await;
                    }

                    let mat = mat.unwrap();
                    if *whole_word {
                        let classifier = buffer.char_classifier_at(range_offset + mat.start());

                        let prev_kind = rope
                            .reversed_chars_at(mat.start())
                            .next()
                            .map(|c| classifier.kind(c));
                        let start_kind =
                            classifier.kind(rope.chars_at(mat.start()).next().unwrap());
                        let end_kind =
                            classifier.kind(rope.reversed_chars_at(mat.end()).next().unwrap());
                        let next_kind = rope.chars_at(mat.end()).next().map(|c| classifier.kind(c));
                        if (Some(start_kind) == prev_kind && start_kind == CharKind::Word)
                            || (Some(end_kind) == next_kind && end_kind == CharKind::Word)
                        {
                            continue;
                        }
                    }
                    matches.push(mat.start()..mat.end())
                }
            }

            Self::Regex {
                regex,
                one_match_per_line,
                ..
            } => {
                let text = rope.to_string();
                let mut seen_lines = BTreeSet::default();
                for (ix, mat) in regex.find_iter(&text).enumerate() {
                    if (ix + 1) % YIELD_INTERVAL == 0 {
                        yield_now().await;
                    }

                    if let std::result::Result::Ok(mat) = mat {
                        let should_push = if *one_match_per_line {
                            // ensure that only one match per line is returned.
                            let pos = buffer.offset_to_point(mat.start());
                            seen_lines.insert(pos.row)
                        } else {
                            true
                        };
                        if should_push {
                            matches.push(mat.start()..mat.end());
                        }
                    }
                }
            }
        }

        matches
    }

    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    pub fn as_str(&self) -> &str {
        self.as_inner().as_str()
    }

    pub fn whole_word(&self) -> bool {
        match self {
            Self::Text { whole_word, .. } => *whole_word,
            Self::Regex { whole_word, .. } => *whole_word,
        }
    }

    pub fn case_sensitive(&self) -> bool {
        match self {
            Self::Text { case_sensitive, .. } => *case_sensitive,
            Self::Regex { case_sensitive, .. } => *case_sensitive,
        }
    }

    pub fn include_ignored(&self) -> bool {
        match self {
            Self::Text {
                include_ignored, ..
            } => *include_ignored,
            Self::Regex {
                include_ignored, ..
            } => *include_ignored,
        }
    }

    pub fn is_regex(&self) -> bool {
        matches!(self, Self::Regex { .. })
    }

    pub fn files_to_include(&self) -> &PathMatcher {
        self.as_inner().files_to_include()
    }

    pub fn files_to_exclude(&self) -> &PathMatcher {
        self.as_inner().files_to_exclude()
    }

    pub fn metadata_filters(&self) -> &MetadataFilters {
        self.as_inner().metadata_filters()
    }

    /// Attaches `find(1)`-style size/age filters. Kept as a builder rather than
    /// a constructor parameter so the many existing `text`/`regex` call sites
    /// stay untouched.
    pub fn with_metadata_filters(mut self, filters: MetadataFilters) -> Self {
        match &mut self {
            Self::Text { inner, .. } | Self::Regex { inner, .. } => {
                inner.metadata_filters = filters
            }
        }
        self
    }

    /// Whether any file metadata predicate is configured. Deliberately separate
    /// from [`Self::filters_path`], which gates the path glob check.
    pub fn filters_metadata(&self) -> bool {
        !self.metadata_filters().is_empty()
    }

    pub fn buffers(&self) -> Option<&Vec<Entity<Buffer>>> {
        self.as_inner().buffers.as_ref()
    }

    pub fn is_opened_only(&self) -> bool {
        self.as_inner().buffers.is_some()
    }

    pub fn filters_path(&self) -> bool {
        !(self.files_to_exclude().sources().next().is_none()
            && self.files_to_include().sources().next().is_none())
    }

    pub fn match_full_paths(&self) -> bool {
        self.as_inner().match_full_paths
    }

    /// Check match full paths to determine whether you're required to pass a fully qualified
    /// project path (starts with a project root).
    pub fn match_path(&self, file_path: &RelPath) -> bool {
        let mut path = file_path.to_rel_path_buf();
        loop {
            if self.files_to_exclude().is_match(&path) {
                return false;
            } else if self.files_to_include().sources().next().is_none()
                || self.files_to_include().is_match(&path)
            {
                return true;
            } else if !path.pop() {
                return false;
            }
        }
    }
    pub fn as_inner(&self) -> &SearchInputs {
        match self {
            Self::Regex { inner, .. } | Self::Text { inner, .. } => inner,
        }
    }

    pub fn search_str(&self, text: &str) -> Vec<Range<usize>> {
        if self.as_str().is_empty() {
            return Vec::new();
        }

        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

        let mut matches = Vec::new();
        match self {
            Self::Text {
                search, whole_word, ..
            } => {
                for mat in search.find_iter(text.as_bytes()) {
                    if *whole_word {
                        let prev_char = text[..mat.start()].chars().last();
                        let next_char = text[mat.end()..].chars().next();
                        if prev_char.is_some_and(&is_word_char)
                            || next_char.is_some_and(&is_word_char)
                        {
                            continue;
                        }
                    }
                    matches.push(mat.start()..mat.end());
                }
            }
            Self::Regex { regex, .. } => {
                for mat in regex.find_iter(text).flatten() {
                    matches.push(mat.start()..mat.end());
                }
            }
        }
        matches
    }
}

#[cfg(test)]
mod metadata_filter_tests {
    use super::*;

    /// A fixed clock, so `-mtime`/`-mmin` assertions don't depend on wall time.
    fn now() -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000_000)
    }

    fn filters(source: &str) -> MetadataFilters {
        MetadataFilters::new_at(source, now()).unwrap()
    }

    fn aged(seconds: u64) -> Option<MTime> {
        let since_epoch = now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Some(MTime::from_seconds_and_nanos(since_epoch - seconds, 0))
    }

    /// Assertions that are only about size or age use a fixed base name.
    const ANY_NAME: &str = "file.txt";

    #[test]
    fn parses_all_predicates() {
        let parsed = filters("-iname *.rs -size +1k -mtime -7 -mmin +30");
        assert!(parsed.name.is_some());
        assert_eq!(parsed.size, Some(FindComparison::GreaterThan(1024)));
        assert_eq!(parsed.mtime, Some(FindComparison::LessThan(7)));
        assert_eq!(parsed.mmin, Some(FindComparison::GreaterThan(30)));
        assert!(!parsed.is_empty());
    }

    #[test]
    fn iname_folds_case_and_name_does_not() {
        let any_case = filters("-iname *.rs");
        assert!(any_case.matches("main.rs", 0, None));
        assert!(any_case.matches("MAIN.RS", 0, None));
        assert!(!any_case.matches("main.ts", 0, None));

        let exact_case = filters("-name *.rs");
        assert!(exact_case.matches("main.rs", 0, None));
        assert!(!exact_case.matches("MAIN.RS", 0, None));

        // Both write the same slot, so the last one wins.
        assert!(!filters("-iname *.rs -name *.rs").matches("MAIN.RS", 0, None));
        assert!(filters("-name *.rs -iname *.rs").matches("MAIN.RS", 0, None));

        // The glob applies to the whole base name, not a substring of it.
        let readme = filters("-iname README*");
        assert!(readme.matches("readme.md", 0, None));
        assert!(!readme.matches("a-readme.md", 0, None));

        assert!(MetadataFilters::new_at("-name [", now()).is_err());
        assert!(MetadataFilters::new_at("-iname [", now()).is_err());
    }

    #[test]
    fn empty_source_filters_nothing() {
        let parsed = filters("   ");
        assert!(parsed.is_empty());
        assert!(parsed.matches(ANY_NAME, 0, None));
    }

    #[test]
    fn size_defaults_to_kib_and_greater_than() {
        // An unsigned `-size` means "greater than", and its unit is KiB.
        assert_eq!(
            filters("-size 9").size,
            Some(FindComparison::GreaterThan(9 * 1024))
        );
        assert_eq!(
            filters("-size +1").size,
            Some(FindComparison::GreaterThan(1024))
        );
        assert_eq!(
            filters("-size -4").size,
            Some(FindComparison::LessThan(4 * 1024))
        );
    }

    #[test]
    fn parses_size_suffixes() {
        // An explicit suffix overrides the KiB default; the unsigned-means-
        // greater-than rule still applies.
        assert_eq!(
            filters("-size 100c").size,
            Some(FindComparison::GreaterThan(100))
        );
        assert_eq!(
            filters("-size 2b").size,
            Some(FindComparison::GreaterThan(2 * 512))
        );
        assert_eq!(
            filters("-size -4k").size,
            Some(FindComparison::LessThan(4 * 1024))
        );
        assert_eq!(
            filters("-size +3M").size,
            Some(FindComparison::GreaterThan(3 * 1024 * 1024))
        );
        assert_eq!(
            filters("-size +1G").size,
            Some(FindComparison::GreaterThan(1024 * 1024 * 1024))
        );
    }

    #[test]
    fn rejects_malformed_input() {
        for source in [
            "-size",
            "-size abc",
            "-mtime",
            "-mtime 1.5",
            "-mmin +",
            "-atime 3",
            "-name",
            "-iname",
            "size +1k",
        ] {
            assert!(
                MetadataFilters::new_at(source, now()).is_err(),
                "expected `{source}` to be rejected"
            );
        }
    }

    #[test]
    fn matches_size() {
        let larger_than_1k = filters("-size +1k");
        assert!(larger_than_1k.matches(ANY_NAME, 2048, None));
        assert!(!larger_than_1k.matches(ANY_NAME, 1024, None));
        assert!(!larger_than_1k.matches(ANY_NAME, 0, None));

        let smaller_than_1k = filters("-size -1k");
        assert!(smaller_than_1k.matches(ANY_NAME, 1023, None));
        assert!(!smaller_than_1k.matches(ANY_NAME, 1024, None));
    }

    #[test]
    fn matches_age_in_days_and_minutes() {
        let day = SECONDS_PER_DAY;

        let modified_within_a_week = filters("-mtime -7");
        assert!(modified_within_a_week.matches(ANY_NAME, 0, aged(3 * day)));
        assert!(!modified_within_a_week.matches(ANY_NAME, 0, aged(8 * day)));

        let older_than_a_week = filters("-mtime +7");
        assert!(older_than_a_week.matches(ANY_NAME, 0, aged(8 * day)));
        assert!(!older_than_a_week.matches(ANY_NAME, 0, aged(3 * day)));

        // Truncating division, as `find` does: 7 days and change is still "7".
        let exactly_seven_days = filters("-mtime 7");
        assert!(exactly_seven_days.matches(ANY_NAME, 0, aged(7 * day + 60)));
        assert!(!exactly_seven_days.matches(ANY_NAME, 0, aged(8 * day)));

        let modified_in_last_half_hour = filters("-mmin -30");
        assert!(modified_in_last_half_hour.matches(ANY_NAME, 0, aged(60)));
        assert!(!modified_in_last_half_hour.matches(ANY_NAME, 0, aged(60 * 60)));
    }

    #[test]
    fn predicates_are_conjunctive() {
        let both = filters("-size +1k -mmin -30");
        assert!(both.matches(ANY_NAME, 2048, aged(60)));
        assert!(!both.matches(ANY_NAME, 512, aged(60)));
        assert!(!both.matches(ANY_NAME, 2048, aged(60 * 60)));
    }

    #[test]
    fn unreadable_mtime_cannot_satisfy_an_age_filter() {
        assert!(!filters("-mtime -7").matches(ANY_NAME, 0, None));
        assert!(!filters("-mmin +1").matches(ANY_NAME, 0, None));
        // ...but a size-only filter doesn't care about mtime at all.
        assert!(filters("-size +1k").matches(ANY_NAME, 2048, None));
    }

    #[test]
    fn mtime_in_the_future_is_treated_as_age_zero() {
        let future = now() + Duration::from_secs(SECONDS_PER_DAY);
        let since_epoch = future
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mtime = Some(MTime::from_seconds_and_nanos(since_epoch, 0));

        assert!(filters("-mmin -30").matches(ANY_NAME, 0, mtime));
        assert!(!filters("-mmin +30").matches(ANY_NAME, 0, mtime));
    }

    #[test]
    fn source_round_trips_verbatim() {
        let source = "-size +1k -mtime -7";
        assert_eq!(filters(source).source(), source);
        assert_eq!(MetadataFilters::default().source(), "");
    }
}
