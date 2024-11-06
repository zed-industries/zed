use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::Result;
use client::proto;
use fancy_regex::Regex as FancyRegex;
use gpui::Model;
use language::{Buffer, BufferSnapshot};
use regex::{Captures, Regex, RegexBuilder};
use smol::future::yield_now;
use std::{
    borrow::Cow,
    io::{BufRead, BufReader, Read},
    ops::Range,
    path::Path,
    sync::{Arc, OnceLock},
};
use text::Anchor;
use util::paths::PathMatcher;

static TEXT_REPLACEMENT_SPECIAL_CHARACTERS_REGEX: OnceLock<Regex> = OnceLock::new();

pub enum SearchResult {
    Buffer {
        buffer: Model<Buffer>,
        ranges: Vec<Range<Anchor>>,
    },
    LimitReached,
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
    buffers: Option<Vec<Model<Buffer>>>,
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
    pub fn buffers(&self) -> &Option<Vec<Model<Buffer>>> {
        &self.buffers
    }
}

#[derive(Clone, Debug)]
enum RegexEngine {
    Regex(Regex),
    FancyRegex(FancyRegex),
}

impl RegexEngine {
    fn detect(&self, text: &str) -> Result<bool> {
        match self {
            Self::Regex(regex) => Ok(regex.find(text).is_some()),
            Self::FancyRegex(fancy_regex) => Ok(fancy_regex.find(text)?.is_some()),
        }
    }
    fn replace<'a>(&self, text: &'a str, replacement: &str) -> Cow<'a, str> {
        match self {
            Self::Regex(regex) => regex.replace(text, replacement),
            Self::FancyRegex(fancy_regex) => fancy_regex.replace(text, replacement),
        }
    }
    async fn find_and_extend_matches(
        &self,
        text: &str,
        offset: usize,
        matches: &mut Vec<Range<usize>>,
        yield_interval: usize,
    ) {
        match self {
            Self::Regex(regex) => {
                for (i, mat) in regex.find_iter(text).enumerate() {
                    if (i + 1) % yield_interval == 0 {
                        yield_now().await;
                    }
                    matches.push(mat.start() + offset..mat.end() + offset)
                }
            }
            Self::FancyRegex(fancy_regex) => {
                for (i, mat) in fancy_regex.find_iter(text).enumerate() {
                    if (i + 1) % yield_interval == 0 {
                        // REVIEW: revisit this yield interval and how it interacts with the outer
                        // line loop, etc...
                        yield_now().await;
                    }
                    if let Ok(mat) = mat {
                        matches.push(mat.start() + offset..mat.end() + offset)
                    } else {
                        // REVIEW: can consider ignoring or percolating up, or logging to see
                        // if this ever actually happens.
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SearchQuery {
    method: SearchQueryMethod,
    replacement: Option<String>,
    pub whole_word: bool,
    pub case_sensitive: bool,
    inner: SearchInputs,
    pub include_ignored: bool,
}

#[derive(Clone, Debug)]
pub enum SearchQueryMethod {
    Text { search: Arc<AhoCorasick> },
    Regex { regex: RegexEngine, multiline: bool },
}

impl SearchQuery {
    pub fn text(
        query: impl ToString,
        whole_word: bool,
        case_sensitive: bool,
        include_ignored: bool,
        files_to_include: PathMatcher,
        files_to_exclude: PathMatcher,
        buffers: Option<Vec<Model<Buffer>>>,
    ) -> Result<Self> {
        let query = query.to_string();
        let search = AhoCorasickBuilder::new()
            .ascii_case_insensitive(!case_sensitive)
            .build([&query])?;
        let inner = SearchInputs {
            query: query.into(),
            files_to_exclude,
            files_to_include,
            buffers,
        };
        Ok(Self {
            method: SearchQueryMethod::Text {
                search: Arc::new(search),
            },
            replacement: None,
            whole_word,
            case_sensitive,
            include_ignored,
            inner,
        })
    }

    pub fn regex(
        query: impl ToString,
        whole_word: bool,
        case_sensitive: bool,
        include_ignored: bool,
        files_to_include: PathMatcher,
        files_to_exclude: PathMatcher,
        buffers: Option<Vec<Model<Buffer>>>,
    ) -> Result<Self> {
        let mut query = query.to_string();
        let initial_query = Arc::from(query.as_str());
        if whole_word {
            let mut word_query = String::new();
            word_query.push_str("\\b");
            word_query.push_str(&query);
            word_query.push_str("\\b");
            query = word_query
        }

        let multiline = query.contains('\n') || query.contains("\\n") || query.contains("\\s");
        let regex = RegexBuilder::new(&query)
            .case_insensitive(!case_sensitive)
            .multi_line(multiline)
            .build()
            .map_err(|e| {
                // REVIEW: Found by using: (?<!user|tenant)RecordId = randomUUID\(\);
                log::error!("Failed to build regex: {}", e);
                e
            })?;
        let inner = SearchInputs {
            query: initial_query,
            files_to_exclude,
            files_to_include,
            buffers,
        };
        Ok(Self {
            method: SearchQueryMethod::Regex {
                regex: RegexEngine::Regex(regex),
                multiline,
            },
            replacement: None,
            whole_word,
            case_sensitive,
            include_ignored,
            inner,
        })
    }

    pub fn from_proto(message: proto::SearchQuery) -> Result<Self> {
        if message.regex {
            Self::regex(
                message.query,
                message.whole_word,
                message.case_sensitive,
                message.include_ignored,
                deserialize_path_matches(&message.files_to_include)?,
                deserialize_path_matches(&message.files_to_exclude)?,
                None, // search opened only don't need search remote
            )
        } else {
            Self::text(
                message.query,
                message.whole_word,
                message.case_sensitive,
                message.include_ignored,
                deserialize_path_matches(&message.files_to_include)?,
                deserialize_path_matches(&message.files_to_exclude)?,
                None, // search opened only don't need search remote
            )
        }
    }

    pub fn with_replacement(mut self, new_replacement: String) -> Self {
        self.replacement = Some(new_replacement);
        self
    }

    pub fn to_proto(&self) -> proto::SearchQuery {
        proto::SearchQuery {
            query: self.as_str().to_string(),
            regex: self.is_regex(),
            whole_word: self.whole_word,
            case_sensitive: self.case_sensitive,
            include_ignored: self.include_ignored,
            files_to_include: self.files_to_include().sources().join(","),
            files_to_exclude: self.files_to_exclude().sources().join(","),
        }
    }

    pub fn detect<T: Read>(&self, stream: T) -> Result<bool> {
        if self.as_str().is_empty() {
            return Ok(false);
        }

        match &self.method {
            SearchQueryMethod::Text { search, .. } => {
                let mat = search.stream_find_iter(stream).next();
                match mat {
                    Some(Ok(_)) => Ok(true),
                    Some(Err(err)) => Err(err.into()),
                    None => Ok(false),
                }
            }
            SearchQueryMethod::Regex {
                regex, multiline, ..
            } => {
                let mut reader = BufReader::new(stream);
                if *multiline {
                    let mut text = String::new();
                    if let Err(err) = reader.read_to_string(&mut text) {
                        Err(err.into())
                    } else {
                        regex.detect(&text)
                    }
                } else {
                    for line in reader.lines() {
                        let line = line?;
                        if regex.detect(&line)? {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
            }
        }
    }
    /// Returns the replacement text for this `SearchQuery`.
    pub fn replacement(&self) -> Option<&str> {
        self.replacement.as_deref()
    }
    /// Replaces search hits if replacement is set. `text` is assumed to be a string that matches this `SearchQuery` exactly, without any leftovers on either side.
    pub fn replacement_for<'a>(&self, text: &'a str) -> Option<Cow<'a, str>> {
        match &self.method {
            SearchQueryMethod::Text { .. } => self.replacement.clone().map(Cow::from),
            SearchQueryMethod::Regex { regex, .. } => {
                if let Some(ref replacement) = self.replacement {
                    let replacement = TEXT_REPLACEMENT_SPECIAL_CHARACTERS_REGEX
                        .get_or_init(|| Regex::new(r"\\\\|\\n|\\t").unwrap())
                        .replace_all(replacement, |c: &Captures| {
                            match c.get(0).unwrap().as_str() {
                                r"\\" => "\\",
                                r"\n" => "\n",
                                r"\t" => "\t",
                                x => unreachable!("Unexpected escape sequence: {}", x),
                            }
                        });
                    Some(regex.replace(text, &replacement))
                } else {
                    None
                }
            }
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

        let mut matches: Vec<Range<usize>> = Vec::new();
        match &self.method {
            SearchQueryMethod::Text { search, .. } => {
                for (ix, mat) in search
                    .stream_find_iter(rope.bytes_in_range(0..rope.len()))
                    .enumerate()
                {
                    if (ix + 1) % YIELD_INTERVAL == 0 {
                        yield_now().await;
                    }

                    let mat = mat.unwrap();
                    if self.whole_word {
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
                        if Some(start_kind) == prev_kind || Some(end_kind) == next_kind {
                            continue;
                        }
                    }
                    matches.push(mat.start()..mat.end())
                }
            }

            SearchQueryMethod::Regex {
                regex, multiline, ..
            } => {
                if *multiline {
                    let text = rope.to_string();
                    regex
                        .find_and_extend_matches(&text, 0, &mut matches, YIELD_INTERVAL)
                        .await;
                } else {
                    let mut line = String::new();
                    let mut line_offset = 0;
                    for (chunk_ix, chunk) in rope.chunks().chain(["\n"]).enumerate() {
                        if (chunk_ix + 1) % YIELD_INTERVAL == 0 {
                            yield_now().await;
                        }

                        for (newline_ix, text) in chunk.split('\n').enumerate() {
                            if newline_ix > 0 {
                                regex
                                    .find_and_extend_matches(
                                        &line,
                                        line_offset,
                                        &mut matches,
                                        YIELD_INTERVAL,
                                    )
                                    .await;
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

    pub fn is_regex(&self) -> bool {
        matches!(self.method, SearchQueryMethod::Regex { .. })
    }

    pub fn files_to_include(&self) -> &PathMatcher {
        self.as_inner().files_to_include()
    }

    pub fn files_to_exclude(&self) -> &PathMatcher {
        self.as_inner().files_to_exclude()
    }

    pub fn buffers(&self) -> Option<&Vec<Model<Buffer>>> {
        self.as_inner().buffers.as_ref()
    }

    pub fn is_opened_only(&self) -> bool {
        self.as_inner().buffers.is_some()
    }

    pub fn filters_path(&self) -> bool {
        !(self.files_to_exclude().sources().is_empty()
            && self.files_to_include().sources().is_empty())
    }

    pub fn file_matches(&self, file_path: &Path) -> bool {
        let mut path = file_path.to_path_buf();
        loop {
            if self.files_to_exclude().is_match(&path) {
                return false;
            } else if self.files_to_include().sources().is_empty()
                || self.files_to_include().is_match(&path)
            {
                return true;
            } else if !path.pop() {
                return false;
            }
        }
    }
    pub fn as_inner(&self) -> &SearchInputs {
        &self.inner
    }
}

pub fn deserialize_path_matches(glob_set: &str) -> anyhow::Result<PathMatcher> {
    let globs = glob_set
        .split(',')
        .map(str::trim)
        .filter(|&glob_str| (!glob_str.is_empty()))
        .map(|glob_str| glob_str.to_owned())
        .collect::<Vec<_>>();
    Ok(PathMatcher::new(&globs)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_matcher_creation_for_valid_paths() {
        for valid_path in [
            "file",
            "Cargo.toml",
            ".DS_Store",
            "~/dir/another_dir/",
            "./dir/file",
            "dir/[a-z].txt",
            "../dir/filé",
        ] {
            let path_matcher = PathMatcher::new(&[valid_path.to_owned()]).unwrap_or_else(|e| {
                panic!("Valid path {valid_path} should be accepted, but got: {e}")
            });
            assert!(
                path_matcher.is_match(valid_path),
                "Path matcher for valid path {valid_path} should match itself"
            )
        }
    }

    #[test]
    fn path_matcher_creation_for_globs() {
        for invalid_glob in ["dir/[].txt", "dir/[a-z.txt", "dir/{file"] {
            match PathMatcher::new(&[invalid_glob.to_owned()]) {
                Ok(_) => panic!("Invalid glob {invalid_glob} should not be accepted"),
                Err(_expected) => {}
            }
        }

        for valid_glob in [
            "dir/?ile",
            "dir/*.txt",
            "dir/**/file",
            "dir/[a-z].txt",
            "{dir,file}",
        ] {
            match PathMatcher::new(&[valid_glob.to_owned()]) {
                Ok(_expected) => {}
                Err(e) => panic!("Valid glob should be accepted, but got: {e}"),
            }
        }
    }
}
