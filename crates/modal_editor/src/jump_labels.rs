use std::ops::Range;

use editor::display_map::{DisplayRow, DisplaySnapshot};
use editor::{
    Anchor, DisplayPoint, Editor, MultiBufferOffset, NavigationOverlayKey, NavigationOverlayLabel,
    NavigationTargetOverlay,
};
use gpui::{App, Context, Font, Hsla, Pixels, Window, WindowTextSystem, px};
use language::{Point, Selection};
use multi_buffer::MultiBufferSnapshot;
use text::Bias;
use theme::ActiveTheme as _;

enum HelixJumpNavigationOverlay {}

pub const HELIX_JUMP_OVERLAY_KEY: NavigationOverlayKey =
    NavigationOverlayKey::unique::<HelixJumpNavigationOverlay>();

#[derive(Clone, Debug, PartialEq)]
pub struct HelixJumpLabel {
    pub label: [char; 2],
    pub range: Range<Anchor>,
}

/// Labels the words visible in `editor` for jumping, nearest to the newest
/// selection first, along with the overlays that draw the labels.
pub fn collect_helix_jump_data(
    editor: &mut Editor,
    allow_targets_in_selection: bool,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> HelixJumpUiData {
    let snapshot = editor.snapshot(window, cx);
    let display_snapshot = &snapshot.display_snapshot;
    let buffer_snapshot = display_snapshot.buffer_snapshot();
    let visible_range = visible_jump_range(editor, &snapshot, display_snapshot, cx);
    let start_offset = buffer_snapshot.point_to_offset(visible_range.start);
    let end_offset = buffer_snapshot.point_to_offset(visible_range.end);

    let selections = editor.selections.all::<Point>(&display_snapshot);
    let skip_data =
        selection_skip_offsets(buffer_snapshot, &selections, allow_targets_in_selection);

    // Get the primary cursor position for alternating forward/backward labeling
    let cursor_offset = selections
        .first()
        .map(|s| buffer_snapshot.point_to_offset(s.head()))
        .unwrap_or(start_offset);

    let style = editor.style(cx);
    let font = style.text.font();
    let font_size = style.text.font_size.to_pixels(window.rem_size());
    let label_color = cx.theme().colors().vim_helix_jump_label_foreground;

    build_helix_jump_ui_data(
        buffer_snapshot,
        start_offset,
        end_offset,
        cursor_offset,
        label_color,
        &skip_data,
        window.text_system(),
        font,
        font_size,
    )
}

fn visible_jump_range(
    editor: &Editor,
    snapshot: &editor::EditorSnapshot,
    display_snapshot: &DisplaySnapshot,
    cx: &App,
) -> Range<Point> {
    let visible_range = editor.multi_buffer_visible_range(display_snapshot, cx);
    if editor.visible_line_count().is_some() || visible_range.start != visible_range.end {
        return visible_range;
    }

    let scroll_position = snapshot.scroll_position();
    let top_row = scroll_position.y.floor().max(0.0) as u32;
    let visible_rows = display_snapshot
        .max_point()
        .row()
        .0
        .saturating_sub(top_row)
        .saturating_add(1);
    let start_display_point = DisplayPoint::new(DisplayRow(top_row), 0);
    let end_display_point = DisplayPoint::new(DisplayRow(top_row.saturating_add(visible_rows)), 0);

    display_snapshot.display_point_to_point(start_display_point, Bias::Left)
        ..display_snapshot.display_point_to_point(end_display_point, Bias::Right)
}

pub fn build_helix_jump_ui_data(
    buffer: &MultiBufferSnapshot,
    start_offset: MultiBufferOffset,
    end_offset: MultiBufferOffset,
    cursor_offset: MultiBufferOffset,
    label_color: Hsla,
    skip_data: &HelixJumpSkipData,
    text_system: &WindowTextSystem,
    font: Font,
    font_size: Pixels,
) -> HelixJumpUiData {
    if start_offset >= end_offset {
        return HelixJumpUiData::default();
    }

    // First pass: collect all word candidates without assigning labels
    let candidates = collect_jump_candidates(buffer, start_offset, end_offset, skip_data);

    if candidates.is_empty() {
        return HelixJumpUiData::default();
    }

    let ordered_candidates = order_jump_candidates(candidates, cursor_offset);

    // Now assign labels and build UI data
    let mut labels = Vec::with_capacity(ordered_candidates.len());
    let mut overlays = Vec::with_capacity(ordered_candidates.len());

    let width_of = |text: &str| -> Pixels {
        if text.is_empty() {
            return px(0.0);
        }

        let run = gpui::TextRun {
            len: text.len(),
            font: font.clone(),
            color: Hsla::default(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };

        text_system.layout_line(text, font_size, &[run], None).width
    };

    let is_monospace = is_monospace_jump_font(text_system, &font, font_size);

    for (label_index, candidate) in ordered_candidates.into_iter().enumerate() {
        let start_anchor = buffer.anchor_after(candidate.word_start);
        let end_anchor = buffer.anchor_after(candidate.word_end);
        let label = jump_label_for_index(label_index);
        let label_text = label.iter().collect::<String>();
        // Monospace fonts: the label always matches the width of the first two characters,
        // so no per-word measurement is needed.
        // Proportional fonts: a label like "mw" can be wider than a short word like "if",
        // so we hide enough of the word (and possibly trailing whitespace) to make room,
        // or shift the label left into preceding whitespace.
        let fit = if is_monospace {
            JumpLabelFit::monospace(candidate.first_two_end)
        } else {
            let label_width = width_of(&label_text);
            fit_proportional_jump_label(buffer, &candidate, end_offset, label_width, &width_of)
        };

        let hide_end_anchor = buffer.anchor_after(fit.hide_end_offset);

        labels.push(HelixJumpLabel {
            label,
            range: start_anchor..end_anchor,
        });

        overlays.push(NavigationTargetOverlay {
            target_range: start_anchor..end_anchor,
            label: NavigationOverlayLabel {
                text: label_text.into(),
                text_color: label_color,
                x_offset: -fit.left_shift,
                scale_factor: fit.scale_factor,
            },
            covered_text_range: Some(start_anchor..hide_end_anchor),
        });
    }

    HelixJumpUiData { labels, overlays }
}

fn collect_jump_candidates(
    buffer: &MultiBufferSnapshot,
    start_offset: MultiBufferOffset,
    end_offset: MultiBufferOffset,
    skip_data: &HelixJumpSkipData,
) -> Vec<JumpCandidate> {
    let mut candidates = Vec::new();

    let mut offset = start_offset;
    let mut in_word = false;
    let mut word_start = start_offset;
    let mut first_two_end = start_offset;
    let mut char_count = 0;

    for chunk in buffer.text_for_range(start_offset..end_offset) {
        for (idx, ch) in chunk.char_indices() {
            let absolute = offset + idx;
            let is_word = is_jump_word_char(ch);
            if is_word {
                if !in_word {
                    in_word = true;
                    word_start = absolute;
                    char_count = 0;
                }
                if char_count == 1 {
                    first_two_end = absolute + ch.len_utf8();
                }
                char_count += 1;
            }

            if !is_word && in_word {
                if char_count >= 2 && !should_skip_jump_candidate(word_start, absolute, skip_data) {
                    candidates.push(JumpCandidate {
                        word_start,
                        word_end: absolute,
                        first_two_end,
                    });
                }
                in_word = false;
            }
        }
        offset += chunk.len();
    }

    // Handle word at end of buffer
    if in_word && char_count >= 2 && !should_skip_jump_candidate(word_start, end_offset, skip_data)
    {
        candidates.push(JumpCandidate {
            word_start,
            word_end: end_offset,
            first_two_end,
        });
    }

    candidates
}

pub fn selection_skip_offsets(
    buffer: &MultiBufferSnapshot,
    selections: &[Selection<Point>],
    allow_targets_in_selection: bool,
) -> HelixJumpSkipData {
    let mut skip_points = Vec::with_capacity(selections.len());
    let mut skip_ranges = Vec::new();

    for selection in selections {
        let head_offset = buffer.point_to_offset(selection.head());
        skip_points.push(head_offset);

        if !allow_targets_in_selection && selection.start != selection.end {
            let mut start = buffer.point_to_offset(selection.start);
            let mut end = buffer.point_to_offset(selection.end);
            if start > end {
                std::mem::swap(&mut start, &mut end);
            }
            skip_ranges.push(start..end);
        }
    }

    skip_points.sort_unstable();

    skip_ranges.sort_unstable_by_key(|range| range.start);
    let mut merged_ranges: Vec<Range<MultiBufferOffset>> = Vec::with_capacity(skip_ranges.len());
    for range in skip_ranges {
        if let Some(previous_range) = merged_ranges.last_mut()
            && range.start <= previous_range.end
        {
            previous_range.end = previous_range.end.max(range.end);
        } else {
            merged_ranges.push(range);
        }
    }

    HelixJumpSkipData {
        points: skip_points,
        ranges: merged_ranges,
    }
}

fn should_skip_jump_candidate(
    word_start: MultiBufferOffset,
    word_end: MultiBufferOffset,
    skip_data: &HelixJumpSkipData,
) -> bool {
    // word_end is exclusive, so points at the following delimiter should not skip the word.
    let point_index = skip_data
        .points
        .partition_point(|offset| *offset < word_start);
    if skip_data
        .points
        .get(point_index)
        .is_some_and(|offset| *offset < word_end)
    {
        return true;
    }

    let range_index = skip_data
        .ranges
        .partition_point(|range| range.end <= word_start);
    skip_data
        .ranges
        .get(range_index)
        .is_some_and(|range| range.start < word_end)
}

/// Interleave candidates so forward targets get even label indices (aa, ac, ae...)
/// and backward targets get odd indices (ab, ad, af...), matching Helix's algorithm.
/// This keeps the earliest label assignments close to the cursor in both directions.
fn order_jump_candidates(
    candidates: Vec<JumpCandidate>,
    cursor_offset: MultiBufferOffset,
) -> Vec<JumpCandidate> {
    let mut forward = Vec::with_capacity(candidates.len());
    let mut backward = Vec::new();

    for candidate in candidates {
        if candidate.word_start < cursor_offset {
            backward.push(candidate);
        } else {
            forward.push(candidate);
        }
    }

    backward.reverse();

    let mut ordered_candidates =
        Vec::with_capacity((forward.len() + backward.len()).min(HELIX_JUMP_LABEL_LIMIT));
    let mut forward_candidates = forward.into_iter();
    let mut backward_candidates = backward.into_iter();

    loop {
        let mut pushed_candidate = false;

        if ordered_candidates.len() < HELIX_JUMP_LABEL_LIMIT
            && let Some(candidate) = forward_candidates.next()
        {
            ordered_candidates.push(candidate);
            pushed_candidate = true;
        }

        if ordered_candidates.len() < HELIX_JUMP_LABEL_LIMIT
            && let Some(candidate) = backward_candidates.next()
        {
            ordered_candidates.push(candidate);
            pushed_candidate = true;
        }

        if !pushed_candidate {
            break;
        }
    }

    ordered_candidates
}

fn jump_label_for_index(index: usize) -> [char; 2] {
    [
        HELIX_JUMP_ALPHABET[index / HELIX_JUMP_ALPHABET.len()],
        HELIX_JUMP_ALPHABET[index % HELIX_JUMP_ALPHABET.len()],
    ]
}

fn is_monospace_jump_font(text_system: &WindowTextSystem, font: &Font, font_size: Pixels) -> bool {
    let font_id = text_system.resolve_font(font);
    let width_of_char = |ch| {
        text_system
            .advance(font_id, font_size, ch)
            .map(|size| size.width)
            .unwrap_or_else(|_| text_system.layout_width(font_id, font_size, ch))
    };

    let a = width_of_char('i');
    let b = width_of_char('w');
    let c = width_of_char('0');
    let d = width_of_char('1');
    let diff_1 = if a > b { a - b } else { b - a };
    let diff_2 = if c > d { c - d } else { d - c };
    diff_1 <= HELIX_JUMP_MONOSPACE_TOLERANCE && diff_2 <= HELIX_JUMP_MONOSPACE_TOLERANCE
}

/// Fit a jump label over a word in a proportional font.
///
/// Prefer fitting within the word itself, using available whitespace to the left
/// before consuming trailing whitespace after the word. If the label still cannot
/// fit cleanly, allow a small amount of scaling.
fn fit_proportional_jump_label<F: Fn(&str) -> Pixels>(
    buffer: &MultiBufferSnapshot,
    candidate: &JumpCandidate,
    end_offset: MultiBufferOffset,
    label_width: Pixels,
    width_of: &F,
) -> JumpLabelFit {
    let fit_budget = jump_label_fit_budget(buffer, candidate, end_offset, width_of);

    let mut hidden_prefix = HiddenPrefixFitState::new(candidate.first_two_end);
    let min_label_scale = if fit_budget.preserve_full_scale {
        1.0
    } else {
        HELIX_JUMP_MIN_LABEL_SCALE
    };

    hidden_prefix.extend_to_fit(
        buffer,
        candidate.word_start,
        candidate.word_end,
        candidate.word_end,
        label_width,
        fit_budget.max_left_shift,
        min_label_scale,
        width_of,
    );

    if label_width > px(0.0)
        && hidden_prefix.needs_more_width(label_width, fit_budget.max_left_shift)
        && fit_budget.allowed_trailing_hide_end > candidate.word_end
    {
        hidden_prefix.extend_to_fit(
            buffer,
            candidate.word_end,
            fit_budget.allowed_trailing_hide_end,
            candidate.word_end,
            label_width,
            fit_budget.max_left_shift,
            min_label_scale,
            width_of,
        );
    }

    // Jump candidates always contain at least two word characters, and the initial
    // scan above always measures through that second character before we read the width.
    let hidden_width = hidden_prefix.hidden_width;

    let left_shift = if label_width > hidden_width {
        (label_width - hidden_width).min(fit_budget.max_left_shift)
    } else {
        px(0.0)
    };

    let scale_factor = if label_width > px(0.0) {
        let scale = ((hidden_width + left_shift) / label_width).min(1.0);
        if scale < 1.0 { scale * 0.99 } else { 1.0 }
    } else {
        1.0
    };

    JumpLabelFit {
        hide_end_offset: hidden_prefix.hide_end_offset,
        left_shift,
        scale_factor: if fit_budget.preserve_full_scale {
            1.0
        } else {
            scale_factor
        },
    }
}

fn jump_label_fit_budget<F: Fn(&str) -> Pixels>(
    buffer: &MultiBufferSnapshot,
    candidate: &JumpCandidate,
    end_offset: MultiBufferOffset,
    width_of: &F,
) -> JumpLabelFitBudget {
    let mut left_ws_rev = String::new();
    let mut left_ws_count = 0usize;
    let mut left_stopped_at_line_break = false;
    let mut left_stopped_at_non_ws = false;
    let mut left_hit_limit = false;

    for ch in buffer.reversed_chars_at(candidate.word_start) {
        if ch == '\n' || ch == '\r' {
            left_stopped_at_line_break = true;
            break;
        }

        if !ch.is_whitespace() {
            left_stopped_at_non_ws = true;
            break;
        }

        left_ws_count += 1;
        if left_ws_count > HELIX_JUMP_MAX_LEFT_WS_CHARS {
            left_hit_limit = true;
            break;
        }

        left_ws_rev.push(ch);
    }

    let left_ws: String = left_ws_rev.chars().rev().collect();
    let left_ws_width = width_of(&left_ws);
    let left_is_indentation =
        left_stopped_at_line_break || (!left_stopped_at_non_ws && !left_hit_limit);
    // Between tokens leave a small gap so the label doesn't touch the previous word;
    // for line-leading indentation the full width is safe.
    let min_left_gap = if left_is_indentation {
        px(0.0)
    } else {
        px(2.0)
    };
    let max_left_shift = (left_ws_width - min_left_gap).max(px(0.0));

    let mut allowed_trailing_hide_end = candidate.word_end;
    let mut ws_count = 0usize;
    let mut last_ws_start = candidate.word_end;
    let mut ws_end_offset = candidate.word_end;
    let mut next_non_ws = None;
    let mut hit_line_break_after_word = false;

    let mut ws_scan_offset = candidate.word_end;
    'scan: for chunk in buffer.text_for_range(candidate.word_end..end_offset) {
        for (idx, ch) in chunk.char_indices() {
            let absolute = ws_scan_offset + idx;
            if ch == '\n' || ch == '\r' {
                hit_line_break_after_word = true;
                break 'scan;
            }
            if !ch.is_whitespace() {
                next_non_ws = Some(ch);
                break 'scan;
            }

            ws_count += 1;
            last_ws_start = absolute;
            ws_end_offset = absolute + ch.len_utf8();
        }
        ws_scan_offset += chunk.len();
    }

    let preserve_full_scale = hit_line_break_after_word && next_non_ws.is_none()
        || matches!(
            buffer.chars_at(candidate.word_end).next(),
            None | Some('\n') | Some('\r')
        );

    if ws_count > 0 {
        let next_is_word = match next_non_ws {
            Some(ch) => is_jump_word_char(ch),
            None => false,
        };

        if next_is_word {
            // Keep at least one whitespace character visible so adjacent labels
            // remain visually separated.
            if ws_count > 1 {
                allowed_trailing_hide_end = last_ws_start;
            }
        } else {
            // Next token is punctuation or end-of-range — safe to hide all whitespace.
            allowed_trailing_hide_end = ws_end_offset;
        }
    }

    JumpLabelFitBudget {
        max_left_shift,
        allowed_trailing_hide_end,
        preserve_full_scale,
    }
}
const HELIX_JUMP_ALPHABET: &[char; 26] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
pub const HELIX_JUMP_LABEL_LIMIT: usize = HELIX_JUMP_ALPHABET.len() * HELIX_JUMP_ALPHABET.len();
const HELIX_JUMP_MONOSPACE_TOLERANCE: Pixels = px(0.5);
const HELIX_JUMP_MIN_LABEL_SCALE: f32 = 1.0;
const HELIX_JUMP_MAX_HIDDEN_CHARS: usize = 16;
const HELIX_JUMP_MAX_LEFT_WS_CHARS: usize = 32;

fn is_jump_word_char(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

/// A word candidate for jump labels, before label assignment.
#[derive(Clone)]
struct JumpCandidate {
    word_start: MultiBufferOffset,
    word_end: MultiBufferOffset,
    first_two_end: MultiBufferOffset,
}

pub struct HelixJumpSkipData {
    points: Vec<MultiBufferOffset>,
    ranges: Vec<Range<MultiBufferOffset>>,
}

struct JumpLabelFit {
    hide_end_offset: MultiBufferOffset,
    left_shift: Pixels,
    scale_factor: f32,
}

struct JumpLabelFitBudget {
    max_left_shift: Pixels,
    allowed_trailing_hide_end: MultiBufferOffset,
    preserve_full_scale: bool,
}

struct HiddenPrefixFitState {
    text: String,
    hide_end_offset: MultiBufferOffset,
    hidden_width: Pixels,
    total_char_count: usize,
    word_char_count: usize,
}

impl JumpLabelFit {
    fn monospace(hide_end_offset: MultiBufferOffset) -> Self {
        Self {
            hide_end_offset,
            left_shift: px(0.0),
            scale_factor: 1.0,
        }
    }
}

impl HiddenPrefixFitState {
    fn new(hide_end_offset: MultiBufferOffset) -> Self {
        Self {
            text: String::new(),
            hide_end_offset,
            hidden_width: px(0.0),
            total_char_count: 0,
            word_char_count: 0,
        }
    }

    fn needs_more_width(&self, label_width: Pixels, max_left_shift: Pixels) -> bool {
        (self.hidden_width + max_left_shift) / label_width < HELIX_JUMP_MIN_LABEL_SCALE
    }

    fn extend_to_fit<F: Fn(&str) -> Pixels>(
        &mut self,
        buffer: &MultiBufferSnapshot,
        range_start: MultiBufferOffset,
        range_end: MultiBufferOffset,
        word_end: MultiBufferOffset,
        label_width: Pixels,
        max_left_shift: Pixels,
        min_label_scale: f32,
        width_of: &F,
    ) {
        let mut offset = range_start;
        for chunk in buffer.text_for_range(range_start..range_end) {
            for (idx, ch) in chunk.char_indices() {
                let absolute = offset + idx;

                self.total_char_count += 1;
                if self.total_char_count > HELIX_JUMP_MAX_HIDDEN_CHARS {
                    return;
                }

                self.text.push(ch);
                let end_offset = absolute + ch.len_utf8();

                if absolute < word_end && is_jump_word_char(ch) {
                    self.word_char_count += 1;
                }

                if self.word_char_count < 2 {
                    continue;
                }

                self.hide_end_offset = end_offset;
                self.hidden_width = width_of(self.text.as_str());

                let effective_width = self.hidden_width + max_left_shift;
                let scale_needed = if label_width > px(0.0) {
                    (effective_width / label_width).min(1.0)
                } else {
                    1.0
                };

                if scale_needed >= min_label_scale {
                    return;
                }
            }
            offset += chunk.len();
        }
    }
}

#[derive(Default)]
pub struct HelixJumpUiData {
    pub labels: Vec<HelixJumpLabel>,
    pub overlays: Vec<NavigationTargetOverlay>,
}
