use crate::actions::ShowSignatureHelp;
use crate::{Editor, EditorSettings, ToggleAutoSignatureHelp};
use gpui::{
    App, Context, HighlightStyle, MouseButton, Size, StyledText, Task, TextStyle, Window,
    combine_highlights,
};
use language::BufferSnapshot;
use multi_buffer::{Anchor, ToOffset};
use settings::Settings;
use std::ops::Range;
use text::Rope;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, InteractiveElement, IntoElement, ParentElement, Pixels, SharedString,
    StatefulInteractiveElement, Styled, StyledExt, div, relative,
};

// Language-specific settings may define quotes as "brackets", so filter them out separately.
const QUOTE_PAIRS: [(&str, &str); 3] = [("'", "'"), ("\"", "\""), ("`", "`")];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignatureHelpHiddenBy {
    AutoClose,
    Escape,
    Selection,
}

impl Editor {
    pub fn toggle_auto_signature_help_menu(
        &mut self,
        _: &ToggleAutoSignatureHelp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.auto_signature_help = self
            .auto_signature_help
            .map(|auto_signature_help| !auto_signature_help)
            .or_else(|| Some(!EditorSettings::get_global(cx).auto_signature_help));
        match self.auto_signature_help {
            Some(auto_signature_help) if auto_signature_help => {
                self.show_signature_help(&ShowSignatureHelp, window, cx);
            }
            Some(_) => {
                self.hide_signature_help(cx, SignatureHelpHiddenBy::AutoClose);
            }
            None => {}
        }
        cx.notify();
    }

    pub(super) fn hide_signature_help(
        &mut self,
        cx: &mut Context<Self>,
        signature_help_hidden_by: SignatureHelpHiddenBy,
    ) -> bool {
        if self.signature_help_state.is_shown() {
            self.signature_help_state.kill_task();
            self.signature_help_state.hide(signature_help_hidden_by);
            cx.notify();
            true
        } else {
            false
        }
    }

    pub fn auto_signature_help_enabled(&self, cx: &App) -> bool {
        if let Some(auto_signature_help) = self.auto_signature_help {
            auto_signature_help
        } else {
            EditorSettings::get_global(cx).auto_signature_help
        }
    }

    pub(super) fn should_open_signature_help_automatically(
        &mut self,
        old_cursor_position: &Anchor,
        backspace_pressed: bool,

        cx: &mut Context<Self>,
    ) -> bool {
        if !(self.signature_help_state.is_shown() || self.auto_signature_help_enabled(cx)) {
            return false;
        }
        let newest_selection = self.selections.newest::<usize>(cx);
        let head = newest_selection.head();

        // There are two cases where the head and tail of a selection are different: selecting multiple ranges and using backspace.
        // If we don’t exclude the backspace case, signature_help will blink every time backspace is pressed, so we need to prevent this.
        if !newest_selection.is_empty() && !backspace_pressed && head != newest_selection.tail() {
            self.signature_help_state
                .hide(SignatureHelpHiddenBy::Selection);
            return false;
        }

        let buffer_snapshot = self.buffer().read(cx).snapshot(cx);
        let bracket_range = |position: usize| match (position, position + 1) {
            (0, b) if b <= buffer_snapshot.len() => 0..b,
            (0, b) => 0..b - 1,
            (a, b) if b <= buffer_snapshot.len() => a - 1..b,
            (a, b) => a - 1..b - 1,
        };
        let not_quote_like_brackets =
            |buffer: &BufferSnapshot, start: Range<usize>, end: Range<usize>| {
                let text_start = buffer.text_for_range(start).collect::<String>();
                let text_end = buffer.text_for_range(end).collect::<String>();
                QUOTE_PAIRS
                    .into_iter()
                    .all(|(start, end)| text_start != start && text_end != end)
            };

        let previous_position = old_cursor_position.to_offset(&buffer_snapshot);
        let previous_brackets_range = bracket_range(previous_position);
        let previous_brackets_surround = buffer_snapshot
            .innermost_enclosing_bracket_ranges(
                previous_brackets_range,
                Some(&not_quote_like_brackets),
            )
            .filter(|(start_bracket_range, end_bracket_range)| {
                start_bracket_range.start != previous_position
                    && end_bracket_range.end != previous_position
            });
        let current_brackets_range = bracket_range(head);
        let current_brackets_surround = buffer_snapshot
            .innermost_enclosing_bracket_ranges(
                current_brackets_range,
                Some(&not_quote_like_brackets),
            )
            .filter(|(start_bracket_range, end_bracket_range)| {
                start_bracket_range.start != head && end_bracket_range.end != head
            });

        match (previous_brackets_surround, current_brackets_surround) {
            (None, None) => {
                self.signature_help_state
                    .hide(SignatureHelpHiddenBy::AutoClose);
                false
            }
            (Some(_), None) => {
                self.signature_help_state
                    .hide(SignatureHelpHiddenBy::AutoClose);
                false
            }
            (None, Some(_)) => true,
            (Some(previous), Some(current)) => {
                let condition = self.signature_help_state.hidden_by_selection()
                    || previous != current
                    || (previous == current && self.signature_help_state.is_shown());
                if !condition {
                    self.signature_help_state
                        .hide(SignatureHelpHiddenBy::AutoClose);
                }
                condition
            }
        }
    }

    pub fn show_signature_help(
        &mut self,
        _: &ShowSignatureHelp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.pending_rename.is_some() || self.has_visible_completions_menu() {
            return;
        }

        let position = self.selections.newest_anchor().head();
        let Some((buffer, buffer_position)) =
            self.buffer.read(cx).text_anchor_for_position(position, cx)
        else {
            return;
        };
        let Some(lsp_store) = self.project.as_ref().map(|p| p.read(cx).lsp_store()) else {
            return;
        };
        let task = lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.signature_help(&buffer, buffer_position, cx)
        });
        let language = self.language_at(position, cx);

        self.signature_help_state
            .set_task(cx.spawn_in(window, async move |editor, cx| {
                let signature_help = task.await;
                editor
                    .update(cx, |editor, cx| {
                        let Some(mut signature_help) = signature_help.into_iter().next() else {
                            editor
                                .signature_help_state
                                .hide(SignatureHelpHiddenBy::AutoClose);
                            return;
                        };

                        if let Some(language) = language {
                            let text = Rope::from(signature_help.label.clone());
                            let highlights = language
                                .highlight_text(&text, 0..signature_help.label.len())
                                .into_iter()
                                .flat_map(|(range, highlight_id)| {
                                    Some((range, highlight_id.style(&cx.theme().syntax())?))
                                });
                            signature_help.highlights =
                                combine_highlights(signature_help.highlights, highlights).collect()
                        }
                        let settings = ThemeSettings::get_global(cx);
                        let text_style = TextStyle {
                            color: cx.theme().colors().text,
                            font_family: settings.buffer_font.family.clone(),
                            font_fallbacks: settings.buffer_font.fallbacks.clone(),
                            font_size: settings.buffer_font_size(cx).into(),
                            font_weight: settings.buffer_font.weight,
                            line_height: relative(settings.buffer_line_height.value()),
                            ..Default::default()
                        };

                        let signature_help_popover = SignatureHelpPopover {
                            label: signature_help.label.into(),
                            highlights: signature_help.highlights,
                            style: text_style,
                        };
                        editor
                            .signature_help_state
                            .set_popover(signature_help_popover);
                        cx.notify();
                    })
                    .ok();
            }));
    }
}

#[derive(Default, Debug)]
pub struct SignatureHelpState {
    task: Option<Task<()>>,
    popover: Option<SignatureHelpPopover>,
    hidden_by: Option<SignatureHelpHiddenBy>,
    backspace_pressed: bool,
}

impl SignatureHelpState {
    pub fn set_task(&mut self, task: Task<()>) {
        self.task = Some(task);
        self.hidden_by = None;
    }

    pub fn kill_task(&mut self) {
        self.task = None;
    }

    #[cfg(test)]
    pub fn popover(&self) -> Option<&SignatureHelpPopover> {
        self.popover.as_ref()
    }

    pub fn popover_mut(&mut self) -> Option<&mut SignatureHelpPopover> {
        self.popover.as_mut()
    }

    pub fn backspace_pressed(&self) -> bool {
        self.backspace_pressed
    }

    pub fn set_backspace_pressed(&mut self, backspace_pressed: bool) {
        self.backspace_pressed = backspace_pressed;
    }

    pub fn set_popover(&mut self, popover: SignatureHelpPopover) {
        self.popover = Some(popover);
        self.hidden_by = None;
    }

    pub fn hide(&mut self, hidden_by: SignatureHelpHiddenBy) {
        if self.hidden_by.is_none() {
            self.popover = None;
            self.hidden_by = Some(hidden_by);
        }
    }

    pub fn hidden_by_selection(&self) -> bool {
        self.hidden_by == Some(SignatureHelpHiddenBy::Selection)
    }

    pub fn is_shown(&self) -> bool {
        self.popover.is_some()
    }
}

#[cfg(test)]
impl SignatureHelpState {
    pub fn task(&self) -> Option<&Task<()>> {
        self.task.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SignatureHelpPopover {
    pub label: SharedString,
    pub style: TextStyle,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
}

impl SignatureHelpPopover {
    pub fn render(&mut self, max_size: Size<Pixels>, cx: &mut Context<Editor>) -> AnyElement {
        div()
            .id("signature_help_popover")
            .elevation_2(cx)
            .overflow_y_scroll()
            .max_w(max_size.width)
            .max_h(max_size.height)
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div().px_2().py_0p5().child(
                    StyledText::new(self.label.clone())
                        .with_default_highlights(&self.style, self.highlights.iter().cloned()),
                ),
            )
            .into_any_element()
    }
}
