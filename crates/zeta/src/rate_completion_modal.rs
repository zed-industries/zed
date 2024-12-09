use crate::{InlineCompletion, InlineCompletionRating, Zeta};
use editor::Editor;
use gpui::{
    prelude::*, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, HighlightStyle,
    Model, StyledText, TextStyle, View, ViewContext,
};
use language::{language_settings, OffsetRangeExt};
use settings::Settings;
use theme::ThemeSettings;
use ui::{prelude::*, ListItem, ListItemSpacing};
use workspace::{ModalView, Workspace};

pub struct RateCompletionModal {
    zeta: Model<Zeta>,
    active_completion: Option<ActiveCompletion>,
    focus_handle: FocusHandle,
    _subscription: gpui::Subscription,
}

struct ActiveCompletion {
    completion: InlineCompletion,
    feedback_editor: View<Editor>,
}

impl RateCompletionModal {
    pub fn toggle(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        if let Some(zeta) = Zeta::global(cx) {
            workspace.toggle_modal(cx, |cx| RateCompletionModal::new(zeta, cx));
        }
    }

    pub fn new(zeta: Model<Zeta>, cx: &mut ViewContext<Self>) -> Self {
        let subscription = cx.observe(&zeta, |_, _, cx| cx.notify());
        Self {
            zeta,
            focus_handle: cx.focus_handle(),
            active_completion: None,
            _subscription: subscription,
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }

    pub fn select_completion(
        &mut self,
        completion: Option<InlineCompletion>,
        cx: &mut ViewContext<Self>,
    ) {
        // Avoid resetting completion rating if it's already selected.
        if let Some(completion) = completion.as_ref() {
            if let Some(prev_completion) = self.active_completion.as_ref() {
                if completion.id == prev_completion.completion.id {
                    return;
                }
            }
        }

        self.active_completion = completion.map(|completion| ActiveCompletion {
            completion,
            feedback_editor: cx.new_view(|cx| {
                let mut editor = Editor::multi_line(cx);
                editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
                editor.set_show_line_numbers(false, cx);
                editor.set_show_git_diff_gutter(false, cx);
                editor.set_show_code_actions(false, cx);
                editor.set_show_runnables(false, cx);
                editor.set_show_wrap_guides(false, cx);
                editor.set_show_indent_guides(false, cx);
                editor.set_show_inline_completions(Some(false), cx);
                editor.set_placeholder_text("Your feedback about this completion...", cx);
                editor
            }),
        });
    }

    fn render_active_completion(&mut self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let active_completion = self.active_completion.as_ref()?;
        let completion_id = active_completion.completion.id;

        let mut diff = active_completion
            .completion
            .snapshot
            .text_for_range(active_completion.completion.excerpt_range.clone())
            .collect::<String>();

        let mut delta = 0;
        let mut diff_highlights = Vec::new();
        for (old_range, new_text) in active_completion.completion.edits.iter() {
            let old_range = old_range.to_offset(&active_completion.completion.snapshot);
            let old_start_in_text =
                old_range.start - active_completion.completion.excerpt_range.start + delta;
            let old_end_in_text =
                old_range.end - active_completion.completion.excerpt_range.start + delta;
            if old_start_in_text < old_end_in_text {
                diff_highlights.push((
                    old_start_in_text..old_end_in_text,
                    HighlightStyle {
                        background_color: Some(cx.theme().status().deleted_background),
                        strikethrough: Some(gpui::StrikethroughStyle {
                            thickness: px(1.),
                            color: Some(cx.theme().colors().text_muted),
                        }),
                        ..Default::default()
                    },
                ));
            }

            if !new_text.is_empty() {
                diff.insert_str(old_end_in_text, new_text);
                diff_highlights.push((
                    old_end_in_text..old_end_in_text + new_text.len(),
                    HighlightStyle {
                        background_color: Some(cx.theme().status().created_background),
                        ..Default::default()
                    },
                ));
                delta += new_text.len();
            }
        }

        let settings = ThemeSettings::get_global(cx).clone();
        let text_style = TextStyle {
            color: cx.theme().colors().editor_foreground,
            font_size: settings.buffer_font_size(cx).into(),
            font_family: settings.buffer_font.family,
            font_features: settings.buffer_font.features,
            font_fallbacks: settings.buffer_font.fallbacks,
            line_height: relative(settings.buffer_line_height.value()),
            font_weight: settings.buffer_font.weight,
            font_style: settings.buffer_font.style,
            ..Default::default()
        };

        let rated = self.zeta.read(cx).is_completion_rated(completion_id);
        Some(
            v_flex()
                .flex_1()
                .size_full()
                .gap_2()
                .child(h_flex().justify_center().children(if rated {
                    Some(
                        Label::new("This completion was already rated")
                            .color(Color::Muted)
                            .size(LabelSize::Large),
                    )
                } else if active_completion.completion.edits.is_empty() {
                    Some(
                        Label::new("This completion didn't produce any edits")
                            .color(Color::Warning)
                            .size(LabelSize::Large),
                    )
                } else {
                    None
                }))
                .child(
                    v_flex()
                        .id("diff")
                        .flex_1()
                        .flex_basis(relative(0.75))
                        .bg(cx.theme().colors().editor_background)
                        .overflow_y_scroll()
                        .p_2()
                        .border_color(cx.theme().colors().border)
                        .border_1()
                        .rounded_lg()
                        .child(StyledText::new(diff).with_highlights(&text_style, diff_highlights)),
                )
                .child(
                    div()
                        .flex_1()
                        .flex_basis(relative(0.25))
                        .bg(cx.theme().colors().editor_background)
                        .border_color(cx.theme().colors().border)
                        .border_1()
                        .rounded_lg()
                        .child(active_completion.feedback_editor.clone()),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .justify_end()
                        .child(
                            Button::new("bad", "👎 Bad Completion")
                                .size(ButtonSize::Large)
                                .disabled(rated)
                                .label_size(LabelSize::Large)
                                .color(Color::Error)
                                .on_click({
                                    let completion = active_completion.completion.clone();
                                    let feedback_editor = active_completion.feedback_editor.clone();
                                    cx.listener(move |this, _, cx| {
                                        this.zeta.update(cx, |zeta, cx| {
                                            zeta.rate_completion(
                                                &completion,
                                                InlineCompletionRating::Negative,
                                                feedback_editor.read(cx).text(cx),
                                                cx,
                                            )
                                        })
                                    })
                                }),
                        )
                        .child(
                            Button::new("good", "👍 Good Completion")
                                .size(ButtonSize::Large)
                                .disabled(rated)
                                .label_size(LabelSize::Large)
                                .color(Color::Success)
                                .on_click({
                                    let completion = active_completion.completion.clone();
                                    let feedback_editor = active_completion.feedback_editor.clone();
                                    cx.listener(move |this, _, cx| {
                                        this.zeta.update(cx, |zeta, cx| {
                                            zeta.rate_completion(
                                                &completion,
                                                InlineCompletionRating::Positive,
                                                feedback_editor.read(cx).text(cx),
                                                cx,
                                            )
                                        })
                                    })
                                }),
                        ),
                ),
        )
    }
}

impl Render for RateCompletionModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .bg(cx.theme().colors().elevated_surface_background)
            .w(cx.viewport_size().width - px(256.))
            .h(cx.viewport_size().height - px(256.))
            .rounded_lg()
            .shadow_lg()
            .p_2()
            .key_context("RateCompletionModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::dismiss))
            .child(
                div()
                    .id("completion_list")
                    .w_96()
                    .h_full()
                    .overflow_y_scroll()
                    .child(
                        ui::List::new()
                            .empty_message(
                                "No completions, use the editor to generate some and rate them!",
                            )
                            .children(self.zeta.read(cx).recent_completions().cloned().map(
                                |completion| {
                                    let selected =
                                        self.active_completion.as_ref().map_or(false, |selected| {
                                            selected.completion.id == completion.id
                                        });
                                    let rated =
                                        self.zeta.read(cx).is_completion_rated(completion.id);
                                    ListItem::new(completion.id)
                                        .spacing(ListItemSpacing::Sparse)
                                        .selected(selected)
                                        .end_slot(if rated {
                                            Icon::new(IconName::Check).color(Color::Success)
                                        } else if completion.edits.is_empty() {
                                            Icon::new(IconName::Ellipsis).color(Color::Muted)
                                        } else {
                                            Icon::new(IconName::Diff).color(Color::Muted)
                                        })
                                        .child(Label::new(
                                            completion.path.to_string_lossy().to_string(),
                                        ))
                                        .child(
                                            Label::new(format!("({})", completion.id))
                                                .color(Color::Muted)
                                                .size(LabelSize::XSmall),
                                        )
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.select_completion(Some(completion.clone()), cx);
                                        }))
                                },
                            )),
                    ),
            )
            .children(self.render_active_completion(cx))
            .on_mouse_down_out(cx.listener(|_, _, cx| cx.emit(DismissEvent)))
    }
}

impl EventEmitter<DismissEvent> for RateCompletionModal {}

impl FocusableView for RateCompletionModal {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for RateCompletionModal {}
