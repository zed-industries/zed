use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use anyhow::{Context, Result};
use client::proto;
use collections::HashMap;
use fs::Fs;
use futures::StreamExt;
use gpui::{AppContext, BackgroundExecutor, Model, Task};
use language::{char_kind, proto::serialize_anchor, Buffer, BufferSnapshot};
use regex::{Captures, Regex, RegexBuilder};
use smol::{
    channel::{Receiver, Sender},
    future::yield_now,
    lock::Semaphore,
};
use std::{
    borrow::Cow,
    cmp::{self, Ordering},
    collections::VecDeque,
    io::{BufRead, BufReader, Read},
    ops::Range,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use text::Anchor;
use util::{
    paths::{compare_paths, PathMatcher},
    ResultExt,
};
use worktree::{Entry, Snapshot, WorktreeId, WorktreeSettings};

use crate::{buffer_store::BufferStore, worktree_store::WorktreeStore, Item, ProjectPath};

static TEXT_REPLACEMENT_SPECIAL_CHARACTERS_REGEX: OnceLock<Regex> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct SearchInputs {
    query: Arc<str>,
    files_to_include: PathMatcher,
    files_to_exclude: PathMatcher,
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
}
#[derive(Clone, Debug)]
pub enum SearchQuery {
    Text {
        search: Arc<AhoCorasick>,
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
        inner: SearchInputs,
    },
}

impl SearchQuery {
    pub fn text(
        query: impl ToString,
        whole_word: bool,
        case_sensitive: bool,
        include_ignored: bool,
        files_to_include: PathMatcher,
        files_to_exclude: PathMatcher,
    ) -> Result<Self> {
        let query = query.to_string();
        let search = AhoCorasickBuilder::new()
            .ascii_case_insensitive(!case_sensitive)
            .build(&[&query])?;
        let inner = SearchInputs {
            query: query.into(),
            files_to_exclude,
            files_to_include,
        };
        Ok(Self::Text {
            search: Arc::new(search),
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
        let inner = SearchInputs {
            query: initial_query,
            files_to_exclude,
            files_to_include,
        };
        Ok(Self::Regex {
            regex,
            replacement: None,
            multiline,
            whole_word,
            case_sensitive,
            include_ignored,
            inner,
        })
    }

    pub fn from_proto(message: proto::SearchProject) -> Result<Self> {
        if message.regex {
            Self::regex(
                message.query,
                message.whole_word,
                message.case_sensitive,
                message.include_ignored,
                deserialize_path_matches(&message.files_to_include)?,
                deserialize_path_matches(&message.files_to_exclude)?,
            )
        } else {
            Self::text(
                message.query,
                message.whole_word,
                message.case_sensitive,
                message.include_ignored,
                deserialize_path_matches(&message.files_to_include)?,
                deserialize_path_matches(&message.files_to_exclude)?,
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
    pub fn to_proto(&self, project_id: u64) -> proto::SearchProject {
        proto::SearchProject {
            project_id,
            query: self.as_str().to_string(),
            regex: self.is_regex(),
            whole_word: self.whole_word(),
            case_sensitive: self.case_sensitive(),
            include_ignored: self.include_ignored(),
            files_to_include: self.files_to_include().sources().join(","),
            files_to_exclude: self.files_to_exclude().sources().join(","),
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
            SearchQuery::Text { replacement, .. } => replacement.clone().map(Cow::from),
            SearchQuery::Regex {
                regex, replacement, ..
            } => {
                if let Some(replacement) = replacement {
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
                    Some(regex.replace(text, replacement))
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
                        let scope = buffer.language_scope_at(range_offset + mat.start());
                        let kind = |c| char_kind(&scope, c);

                        let prev_kind = rope.reversed_chars_at(mat.start()).next().map(kind);
                        let start_kind = kind(rope.chars_at(mat.start()).next().unwrap());
                        let end_kind = kind(rope.reversed_chars_at(mat.end()).next().unwrap());
                        let next_kind = rope.chars_at(mat.end()).next().map(kind);
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

    pub fn file_matches(&self, file_path: Option<&Path>) -> bool {
        match file_path {
            Some(file_path) => {
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
            None => self.files_to_include().sources().is_empty(),
        }
    }
    pub fn as_inner(&self) -> &SearchInputs {
        match self {
            Self::Regex { inner, .. } | Self::Text { inner, .. } => inner,
        }
    }
}

fn deserialize_path_matches(glob_set: &str) -> anyhow::Result<PathMatcher> {
    let globs = glob_set
        .split(',')
        .map(str::trim)
        .filter_map(|glob_str| (!glob_str.is_empty()).then(|| glob_str.to_owned()))
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

#[derive(Clone, Debug, PartialEq)]
enum SearchMatchCandidate {
    OpenBuffer {
        buffer: Model<Buffer>,
        // This might be an unnamed file without representation on filesystem
        path: Option<Arc<Path>>,
    },
    Path {
        worktree_id: WorktreeId,
        path: Arc<Path>,
    },
}

pub enum SearchResult {
    Buffer {
        buffer: Model<Buffer>,
        ranges: Vec<Range<Anchor>>,
    },
    LimitReached,
}

impl SearchResult {
    pub fn serialize_range(range: &Range<Anchor>) -> Range<proto::Anchor> {
        serialize_anchor(&range.start)..serialize_anchor(&range.end)
    }
}

pub fn search(
    query: SearchQuery,
    buffer_store: Model<BufferStore>,
    worktree_store: Model<WorktreeStore>,
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
) -> Receiver<SearchResult> {
    let include_root = worktree_store
        .read(cx)
        .visible_worktrees(cx)
        .collect::<Vec<_>>()
        .len()
        > 1;

    let mut unnamed_buffers = vec![];
    let opened_buffers: HashMap<_> = buffer_store.update(cx, |buffer_store, cx| {
        buffer_store
            .buffers()
            .filter_map(|buffer| {
                let (is_ignored, snapshot) = buffer.update(cx, |buffer, cx| {
                    let worktree_store = worktree_store.read(cx);
                    let is_ignored = buffer
                        .entry_id(cx)
                        .and_then(|entry_id| worktree_store.entry_for_id(entry_id, cx))
                        .map_or(false, |entry| entry.is_ignored);
                    (is_ignored, buffer.snapshot())
                });
                if is_ignored && !query.include_ignored() {
                    return None;
                } else if let Some(file) = snapshot.file() {
                    let matched_path = if include_root {
                        query.file_matches(Some(&file.full_path(cx)))
                    } else {
                        query.file_matches(Some(file.path()))
                    };

                    if matched_path {
                        Some((file.path().clone(), (buffer, snapshot)))
                    } else {
                        None
                    }
                } else {
                    unnamed_buffers.push(buffer);
                    None
                }
            })
            .collect()
    });

    let (matching_paths_tx, matching_paths_rx) = smol::channel::bounded(1024);

    cx.spawn(|cx| async move {
        for buffer in unnamed_buffers {
            matching_paths_tx
                .send(SearchMatchCandidate::OpenBuffer {
                    buffer: buffer.clone(),
                    path: None,
                })
                .await
                .log_err();
        }
        for (path, (buffer, _)) in opened_buffers.iter() {
            matching_paths_tx
                .send(SearchMatchCandidate::OpenBuffer {
                    buffer: buffer.clone(),
                    path: Some(path.clone()),
                })
                .await
                .log_err();
        }

        find_match_candidates(
            unnamed_buffers,
            opened_buffers,
            cx.background_executor().clone(),
            fs,
            query.clone(),
            include_root,
            matching_paths_tx,
        )
        .await
    })
    .detach();

    let (result_tx, result_rx) = smol::channel::bounded(1024);
    let query = Arc::new(query);

    cx.spawn(|mut cx| async move {
        const MAX_SEARCH_RESULT_FILES: usize = 5_000;
        const MAX_SEARCH_RESULT_RANGES: usize = 10_000;

        let mut matching_paths = matching_paths_rx
            .take(MAX_SEARCH_RESULT_FILES + 1)
            .collect::<Vec<_>>()
            .await;
        let mut limit_reached = if matching_paths.len() > MAX_SEARCH_RESULT_FILES {
            matching_paths.pop();
            true
        } else {
            false
        };
        cx.update(|cx| {
            sort_search_matches(&mut matching_paths, cx);
        })?;

        let mut range_count = 0;

        // Now that we know what paths match the query, we will load at most
        // 64 buffers at a time to avoid overwhelming the main thread. For each
        // opened buffer, we will spawn a background task that retrieves all the
        // ranges in the buffer matched by the query.
        'outer: for matching_paths_chunk in matching_paths.chunks(64) {
            let mut chunk_results = Vec::new();
            for matching_path in matching_paths_chunk {
                let query = query.clone();
                let buffer = match matching_path {
                    SearchMatchCandidate::OpenBuffer { buffer, .. } => {
                        Task::ready(Ok(buffer.clone()))
                    }
                    SearchMatchCandidate::Path {
                        worktree_id, path, ..
                    } => buffer_store.update(&mut cx, |buffer_store, cx| {
                        buffer_store.open_buffer(
                            ProjectPath {
                                worktree_id: *worktree_id,
                                path: path.clone(),
                            },
                            cx,
                        )
                    })?,
                };

                chunk_results.push(cx.spawn(|cx| async move {
                    let buffer = buffer.await?;
                    let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot())?;
                    let ranges = cx
                        .background_executor()
                        .spawn(async move {
                            query
                                .search(&snapshot, None)
                                .await
                                .iter()
                                .map(|range| {
                                    snapshot.anchor_before(range.start)
                                        ..snapshot.anchor_after(range.end)
                                })
                                .collect::<Vec<_>>()
                        })
                        .await;
                    anyhow::Ok((buffer, ranges))
                }));
            }

            let chunk_results = futures::future::join_all(chunk_results).await;
            for result in chunk_results {
                if let Some((buffer, ranges)) = result.log_err() {
                    range_count += ranges.len();
                    result_tx
                        .send(SearchResult::Buffer { buffer, ranges })
                        .await?;
                    if range_count > MAX_SEARCH_RESULT_RANGES {
                        limit_reached = true;
                        break 'outer;
                    }
                }
            }
        }

        if limit_reached {
            result_tx.send(SearchResult::LimitReached).await?;
        }

        anyhow::Ok(())
    })
    .detach();

    result_rx
}

/// Pick paths that might potentially contain a match of a given search query.
#[allow(clippy::too_many_arguments)]
async fn find_match_candidates(
    worktree_store: Model<WorktreeStore>,
    opened_buffers: HashMap<Arc<Path>, (Model<Buffer>, BufferSnapshot)>,
    executor: BackgroundExecutor,
    fs: Arc<dyn Fs>,
    query: SearchQuery,
    snapshots: Vec<(Snapshot, WorktreeSettings)>,
    matching_paths_tx: Sender<SearchMatchCandidate>,
) {
    let fs = &fs;
    let query = &query;
    let matching_paths_tx = &matching_paths_tx;
    let snapshots = &snapshots;

    let snapshots = worktree_store
        .read(cx)
        .visible_worktrees(cx)
        .filter_map(|tree| {
            let tree = tree.read(cx);
            Some((tree.snapshot(), tree.as_local()?.settings()))
        })
        .collect::<Vec<_>>();
    let include_root = snapshots.len() > 1;
    let mut path_count: usize = snapshots
        .iter()
        .map(|(snapshot, _)| {
            if query.include_ignored() {
                snapshot.file_count()
            } else {
                snapshot.visible_file_count()
            }
        })
        .sum();
    path_count = path_count.max(1);
    if path_count == 0 {
        return;
    }
    let workers = cx.background_executor().num_cpus().min(path_count);

    let paths_per_worker = (path_count + workers - 1) / workers;

    executor
        .scoped(|scope| {
            let max_concurrent_workers = Arc::new(Semaphore::new(workers));

            for worker_ix in 0..workers {
                let worker_start_ix = worker_ix * paths_per_worker;
                let worker_end_ix = worker_start_ix + paths_per_worker;
                let opened_buffers = opened_buffers.clone();
                let limiter = Arc::clone(&max_concurrent_workers);
                scope.spawn({
                    async move {
                        let _guard = limiter.acquire().await;
                        search_snapshots(
                            snapshots,
                            worker_start_ix,
                            worker_end_ix,
                            query,
                            matching_paths_tx,
                            &opened_buffers,
                            include_root,
                            fs,
                        )
                        .await;
                    }
                });
            }

            if query.include_ignored() {
                for (snapshot, settings) in snapshots {
                    for ignored_entry in snapshot.entries(true, 0).filter(|e| e.is_ignored) {
                        let limiter = Arc::clone(&max_concurrent_workers);
                        scope.spawn(async move {
                            let _guard = limiter.acquire().await;
                            search_ignored_entry(
                                snapshot,
                                settings,
                                ignored_entry,
                                fs,
                                query,
                                matching_paths_tx,
                            )
                            .await;
                        });
                    }
                }
            }
        })
        .await;
}

async fn search_ignored_entry(
    snapshot: &Snapshot,
    settings: &WorktreeSettings,
    ignored_entry: &Entry,
    fs: &Arc<dyn Fs>,
    query: &SearchQuery,
    counter_tx: &Sender<SearchMatchCandidate>,
) {
    let mut ignored_paths_to_process =
        VecDeque::from([snapshot.abs_path().join(&ignored_entry.path)]);

    while let Some(ignored_abs_path) = ignored_paths_to_process.pop_front() {
        let metadata = fs
            .metadata(&ignored_abs_path)
            .await
            .with_context(|| format!("fetching fs metadata for {ignored_abs_path:?}"))
            .log_err()
            .flatten();

        if let Some(fs_metadata) = metadata {
            if fs_metadata.is_dir {
                let files = fs
                    .read_dir(&ignored_abs_path)
                    .await
                    .with_context(|| format!("listing ignored path {ignored_abs_path:?}"))
                    .log_err();

                if let Some(mut subfiles) = files {
                    while let Some(subfile) = subfiles.next().await {
                        if let Some(subfile) = subfile.log_err() {
                            ignored_paths_to_process.push_back(subfile);
                        }
                    }
                }
            } else if !fs_metadata.is_symlink {
                if !query.file_matches(Some(&ignored_abs_path))
                    || settings.is_path_excluded(&ignored_entry.path)
                {
                    continue;
                }
                let matches = if let Some(file) = fs
                    .open_sync(&ignored_abs_path)
                    .await
                    .with_context(|| format!("Opening ignored path {ignored_abs_path:?}"))
                    .log_err()
                {
                    query.detect(file).unwrap_or(false)
                } else {
                    false
                };

                if matches {
                    let project_path = SearchMatchCandidate::Path {
                        worktree_id: snapshot.id(),
                        path: Arc::from(
                            ignored_abs_path
                                .strip_prefix(snapshot.abs_path())
                                .expect("scanning worktree-related files"),
                        ),
                    };
                    if counter_tx.send(project_path).await.is_err() {
                        return;
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn search_snapshots(
    snapshots: &Vec<(Snapshot, WorktreeSettings)>,
    worker_start_ix: usize,
    worker_end_ix: usize,
    query: &SearchQuery,
    results_tx: &Sender<SearchMatchCandidate>,
    opened_buffers: &HashMap<Arc<Path>, (Model<Buffer>, BufferSnapshot)>,
    include_root: bool,
    fs: &Arc<dyn Fs>,
) {
    let mut snapshot_start_ix = 0;
    let mut abs_path = PathBuf::new();

    for (snapshot, _) in snapshots {
        let snapshot_end_ix = snapshot_start_ix
            + if query.include_ignored() {
                snapshot.file_count()
            } else {
                snapshot.visible_file_count()
            };
        if worker_end_ix <= snapshot_start_ix {
            break;
        } else if worker_start_ix > snapshot_end_ix {
            snapshot_start_ix = snapshot_end_ix;
            continue;
        } else {
            let start_in_snapshot = worker_start_ix.saturating_sub(snapshot_start_ix);
            let end_in_snapshot = cmp::min(worker_end_ix, snapshot_end_ix) - snapshot_start_ix;

            for entry in snapshot
                .files(false, start_in_snapshot)
                .take(end_in_snapshot - start_in_snapshot)
            {
                if results_tx.is_closed() {
                    break;
                }
                if opened_buffers.contains_key(&entry.path) {
                    continue;
                }

                let matched_path = if include_root {
                    let mut full_path = PathBuf::from(snapshot.root_name());
                    full_path.push(&entry.path);
                    query.file_matches(Some(&full_path))
                } else {
                    query.file_matches(Some(&entry.path))
                };

                let matches = if matched_path {
                    abs_path.clear();
                    abs_path.push(&snapshot.abs_path());
                    abs_path.push(&entry.path);
                    if let Some(file) = fs.open_sync(&abs_path).await.log_err() {
                        query.detect(file).unwrap_or(false)
                    } else {
                        false
                    }
                } else {
                    false
                };

                if matches {
                    let project_path = SearchMatchCandidate::Path {
                        worktree_id: snapshot.id(),
                        path: entry.path.clone(),
                    };
                    if results_tx.send(project_path).await.is_err() {
                        return;
                    }
                }
            }

            snapshot_start_ix = snapshot_end_ix;
        }
    }
}

fn sort_search_matches(search_matches: &mut Vec<SearchMatchCandidate>, cx: &AppContext) {
    search_matches.sort_by(|entry_a, entry_b| match (entry_a, entry_b) {
        (
            SearchMatchCandidate::OpenBuffer {
                buffer: buffer_a,
                path: None,
            },
            SearchMatchCandidate::OpenBuffer {
                buffer: buffer_b,
                path: None,
            },
        ) => buffer_a
            .read(cx)
            .remote_id()
            .cmp(&buffer_b.read(cx).remote_id()),
        (
            SearchMatchCandidate::OpenBuffer { path: None, .. },
            SearchMatchCandidate::Path { .. }
            | SearchMatchCandidate::OpenBuffer { path: Some(_), .. },
        ) => Ordering::Less,
        (
            SearchMatchCandidate::OpenBuffer { path: Some(_), .. }
            | SearchMatchCandidate::Path { .. },
            SearchMatchCandidate::OpenBuffer { path: None, .. },
        ) => Ordering::Greater,
        (
            SearchMatchCandidate::OpenBuffer {
                path: Some(path_a), ..
            },
            SearchMatchCandidate::Path { path: path_b, .. },
        ) => compare_paths((path_a.as_ref(), true), (path_b.as_ref(), true)),
        (
            SearchMatchCandidate::Path { path: path_a, .. },
            SearchMatchCandidate::OpenBuffer {
                path: Some(path_b), ..
            },
        ) => compare_paths((path_a.as_ref(), true), (path_b.as_ref(), true)),
        (
            SearchMatchCandidate::OpenBuffer {
                path: Some(path_a), ..
            },
            SearchMatchCandidate::OpenBuffer {
                path: Some(path_b), ..
            },
        ) => compare_paths((path_a.as_ref(), true), (path_b.as_ref(), true)),
        (
            SearchMatchCandidate::Path {
                worktree_id: worktree_id_a,
                path: path_a,
                ..
            },
            SearchMatchCandidate::Path {
                worktree_id: worktree_id_b,
                path: path_b,
                ..
            },
        ) => worktree_id_a
            .cmp(&worktree_id_b)
            .then_with(|| compare_paths((path_a.as_ref(), true), (path_b.as_ref(), true))),
    });
}
