use std::collections::VecDeque;
use std::ops::Range;

use collections::HashMap;
use editor::display_map::HighlightKey;
use editor::{
    Anchor, Bias, Editor, MultiBufferOffset, NavigationOverlayKey, NavigationOverlayLabel,
    NavigationTargetOverlay, ToOffset,
};
use gpui::{Context, Font, Hsla, Pixels, Window, WindowTextSystem, actions};
use language::Point;
use multi_buffer::MultiBufferSnapshot;
use settings::Settings;
use ui::px;

use crate::{
    ClearOperators, Vim, VimSettings,
    motion::{self, Motion},
    state::{FlashJumpLabel, Operator},
};

actions!(
    vim,
    [
        /// Starts a flash-style jump: type a search pattern, then press a
        /// highlighted label character to jump to that match.
        PushFlash,
        /// Removes the last character from the flash jump search pattern.
        FlashBackspace,
    ]
);

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::push_flash);
    Vim::action(editor, cx, Vim::flash_backspace);
}

enum FlashJumpNavigationOverlay {}

const FLASH_JUMP_OVERLAY_KEY: NavigationOverlayKey =
    NavigationOverlayKey::unique::<FlashJumpNavigationOverlay>();

// flash.nvim's default label alphabet: home row first, never uppercase, so
// uppercase input is unambiguously a pattern character.
const FLASH_JUMP_ALPHABET: &[char] = &[
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p',
    'z', 'x', 'c', 'v', 'b', 'n', 'm',
];

// The visible range is normally a small viewport, but it can span an entire
// minified line, or the rest of the buffer when the editor has not been laid
// out yet, and the scan runs on the foreground thread on every keystroke.
const FLASH_JUMP_MAX_SEARCH_BYTES: usize = 256 * 1024;
const FLASH_JUMP_MAX_MATCHES: usize = 4096;

#[derive(Default)]
struct FlashJumpUiData {
    labels: Vec<FlashJumpLabel>,
    target: Option<Range<Anchor>>,
    overlays: Vec<NavigationTargetOverlay>,
    match_ranges: Vec<Range<Anchor>>,
}

struct FlashMatch {
    range: Range<MultiBufferOffset>,
    /// The character right after the match, captured during the scan so the
    /// label-conflict check doesn't need a buffer seek per match.
    next_char: Option<char>,
}

impl Vim {
    fn push_flash(&mut self, _: &PushFlash, window: &mut Window, cx: &mut Context<Self>) {
        self.push_operator(
            Operator::FlashJump {
                pattern: String::new(),
                labels: Vec::new(),
                target: None,
            },
            window,
            cx,
        );
    }

    fn flash_backspace(&mut self, _: &FlashBackspace, window: &mut Window, cx: &mut Context<Self>) {
        let Some(Operator::FlashJump {
            mut pattern,
            labels,
            ..
        }) = self.active_operator()
        else {
            return;
        };
        self.pop_operator(window, cx);
        pattern.pop();
        self.update_flash_state(pattern, labels, window, cx);
    }

    pub(crate) fn handle_flash_jump_input(
        &mut self,
        operator: Operator,
        input_char: char,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Operator::FlashJump {
            mut pattern,
            labels,
            target,
        } = operator
        else {
            return;
        };
        self.pop_operator(window, cx);

        if input_char == '\n' {
            if let Some(target) = target {
                self.clear_flash_jump_ui(cx);
                self.finish_flash_jump(target, window, cx);
            } else {
                self.abort_flash_jump(window, cx);
            }
            return;
        }

        if let Some(label) = labels.iter().find(|label| label.label == input_char) {
            let range = label.range.clone();
            self.clear_flash_jump_ui(cx);
            self.finish_flash_jump(range, window, cx);
            return;
        }

        pattern.push(input_char);
        self.update_flash_state(pattern, labels, window, cx);
    }

    fn update_flash_state(
        &mut self,
        pattern: String,
        previous_labels: Vec<FlashJumpLabel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if pattern.is_empty() {
            self.clear_flash_jump_ui(cx);
            self.push_operator(
                Operator::FlashJump {
                    pattern,
                    labels: Vec::new(),
                    target: None,
                },
                window,
                cx,
            );
            return;
        }

        let smartcase = VimSettings::get_global(cx).use_smartcase_find;
        let applied = self.update_editor(cx, |_, editor, cx| {
            let FlashJumpUiData {
                labels,
                target,
                overlays,
                match_ranges,
            } = Self::collect_flash_jump_data(
                editor,
                &pattern,
                &previous_labels,
                smartcase,
                window,
                cx,
            );

            if match_ranges.is_empty() {
                editor.clear_navigation_overlays(FLASH_JUMP_OVERLAY_KEY, cx);
                editor.clear_background_highlights(HighlightKey::VimFlash, cx);
                None
            } else {
                editor.set_navigation_overlays(FLASH_JUMP_OVERLAY_KEY, overlays, cx);
                editor.highlight_background(
                    HighlightKey::VimFlash,
                    &match_ranges,
                    |_, theme| theme.colors().search_match_background,
                    cx,
                );
                Some((labels, target))
            }
        });

        match applied.flatten() {
            Some((labels, target)) => self.push_operator(
                Operator::FlashJump {
                    pattern,
                    labels,
                    target,
                },
                window,
                cx,
            ),
            None => self.abort_flash_jump(window, cx),
        }
    }

    /// Exits flash without jumping: flash.nvim leaves as soon as the pattern
    /// stops matching. A pending operator is aborted like a failed `f`, which
    /// also means ending dot recording the way `delete_motion` and the
    /// observe_keystrokes safety net do.
    fn abort_flash_jump(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.clear_flash_jump_ui(cx);
        self.clear_operator(window, cx);
        self.stop_recording_immediately(Box::new(ClearOperators), cx);
    }

    fn collect_flash_jump_data(
        editor: &mut Editor,
        pattern: &str,
        previous_labels: &[FlashJumpLabel],
        smartcase: bool,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> FlashJumpUiData {
        let (snapshot, font, font_size, label_color) = Self::jump_ui_context(editor, window, cx);
        let display_snapshot = &snapshot.display_snapshot;
        let buffer_snapshot = display_snapshot.buffer_snapshot();
        let visible_range = Self::visible_jump_range(editor, &snapshot, display_snapshot, cx);
        let visible_start = buffer_snapshot.point_to_offset(visible_range.start).0;
        let visible_end = buffer_snapshot
            .point_to_offset(visible_range.end)
            .0
            .max(visible_start);

        let cursor_offset = MultiBufferOffset(
            buffer_snapshot
                .point_to_offset(editor.selections.newest::<Point>(display_snapshot).head())
                .0
                .clamp(visible_start, visible_end),
        );

        // The scan window is centered on the cursor so that when the visible
        // range exceeds the byte cap (e.g. one enormous unwrapped line), the
        // matches next to the cursor are the ones that survive.
        let scan_start = buffer_snapshot.clip_offset(
            MultiBufferOffset(
                cursor_offset
                    .0
                    .saturating_sub(FLASH_JUMP_MAX_SEARCH_BYTES / 2)
                    .max(visible_start),
            ),
            Bias::Left,
        );
        let scan_end = buffer_snapshot.clip_offset(
            MultiBufferOffset((scan_start.0 + FLASH_JUMP_MAX_SEARCH_BYTES).min(visible_end)),
            Bias::Left,
        );

        let previous_labels = previous_labels
            .iter()
            .map(|label| (label.range.start.to_offset(buffer_snapshot), label.label))
            .collect::<HashMap<_, _>>();

        Self::build_flash_jump_ui_data(
            buffer_snapshot,
            scan_start,
            scan_end,
            cursor_offset,
            pattern,
            smartcase,
            &previous_labels,
            label_color,
            window.text_system(),
            font,
            font_size,
        )
    }

    fn build_flash_jump_ui_data(
        buffer: &MultiBufferSnapshot,
        start_offset: MultiBufferOffset,
        end_offset: MultiBufferOffset,
        cursor_offset: MultiBufferOffset,
        pattern: &str,
        smartcase: bool,
        previous_labels: &HashMap<MultiBufferOffset, char>,
        label_color: Hsla,
        text_system: &WindowTextSystem,
        font: Font,
        font_size: Pixels,
    ) -> FlashJumpUiData {
        if start_offset >= end_offset {
            return FlashJumpUiData::default();
        }

        let matches =
            Self::find_flash_matches(buffer, start_offset, end_offset, pattern, smartcase);
        if matches.is_empty() {
            return FlashJumpUiData::default();
        }

        let match_ranges = matches
            .iter()
            .map(|flash_match| {
                buffer.anchor_after(flash_match.range.start)
                    ..buffer.anchor_after(flash_match.range.end)
            })
            .collect::<Vec<_>>();

        // Like vim search, a match starting exactly at the cursor is not the
        // target — enter should always go somewhere.
        let target_index = matches
            .iter()
            .position(|flash_match| flash_match.range.start > cursor_offset)
            .unwrap_or(0);
        let target = match_ranges.get(target_index).cloned();

        let allowed_labels = Self::flash_allowed_labels(&matches, smartcase);
        let assignments =
            Self::assign_flash_labels(&matches, cursor_offset, allowed_labels, previous_labels);

        let font_id = text_system.resolve_font(&font);
        let is_monospace = Self::is_monospace_jump_font(text_system, font_id, font_size);
        let width_of_char = |ch| Self::jump_font_char_width(text_system, font_id, font_size, ch);

        let mut labels = Vec::with_capacity(assignments.len());
        let mut overlays = Vec::with_capacity(assignments.len());
        for (label, match_index) in assignments {
            let Some((flash_match, match_anchors)) =
                matches.get(match_index).zip(match_ranges.get(match_index))
            else {
                continue;
            };
            labels.push(FlashJumpLabel {
                label,
                range: match_anchors.clone(),
            });

            // The label is drawn over the text that follows the match, like
            // flash.nvim. A monospace label covers exactly the one character
            // already captured by the scan; in a proportional font the label
            // glyph can be wider, so fade out as many characters as it needs.
            // At a line end (or the scan boundary) there is nothing to cover
            // and the label renders in the empty space.
            let covered_end = if is_monospace {
                flash_match
                    .next_char
                    .filter(|ch| *ch != '\n' && *ch != '\r')
                    .map(|ch| flash_match.range.end + ch.len_utf8())
            } else {
                let label_width = width_of_char(label);
                let mut covered_end = flash_match.range.end;
                let mut covered_width = px(0.0);
                for ch in buffer.chars_at(flash_match.range.end) {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    covered_end = covered_end + ch.len_utf8();
                    covered_width += width_of_char(ch);
                    if covered_width >= label_width {
                        break;
                    }
                }
                (covered_end > flash_match.range.end).then_some(covered_end)
            };
            let covered_text_range =
                covered_end.map(|end| match_anchors.end..buffer.anchor_after(end));

            overlays.push(NavigationTargetOverlay {
                target_range: covered_text_range
                    .clone()
                    .unwrap_or(match_anchors.end..match_anchors.end),
                label: NavigationOverlayLabel {
                    text: label.to_string().into(),
                    text_color: label_color,
                    x_offset: px(0.),
                    scale_factor: 1.0,
                },
                covered_text_range,
            });
        }

        FlashJumpUiData {
            labels,
            target,
            overlays,
            match_ranges,
        }
    }

    fn find_flash_matches(
        buffer: &MultiBufferSnapshot,
        start_offset: MultiBufferOffset,
        end_offset: MultiBufferOffset,
        pattern: &str,
        smartcase: bool,
    ) -> Vec<FlashMatch> {
        let pattern_chars = pattern.chars().collect::<Vec<_>>();
        if pattern_chars.is_empty() {
            return Vec::new();
        }

        // A single streaming pass: each character is decoded once and slides
        // through a pattern-sized window of candidate positions.
        let mut matches: Vec<FlashMatch> = Vec::new();
        let mut window = VecDeque::with_capacity(pattern_chars.len() + 1);
        let mut chunk_start = start_offset;
        'chunks: for chunk in buffer.text_for_range(start_offset..end_offset) {
            for (index, ch) in chunk.char_indices() {
                let offset = chunk_start + index;

                if let Some(last) = matches.last_mut()
                    && last.range.end == offset
                {
                    last.next_char = Some(ch);
                }
                if matches.len() >= FLASH_JUMP_MAX_MATCHES {
                    break 'chunks;
                }

                window.push_back((offset, ch));
                if window.len() > pattern_chars.len() {
                    window.pop_front();
                }
                if window.len() == pattern_chars.len()
                    && let Some((match_start, _)) = window.front().copied()
                    && window
                        .iter()
                        .zip(&pattern_chars)
                        .all(|((_, buffer_char), pattern_char)| {
                            motion::is_character_match(*pattern_char, *buffer_char, smartcase)
                        })
                {
                    matches.push(FlashMatch {
                        range: match_start..offset + ch.len_utf8(),
                        next_char: None,
                    });
                }
            }
            chunk_start = chunk_start + chunk.len();
        }
        matches
    }

    /// Excludes labels that could be the next typed pattern character: a label
    /// equal to the character right after a match would be ambiguous between
    /// jumping and extending the pattern.
    fn flash_allowed_labels(matches: &[FlashMatch], smartcase: bool) -> Vec<char> {
        let mut follower_chars = matches
            .iter()
            .filter_map(|flash_match| flash_match.next_char)
            .collect::<Vec<_>>();
        follower_chars.sort_unstable();
        follower_chars.dedup();

        let mut labels = FLASH_JUMP_ALPHABET.to_vec();
        for next_char in follower_chars {
            if labels.is_empty() {
                break;
            }
            labels.retain(|label| !motion::is_character_match(*label, next_char, smartcase));
        }
        labels
    }

    /// Assigns labels to matches by distance from the cursor, returning
    /// `(label, index into matches)` pairs.
    fn assign_flash_labels(
        matches: &[FlashMatch],
        cursor_offset: MultiBufferOffset,
        allowed_labels: Vec<char>,
        previous_labels: &HashMap<MultiBufferOffset, char>,
    ) -> Vec<(char, usize)> {
        let mut ordered = (0..matches.len()).collect::<Vec<_>>();
        ordered.sort_by_key(|match_index| {
            matches[*match_index]
                .range
                .start
                .0
                .abs_diff(cursor_offset.0)
        });

        // Matches keep the label they had on the previous keystroke when
        // possible, so labels don't shuffle while the user types.
        let mut available = allowed_labels;
        let mut assignments = vec![None; ordered.len()];
        for (slot, match_index) in ordered.iter().enumerate() {
            if let Some(previous) = previous_labels.get(&matches[*match_index].range.start)
                && let Some(position) = available.iter().position(|label| label == previous)
            {
                assignments[slot] = Some(available.remove(position));
            }
        }

        let mut available = available.into_iter();
        for assignment in assignments.iter_mut() {
            if assignment.is_none() {
                match available.next() {
                    Some(label) => *assignment = Some(label),
                    None => break,
                }
            }
        }

        ordered
            .into_iter()
            .zip(assignments)
            .filter_map(|(match_index, label)| Some((label?, match_index)))
            .collect()
    }

    pub(crate) fn clear_flash_jump_ui(&mut self, cx: &mut Context<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            editor.clear_navigation_overlays(FLASH_JUMP_OVERLAY_KEY, cx);
            editor.clear_background_highlights(HighlightKey::VimFlash, cx);
        });
    }

    fn finish_flash_jump(
        &mut self,
        target: Range<Anchor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Like flash.nvim, the jump goes to the start of the match and is an
        // inclusive motion when there is a pending operator.
        self.motion(
            Motion::Jump {
                anchor: target.start,
                line: false,
                inclusive: true,
            },
            window,
            cx,
        );
    }
}

#[cfg(test)]
mod test {
    use editor::{HighlightKey, ToOffset};
    use gpui::KeyBinding;
    use language::Point;
    use settings::SettingsStore;

    use super::{FLASH_JUMP_MAX_SEARCH_BYTES, FLASH_JUMP_OVERLAY_KEY, PushFlash};
    use crate::{
        Vim, VimAddon,
        state::{Mode, Operator},
        test::VimTestContext,
    };

    fn bind_flash(cx: &mut VimTestContext) {
        cx.update(|_, cx| {
            cx.bind_keys([KeyBinding::new(
                "s",
                PushFlash,
                Some("vim_mode == normal || vim_mode == visual || vim_mode == operator"),
            )])
        });
    }

    fn active_flash_labels(cx: &mut VimTestContext) -> Vec<(char, String, usize)> {
        cx.update_editor(|editor, window, cx| {
            let labels = match editor
                .addon::<VimAddon>()
                .unwrap()
                .entity
                .read(cx)
                .operator_stack
                .last()
                .cloned()
            {
                Some(Operator::FlashJump { labels, .. }) => labels,
                other => panic!("expected active FlashJump operator, got {other:?}"),
            };

            let snapshot = editor.snapshot(window, cx);
            let buffer_snapshot = snapshot.display_snapshot.buffer_snapshot();

            labels
                .into_iter()
                .map(|label| {
                    let text = buffer_snapshot
                        .text_for_range(label.range.clone())
                        .collect::<String>();
                    let offset = label.range.start.to_offset(buffer_snapshot);
                    (label.label, text, offset.0)
                })
                .collect()
        })
    }

    fn flash_label_at(cx: &mut VimTestContext, offset: usize) -> String {
        let labels = active_flash_labels(cx);
        labels
            .iter()
            .find_map(|(label, _, start)| (*start == offset).then(|| label.to_string()))
            .unwrap_or_else(|| panic!("expected a label at offset {offset}, got {labels:?}"))
    }

    fn assert_flash_cleared(cx: &mut VimTestContext) {
        assert_eq!(cx.active_operator(), None);
        let (covered_count, has_match_highlight) = cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let covered_count = snapshot
                .text_highlight_ranges(HighlightKey::NavigationOverlay(FLASH_JUMP_OVERLAY_KEY))
                .map(|ranges| ranges.as_ref().clone().1.len())
                .unwrap_or_default();
            (
                covered_count,
                editor.has_background_highlights(HighlightKey::VimFlash),
            )
        });
        assert_eq!(covered_count, 0, "expected flash overlays to be cleared");
        assert!(
            !has_match_highlight,
            "expected flash match highlights to be cleared"
        );
    }

    #[gpui::test]
    async fn test_flash_jump_basic(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇlorem ipsum dolor", Mode::Normal);

        cx.simulate_keystrokes("s d o");
        let label = flash_label_at(&mut cx, 12);
        cx.simulate_keystrokes(&label);

        cx.assert_state("lorem ipsum ˇdolor", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_enter_jumps_to_nearest_match(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);

        cx.set_state("alpha beta ˇgamma beta alpha", Mode::Normal);
        cx.simulate_keystrokes("s b e enter");
        cx.assert_state("alpha beta gamma ˇbeta alpha", Mode::Normal);
        assert_flash_cleared(&mut cx);

        // The target wraps to the first visible match when all matches are
        // before the cursor.
        cx.set_state("beta gamma ˇdelta", Mode::Normal);
        cx.simulate_keystrokes("s b e enter");
        cx.assert_state("ˇbeta gamma delta", Mode::Normal);
        assert_flash_cleared(&mut cx);

        // Enter with an empty pattern exits without moving.
        cx.set_state("beta gamma ˇdelta", Mode::Normal);
        cx.simulate_keystrokes("s enter");
        cx.assert_state("beta gamma ˇdelta", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_cancels_on_escape(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇalpha beta gamma", Mode::Normal);

        cx.simulate_keystrokes("s b e");
        assert!(matches!(
            cx.active_operator(),
            Some(Operator::FlashJump { .. })
        ));
        cx.simulate_keystrokes("escape");

        cx.assert_state("ˇalpha beta gamma", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_exits_when_pattern_stops_matching(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);

        cx.set_state("ˇalpha beta", Mode::Normal);
        cx.simulate_keystrokes("s q");
        cx.assert_state("ˇalpha beta", Mode::Normal);
        assert_flash_cleared(&mut cx);

        // A pending operator is aborted along with the jump.
        cx.set_state("ˇalpha beta", Mode::Normal);
        cx.simulate_keystrokes("d s q");
        cx.assert_state("ˇalpha beta", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_backspace_edits_pattern(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇab ac ad", Mode::Normal);

        cx.simulate_keystrokes("s a b");
        assert_eq!(active_flash_labels(&mut cx).len(), 1);

        cx.simulate_keystrokes("backspace");
        assert_eq!(active_flash_labels(&mut cx).len(), 3);

        // Removing the last pattern character keeps flash active.
        cx.simulate_keystrokes("backspace");
        assert!(matches!(
            cx.active_operator(),
            Some(Operator::FlashJump { .. })
        ));
        assert_eq!(active_flash_labels(&mut cx).len(), 0);

        cx.simulate_keystrokes("escape");
        cx.assert_state("ˇab ac ad", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_skips_conflicting_labels(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇfa fs fd fg", Mode::Normal);

        cx.simulate_keystrokes("s f");

        let labels = active_flash_labels(&mut cx);
        assert_eq!(labels.len(), 4);
        for (label, _, _) in &labels {
            assert!(
                !"asdg".contains(*label),
                "label {label:?} conflicts with a possible next pattern character"
            );
        }

        cx.simulate_keystrokes("escape");
    }

    #[gpui::test]
    async fn test_flash_jump_label_reuse(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇab ac ad", Mode::Normal);

        cx.simulate_keystrokes("s a");
        let first_round = flash_label_at(&mut cx, 3);

        cx.simulate_keystrokes("c");
        let second_round = flash_label_at(&mut cx, 3);
        assert_eq!(
            first_round, second_round,
            "expected the match to keep its label as the pattern grows"
        );

        cx.simulate_keystrokes("escape");
    }

    #[gpui::test]
    async fn test_flash_jump_smartcase(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |s| {
                s.vim.get_or_insert_default().use_smartcase_find = Some(true);
            });
        });
        bind_flash(&mut cx);
        cx.set_state("ˇHello hello", Mode::Normal);

        // A lowercase pattern matches both cases.
        cx.simulate_keystrokes("s h e");
        assert_eq!(active_flash_labels(&mut cx).len(), 2);
        cx.simulate_keystrokes("escape");

        // An uppercase pattern only matches exactly.
        cx.simulate_keystrokes("s shift-h");
        let labels = active_flash_labels(&mut cx);
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].2, 0);
        cx.simulate_keystrokes("escape");
    }

    #[gpui::test]
    async fn test_flash_jump_operator_pending(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);

        // Delete is inclusive of the match start, like flash.nvim.
        cx.set_state("ˇlorem ipsum", Mode::Normal);
        cx.simulate_keystrokes("d s i p");
        let label = flash_label_at(&mut cx, 6);
        cx.simulate_keystrokes(&label);
        cx.assert_state("ˇpsum", Mode::Normal);
        assert_flash_cleared(&mut cx);

        cx.set_state("ˇlorem ipsum", Mode::Normal);
        cx.simulate_keystrokes("c s i p");
        let label = flash_label_at(&mut cx, 6);
        cx.simulate_keystrokes(&label);
        cx.assert_state("ˇpsum", Mode::Insert);
        assert_flash_cleared(&mut cx);

        cx.set_state("ˇlorem ipsum", Mode::Normal);
        cx.simulate_keystrokes("y s i p");
        let label = flash_label_at(&mut cx, 6);
        cx.simulate_keystrokes(&label);
        cx.assert_state("ˇlorem ipsum", Mode::Normal);
        cx.simulate_keystrokes("p");
        cx.assert_state("llorem ˇiorem ipsum", Mode::Normal);
    }

    #[gpui::test]
    async fn test_flash_jump_operator_pending_backward(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);

        // A backward jump includes the character under the cursor, matching
        // flash.nvim's unconditional inclusive toggle in op-pending mode.
        cx.set_state("lorem ipsum ˇdolor", Mode::Normal);
        cx.simulate_keystrokes("d s l o");
        let label = flash_label_at(&mut cx, 0);
        cx.simulate_keystrokes(&label);
        cx.assert_state("ˇolor", Mode::Normal);
        assert_flash_cleared(&mut cx);

        cx.set_state("lorem ipsum ˇdolor", Mode::Normal);
        cx.simulate_keystrokes("c s l o");
        let label = flash_label_at(&mut cx, 0);
        cx.simulate_keystrokes(&label);
        cx.assert_state("ˇolor", Mode::Insert);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_operator_pending_multiline(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);

        // Forward across a line boundary.
        cx.set_state("ˇone\ntwo three", Mode::Normal);
        cx.simulate_keystrokes("d s t h");
        let label = flash_label_at(&mut cx, 8);
        cx.simulate_keystrokes(&label);
        cx.assert_state("ˇhree", Mode::Normal);
        assert_flash_cleared(&mut cx);

        // Backward across a line boundary.
        cx.set_state("one two\nthrˇee", Mode::Normal);
        cx.simulate_keystrokes("d s o n");
        let label = flash_label_at(&mut cx, 0);
        cx.simulate_keystrokes(&label);
        cx.assert_state("ˇe", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_visual_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇlorem ipsum", Mode::Normal);

        cx.simulate_keystrokes("v s i p");
        let label = flash_label_at(&mut cx, 6);
        cx.simulate_keystrokes(&label);

        cx.assert_state("«lorem iˇ»psum", Mode::Visual);
    }

    #[gpui::test]
    async fn test_flash_jump_multibyte(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇPočet hostů", Mode::Normal);

        cx.simulate_keystrokes("s t ů");
        let labels = active_flash_labels(&mut cx);
        assert_eq!(labels.len(), 1);
        cx.simulate_keystrokes(&labels[0].0.to_string());

        cx.assert_state("Počet hosˇtů", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_pushes_to_jump_list(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        // Nav history entries are only created when the cursor moves more
        // than MIN_NAVIGATION_HISTORY_ROW_DELTA rows.
        let blank_lines = "\n".repeat(12);
        cx.set_state(&format!("ˇone{blank_lines}two three"), Mode::Normal);

        cx.simulate_keystrokes("s t w");
        let label = flash_label_at(&mut cx, 15);
        cx.simulate_keystrokes(&label);
        cx.assert_state(&format!("one{blank_lines}ˇtwo three"), Mode::Normal);

        cx.simulate_keystrokes("ctrl-o");
        cx.assert_state(&format!("ˇone{blank_lines}two three"), Mode::Normal);
    }

    #[gpui::test]
    async fn test_flash_jump_ui_cleared_on_mouse_selection(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇalpha beta gamma", Mode::Normal);

        cx.simulate_keystrokes("s b e");
        assert_eq!(active_flash_labels(&mut cx).len(), 1);

        // A mouse drag updates selections directly; vim reacts by switching
        // to visual mode, which discards the operator stack without going
        // through clear_operator.
        cx.update_editor(|editor, window, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges([Point::new(0, 0)..Point::new(0, 5)]);
            });
        });
        cx.run_until_parked();

        assert_eq!(cx.mode(), Mode::Visual);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_ui_cleared_with_stacked_operator(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇalpha beta gamma", Mode::Normal);

        cx.simulate_keystrokes("s b e");
        assert_eq!(active_flash_labels(&mut cx).len(), 1);

        // ctrl-k stacks a digraph operator on top of flash; escape then
        // discards the whole stack, with flash no longer on top.
        cx.simulate_keystrokes("ctrl-k escape");

        cx.assert_state("ˇalpha beta gamma", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_abort_stops_dot_recording(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇabc def", Mode::Normal);

        cx.simulate_keystrokes("d s q");
        cx.assert_state("ˇabc def", Mode::Normal);
        assert!(
            !cx.update(|_, cx| Vim::globals(cx).dot_recording),
            "aborted flash must not leave dot recording on"
        );

        // Recording state is sane afterwards: a new change records and
        // repeats normally.
        cx.simulate_keystrokes("x");
        cx.assert_state("ˇbc def", Mode::Normal);
        cx.simulate_keystrokes(".");
        cx.assert_state("ˇc def", Mode::Normal);
    }

    #[gpui::test]
    async fn test_flash_jump_scan_window_follows_cursor(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);

        // A single line longer than the scan cap with the cursor at its end:
        // the capped window must cover the text around the cursor, not just
        // the start of the visible range.
        let filler = "x".repeat(FLASH_JUMP_MAX_SEARCH_BYTES + 16 * 1024);
        cx.set_state(&format!("{filler}ˇneedle"), Mode::Normal);

        cx.simulate_keystrokes("s n e");
        let labels = active_flash_labels(&mut cx);
        assert_eq!(labels.len(), 1);
        cx.simulate_keystrokes("escape");
    }

    #[gpui::test]
    async fn test_flash_jump_consumes_full_ime_input(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇにほんご にほ", Mode::Normal);

        cx.simulate_keystrokes("s");
        // An IME commit delivers the whole string in one InputIgnored event
        // (vim disables editor input in normal mode).
        cx.update_editor(|editor, window, cx| {
            editor.replay_insert_event("にほ", None, window, cx)
        });

        let labels = active_flash_labels(&mut cx);
        assert_eq!(labels.len(), 2);
        cx.simulate_keystrokes("escape");
    }

    #[gpui::test]
    async fn test_flash_jump_enter_skips_match_at_cursor(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);

        cx.set_state("ˇbeta gamma beta", Mode::Normal);
        cx.simulate_keystrokes("s b e enter");
        cx.assert_state("beta gamma ˇbeta", Mode::Normal);
        assert_flash_cleared(&mut cx);

        // Operating to the target must not degenerate into a zero-width jump
        // deleting a single character.
        cx.set_state("ˇbeta gamma beta", Mode::Normal);
        cx.simulate_keystrokes("d s b e enter");
        cx.assert_state("ˇeta", Mode::Normal);
        assert_flash_cleared(&mut cx);
    }

    #[gpui::test]
    async fn test_flash_jump_dot_repeat_does_not_crash(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        bind_flash(&mut cx);
        cx.set_state("ˇab ab ab", Mode::Normal);

        cx.simulate_keystrokes("d s a b");
        let label = flash_label_at(&mut cx, 3);
        cx.simulate_keystrokes(&label);

        // Repeating replays the recorded keystrokes against the new buffer
        // contents; the exact target is unspecified, it just must not panic.
        cx.simulate_keystrokes(".");
        cx.simulate_keystrokes("escape");
    }
}
