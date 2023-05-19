use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::Result;
use client::proto;
use globset::{Glob, GlobMatcher};
use itertools::Itertools;
use language::{char_kind, Rope};
use regex::{Regex, RegexBuilder};
use smol::future::yield_now;
use std::{
    io::{BufRead, BufReader, Read},
    ops::Range,
    path::Path,
    sync::Arc,
};

#[derive(Clone, Debug)]
pub enum SearchQuery {
    Text {
        search: Arc<AhoCorasick<usize>>,
        query: Arc<str>,
        whole_word: bool,
        case_sensitive: bool,
        files_to_include: Vec<GlobMatcher>,
        files_to_exclude: Vec<GlobMatcher>,
    },
    Regex {
        regex: Regex,
        query: Arc<str>,
        multiline: bool,
        whole_word: bool,
        case_sensitive: bool,
        files_to_include: Vec<GlobMatcher>,
        files_to_exclude: Vec<GlobMatcher>,
    },
}

impl SearchQuery {
    pub fn text(
        query: impl ToString,
        whole_word: bool,
        case_sensitive: bool,
        files_to_include: Vec<GlobMatcher>,
        files_to_exclude: Vec<GlobMatcher>,
    ) -> Self {
        let query = query.to_string();
        let search = AhoCorasickBuilder::new()
            .auto_configure(&[&query])
            .ascii_case_insensitive(!case_sensitive)
            .build(&[&query]);
        Self::Text {
            search: Arc::new(search),
            query: Arc::from(query),
            whole_word,
            case_sensitive,
            files_to_include,
            files_to_exclude,
        }
    }

    pub fn regex(
        query: impl ToString,
        whole_word: bool,
        case_sensitive: bool,
        files_to_include: Vec<GlobMatcher>,
        files_to_exclude: Vec<GlobMatcher>,
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

        let multiline = query.contains('\n') || query.contains("\\n");
        let regex = RegexBuilder::new(&query)
            .case_insensitive(!case_sensitive)
            .multi_line(multiline)
            .build()?;
        Ok(Self::Regex {
            regex,
            query: initial_query,
            multiline,
            whole_word,
            case_sensitive,
            files_to_include,
            files_to_exclude,
        })
    }

    pub fn from_proto(message: proto::SearchProject) -> Result<Self> {
        if message.regex {
            Self::regex(
                message.query,
                message.whole_word,
                message.case_sensitive,
                deserialize_globs(&message.files_to_include)?,
                deserialize_globs(&message.files_to_exclude)?,
            )
        } else {
            Ok(Self::text(
                message.query,
                message.whole_word,
                message.case_sensitive,
                deserialize_globs(&message.files_to_include)?,
                deserialize_globs(&message.files_to_exclude)?,
            ))
        }
    }

    pub fn to_proto(&self, project_id: u64) -> proto::SearchProject {
        proto::SearchProject {
            project_id,
            query: self.as_str().to_string(),
            regex: self.is_regex(),
            whole_word: self.whole_word(),
            case_sensitive: self.case_sensitive(),
            files_to_include: self
                .files_to_include()
                .iter()
                .map(|g| g.glob().to_string())
                .join(","),
            files_to_exclude: self
                .files_to_exclude()
                .iter()
                .map(|g| g.glob().to_string())
                .join(","),
        }
    }

    pub fn detect<T: Read>(&self, stream: T) -> Result<bool> {
        if self.as_str().is_empty() {
            return Ok(false);
        }

        match self {
            Self::Text { search, .. } => {
                let mat = search.stream_find_iter(stream).next();
                match mat {
                    Some(Ok(_)) => Ok(true),
                    Some(Err(err)) => Err(err.into()),
                    None => Ok(false),
                }
            }
            Self::Regex {
                regex, multiline, ..
            } => {
                let mut reader = BufReader::new(stream);
                if *multiline {
                    let mut text = String::new();
                    if let Err(err) = reader.read_to_string(&mut text) {
                        Err(err.into())
                    } else {
                        Ok(regex.find(&text).is_some())
                    }
                } else {
                    for line in reader.lines() {
                        let line = line?;
                        if regex.find(&line).is_some() {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
            }
        }
    }

    pub async fn search(&self, rope: &Rope) -> Vec<Range<usize>> {
        const YIELD_INTERVAL: usize = 20000;

        if self.as_str().is_empty() {
            return Default::default();
        }

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
                        let prev_kind = rope.reversed_chars_at(mat.start()).next().map(char_kind);
                        let start_kind = char_kind(rope.chars_at(mat.start()).next().unwrap());
                        let end_kind = char_kind(rope.reversed_chars_at(mat.end()).next().unwrap());
                        let next_kind = rope.chars_at(mat.end()).next().map(char_kind);
                        if Some(start_kind) == prev_kind || Some(end_kind) == next_kind {
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

                        matches.push(mat.start()..mat.end());
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
                                for mat in regex.find_iter(&line) {
                                    let start = line_offset + mat.start();
                                    let end = line_offset + mat.end();
                                    matches.push(start..end);
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

    pub fn as_str(&self) -> &str {
        match self {
            Self::Text { query, .. } => query.as_ref(),
            Self::Regex { query, .. } => query.as_ref(),
        }
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

    pub fn is_regex(&self) -> bool {
        matches!(self, Self::Regex { .. })
    }

    pub fn files_to_include(&self) -> &[GlobMatcher] {
        match self {
            Self::Text {
                files_to_include, ..
            } => files_to_include,
            Self::Regex {
                files_to_include, ..
            } => files_to_include,
        }
    }

    pub fn files_to_exclude(&self) -> &[GlobMatcher] {
        match self {
            Self::Text {
                files_to_exclude, ..
            } => files_to_exclude,
            Self::Regex {
                files_to_exclude, ..
            } => files_to_exclude,
        }
    }

    pub fn file_matches(&self, file_path: Option<&Path>) -> bool {
        match file_path {
            Some(file_path) => {
                !self
                    .files_to_exclude()
                    .iter()
                    .any(|exclude_glob| exclude_glob.is_match(file_path))
                    && (self.files_to_include().is_empty()
                        || self
                            .files_to_include()
                            .iter()
                            .any(|include_glob| include_glob.is_match(file_path)))
            }
            None => self.files_to_include().is_empty(),
        }
    }
}

fn deserialize_globs(glob_set: &str) -> Result<Vec<GlobMatcher>> {
    glob_set
        .split(',')
        .map(str::trim)
        .filter(|glob_str| !glob_str.is_empty())
        .map(|glob_str| Ok(Glob::new(glob_str)?.compile_matcher()))
        .collect()
}
