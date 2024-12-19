use editor::Editor;
use gpui::{
    actions, prelude::*, AnyElement, AppContext, ClickEvent, DismissEvent, EventEmitter,
    FocusHandle, FocusableView, HighlightStyle, Model, StyledText, TextStyle, View, ViewContext,
};
use language::{language_settings, OffsetRangeExt};
use settings::Settings;
use std::{iter, time::Duration};
use telemetry_events::InlineCompletionRating as InlineResponseRating;
use theme::ThemeSettings;
use ui::{prelude::*, KeyBinding, List, ListItem, ListItemSpacing, Tab, TabBar, Tooltip};
use workspace::{ModalView, Workspace};
use zeta::{InlineCompletion as InlineResponse, Zeta};

actions!(
    llm_feedback,
    [
        RateResponses,
        ThumbsUp,
        ThumbsDown,
        ThumbsUpActiveResponse,
        ThumbsDownActiveResponse,
        NextEdit,
        PreviousEdit,
        FocusResponse,
        PreviewResponse,
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(move |workspace: &mut Workspace, _cx| {
        workspace.register_action(|workspace, _: &RateResponses, cx| {
            RateResponseModal::toggle(workspace, cx);
        });
    })
    .detach();
}

struct ZetaFeedback {
    zeta: Model<Zeta>,
    active_completion: Option<(usize, ActiveResponse)>,
    selected_index: usize,
    _subscription: gpui::Subscription,
}

impl ZetaFeedback {
    fn render_index(
        &self,
        index: usize,
        cx: &mut ViewContext<RateResponseModal>,
    ) -> Option<AnyElement> {
        let completion = self.zeta.read(cx).recent_completion(index).cloned()?;

        let selected = self
            .active_completion
            .as_ref()
            .map_or(false, |(_, active_completion)| {
                active_completion.completion.id == completion.id
            });
        let rated = self.zeta.read(cx).is_completion_rated(completion.id);

        Some(
            ListItem::new(completion.id)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .focused(index == self.selected_index)
                .toggle_state(selected)
                .start_slot(if rated {
                    Icon::new(IconName::Check)
                        .color(Color::Success)
                        .size(IconSize::Small)
                } else if completion.edits.is_empty() {
                    Icon::new(IconName::File)
                        .color(Color::Muted)
                        .size(IconSize::Small)
                } else {
                    Icon::new(IconName::FileDiff)
                        .color(Color::Accent)
                        .size(IconSize::Small)
                })
                .child(
                    v_flex()
                        .pl_1p5()
                        .child(
                            Label::new(completion.path.to_string_lossy().to_string())
                                .size(LabelSize::Small),
                        )
                        .child(
                            Label::new(format!(
                                "{} ago, {:.2?}",
                                format_time_ago(completion.response_received_at.elapsed()),
                                completion.latency()
                            ))
                            .color(Color::Muted)
                            .size(LabelSize::XSmall),
                        ),
                )
                .on_click(cx.listener(move |this, _, cx| {
                    this.tab.select_completion(Some(index), true, cx);
                }))
                .into_any_element(),
        )
    }

    fn render_active(
        &mut self,
        modal_handle: FocusHandle,
        cx: &mut ViewContext<RateResponseModal>,
    ) -> Option<AnyElement> {
        let (_, active_completion) = self.active_completion.as_ref()?;
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
        let was_shown = self.zeta.read(cx).was_completion_shown(completion_id);
        let feedback_empty = active_completion
            .feedback_editor
            .read(cx)
            .text(cx)
            .is_empty();

        let border_color = cx.theme().colors().border;
        let bg_color = cx.theme().colors().editor_background;

        let label_container = || h_flex().pl_1().gap_1p5();

        Some(
            v_flex()
                .size_full()
                .overflow_hidden()
                .child(
                    div()
                        .id("diff")
                        .py_4()
                        .px_6()
                        .size_full()
                        .bg(bg_color)
                        .overflow_scroll()
                        .child(StyledText::new(diff).with_highlights(&text_style, diff_highlights)),
                )
                .when_some((!rated).then(|| ()), |this, _| {
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
                                        Label::new("Ensure you explain why this completion is negative or positive. In case it's negative, report what you expected instead.")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                    )
                            )
                    )
                })
                .when_some((!rated).then(|| ()), |this, _| {
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
                                label_container()
                                    .child(
                                        Icon::new(IconName::Check)
                                            .size(IconSize::Small)
                                            .color(Color::Success),
                                    )
                                    .child(Label::new("Rated completion.").color(Color::Muted)),
                            )
                        } else if active_completion.completion.edits.is_empty() {
                            Some(
                                label_container()
                                    .child(
                                        Icon::new(IconName::Warning)
                                            .size(IconSize::Small)
                                            .color(Color::Warning),
                                    )
                                    .child(Label::new("No edits produced.").color(Color::Muted)),
                            )
                        } else if !was_shown {
                            Some(
                                label_container()
                                    .child(
                                        Icon::new(IconName::Warning)
                                            .size(IconSize::Small)
                                            .color(Color::Warning),
                                    )
                                    .child(Label::new("Response wasn't shown because another valid one was already on screen.")),
                            )
                        } else {
                            Some(label_container())
                        })
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Button::new("bad", "Bad Response")
                                        .key_binding(KeyBinding::for_action_in(
                                            &ThumbsDown,
                                            &modal_handle,
                                            cx,
                                        ))
                                        .style(ButtonStyle::Filled)
                                        .icon(IconName::ThumbsDown)
                                        .icon_size(IconSize::Small)
                                        .icon_position(IconPosition::Start)
                                        .disabled(rated || feedback_empty)
                                        .when(feedback_empty, |this| {
                                            this.tooltip(|cx| {
                                                Tooltip::text("Explain what's bad about it before reporting it", cx)
                                            })
                                        })
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.thumbs_down_active(
                                                &ThumbsDownActiveResponse,
                                                cx,
                                            );
                                        })),
                                )
                                .child(
                                    Button::new("good", "Good Response")
                                        .key_binding(KeyBinding::for_action_in(
                                            &ThumbsUp,
                                            &modal_handle,
                                            cx,
                                        ))
                                        .style(ButtonStyle::Filled)
                                        .icon(IconName::ThumbsUp)
                                        .icon_size(IconSize::Small)
                                        .icon_position(IconPosition::Start)
                                        .disabled(rated)
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.thumbs_up_active(&ThumbsUpActiveResponse, cx);
                                        })),
                                ),
                        ),
                ).into_any()
        )
    }
}

enum FeedbackTabs {
    Zeta(ZetaFeedback),
    InlineAssist(usize),
    Chat(usize),
}

impl FeedbackTabs {
    fn selected_index_mut(&mut self) -> &mut usize {
        match self {
            FeedbackTabs::Zeta(feedback) => &mut feedback.selected_index,
            FeedbackTabs::InlineAssist(index) => index,
            FeedbackTabs::Chat(index) => index,
        }
    }

    fn responses_length(&mut self, cx: &mut WindowContext) -> usize {
        match self {
            FeedbackTabs::Zeta(feedback) => feedback.zeta.read(cx).recent_completions().count(),
            FeedbackTabs::InlineAssist(_index) => 0,
            FeedbackTabs::Chat(_index) => 0,
        }
    }

    fn is_useful_response(&mut self, index: usize, cx: &mut WindowContext) -> bool {
        match self {
            FeedbackTabs::Zeta(feedback) => feedback
                .zeta
                .read(cx)
                .recent_completion(index)
                .map(|completion| !completion.edits.is_empty())
                .unwrap_or(false),

            FeedbackTabs::InlineAssist(_index) => {
                todo!("Implement is_useful_response for InlineAssist")
            }
            FeedbackTabs::Chat(_index) => todo!("Implement is_useful_response for Chat"),
        }
    }

    fn thumbs_up_selected(&self, cx: &mut WindowContext) {
        match self {
            FeedbackTabs::Zeta(feedback) => {
                feedback.zeta.update(cx, |zeta, cx| {
                    let completion = zeta
                        .recent_completions()
                        .skip(feedback.selected_index)
                        .next()
                        .cloned();

                    if let Some(completion) = completion {
                        zeta.rate_completion(
                            &completion,
                            InlineResponseRating::Positive,
                            "".to_string(),
                            cx,
                        );
                    }
                });
            }
            FeedbackTabs::InlineAssist(_index) => todo!("Implement thumbs_up for InlineAssist"),
            FeedbackTabs::Chat(_index) => todo!("Implement thumbs_up for Chat"),
        };
    }

    fn thumbs_up_active(&self, cx: &mut WindowContext) {
        match self {
            FeedbackTabs::Zeta(feedback) => {
                feedback.zeta.update(cx, |zeta, cx| {
                    if let Some((_, active)) = &feedback.active_completion {
                        zeta.rate_completion(
                            &active.completion,
                            InlineResponseRating::Positive,
                            active.feedback_editor.read(cx).text(cx),
                            cx,
                        );
                    }
                });
            }
            FeedbackTabs::InlineAssist(_index) => todo!("Implement thumbs_up for InlineAssist"),
            FeedbackTabs::Chat(_index) => todo!("Implement thumbs_up for Chat"),
        };
    }

    fn thumbs_down_active(&self, cx: &mut WindowContext) {
        match self {
            FeedbackTabs::Zeta(zeta_feedback) => {
                if let Some((_, active)) = &zeta_feedback.active_completion {
                    if active.feedback_editor.read(cx).text(cx).is_empty() {
                        return;
                    }

                    zeta_feedback.zeta.update(cx, |zeta, cx| {
                        zeta.rate_completion(
                            &active.completion,
                            InlineResponseRating::Negative,
                            active.feedback_editor.read(cx).text(cx),
                            cx,
                        );
                    });
                }
            }
            FeedbackTabs::InlineAssist(_) => todo!(),
            FeedbackTabs::Chat(_) => todo!(),
        }
    }

    pub fn active_response(&self) -> Option<usize> {
        match self {
            FeedbackTabs::Zeta(zeta_feedback) => {
                zeta_feedback.active_completion.as_ref().map(|(ix, _)| *ix)
            }
            FeedbackTabs::InlineAssist(_) => todo!(),
            FeedbackTabs::Chat(_) => todo!(),
        }
    }

    pub fn select_completion(
        &mut self,
        completion_ix: Option<usize>, // If None, use internal active completion, ELSE use the provided index
        focus: bool,
        cx: &mut WindowContext,
    ) {
        match self {
            FeedbackTabs::Zeta(zeta_tab) => {
                if let Some(completion_ix) = completion_ix {
                    let zeta = zeta_tab.zeta.read(cx);
                    // Avoid resetting completion rating if it's already selected.
                    if let Some(completion) = zeta.recent_completion(completion_ix) {
                        if let Some((_, prev_completion)) = zeta_tab.active_completion.as_ref() {
                            if completion.id == prev_completion.completion.id {
                                if focus {
                                    cx.focus_view(&prev_completion.feedback_editor);
                                }
                                return;
                            }
                        }

                        zeta_tab.active_completion = Some((
                            completion_ix,
                            ActiveResponse {
                                completion: completion.clone(),
                                feedback_editor: cx.new_view(|cx| {
                                    let mut editor = Editor::multi_line(cx);
                                    editor.set_soft_wrap_mode(
                                        language_settings::SoftWrap::EditorWidth,
                                        cx,
                                    );
                                    editor.set_show_line_numbers(false, cx);
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
                            },
                        ));

                        return;
                    }
                }

                zeta_tab.active_completion = None;
            }
            FeedbackTabs::InlineAssist(_) => todo!(),
            FeedbackTabs::Chat(_) => todo!(),
        }
    }

    fn render_active(
        &mut self,
        modal_handle: FocusHandle,
        cx: &mut ViewContext<RateResponseModal>,
    ) -> Option<AnyElement> {
        match self {
            FeedbackTabs::Zeta(feedback) => feedback.render_active(modal_handle, cx),
            FeedbackTabs::InlineAssist(_) => todo!("Implement render_index for InlineAssist"),
            FeedbackTabs::Chat(_) => todo!("Implement render_index for Chat"),
        }
    }

    fn empty_message(&self) -> &'static str {
        match self {
            FeedbackTabs::Zeta(_) => {
                "No completions yet. Use the editor to generate some and rate them!"
            }
            FeedbackTabs::InlineAssist(_) => todo!(),
            FeedbackTabs::Chat(_) => todo!(),
        }
    }

    // Small list view
    fn render_index(
        &mut self,
        index: usize,
        cx: &mut ViewContext<RateResponseModal>,
    ) -> Option<AnyElement> {
        match self {
            FeedbackTabs::Zeta(feedback) => feedback.render_index(index, cx),
            FeedbackTabs::InlineAssist(_) => todo!("Implement render_index for InlineAssist"),
            FeedbackTabs::Chat(_) => todo!("Implement render_index for Chat"),
        }
    }
}

pub struct RateResponseModal {
    focus_handle: FocusHandle,
    tab: FeedbackTabs,
}
struct ActiveResponse {
    completion: InlineResponse,
    feedback_editor: View<Editor>,
}

impl RateResponseModal {
    pub fn toggle(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        if let Some(zeta) = Zeta::global(cx) {
            workspace.toggle_modal(cx, |cx| RateResponseModal::new(zeta, cx));
        }
    }

    pub fn new(zeta: Model<Zeta>, cx: &mut ViewContext<Self>) -> Self {
        let subscription = cx.observe(&zeta, |_, _, cx| cx.notify());
        Self {
            focus_handle: cx.focus_handle(),
            tab: FeedbackTabs::Zeta(ZetaFeedback {
                zeta,
                active_completion: None,
                selected_index: 0,
                _subscription: subscription,
            }),
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }

    fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let length = self.tab.responses_length(cx);
        let selected_index = self.tab.selected_index_mut();
        *selected_index += 1;
        *selected_index = usize::min(*selected_index, length);
        cx.notify();
    }

    fn select_prev(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let selected_index = self.tab.selected_index_mut();

        *selected_index = selected_index.saturating_sub(1);
        cx.notify();
    }

    fn select_next_edit(&mut self, _: &NextEdit, cx: &mut ViewContext<Self>) {
        let selected_index = *self.tab.selected_index_mut();
        let responses_length = self.tab.responses_length(cx);
        for i in selected_index..responses_length {
            if self.tab.is_useful_response(selected_index + i, cx) {
                *self.tab.selected_index_mut() = selected_index + i;
                cx.notify();
                return;
            }
        }
        self.select_next(&menu::SelectNext, cx);
    }

    fn select_prev_edit(&mut self, _: &PreviousEdit, cx: &mut ViewContext<Self>) {
        let selected_index = *self.tab.selected_index_mut();
        let responses_length = self.tab.responses_length(cx);
        for i in 0..(responses_length - selected_index) {
            if self.tab.is_useful_response(selected_index - i, cx) {
                *self.tab.selected_index_mut() = selected_index - i;
                cx.notify();
                return;
            }
        }
        self.select_prev(&menu::SelectPrev, cx);
    }

    fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        *self.tab.selected_index_mut() = 0;
        cx.notify();
    }

    fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let length = self.tab.responses_length(cx);
        *self.tab.selected_index_mut() = length - 1;
        cx.notify();
    }

    fn thumbs_up(&mut self, _: &ThumbsUp, cx: &mut ViewContext<Self>) {
        self.tab.thumbs_up_selected(cx);
        self.select_next_edit(&Default::default(), cx);
        cx.notify();
    }

    fn thumbs_up_active(&mut self, _: &ThumbsUpActiveResponse, cx: &mut ViewContext<Self>) {
        self.tab.thumbs_up_active(cx);
        let active_ix = self.tab.active_response();
        self.tab.select_completion(active_ix, false, cx);
        self.select_next_edit(&Default::default(), cx);
        self.confirm(&Default::default(), cx);

        cx.notify();
    }

    fn thumbs_down_active(&mut self, _: &ThumbsDownActiveResponse, cx: &mut ViewContext<Self>) {
        self.tab.thumbs_down_active(cx);
        let active_ix = self.tab.active_response();
        self.tab.select_completion(active_ix, false, cx);
        self.select_next_edit(&Default::default(), cx);
        self.confirm(&Default::default(), cx);

        cx.notify();
    }

    fn focus_completions(&mut self, _: &FocusResponse, cx: &mut ViewContext<Self>) {
        cx.focus_self();
        cx.notify();
    }

    fn preview_completion(&mut self, _: &PreviewResponse, cx: &mut ViewContext<Self>) {
        let selected_completion = *self.tab.selected_index_mut();

        self.tab
            .select_completion(Some(selected_completion), false, cx);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let selected_completion = *self.tab.selected_index_mut();

        self.tab
            .select_completion(Some(selected_completion), true, cx);
    }

    fn render_active_completion(&mut self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        self.tab.render_active(self.focus_handle.clone(), cx)
    }
}

impl Render for RateResponseModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let border_color = cx.theme().colors().border;

        h_flex()
            .key_context("RateResponseModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_prev_edit))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_next_edit))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::thumbs_up))
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
                    .border_r_1()
                    .border_color(border_color)
                    .w_96()
                    .h_full()
                    .flex_shrink_0()
                    .overflow_hidden()
                    .child(
                        TabBar::new("llm_feedback_tabs")
                            .child(
                                Tab::new("completions")
                                    .child(
                                        div()
                                            .child(
                                                Icon::new(IconName::ZedPredict)
                                                    .size(IconSize::Small),
                                            )
                                            .child(Label::new("Completions")),
                                    )
                                    .on_click(cx.listener(|this, _: &ClickEvent, cx| {})),
                            )
                            .child(
                                Tab::new("feedback")
                                    .child(Label::new("Feedback"))
                                    .on_click(cx.listener(|this, _: &ClickEvent, cx| {})),
                            ),
                    ),
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
                                    .child(Label::new(self.tab.empty_message()).color(Color::Muted))
                                    .into_any_element(),
                            )
                            .children({
                                let responses_length = self.tab.responses_length(cx);
                                (0..responses_length)
                                    .into_iter()
                                    .filter_map(|ix| self.tab.render_index(ix, cx))
                            }),
                    ),
            )
            .children(self.render_active_completion(cx))
            .on_mouse_down_out(cx.listener(|_, _, cx| cx.emit(DismissEvent)))
    }
}

impl EventEmitter<DismissEvent> for RateResponseModal {}

impl FocusableView for RateResponseModal {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for RateResponseModal {}

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
