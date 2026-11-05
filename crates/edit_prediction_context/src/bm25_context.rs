use crate::editable_context::EditHistoryContextEntry;
use anyhow::{Context as _, Result, bail};
use gpui::{AppContext as _, AsyncApp, Entity};
use language::{Buffer, Point, ToPoint as _};
use project::Project;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
    ops::Range,
    path::{Path, PathBuf},
    time::Instant,
};
use text::Anchor;
use util::command::new_command;

const BM25_CONTEXT_QUERY_LINE_COUNT: u32 = 20;
const BM25_CONTEXT_EDIT_HISTORY_QUERY_ENTRY_COUNT: usize = 8;
const BM25_CONTEXT_CHUNK_LINE_COUNT: usize = 40;
const BM25_CONTEXT_CHUNK_OVERLAP_LINE_COUNT: usize = 10;
const BM25_CONTEXT_CHUNK_COUNT: usize = 12;
const BM25_CONTEXT_MAX_CHUNKS_PER_FILE: usize = 3;
const BM25_CONTEXT_MAX_FILE_BYTES: u64 = 1_000_000;
const BM25_K1: f64 = 1.2;
const BM25_B: f64 = 0.75;

pub(super) struct Bm25ContextCandidate {
    pub path: PathBuf,
    pub row_range: Range<u32>,
    pub order: usize,
}

pub async fn collect_bm25_context(
    project: Entity<Project>,
    active_buffer: Entity<Buffer>,
    cursor_position: Anchor,
    edit_history: &[EditHistoryContextEntry],
    next_order: usize,
    cx: &mut AsyncApp,
) -> Vec<Bm25ContextCandidate> {
    let Some(query) = build_query(&project, &active_buffer, cursor_position, edit_history, cx)
    else {
        return Vec::new();
    };

    let result = cx
        .background_spawn(async move { collect_bm25_context_from_disk(query, next_order).await })
        .await;

    match result {
        Ok(context) => context,
        Err(error) => {
            log::debug!("failed to collect BM25 context: {error:#}");
            Vec::new()
        }
    }
}

struct Bm25ContextQuery {
    worktree_abs_path: PathBuf,
    worktree_root_name: String,
    active_path: String,
    cursor_excerpt: String,
    edit_history_excerpts: Vec<String>,
}

fn build_query(
    project: &Entity<Project>,
    active_buffer: &Entity<Buffer>,
    cursor_position: Anchor,
    edit_history: &[EditHistoryContextEntry],
    cx: &mut AsyncApp,
) -> Option<Bm25ContextQuery> {
    let (worktree_abs_path, worktree_root_name, active_path, cursor_excerpt) = cx.update(|cx| {
        let buffer = active_buffer.read(cx);
        let file = buffer.file()?;
        let project = project.read(cx);
        if !project.is_local() {
            return None;
        }
        let worktree = project.worktree_for_id(file.worktree_id(cx), cx)?;
        let worktree = worktree.read(cx);
        if !worktree.is_local() {
            return None;
        }

        let snapshot = buffer.snapshot();
        let range = expanded_anchor_range(&snapshot, cursor_position..cursor_position);
        let cursor_excerpt = snapshot.text_for_range(range).collect::<String>();

        Some((
            worktree.abs_path(),
            worktree.root_name().as_unix_str().to_string(),
            file.path().as_unix_str().to_string(),
            cursor_excerpt,
        ))
    })?;

    let edit_history_excerpts = edit_history
        .iter()
        .take(BM25_CONTEXT_EDIT_HISTORY_QUERY_ENTRY_COUNT)
        .map(|entry| {
            entry.buffer.read_with(cx, |buffer, _cx| {
                let snapshot = buffer.snapshot();
                let range = expanded_anchor_range(&snapshot, entry.edited_range.clone());
                snapshot.text_for_range(range).collect::<String>()
            })
        })
        .collect();

    Some(Bm25ContextQuery {
        worktree_abs_path: worktree_abs_path.to_path_buf(),
        worktree_root_name,
        active_path,
        cursor_excerpt,
        edit_history_excerpts,
    })
}

fn expanded_anchor_range(
    snapshot: &language::BufferSnapshot,
    range: Range<Anchor>,
) -> Range<Anchor> {
    let start = range.start.to_point(snapshot);
    let end = range.end.to_point(snapshot);
    let start_row = start.row.saturating_sub(BM25_CONTEXT_QUERY_LINE_COUNT);
    let end_row = end
        .row
        .saturating_add(BM25_CONTEXT_QUERY_LINE_COUNT)
        .min(snapshot.max_point().row);
    let start = snapshot.anchor_before(Point::new(start_row, 0));
    let end = snapshot.anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
    start..end
}

async fn collect_bm25_context_from_disk(
    query: Bm25ContextQuery,
    next_order: usize,
) -> Result<Vec<Bm25ContextCandidate>> {
    let query_terms = query_terms(&query);
    if query_terms.is_empty() {
        return Ok(Vec::new());
    }

    let started_at = Instant::now();
    let index = Bm25Index::build(&query.worktree_abs_path).await?;
    let elapsed = started_at.elapsed();
    log::debug!(
        "built BM25 context index: candidate_files:{}, indexed_files:{}, indexed_bytes:{}, chunks:{}, terms:{}, latency:{elapsed:?}",
        index.stats.candidate_file_count,
        index.stats.indexed_file_count,
        index.stats.indexed_bytes,
        index.stats.document_count,
        index.stats.term_count,
    );

    let candidates = index.search(&query_terms, &query.worktree_root_name, next_order);
    log::debug!("selected {} BM25 context chunks", candidates.len());
    Ok(candidates)
}

fn query_terms(query: &Bm25ContextQuery) -> HashMap<String, f64> {
    let mut terms = HashMap::new();
    add_query_terms(&mut terms, &query.active_path, 3.0);
    add_query_terms(&mut terms, &query.cursor_excerpt, 1.0);
    for excerpt in &query.edit_history_excerpts {
        add_query_terms(&mut terms, excerpt, 2.0);
    }
    terms
}

fn add_query_terms(terms: &mut HashMap<String, f64>, text: &str, weight: f64) {
    for token in tokenize(text) {
        *terms.entry(token).or_default() += weight;
    }
}

struct Bm25Index {
    documents: Vec<Document>,
    document_frequencies: HashMap<String, usize>,
    average_document_len: f64,
    stats: Bm25IndexStats,
}

#[derive(Default)]
struct Bm25IndexStats {
    candidate_file_count: usize,
    indexed_file_count: usize,
    indexed_bytes: u64,
    document_count: usize,
    term_count: usize,
}

struct Document {
    relative_path: PathBuf,
    row_range: Range<u32>,
    term_frequencies: HashMap<String, usize>,
    len: usize,
}

struct ScoredDocument {
    document_index: usize,
    score: f64,
}

struct DocumentsForFile {
    documents: Vec<Document>,
    byte_len: u64,
}

impl Bm25Index {
    async fn build(worktree_abs_path: &Path) -> Result<Self> {
        let relative_paths = git_ls_files(worktree_abs_path).await?;
        let mut stats = Bm25IndexStats {
            candidate_file_count: relative_paths.len(),
            ..Default::default()
        };
        let mut documents = Vec::new();
        for relative_path in relative_paths {
            let Some(documents_for_file) = documents_for_file(worktree_abs_path, relative_path)
            else {
                continue;
            };

            stats.indexed_file_count += 1;
            stats.indexed_bytes += documents_for_file.byte_len;
            documents.extend(documents_for_file.documents);
        }

        let mut document_frequencies = HashMap::new();
        let mut total_document_len = 0;
        for document in &documents {
            total_document_len += document.len;
            let mut seen_terms = HashSet::new();
            for term in document.term_frequencies.keys() {
                if seen_terms.insert(term) {
                    *document_frequencies.entry(term.clone()).or_default() += 1;
                }
            }
        }

        let average_document_len = if documents.is_empty() {
            0.0
        } else {
            total_document_len as f64 / documents.len() as f64
        };
        stats.document_count = documents.len();
        stats.term_count = document_frequencies.len();

        Ok(Self {
            documents,
            document_frequencies,
            average_document_len,
            stats,
        })
    }

    fn search(
        &self,
        query_terms: &HashMap<String, f64>,
        worktree_root_name: &str,
        next_order: usize,
    ) -> Vec<Bm25ContextCandidate> {
        if self.documents.is_empty() || self.average_document_len == 0.0 {
            return Vec::new();
        }

        let mut scored_documents = self
            .documents
            .iter()
            .enumerate()
            .filter_map(|(document_index, document)| {
                let score = self.score_document(document, query_terms);
                (score > 0.0).then_some(ScoredDocument {
                    document_index,
                    score,
                })
            })
            .collect::<Vec<_>>();

        scored_documents.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    self.documents[left.document_index]
                        .relative_path
                        .cmp(&self.documents[right.document_index].relative_path)
                })
                .then_with(|| {
                    self.documents[left.document_index]
                        .row_range
                        .start
                        .cmp(&self.documents[right.document_index].row_range.start)
                })
        });

        let mut selected_documents = Vec::new();
        let mut chunks_per_file = HashMap::<PathBuf, usize>::new();
        for scored_document in scored_documents {
            let document = &self.documents[scored_document.document_index];
            let chunk_count = chunks_per_file
                .entry(document.relative_path.clone())
                .or_default();
            if *chunk_count >= BM25_CONTEXT_MAX_CHUNKS_PER_FILE {
                continue;
            }

            *chunk_count += 1;
            selected_documents.push(Bm25ContextCandidate {
                path: Path::new(&format!(
                    "{}/{}",
                    worktree_root_name,
                    document.relative_path.to_string_lossy()
                ))
                .into(),
                row_range: document.row_range.clone(),
                order: next_order + selected_documents.len(),
            });

            if selected_documents.len() >= BM25_CONTEXT_CHUNK_COUNT {
                break;
            }
        }

        selected_documents
    }

    fn score_document(&self, document: &Document, query_terms: &HashMap<String, f64>) -> f64 {
        let document_count = self.documents.len() as f64;
        let document_len = document.len as f64;
        let mut score = 0.0;

        for (term, query_weight) in query_terms {
            let Some(term_frequency) = document.term_frequencies.get(term) else {
                continue;
            };
            let document_frequency = self
                .document_frequencies
                .get(term)
                .copied()
                .unwrap_or_default() as f64;
            if document_frequency == 0.0 {
                continue;
            }

            let inverse_document_frequency =
                ((document_count - document_frequency + 0.5) / (document_frequency + 0.5) + 1.0)
                    .ln();
            let term_frequency = *term_frequency as f64;
            let denominator = term_frequency
                + BM25_K1
                    * (1.0 - BM25_B + BM25_B * document_len / self.average_document_len.max(1.0));
            score += query_weight * inverse_document_frequency * term_frequency * (BM25_K1 + 1.0)
                / denominator;
        }

        score
    }
}

async fn git_ls_files(worktree_abs_path: &Path) -> Result<Vec<PathBuf>> {
    let output = new_command("git")
        .arg("ls-files")
        .arg("-z")
        .current_dir(worktree_abs_path)
        .output()
        .await
        .with_context(|| {
            format!(
                "failed to run git ls-files in {}",
                worktree_abs_path.display()
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "git ls-files failed in {} with status {}: {}",
            worktree_abs_path.display(),
            output.status,
            stderr.trim()
        );
    }

    let output =
        String::from_utf8(output.stdout).context("git ls-files output was not valid UTF-8")?;
    Ok(output
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .collect())
}

fn documents_for_file(
    worktree_abs_path: &Path,
    relative_path: PathBuf,
) -> Option<DocumentsForFile> {
    let absolute_path = worktree_abs_path.join(&relative_path);
    let metadata = fs::symlink_metadata(&absolute_path).ok()?;
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.len() > BM25_CONTEXT_MAX_FILE_BYTES
    {
        return None;
    }

    let text = fs::read_to_string(&absolute_path).ok()?;
    if text.is_empty() {
        return None;
    }

    let byte_len = metadata.len();
    let lines = lines(&text);
    let path_tokens = tokenize(&relative_path.to_string_lossy());

    let documents = chunk_line_ranges(
        &lines,
        BM25_CONTEXT_CHUNK_LINE_COUNT,
        BM25_CONTEXT_CHUNK_OVERLAP_LINE_COUNT,
    )
    .into_iter()
    .filter_map(|row_range| {
        let chunk_text = text_for_line_range(&text, row_range.clone());
        let mut term_frequencies = HashMap::new();
        add_term_frequencies(&mut term_frequencies, tokenize(&chunk_text), 1);
        add_term_frequencies(&mut term_frequencies, path_tokens.clone(), 2);
        let len = term_frequencies.values().sum();
        if len == 0 {
            return None;
        }

        Some(Document {
            relative_path: relative_path.clone(),
            row_range: row_range.start as u32..row_range.end as u32,
            term_frequencies,
            len,
        })
    })
    .collect::<Vec<_>>();

    (!documents.is_empty()).then_some(DocumentsForFile {
        documents,
        byte_len,
    })
}

fn add_term_frequencies(
    term_frequencies: &mut HashMap<String, usize>,
    tokens: Vec<String>,
    weight: usize,
) {
    for token in tokens {
        *term_frequencies.entry(token).or_default() += weight;
    }
}

fn chunk_line_ranges(
    lines: &[&str],
    target_line_count: usize,
    overlap_line_count: usize,
) -> Vec<Range<usize>> {
    if lines.is_empty() || target_line_count == 0 {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut start = 0;
    while start < lines.len() {
        let ideal_end = start.saturating_add(target_line_count).min(lines.len());
        let mut end = ideal_end;
        if ideal_end < lines.len()
            && let Some(boundary) =
                empty_line_boundary_near(lines, start, ideal_end, overlap_line_count)
        {
            end = boundary;
        }
        if end <= start {
            end = ideal_end;
        }
        if end <= start {
            break;
        }

        ranges.push(start..end);
        if end == lines.len() {
            break;
        }

        let next_start = end.saturating_sub(overlap_line_count);
        start = if next_start <= start { end } else { next_start };
    }

    ranges
}

fn empty_line_boundary_near(
    lines: &[&str],
    start: usize,
    ideal_end: usize,
    overlap_line_count: usize,
) -> Option<usize> {
    let search_start = ideal_end.saturating_sub(overlap_line_count).max(start + 1);
    let search_end = ideal_end
        .saturating_add(overlap_line_count)
        .min(lines.len());

    (search_start..search_end)
        .filter(|row| lines[*row].trim().is_empty())
        .min_by_key(|row| row.abs_diff(ideal_end))
        .map(|row| row + 1)
}

fn lines(text: &str) -> Vec<&str> {
    text.split_inclusive('\n').collect()
}

fn text_for_line_range(text: &str, range: Range<usize>) -> String {
    lines(text)
        .into_iter()
        .skip(range.start)
        .take(range.end.saturating_sub(range.start))
        .collect()
}

fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut segment = String::new();

    for character in text.chars() {
        if character.is_alphanumeric() || character == '_' || character == '-' {
            segment.push(character);
        } else {
            push_segment_tokens(&segment, &mut tokens);
            segment.clear();
        }
    }
    push_segment_tokens(&segment, &mut tokens);

    tokens
}

fn push_segment_tokens(segment: &str, tokens: &mut Vec<String>) {
    if segment.is_empty() {
        return;
    }

    let mut segment_tokens = Vec::new();
    push_token(segment, &mut segment_tokens);
    for part in segment.split(['_', '-']).filter(|part| !part.is_empty()) {
        push_token(part, &mut segment_tokens);
        for camel_part in camel_case_parts(part) {
            push_token(camel_part, &mut segment_tokens);
        }
    }

    let mut unique_segment_tokens = Vec::new();
    for token in segment_tokens {
        if !unique_segment_tokens.contains(&token) {
            unique_segment_tokens.push(token);
        }
    }
    tokens.extend(unique_segment_tokens);
}

fn camel_case_parts(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut previous = None;

    for (index, character) in text.char_indices() {
        if index > 0
            && character.is_uppercase()
            && previous
                .is_some_and(|previous: char| previous.is_lowercase() || previous.is_numeric())
        {
            parts.push(&text[start..index]);
            start = index;
        }
        previous = Some(character);
    }

    if start < text.len() {
        parts.push(&text[start..]);
    }

    parts
}

fn push_token(token: &str, tokens: &mut Vec<String>) {
    let token = token.to_lowercase();
    if token.len() <= 1
        || token.len() > 128
        || !token.chars().any(|character| character.is_alphabetic())
    {
        return;
    }
    tokens.push(token);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_splits_code_identifiers() {
        let tokens =
            tokenize("PrivateNetworkRequestPolicy foo_bar config/reg_default_16M_retrieval.json");

        assert!(tokens.contains(&"privatenetworkrequestpolicy".to_string()));
        assert!(tokens.contains(&"private".to_string()));
        assert!(tokens.contains(&"network".to_string()));
        assert!(tokens.contains(&"request".to_string()));
        assert!(tokens.contains(&"policy".to_string()));
        assert!(tokens.contains(&"foo_bar".to_string()));
        assert!(tokens.contains(&"foo".to_string()));
        assert!(tokens.contains(&"bar".to_string()));
        assert!(tokens.contains(&"reg_default_16m_retrieval".to_string()));
        assert!(tokens.contains(&"retrieval".to_string()));
    }

    #[test]
    fn test_chunk_line_ranges_prefers_empty_line_boundaries_with_overlap() {
        let text = "a\nb\n\nc\nd\ne\nf\n\ng\nh\ni\nj\n";
        let lines = lines(text);
        let ranges = chunk_line_ranges(&lines, 3, 1);

        assert_eq!(ranges[0], 0..3);
        assert!(ranges[1].start < ranges[0].end);
    }

    #[test]
    fn test_bm25_ranks_matching_chunk() {
        let documents = vec![
            Document {
                relative_path: PathBuf::from("src/unrelated.rs"),
                row_range: 0..1,
                term_frequencies: {
                    let mut terms = HashMap::new();
                    add_term_frequencies(&mut terms, tokenize("fn unrelated"), 1);
                    terms
                },
                len: 2,
            },
            Document {
                relative_path: PathBuf::from("src/network.rs"),
                row_range: 0..1,
                term_frequencies: {
                    let mut terms = HashMap::new();
                    add_term_frequencies(
                        &mut terms,
                        tokenize("fn update_private_network_request_policy"),
                        1,
                    );
                    terms
                },
                len: 6,
            },
        ];
        let mut document_frequencies = HashMap::new();
        for document in &documents {
            for term in document.term_frequencies.keys() {
                *document_frequencies.entry(term.clone()).or_default() += 1;
            }
        }
        let index = Bm25Index {
            documents,
            document_frequencies,
            average_document_len: 4.0,
            stats: Bm25IndexStats::default(),
        };
        let mut query = HashMap::new();
        add_query_terms(&mut query, "PrivateNetworkRequestPolicy", 1.0);

        let candidates = index.search(&query, "repo", 0);

        assert_eq!(candidates[0].path, Path::new("repo/src/network.rs"));
        assert_eq!(candidates[0].row_range, 0..1);
        assert_eq!(candidates[0].order, 0);
    }
}
