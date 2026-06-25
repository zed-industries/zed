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
//! so the cursor file is expected to be one of the related files. Inputs that
//! weren't run through current-file retrieval can be normalized with
//! [`ensure_cursor_file_excerpt`] before rendering or parsing.

use crate::{ContextSource, RelatedExcerpt, RelatedFile, Zeta2PromptInput, multi_region, udiff};
use anyhow::{Context as _, Result, anyhow};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    ops::Range,
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
pub fn build_marker_table(input: &Zeta2PromptInput) -> Vec<SnippetMarkers> {
    build_marker_table_with_filter(input, |_| true)
}

pub fn build_editable_marker_table(input: &Zeta2PromptInput) -> Vec<SnippetMarkers> {
    build_marker_table_with_filter(input, is_hash_region_editable_context_source)
}

pub fn is_hash_region_editable_context_source(context_source: ContextSource) -> bool {
    matches!(
        context_source,
        ContextSource::CurrentFile | ContextSource::EditHistory
    )
}

fn build_marker_table_with_filter(
    input: &Zeta2PromptInput,
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

pub fn locate_cursor_in_related_files(input: &Zeta2PromptInput) -> Option<RelatedFileCursor> {
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

/// Ensure the cursor file is represented by a related-file excerpt that covers
/// the cursor, synthesizing one from `cursor_excerpt` when it isn't.
///
/// All hashed-region context — including the current file — is addressed
/// through `related_files` (see module docs), so a prompt built from a
/// `Zeta2PromptInput` whose `related_files` don't cover the cursor cannot be
/// rendered or parsed. Inputs produced by current-file context retrieval
/// (`ContextSource::CurrentFile`) are already covered and left untouched; this
/// normalizes the rest (e.g. raw settled-data samples, or any caller that
/// didn't run current-file retrieval) from the `cursor_excerpt` the input
/// already carries, so the format is usable without re-running context
/// collection.
///
/// When synthesis is needed, any pre-existing excerpts of the cursor file are
/// **replaced** by the synthesized window: the renderer emits excerpts verbatim
/// without coalescing, so keeping overlapping fragments would duplicate lines
/// with conflicting markers. Other related files are left untouched.
///
/// Returns whether the cursor file is covered after the call (already, or via
/// the synthesized excerpt). Returns `false` only when coverage couldn't be
/// established — e.g. a missing `excerpt_start_row` or an empty
/// `cursor_excerpt` — in which case the input is left unchanged.
pub fn ensure_cursor_file_excerpt(input: &mut Zeta2PromptInput) -> bool {
    if locate_cursor_in_related_files(input).is_some() {
        return true;
    }
    let Some(excerpt_start_row) = input.excerpt_start_row else {
        return false;
    };
    if input.cursor_excerpt.is_empty() {
        return false;
    }

    let cursor_excerpt = input.cursor_excerpt.clone();
    let end_row = excerpt_start_row + cursor_excerpt.matches('\n').count() as u32;
    let synthesized = RelatedExcerpt {
        row_range: excerpt_start_row..end_row,
        text: cursor_excerpt,
        order: 0,
        context_source: ContextSource::CurrentFile,
    };

    let cursor_path = input.cursor_path.clone();
    let in_open_source_repo = input.in_open_source_repo;
    let related_files = input.related_files.get_or_insert_with(Vec::new);
    if let Some(file) = related_files
        .iter_mut()
        .find(|file| related_file_patch_path(&cursor_path, &file.path) == cursor_path.as_ref())
    {
        file.max_row = file.max_row.max(end_row);
        file.excerpts = vec![synthesized];
    } else {
        related_files.insert(
            0,
            RelatedFile {
                path: cursor_path,
                max_row: end_row,
                excerpts: vec![synthesized],
                in_open_source_repo,
            },
        );
    }

    // Confirm the synthesized excerpt actually covers the cursor (guards against
    // a cursor offset that lies outside the excerpt text).
    locate_cursor_in_related_files(input).is_some()
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
    input: &Zeta2PromptInput,
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
    input: &Zeta2PromptInput,
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

/// Encode a single marker-bounded edit block for one snippet, given its old and
/// new text. The returned block starts and ends with a marker tag and does
/// **not** include the output end marker; callers concatenate blocks and append
/// [`V0615_END_MARKER`] once after the last block.
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
        return Ok(format!("{tag}{tag}"));
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
    Ok(result)
}

/// Parse student model output (raw marker spans, no markdown code fences) into
/// a unified patch.
///
/// The output is a run of marker tags with no fences, so blocks are delimited by
/// pairing tags two at a time: `(1, 2), (3, 4), ...`. This matches the encoder,
/// which emits exactly two tags per block and no intermediate tags. Any
/// unpaired trailing tag is ignored.
pub fn parse_output_as_patch(
    input: &Zeta2PromptInput,
    output: &str,
    cursor_marker: &str,
) -> Result<String> {
    let output = output.strip_suffix(V0615_END_MARKER).unwrap_or(output);
    if output.trim() == NO_EDITS {
        return Ok(String::new());
    }

    let spans = pair_marker_spans(output)?;
    let (patch, _cursor) = build_patch_from_spans(input, &spans, cursor_marker)?;
    Ok(patch)
}

/// A cursor position resolved while turning marker-span edits into a patch.
pub struct HashRegionCursor {
    pub path: PathBuf,
    /// Byte offset of the cursor within `new_text`.
    pub cursor_offset_in_new_text: usize,
    /// Full new text of the edited snippet, after applying all of its edits.
    pub new_text: String,
    /// Original text of the edited snippet.
    pub old_text: String,
    /// 0-based row where the snippet starts in its file.
    pub start_row: u32,
}

/// One marker-bounded edit resolved against a parse snippet.
struct ParsedSpanEdit {
    snippet_ix: usize,
    range: Range<usize>,
    new_text: String,
    cursor_offset_in_new_text: Option<usize>,
}

/// Split raw model output into marker-bounded spans by pairing marker tags two
/// at a time. Returns `(start_id, end_id, raw_new_span)` per pair, where
/// `raw_new_span` may still contain the cursor marker.
fn pair_marker_spans(output: &str) -> Result<Vec<(String, String, String)>> {
    let tags = find_all_marker_tags(output);
    if tags.len() < 2 {
        return Err(anyhow!("output does not contain a marker-bounded span"));
    }
    let mut spans = Vec::new();
    let mut i = 0;
    while i + 1 < tags.len() {
        let (start_id, _, start_tag_end) = &tags[i];
        let (end_id, end_tag_start, _) = &tags[i + 1];
        let content = &output[*start_tag_end..*end_tag_start];
        let content = content.strip_prefix('\n').unwrap_or(content);
        let content = multi_region::strip_marker_tags(content);
        spans.push((start_id.clone(), end_id.clone(), content));
        i += 2;
    }
    Ok(spans)
}

/// Find every marker tag in `text`, in order, as `(id, tag_start, tag_end)`.
fn find_all_marker_tags(text: &str) -> Vec<(String, usize, usize)> {
    let mut tags = Vec::new();
    let mut search = 0;
    while let Some(rel) = text[search..].find(MARKER_TAG_PREFIX) {
        let tag_start = search + rel;
        let id_start = tag_start + MARKER_TAG_PREFIX.len();
        let Some(suffix_rel) = text[id_start..].find(MARKER_TAG_SUFFIX) else {
            break;
        };
        let id_end = id_start + suffix_rel;
        let tag_end = id_end + MARKER_TAG_SUFFIX.len();
        tags.push((text[id_start..id_end].to_string(), tag_start, tag_end));
        search = tag_end;
    }
    tags
}

/// Resolve a list of marker spans into per-snippet edits and assemble a unified
/// patch.
///
/// `spans` is a list of `(start_id, end_id, raw_new_span)` where `raw_new_span`
/// may still contain `cursor_marker`. This is shared by the student parser
/// (which pairs raw marker tags) and the teacher parser (which extracts spans
/// from markdown code fences). Edits that overlap an already-accepted edit in
/// the same snippet are skipped (lenient). The cursor marker is honored in
/// every region that contains it; the returned [`HashRegionCursor`] reports the
/// first such position.
pub fn build_patch_from_spans(
    input: &Zeta2PromptInput,
    spans: &[(String, String, String)],
    cursor_marker: &str,
) -> Result<(String, Option<HashRegionCursor>)> {
    let marker_table = build_marker_table(input);
    let snippets = merge_contiguous_snippets(input, marker_table)?;
    let mut marker_index: HashMap<&str, (usize, usize)> = HashMap::new();
    for (snippet_ix, snippet) in snippets.iter().enumerate() {
        for (id, offset) in &snippet.markers {
            marker_index.insert(id.as_str(), (snippet_ix, *offset));
        }
    }

    let mut edits: Vec<ParsedSpanEdit> = Vec::new();
    for (start_id, end_id, raw_new_span) in spans {
        let &(start_snippet, start_byte) = marker_index
            .get(start_id.as_str())
            .with_context(|| format!("unknown start marker `{start_id}`"))?;
        let &(end_snippet, end_byte) = marker_index
            .get(end_id.as_str())
            .with_context(|| format!("unknown end marker `{end_id}`"))?;

        if start_snippet != end_snippet {
            return Err(anyhow!(
                "markers `{start_id}` and `{end_id}` belong to different context snippets \
                 that are not contiguous excerpts of the same file"
            ));
        }
        if start_byte > end_byte {
            return Err(anyhow!(
                "start marker `{start_id}` must come before end marker `{end_id}`"
            ));
        }

        let old_text = snippets[start_snippet].text.as_ref();
        let old_span = &old_text[start_byte..end_byte];

        let cursor_in_span = raw_new_span.find(cursor_marker);
        let mut new_span = raw_new_span.replace(cursor_marker, "");
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

        if !new_span.is_empty()
            && let Some(dropped) = detect_trailing_deletion(old_span, &new_span)
        {
            return Err(anyhow!(
                "edit span `{start_id}`..`{end_id}` looks truncated: the replacement \
                 stops before the end marker, which would silently delete:\n{dropped}"
            ));
        }

        // `cursor_in_span` was located in `raw_new_span` before the trailing
        // newline normalization above, which can drop a byte. Clamp it to the
        // finalized replacement so the offset never points past `new_span`
        // (downstream cursor mapping byte-slices `new_text` by this offset).
        let cursor_offset_in_new_text = cursor_in_span.map(|offset| offset.min(new_span.len()));
        edits.push(ParsedSpanEdit {
            snippet_ix: start_snippet,
            range: start_byte..end_byte,
            new_text: new_span,
            cursor_offset_in_new_text,
        });
    }

    assemble_patch_from_edits(input, &snippets, edits)
}

/// Apply resolved edits to their snippets and emit one diff section per edited
/// snippet, in the order snippets first appear in the edit sequence.
fn assemble_patch_from_edits(
    input: &Zeta2PromptInput,
    snippets: &[ParseSnippet<'_>],
    edits: Vec<ParsedSpanEdit>,
) -> Result<(String, Option<HashRegionCursor>)> {
    let mut snippet_order: Vec<usize> = Vec::new();
    for edit in &edits {
        if !snippet_order.contains(&edit.snippet_ix) {
            snippet_order.push(edit.snippet_ix);
        }
    }

    let mut diff_output = String::new();
    let mut cursor = None;

    for &snippet_ix in &snippet_order {
        let snippet = &snippets[snippet_ix];
        let mut snippet_edits: Vec<&ParsedSpanEdit> = edits
            .iter()
            .filter(|edit| edit.snippet_ix == snippet_ix)
            .collect();
        snippet_edits.sort_by_key(|edit| edit.range.start);

        // Lenient overlap handling: keep edits in line order, dropping any whose
        // range starts before the previous accepted edit ended.
        let mut accepted: Vec<&ParsedSpanEdit> = Vec::new();
        let mut last_end = 0usize;
        for edit in snippet_edits {
            if !accepted.is_empty() && edit.range.start < last_end {
                continue;
            }
            last_end = edit.range.end;
            accepted.push(edit);
        }

        let old_text = snippet.text.as_ref();
        let (path, start_row) = snippet_path_and_start_row(input, snippet)?;

        let mut new_text = String::new();
        let mut position = 0;
        let mut cursor_in_new_text = None;
        for edit in &accepted {
            new_text.push_str(&old_text[position..edit.range.start]);
            if let Some(cursor_offset) = edit.cursor_offset_in_new_text {
                cursor_in_new_text = Some(new_text.len() + cursor_offset);
            }
            new_text.push_str(&edit.new_text);
            position = edit.range.end;
        }
        new_text.push_str(&old_text[position..]);

        let diff = udiff::unified_diff_with_context(old_text, &new_text, start_row, start_row, 3);
        if !diff.is_empty() {
            let path_str = path
                .iter()
                .map(|component| component.to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");
            diff_output.push_str(&format!("--- a/{path_str}\n+++ b/{path_str}\n"));
            diff_output.push_str(&diff);
            if !diff_output.ends_with('\n') {
                diff_output.push('\n');
            }
        }

        if cursor.is_none()
            && let Some(cursor_offset) = cursor_in_new_text
        {
            cursor = Some(HashRegionCursor {
                path: path.clone(),
                cursor_offset_in_new_text: cursor_offset,
                new_text: new_text.clone(),
                old_text: old_text.to_string(),
                start_row,
            });
        }
    }

    Ok((diff_output, cursor))
}

/// Detects a span replacement that ends in a pure deletion of the span's tail,
/// the signature of a model that stopped writing before reaching its end
/// marker.
///
/// Returns the deleted tail if the line diff between `old_span` and `new_span`
/// ends with a deletion-only group that reaches the last line of `old_span` and
/// drops more than `MAX_TRAILING_DELETED_LINES` non-blank lines.
fn detect_trailing_deletion(old_span: &str, new_span: &str) -> Option<String> {
    const MAX_TRAILING_DELETED_LINES: usize = 3;

    fn flag_if_large(deleted_tail: &str) -> Option<String> {
        let non_blank_deleted = deleted_tail
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count();
        (non_blank_deleted > MAX_TRAILING_DELETED_LINES)
            .then(|| deleted_tail.trim_end().to_string())
    }

    // A verbatim prefix is checked at the byte level so that a replacement
    // stopping mid-line is caught too; the line diff below would see that as a
    // trailing replace group rather than a pure deletion.
    if let Some(deleted_tail) = old_span.strip_prefix(new_span) {
        return flag_if_large(deleted_tail);
    }

    // With zero context lines, hunks contain only `-` and `+` lines, and within
    // a hunk deletions precede insertions, so a diff whose final line is a
    // deletion ends with a deletion-only group.
    let diff = udiff::unified_diff_with_context(old_span, new_span, 0, 0, 0);
    let lines: Vec<&str> = diff.lines().collect();
    let mut deletion_start = lines.len();
    while deletion_start > 0 && lines[deletion_start - 1].starts_with('-') {
        deletion_start -= 1;
    }
    let deleted: Vec<&str> = lines[deletion_start..]
        .iter()
        .map(|line| line.strip_prefix('-').unwrap_or(line))
        .collect();
    if deleted.is_empty() {
        return None;
    }

    // The trailing `-` run is preceded by its hunk header exactly when the hunk
    // is deletion-only (a replacement group would interpose `+` lines).
    let header = lines.get(deletion_start.checked_sub(1)?)?;
    let old_range_start: usize = header
        .strip_prefix("@@ -")?
        .split(',')
        .next()?
        .parse()
        .ok()?;

    // Only flag deletions that reach the end of the span; a deletion in the
    // middle is followed by reproduced context, so the model demonstrably kept
    // writing past it.
    if old_range_start + deleted.len() - 1 != old_span.lines().count() {
        return None;
    }

    flag_if_large(&deleted.join("\n"))
}

/// Encode an expected unified patch into the training output for a student.
///
/// Emits **one marker-bounded block per diff hunk**, in patch order (which
/// preserves the teacher's cross-file ordering), with no markdown code fences,
/// terminated by a single [`V0615_END_MARKER`]. Blocks are separated by a
/// newline; the parser re-pairs marker tags two at a time, so the separator is
/// not significant.
///
/// Reachability is per hunk: a hunk whose file is absent from the prompt
/// context, or whose location can't be resolved within its snippet, is skipped.
/// If at least one hunk is reachable, the remaining hunks are still encoded
/// (partial edit). If no hunk is reachable, the output is `NO_EDITS`.
pub fn encode_patch_as_output(
    input: &Zeta2PromptInput,
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
    let mut blocks: Vec<String> = Vec::new();

    while let Some(event) = parser.next().context("failed to parse expected patch")? {
        let udiff::DiffEvent::Hunk {
            path,
            mut hunk,
            status: _,
        } = event
        else {
            continue;
        };

        // A hunk whose file isn't in the prompt context is unreachable; skip it
        // and keep any other reachable hunks (partial edit).
        let Some((snippet_ix, start_row)) =
            snippets
                .iter()
                .enumerate()
                .find_map(|(snippet_ix, snippet)| {
                    let (snippet_path, start_row) =
                        snippet_path_and_start_row(input, snippet).ok()?;
                    (snippet_path == Path::new(path.as_ref())).then_some((snippet_ix, start_row))
                })
        else {
            continue;
        };
        let snippet = &snippets[snippet_ix];
        let old_text = snippet.text.as_ref();
        let candidates = udiff::find_context_candidates(old_text, &mut hunk);
        // A hunk whose location can't be pinned down within the snippet is
        // unreachable; skip it.
        let Some(hunk_offset) =
            udiff::disambiguate_by_line_number(&candidates, hunk.start_line, &|offset| {
                start_row + old_text[..offset].matches('\n').count() as u32
            })
        else {
            continue;
        };

        let mut new_text = old_text.to_string();
        for edit in hunk.edits.iter().rev() {
            let range = (hunk_offset + edit.range.start)..(hunk_offset + edit.range.end);
            new_text.replace_range(range, &edit.text);
        }
        // The cursor marker is placed in every region whose span contains it.
        // The extracted `cursor_offset` is hunk-relative, so map it through each
        // hunk's offset; `encode_from_old_and_new` inserts it only when it lands
        // within that block's span.
        let cursor_in_new = cursor_offset.map(|cursor| (hunk_offset + cursor).min(new_text.len()));
        blocks.push(encode_from_old_and_new(
            old_text,
            &new_text,
            &snippet.markers,
            cursor_in_new,
            cursor_marker,
        )?);
    }

    if blocks.is_empty() {
        return Ok(format!("{NO_EDITS}{V0615_END_MARKER}"));
    }

    let mut output = blocks.join("\n");
    output.push_str(V0615_END_MARKER);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContextSource, RelatedExcerpt, RelatedFile};
    use std::path::PathBuf;
    use std::sync::Arc;

    fn make_input(cursor_excerpt: &str, related: &[(&str, &[&str])]) -> Zeta2PromptInput {
        Zeta2PromptInput {
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
    fn test_ensure_cursor_file_excerpt_synthesizes_when_uncovered() {
        // The cursor file's only related excerpt is a fragment elsewhere in the
        // file (rows 40..42), not covering the cursor at row 1.
        let mut input = make_input(
            "fn main() {\n    let x = 1;\n}\n",
            &[("src/main.rs", &["// unrelated\n// fragment\n"])],
        );
        input.cursor_offset_in_excerpt = 16; // inside "    let x = 1;"
        input.related_files.as_mut().unwrap()[0].excerpts[0].row_range = 40..42;

        assert!(locate_cursor_in_related_files(&input).is_none());
        assert!(ensure_cursor_file_excerpt(&mut input));

        let cursor =
            locate_cursor_in_related_files(&input).expect("cursor covered after synthesis");
        let file = &input.related_files.as_ref().unwrap()[cursor.file_ix];
        // The fragment was replaced by the synthesized full window, so the file
        // content isn't duplicated with overlapping markers.
        assert_eq!(file.excerpts.len(), 1);
        assert_eq!(file.excerpts[0].context_source, ContextSource::CurrentFile);
        assert_eq!(file.excerpts[0].row_range, 0..3);
        assert_eq!(
            file.excerpts[0].text.as_ref(),
            "fn main() {\n    let x = 1;\n}\n"
        );
    }

    #[test]
    fn test_ensure_cursor_file_excerpt_noop_when_covered() {
        // make_input places the cursor file's excerpt at rows 0..3, covering the
        // cursor at row 1.
        let mut input = make_input(
            "fn main() {\n    let x = 1;\n}\n",
            &[("src/main.rs", &["fn main() {\n    let x = 1;\n}\n"])],
        );
        input.cursor_offset_in_excerpt = 16;
        let before = input.clone();
        assert!(ensure_cursor_file_excerpt(&mut input));
        assert_eq!(input, before);
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

    const MULTI_FN_EXCERPT: &str = "fn alpha() {\n    one();\n}\n\nfn beta() {\n    two();\n}\n\nfn gamma() {\n    three();\n}\n";

    const TWO_HUNK_PATCH: &str = concat!(
        "--- a/src/main.rs\n",
        "+++ b/src/main.rs\n",
        "@@ -1,3 +1,3 @@\n",
        " fn alpha() {\n",
        "-    one();\n",
        "+    uno();\n",
        " }\n",
        "@@ -9,3 +9,3 @@\n",
        " fn gamma() {\n",
        "-    three();\n",
        "+    tres();\n",
        " }\n",
    );

    #[test]
    fn test_encode_multi_hunk_emits_multiple_blocks() {
        let input = make_input(MULTI_FN_EXCERPT, &[("src/main.rs", &[MULTI_FN_EXCERPT])]);
        let output =
            encode_patch_as_output(&input, TWO_HUNK_PATCH, None, "<|user_cursor|>").unwrap();

        assert!(output.ends_with(V0615_END_MARKER), "output: {output}");
        // Two blocks => four marker tags, exactly one end marker.
        assert_eq!(
            output.matches(MARKER_TAG_PREFIX).count(),
            4,
            "output: {output}"
        );
        assert_eq!(
            output.matches(V0615_END_MARKER).count(),
            1,
            "output: {output}"
        );
        assert!(output.contains("uno();"), "output: {output}");
        assert!(output.contains("tres();"), "output: {output}");
    }

    #[test]
    fn test_round_trip_multi_hunk() {
        let input = make_input(MULTI_FN_EXCERPT, &[("src/main.rs", &[MULTI_FN_EXCERPT])]);
        let output =
            encode_patch_as_output(&input, TWO_HUNK_PATCH, None, "<|user_cursor|>").unwrap();
        let patch = parse_output_as_patch(&input, &output, "<|user_cursor|>").unwrap();

        assert!(patch.contains("-    one();"), "patch: {patch}");
        assert!(patch.contains("+    uno();"), "patch: {patch}");
        assert!(patch.contains("-    three();"), "patch: {patch}");
        assert!(patch.contains("+    tres();"), "patch: {patch}");
    }

    #[test]
    fn test_encode_partial_skips_unreachable_hunk() {
        // Second hunk targets a file that is not in the prompt context, so it
        // is unreachable. The first (reachable) hunk is still encoded.
        let patch = format!(
            "{TWO_HUNK_PATCH}--- a/other.rs\n+++ b/other.rs\n@@ -1,1 +1,1 @@\n-gone();\n+kept();\n"
        );
        let input = make_input(MULTI_FN_EXCERPT, &[("src/main.rs", &[MULTI_FN_EXCERPT])]);
        let output = encode_patch_as_output(&input, &patch, None, "<|user_cursor|>").unwrap();

        assert_ne!(output.trim_end_matches(V0615_END_MARKER), NO_EDITS);
        assert!(output.contains("uno();"), "output: {output}");
        assert!(output.contains("tres();"), "output: {output}");
        assert!(!output.contains("kept();"), "output: {output}");
    }

    #[test]
    fn test_encode_no_edits_when_all_hunks_unreachable() {
        let patch = "--- a/other.rs\n+++ b/other.rs\n@@ -1,3 +1,3 @@\n fn x() {\n-    gone();\n+    kept();\n }\n";
        let input = make_input(MULTI_FN_EXCERPT, &[("src/main.rs", &[MULTI_FN_EXCERPT])]);
        let output = encode_patch_as_output(&input, patch, None, "<|user_cursor|>").unwrap();

        assert_eq!(output, format!("{NO_EDITS}{V0615_END_MARKER}"));
    }

    #[test]
    fn test_parse_multiple_direct_marker_blocks() {
        // The student emits raw marker spans with no code fences; blocks are
        // delimited by pairing tags two at a time.
        let input = make_input(MULTI_FN_EXCERPT, &[("src/main.rs", &[MULTI_FN_EXCERPT])]);
        let markers = build_marker_table(&input)[0].markers.clone();
        assert!(markers.len() >= 3, "expected internal markers: {markers:?}");

        let tag = |ix: usize| marker_tag(&markers[ix].0);
        let old_first = &MULTI_FN_EXCERPT[markers[0].1..markers[1].1];
        let old_second = &MULTI_FN_EXCERPT[markers[1].1..markers[markers.len() - 1].1];
        let new_first = old_first.replace("one()", "uno()");
        let new_second = old_second.replace("three()", "tres()");

        let output = format!(
            "{}\n{}{}\n{}\n{}{}{}",
            tag(0),
            new_first,
            tag(1),
            tag(1),
            new_second,
            tag(markers.len() - 1),
            V0615_END_MARKER,
        );

        let patch = parse_output_as_patch(&input, &output, "<|user_cursor|>").unwrap();
        assert!(patch.contains("+    uno();"), "patch: {patch}");
        assert!(patch.contains("+    tres();"), "patch: {patch}");
        assert_eq!(
            patch.matches("--- a/src/main.rs").count(),
            1,
            "patch: {patch}"
        );
    }
}
