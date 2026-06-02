use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::Result;
use client::proto;
use fancy_regex::{Captures, Regex, RegexBuilder};
use gpui::Entity;
use itertools::Itertools as _;
use language::{Buffer, BufferSnapshot, CharKind};
use smol::future::yield_now;
use std::{
    borrow::Cow,
    io::{BufRead, BufReader, Read},
    ops::Range,
    path::Path,
    sync::{Arc, LazyLock},
};
use text::Anchor;
use util::{
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};

use crate::ProjectPath;

#[derive(Debug)]
pub enum SearchResult {
    Buffer {
        buffer: Entity<Buffer>,
        ranges: Vec<Range<Anchor>>,
    },
    /// Emitted for files searched without loading into a `Buffer`. The
    /// captured `MatchLocation` entries are bounded by the per-file cap, so
    /// peak memory does not grow with file size.
    DeferredFile(FileMatchSummary),
    LimitReached,
    WaitingForScan,
    Searching,
}

/// `byte_range` is in disk-file UTF-8 byte coordinates so the buffer can
/// resolve it to a `language::Anchor` at hydration time.
#[derive(Debug, Clone)]
pub struct MatchLocation {
    pub byte_range: Range<u64>,
    /// 1-indexed.
    pub line_number: u32,
    pub line_byte_offset: u32,
    pub snippet: Arc<str>,
    pub snippet_match_range: Range<u32>,
}

#[derive(Debug, Clone)]
pub struct FileMatchSummary {
    pub path: ProjectPath,
    pub abs_path: Arc<Path>,
    pub file_size: u64,
    pub matches: Vec<MatchLocation>,
    /// True if more matches existed beyond the per-file cap, or if the
    /// query is multiline (in which case `matches` is empty because the
    /// streaming scan can't enumerate multiline matches).
    pub truncated: bool,
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
    match_full_paths: bool,
    buffers: Option<Vec<Entity<Buffer>>>,
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
        multiline: bool,
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
        let query = query.to_string();
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
                false,
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
        let query = query.to_string();
        let inner = SearchInputs {
            query: Arc::from(query.as_str()),
            files_to_include,
            files_to_exclude,
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

        let multiline = pattern.contains('\n') || pattern.contains("\\n");
        if multiline {
            pattern.insert_str(0, "(?m)");
        }

        let regex = RegexBuilder::new(&pattern)
            .case_insensitive(!case_sensitive)
            .build()?;
        Ok(Self::Regex {
            regex,
            replacement: None,
            multiline,
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

        if message.regex {
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
            )
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
            )
        }
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
            // Populate legacy fields for backwards compatibility
            files_to_include_legacy: files_to_include.join(","),
            files_to_exclude_legacy: files_to_exclude.join(","),
        }
    }

    /// Returns `(matches, truncated)`. `truncated` is set when the cap is
    /// hit or when the query is multiline (in which case `matches` is empty
    /// because multiline can't be enumerated without buffering the file).
    ///
    /// Peak memory is bounded by `max_matches × snippet_budget`, except for
    /// the `BufRead::read_line` line allocation — a file with no newlines
    /// still allocates proportional to line length (see follow-up).
    pub(crate) async fn search_streaming(
        &self,
        mut reader: BufReader<Box<dyn Read + Send + Sync>>,
        max_matches: usize,
    ) -> Result<(Vec<MatchLocation>, bool)> {
        let query_str = self.as_str();
        if query_str.is_empty() || max_matches == 0 {
            return Ok((Vec::new(), false));
        }

        const SNIPPET_BUDGET: usize = 256;
        const YIELD_THRESHOLD: usize = 20 * 1024;

        let is_multiline_pattern = match self {
            Self::Text { .. } => query_str.contains('\n'),
            Self::Regex { multiline, .. } => *multiline,
        };
        if is_multiline_pattern {
            // Surface existence only — multiline needs the whole file.
            return Ok((Vec::new(), true));
        }

        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

        let mut matches: Vec<MatchLocation> = Vec::new();
        let mut byte_offset: u64 = 0;
        let mut line_number: u32 = 0;
        let mut bytes_since_yield: usize = 0;
        let mut text = String::new();

        loop {
            text.clear();
            let read = reader.read_line(&mut text)?;
            if read == 0 {
                break;
            }
            line_number = line_number.saturating_add(1);

            let line_matches: Vec<Range<usize>> = match self {
                Self::Text {
                    search, whole_word, ..
                } => {
                    let mut found = Vec::new();
                    for mat in search.find_iter(text.as_bytes()) {
                        if *whole_word {
                            let prev_char = text[..mat.start()].chars().next_back();
                            let next_char = text[mat.end()..].chars().next();
                            if prev_char.is_some_and(&is_word_char)
                                || next_char.is_some_and(&is_word_char)
                            {
                                continue;
                            }
                        }
                        found.push(mat.start()..mat.end());
                    }
                    found
                }
                Self::Regex {
                    regex,
                    whole_word,
                    one_match_per_line,
                    ..
                } => {
                    let mut found = Vec::new();
                    for mat in regex.find_iter(&text).flatten() {
                        let start = mat.start();
                        let end = mat.end();
                        if *whole_word {
                            let prev_char = text[..start].chars().next_back();
                            let next_char = text[end..].chars().next();
                            if prev_char.is_some_and(&is_word_char)
                                || next_char.is_some_and(&is_word_char)
                            {
                                continue;
                            }
                        }
                        found.push(start..end);
                        if *one_match_per_line {
                            break;
                        }
                    }
                    found
                }
            };

            for range in line_matches {
                let (snippet, snippet_match_range) =
                    extract_snippet(&text, range.clone(), SNIPPET_BUDGET);
                matches.push(MatchLocation {
                    byte_range: (byte_offset + range.start as u64)
                        ..(byte_offset + range.end as u64),
                    line_number,
                    line_byte_offset: range.start as u32,
                    snippet,
                    snippet_match_range,
                });
                if matches.len() >= max_matches {
                    return Ok((matches, true));
                }
            }

            byte_offset = byte_offset.saturating_add(text.len() as u64);
            bytes_since_yield = bytes_since_yield.saturating_add(text.len());
            if bytes_since_yield >= YIELD_THRESHOLD {
                bytes_since_yield = 0;
                yield_now().await;
            }
        }

        Ok((matches, false))
    }

    pub(crate) async fn detect(
        &self,
        mut reader: BufReader<Box<dyn Read + Send + Sync>>,
    ) -> Result<bool> {
        let query_str = self.as_str();
        if query_str.is_empty() {
            return Ok(false);
        }

        // Yield from this function every 20KB scanned.
        const YIELD_THRESHOLD: usize = 20 * 1024;

        match self {
            Self::Text { search, .. } => {
                let mut text = String::new();
                if query_str.contains('\n') {
                    reader.read_to_string(&mut text)?;
                    Ok(search.is_match(&text))
                } else {
                    let mut bytes_read = 0;
                    while reader.read_line(&mut text)? > 0 {
                        if search.is_match(&text) {
                            return Ok(true);
                        }
                        bytes_read += text.len();
                        if bytes_read >= YIELD_THRESHOLD {
                            bytes_read = 0;
                            smol::future::yield_now().await;
                        }
                        text.clear();
                    }
                    Ok(false)
                }
            }
            Self::Regex {
                regex, multiline, ..
            } => {
                let mut text = String::new();
                if *multiline {
                    reader.read_to_string(&mut text)?;
                    Ok(regex.is_match(&text)?)
                } else {
                    let mut bytes_read = 0;
                    while reader.read_line(&mut text)? > 0 {
                        if regex.is_match(&text)? {
                            return Ok(true);
                        }
                        bytes_read += text.len();
                        if bytes_read >= YIELD_THRESHOLD {
                            bytes_read = 0;
                            smol::future::yield_now().await;
                        }
                        text.clear();
                    }
                    Ok(false)
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
                regex, multiline, ..
            } => {
                if *multiline {
                    let text = rope.to_string();
                    for (ix, mat) in regex.find_iter(&text).enumerate() {
                        if (ix + 1) % YIELD_INTERVAL == 0 {
                            yield_now().await;
                        }

                        if let Ok(mat) = mat {
                            matches.push(mat.start()..mat.end());
                        }
                    }
                } else {
                    let mut line = String::new();
                    let mut line_offset = 0;
                    for (chunk_ix, chunk) in rope.chunks().chain(["\n"]).enumerate() {
                        if (chunk_ix + 1) % YIELD_INTERVAL == 0 {
                            yield_now().await;
                        }

                        for (newline_ix, text) in chunk.split('\n').enumerate() {
                            if newline_ix > 0 {
                                for mat in regex.find_iter(&line).flatten() {
                                    let start = line_offset + mat.start();
                                    let end = line_offset + mat.end();
                                    matches.push(start..end);
                                    if self.one_match_per_line() == Some(true) {
                                        break;
                                    }
                                }

                                line_offset += line.len() + 1;
                                line.clear();
                            }
                            line.push_str(text);
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

    /// Whether this search should replace only one match per line, instead of
    /// all matches.
    /// Returns `None` for text searches, as only regex searches support this
    /// option.
    pub fn one_match_per_line(&self) -> Option<bool> {
        match self {
            Self::Regex {
                one_match_per_line, ..
            } => Some(*one_match_per_line),
            Self::Text { .. } => None,
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
            Self::Regex {
                regex,
                multiline,
                one_match_per_line,
                ..
            } => {
                if *multiline {
                    for mat in regex.find_iter(text).flatten() {
                        matches.push(mat.start()..mat.end());
                    }
                } else {
                    let mut line_offset = 0;
                    for line in text.split('\n') {
                        for mat in regex.find_iter(line).flatten() {
                            matches.push((line_offset + mat.start())..(line_offset + mat.end()));
                            if *one_match_per_line {
                                break;
                            }
                        }
                        line_offset += line.len() + 1;
                    }
                }
            }
        }
        matches
    }
}

/// `budget` bounds the prefix + suffix around the match; the match itself
/// is never clipped (so a pathologically long match exceeds `budget`).
fn extract_snippet(
    line: &str,
    match_range: Range<usize>,
    budget: usize,
) -> (Arc<str>, Range<u32>) {
    // Strip "\n" and any preceding "\r" (Windows line endings).
    let line = line.strip_suffix('\n').unwrap_or(line);
    let line = line.strip_suffix('\r').unwrap_or(line);

    let match_start = match_range.start.min(line.len());
    let match_end = match_range.end.min(line.len());

    if line.len() <= budget {
        return (
            Arc::from(line),
            (match_start as u32)..(match_end as u32),
        );
    }

    let half = budget / 2;
    let raw_start = match_start.saturating_sub(half);
    let raw_end = match_end.saturating_add(half).min(line.len());

    // Snap outward to UTF-8 boundaries; never clip into the match.
    let snippet_start = (0..=raw_start)
        .rev()
        .find(|&i| line.is_char_boundary(i))
        .unwrap_or(0);
    let snippet_end = (raw_end..=line.len())
        .find(|&i| line.is_char_boundary(i))
        .unwrap_or(line.len());

    let snippet = &line[snippet_start..snippet_end];
    (
        Arc::from(snippet),
        ((match_start - snippet_start) as u32)..((match_end - snippet_start) as u32),
    )
}
