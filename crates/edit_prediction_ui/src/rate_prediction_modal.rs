use buffer_diff::BufferDiff;
use edit_prediction::{EditPrediction, EditPredictionRating, EditPredictionStore};
use editor::{Editor, ExcerptRange, MultiBuffer};
use feature_flags::FeatureFlag;
use gpui::{
    App, BorderStyle, DismissEvent, EdgesRefinement, Entity, EventEmitter, FocusHandle, Focusable,
    Length, StyleRefinement, TextStyleRefinement, Window, actions, prelude::*,
};
use language::{LanguageRegistry, Point, language_settings};
use markdown::{Markdown, MarkdownStyle};
use settings::Settings as _;
use std::{fmt::Write, sync::Arc, time::Duration};
use theme::ThemeSettings;
use ui::{KeyBinding, List, ListItem, ListItemSpacing, Tooltip, prelude::*};
use workspace::{ModalView, Workspace};

actions!(
    zeta,
    [
        /// Rates the active completion with a thumbs up.
        ThumbsUpActivePrediction,
        /// Rates the active completion with a thumbs down.
        ThumbsDownActivePrediction,
        /// Navigates to the next edit in the completion history.
        NextEdit,
        /// Navigates to the previous edit in the completion history.
        PreviousEdit,
        /// Focuses on the completions list.
        FocusPredictions,
        /// Previews the selected completion.
        PreviewPrediction,
    ]
);

pub struct PredictEditsRatePredictionsFeatureFlag;

impl FeatureFlag for PredictEditsRatePredictionsFeatureFlag {
    const NAME: &'static str = "predict-edits-rate-completions";
}

pub struct RatePredictionsModal {
    ep_store: Entity<EditPredictionStore>,
    language_registry: Arc<LanguageRegistry>,
    active_prediction: Option<ActivePrediction>,
    selected_index: usize,
    diff_editor: Entity<Editor>,
    focus_handle: FocusHandle,
    _subscription: gpui::Subscription,
    current_view: RatePredictionView,
}

struct ActivePrediction {
    prediction: EditPrediction,
    feedback_editor: Entity<Editor>,
    formatted_inputs: Entity<Markdown>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum RatePredictionView {
    SuggestedEdits,
    RawInput,
}

impl RatePredictionView {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SuggestedEdits => "Suggested Edits",
            Self::RawInput => "Recorded Events & Input",
        }
    }
}

impl RatePredictionsModal {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        if let Some(ep_store) = EditPredictionStore::try_global(cx) {
            let language_registry = workspace.app_state().languages.clone();
            workspace.toggle_modal(window, cx, |window, cx| {
                RatePredictionsModal::new(ep_store, language_registry, window, cx)
            });

            telemetry::event!("Rate Prediction Modal Open", source = "Edit Prediction");
        }
    }

    pub fn new(
        ep_store: Entity<EditPredictionStore>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe(&ep_store, |_, _, cx| cx.notify());

        Self {
            ep_store,
            language_registry,
            selected_index: 0,
            focus_handle: cx.focus_handle(),
            active_prediction: None,
            _subscription: subscription,
            diff_editor: cx.new(|cx| {
                let multibuffer = cx.new(|_| MultiBuffer::new(language::Capability::ReadOnly));
                let mut editor = Editor::for_multibuffer(multibuffer, None, window, cx);
                editor.disable_inline_diagnostics();
                editor.set_expand_all_diff_hunks(cx);
                editor.set_show_git_diff_gutter(false, cx);
                editor
            }),
            current_view: RatePredictionView::SuggestedEdits,
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_index += 1;
        self.selected_index = usize::min(
            self.selected_index,
            self.ep_store.read(cx).shown_predictions().count(),
        );
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected_index = self.selected_index.saturating_sub(1);
        cx.notify();
    }

    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {
        let next_index = self
            .ep_store
            .read(cx)
            .shown_predictions()
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

    fn select_prev_edit(&mut self, _: &PreviousEdit, _: &mut Window, cx: &mut Context<Self>) {
        let ep_store = self.ep_store.read(cx);
        let completions_len = ep_store.shown_completions_len();

        let prev_index = self
            .ep_store
            .read(cx)
            .shown_predictions()
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

    fn select_first(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_index = 0;
        cx.notify();
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        self.selected_index = self.ep_store.read(cx).shown_completions_len() - 1;
        cx.notify();
    }

    pub fn thumbs_up_active(
        &mut self,
        _: &ThumbsUpActivePrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.ep_store.update(cx, |ep_store, cx| {
            if let Some(active) = &self.active_prediction {
                ep_store.rate_prediction(
                    &active.prediction,
                    EditPredictionRating::Positive,
                    active.feedback_editor.read(cx).text(cx),
                    cx,
                );
            }
        });

        let current_completion = self
            .active_prediction
            .as_ref()
            .map(|completion| completion.prediction.clone());
        self.select_completion(current_completion, false, window, cx);
        self.select_next_edit(&Default::default(), window, cx);
        self.confirm(&Default::default(), window, cx);

        cx.notify();
    }

    pub fn thumbs_down_active(
        &mut self,
        _: &ThumbsDownActivePrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active) = &self.active_prediction {
            if active.feedback_editor.read(cx).text(cx).is_empty() {
                return;
            }

            self.ep_store.update(cx, |ep_store, cx| {
                ep_store.rate_prediction(
                    &active.prediction,
                    EditPredictionRating::Negative,
                    active.feedback_editor.read(cx).text(cx),
                    cx,
                );
            });
        }

        let current_completion = self
            .active_prediction
            .as_ref()
            .map(|completion| completion.prediction.clone());
        self.select_completion(current_completion, false, window, cx);
        self.select_next_edit(&Default::default(), window, cx);
        self.confirm(&Default::default(), window, cx);

        cx.notify();
    }

    fn focus_completions(
        &mut self,
        _: &FocusPredictions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.focus_self(window);
        cx.notify();
    }

    fn preview_completion(
        &mut self,
        _: &PreviewPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let completion = self
            .ep_store
            .read(cx)
            .shown_predictions()
            .skip(self.selected_index)
            .take(1)
            .next()
            .cloned();

        self.select_completion(completion, false, window, cx);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let completion = self
            .ep_store
            .read(cx)
            .shown_predictions()
            .skip(self.selected_index)
            .take(1)
            .next()
            .cloned();

        self.select_completion(completion, true, window, cx);
    }

    pub fn select_completion(
        &mut self,
        prediction: Option<EditPrediction>,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Avoid resetting completion rating if it's already selected.
        if let Some(prediction) = prediction {
            self.selected_index = self
                .ep_store
                .read(cx)
                .shown_predictions()
                .enumerate()
                .find(|(_, completion_b)| prediction.id == completion_b.id)
                .map(|(ix, _)| ix)
                .unwrap_or(self.selected_index);
            cx.notify();

            if let Some(prev_prediction) = self.active_prediction.as_ref()
                && prediction.id == prev_prediction.prediction.id
            {
                if focus {
                    window.focus(&prev_prediction.feedback_editor.focus_handle(cx), cx);
                }
                return;
            }

            self.diff_editor.update(cx, |editor, cx| {
                let new_buffer = prediction.edit_preview.build_result_buffer(cx);
                let new_buffer_snapshot = new_buffer.read(cx).snapshot();
                let old_buffer_snapshot = prediction.snapshot.clone();
                let new_buffer_id = new_buffer_snapshot.remote_id();

                let range = prediction
                    .edit_preview
                    .compute_visible_range(&prediction.edits)
                    .unwrap_or(Point::zero()..Point::zero());
                let start = Point::new(range.start.row.saturating_sub(5), 0);
                let end = Point::new(range.end.row + 5, 0).min(new_buffer_snapshot.max_point());

                let language = new_buffer_snapshot.language().cloned();
                let diff = cx.new(|cx| BufferDiff::new(&new_buffer_snapshot.text, cx));
                diff.update(cx, |diff, cx| {
                    let update = diff.update_diff(
                        new_buffer_snapshot.text.clone(),
                        Some(old_buffer_snapshot.text().into()),
                        true,
                        language,
                        cx,
                    );
                    cx.spawn(async move |diff, cx| {
                        let update = update.await;
                        if let Some(task) = diff
                            .update(cx, |diff, cx| {
                                diff.set_snapshot(update, &new_buffer_snapshot.text, cx)
                            })
                            .ok()
                        {
                            task.await;
                        }
                    })
                    .detach();
                });

                editor.disable_header_for_buffer(new_buffer_id, cx);
                editor.buffer().update(cx, |multibuffer, cx| {
                    multibuffer.clear(cx);
                    multibuffer.push_excerpts(
                        new_buffer,
                        vec![ExcerptRange {
                            context: start..end,
                            primary: start..end,
                        }],
                        cx,
                    );
                    multibuffer.add_diff(diff, cx);
                });
            });

            let mut formatted_inputs = String::new();

            write!(&mut formatted_inputs, "## Events\n\n").unwrap();

            for event in &prediction.inputs.events {
                formatted_inputs.push_str("```diff\n");
                zeta_prompt::write_event(&mut formatted_inputs, event.as_ref());
                formatted_inputs.push_str("```\n\n");
            }

            write!(&mut formatted_inputs, "## Related files\n\n").unwrap();

            for included_file in prediction.inputs.related_files.iter() {
                write!(
                    &mut formatted_inputs,
                    "### {}\n\n",
                    included_file.path.display()
                )
                .unwrap();

                for excerpt in included_file.excerpts.iter() {
                    write!(
                        &mut formatted_inputs,
                        "```{}\n{}\n```\n",
                        included_file.path.display(),
                        excerpt.text
                    )
                    .unwrap();
                }
            }

            write!(&mut formatted_inputs, "## Cursor Excerpt\n\n").unwrap();

            writeln!(
                &mut formatted_inputs,
                "```{}\n{}<CURSOR>{}\n```\n",
                prediction.inputs.cursor_path.display(),
                &prediction.inputs.cursor_excerpt[..prediction.inputs.cursor_offset_in_excerpt],
                &prediction.inputs.cursor_excerpt[prediction.inputs.cursor_offset_in_excerpt..],
            )
            .unwrap();

            self.active_prediction = Some(ActivePrediction {
                prediction,
                feedback_editor: cx.new(|cx| {
                    let mut editor = Editor::multi_line(window, cx);
                    editor.disable_scrollbars_and_minimap(window, cx);
                    editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
                    editor.set_show_line_numbers(false, cx);
                    editor.set_show_git_diff_gutter(false, cx);
                    editor.set_show_code_actions(false, cx);
                    editor.set_show_runnables(false, cx);
                    editor.set_show_breakpoints(false, cx);
                    editor.set_show_wrap_guides(false, cx);
                    editor.set_show_indent_guides(false, cx);
                    editor.set_show_edit_predictions(Some(false), window, cx);
                    editor.set_placeholder_text("Add your feedback…", window, cx);
                    if focus {
                        cx.focus_self(window);
                    }
                    editor
                }),
                formatted_inputs: cx.new(|cx| {
                    Markdown::new(
                        formatted_inputs.into(),
                        Some(self.language_registry.clone()),
                        None,
                        cx,
                    )
                }),
            });
        } else {
            self.active_prediction = None;
        }

        cx.notify();
    }

    fn render_view_nav(&self, cx: &Context<Self>) -> impl IntoElement {
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
                    RatePredictionView::SuggestedEdits.name(),
                )
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.current_view = RatePredictionView::SuggestedEdits;
                    cx.notify();
                }))
                .toggle_state(self.current_view == RatePredictionView::SuggestedEdits),
            )
            .child(
                Button::new(
                    ElementId::Name("raw-input".into()),
                    RatePredictionView::RawInput.name(),
                )
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.current_view = RatePredictionView::RawInput;
                    cx.notify();
                }))
                .toggle_state(self.current_view == RatePredictionView::RawInput),
            )
    }

    fn render_suggested_edits(&self, cx: &mut Context<Self>) -> Option<gpui::Stateful<Div>> {
        let bg_color = cx.theme().colors().editor_background;
        Some(
            div()
                .id("diff")
                .p_4()
                .size_full()
                .bg(bg_color)
                .overflow_scroll()
                .whitespace_nowrap()
                .child(self.diff_editor.clone()),
        )
    }

    fn render_raw_input(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Stateful<Div>> {
        let theme_settings = ThemeSettings::get_global(cx);
        let buffer_font_size = theme_settings.buffer_font_size(cx);

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
                        .child(if let Some(active_prediction) = &self.active_prediction {
                            markdown::MarkdownElement::new(
                                active_prediction.formatted_inputs.clone(),
                                MarkdownStyle {
                                    base_text_style: window.text_style(),
                                    syntax: cx.theme().syntax().clone(),
                                    code_block: StyleRefinement {
                                        text: TextStyleRefinement {
                                            font_family: Some(
                                                theme_settings.buffer_font.family.clone(),
                                            ),
                                            font_size: Some(buffer_font_size.into()),
                                            ..Default::default()
                                        },
                                        padding: EdgesRefinement {
                                            top: Some(DefiniteLength::Absolute(
                                                AbsoluteLength::Pixels(px(8.)),
                                            )),
                                            left: Some(DefiniteLength::Absolute(
                                                AbsoluteLength::Pixels(px(8.)),
                                            )),
                                            right: Some(DefiniteLength::Absolute(
                                                AbsoluteLength::Pixels(px(8.)),
                                            )),
                                            bottom: Some(DefiniteLength::Absolute(
                                                AbsoluteLength::Pixels(px(8.)),
                                            )),
                                        },
                                        margin: EdgesRefinement {
                                            top: Some(Length::Definite(px(8.).into())),
                                            left: Some(Length::Definite(px(0.).into())),
                                            right: Some(Length::Definite(px(0.).into())),
                                            bottom: Some(Length::Definite(px(12.).into())),
                                        },
                                        border_style: Some(BorderStyle::Solid),
                                        border_widths: EdgesRefinement {
                                            top: Some(AbsoluteLength::Pixels(px(1.))),
                                            left: Some(AbsoluteLength::Pixels(px(1.))),
                                            right: Some(AbsoluteLength::Pixels(px(1.))),
                                            bottom: Some(AbsoluteLength::Pixels(px(1.))),
                                        },
                                        border_color: Some(cx.theme().colors().border_variant),
                                        background: Some(
                                            cx.theme().colors().editor_background.into(),
                                        ),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                            )
                            .into_any_element()
                        } else {
                            div()
                                .child("No active completion".to_string())
                                .into_any_element()
                        }),
                )
                .id("raw-input-view"),
        )
    }

    fn render_active_completion(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let active_prediction = self.active_prediction.as_ref()?;
        let completion_id = active_prediction.prediction.id.clone();
        let focus_handle = &self.focus_handle(cx);

        let border_color = cx.theme().colors().border;
        let bg_color = cx.theme().colors().editor_background;

        let rated = self.ep_store.read(cx).is_prediction_rated(&completion_id);
        let feedback_empty = active_prediction
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
                        .when_some(
                            match self.current_view {
                                RatePredictionView::SuggestedEdits => {
                                    self.render_suggested_edits(cx)
                                }
                                RatePredictionView::RawInput => self.render_raw_input(window, cx),
                            },
                            |this, element| this.child(element),
                        ),
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
                                    .color(Color::Muted),
                            )
                            .child(
                                div().w_full().pr_2().flex_wrap().child(
                                    Label::new(concat!(
                                        "Explain why this completion is good or bad. ",
                                        "If it's negative, describe what you expected instead."
                                    ))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                                ),
                            ),
                    )
                })
                .when(!rated, |this| {
                    this.child(
                        div()
                            .h_40()
                            .pt_1()
                            .bg(bg_color)
                            .child(active_prediction.feedback_editor.clone()),
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
                        } else if active_prediction.prediction.edits.is_empty() {
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
                                    Button::new("bad", "Bad Prediction")
                                        .icon(IconName::ThumbsDown)
                                        .icon_size(IconSize::Small)
                                        .icon_position(IconPosition::Start)
                                        .disabled(rated || feedback_empty)
                                        .when(feedback_empty, |this| {
                                            this.tooltip(Tooltip::text(
                                                "Explain what's bad about it before reporting it",
                                            ))
                                        })
                                        .key_binding(KeyBinding::for_action_in(
                                            &ThumbsDownActivePrediction,
                                            focus_handle,
                                            cx,
                                        ))
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            if this.active_prediction.is_some() {
                                                this.thumbs_down_active(
                                                    &ThumbsDownActivePrediction,
                                                    window,
                                                    cx,
                                                );
                                            }
                                        })),
                                )
                                .child(
                                    Button::new("good", "Good Prediction")
                                        .icon(IconName::ThumbsUp)
                                        .icon_size(IconSize::Small)
                                        .icon_position(IconPosition::Start)
                                        .disabled(rated)
                                        .key_binding(KeyBinding::for_action_in(
                                            &ThumbsUpActivePrediction,
                                            focus_handle,
                                            cx,
                                        ))
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            if this.active_prediction.is_some() {
                                                this.thumbs_up_active(
                                                    &ThumbsUpActivePrediction,
                                                    window,
                                                    cx,
                                                );
                                            }
                                        })),
                                ),
                        ),
                ),
        )
    }

    fn render_shown_completions(&self, cx: &Context<Self>) -> impl Iterator<Item = ListItem> {
        self.ep_store
            .read(cx)
            .shown_predictions()
            .cloned()
            .enumerate()
            .map(|(index, completion)| {
                let selected = self
                    .active_prediction
                    .as_ref()
                    .is_some_and(|selected| selected.prediction.id == completion.id);
                let rated = self.ep_store.read(cx).is_prediction_rated(&completion.id);

                let (icon_name, icon_color, tooltip_text) =
                    match (rated, completion.edits.is_empty()) {
                        (true, _) => (IconName::Check, Color::Success, "Rated Prediction"),
                        (false, true) => (IconName::File, Color::Muted, "No Edits Produced"),
                        (false, false) => (IconName::FileDiff, Color::Accent, "Edits Available"),
                    };

                let file = completion.buffer.read(cx).file();
                let file_name = file
                    .as_ref()
                    .map_or(SharedString::new_static("untitled"), |file| {
                        file.file_name(cx).to_string().into()
                    });
                let file_path = file.map(|file| file.path().as_unix_str().to_string());

                ListItem::new(completion.id.clone())
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .focused(index == self.selected_index)
                    .toggle_state(selected)
                    .child(
                        h_flex()
                            .id("completion-content")
                            .gap_3()
                            .child(Icon::new(icon_name).color(icon_color).size(IconSize::Small))
                            .child(
                                v_flex()
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .child(Label::new(file_name).size(LabelSize::Small))
                                            .when_some(file_path, |this, p| {
                                                this.child(
                                                    Label::new(p)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                )
                                            }),
                                    )
                                    .child(
                                        Label::new(format!(
                                            "{} ago, {:.2?}",
                                            format_time_ago(
                                                completion.response_received_at.elapsed()
                                            ),
                                            completion.latency()
                                        ))
                                        .color(Color::Muted)
                                        .size(LabelSize::XSmall),
                                    ),
                            ),
                    )
                    .tooltip(Tooltip::text(tooltip_text))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.select_completion(Some(completion.clone()), true, window, cx);
                    }))
            })
    }
}

impl Render for RatePredictionsModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let border_color = cx.theme().colors().border;

        h_flex()
            .key_context("RatePredictionModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::select_previous))
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
            .w(window.viewport_size().width - px(320.))
            .h(window.viewport_size().height - px(300.))
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
                            .child(Icon::new(IconName::ZedPredict).size(IconSize::Small))
                            .child(
                                Label::new("From most recent to oldest")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
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
                                            .child(
                                                Label::new(concat!(
                                                    "No completions yet. ",
                                                    "Use the editor to generate some, ",
                                                    "and make sure to rate them!"
                                                ))
                                                .color(Color::Muted),
                                            )
                                            .into_any_element(),
                                    )
                                    .children(self.render_shown_completions(cx)),
                            ),
                    ),
            )
            .children(self.render_active_completion(window, cx))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| cx.emit(DismissEvent)))
    }
}

impl EventEmitter<DismissEvent> for RatePredictionsModal {}

impl Focusable for RatePredictionsModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for RatePredictionsModal {}

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
