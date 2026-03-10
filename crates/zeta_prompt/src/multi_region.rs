use anyhow::{Context as _, Result, anyhow};

pub const MARKER_TAG_PREFIX: &str = "<|marker_";
pub const MARKER_TAG_SUFFIX: &str = "|>";
const MIN_BLOCK_LINES: usize = 3;
const MAX_BLOCK_LINES: usize = 8;

pub fn marker_tag(number: usize) -> String {
    format!("{MARKER_TAG_PREFIX}{number}{MARKER_TAG_SUFFIX}")
}

/// Compute byte offsets within `editable_text` where marker boundaries should
/// be placed.
///
/// Returns a sorted `Vec<usize>` that always starts with `0` and ends with
/// `editable_text.len()`. Interior offsets are placed at line boundaries
/// (right after a `\n`), preferring blank-line boundaries when available and
/// respecting `MIN_BLOCK_LINES` / `MAX_BLOCK_LINES` constraints.
pub fn compute_marker_offsets(editable_text: &str) -> Vec<usize> {
    if editable_text.is_empty() {
        return vec![0, 0];
    }

    let mut offsets = vec![0usize];
    let mut lines_since_last_marker = 0usize;
    let mut byte_offset = 0usize;

    for line in editable_text.split('\n') {
        let line_end = byte_offset + line.len() + 1;
        let is_past_end = line_end > editable_text.len();
        let actual_line_end = line_end.min(editable_text.len());
        lines_since_last_marker += 1;

        let is_blank = line.trim().is_empty();

        if !is_past_end && lines_since_last_marker >= MIN_BLOCK_LINES {
            if is_blank {
                // Blank-line boundary found. We'll place the marker when we
                // find the next non-blank line (handled below).
            } else if lines_since_last_marker >= MAX_BLOCK_LINES {
                offsets.push(actual_line_end);
                lines_since_last_marker = 0;
            }
        }

        // Non-blank line immediately following blank line(s): split here so
        // the new block starts with this line.
        if !is_blank && byte_offset > 0 && lines_since_last_marker >= MIN_BLOCK_LINES {
            let before = &editable_text[..byte_offset];
            let has_preceding_blank_line = before
                .strip_suffix('\n')
                .map(|stripped| {
                    let last_line = match stripped.rfind('\n') {
                        Some(pos) => &stripped[pos + 1..],
                        None => stripped,
                    };
                    last_line.trim().is_empty()
                })
                .unwrap_or(false);

            if has_preceding_blank_line {
                offsets.push(byte_offset);
                lines_since_last_marker = 1;
            }
        }

        byte_offset = actual_line_end;

        // Re-check after blank-line logic since lines_since_last_marker may
        // have been reset.
        if !is_past_end && lines_since_last_marker >= MAX_BLOCK_LINES {
            if *offsets.last().unwrap_or(&0) != actual_line_end {
                offsets.push(actual_line_end);
                lines_since_last_marker = 0;
            }
        }
    }

    let end = editable_text.len();
    if *offsets.last().unwrap_or(&0) != end {
        offsets.push(end);
    }

    offsets
}

/// Write the editable region content with marker tags, inserting the cursor
/// marker at the given offset within the editable text.
pub fn write_editable_with_markers(
    output: &mut String,
    editable_text: &str,
    cursor_offset_in_editable: usize,
    cursor_marker: &str,
) {
    let marker_offsets = compute_marker_offsets(editable_text);
    let mut cursor_placed = false;
    for (i, &offset) in marker_offsets.iter().enumerate() {
        let marker_num = i + 1;
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }
        output.push_str(&marker_tag(marker_num));

        if let Some(&next_offset) = marker_offsets.get(i + 1) {
            output.push('\n');
            let block = &editable_text[offset..next_offset];
            if !cursor_placed
                && cursor_offset_in_editable >= offset
                && cursor_offset_in_editable <= next_offset
            {
                cursor_placed = true;
                let cursor_in_block = cursor_offset_in_editable - offset;
                output.push_str(&block[..cursor_in_block]);
                output.push_str(cursor_marker);
                output.push_str(&block[cursor_in_block..]);
            } else {
                output.push_str(block);
            }
        }
    }
}

/// Strip any `<|marker_N|>` tags from `text`.
///
/// When a marker tag sits on its own line (followed by `\n`), the trailing
/// newline is also removed so the surrounding lines stay joined naturally.
fn strip_marker_tags(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut pos = 0;
    let bytes = text.as_bytes();
    while let Some(rel) = text[pos..].find(MARKER_TAG_PREFIX) {
        result.push_str(&text[pos..pos + rel]);
        let num_start = pos + rel + MARKER_TAG_PREFIX.len();
        if let Some(suffix_rel) = text[num_start..].find(MARKER_TAG_SUFFIX) {
            let mut tag_end = num_start + suffix_rel + MARKER_TAG_SUFFIX.len();
            if bytes.get(tag_end) == Some(&b'\n') {
                tag_end += 1;
            }
            pos = tag_end;
        } else {
            result.push_str(MARKER_TAG_PREFIX);
            pos = num_start;
        }
    }
    result.push_str(&text[pos..]);
    result
}

/// Parse model output that uses the marker format.
///
/// Returns `(start_marker_num, end_marker_num, content_between_markers)`.
/// The leading format-level newline after the start marker is stripped.
/// Trailing newlines are preserved so blank-line endings in the editable
/// region are not lost.
///
/// Any extra intermediate marker tags that the model may have inserted
/// between the first and last markers are stripped from the returned content.
pub fn extract_marker_span(text: &str) -> Result<(usize, usize, String)> {
    let first_tag_start = text
        .find(MARKER_TAG_PREFIX)
        .context("no start marker found in output")?;
    let first_num_start = first_tag_start + MARKER_TAG_PREFIX.len();
    let first_num_end = text[first_num_start..]
        .find(MARKER_TAG_SUFFIX)
        .map(|i| i + first_num_start)
        .context("malformed start marker tag")?;
    let start_num: usize = text[first_num_start..first_num_end]
        .parse()
        .context("start marker number is not a valid integer")?;
    let first_tag_end = first_num_end + MARKER_TAG_SUFFIX.len();

    let last_tag_start = text
        .rfind(MARKER_TAG_PREFIX)
        .context("no end marker found in output")?;
    let last_num_start = last_tag_start + MARKER_TAG_PREFIX.len();
    let last_num_end = text[last_num_start..]
        .find(MARKER_TAG_SUFFIX)
        .map(|i| i + last_num_start)
        .context("malformed end marker tag")?;
    let end_num: usize = text[last_num_start..last_num_end]
        .parse()
        .context("end marker number is not a valid integer")?;

    if start_num == end_num {
        return Err(anyhow!(
            "start and end markers are the same (marker {})",
            start_num
        ));
    }

    let mut content_start = first_tag_end;
    if text.as_bytes().get(content_start) == Some(&b'\n') {
        content_start += 1;
    }
    let content_end = last_tag_start;

    let content = &text[content_start..content_end.max(content_start)];
    let content = strip_marker_tags(content);
    Ok((start_num, end_num, content))
}

/// Given old editable text and model output with marker span, reconstruct the
/// full new editable region.
pub fn apply_marker_span(old_editable: &str, output: &str) -> Result<String> {
    let (start_num, end_num, raw_new_span) = extract_marker_span(output)?;
    let marker_offsets = compute_marker_offsets(old_editable);

    let start_idx = start_num
        .checked_sub(1)
        .context("marker numbers are 1-indexed")?;
    let end_idx = end_num
        .checked_sub(1)
        .context("marker numbers are 1-indexed")?;
    let start_byte = *marker_offsets
        .get(start_idx)
        .context("start marker number out of range")?;
    let end_byte = *marker_offsets
        .get(end_idx)
        .context("end marker number out of range")?;

    if start_byte > end_byte {
        return Err(anyhow!("start marker must come before end marker"));
    }

    let old_span = &old_editable[start_byte..end_byte];
    let mut new_span = raw_new_span;
    if old_span.ends_with('\n') && !new_span.ends_with('\n') && !new_span.is_empty() {
        new_span.push('\n');
    }
    if !old_span.ends_with('\n') && new_span.ends_with('\n') {
        new_span.pop();
    }

    let mut result = String::new();
    result.push_str(&old_editable[..start_byte]);
    result.push_str(&new_span);
    result.push_str(&old_editable[end_byte..]);

    Ok(result)
}

/// Compare old and new editable text, find the minimal marker span that covers
/// all changes, and encode the result with marker tags.
pub fn encode_from_old_and_new(
    old_editable: &str,
    new_editable: &str,
    cursor_offset_in_new: Option<usize>,
    cursor_marker: &str,
    end_marker: &str,
    no_edits_marker: &str,
) -> Result<String> {
    if old_editable == new_editable {
        return Ok(format!("{no_edits_marker}{end_marker}"));
    }

    let marker_offsets = compute_marker_offsets(old_editable);

    let common_prefix = old_editable
        .bytes()
        .zip(new_editable.bytes())
        .take_while(|(a, b)| a == b)
        .count();

    let old_remaining = old_editable.len() - common_prefix;
    let new_remaining = new_editable.len() - common_prefix;
    let max_suffix = old_remaining.min(new_remaining);
    let common_suffix = old_editable.as_bytes()[old_editable.len() - max_suffix..]
        .iter()
        .rev()
        .zip(
            new_editable.as_bytes()[new_editable.len() - max_suffix..]
                .iter()
                .rev(),
        )
        .take_while(|(a, b)| a == b)
        .count();

    let change_end_in_old = old_editable.len() - common_suffix;

    let start_marker_idx = marker_offsets
        .iter()
        .rposition(|&offset| offset <= common_prefix)
        .unwrap_or(0);
    let end_marker_idx = marker_offsets
        .iter()
        .position(|&offset| offset >= change_end_in_old)
        .unwrap_or(marker_offsets.len() - 1);

    let old_start = marker_offsets[start_marker_idx];
    let old_end = marker_offsets[end_marker_idx];

    let new_start = old_start;
    let new_end = new_editable
        .len()
        .saturating_sub(old_editable.len().saturating_sub(old_end));

    let new_span = &new_editable[new_start..new_end];

    let start_marker_num = start_marker_idx + 1;
    let end_marker_num = end_marker_idx + 1;

    let mut result = String::new();
    result.push_str(&marker_tag(start_marker_num));
    result.push('\n');

    if let Some(cursor_offset) = cursor_offset_in_new {
        if cursor_offset >= new_start && cursor_offset <= new_end {
            let cursor_in_span = cursor_offset - new_start;
            let bounded = cursor_in_span.min(new_span.len());
            result.push_str(&new_span[..bounded]);
            result.push_str(cursor_marker);
            result.push_str(&new_span[bounded..]);
        } else {
            result.push_str(new_span);
        }
    } else {
        result.push_str(new_span);
    }

    if !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(&marker_tag(end_marker_num));
    result.push('\n');
    result.push_str(end_marker);

    Ok(result)
}

/// Extract the full editable region from text that uses marker tags.
///
/// Returns the concatenation of all block contents between the first and last
/// markers, with intermediate marker tags stripped.
pub fn extract_editable_region_from_markers(text: &str) -> Option<String> {
    let first_marker_start = text.find(MARKER_TAG_PREFIX)?;

    let mut markers: Vec<(usize, usize)> = Vec::new();
    let mut search_start = first_marker_start;
    while let Some(rel_pos) = text[search_start..].find(MARKER_TAG_PREFIX) {
        let tag_start = search_start + rel_pos;
        let num_start = tag_start + MARKER_TAG_PREFIX.len();
        let num_end = text[num_start..].find(MARKER_TAG_SUFFIX)?;
        let tag_end = num_start + num_end + MARKER_TAG_SUFFIX.len();
        markers.push((tag_start, tag_end));
        search_start = tag_end;
    }

    if markers.len() < 2 {
        return None;
    }

    let (_, first_tag_end) = markers[0];
    let (last_tag_start, _) = markers[markers.len() - 1];

    let mut content_start = first_tag_end;
    if text.as_bytes().get(content_start) == Some(&b'\n') {
        content_start += 1;
    }
    let mut content_end = last_tag_start;
    if content_end > content_start && text.as_bytes().get(content_end - 1) == Some(&b'\n') {
        content_end -= 1;
    }

    let raw = &text[content_start..content_end];
    let result = strip_marker_tags(raw);
    let result = result.strip_suffix('\n').unwrap_or(&result).to_string();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_marker_offsets_small_block() {
        let text = "aaa\nbbb\nccc\n";
        let offsets = compute_marker_offsets(text);
        assert_eq!(offsets, vec![0, text.len()]);
    }

    #[test]
    fn test_compute_marker_offsets_blank_line_split() {
        let text = "aaa\nbbb\nccc\n\nddd\neee\nfff\n";
        let offsets = compute_marker_offsets(text);
        assert_eq!(offsets[0], 0);
        assert!(offsets.contains(&13), "offsets: {:?}", offsets);
        assert_eq!(*offsets.last().unwrap(), text.len());
    }

    #[test]
    fn test_compute_marker_offsets_max_lines_split() {
        let text = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n";
        let offsets = compute_marker_offsets(text);
        assert!(offsets.len() >= 3, "offsets: {:?}", offsets);
    }

    #[test]
    fn test_compute_marker_offsets_empty() {
        let offsets = compute_marker_offsets("");
        assert_eq!(offsets, vec![0, 0]);
    }

    #[test]
    fn test_extract_marker_span() {
        let text = "<|marker_2|>\n    new content\n<|marker_3|>\n";
        let (start, end, content) = extract_marker_span(text).unwrap();
        assert_eq!(start, 2);
        assert_eq!(end, 3);
        assert_eq!(content, "    new content\n");
    }

    #[test]
    fn test_extract_marker_span_multi_line() {
        let text = "<|marker_1|>\nline1\nline2\nline3\n<|marker_4|>";
        let (start, end, content) = extract_marker_span(text).unwrap();
        assert_eq!(start, 1);
        assert_eq!(end, 4);
        assert_eq!(content, "line1\nline2\nline3\n");
    }

    #[test]
    fn test_apply_marker_span_basic() {
        let old = "aaa\nbbb\nccc\n";
        let output = "<|marker_1|>\naaa\nBBB\nccc\n<|marker_2|>";
        let result = apply_marker_span(old, output).unwrap();
        assert_eq!(result, "aaa\nBBB\nccc\n");
    }

    #[test]
    fn test_apply_marker_span_preserves_trailing_blank_line() {
        let old = "/\nresult\n\n";
        let output = "<|marker_1|>\n//\nresult\n\n<|marker_2|>";
        let result = apply_marker_span(old, output).unwrap();
        assert_eq!(result, "//\nresult\n\n");
    }

    #[test]
    fn test_encode_no_edits() {
        let old = "aaa\nbbb\nccc\n";
        let result = encode_from_old_and_new(
            old,
            old,
            None,
            "<|user_cursor|>",
            ">>>>>>> UPDATED\n",
            "NO_EDITS\n",
        )
        .unwrap();
        assert_eq!(result, "NO_EDITS\n>>>>>>> UPDATED\n");
    }

    #[test]
    fn test_encode_with_change() {
        let old = "aaa\nbbb\nccc\n";
        let new = "aaa\nBBB\nccc\n";
        let result = encode_from_old_and_new(
            old,
            new,
            None,
            "<|user_cursor|>",
            ">>>>>>> UPDATED\n",
            "NO_EDITS\n",
        )
        .unwrap();
        assert!(result.contains("<|marker_1|>"));
        assert!(result.contains("<|marker_2|>"));
        assert!(result.contains("aaa\nBBB\nccc\n"));
        assert!(result.ends_with(">>>>>>> UPDATED\n"));
    }

    #[test]
    fn test_roundtrip_encode_apply() {
        let old = "line1\nline2\nline3\n\nline5\nline6\nline7\nline8\nline9\nline10\n";
        let new = "line1\nline2\nline3\n\nline5\nLINE6\nline7\nline8\nline9\nline10\n";
        let encoded = encode_from_old_and_new(
            old,
            new,
            None,
            "<|user_cursor|>",
            ">>>>>>> UPDATED\n",
            "NO_EDITS\n",
        )
        .unwrap();
        let output = encoded
            .strip_suffix(">>>>>>> UPDATED\n")
            .expect("should have end marker");
        let reconstructed = apply_marker_span(old, output).unwrap();
        assert_eq!(reconstructed, new);
    }

    #[test]
    fn test_extract_editable_region_from_markers_multi() {
        let text = "prefix\n<|marker_1|>\naaa\nbbb\n<|marker_2|>\nccc\nddd\n<|marker_3|>\nsuffix";
        let parsed = extract_editable_region_from_markers(text).unwrap();
        assert_eq!(parsed, "aaa\nbbb\nccc\nddd");
    }

    #[test]
    fn test_extract_editable_region_two_markers() {
        let text = "<|marker_1|>\none\ntwo three\n<|marker_2|>";
        let parsed = extract_editable_region_from_markers(text).unwrap();
        assert_eq!(parsed, "one\ntwo three");
    }

    #[test]
    fn test_encode_with_cursor() {
        let old = "aaa\nbbb\nccc\n";
        let new = "aaa\nBBB\nccc\n";
        let result = encode_from_old_and_new(
            old,
            new,
            Some(5),
            "<|user_cursor|>",
            ">>>>>>> UPDATED\n",
            "NO_EDITS\n",
        )
        .unwrap();
        assert!(result.contains("<|user_cursor|>"), "result: {result}");
        assert!(result.contains("B<|user_cursor|>BB"), "result: {result}");
    }

    #[test]
    fn test_extract_marker_span_strips_intermediate_markers() {
        let text = "<|marker_2|>\nline1\n<|marker_3|>\nline2\n<|marker_4|>";
        let (start, end, content) = extract_marker_span(text).unwrap();
        assert_eq!(start, 2);
        assert_eq!(end, 4);
        assert_eq!(content, "line1\nline2\n");
    }

    #[test]
    fn test_extract_marker_span_strips_multiple_intermediate_markers() {
        let text = "<|marker_1|>\naaa\n<|marker_2|>\nbbb\n<|marker_3|>\nccc\n<|marker_4|>";
        let (start, end, content) = extract_marker_span(text).unwrap();
        assert_eq!(start, 1);
        assert_eq!(end, 4);
        assert_eq!(content, "aaa\nbbb\nccc\n");
    }

    #[test]
    fn test_apply_marker_span_with_extra_intermediate_marker() {
        let old = "aaa\nbbb\nccc\n";
        let output = "<|marker_1|>\naaa\n<|marker_1|>\nBBB\nccc\n<|marker_2|>";
        let result = apply_marker_span(old, output).unwrap();
        assert_eq!(result, "aaa\nBBB\nccc\n");
    }

    #[test]
    fn test_strip_marker_tags_inline() {
        assert_eq!(strip_marker_tags("no markers here"), "no markers here");
        assert_eq!(strip_marker_tags("before<|marker_5|>after"), "beforeafter");
        assert_eq!(
            strip_marker_tags("line1\n<|marker_3|>\nline2"),
            "line1\nline2"
        );
    }
}
