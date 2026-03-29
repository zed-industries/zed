use anyhow::{Context as _, Result, anyhow};

pub const MARKER_TAG_PREFIX: &str = "<|marker_";
pub const MARKER_TAG_SUFFIX: &str = "|>";
pub const RELATIVE_MARKER_TAG_PREFIX: &str = "<|marker";
const V0316_MIN_BLOCK_LINES: usize = 3;
const V0316_MAX_BLOCK_LINES: usize = 8;
const V0318_MIN_BLOCK_LINES: usize = 6;
const V0318_MAX_BLOCK_LINES: usize = 16;
const MAX_NUDGE_LINES: usize = 5;
pub const V0316_END_MARKER: &str = "<[end▁of▁sentence]>";
pub const V0317_END_MARKER: &str = "<[end▁of▁sentence]>";
pub const V0318_END_MARKER: &str = "<[end▁of▁sentence]>";

pub fn marker_tag(number: usize) -> String {
    format!("{MARKER_TAG_PREFIX}{number}{MARKER_TAG_SUFFIX}")
}

pub fn marker_tag_relative(delta: isize) -> String {
    if delta > 0 {
        format!("<|marker+{delta}|>")
    } else if delta == 0 {
        String::from("<|marker-0|>")
    } else {
        format!("<|marker{delta}|>")
    }
}

struct LineInfo {
    start: usize,
    is_blank: bool,
    is_good_start: bool,
}

fn collect_line_info(text: &str) -> Vec<LineInfo> {
    let mut lines = Vec::new();
    let mut offset = 0;
    for line in text.split('\n') {
        let trimmed = line.trim();
        let is_blank = trimmed.is_empty();
        let is_good_start = !is_blank && !is_structural_tail(trimmed);
        lines.push(LineInfo {
            start: offset,
            is_blank,
            is_good_start,
        });
        offset += line.len() + 1;
    }
    // split('\n') on "abc\n" yields ["abc", ""] — drop the phantom trailing
    // empty element when the text ends with '\n'.
    if text.ends_with('\n') && lines.len() > 1 {
        lines.pop();
    }
    lines
}

fn is_structural_tail(trimmed_line: &str) -> bool {
    if trimmed_line.starts_with(&['}', ']', ')']) {
        return true;
    }
    matches!(
        trimmed_line.trim_end_matches(';'),
        "break" | "continue" | "return" | "throw" | "end"
    )
}

/// Starting from line `from`, scan up to `MAX_NUDGE_LINES` forward to find a
/// line with `is_good_start`. Returns `None` if no suitable line is found.
fn skip_to_good_start(lines: &[LineInfo], from: usize) -> Option<usize> {
    (from..lines.len().min(from + MAX_NUDGE_LINES)).find(|&i| lines[i].is_good_start)
}

/// Compute byte offsets within `editable_text` where marker boundaries should
/// be placed.
///
/// Returns a sorted `Vec<usize>` that always starts with `0` and ends with
/// `editable_text.len()`. Interior offsets are placed at line boundaries
/// (right after a `\n`), preferring blank-line boundaries when available and
/// respecting `min_block_lines` / `max_block_lines` constraints.
fn compute_marker_offsets_with_limits(
    editable_text: &str,
    min_block_lines: usize,
    max_block_lines: usize,
) -> Vec<usize> {
    if editable_text.is_empty() {
        return vec![0, 0];
    }

    let lines = collect_line_info(editable_text);
    let mut offsets = vec![0usize];
    let mut last_boundary_line = 0;
    let mut i = 0;

    while i < lines.len() {
        let gap = i - last_boundary_line;

        // Blank-line split: non-blank line following blank line(s) with enough
        // accumulated lines.
        if gap >= min_block_lines && !lines[i].is_blank && i > 0 && lines[i - 1].is_blank {
            let target = if lines[i].is_good_start {
                i
            } else {
                skip_to_good_start(&lines, i).unwrap_or(i)
            };
            if lines.len() - target >= min_block_lines
                && lines[target].start > *offsets.last().unwrap_or(&0)
            {
                offsets.push(lines[target].start);
                last_boundary_line = target;
                i = target + 1;
                continue;
            }
        }

        // Hard cap: too many lines without a split.
        if gap >= max_block_lines {
            let target = skip_to_good_start(&lines, i).unwrap_or(i);
            if lines[target].start > *offsets.last().unwrap_or(&0) {
                offsets.push(lines[target].start);
                last_boundary_line = target;
                i = target + 1;
                continue;
            }
        }

        i += 1;
    }

    let end = editable_text.len();
    if *offsets.last().unwrap_or(&0) != end {
        offsets.push(end);
    }

    offsets
}

/// Compute byte offsets within `editable_text` for the V0316/V0317 block sizing rules.
pub fn compute_marker_offsets(editable_text: &str) -> Vec<usize> {
    compute_marker_offsets_with_limits(editable_text, V0316_MIN_BLOCK_LINES, V0316_MAX_BLOCK_LINES)
}

pub fn compute_marker_offsets_v0318(editable_text: &str) -> Vec<usize> {
    compute_marker_offsets_with_limits(editable_text, V0318_MIN_BLOCK_LINES, V0318_MAX_BLOCK_LINES)
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
    let (common_prefix, common_suffix) =
        common_prefix_suffix(old_editable.as_bytes(), new_editable.as_bytes());
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

struct ParsedTag {
    value: isize,
    tag_start: usize,
    tag_end: usize,
}

fn collect_tags(text: &str, prefix: &str, parse: fn(&str) -> Option<isize>) -> Vec<ParsedTag> {
    let mut tags = Vec::new();
    let mut search_from = 0;
    while let Some(rel_pos) = text[search_from..].find(prefix) {
        let tag_start = search_from + rel_pos;
        let payload_start = tag_start + prefix.len();
        if let Some(suffix_rel) = text[payload_start..].find(MARKER_TAG_SUFFIX) {
            let payload_end = payload_start + suffix_rel;
            if let Some(value) = parse(&text[payload_start..payload_end]) {
                let tag_end = payload_end + MARKER_TAG_SUFFIX.len();
                tags.push(ParsedTag {
                    value,
                    tag_start,
                    tag_end,
                });
                search_from = tag_end;
                continue;
            }
        }
        search_from = tag_start + prefix.len();
    }
    tags
}

fn collect_marker_tags(text: &str) -> Vec<ParsedTag> {
    collect_tags(text, MARKER_TAG_PREFIX, |s| {
        s.parse::<usize>().ok().map(|n| n as isize)
    })
}

fn collect_relative_marker_tags(text: &str) -> Vec<ParsedTag> {
    collect_tags(text, RELATIVE_MARKER_TAG_PREFIX, |s| {
        s.parse::<isize>().ok()
    })
}

pub fn nearest_marker_number(cursor_offset: Option<usize>, marker_offsets: &[usize]) -> usize {
    let cursor = cursor_offset.unwrap_or(0);
    marker_offsets
        .iter()
        .enumerate()
        .min_by_key(|(_, offset)| (**offset as isize - cursor as isize).unsigned_abs())
        .map(|(idx, _)| idx + 1)
        .unwrap_or(1)
}

fn cursor_block_index(cursor_offset: Option<usize>, marker_offsets: &[usize]) -> usize {
    let cursor = cursor_offset.unwrap_or(0);
    marker_offsets
        .windows(2)
        .position(|window| cursor >= window[0] && cursor < window[1])
        .unwrap_or_else(|| marker_offsets.len().saturating_sub(2))
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

/// Map a byte offset from old span coordinates to new span coordinates,
/// using common prefix/suffix within the span for accuracy.
fn map_boundary_offset(
    old_rel: usize,
    old_span_len: usize,
    new_span_len: usize,
    span_common_prefix: usize,
    span_common_suffix: usize,
) -> usize {
    if old_rel <= span_common_prefix {
        old_rel
    } else if old_rel >= old_span_len - span_common_suffix {
        new_span_len - (old_span_len - old_rel)
    } else {
        let old_changed_start = span_common_prefix;
        let old_changed_len = old_span_len
            .saturating_sub(span_common_prefix)
            .saturating_sub(span_common_suffix);
        let new_changed_start = span_common_prefix;
        let new_changed_len = new_span_len
            .saturating_sub(span_common_prefix)
            .saturating_sub(span_common_suffix);

        if old_changed_len == 0 {
            new_changed_start
        } else {
            new_changed_start + ((old_rel - old_changed_start) * new_changed_len / old_changed_len)
        }
    }
}

fn snap_to_line_start(text: &str, offset: usize) -> usize {
    let bounded = offset.min(text.len());
    let bounded = text.floor_char_boundary(bounded);

    if bounded >= text.len() {
        return text.len();
    }

    if bounded == 0 || text.as_bytes().get(bounded - 1) == Some(&b'\n') {
        return bounded;
    }

    if let Some(next_nl_rel) = text[bounded..].find('\n') {
        let next = bounded + next_nl_rel + 1;
        return text.floor_char_boundary(next.min(text.len()));
    }

    let prev_start = text[..bounded].rfind('\n').map(|idx| idx + 1).unwrap_or(0);
    text.floor_char_boundary(prev_start)
}

/// Write the editable region content with byte-exact marker tags, inserting the
/// cursor marker at the given offset within the editable text.
///
/// The `tag_for_index` closure maps a boundary index to the marker tag string.
fn write_editable_with_markers_impl(
    output: &mut String,
    editable_text: &str,
    cursor_offset_in_editable: usize,
    cursor_marker: &str,
    marker_offsets: &[usize],
    tag_for_index: impl Fn(usize) -> String,
) {
    let mut cursor_placed = false;
    for (i, &offset) in marker_offsets.iter().enumerate() {
        output.push_str(&tag_for_index(i));

        if let Some(&next_offset) = marker_offsets.get(i + 1) {
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

pub fn write_editable_with_markers_v0316(
    output: &mut String,
    editable_text: &str,
    cursor_offset_in_editable: usize,
    cursor_marker: &str,
) {
    let marker_offsets = compute_marker_offsets(editable_text);
    write_editable_with_markers_impl(
        output,
        editable_text,
        cursor_offset_in_editable,
        cursor_marker,
        &marker_offsets,
        |i| marker_tag(i + 1),
    );
}

pub fn write_editable_with_markers_v0317(
    output: &mut String,
    editable_text: &str,
    cursor_offset_in_editable: usize,
    cursor_marker: &str,
) {
    let marker_offsets = compute_marker_offsets(editable_text);
    let anchor_idx = cursor_block_index(Some(cursor_offset_in_editable), &marker_offsets);
    write_editable_with_markers_impl(
        output,
        editable_text,
        cursor_offset_in_editable,
        cursor_marker,
        &marker_offsets,
        |i| marker_tag_relative(i as isize - anchor_idx as isize),
    );
}

pub fn write_editable_with_markers_v0318(
    output: &mut String,
    editable_text: &str,
    cursor_offset_in_editable: usize,
    cursor_marker: &str,
) {
    let marker_offsets = compute_marker_offsets_v0318(editable_text);
    write_editable_with_markers_impl(
        output,
        editable_text,
        cursor_offset_in_editable,
        cursor_marker,
        &marker_offsets,
        |i| marker_tag(i + 1),
    );
}

/// Parse byte-exact model output and reconstruct the full new editable region.
///
/// `resolve_boundary` maps a parsed tag value to an absolute byte offset in
/// old_editable, given the marker_offsets. Returns `(start_byte, end_byte)` or
/// an error.
fn apply_marker_span_impl(
    old_editable: &str,
    tags: &[ParsedTag],
    output: &str,
    resolve_boundaries: impl Fn(isize, isize) -> Result<(usize, usize)>,
) -> Result<String> {
    if tags.is_empty() {
        return Err(anyhow!("no marker tags found in output"));
    }
    if tags.len() == 1 {
        return Err(anyhow!(
            "only one marker tag found in output, expected at least two"
        ));
    }

    let start_value = tags[0].value;
    let end_value = tags[tags.len() - 1].value;

    if start_value == end_value {
        return Ok(old_editable.to_string());
    }

    let (start_byte, end_byte) = resolve_boundaries(start_value, end_value)?;

    if start_byte > end_byte {
        return Err(anyhow!("start marker must come before end marker"));
    }

    let mut new_content = String::new();
    for i in 0..tags.len() - 1 {
        let content_start = tags[i].tag_end;
        let content_end = tags[i + 1].tag_start;
        if content_start <= content_end {
            new_content.push_str(&output[content_start..content_end]);
        }
    }

    let mut result = String::new();
    result.push_str(&old_editable[..start_byte]);
    result.push_str(&new_content);
    result.push_str(&old_editable[end_byte..]);

    Ok(result)
}

pub fn apply_marker_span_v0316(old_editable: &str, output: &str) -> Result<String> {
    let tags = collect_marker_tags(output);

    // Validate monotonically increasing with no gaps (best-effort warning)
    if tags.len() >= 2 {
        let start_num = tags[0].value;
        let end_num = tags[tags.len() - 1].value;
        if start_num != end_num {
            let expected: Vec<isize> = (start_num..=end_num).collect();
            let actual: Vec<isize> = tags.iter().map(|t| t.value).collect();
            if actual != expected {
                eprintln!(
                    "V0316 marker sequence validation failed: expected {:?}, got {:?}. Attempting best-effort parse.",
                    expected, actual
                );
            }
        }
    }

    let marker_offsets = compute_marker_offsets(old_editable);
    apply_marker_span_impl(old_editable, &tags, output, |start_val, end_val| {
        let start_idx = (start_val as usize)
            .checked_sub(1)
            .context("marker numbers are 1-indexed")?;
        let end_idx = (end_val as usize)
            .checked_sub(1)
            .context("marker numbers are 1-indexed")?;
        let start_byte = *marker_offsets
            .get(start_idx)
            .context("start marker number out of range")?;
        let end_byte = *marker_offsets
            .get(end_idx)
            .context("end marker number out of range")?;
        Ok((start_byte, end_byte))
    })
}

pub fn apply_marker_span_v0317(
    old_editable: &str,
    output: &str,
    cursor_offset_in_old: Option<usize>,
) -> Result<String> {
    let tags = collect_relative_marker_tags(output);
    let marker_offsets = compute_marker_offsets(old_editable);
    let anchor_idx = cursor_block_index(cursor_offset_in_old, &marker_offsets);

    apply_marker_span_impl(old_editable, &tags, output, |start_delta, end_delta| {
        let start_idx_signed = anchor_idx as isize + start_delta;
        let end_idx_signed = anchor_idx as isize + end_delta;
        if start_idx_signed < 0 || end_idx_signed < 0 {
            return Err(anyhow!("relative marker maps before first marker"));
        }
        let start_idx = usize::try_from(start_idx_signed).context("invalid start marker index")?;
        let end_idx = usize::try_from(end_idx_signed).context("invalid end marker index")?;
        let start_byte = *marker_offsets
            .get(start_idx)
            .context("start marker number out of range")?;
        let end_byte = *marker_offsets
            .get(end_idx)
            .context("end marker number out of range")?;
        Ok((start_byte, end_byte))
    })
}

pub fn apply_marker_span_v0318(old_editable: &str, output: &str) -> Result<String> {
    let tags = collect_marker_tags(output);

    if tags.len() >= 2 {
        let start_num = tags[0].value;
        let end_num = tags[tags.len() - 1].value;
        if start_num != end_num {
            let expected: Vec<isize> = (start_num..=end_num).collect();
            let actual: Vec<isize> = tags.iter().map(|t| t.value).collect();
            if actual != expected {
                eprintln!(
                    "V0318 marker sequence validation failed: expected {:?}, got {:?}. Attempting best-effort parse.",
                    expected, actual
                );
            }
        }
    }

    let marker_offsets = compute_marker_offsets_v0318(old_editable);
    apply_marker_span_impl(old_editable, &tags, output, |start_val, end_val| {
        let start_idx = (start_val as usize)
            .checked_sub(1)
            .context("marker numbers are 1-indexed")?;
        let end_idx = (end_val as usize)
            .checked_sub(1)
            .context("marker numbers are 1-indexed")?;
        let start_byte = *marker_offsets
            .get(start_idx)
            .context("start marker number out of range")?;
        let end_byte = *marker_offsets
            .get(end_idx)
            .context("end marker number out of range")?;
        Ok((start_byte, end_byte))
    })
}

/// Encode the training target from old and new editable text.
///
/// Shared implementation for V0316, V0317, and V0318. The `tag_for_block_idx`
/// closure maps a block index to the appropriate marker tag string.
/// `no_edit_tag` is the marker tag to repeat when there are no edits.
fn encode_from_old_and_new_impl(
    old_editable: &str,
    new_editable: &str,
    cursor_offset_in_new: Option<usize>,
    cursor_marker: &str,
    end_marker: &str,
    no_edit_tag: &str,
    marker_offsets: &[usize],
    tag_for_block_idx: impl Fn(usize) -> String,
) -> Result<String> {
    if old_editable == new_editable {
        return Ok(format!("{no_edit_tag}{no_edit_tag}{end_marker}"));
    }

    let (common_prefix, common_suffix) =
        common_prefix_suffix(old_editable.as_bytes(), new_editable.as_bytes());
    let change_end_in_old = old_editable.len() - common_suffix;

    let mut start_marker_idx = marker_offsets
        .iter()
        .rposition(|&offset| offset <= common_prefix)
        .unwrap_or(0);
    let mut end_marker_idx = marker_offsets
        .iter()
        .position(|&offset| offset >= change_end_in_old)
        .unwrap_or(marker_offsets.len() - 1);

    if start_marker_idx == end_marker_idx {
        if end_marker_idx < marker_offsets.len().saturating_sub(1) {
            end_marker_idx += 1;
        } else if start_marker_idx > 0 {
            start_marker_idx -= 1;
        }
    }

    let old_start = marker_offsets[start_marker_idx];
    let old_end = marker_offsets[end_marker_idx];

    let new_start = old_start;
    let new_end = new_editable
        .len()
        .saturating_sub(old_editable.len().saturating_sub(old_end));

    let new_span = &new_editable[new_start..new_end];
    let old_span = &old_editable[old_start..old_end];

    let (span_common_prefix, span_common_suffix) =
        common_prefix_suffix(old_span.as_bytes(), new_span.as_bytes());

    let mut result = String::new();
    let mut prev_new_rel = 0usize;
    let mut cursor_placed = false;

    for block_idx in start_marker_idx..end_marker_idx {
        result.push_str(&tag_for_block_idx(block_idx));

        let new_rel_end = if block_idx + 1 == end_marker_idx {
            new_span.len()
        } else {
            let old_rel = marker_offsets[block_idx + 1] - old_start;
            let mapped = map_boundary_offset(
                old_rel,
                old_span.len(),
                new_span.len(),
                span_common_prefix,
                span_common_suffix,
            );
            snap_to_line_start(new_span, mapped)
        };

        let new_rel_end = new_rel_end.max(prev_new_rel);
        let block_content = &new_span[prev_new_rel..new_rel_end];

        if !cursor_placed {
            if let Some(cursor_offset) = cursor_offset_in_new {
                let abs_start = new_start + prev_new_rel;
                let abs_end = new_start + new_rel_end;
                if cursor_offset >= abs_start && cursor_offset <= abs_end {
                    cursor_placed = true;
                    let cursor_in_block = cursor_offset - abs_start;
                    let bounded = cursor_in_block.min(block_content.len());
                    result.push_str(&block_content[..bounded]);
                    result.push_str(cursor_marker);
                    result.push_str(&block_content[bounded..]);
                    prev_new_rel = new_rel_end;
                    continue;
                }
            }
        }

        result.push_str(block_content);
        prev_new_rel = new_rel_end;
    }

    result.push_str(&tag_for_block_idx(end_marker_idx));
    result.push_str(end_marker);

    Ok(result)
}

pub fn encode_from_old_and_new_v0316(
    old_editable: &str,
    new_editable: &str,
    cursor_offset_in_new: Option<usize>,
    cursor_marker: &str,
    end_marker: &str,
) -> Result<String> {
    let marker_offsets = compute_marker_offsets(old_editable);
    let no_edit_tag = marker_tag(nearest_marker_number(cursor_offset_in_new, &marker_offsets));
    encode_from_old_and_new_impl(
        old_editable,
        new_editable,
        cursor_offset_in_new,
        cursor_marker,
        end_marker,
        &no_edit_tag,
        &marker_offsets,
        |block_idx| marker_tag(block_idx + 1),
    )
}

pub fn encode_from_old_and_new_v0317(
    old_editable: &str,
    new_editable: &str,
    cursor_offset_in_new: Option<usize>,
    cursor_marker: &str,
    end_marker: &str,
) -> Result<String> {
    let marker_offsets = compute_marker_offsets(old_editable);
    let anchor_idx = cursor_block_index(cursor_offset_in_new, &marker_offsets);
    let no_edit_tag = marker_tag_relative(0);
    encode_from_old_and_new_impl(
        old_editable,
        new_editable,
        cursor_offset_in_new,
        cursor_marker,
        end_marker,
        &no_edit_tag,
        &marker_offsets,
        |block_idx| marker_tag_relative(block_idx as isize - anchor_idx as isize),
    )
}

pub fn encode_from_old_and_new_v0318(
    old_editable: &str,
    new_editable: &str,
    cursor_offset_in_new: Option<usize>,
    cursor_marker: &str,
    end_marker: &str,
) -> Result<String> {
    let marker_offsets = compute_marker_offsets_v0318(old_editable);
    let no_edit_tag = marker_tag(nearest_marker_number(cursor_offset_in_new, &marker_offsets));
    encode_from_old_and_new_impl(
        old_editable,
        new_editable,
        cursor_offset_in_new,
        cursor_marker,
        end_marker,
        &no_edit_tag,
        &marker_offsets,
        |block_idx| marker_tag(block_idx + 1),
    )
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
    fn test_compute_marker_offsets_blank_line_split_overrides_pending_hard_cap_boundary() {
        let text = "\
class OCRDataframe(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    df: pl.DataFrame

    def page(self, page_number: int = 0) -> \"OCRDataframe\":
        # Filter dataframe on specific page
        df_page = self.df.filter(pl.col(\"page\") == page_number)
        return OCRDataframe(df=df_page)

    def get_text_cell(
        self,
        cell: Cell,
        margin: int = 0,
        page_number: Optional[int] = None,
        min_confidence: int = 50,
    ) -> Optional[str]:
        \"\"\"
        Get text corresponding to cell
";
        let offsets = compute_marker_offsets(text);

        let def_start = text
            .find("    def get_text_cell(")
            .expect("def line exists");
        let self_start = text.find("        self,").expect("self line exists");

        assert!(
            offsets.contains(&def_start),
            "expected boundary at def line start ({def_start}), got {offsets:?}"
        );
        assert!(
            !offsets.contains(&self_start),
            "did not expect boundary at self line start ({self_start}), got {offsets:?}"
        );
    }

    #[test]
    fn test_compute_marker_offsets_blank_line_split_skips_closer_line() {
        let text = "\
impl Plugin for AhoySchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            self.schedule,
            (
                AhoySystems::MoveCharacters,
                AhoySystems::ApplyForcesToDynamicRigidBodies,
            )
                .chain()
                .before(PhysicsSystems::First),
        );

    }
}

/// System set used by all systems of `bevy_ahoy`.
#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AhoySystems {
    MoveCharacters,
    ApplyForcesToDynamicRigidBodies,
}
";
        let offsets = compute_marker_offsets(text);

        let closer_start = text.find("    }\n").expect("closer line exists");
        let doc_start = text
            .find("/// System set used by all systems of `bevy_ahoy`.")
            .expect("doc line exists");

        assert!(
            !offsets.contains(&closer_start),
            "did not expect boundary at closer line start ({closer_start}), got {offsets:?}"
        );
        assert!(
            offsets.contains(&doc_start),
            "expected boundary at doc line start ({doc_start}), got {offsets:?}"
        );
    }

    #[test]
    fn test_compute_marker_offsets_max_lines_split() {
        let text = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n";
        let offsets = compute_marker_offsets(text);
        assert!(offsets.len() >= 3, "offsets: {:?}", offsets);
    }

    #[test]
    fn test_compute_marker_offsets_hard_cap_nudges_past_closer_to_case_line() {
        let text = "a1\na2\na3\na4\na5\na6\na7\na8\n}\ncase 'x': {\nbody\n";
        let offsets = compute_marker_offsets(text);

        let expected = text.find("case 'x': {").expect("case line exists");
        assert!(
            offsets.contains(&expected),
            "expected nudged boundary at case line start ({expected}), got {offsets:?}"
        );
    }

    #[test]
    fn test_compute_marker_offsets_hard_cap_nudge_respects_max_forward_lines() {
        let text = "a1\na2\na3\na4\na5\na6\na7\na8\n}\n}\n}\n}\n}\ncase 'x': {\nbody\n";
        let offsets = compute_marker_offsets(text);

        let case_start = text.find("case 'x': {").expect("case line exists");
        assert!(
            !offsets.contains(&case_start),
            "boundary should not nudge beyond max forward lines; offsets: {offsets:?}"
        );
    }

    #[test]
    fn test_compute_marker_offsets_stay_sorted_when_hard_cap_boundary_nudges_forward() {
        let text = "\
aaaaaaaaaa = 1;
bbbbbbbbbb = 2;
cccccccccc = 3;
dddddddddd = 4;
eeeeeeeeee = 5;
ffffffffff = 6;
gggggggggg = 7;
hhhhhhhhhh = 8;
          };
        };

        grafanaDashboards = {
          cluster-overview.spec = {
            inherit instanceSelector;
            folderRef = \"infrastructure\";
            json = builtins.readFile ./grafana/dashboards/cluster-overview.json;
          };
        };
";
        let offsets = compute_marker_offsets(text);

        assert_eq!(offsets.first().copied(), Some(0), "offsets: {offsets:?}");
        assert_eq!(
            offsets.last().copied(),
            Some(text.len()),
            "offsets: {offsets:?}"
        );
        assert!(
            offsets.windows(2).all(|window| window[0] <= window[1]),
            "offsets must be sorted: {offsets:?}"
        );
    }

    #[test]
    fn test_compute_marker_offsets_empty() {
        let offsets = compute_marker_offsets("");
        assert_eq!(offsets, vec![0, 0]);
    }

    #[test]
    fn test_compute_marker_offsets_avoid_short_markdown_blocks() {
        let text = "\
# Spree Posts

This is a Posts extension for [Spree Commerce](https://spreecommerce.org), built with Ruby on Rails.

## Installation

1. Add this extension to your Gemfile with this line:

    ```ruby
    bundle add spree_posts
    ```

2. Run the install generator

    ```ruby
    bundle exec rails g spree_posts:install
    ```

3. Restart your server

  If your server was running, restart it so that it can find the assets properly.

## Developing

1. Create a dummy app

    ```bash
    bundle update
    bundle exec rake test_app
    ```

2. Add your new code
3. Run tests

    ```bash
    bundle exec rspec
    ```

When testing your applications integration with this extension you may use it's factories.
Simply add this require statement to your spec_helper:

```ruby
require 'spree_posts/factories'
```

## Releasing a new version

```shell
bundle exec gem bump -p -t
bundle exec gem release
```

For more options please see [gem-release README](https://github.com/svenfuchs/gem-release)

## Contributing

If you'd like to contribute, please take a look at the contributing guide.
";
        let offsets = compute_marker_offsets(text);

        assert_eq!(offsets.first().copied(), Some(0), "offsets: {offsets:?}");
        assert_eq!(
            offsets.last().copied(),
            Some(text.len()),
            "offsets: {offsets:?}"
        );

        for window in offsets.windows(2) {
            let block = &text[window[0]..window[1]];
            let line_count = block.lines().count();
            assert!(
                line_count >= V0316_MIN_BLOCK_LINES,
                "block too short: {line_count} lines in block {block:?} with offsets {offsets:?}"
            );
        }
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

    #[test]
    fn test_write_editable_with_markers_v0316_byte_exact() {
        let editable = "aaa\nbbb\nccc\n";
        let mut output = String::new();
        write_editable_with_markers_v0316(&mut output, editable, 4, "<|user_cursor|>");
        assert!(output.starts_with("<|marker_1|>"));
        assert!(output.contains("<|user_cursor|>"));
        let stripped = output.replace("<|user_cursor|>", "");
        let stripped = strip_marker_tags(&stripped);
        assert_eq!(stripped, editable);
    }

    #[test]
    fn test_apply_marker_span_v0316_basic() {
        let old = "aaa\nbbb\nccc\n";
        let output = "<|marker_1|>aaa\nBBB\nccc\n<|marker_2|>";
        let result = apply_marker_span_v0316(old, output).unwrap();
        assert_eq!(result, "aaa\nBBB\nccc\n");
    }

    #[test]
    fn test_apply_marker_span_v0316_no_edit() {
        let old = "aaa\nbbb\nccc\n";
        let output = "<|marker_1|><|marker_1|>";
        let result = apply_marker_span_v0316(old, output).unwrap();
        assert_eq!(result, old);
    }

    #[test]
    fn test_apply_marker_span_v0316_no_edit_any_marker() {
        let old = "aaa\nbbb\nccc\n";
        let output = "<|marker_2|>ignored content<|marker_2|>";
        let result = apply_marker_span_v0316(old, output).unwrap();
        assert_eq!(result, old);
    }

    #[test]
    fn test_apply_marker_span_v0316_multi_block() {
        let old = "line1\nline2\nline3\n\nline5\nline6\nline7\nline8\n";
        let marker_offsets = compute_marker_offsets(old);
        assert!(
            marker_offsets.len() >= 3,
            "expected at least 3 offsets, got {:?}",
            marker_offsets
        );

        let new_content = "LINE1\nLINE2\nLINE3\n\nLINE5\nLINE6\nLINE7\nLINE8\n";
        let mut output = String::new();
        output.push_str("<|marker_1|>");
        for i in 0..marker_offsets.len() - 1 {
            if i > 0 {
                output.push_str(&marker_tag(i + 1));
            }
            let start = marker_offsets[i];
            let end = marker_offsets[i + 1];
            let block_len = end - start;
            output.push_str(&new_content[start..start + block_len]);
        }
        let last_marker_num = marker_offsets.len();
        output.push_str(&marker_tag(last_marker_num));
        let result = apply_marker_span_v0316(old, &output).unwrap();
        assert_eq!(result, new_content);
    }

    #[test]
    fn test_apply_marker_span_v0316_byte_exact_no_normalization() {
        let old = "aaa\nbbb\nccc\n";
        let output = "<|marker_1|>aaa\nBBB\nccc<|marker_2|>";
        let result = apply_marker_span_v0316(old, output).unwrap();
        assert_eq!(result, "aaa\nBBB\nccc");
    }

    #[test]
    fn test_encode_v0316_no_edits() {
        let old = "aaa\nbbb\nccc\n";
        let result =
            encode_from_old_and_new_v0316(old, old, Some(5), "<|user_cursor|>", "<|end|>").unwrap();
        assert!(result.ends_with("<|end|>"));
        let stripped = result.strip_suffix("<|end|>").unwrap();
        let result_parsed = apply_marker_span_v0316(old, stripped).unwrap();
        assert_eq!(result_parsed, old);
    }

    #[test]
    fn test_encode_v0316_with_change() {
        let old = "aaa\nbbb\nccc\n";
        let new = "aaa\nBBB\nccc\n";
        let result =
            encode_from_old_and_new_v0316(old, new, None, "<|user_cursor|>", "<|end|>").unwrap();
        assert!(result.contains("<|marker_1|>"));
        assert!(result.contains("<|marker_2|>"));
        assert!(result.ends_with("<|end|>"));
    }

    #[test]
    fn test_roundtrip_v0316() {
        let old = "line1\nline2\nline3\n\nline5\nline6\nline7\nline8\nline9\nline10\n";
        let new = "line1\nline2\nline3\n\nline5\nLINE6\nline7\nline8\nline9\nline10\n";
        let encoded =
            encode_from_old_and_new_v0316(old, new, None, "<|user_cursor|>", "<|end|>").unwrap();
        let stripped = encoded
            .strip_suffix("<|end|>")
            .expect("should have end marker");
        let reconstructed = apply_marker_span_v0316(old, stripped).unwrap();
        assert_eq!(reconstructed, new);
    }

    #[test]
    fn test_roundtrip_v0316_with_cursor() {
        let old = "aaa\nbbb\nccc\n";
        let new = "aaa\nBBB\nccc\n";
        let result =
            encode_from_old_and_new_v0316(old, new, Some(5), "<|user_cursor|>", "<|end|>").unwrap();
        assert!(result.contains("<|user_cursor|>"), "result: {result}");
        assert!(result.contains("B<|user_cursor|>BB"), "result: {result}");
    }

    #[test]
    fn test_roundtrip_v0316_multi_block_change() {
        let old = "line1\nline2\nline3\n\nline5\nline6\nline7\nline8\n";
        let new = "line1\nLINE2\nline3\n\nline5\nLINE6\nline7\nline8\n";
        let encoded =
            encode_from_old_and_new_v0316(old, new, None, "<|user_cursor|>", "<|end|>").unwrap();
        let stripped = encoded
            .strip_suffix("<|end|>")
            .expect("should have end marker");
        let reconstructed = apply_marker_span_v0316(old, stripped).unwrap();
        assert_eq!(reconstructed, new);
    }

    #[test]
    fn test_nearest_marker_number() {
        let offsets = vec![0, 10, 20, 30];
        assert_eq!(nearest_marker_number(Some(0), &offsets), 1);
        assert_eq!(nearest_marker_number(Some(9), &offsets), 2);
        assert_eq!(nearest_marker_number(Some(15), &offsets), 2);
        assert_eq!(nearest_marker_number(Some(25), &offsets), 3);
        assert_eq!(nearest_marker_number(Some(30), &offsets), 4);
        assert_eq!(nearest_marker_number(None, &offsets), 1);
    }

    #[test]
    fn test_marker_tag_relative_formats_as_expected() {
        assert_eq!(marker_tag_relative(-2), "<|marker-2|>");
        assert_eq!(marker_tag_relative(-1), "<|marker-1|>");
        assert_eq!(marker_tag_relative(0), "<|marker-0|>");
        assert_eq!(marker_tag_relative(1), "<|marker+1|>");
        assert_eq!(marker_tag_relative(2), "<|marker+2|>");
    }

    #[test]
    fn test_write_editable_with_markers_v0317_includes_relative_markers_and_cursor() {
        let editable = "aaa\nbbb\nccc\n";
        let mut output = String::new();
        write_editable_with_markers_v0317(&mut output, editable, 4, "<|user_cursor|>");

        assert!(output.contains("<|marker-0|>"));
        assert!(output.contains("<|user_cursor|>"));

        let stripped = output.replace("<|user_cursor|>", "");
        let stripped =
            collect_relative_marker_tags(&stripped)
                .iter()
                .fold(stripped.clone(), |acc, marker| {
                    let tag = &stripped[marker.tag_start..marker.tag_end];
                    acc.replace(tag, "")
                });
        assert_eq!(stripped, editable);
    }

    #[test]
    fn test_apply_marker_span_v0317_basic() {
        let old = "aaa\nbbb\nccc\n";
        let output = "<|marker-0|>aaa\nBBB\nccc\n<|marker+1|>";
        let result = apply_marker_span_v0317(old, output, Some(0)).unwrap();
        assert_eq!(result, "aaa\nBBB\nccc\n");
    }

    #[test]
    fn test_apply_marker_span_v0317_no_edit() {
        let old = "aaa\nbbb\nccc\n";
        let output = "<|marker-0|><|marker-0|>";
        let result = apply_marker_span_v0317(old, output, Some(0)).unwrap();
        assert_eq!(result, old);
    }

    #[test]
    fn test_encode_v0317_no_edits() {
        let old = "aaa\nbbb\nccc\n";
        let result =
            encode_from_old_and_new_v0317(old, old, Some(5), "<|user_cursor|>", "<|end|>").unwrap();
        assert_eq!(result, "<|marker-0|><|marker-0|><|end|>");
    }

    #[test]
    fn test_roundtrip_v0317() {
        let old = "line1\nline2\nline3\n\nline5\nline6\nline7\nline8\n";
        let new = "line1\nLINE2\nline3\n\nline5\nLINE6\nline7\nline8\n";
        let cursor = Some(6);

        let encoded =
            encode_from_old_and_new_v0317(old, new, cursor, "<|user_cursor|>", "<|end|>").unwrap();
        let stripped = encoded
            .strip_suffix("<|end|>")
            .expect("should have end marker");
        let stripped = stripped.replace("<|user_cursor|>", "");
        let reconstructed = apply_marker_span_v0317(old, &stripped, cursor).unwrap();
        assert_eq!(reconstructed, new);
    }

    #[test]
    fn test_roundtrip_v0317_with_cursor_marker() {
        let old = "aaa\nbbb\nccc\n";
        let new = "aaa\nBBB\nccc\n";
        let result =
            encode_from_old_and_new_v0317(old, new, Some(5), "<|user_cursor|>", "<|end|>").unwrap();
        assert!(result.contains("<|user_cursor|>"), "result: {result}");
        assert!(result.contains("<|marker-0|>"), "result: {result}");
    }

    #[test]
    fn test_compute_marker_offsets_v0318_uses_larger_block_sizes() {
        let text = "l1\nl2\nl3\n\nl5\nl6\nl7\nl8\nl9\nl10\nl11\nl12\nl13\n";
        let v0316_offsets = compute_marker_offsets(text);
        let v0318_offsets = compute_marker_offsets_v0318(text);

        assert!(v0318_offsets.len() < v0316_offsets.len());
        assert_eq!(v0316_offsets.first().copied(), Some(0));
        assert_eq!(v0318_offsets.first().copied(), Some(0));
        assert_eq!(v0316_offsets.last().copied(), Some(text.len()));
        assert_eq!(v0318_offsets.last().copied(), Some(text.len()));
    }

    #[test]
    fn test_roundtrip_v0318() {
        let old = "line1\nline2\nline3\n\nline5\nline6\nline7\nline8\nline9\nline10\n";
        let new = "line1\nline2\nline3\n\nline5\nLINE6\nline7\nline8\nline9\nline10\n";
        let encoded =
            encode_from_old_and_new_v0318(old, new, None, "<|user_cursor|>", "<|end|>").unwrap();
        let stripped = encoded
            .strip_suffix("<|end|>")
            .expect("should have end marker");
        let reconstructed = apply_marker_span_v0318(old, stripped).unwrap();
        assert_eq!(reconstructed, new);
    }

    #[test]
    fn test_roundtrip_v0318_append_at_end_of_editable_region() {
        let old = "line1\nline2\nline3\n";
        let new = "line1\nline2\nline3\nline4\n";
        let encoded =
            encode_from_old_and_new_v0318(old, new, None, "<|user_cursor|>", "<|end|>").unwrap();

        assert_ne!(encoded, "<|marker_2|><|end|>");

        let stripped = encoded
            .strip_suffix("<|end|>")
            .expect("should have end marker");
        let reconstructed = apply_marker_span_v0318(old, stripped).unwrap();
        assert_eq!(reconstructed, new);
    }

    #[test]
    fn test_roundtrip_v0318_insert_at_internal_marker_boundary() {
        let old = "alpha\nbeta\n\ngamma\ndelta\n";
        let new = "alpha\nbeta\n\ninserted\ngamma\ndelta\n";
        let encoded =
            encode_from_old_and_new_v0318(old, new, None, "<|user_cursor|>", "<|end|>").unwrap();

        let stripped = encoded
            .strip_suffix("<|end|>")
            .expect("should have end marker");
        let reconstructed = apply_marker_span_v0318(old, stripped).unwrap();
        assert_eq!(reconstructed, new);
    }

    #[test]
    fn test_encode_v0317_markers_stay_on_line_boundaries() {
        let old = "\
\t\t\t\tcontinue outer;
\t\t\t}
\t\t}
\t}

\tconst intersectionObserver = new IntersectionObserver((entries) => {
\t\tfor (const entry of entries) {
\t\t\tif (entry.isIntersecting) {
\t\t\t\tintersectionObserver.unobserve(entry.target);
\t\t\t\tanchorPreload(/** @type {HTMLAnchorElement} */ (entry.target));
\t\t\t}
\t\t}
\t});

\tconst observer = new MutationObserver(() => {
\t\tconst links = /** @type {NodeListOf<HTMLAnchorElement>} */ (
\t\t\tdocument.querySelectorAll('a[data-preload]')
\t\t);

\t\tfor (const link of links) {
\t\t\tif (linkSet.has(link)) continue;
\t\t\tlinkSet.add(link);

\t\t\tswitch (link.dataset.preload) {
\t\t\t\tcase '':
\t\t\t\tcase 'true':
\t\t\t\tcase 'hover': {
\t\t\t\t\tlink.addEventListener('mouseenter', function callback() {
\t\t\t\t\t\tlink.removeEventListener('mouseenter', callback);
\t\t\t\t\t\tanchorPreload(link);
\t\t\t\t\t});
";
        let new = old.replacen(
            "\t\t\t\tcase 'true':\n",
            "\t\t\t\tcase 'TRUE':<|user_cursor|>\n",
            1,
        );

        let cursor_offset = new.find("<|user_cursor|>").expect("cursor marker in new");
        let new_without_cursor = new.replace("<|user_cursor|>", "");

        let encoded = encode_from_old_and_new_v0317(
            old,
            &new_without_cursor,
            Some(cursor_offset),
            "<|user_cursor|>",
            "<|end|>",
        )
        .unwrap();

        let core = encoded.strip_suffix("<|end|>").unwrap_or(&encoded);
        for marker in collect_relative_marker_tags(core) {
            let tag_start = marker.tag_start;
            assert!(
                tag_start == 0 || core.as_bytes()[tag_start - 1] == b'\n',
                "marker not at line boundary: {} in output:\n{}",
                marker_tag_relative(marker.value),
                core
            );
        }
    }
}
