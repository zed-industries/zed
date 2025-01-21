use crate::{CompletionDiffElement, InlineCompletion, InlineCompletionRating, Zeta};
use editor::Editor;
use gpui::{
    actions, prelude::*, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    View, ViewContext,
};
use language::language_settings;
use std::time::Duration;
use ui::{prelude::*, KeyBinding, List, ListItem, ListItemSpacing, Tooltip};
use workspace::{ModalView, Workspace};

actions!(
    zeta,
    [
        RateCompletions,
        ThumbsUpActiveCompletion,
        ThumbsDownActiveCompletion,
        NextEdit,
        PreviousEdit,
        FocusCompletions,
        PreviewCompletion,
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(move |workspace: &mut Workspace, _cx| {
        workspace.register_action(|workspace, _: &RateCompletions, cx| {
            RateCompletionModal::toggle(workspace, cx);
        });
    })
    .detach();
}

pub struct RateCompletionModal {
    zeta: Model<Zeta>,
    active_completion: Option<ActiveCompletion>,
    selected_index: usize,
    focus_handle: FocusHandle,
    _subscription: gpui::Subscription,
    current_view: RateCompletionView,
}

struct ActiveCompletion {
    completion: InlineCompletion,
    feedback_editor: View<Editor>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum RateCompletionView {
    SuggestedEdits,
    RawInput,
}

impl RateCompletionView {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SuggestedEdits => "Suggested Edits",
            Self::RawInput => "Recorded Events & Input",
        }
    }
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
            selected_index: 0,
            focus_handle: cx.focus_handle(),
            active_completion: None,
            _subscription: subscription,
            current_view: RateCompletionView::SuggestedEdits,
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }

    fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        self.selected_index += 1;
        self.selected_index = usize::min(
            self.selected_index,
            self.zeta.read(cx).shown_completions().count(),
        );
        cx.notify();
    }

    fn select_prev(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        self.selected_index = self.selected_index.saturating_sub(1);
        cx.notify();
    }

    fn select_next_edit(&mut self, _: &NextEdit, cx: &mut ViewContext<Self>) {
        let next_index = self
            .zeta
            .read(cx)
            .shown_completions()
            .skip(self.selected_index)
            .enumerate()
            .skip(1) // Skip straight to the next item
            .find(|(_, completion)| !completion.edits.is_empty())
            .map(|(ix, _)| ix + self.selected_index);

        if let Some(next_index) = next_index {
            self.selected_index = next_index;
            cx.notify();
        }
    }

    fn select_prev_edit(&mut self, _: &PreviousEdit, cx: &mut ViewContext<Self>) {
        let zeta = self.zeta.read(cx);
        let completions_len = zeta.shown_completions_len();

        let prev_index = self
            .zeta
            .read(cx)
            .shown_completions()
            .rev()
            .skip((completions_len - 1) - self.selected_index)
            .enumerate()
            .skip(1) // Skip straight to the previous item
            .find(|(_, completion)| !completion.edits.is_empty())
            .map(|(ix, _)| self.selected_index - ix);

        if let Some(prev_index) = prev_index {
            self.selected_index = prev_index;
            cx.notify();
        }
        cx.notify();
    }

    fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        self.selected_index = 0;
        cx.notify();
    }

    fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        self.selected_index = self.zeta.read(cx).shown_completions_len() - 1;
        cx.notify();
    }

    pub fn thumbs_up_active(&mut self, _: &ThumbsUpActiveCompletion, cx: &mut ViewContext<Self>) {
        self.zeta.update(cx, |zeta, cx| {
            if let Some(active) = &self.active_completion {
                zeta.rate_completion(
                    &active.completion,
                    InlineCompletionRating::Positive,
                    active.feedback_editor.read(cx).text(cx),
                    cx,
                );
            }
        });

        let current_completion = self
            .active_completion
            .as_ref()
            .map(|completion| completion.completion.clone());
        self.select_completion(current_completion, false, cx);
        self.select_next_edit(&Default::default(), cx);
        self.confirm(&Default::default(), cx);

        cx.notify();
    }

    pub fn thumbs_down_active(
        &mut self,
        _: &ThumbsDownActiveCompletion,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(active) = &self.active_completion {
            if active.feedback_editor.read(cx).text(cx).is_empty() {
                return;
            }

            self.zeta.update(cx, |zeta, cx| {
                zeta.rate_completion(
                    &active.completion,
                    InlineCompletionRating::Negative,
                    active.feedback_editor.read(cx).text(cx),
                    cx,
                );
            });
        }

        let current_completion = self
            .active_completion
            .as_ref()
            .map(|completion| completion.completion.clone());
        self.select_completion(current_completion, false, cx);
        self.select_next_edit(&Default::default(), cx);
        self.confirm(&Default::default(), cx);

        cx.notify();
    }

    fn focus_completions(&mut self, _: &FocusCompletions, cx: &mut ViewContext<Self>) {
        cx.focus_self();
        cx.notify();
    }

    fn preview_completion(&mut self, _: &PreviewCompletion, cx: &mut ViewContext<Self>) {
        let completion = self
            .zeta
            .read(cx)
            .shown_completions()
            .skip(self.selected_index)
            .take(1)
            .next()
            .cloned();

        self.select_completion(completion, false, cx);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let completion = self
            .zeta
            .read(cx)
            .shown_completions()
            .skip(self.selected_index)
            .take(1)
            .next()
            .cloned();

        self.select_completion(completion, true, cx);
    }

    pub fn select_completion(
        &mut self,
        completion: Option<InlineCompletion>,
        focus: bool,
        cx: &mut ViewContext<Self>,
    ) {
        // Avoid resetting completion rating if it's already selected.
        if let Some(completion) = completion.as_ref() {
            self.selected_index = self
                .zeta
                .read(cx)
                .shown_completions()
                .enumerate()
                .find(|(_, completion_b)| completion.id == completion_b.id)
                .map(|(ix, _)| ix)
                .unwrap_or(self.selected_index);
            cx.notify();

            if let Some(prev_completion) = self.active_completion.as_ref() {
                if completion.id == prev_completion.completion.id {
                    if focus {
                        cx.focus_view(&prev_completion.feedback_editor);
                    }
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
                editor.set_show_scrollbars(false, cx);
                editor.set_show_git_diff_gutter(false, cx);
                editor.set_show_code_actions(false, cx);
                editor.set_show_runnables(false, cx);
                editor.set_show_wrap_guides(false, cx);
                editor.set_show_indent_guides(false, cx);
                editor.set_show_inline_completions(Some(false), cx);
                editor.set_placeholder_text("Add your feedback…", cx);
                if focus {
                    cx.focus_self();
                }
                editor
            }),
        });
        cx.notify();
    }

    fn render_view_nav(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .h_8()
            .px_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().elevated_surface_background)
            .gap_1()
            .child(
                Button::new(
                    ElementId::Name("suggested-edits".into()),
                    RateCompletionView::SuggestedEdits.name(),
                )
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, cx| {
                    this.current_view = RateCompletionView::SuggestedEdits;
                    cx.notify();
                }))
                .toggle_state(self.current_view == RateCompletionView::SuggestedEdits),
            )
            .child(
                Button::new(
                    ElementId::Name("raw-input".into()),
                    RateCompletionView::RawInput.name(),
                )
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, cx| {
                    this.current_view = RateCompletionView::RawInput;
                    cx.notify();
                }))
                .toggle_state(self.current_view == RateCompletionView::RawInput),
            )
    }

    fn render_suggested_edits(&self, cx: &mut ViewContext<Self>) -> Option<gpui::Stateful<Div>> {
        let active_completion = self.active_completion.as_ref()?;
        let bg_color = cx.theme().colors().editor_background;

        Some(
            div()
                .id("diff")
                .p_4()
                .size_full()
                .bg(bg_color)
                .overflow_scroll()
                .whitespace_nowrap()
                .child(CompletionDiffElement::new(
                    &active_completion.completion,
                    cx,
                )),
        )
    }

    fn render_raw_input(&self, cx: &mut ViewContext<Self>) -> Option<gpui::Stateful<Div>> {
        Some(
            v_flex()
                .size_full()
                .overflow_hidden()
                .relative()
                .child(
                    div()
                        .id("raw-input")
                        .py_4()
                        .px_6()
                        .size_full()
                        .bg(cx.theme().colors().editor_background)
                        .overflow_scroll()
                        .child(if let Some(active_completion) = &self.active_completion {
                            format!(
                                "{}\n{}",
                                active_completion.completion.input_events,
                                active_completion.completion.input_excerpt
                            )
                        } else {
                            "No active completion".to_string()
                        }),
                )
                .id("raw-input-view"),
        )
    }

    fn render_active_completion(&mut self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let active_completion = self.active_completion.as_ref()?;
        let completion_id = active_completion.completion.id;
        let focus_handle = &self.focus_handle(cx);

        let border_color = cx.theme().colors().border;
        let bg_color = cx.theme().colors().editor_background;

        let rated = self.zeta.read(cx).is_completion_rated(completion_id);
        let feedback_empty = active_completion
            .feedback_editor
            .read(cx)
            .text(cx)
            .is_empty();

        let label_container = h_flex().pl_1().gap_1p5();

        Some(
            v_flex()
                .size_full()
                .overflow_hidden()
                .relative()
                .child(
                    v_flex()
                        .size_full()
                        .overflow_hidden()
                        .relative()
                        .child(self.render_view_nav(cx))
                        .when_some(match self.current_view {
                            RateCompletionView::SuggestedEdits => self.render_suggested_edits(cx),
                            RateCompletionView::RawInput => self.render_raw_input(cx),
                        }, |this, element| this.child(element))
                )
                .when(!rated, |this| {
                    this.child(
                        h_flex()
                            .p_2()
                            .gap_2()
                            .border_y_1()
                            .border_color(border_color)
                            .child(
                                Icon::new(IconName::Info)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted)
                            )
                            .child(
                                div()
                                    .w_full()
                                    .pr_2()
                                    .flex_wrap()
                                    .child(
                                        Label::new("Explain why this completion is good or bad. If it's negative, describe what you expected instead.")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                    )
                            )
                    )
                })
                .when(!rated, |this| {
                    this.child(
                        div()
                            .h_40()
                            .pt_1()
                            .bg(bg_color)
                            .child(active_completion.feedback_editor.clone())
                    )
                })
                .child(
                    h_flex()
                        .p_1()
                        .h_8()
                        .max_h_8()
                        .border_t_1()
                        .border_color(border_color)
                        .max_w_full()
                        .justify_between()
                        .children(if rated {
                            Some(
                                label_container
                                    .child(
                                        Icon::new(IconName::Check)
                                            .size(IconSize::Small)
                                            .color(Color::Success),
                                    )
                                    .child(Label::new("Rated completion.").color(Color::Muted)),
                            )
                        } else if active_completion.completion.edits.is_empty() {
                            Some(
                                label_container
                                    .child(
                                        Icon::new(IconName::Warning)
                                            .size(IconSize::Small)
                                            .color(Color::Warning),
                                    )
                                    .child(Label::new("No edits produced.").color(Color::Muted)),
                            )
                        } else {
                            Some(label_container)
                        })
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Button::new("bad", "Bad Completion")
                                        .icon(IconName::ThumbsDown)
                                        .icon_size(IconSize::Small)
                                        .icon_position(IconPosition::Start)
                                        .disabled(rated || feedback_empty)
                                        .when(feedback_empty, |this| {
                                            this.tooltip(|cx| {
                                                Tooltip::text("Explain what's bad about it before reporting it", cx)
                                            })
                                        })
                                        .key_binding(KeyBinding::for_action_in(
                                            &ThumbsDownActiveCompletion,
                                            focus_handle,
                                            cx,
                                        ))
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.thumbs_down_active(
                                                &ThumbsDownActiveCompletion,
                                                cx,
                                            );
                                        })),
                                )
                                .child(
                                    Button::new("good", "Good Completion")
                                        .icon(IconName::ThumbsUp)
                                        .icon_size(IconSize::Small)
                                        .icon_position(IconPosition::Start)
                                        .disabled(rated)
                                        .key_binding(KeyBinding::for_action_in(
                                            &ThumbsUpActiveCompletion,
                                            focus_handle,
                                            cx,
                                        ))
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.thumbs_up_active(&ThumbsUpActiveCompletion, cx);
                                        })),
                                ),
                        ),
                ),
        )
    }
}

impl Render for RateCompletionModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let border_color = cx.theme().colors().border;

        h_flex()
            .key_context("RateCompletionModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_prev_edit))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_next_edit))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::thumbs_up_active))
            .on_action(cx.listener(Self::thumbs_down_active))
            .on_action(cx.listener(Self::focus_completions))
            .on_action(cx.listener(Self::preview_completion))
            .bg(cx.theme().colors().elevated_surface_background)
            .border_1()
            .border_color(border_color)
            .w(cx.viewport_size().width - px(320.))
            .h(cx.viewport_size().height - px(300.))
            .rounded_lg()
            .shadow_lg()
            .child(
                v_flex()
                    .w_72()
                    .h_full()
                    .border_r_1()
                    .border_color(border_color)
                    .flex_shrink_0()
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .h_8()
                            .px_2()
                            .justify_between()
                            .border_b_1()
                            .border_color(border_color)
                            .child(
                                Icon::new(IconName::ZedPredict)
                                    .size(IconSize::Small)
                            )
                            .child(
                                Label::new("From most recent to oldest")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            )
                    )
                    .child(
                        div()
                            .id("completion_list")
                            .p_0p5()
                            .h_full()
                            .overflow_y_scroll()
                            .child(
                                List::new()
                                    .empty_message(
                                        div()
                                            .p_2()
                                            .child(
                                                Label::new("No completions yet. Use the editor to generate some, and make sure to rate them!")
                                                    .color(Color::Muted),
                                            )
                                            .into_any_element(),
                                    )
                                    .children(self.zeta.read(cx).shown_completions().cloned().enumerate().map(
                                        |(index, completion)| {
                                            let selected =
                                                self.active_completion.as_ref().map_or(false, |selected| {
                                                    selected.completion.id == completion.id
                                                });
                                            let rated =
                                                self.zeta.read(cx).is_completion_rated(completion.id);

                                            let (icon_name, icon_color, tooltip_text) = match (rated, completion.edits.is_empty()) {
                                                (true, _) => (IconName::Check, Color::Success, "Rated Completion"),
                                                (false, true) => (IconName::File, Color::Muted, "No Edits Produced"),
                                                (false, false) => (IconName::FileDiff, Color::Accent, "Edits Available"),
                                            };

                                            let file_name = completion.path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or("untitled".to_string());
                                            let file_path = completion.path.parent().map(|p| p.to_string_lossy().to_string());

                                            ListItem::new(completion.id)
                                                .inset(true)
                                                .spacing(ListItemSpacing::Sparse)
                                                .focused(index == self.selected_index)
                                                .toggle_state(selected)
                                                .child(
                                                    h_flex()
                                                        .id("completion-content")
                                                        .gap_3()
                                                        .child(
                                                            Icon::new(icon_name)
                                                                .color(icon_color)
                                                                .size(IconSize::Small)
                                                        )
                                                        .child(
                                                            v_flex()
                                                                .child(
                                                                    h_flex().gap_1()
                                                                        .child(Label::new(file_name).size(LabelSize::Small))
                                                                        .when_some(file_path, |this, p| this.child(Label::new(p).size(LabelSize::Small).color(Color::Muted)))
                                                                )
                                                                .child(Label::new(format!("{} ago, {:.2?}", format_time_ago(completion.response_received_at.elapsed()), completion.latency()))
                                                                    .color(Color::Muted)
                                                                    .size(LabelSize::XSmall)
                                                                )
                                                        )
                                                )
                                                .tooltip(move |cx| {
                                                    Tooltip::text(tooltip_text, cx)
                                                })
                                                .on_click(cx.listener(move |this, _, cx| {
                                                    this.select_completion(Some(completion.clone()), true, cx);
                                                }))
                                        },
                                    )),
                            )
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

fn format_time_ago(elapsed: Duration) -> String {
    let seconds = elapsed.as_secs();
    if seconds < 120 {
        "1 minute".to_string()
    } else if seconds < 3600 {
        format!("{} minutes", seconds / 60)
    } else if seconds < 7200 {
        "1 hour".to_string()
    } else if seconds < 86400 {
        format!("{} hours", seconds / 3600)
    } else if seconds < 172800 {
        "1 day".to_string()
    } else {
        format!("{} days", seconds / 86400)
    }
}
