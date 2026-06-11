use std::ops::Range;

use collections::HashMap;
use editor::display_map::HighlightKey;
use editor::{
    Anchor, Editor, MultiBufferOffset, NavigationOverlayKey, NavigationOverlayLabel,
    NavigationTargetOverlay, ToOffset,
};
use gpui::{Context, Hsla, Window, actions};
use language::Point;
use multi_buffer::{MultiBufferRow, MultiBufferSnapshot};
use settings::Settings;
use theme::ActiveTheme as _;
use ui::px;

use crate::{
    Vim, VimSettings,
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

#[derive(Default)]
struct FlashJumpUiData {
    labels: Vec<FlashJumpLabel>,
    target: Option<Range<Anchor>>,
    overlays: Vec<NavigationTargetOverlay>,
    match_ranges: Vec<Range<Anchor>>,
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
            self.clear_flash_jump_ui(cx);
            if let Some(target) = target {
                self.finish_flash_jump(target, window, cx);
            } else {
                self.clear_operator(window, cx);
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

        let Some(data) = self.collect_flash_jump_data(&pattern, &previous_labels, window, cx)
        else {
            self.clear_flash_jump_ui(cx);
            return;
        };

        if data.match_ranges.is_empty() {
            // flash.nvim exits as soon as the pattern stops matching. This also
            // aborts any pending operator, like an unsuccessful `f`.
            self.clear_flash_jump_ui(cx);
            self.clear_operator(window, cx);
            return;
        }

        if !self.apply_flash_jump_ui(data.overlays, &data.match_ranges, cx) {
            return;
        }

        self.push_operator(
            Operator::FlashJump {
                pattern,
                labels: data.labels,
                target: data.target,
            },
            window,
            cx,
        );
    }

    fn collect_flash_jump_data(
        &mut self,
        pattern: &str,
        previous_labels: &[FlashJumpLabel],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<FlashJumpUiData> {
        let smartcase = VimSettings::get_global(cx).use_smartcase_find;
        self.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            let display_snapshot = &snapshot.display_snapshot;
            let buffer_snapshot = display_snapshot.buffer_snapshot();
            let visible_range = Self::visible_jump_range(editor, &snapshot, display_snapshot, cx);
            let start_offset = buffer_snapshot.point_to_offset(visible_range.start);
            // In normal mode the visible range end is clipped to sit before
            // the last character of a line, which would exclude that character
            // from the search; extend the end to cover its whole line.
            let mut visible_end = visible_range.end;
            if visible_end.column > 0 {
                visible_end.column = buffer_snapshot.line_len(MultiBufferRow(visible_end.row));
            }
            let end_offset = buffer_snapshot.point_to_offset(visible_end);

            let selections = editor.selections.all::<Point>(&display_snapshot);
            let cursor_offset = selections
                .first()
                .map(|selection| buffer_snapshot.point_to_offset(selection.head()))
                .unwrap_or(start_offset);

            let previous_labels = previous_labels
                .iter()
                .map(|label| (label.range.start.to_offset(buffer_snapshot), label.label))
                .collect::<HashMap<_, _>>();

            let label_color = cx.theme().colors().vim_helix_jump_label_foreground;

            Self::build_flash_jump_ui_data(
                buffer_snapshot,
                start_offset,
                end_offset,
                cursor_offset,
                pattern,
                smartcase,
                &previous_labels,
                label_color,
            )
        })
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
    ) -> FlashJumpUiData {
        if start_offset >= end_offset {
            return FlashJumpUiData::default();
        }

        let matches =
            Self::find_flash_matches(buffer, start_offset, end_offset, pattern, smartcase);
        if matches.is_empty() {
            return FlashJumpUiData::default();
        }

        let target = matches
            .iter()
            .find(|range| range.start >= cursor_offset)
            .or_else(|| matches.first())
            .map(|range| buffer.anchor_after(range.start)..buffer.anchor_after(range.end));

        let allowed_labels = Self::flash_allowed_labels(buffer, &matches, smartcase);
        let assignments =
            Self::assign_flash_labels(&matches, cursor_offset, allowed_labels, previous_labels);

        let match_ranges = matches
            .iter()
            .map(|range| buffer.anchor_after(range.start)..buffer.anchor_after(range.end))
            .collect();

        let mut labels = Vec::with_capacity(assignments.len());
        let mut overlays = Vec::with_capacity(assignments.len());
        for (label, range) in assignments {
            let start_anchor = buffer.anchor_after(range.start);
            let end_anchor = buffer.anchor_after(range.end);
            labels.push(FlashJumpLabel {
                label,
                range: start_anchor..end_anchor,
            });

            // The label is drawn over the character that follows the match,
            // like flash.nvim. At the end of a line there is no character to
            // cover and the label renders in the empty space past it.
            let covered_text_range = buffer
                .chars_at(range.end)
                .next()
                .filter(|ch| *ch != '\n' && *ch != '\r')
                .map(|ch| end_anchor..buffer.anchor_after(range.end + ch.len_utf8()));

            overlays.push(NavigationTargetOverlay {
                target_range: covered_text_range
                    .clone()
                    .unwrap_or(end_anchor..end_anchor),
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
    ) -> Vec<Range<MultiBufferOffset>> {
        if pattern.is_empty() {
            return Vec::new();
        }
        let pattern_chars = pattern.chars().collect::<Vec<_>>();

        let mut text = String::new();
        for chunk in buffer.text_for_range(start_offset..end_offset) {
            text.push_str(chunk);
        }

        let mut matches = Vec::new();
        for (byte_offset, _) in text.char_indices() {
            let mut buffer_chars = text[byte_offset..].chars();
            let mut match_len = 0;
            let mut matched = true;
            for pattern_char in &pattern_chars {
                match buffer_chars.next() {
                    Some(buffer_char)
                        if motion::is_character_match(*pattern_char, buffer_char, smartcase) =>
                    {
                        match_len += buffer_char.len_utf8();
                    }
                    _ => {
                        matched = false;
                        break;
                    }
                }
            }
            if matched {
                let match_start = start_offset + byte_offset;
                matches.push(match_start..match_start + match_len);
            }
        }
        matches
    }

    /// Excludes labels that could be the next typed pattern character: a label
    /// equal to the character right after a match would be ambiguous between
    /// jumping and extending the pattern.
    fn flash_allowed_labels(
        buffer: &MultiBufferSnapshot,
        matches: &[Range<MultiBufferOffset>],
        smartcase: bool,
    ) -> Vec<char> {
        let mut labels = FLASH_JUMP_ALPHABET.to_vec();
        for match_range in matches {
            if labels.is_empty() {
                break;
            }
            if let Some(next_char) = buffer.chars_at(match_range.end).next() {
                labels.retain(|label| !motion::is_character_match(*label, next_char, smartcase));
            }
        }
        labels
    }

    fn assign_flash_labels(
        matches: &[Range<MultiBufferOffset>],
        cursor_offset: MultiBufferOffset,
        allowed_labels: Vec<char>,
        previous_labels: &HashMap<MultiBufferOffset, char>,
    ) -> Vec<(char, Range<MultiBufferOffset>)> {
        let mut ordered = matches.to_vec();
        ordered.sort_by_key(|range| range.start.0.abs_diff(cursor_offset.0));

        // Matches keep the label they had on the previous keystroke when
        // possible, so labels don't shuffle while the user types.
        let mut available = allowed_labels;
        let mut assignments = vec![None; ordered.len()];
        for (index, range) in ordered.iter().enumerate() {
            if let Some(previous) = previous_labels.get(&range.start)
                && let Some(position) = available.iter().position(|label| label == previous)
            {
                assignments[index] = Some(available.remove(position));
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
            .filter_map(|(range, label)| Some((label?, range)))
            .collect()
    }

    fn apply_flash_jump_ui(
        &mut self,
        overlays: Vec<NavigationTargetOverlay>,
        match_ranges: &[Range<Anchor>],
        cx: &mut Context<Self>,
    ) -> bool {
        self.update_editor(cx, |_, editor, cx| {
            editor.set_navigation_overlays(FLASH_JUMP_OVERLAY_KEY, overlays, cx);
            editor.highlight_background(
                HighlightKey::VimFlash,
                match_ranges,
                |_, theme| theme.colors().search_match_background,
                cx,
            );
        })
        .is_some()
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
    use settings::SettingsStore;

    use super::{FLASH_JUMP_OVERLAY_KEY, PushFlash};
    use crate::{
        VimAddon,
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
