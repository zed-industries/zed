use std::rc::Rc;

use editor::{Editor, MultiBufferOffset};
use gpui::{
    A11ySubtreeBuilder, AccessibleAction, AnyElement, ElementId, Entity, Focusable, Role,
    TextStyleRefinement,
    accesskit::{self, ActionData},
};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::{Tooltip, prelude::*, rems};

#[derive(IntoElement)]
pub struct SettingsInputField {
    id: ElementId,
    initial_text: Option<String>,
    placeholder: Option<&'static str>,
    confirm: Option<Rc<dyn Fn(Option<String>, &mut Window, &mut App)>>,
    tab_index: Option<isize>,
    use_buffer_font: bool,
    display_confirm_button: bool,
    display_clear_button: bool,
    clear_on_confirm: bool,
    confirm_on_focus_out: bool,
    action_slot: Option<AnyElement>,
    color: Option<Color>,
    aria_label: Option<SharedString>,
}

impl SettingsInputField {
    /// Creates a new input field.
    ///
    /// The `id` must be unique among sibling elements: it keys the underlying
    /// editor's state across frames and identifies this field in the
    /// accessibility tree. Derive it from data unique to the setting being
    /// edited (e.g. its JSON path).
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            initial_text: None,
            placeholder: None,
            confirm: None,
            tab_index: None,
            use_buffer_font: false,
            display_confirm_button: false,
            display_clear_button: false,
            clear_on_confirm: false,
            confirm_on_focus_out: false,
            action_slot: None,
            color: None,
            aria_label: None,
        }
    }

    pub fn with_initial_text(mut self, initial_text: String) -> Self {
        self.initial_text = Some(initial_text);
        self
    }

    pub fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn on_confirm(
        mut self,
        confirm: impl Fn(Option<String>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.confirm = Some(Rc::new(confirm));
        self
    }

    pub fn display_confirm_button(mut self) -> Self {
        self.display_confirm_button = true;
        self
    }

    pub fn display_clear_button(mut self) -> Self {
        self.display_clear_button = true;
        self
    }

    pub fn clear_on_confirm(mut self) -> Self {
        self.clear_on_confirm = true;
        self
    }

    pub fn confirm_on_focus_out(mut self) -> Self {
        self.confirm_on_focus_out = true;
        self
    }

    pub fn action_slot(mut self, action: impl IntoElement) -> Self {
        self.action_slot = Some(action.into_any_element());
        self
    }

    pub(crate) fn tab_index(mut self, arg: isize) -> Self {
        self.tab_index = Some(arg);
        self
    }

    pub fn with_buffer_font(mut self) -> Self {
        self.use_buffer_font = true;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the label announced by assistive technology.
    /// Defaults to the placeholder text, if any.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }
}

impl RenderOnce for SettingsInputField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl ui::IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let use_buffer_font = self.use_buffer_font;
        let color = self.color.map(|c| c.color(cx));
        let styles = TextStyleRefinement {
            font_family: use_buffer_font.then(|| settings.buffer_font.family.clone()),
            font_size: use_buffer_font.then(|| rems(0.75).into()),
            color,
            ..Default::default()
        };

        let first_render_initial_text = window.use_keyed_state(
            (self.id.clone(), "first-render-initial-text"),
            cx,
            |_, _| self.initial_text.clone(),
        );

        let editor = window.use_keyed_state((self.id.clone(), "editor"), cx, {
            let initial_text = self.initial_text.clone();
            let placeholder = self.placeholder;
            let mut confirm = self.confirm.clone();

            move |window, cx| {
                let mut editor = Editor::single_line(window, cx);
                let editor_focus_handle = editor.focus_handle(cx);
                if let Some(text) = initial_text {
                    editor.set_text(text, window, cx);
                }

                if let Some(confirm) = confirm.take()
                    && (self.confirm_on_focus_out
                        || (!self.display_confirm_button
                            && !self.display_clear_button
                            && !self.clear_on_confirm))
                {
                    cx.on_focus_out(
                        &editor_focus_handle,
                        window,
                        move |editor, _, window, cx| {
                            let text = Some(editor.text(cx));
                            confirm(text, window, cx);
                        },
                    )
                    .detach();
                }

                if let Some(placeholder) = placeholder {
                    editor.set_placeholder_text(placeholder, window, cx);
                }
                editor.set_text_style_refinement(styles);
                editor
            }
        });

        let is_editor_focused = editor.read(cx).is_focused(window);
        let editor_text = editor.read(cx).text(cx);

        // The cached editor keeps stale text when the setting changes underneath it, so
        // reconcile it here, skipping focused editors with unsaved edits to avoid clobbering.
        let synced_text = first_render_initial_text.read(cx);
        if &self.initial_text != synced_text {
            let has_unsaved_edits = editor_text != synced_text.as_deref().unwrap_or_default();
            if !is_editor_focused || !has_unsaved_edits {
                *first_render_initial_text.as_mut(cx) = self.initial_text.clone();
                let weak_editor = editor.downgrade();
                let new_text = self.initial_text.clone().unwrap_or_default();

                window.defer(cx, move |window, cx| {
                    weak_editor
                        .update(cx, |editor, cx| {
                            editor.set_text(new_text, window, cx);
                        })
                        .ok();
                });
            }
        }

        let weak_editor = editor.downgrade();
        let weak_editor_for_button = editor.downgrade();
        let weak_editor_for_clear = editor.downgrade();

        let clear_on_confirm = self.clear_on_confirm;
        let clear_on_confirm_for_button = self.clear_on_confirm;

        let display_confirm_button = self.display_confirm_button;
        let display_clear_button = self.display_clear_button;
        let confirm_for_button = self.confirm.clone();
        let is_editor_empty = editor_text.trim().is_empty();

        let aria_label = self
            .aria_label
            .or_else(|| self.placeholder.map(SharedString::new_static));

        let (a11y_value, a11y_text_runs) =
            text_field_a11y_state(self.id.clone(), &editor, window, cx);

        let theme_colors = cx.theme().colors();

        h_flex()
            .id(self.id.clone())
            .role(Role::TextInput)
            .when_some(aria_label, |this, label| this.aria_label(label))
            .aria_value(a11y_value)
            .when_some(self.placeholder, |this, placeholder| {
                this.aria_placeholder(placeholder)
            })
            .a11y_synthetic_children(a11y_text_runs)
            .on_a11y_action(AccessibleAction::SetValue, {
                let weak_editor = editor.downgrade();
                let confirm = self.confirm.clone();
                move |data, window, cx| {
                    let Some(ActionData::Value(text)) = data else {
                        return;
                    };
                    let Some(editor) = weak_editor.upgrade() else {
                        return;
                    };
                    let text = text.to_string();
                    editor.update(cx, |editor, cx| {
                        editor.set_text(text.clone(), window, cx);
                    });
                    if let Some(confirm) = confirm.as_ref() {
                        let new_value = (!text.is_empty()).then_some(text);
                        confirm(new_value, window, cx);
                    }
                }
            })
            .group("settings-input-field-editor")
            .relative()
            .py_1()
            .px_2()
            .h_8()
            .min_w_64()
            .rounded_md()
            .border_1()
            .border_color(theme_colors.border)
            .bg(theme_colors.editor_background)
            .map(|this| {
                let focus_handle = editor.focus_handle(cx);
                let focus_handle = if let Some(tab_index) = self.tab_index {
                    focus_handle.tab_index(tab_index).tab_stop(true)
                } else {
                    focus_handle
                };
                this.track_focus(&focus_handle)
                    .focus(|s| s.border_color(theme_colors.border_focused))
            })
            .child(editor)
            .child(
                h_flex()
                    .absolute()
                    .top_1()
                    .right_1()
                    .invisible()
                    .when(is_editor_focused, |this| this.visible())
                    .group_hover("settings-input-field-editor", |this| this.visible())
                    .when(
                        display_clear_button && !is_editor_empty && is_editor_focused,
                        |this| {
                            this.child(
                                IconButton::new("clear-button", IconName::Close)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .aria_label("Clear")
                                    .tooltip(Tooltip::text("Clear"))
                                    .on_click(move |_, window, cx| {
                                        let Some(editor) = weak_editor_for_clear.upgrade() else {
                                            return;
                                        };
                                        editor.update(cx, |editor, cx| {
                                            editor.set_text("", window, cx);
                                        });
                                    }),
                            )
                        },
                    )
                    .when(
                        display_confirm_button && !is_editor_empty && is_editor_focused,
                        |this| {
                            this.child(
                                IconButton::new("confirm-button", IconName::Check)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Success)
                                    .aria_label("Confirm")
                                    .tooltip(Tooltip::text("Enter to Confirm"))
                                    .on_click(move |_, window, cx| {
                                        let Some(confirm) = confirm_for_button.as_ref() else {
                                            return;
                                        };
                                        let Some(editor) = weak_editor_for_button.upgrade() else {
                                            return;
                                        };
                                        let new_value =
                                            editor.read_with(cx, |editor, cx| editor.text(cx));
                                        let new_value =
                                            (!new_value.is_empty()).then_some(new_value);
                                        confirm(new_value, window, cx);
                                        if clear_on_confirm_for_button {
                                            editor.update(cx, |editor, cx| {
                                                editor.set_text("", window, cx);
                                            });
                                        }
                                    }),
                            )
                        },
                    )
                    .when_some(self.action_slot, |this, action| this.child(action)),
            )
            .when_some(self.confirm, |this, confirm| {
                this.on_action::<menu::Confirm>({
                    move |_, window, cx| {
                        let Some(editor) = weak_editor.upgrade() else {
                            return;
                        };
                        let new_value = editor.read_with(cx, |editor, cx| editor.text(cx));
                        let new_value = (!new_value.is_empty()).then_some(new_value);
                        confirm(new_value, window, cx);
                        if clear_on_confirm {
                            editor.update(cx, |editor, cx| {
                                editor.set_text("", window, cx);
                            });
                        }
                    }
                })
            })
    }
}

/// Compute the shared accessibility state for a focusable wrapper around a
/// single-line [`Editor`]:
///
/// - The value to report via `aria_value`. While the editor is focused this
///   is frozen at its focus-time content: screen readers announce the full value
///   on every change of a focused control, which would re-read the whole content
///   on each keystroke. The snapshot re-syncs on blur.
/// - A closure for `a11y_synthetic_children` exposing the editor's live text
///   and selection as AccessKit text runs, enabling the platform text
///   pattern (caret tracking, review commands, typed-character echo).
///
/// The caller must also give the element an id, a text input role (e.g.
/// [`Role::TextInput`]), a label, and track the editor's focus handle.
///
/// All work is skipped when accessibility is inactive (no assistive
/// technology connected), since the results are only observable through the
/// accessibility tree.
///
/// Note: much of this may want
pub(crate) fn text_field_a11y_state(
    state_key: impl Into<ElementId>,
    editor: &Entity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> (String, impl FnOnce(&mut A11ySubtreeBuilder) + 'static) {
    let state = window.is_a11y_active().then(|| {
        let (text, selection_head, selection_tail) = editor.update(cx, |editor, cx| {
            let display_snapshot = editor.display_snapshot(cx);
            let selection = editor
                .selections
                .newest::<MultiBufferOffset>(&display_snapshot);
            (editor.text(cx), selection.head().0, selection.tail().0)
        });
        let is_focused = editor.read(cx).is_focused(window);

        let a11y_value = window.use_keyed_state((state_key.into(), "a11y-value"), cx, {
            let text = text.clone();
            move |_, _| text
        });
        if !is_focused && *a11y_value.read(cx) != text {
            *a11y_value.as_mut(cx) = text.clone();
        }
        let frozen_value = a11y_value.read(cx).clone();

        (frozen_value, text, selection_head, selection_tail)
    });

    let (frozen_value, run_data) = match state {
        Some((frozen_value, text, selection_head, selection_tail)) => {
            (frozen_value, Some((text, selection_head, selection_tail)))
        }
        None => (String::new(), None),
    };

    let text_runs = move |builder: &mut A11ySubtreeBuilder| {
        if let Some((text, selection_head, selection_tail)) = run_data {
            push_a11y_text_runs(builder, &text, selection_tail, selection_head);
        }
    };

    (frozen_value, text_runs)
}

/// AccessKit's `word_starts` uses `u8` indices, so a single text run cannot
/// exceed this many characters. Longer text is split into multiple runs.
const MAX_CHARS_PER_TEXT_RUN: usize = 255;

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn char_index_for_byte(text: &str, byte_offset: usize) -> usize {
    text.char_indices()
        .take_while(|(byte_ix, _)| *byte_ix < byte_offset)
        .count()
}

/// Convert a character index into an AccessKit text position, accounting for
/// text that is split into multiple runs.
///
/// `synthetic_node_id` maps a chunk index to the run's node id (in practice
/// [`A11ySubtreeBuilder::synthetic_node_id`]); it is a parameter so this
/// arithmetic can be property-tested without constructing a builder.
fn a11y_text_position(
    char_index: usize,
    synthetic_node_id: impl Fn(u64) -> accesskit::NodeId,
) -> accesskit::TextPosition {
    // A position landing exactly on a chunk boundary refers to the end of the
    // previous chunk rather than the start of the next one.
    let chunk_index = if char_index > 0 && char_index.is_multiple_of(MAX_CHARS_PER_TEXT_RUN) {
        char_index / MAX_CHARS_PER_TEXT_RUN - 1
    } else {
        char_index / MAX_CHARS_PER_TEXT_RUN
    };
    accesskit::TextPosition {
        node: synthetic_node_id(chunk_index as u64),
        character_index: char_index - chunk_index * MAX_CHARS_PER_TEXT_RUN,
    }
}

/// Split `text` into AccessKit text runs (chunked small enough that per-run
/// character indices fit AccessKit's `u8`-indexed `word_starts`), and compute
/// the text selection for the given byte offsets.
///
/// `synthetic_node_id` maps a chunk index to that run's node id. Returns the
/// runs in order plus the selection, leaving it to the caller to push them —
/// this keeps the logic free of [`A11ySubtreeBuilder`] so it can be
/// property-tested against arbitrary strings.
///
/// `selection_tail` and `selection_head` are byte offsets into `text`.
fn build_a11y_text_runs(
    text: &str,
    selection_tail: usize,
    selection_head: usize,
    synthetic_node_id: impl Fn(u64) -> accesskit::NodeId,
) -> (
    Vec<(accesskit::NodeId, accesskit::Node)>,
    accesskit::TextSelection,
) {
    let chars: Vec<char> = text.chars().collect();
    let total_chars = chars.len();
    // Build at least one (possibly empty) run so the text pattern remains
    // supported when the field is empty.
    let num_chunks = total_chars.div_ceil(MAX_CHARS_PER_TEXT_RUN).max(1);

    let mut word_starts = Vec::new();
    let mut was_word_char = false;
    for (ix, c) in chars.iter().enumerate() {
        let is_word = is_word_char(*c);
        if is_word && !was_word_char {
            word_starts.push(ix);
        }
        was_word_char = is_word;
    }

    let mut runs = Vec::with_capacity(num_chunks);
    for chunk_index in 0..num_chunks {
        let char_start = chunk_index * MAX_CHARS_PER_TEXT_RUN;
        let char_end = (char_start + MAX_CHARS_PER_TEXT_RUN).min(total_chars);
        let chunk_chars = &chars[char_start..char_end];

        let mut node = accesskit::Node::new(accesskit::Role::TextRun);
        node.set_text_direction(accesskit::TextDirection::LeftToRight);
        node.set_value(chunk_chars.iter().collect::<String>());
        node.set_character_lengths(
            chunk_chars
                .iter()
                .map(|c| c.len_utf8() as u8)
                .collect::<Vec<u8>>(),
        );
        node.set_word_starts(
            word_starts
                .iter()
                .filter(|&&word_start| word_start >= char_start && word_start < char_end)
                .map(|&word_start| (word_start - char_start) as u8)
                .collect::<Vec<u8>>(),
        );
        if chunk_index > 0 {
            node.set_previous_on_line(synthetic_node_id(chunk_index as u64 - 1));
        }
        if chunk_index + 1 < num_chunks {
            node.set_next_on_line(synthetic_node_id(chunk_index as u64 + 1));
        }

        runs.push((synthetic_node_id(chunk_index as u64), node));
    }

    let anchor = a11y_text_position(
        char_index_for_byte(text, selection_tail),
        &synthetic_node_id,
    );
    let focus = a11y_text_position(
        char_index_for_byte(text, selection_head),
        &synthetic_node_id,
    );
    (runs, accesskit::TextSelection { anchor, focus })
}

/// Expose the field's text content as AccessKit text runs, plus the current
/// selection/caret, enabling the platform's text pattern (e.g. UIA
/// TextPattern on Windows) so screen readers can track the caret, review the
/// text, and handle typed-character echo natively.
///
/// `selection_tail` and `selection_head` are byte offsets into `text`.
fn push_a11y_text_runs(
    builder: &mut A11ySubtreeBuilder,
    text: &str,
    selection_tail: usize,
    selection_head: usize,
) {
    let (runs, selection) = build_a11y_text_runs(text, selection_tail, selection_head, |chunk| {
        builder.synthetic_node_id(chunk)
    });
    for (id, node) in runs {
        builder.push_child(id, node);
    }
    builder.parent_node().set_text_selection(selection);
}

#[cfg(test)]
mod tests {
    use super::build_a11y_text_runs;
    use gpui::accesskit::NodeId;
    use gpui::proptest::strategy::Strategy;

    /// A strategy producing strings with a deliberate mix of character
    /// categories — ASCII, Latin accents, Cyrillic, Arabic, CJK, emoji, and
    /// arbitrary scalars — so run-splitting is exercised across scripts and
    /// byte widths (1–4 UTF-8 bytes). Lengths reach past one chunk (255 chars).
    fn arbitrary_text() -> impl Strategy<Value = String> {
        let character = gpui::proptest::prop_oneof![
            gpui::proptest::char::range(' ', '~'), // ASCII printable
            gpui::proptest::char::range('\u{00A1}', '\u{00FF}'), // Latin-1 (accents)
            gpui::proptest::char::range('\u{0100}', '\u{024F}'), // Latin Extended-A/B
            gpui::proptest::char::range('\u{0400}', '\u{04FF}'), // Cyrillic
            gpui::proptest::char::range('\u{0600}', '\u{06FF}'), // Arabic
            gpui::proptest::char::range('\u{4E00}', '\u{9FFF}'), // CJK Unified Ideographs
            gpui::proptest::char::range('\u{1F300}', '\u{1FAFF}'), // emoji & pictographs
            gpui::proptest::char::any(),           // anything else
        ];
        gpui::proptest::collection::vec(character, 0..600)
            .prop_map(|chars| chars.into_iter().collect::<String>())
    }

    /// Splitting an arbitrary string into AccessKit text runs must never panic,
    /// for any text and any byte selection offsets — including empty text, text
    /// spanning multiple chunks, multi-byte characters, and offsets past the end.
    #[gpui::property_test]
    fn building_text_runs_never_panics(
        #[strategy = arbitrary_text()] text: String,
        selection_tail: usize,
        selection_head: usize,
    ) {
        let _ = build_a11y_text_runs(&text, selection_tail, selection_head, NodeId);
    }
}
