//! Hashed Regions (V0609HashedRegions): a variant of the Smart Regions
//! multi-region format where marker tags are identified by a short
//! content-derived hash (e.g. `<|marker_b1f8|>`) instead of a sequence
//! number.
//!
//! Hashed identifiers are self-describing: a tag can be mapped back to its
//! location without reproducing the exact rendering order of the prompt, so
//! markers can be placed across *all* prompt context, and budget-based
//! truncation of related files doesn't shift the addressing of the remaining
//! markers. All context, including the current file, lives in related files:
//! context retrieval includes the current file via `ContextSource::CurrentFile`,
//! so the cursor file is expected to be one of the related files.

use crate::{ContextSource, ZetaPromptInput, multi_region, udiff};
use anyhow::{Context as _, Result, anyhow};
use std::{
    borrow::Cow,
    collections::HashSet,
    path::{Path, PathBuf},
};

pub const MARKER_TAG_PREFIX: &str = "<|marker_";
pub const MARKER_TAG_SUFFIX: &str = "|>";
pub const V0615_END_MARKER: &str = "<[end▁of▁sentence]>";
pub const NO_EDITS: &str = "NO_EDITS";
/// Number of base64 characters in a marker tag identifier.
pub const TAG_ID_LEN: usize = 4;

const BASE64_URL_SAFE_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub fn marker_tag(id: &str) -> String {
    format!("{MARKER_TAG_PREFIX}{id}{MARKER_TAG_SUFFIX}")
}

/// Marker tags assigned to one contiguous snippet of context.
#[derive(Debug, Clone)]
pub struct SnippetMarkers {
    pub file_ix: usize,
    pub excerpt_ix: usize,
    /// `(tag id, byte offset within the snippet text)`, sorted by offset.
    /// The first marker is at offset 0 and the last at `text.len()`.
    pub markers: Vec<(String, usize)>,
}

/// Assign hashed marker tags to every related-file excerpt of `input`.
///
/// The assignment is deterministic and independent of any later budget-based
/// truncation, so the same table can be rebuilt when parsing model output.
pub fn build_marker_table(input: &ZetaPromptInput) -> Vec<SnippetMarkers> {
    build_marker_table_with_filter(input, |_| true)
}

pub fn build_editable_marker_table(input: &ZetaPromptInput) -> Vec<SnippetMarkers> {
    build_marker_table_with_filter(input, is_hash_region_editable_context_source)
}

pub fn is_hash_region_editable_context_source(context_source: ContextSource) -> bool {
    matches!(
        context_source,
        ContextSource::CurrentFile | ContextSource::EditHistory
    )
}

fn build_marker_table_with_filter(
    input: &ZetaPromptInput,
    include_context_source: impl Fn(ContextSource) -> bool,
) -> Vec<SnippetMarkers> {
    let mut used_ids = HashSet::new();
    let mut snippets = Vec::new();
    if let Some(related_files) = input.related_files.as_deref() {
        for (file_ix, file) in related_files.iter().enumerate() {
            for (excerpt_ix, excerpt) in file.excerpts.iter().enumerate() {
                if include_context_source(excerpt.context_source) {
                    snippets.push(SnippetMarkers {
                        file_ix,
                        excerpt_ix,
                        markers: assign_tags(&excerpt.text, &mut used_ids),
                    });
                }
            }
        }
    }
    snippets
}

pub fn markers_for_text(text: &str) -> Vec<(String, usize)> {
    let mut used_ids = HashSet::new();
    assign_tags(text, &mut used_ids)
}

fn assign_tags(text: &str, used_ids: &mut HashSet<String>) -> Vec<(String, usize)> {
    let offsets = multi_region::compute_marker_offsets_v0618(text);
    offsets
        .iter()
        .enumerate()
        .map(|(i, &offset)| {
            let block = match offsets.get(i + 1) {
                Some(&next_offset) => &text[offset..next_offset],
                // The final marker has no following block; hash the preceding
                // one. This collides with the previous marker's tag by
                // construction, which `unique_tag_id` resolves by reseeding.
                None => {
                    let previous_offset = if i == 0 { 0 } else { offsets[i - 1] };
                    &text[previous_offset..offset]
                }
            };
            (unique_tag_id(block, used_ids), offset)
        })
        .collect()
}

fn unique_tag_id(content: &str, used_ids: &mut HashSet<String>) -> String {
    let mut seed = 0u64;
    loop {
        let id = encode_tag_id(hash_with_seed(content, seed));
        if used_ids.insert(id.clone()) {
            return id;
        }
        seed += 1;
    }
}

/// FNV-1a, with the seed folded in ahead of the content.
fn hash_with_seed(content: &str, seed: u64) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hash = FNV_OFFSET;
    for byte in seed.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    for byte in content.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn encode_tag_id(hash: u64) -> String {
    (0..TAG_ID_LEN)
        .map(|i| BASE64_URL_SAFE_ALPHABET[((hash >> (6 * i)) & 0x3f) as usize] as char)
        .collect()
}

/// Write `text` into `output`, inserting marker tags at the given offsets.
/// When `cursor` is provided, its marker string is inserted at the given byte
/// offset within `text`.
pub fn write_snippet_with_markers(
    output: &mut String,
    text: &str,
    markers: &[(String, usize)],
    cursor: Option<(usize, &str)>,
) {
    let mut cursor_placed = false;
    for (i, (id, offset)) in markers.iter().enumerate() {
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }
        output.push_str(&marker_tag(id));

        if let Some((_, next_offset)) = markers.get(i + 1) {
            output.push('\n');
            let block = &text[*offset..*next_offset];
            match cursor {
                Some((cursor_offset, cursor_marker))
                    if !cursor_placed
                        && cursor_offset >= *offset
                        && cursor_offset <= *next_offset =>
                {
                    cursor_placed = true;
                    let cursor_in_block = cursor_offset - offset;
                    output.push_str(&block[..cursor_in_block]);
                    output.push_str(cursor_marker);
                    output.push_str(&block[cursor_in_block..]);
                }
                _ => output.push_str(block),
            }
        }
    }
}

/// Extract a marker-bounded span from a model-output codeblock.
///
/// Returns `(start tag id, end tag id, content)` where `content` is the text
/// between the first and last marker tags, with any intermediate marker tags
/// stripped.
pub fn extract_marker_span(text: &str) -> Result<(String, String, String)> {
    let (start_id, end_id, content) = extract_marker_span_allow_same(text)?;
    if start_id == end_id {
        return Err(anyhow!(
            "start and end markers are the same (marker {start_id})"
        ));
    }
    Ok((start_id, end_id, content))
}

pub fn extract_marker_span_allow_same(text: &str) -> Result<(String, String, String)> {
    let first_tag_start = text
        .find(MARKER_TAG_PREFIX)
        .context("no start marker found in output")?;
    let first_id_start = first_tag_start + MARKER_TAG_PREFIX.len();
    let first_id_end = text[first_id_start..]
        .find(MARKER_TAG_SUFFIX)
        .map(|i| i + first_id_start)
        .context("malformed start marker tag")?;
    let start_id = &text[first_id_start..first_id_end];
    let first_tag_end = first_id_end + MARKER_TAG_SUFFIX.len();

    let last_tag_start = text
        .rfind(MARKER_TAG_PREFIX)
        .context("no end marker found in output")?;
    if last_tag_start == first_tag_start {
        return Err(anyhow!("output span must be bounded by two marker tags"));
    }
    let last_id_start = last_tag_start + MARKER_TAG_PREFIX.len();
    let last_id_end = text[last_id_start..]
        .find(MARKER_TAG_SUFFIX)
        .map(|i| i + last_id_start)
        .context("malformed end marker tag")?;
    let end_id = &text[last_id_start..last_id_end];

    let mut content_start = first_tag_end;
    if text.as_bytes().get(content_start) == Some(&b'\n') {
        content_start += 1;
    }
    let content_end = last_tag_start;
    let content = &text[content_start..content_end.max(content_start)];
    let content = multi_region::strip_marker_tags(content);
    Ok((start_id.to_string(), end_id.to_string(), content))
}

pub struct RelatedFileCursor {
    pub file_ix: usize,
    pub excerpt_ix: usize,
    pub offset_in_excerpt: usize,
}

struct ParseSnippet<'a> {
    file_ix: usize,
    first_excerpt_ix: usize,
    last_excerpt_ix: usize,
    end_row: u32,
    text: Cow<'a, str>,
    markers: Vec<(String, usize)>,
}

pub fn related_file_patch_path(cursor_path: &Path, related_path: &Path) -> PathBuf {
    let stripped: PathBuf = related_path.iter().skip(1).collect();
    if stripped == cursor_path {
        return stripped;
    }

    let cursor_first_component = cursor_path.components().next();
    let related_first_component = related_path.components().next();
    if related_first_component.is_some()
        && cursor_first_component != related_first_component
        && related_path.components().count() > 1
    {
        stripped
    } else {
        related_path.to_path_buf()
    }
}

fn line_start_offset(text: &str, row: usize) -> Option<usize> {
    let mut offset = 0;
    for _ in 0..row {
        offset += text[offset..].find('\n')? + 1;
    }
    Some(offset)
}

pub fn locate_cursor_in_related_files(input: &ZetaPromptInput) -> Option<RelatedFileCursor> {
    let related_files = input.related_files.as_deref()?;
    let excerpt_start_row = input.excerpt_start_row?;
    let cursor_offset = input.cursor_offset_in_excerpt;
    let excerpt_prefix = input.cursor_excerpt.get(..cursor_offset)?;
    let cursor_row = excerpt_start_row + excerpt_prefix.matches('\n').count() as u32;
    let cursor_column = cursor_offset - excerpt_prefix.rfind('\n').map_or(0, |pos| pos + 1);

    for (file_ix, file) in related_files.iter().enumerate() {
        if related_file_patch_path(&input.cursor_path, &file.path) != input.cursor_path.as_ref() {
            continue;
        }

        for (excerpt_ix, excerpt) in file.excerpts.iter().enumerate() {
            if cursor_row < excerpt.row_range.start || cursor_row > excerpt.row_range.end {
                continue;
            }
            let row_in_excerpt = (cursor_row - excerpt.row_range.start) as usize;
            let line_start = line_start_offset(&excerpt.text, row_in_excerpt)?;
            let line_len = excerpt.text[line_start..]
                .lines()
                .next()
                .unwrap_or("")
                .len();
            if cursor_column <= line_len {
                return Some(RelatedFileCursor {
                    file_ix,
                    excerpt_ix,
                    offset_in_excerpt: line_start + cursor_column,
                });
            }
        }
    }

    None
}

pub fn marker_table_for_excerpt(
    marker_table: &[SnippetMarkers],
    file_ix: usize,
    excerpt_ix: usize,
) -> Option<&[(String, usize)]> {
    marker_table.iter().find_map(|snippet| {
        (snippet.file_ix == file_ix && snippet.excerpt_ix == excerpt_ix)
            .then_some(snippet.markers.as_slice())
    })
}

fn merge_contiguous_snippets(
    input: &ZetaPromptInput,
    marker_table: Vec<SnippetMarkers>,
) -> Result<Vec<ParseSnippet<'_>>> {
    let related_files = input
        .related_files
        .as_deref()
        .context("prompt inputs are missing related files")?;
    let mut snippets: Vec<ParseSnippet> = Vec::new();
    for snippet in marker_table {
        let file = related_files
            .get(snippet.file_ix)
            .context("related file index out of range")?;
        let excerpt = file
            .excerpts
            .get(snippet.excerpt_ix)
            .context("related excerpt index out of range")?;
        if let Some(last) = snippets.last_mut()
            && last.file_ix == snippet.file_ix
            && last.last_excerpt_ix + 1 == snippet.excerpt_ix
            && last.end_row == excerpt.row_range.start
        {
            let text = last.text.to_mut();
            if !text.is_empty() && !text.ends_with('\n') {
                text.push('\n');
            }
            let base = text.len();
            text.push_str(&excerpt.text);
            last.markers.extend(
                snippet
                    .markers
                    .into_iter()
                    .map(|(id, offset)| (id, base + offset)),
            );
            last.last_excerpt_ix = snippet.excerpt_ix;
            last.end_row = excerpt.row_range.end;
        } else {
            snippets.push(ParseSnippet {
                file_ix: snippet.file_ix,
                first_excerpt_ix: snippet.excerpt_ix,
                last_excerpt_ix: snippet.excerpt_ix,
                end_row: excerpt.row_range.end,
                text: Cow::Borrowed(excerpt.text.as_ref()),
                markers: snippet.markers,
            });
        }
    }
    Ok(snippets)
}

fn snippet_path_and_start_row(
    input: &ZetaPromptInput,
    snippet: &ParseSnippet<'_>,
) -> Result<(PathBuf, u32)> {
    let related_files = input
        .related_files
        .as_deref()
        .context("prompt inputs are missing related files")?;
    let file = related_files
        .get(snippet.file_ix)
        .context("related file index out of range")?;
    let excerpt = file
        .excerpts
        .get(snippet.first_excerpt_ix)
        .context("related excerpt index out of range")?;
    Ok((
        related_file_patch_path(&input.cursor_path, &file.path),
        excerpt.row_range.start,
    ))
}

fn find_marker(snippets: &[ParseSnippet<'_>], marker_id: &str) -> Option<(usize, usize)> {
    snippets
        .iter()
        .enumerate()
        .find_map(|(snippet_ix, snippet)| {
            snippet
                .markers
                .iter()
                .find_map(|(id, offset)| (id == marker_id).then_some((snippet_ix, *offset)))
        })
}

fn common_prefix_suffix(a: &[u8], b: &[u8]) -> (usize, usize) {
    let prefix = a.iter().zip(b.iter()).take_while(|(x, y)| x == y).count();
    let remaining_a = a.len() - prefix;
    let remaining_b = b.len() - prefix;
    let max_suffix = remaining_a.min(remaining_b);
    let suffix = a[a.len() - max_suffix..]
        .iter()
        .rev()
        .zip(b[b.len() - max_suffix..].iter().rev())
        .take_while(|(x, y)| x == y)
        .count();
    (prefix, suffix)
}

fn nearest_marker_id(markers: &[(String, usize)], cursor_offset: Option<usize>) -> &str {
    let cursor = cursor_offset.unwrap_or(0);
    markers
        .iter()
        .min_by_key(|(_, offset)| (*offset as isize - cursor as isize).unsigned_abs())
        .map(|(id, _)| id.as_str())
        .unwrap_or("unknown")
}

pub fn encode_from_old_and_new(
    old_text: &str,
    new_text: &str,
    markers: &[(String, usize)],
    cursor_offset_in_new: Option<usize>,
    cursor_marker: &str,
) -> Result<String> {
    let no_edit_id = nearest_marker_id(markers, cursor_offset_in_new);
    if old_text == new_text {
        let tag = marker_tag(no_edit_id);
        return Ok(format!("{tag}{tag}{V0615_END_MARKER}"));
    }

    let (common_prefix, common_suffix) =
        common_prefix_suffix(old_text.as_bytes(), new_text.as_bytes());
    let change_end_in_old = old_text.len() - common_suffix;
    let mut start_marker_ix = markers
        .iter()
        .rposition(|(_, offset)| *offset <= common_prefix)
        .unwrap_or(0);
    let mut end_marker_ix = markers
        .iter()
        .position(|(_, offset)| *offset >= change_end_in_old)
        .unwrap_or_else(|| markers.len().saturating_sub(1));

    if start_marker_ix == end_marker_ix {
        if end_marker_ix < markers.len().saturating_sub(1) {
            end_marker_ix += 1;
        } else if start_marker_ix > 0 {
            start_marker_ix -= 1;
        }
    }

    let old_start = markers
        .get(start_marker_ix)
        .map(|(_, offset)| *offset)
        .context("start marker out of range")?;
    let old_end = markers
        .get(end_marker_ix)
        .map(|(_, offset)| *offset)
        .context("end marker out of range")?;
    let new_start = old_start;
    let new_end = new_text
        .len()
        .saturating_sub(old_text.len().saturating_sub(old_end));
    let new_span = &new_text[new_start..new_end];

    let mut result = String::new();
    result.push_str(&marker_tag(&markers[start_marker_ix].0));
    result.push('\n');
    if let Some(cursor_offset) = cursor_offset_in_new {
        if cursor_offset >= new_start && cursor_offset <= new_end {
            let cursor_in_span = cursor_offset - new_start;
            result.push_str(&new_span[..cursor_in_span]);
            result.push_str(cursor_marker);
            result.push_str(&new_span[cursor_in_span..]);
        } else {
            result.push_str(new_span);
        }
    } else {
        result.push_str(new_span);
    }
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(&marker_tag(&markers[end_marker_ix].0));
    result.push_str(V0615_END_MARKER);
    Ok(result)
}

pub fn parse_output_as_patch(
    input: &ZetaPromptInput,
    output: &str,
    cursor_marker: &str,
) -> Result<String> {
    let output = output.strip_suffix(V0615_END_MARKER).unwrap_or(output);
    if output.trim() == NO_EDITS {
        return Ok(String::new());
    }

    let marker_table = build_marker_table(input);
    let snippets = merge_contiguous_snippets(input, marker_table)?;
    let (start_id, end_id, mut new_span) = extract_marker_span_allow_same(output)?;
    let (start_snippet, start_byte) = find_marker(&snippets, &start_id)
        .with_context(|| format!("unknown start marker `{start_id}`"))?;
    let (end_snippet, end_byte) = find_marker(&snippets, &end_id)
        .with_context(|| format!("unknown end marker `{end_id}`"))?;

    if start_snippet != end_snippet {
        return Err(anyhow!(
            "markers `{start_id}` and `{end_id}` belong to different context snippets"
        ));
    }
    if start_byte > end_byte {
        return Err(anyhow!(
            "start marker `{start_id}` must come before end marker `{end_id}`"
        ));
    }
    if start_id == end_id {
        return Ok(String::new());
    }

    let snippet = &snippets[start_snippet];
    let old_text = snippet.text.as_ref();
    let old_span = &old_text[start_byte..end_byte];
    if old_span.is_empty() {
        if !new_span.is_empty() && !new_span.ends_with('\n') {
            new_span.push('\n');
        }
    } else {
        if old_span.ends_with('\n') && !new_span.ends_with('\n') && !new_span.is_empty() {
            new_span.push('\n');
        }
        if !old_span.ends_with('\n') && new_span.ends_with('\n') {
            new_span.pop();
        }
    }

    let mut new_text = String::new();
    new_text.push_str(&old_text[..start_byte]);
    new_text.push_str(&new_span.replace(cursor_marker, ""));
    new_text.push_str(&old_text[end_byte..]);

    let (path, start_row) = snippet_path_and_start_row(input, snippet)?;
    let diff = udiff::unified_diff_with_context(old_text, &new_text, start_row, start_row, 3);
    if diff.is_empty() {
        return Ok(String::new());
    }

    let path = path.to_string_lossy();
    Ok(format!("--- a/{path}\n+++ b/{path}\n{diff}"))
}

pub fn encode_patch_as_output(
    input: &ZetaPromptInput,
    patch: &str,
    cursor_offset: Option<usize>,
    cursor_marker: &str,
) -> Result<String> {
    if patch.lines().count() <= 3 {
        return Ok(format!("{NO_EDITS}{V0615_END_MARKER}"));
    }

    let marker_table = build_marker_table(input);
    let snippets = merge_contiguous_snippets(input, marker_table)?;
    let mut parser = udiff::DiffParser::new(patch);
    let mut encoded_output = None;

    while let Some(event) = parser.next().context("failed to parse expected patch")? {
        let udiff::DiffEvent::Hunk {
            path,
            mut hunk,
            status: _,
        } = event
        else {
            continue;
        };

        if encoded_output.is_some() {
            anyhow::bail!("hashed-region expected-output encoding supports one hunk");
        }

        let (snippet_ix, start_row) = snippets
            .iter()
            .enumerate()
            .find_map(|(snippet_ix, snippet)| {
                let (snippet_path, start_row) = snippet_path_and_start_row(input, snippet).ok()?;
                (snippet_path == Path::new(path.as_ref())).then_some((snippet_ix, start_row))
            })
            .with_context(|| format!("no hash-region context for patch path `{path}`"))?;
        let snippet = &snippets[snippet_ix];
        let old_text = snippet.text.as_ref();
        let candidates = udiff::find_context_candidates(old_text, &mut hunk);
        let hunk_offset =
            udiff::disambiguate_by_line_number(&candidates, hunk.start_line, &|offset| {
                start_row + old_text[..offset].matches('\n').count() as u32
            })
            .ok_or_else(|| anyhow!("couldn't resolve hunk in hash-region context"))?;

        let mut new_text = old_text.to_string();
        for edit in hunk.edits.iter().rev() {
            let range = (hunk_offset + edit.range.start)..(hunk_offset + edit.range.end);
            new_text.replace_range(range, &edit.text);
        }
        let cursor_in_new = cursor_offset.map(|cursor| (hunk_offset + cursor).min(new_text.len()));
        encoded_output = Some(encode_from_old_and_new(
            old_text,
            &new_text,
            &snippet.markers,
            cursor_in_new,
            cursor_marker,
        )?);
    }

    encoded_output.context("expected patch did not contain an encodable hunk")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContextSource, RelatedExcerpt, RelatedFile};
    use std::path::PathBuf;
    use std::sync::Arc;

    fn make_input(cursor_excerpt: &str, related: &[(&str, &[&str])]) -> ZetaPromptInput {
        ZetaPromptInput {
            cursor_path: PathBuf::from("src/main.rs").into(),
            cursor_excerpt: cursor_excerpt.into(),
            cursor_offset_in_excerpt: 0,
            excerpt_start_row: Some(0),
            events: Vec::new(),
            related_files: Some(
                related
                    .iter()
                    .map(|(path, excerpts)| {
                        let mut row = 0;
                        RelatedFile {
                            path: Arc::from(PathBuf::from(path).as_path()),
                            max_row: 1000,
                            excerpts: excerpts
                                .iter()
                                .map(|text| {
                                    let row_count = text.matches('\n').count() as u32;
                                    let excerpt = RelatedExcerpt {
                                        row_range: row..row + row_count,
                                        text: Arc::from(*text),
                                        order: 0,
                                        context_source: ContextSource::CurrentFile,
                                    };
                                    row += row_count + 10;
                                    excerpt
                                })
                                .collect(),
                            in_open_source_repo: false,
                        }
                    })
                    .collect(),
            ),
            active_buffer_diagnostics: Vec::new(),
            excerpt_ranges: crate::ExcerptRanges::default(),
            syntax_ranges: None,
            in_open_source_repo: false,
            can_collect_data: false,
            repo_url: None,
        }
    }

    #[test]
    fn test_tag_ids_are_unique_even_for_identical_blocks() {
        let mut used = HashSet::new();
        let id_a = unique_tag_id("same content", &mut used);
        let id_b = unique_tag_id("same content", &mut used);
        assert_ne!(id_a, id_b);
        assert_eq!(id_a.len(), TAG_ID_LEN);
        assert_eq!(id_b.len(), TAG_ID_LEN);
    }

    #[test]
    fn test_tag_ids_are_deterministic() {
        let mut used_a = HashSet::new();
        let mut used_b = HashSet::new();
        assert_eq!(
            unique_tag_id("hello\nworld\n", &mut used_a),
            unique_tag_id("hello\nworld\n", &mut used_b)
        );
    }

    #[test]
    fn test_build_marker_table_covers_all_context() {
        let input = make_input(
            "fn main() {\n    println!();\n}\n",
            &[
                ("src/a.rs", &["struct A;\n", "impl A {}\n"]),
                ("src/b.rs", &["struct B;\n"]),
            ],
        );
        let table = build_marker_table(&input);
        assert_eq!(table.len(), 3);
        assert_eq!((table[0].file_ix, table[0].excerpt_ix), (0, 0));
        assert_eq!((table[1].file_ix, table[1].excerpt_ix), (0, 1));
        assert_eq!((table[2].file_ix, table[2].excerpt_ix), (1, 0));

        let mut all_ids = HashSet::new();
        for snippet in &table {
            assert!(snippet.markers.len() >= 2);
            assert_eq!(snippet.markers.first().map(|(_, offset)| *offset), Some(0));
            for (id, _) in &snippet.markers {
                assert!(all_ids.insert(id.clone()), "duplicate tag id {id}");
            }
        }
    }

    #[test]
    fn test_write_snippet_with_markers_and_cursor() {
        let text = "fn main() {\n    let x = 1;\n}\n";
        let markers = vec![("aaaa".to_string(), 0), ("bbbb".to_string(), text.len())];
        let mut output = String::new();
        write_snippet_with_markers(&mut output, text, &markers, Some((16, "<|user_cursor|>")));
        assert_eq!(
            output,
            "<|marker_aaaa|>\nfn main() {\n    <|user_cursor|>let x = 1;\n}\n<|marker_bbbb|>"
        );
    }

    #[test]
    fn test_extract_marker_span_round_trip() {
        let codeblock = "<|marker_aaaa|>\nnew content\n<|marker_bbbb|>";
        let (start, end, content) = extract_marker_span(codeblock).unwrap();
        assert_eq!(start, "aaaa");
        assert_eq!(end, "bbbb");
        assert_eq!(content, "new content\n");
    }

    #[test]
    fn test_extract_marker_span_strips_intermediate_tags() {
        let codeblock = "<|marker_aaaa|>\nline one\n<|marker_cccc|>\nline two\n<|marker_bbbb|>";
        let (start, end, content) = extract_marker_span(codeblock).unwrap();
        assert_eq!(start, "aaaa");
        assert_eq!(end, "bbbb");
        assert_eq!(content, "line one\nline two\n");
    }

    #[test]
    fn test_extract_marker_span_rejects_single_marker() {
        assert!(extract_marker_span("<|marker_aaaa|>\ncontent\n").is_err());
    }

    #[test]
    fn test_extract_marker_span_rejects_same_marker() {
        assert!(extract_marker_span("<|marker_aaaa|>\ncontent\n<|marker_aaaa|>").is_err());
    }
}
