//! Hashed Regions (V0609HashedRegions): a variant of the Smart Regions
//! multi-region format where marker tags are identified by a short
//! content-derived hash (e.g. `<|marker_b1f8|>`) instead of a sequence
//! number.
//!
//! Hashed identifiers are self-describing: a tag can be mapped back to its
//! location without reproducing the exact rendering order of the prompt, so
//! markers can be placed across *all* prompt context (the cursor file and
//! every related-file excerpt), and budget-based truncation of related files
//! doesn't shift the addressing of the remaining markers.

use crate::{ZetaPromptInput, multi_region};
use anyhow::{Context as _, Result, anyhow};
use std::collections::HashSet;

pub const MARKER_TAG_PREFIX: &str = "<|marker_";
pub const MARKER_TAG_SUFFIX: &str = "|>";
/// Number of base64 characters in a marker tag identifier.
pub const TAG_ID_LEN: usize = 4;

const BASE64_URL_SAFE_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub fn marker_tag(id: &str) -> String {
    format!("{MARKER_TAG_PREFIX}{id}{MARKER_TAG_SUFFIX}")
}

/// Which piece of prompt context a marker-tagged snippet comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnippetSource {
    CursorFile,
    RelatedFile { file_ix: usize, excerpt_ix: usize },
}

/// Marker tags assigned to one contiguous snippet of context.
#[derive(Debug, Clone)]
pub struct SnippetMarkers {
    pub source: SnippetSource,
    /// `(tag id, byte offset within the snippet text)`, sorted by offset.
    /// The first marker is at offset 0 and the last at `text.len()`.
    pub markers: Vec<(String, usize)>,
}

/// Assign hashed marker tags to the cursor excerpt and every related-file
/// excerpt of `input`.
///
/// The assignment is deterministic and independent of any later budget-based
/// truncation, so the same table can be rebuilt when parsing model output.
pub fn build_marker_table(input: &ZetaPromptInput) -> Vec<SnippetMarkers> {
    let mut used_ids = HashSet::new();
    let mut snippets = vec![SnippetMarkers {
        source: SnippetSource::CursorFile,
        markers: assign_tags(&input.cursor_excerpt, &mut used_ids),
    }];
    if let Some(related_files) = input.related_files.as_deref() {
        for (file_ix, file) in related_files.iter().enumerate() {
            for (excerpt_ix, excerpt) in file.excerpts.iter().enumerate() {
                snippets.push(SnippetMarkers {
                    source: SnippetSource::RelatedFile {
                        file_ix,
                        excerpt_ix,
                    },
                    markers: assign_tags(&excerpt.text, &mut used_ids),
                });
            }
        }
    }
    snippets
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

    if start_id == end_id {
        return Err(anyhow!(
            "start and end markers are the same (marker {start_id})"
        ));
    }

    let mut content_start = first_tag_end;
    if text.as_bytes().get(content_start) == Some(&b'\n') {
        content_start += 1;
    }
    let content_end = last_tag_start;
    let content = &text[content_start..content_end.max(content_start)];
    let content = multi_region::strip_marker_tags(content);
    Ok((start_id.to_string(), end_id.to_string(), content))
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
        assert_eq!(table.len(), 4);
        assert_eq!(table[0].source, SnippetSource::CursorFile);
        assert_eq!(
            table[1].source,
            SnippetSource::RelatedFile {
                file_ix: 0,
                excerpt_ix: 0
            }
        );
        assert_eq!(
            table[3].source,
            SnippetSource::RelatedFile {
                file_ix: 1,
                excerpt_ix: 0
            }
        );

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
