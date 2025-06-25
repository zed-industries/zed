use crate::actions::ShowSignatureHelp;
use crate::hover_popover::open_markdown_url;
use crate::{Editor, EditorSettings, ToggleAutoSignatureHelp, hover_markdown_style};
use gpui::{
    App, AppContext, Context, Entity, HighlightStyle, MouseButton, ScrollHandle, Size, Stateful,
    StyledText, Task, TextStyle, Window, combine_highlights,
};
use language::BufferSnapshot;
use markdown::{Markdown, MarkdownElement};
use multi_buffer::{Anchor, ToOffset};
use settings::Settings;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use text::Rope;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, Button, ButtonCommon, ButtonSize, Clickable, Div, FluentBuilder,
    InteractiveElement, IntoElement, Label, ParentElement, Pixels, Scrollbar, ScrollbarState,
    SharedString, StatefulInteractiveElement, Styled, StyledExt, div, px, relative,
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
        cx: &mut Context<Self>,
    ) -> bool {
        if !(self.signature_help_state.is_shown() || self.auto_signature_help_enabled(cx)) {
            return false;
        }
        let newest_selection = self.selections.newest::<usize>(cx);
        let head = newest_selection.head();

        if !newest_selection.is_empty() && head != newest_selection.tail() {
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
        let languages = lsp_store.read(cx).languages.clone();

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
                            for signature in &mut signature_help.signatures {
                                let text = Rope::from(signature.label.clone());
                                let highlights = language
                                    .highlight_text(&text, 0..signature.label.len())
                                    .into_iter()
                                    .flat_map(|(range, highlight_id)| {
                                        Some((range, highlight_id.style(&cx.theme().syntax())?))
                                    });
                                signature.highlights =
                                    combine_highlights(signature.highlights.clone(), highlights)
                                        .collect();
                            }
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
                        let scroll_handle = ScrollHandle::new();
                        let signature_help_popover = SignatureHelpPopover {
                            style: text_style,
                            signature: signature_help
                                .signatures
                                .into_iter()
                                .map(|s| SignatureHelp {
                                    label: s.label.into(),
                                    documentation: s.documentation.map(|documentation| {
                                        cx.new(|cx| {
                                            Markdown::new(
                                                documentation.into(),
                                                Some(languages.clone()),
                                                None,
                                                cx,
                                            )
                                        })
                                    }),
                                    highlights: s.highlights,
                                })
                                .collect::<Vec<_>>(),
                            current_signature: Rc::new(RefCell::new(
                                signature_help.active_signature,
                            )),
                            scrollbar_state: ScrollbarState::new(scroll_handle.clone()),
                            scroll_handle,
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

    pub fn has_multiple_signatures(&self) -> bool {
        self.popover
            .as_ref()
            .map_or(false, |popover| popover.signature.len() > 1)
    }
}

#[cfg(test)]
impl SignatureHelpState {
    pub fn task(&self) -> Option<&Task<()>> {
        self.task.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SignatureHelp {
    pub(crate) label: SharedString,
    documentation: Option<Entity<Markdown>>,
    highlights: Vec<(Range<usize>, HighlightStyle)>,
}

#[derive(Clone, Debug)]
pub struct SignatureHelpPopover {
    pub style: TextStyle,
    pub signature: Vec<SignatureHelp>,
    pub current_signature: Rc<RefCell<usize>>,
    pub(crate) scroll_handle: ScrollHandle,
    pub(crate) scrollbar_state: ScrollbarState,
}

impl SignatureHelpPopover {
    pub fn render(
        &mut self,
        max_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> AnyElement {
        let Some(signature) = self.signature.get(*self.current_signature.borrow()) else {
            return div().into_any_element();
        };
        let label = signature
            .label
            .clone()
            .when(signature.label.is_empty(), |_| "<No Parameters>".into());
        let signature_count = self.signature.len();
        let signature_label = div().max_w(max_size.width).px_2().py_0p5().child(
            StyledText::new(label)
                .with_default_highlights(&self.style, signature.highlights.iter().cloned()),
        );
        let signature = div()
            .id("signature_help_documentation")
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .flex()
            .flex_col()
            .max_h(max_size.height)
            .child(signature_label)
            .when_some(signature.documentation.clone(), |this, description| {
                this.children(vec![
                    div().border_primary(cx).border_1().into_any_element(),
                    div()
                        .max_w(max_size.width)
                        .px_2()
                        .py_0p5()
                        .child(
                            MarkdownElement::new(description, hover_markdown_style(window, cx))
                                .code_block_renderer(markdown::CodeBlockRenderer::Default {
                                    copy_button: false,
                                    border: false,
                                    copy_button_on_hover: false,
                                })
                                .on_url_click(open_markdown_url),
                        )
                        .into_any_element(),
                ])
            })
            .into_any_element();
        let controls = if self.signature.len() > 1 {
            let prev_button = div().flex().flex_row().justify_center().child({
                let current_signature = self.current_signature.clone();
                Button::new("signature_help_prev_button", "▴")
                    .size(ButtonSize::None)
                    .on_click(move |_, _, _| {
                        let mut current_signature = current_signature.borrow_mut();
                        if *current_signature == 0 {
                            *current_signature = signature_count - 1;
                        } else {
                            *current_signature -= 1;
                        }
                    })
                    .into_any_element()
            });
            let next_button = div().flex().flex_row().justify_center().child({
                let current_signature = self.current_signature.clone();
                Button::new("signature_help_next_button", "▾")
                    .size(ButtonSize::None)
                    .on_click(move |_, _, _| {
                        let mut current_signature = current_signature.borrow_mut();
                        if *current_signature + 1 == signature_count {
                            *current_signature = 0;
                        } else {
                            *current_signature += 1;
                        }
                    })
                    .into_any_element()
            });
            let page = div()
                .flex()
                .flex_row()
                .justify_center()
                .child(Label::new(format!(
                    "{} / {}",
                    *self.current_signature.borrow() + 1,
                    signature_count
                )));

            Some(
                div()
                    .flex()
                    .flex_col()
                    .children([prev_button, page, next_button])
                    .into_any_element(),
            )
        } else {
            None
        };
        div()
            .max_h(max_size.height)
            .elevation_2(cx)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .flex()
            .flex_row()
            .when_some(controls, |this, controls| {
                this.children(vec![
                    div().flex().items_end().child(controls),
                    div().border_primary(cx).border_1(),
                ])
            })
            .children(vec![
                signature,
                self.render_vertical_scrollbar(cx).into_any_element(),
            ])
            .into_any_element()
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Editor>) -> Stateful<Div> {
        div()
            .occlude()
            .id("signature_help_scroll")
            .on_mouse_move(cx.listener(|_, _, _, cx| {
                cx.notify();
                cx.stop_propagation()
            }))
            .on_hover(|_, _, cx| cx.stop_propagation())
            .on_any_mouse_down(|_, _, cx| cx.stop_propagation())
            .on_mouse_up(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_scroll_wheel(cx.listener(|_, _, _, cx| cx.notify()))
            .h_full()
            .absolute()
            .right_1()
            .top_1()
            .bottom_0()
            .w(px(12.))
            .cursor_default()
            .children(Scrollbar::vertical(self.scrollbar_state.clone()))
    }
}
