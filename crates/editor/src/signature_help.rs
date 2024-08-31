mod popover;
mod state;

use crate::actions::ShowSignatureHelp;
use crate::{Editor, EditorSettings, ToggleAutoSignatureHelp};
use gpui::{AppContext, ViewContext};
use language::markdown::parse_markdown;
use multi_buffer::{Anchor, ToOffset};
use settings::Settings;
use std::ops::Range;

pub use popover::SignatureHelpPopover;
pub use state::SignatureHelpState;

// Language-specific settings may define quotes as "brackets", so filter them out separately.
const QUOTE_PAIRS: [(&'static str, &'static str); 3] = [("'", "'"), ("\"", "\""), ("`", "`")];

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
        cx: &mut ViewContext<Self>,
    ) {
        self.auto_signature_help = self
            .auto_signature_help
            .map(|auto_signature_help| !auto_signature_help)
            .or_else(|| Some(!EditorSettings::get_global(cx).auto_signature_help));
        match self.auto_signature_help {
            Some(auto_signature_help) if auto_signature_help => {
                self.show_signature_help(&ShowSignatureHelp, cx);
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
        cx: &mut ViewContext<Self>,
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

    pub fn auto_signature_help_enabled(&self, cx: &AppContext) -> bool {
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
        cx: &mut ViewContext<Self>,
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
        let not_quote_like_brackets = |start: Range<usize>, end: Range<usize>| {
            let text = buffer_snapshot.text();
            let (text_start, text_end) = (text.get(start), text.get(end));
            QUOTE_PAIRS
                .into_iter()
                .all(|(start, end)| text_start != Some(start) && text_end != Some(end))
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

    pub fn show_signature_help(&mut self, _: &ShowSignatureHelp, cx: &mut ViewContext<Self>) {
        if self.pending_rename.is_some() || self.has_active_completions_menu() {
            return;
        }

        let position = self.selections.newest_anchor().head();
        let Some((buffer, buffer_position)) =
            self.buffer.read(cx).text_anchor_for_position(position, cx)
        else {
            return;
        };

        self.signature_help_state
            .set_task(cx.spawn(move |editor, mut cx| async move {
                let signature_help = editor
                    .update(&mut cx, |editor, cx| {
                        let language = editor.language_at(position, cx);
                        let project = editor.project.clone()?;
                        let (markdown, language_registry) = {
                            project.update(cx, |project, mut cx| {
                                let language_registry = project.languages().clone();
                                (
                                    project.signature_help(&buffer, buffer_position, &mut cx),
                                    language_registry,
                                )
                            })
                        };
                        Some((markdown, language_registry, language))
                    })
                    .ok()
                    .flatten();
                let signature_help_popover = if let Some((
                    signature_help_task,
                    language_registry,
                    language,
                )) = signature_help
                {
                    // TODO allow multiple signature helps inside the same popover
                    if let Some(mut signature_help) = signature_help_task.await.into_iter().next() {
                        let mut parsed_content = parse_markdown(
                            signature_help.markdown.as_str(),
                            &language_registry,
                            language,
                        )
                        .await;
                        parsed_content
                            .highlights
                            .append(&mut signature_help.highlights);
                        Some(SignatureHelpPopover { parsed_content })
                    } else {
                        None
                    }
                } else {
                    None
                };
                editor
                    .update(&mut cx, |editor, cx| {
                        let previous_popover = editor.signature_help_state.popover();
                        if previous_popover != signature_help_popover.as_ref() {
                            if let Some(signature_help_popover) = signature_help_popover {
                                editor
                                    .signature_help_state
                                    .set_popover(signature_help_popover);
                            } else {
                                editor
                                    .signature_help_state
                                    .hide(SignatureHelpHiddenBy::AutoClose);
                            }
                            cx.notify();
                        }
                    })
                    .ok();
            }));
    }
}
