use buffer_diff::BufferDiff;
use edit_prediction::{EditPrediction, EditPredictionRating, EditPredictionStore};
use editor::{Editor, Inlay, MultiBuffer};
use feature_flags::{FeatureFlag, PresenceFlag, register_feature_flag};
use gpui::{
    App, BorderStyle, DismissEvent, EdgesRefinement, Entity, EventEmitter, FocusHandle, Focusable,
    Length, StyleRefinement, TextStyleRefinement, Window, actions, prelude::*,
};
use language::{
    Anchor, Bias, Buffer, BufferSnapshot, CodeLabel, LanguageRegistry, Point, ToOffset, ToPoint,
    language_settings::{self, InlayHintKind},
};
use markdown::{Markdown, MarkdownStyle};
use project::{
    Completion, CompletionDisplayOptions, CompletionResponse, CompletionSource, InlayHint,
    InlayHintLabel, InlayId, ResolveState,
};
use settings::Settings as _;
use std::rc::Rc;
use std::{fmt::Write, ops::Range, sync::Arc};
use theme_settings::ThemeSettings;
use ui::{
    ContextMenu, DropdownMenu, KeyBinding, List, ListItem, ListItemSpacing, PopoverMenuHandle,
    Tooltip, prelude::*,
};
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
    type Value = PresenceFlag;
}
register_feature_flag!(PredictEditsRatePredictionsFeatureFlag);

pub struct RatePredictionsModal {
    ep_store: Entity<EditPredictionStore>,
    language_registry: Arc<LanguageRegistry>,
    active_prediction: Option<ActivePrediction>,
    selected_index: usize,
    diff_editor: Entity<Editor>,
    focus_handle: FocusHandle,
    _subscription: gpui::Subscription,
    current_view: RatePredictionView,
    failure_mode_menu_handle: PopoverMenuHandle<ContextMenu>,
}

struct ActivePrediction {
    prediction: EditPrediction,
    feedback_editor: Entity<Editor>,
    expected_buffer: Entity<Buffer>,
    expected_editable_range: Option<Range<Anchor>>,
    expected_editor: Entity<Editor>,
    expected_diff_editor: Entity<Editor>,
    expected_patch_preview: bool,
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
            failure_mode_menu_handle: PopoverMenuHandle::default(),
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
                    self.expected_patch_for_active(cx),
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
                    self.expected_patch_for_active(cx),
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

    fn update_diff_editor(
        diff_editor: &Entity<Editor>,
        new_buffer: Entity<Buffer>,
        old_buffer_snapshot: BufferSnapshot,
        visible_range: Range<Point>,
        cx: &mut Context<Self>,
    ) {
        diff_editor.update(cx, |editor, cx| {
            let new_buffer_snapshot = new_buffer.read(cx).snapshot();
            let new_buffer_id = new_buffer_snapshot.remote_id();
            let language = new_buffer_snapshot.language().cloned();
            let diff = cx.new(|cx| BufferDiff::new(&new_buffer_snapshot.text, cx));
            diff.update(cx, |diff, cx| {
                let update = diff.update_diff(
                    new_buffer_snapshot.text.clone(),
                    Some(old_buffer_snapshot.text().into()),
                    Some(true),
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
                multibuffer.set_excerpts_for_buffer(new_buffer, [visible_range], 0, cx);
                multibuffer.add_diff(diff, cx);
            });
        });
    }

    fn editable_range_for_prediction(prediction: &EditPrediction) -> Option<Range<Anchor>> {
        prediction
            .editable_range
            .clone()
            .or_else(|| Some(prediction.edits.first()?.0.start..prediction.edits.last()?.0.end))
    }

    fn insert_editable_region_markers(
        editor: &Entity<Editor>,
        buffer: &Entity<Buffer>,
        marker_range: Range<usize>,
        cx: &mut Context<Self>,
    ) {
        editor.update(cx, |editor, cx| {
            let buffer_snapshot = buffer.read(cx).snapshot();
            let multibuffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            let start_buffer_anchor = buffer_snapshot
                .anchor_after(buffer_snapshot.clip_offset(marker_range.start, Bias::Left));
            let end_buffer_anchor = buffer_snapshot
                .anchor_after(buffer_snapshot.clip_offset(marker_range.end, Bias::Right));
            let Some(start_anchor) = multibuffer_snapshot.anchor_in_excerpt(start_buffer_anchor)
            else {
                return;
            };
            let Some(end_anchor) = multibuffer_snapshot.anchor_in_excerpt(end_buffer_anchor) else {
                return;
            };
            let Some((start_hint_position, _)) =
                multibuffer_snapshot.anchor_to_buffer_anchor(start_anchor)
            else {
                return;
            };
            let Some((end_hint_position, _)) =
                multibuffer_snapshot.anchor_to_buffer_anchor(end_anchor)
            else {
                return;
            };

            editor.splice_inlays(
                &[InlayId::Hint(0), InlayId::Hint(1)],
                vec![
                    Inlay::hint(
                        InlayId::Hint(0),
                        start_anchor,
                        &InlayHint {
                            position: start_hint_position,
                            label: InlayHintLabel::String("╭─ editable region start\n".into()),
                            kind: Some(InlayHintKind::Parameter),
                            padding_left: false,
                            padding_right: false,
                            tooltip: None,
                            resolve_state: ResolveState::Resolved,
                        },
                    ),
                    Inlay::hint(
                        InlayId::Hint(1),
                        end_anchor,
                        &InlayHint {
                            position: end_hint_position,
                            label: InlayHintLabel::String("\n╰─ editable region end".into()),
                            kind: Some(InlayHintKind::Parameter),
                            padding_left: false,
                            padding_right: false,
                            tooltip: None,
                            resolve_state: ResolveState::Resolved,
                        },
                    ),
                ],
                cx,
            );
        });
    }

    fn expected_patch_for_active(&self, cx: &App) -> Option<String> {
        let active_prediction = self.active_prediction.as_ref()?;
        let expected_text = active_prediction.expected_buffer.read(cx).snapshot().text();
        let original_text = active_prediction.prediction.snapshot.text();
        let diff_body = language::unified_diff(&original_text, &expected_text);

        if diff_body.is_empty() {
            return None;
        }

        let path = active_prediction
            .prediction
            .snapshot
            .file()
            .map(|file| file.path().as_unix_str());
        let header = match path {
            Some(path) => format!("--- a/{path}\n+++ b/{path}\n"),
            None => String::new(),
        };

        Some(format!("{header}{diff_body}"))
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

            let editable_range = Self::editable_range_for_prediction(&prediction);
            let predicted_buffer = prediction.edit_preview.build_result_buffer(cx);
            let predicted_buffer_snapshot = predicted_buffer.read(cx).snapshot();
            let visible_range = prediction
                .edit_preview
                .compute_visible_range(&prediction.edits)
                .unwrap_or(Point::zero()..Point::zero());
            let start = Point::new(visible_range.start.row.saturating_sub(5), 0);
            let end =
                Point::new(visible_range.end.row + 5, 0).min(predicted_buffer_snapshot.max_point());
            Self::update_diff_editor(
                &self.diff_editor,
                predicted_buffer.clone(),
                prediction.snapshot.clone(),
                start..end,
                cx,
            );

            if let Some(editable_range) = editable_range.as_ref() {
                Self::insert_editable_region_markers(
                    &self.diff_editor,
                    &predicted_buffer,
                    prediction
                        .edit_preview
                        .anchor_to_offset_in_result(editable_range.start)
                        ..prediction
                            .edit_preview
                            .anchor_to_offset_in_result(editable_range.end),
                    cx,
                );
            }

            self.diff_editor.update(cx, |editor, cx| {
                if let Some(cursor_position) = prediction.cursor_position.as_ref() {
                    let multibuffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                    let cursor_offset = prediction
                        .edit_preview
                        .anchor_to_offset_in_result(cursor_position.anchor)
                        + cursor_position.offset;
                    let predicted_buffer_snapshot = predicted_buffer.read(cx).snapshot();
                    let cursor_anchor = predicted_buffer_snapshot.anchor_after(
                        predicted_buffer_snapshot.clip_offset(cursor_offset, Bias::Right),
                    );

                    if let Some(anchor) = multibuffer_snapshot.anchor_in_excerpt(cursor_anchor) {
                        editor.splice_inlays(
                            &[InlayId::EditPrediction(0)],
                            vec![Inlay::edit_prediction(0, anchor, "▏")],
                            cx,
                        );
                    }
                }
            });

            let mut formatted_inputs = String::new();

            write!(&mut formatted_inputs, "## Events\n\n").unwrap();

            for event in &prediction.inputs.events {
                formatted_inputs.push_str("```diff\n");
                zeta_prompt::write_event(&mut formatted_inputs, event.as_ref());
                formatted_inputs.push_str("```\n\n");
            }

            write!(&mut formatted_inputs, "## Related files\n\n").unwrap();

            for included_file in prediction
                .inputs
                .related_files
                .as_deref()
                .unwrap_or_default()
                .iter()
            {
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

            let mut cursor_offset = prediction
                .inputs
                .cursor_offset_in_excerpt
                .min(prediction.inputs.cursor_excerpt.len());
            while !prediction
                .inputs
                .cursor_excerpt
                .is_char_boundary(cursor_offset)
            {
                cursor_offset = cursor_offset.saturating_sub(1);
            }
            writeln!(
                &mut formatted_inputs,
                "```{}\n{}<CURSOR>{}\n```\n",
                prediction.inputs.cursor_path.display(),
                &prediction.inputs.cursor_excerpt[..cursor_offset],
                &prediction.inputs.cursor_excerpt[cursor_offset..],
            )
            .unwrap();

            let current_editable_region = editable_range.as_ref().map(|range| {
                prediction
                    .buffer
                    .read(cx)
                    .snapshot()
                    .text_for_range(range.clone())
                    .collect::<String>()
            });
            let expected_buffer = cx.new(|cx| {
                let mut buffer = Buffer::local(prediction.snapshot.text(), cx);
                buffer.set_language_async(prediction.snapshot.language().cloned(), cx);
                buffer
            });
            let expected_editable_range = editable_range.as_ref().map(|editable_range| {
                expected_buffer.update(cx, |buffer, cx| {
                    let snapshot = buffer.snapshot();
                    let editable_point_range = editable_range.start.to_point(&prediction.snapshot)
                        ..editable_range.end.to_point(&prediction.snapshot);
                    let expected_editable_range = snapshot.anchor_before(editable_point_range.start)
                        ..snapshot.anchor_after(editable_point_range.end);
                    if let Some(current_editable_region) = current_editable_region {
                        buffer.edit(
                            [(expected_editable_range.clone(), current_editable_region)],
                            None,
                            cx,
                        );
                    }
                    expected_editable_range
                })
            });
            let expected_buffer_snapshot = expected_buffer.read(cx).snapshot();
            let expected_excerpt_range = expected_editable_range
                .as_ref()
                .map(|range| {
                    range.start.to_point(&expected_buffer_snapshot)
                        ..range.end.to_point(&expected_buffer_snapshot)
                })
                .unwrap_or_else(|| visible_range.clone());
            let expected_editor = cx.new(|cx| {
                let multibuffer = cx.new(|cx| {
                    let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);
                    multibuffer.set_excerpts_for_buffer(
                        expected_buffer.clone(),
                        [expected_excerpt_range],
                        0,
                        cx,
                    );
                    multibuffer
                });
                let mut editor = Editor::for_multibuffer(multibuffer, None, window, cx);
                let expected_buffer_id = expected_buffer.read(cx).remote_id();
                editor.disable_header_for_buffer(expected_buffer_id, cx);
                editor.disable_inline_diagnostics();
                editor.set_show_git_diff_gutter(false, cx);
                editor.set_show_code_actions(false, cx);
                editor.set_show_runnables(false, cx);
                editor.set_show_bookmarks(false, cx);
                editor.set_show_breakpoints(false, cx);
                editor.set_show_wrap_guides(false, cx);
                editor.set_show_edit_predictions(Some(false), window, cx);
                editor
            });
            let expected_diff_editor = cx.new(|cx| {
                let multibuffer = cx.new(|_| MultiBuffer::new(language::Capability::ReadOnly));
                let mut editor = Editor::for_multibuffer(multibuffer, None, window, cx);
                editor.disable_inline_diagnostics();
                editor.set_expand_all_diff_hunks(cx);
                editor.set_show_git_diff_gutter(false, cx);
                editor
            });
            if let Some(expected_editable_range) = expected_editable_range.as_ref() {
                let expected_buffer_snapshot = expected_buffer.read(cx).snapshot();
                Self::insert_editable_region_markers(
                    &expected_editor,
                    &expected_buffer,
                    expected_editable_range
                        .start
                        .to_offset(&expected_buffer_snapshot)
                        ..expected_editable_range
                            .end
                            .to_offset(&expected_buffer_snapshot),
                    cx,
                );
            }

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
                    editor.set_show_bookmarks(false, cx);
                    editor.set_show_breakpoints(false, cx);
                    editor.set_show_wrap_guides(false, cx);
                    editor.set_show_indent_guides(false, cx);
                    editor.set_show_edit_predictions(Some(false), window, cx);
                    editor.set_placeholder_text("Add your feedback…", window, cx);
                    editor.set_completion_provider(Some(Rc::new(FeedbackCompletionProvider)));
                    if focus {
                        cx.focus_self(window);
                    }
                    editor
                }),
                expected_buffer,
                expected_editable_range,
                expected_editor,
                expected_diff_editor,
                expected_patch_preview: false,
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

    fn toggle_expected_patch_preview(&mut self, cx: &mut Context<Self>) {
        if let Some(active_prediction) = &mut self.active_prediction {
            if active_prediction.expected_patch_preview {
                active_prediction.expected_patch_preview = false;
            } else {
                let expected_buffer_snapshot =
                    active_prediction.expected_buffer.read(cx).snapshot();
                let visible_range = active_prediction
                    .prediction
                    .edit_preview
                    .compute_visible_range(&active_prediction.prediction.edits)
                    .unwrap_or(Point::zero()..Point::zero());
                let start = Point::new(visible_range.start.row.saturating_sub(5), 0);
                let end = Point::new(visible_range.end.row + 5, 0)
                    .min(expected_buffer_snapshot.max_point());

                Self::update_diff_editor(
                    &active_prediction.expected_diff_editor,
                    active_prediction.expected_buffer.clone(),
                    active_prediction.prediction.snapshot.clone(),
                    start..end,
                    cx,
                );
                if let Some(expected_editable_range) =
                    active_prediction.expected_editable_range.as_ref()
                {
                    let expected_buffer_snapshot =
                        active_prediction.expected_buffer.read(cx).snapshot();
                    Self::insert_editable_region_markers(
                        &active_prediction.expected_diff_editor,
                        &active_prediction.expected_buffer,
                        expected_editable_range
                            .start
                            .to_offset(&expected_buffer_snapshot)
                            ..expected_editable_range
                                .end
                                .to_offset(&expected_buffer_snapshot),
                        cx,
                    );
                }
                active_prediction.expected_patch_preview = true;
            }
            cx.notify();
        }
    }

    fn render_suggested_edits(&self, cx: &mut Context<Self>) -> Option<gpui::Stateful<Div>> {
        let bg_color = cx.theme().colors().editor_background;
        let border_color = cx.theme().colors().border;
        let active_prediction = self.active_prediction.as_ref()?;
        let expected_patch_preview = active_prediction.expected_patch_preview;

        Some(
            v_flex()
                .id("diff")
                .size_full()
                .bg(bg_color)
                .overflow_hidden()
                .child(
                    v_flex()
                        .flex_1()
                        .min_h_0()
                        .child(
                            h_flex()
                                .h_8()
                                .px_2()
                                .border_b_1()
                                .border_color(border_color)
                                .child(Label::new("Predicted Patch").size(LabelSize::Small)),
                        )
                        .child(
                            div()
                                .id("predicted-patch-diff")
                                .p_4()
                                .flex_1()
                                .min_h_0()
                                .overflow_scroll()
                                .whitespace_nowrap()
                                .child(self.diff_editor.clone()),
                        ),
                )
                .child(
                    v_flex()
                        .flex_1()
                        .min_h_0()
                        .border_t_1()
                        .border_color(border_color)
                        .child(
                            h_flex()
                                .h_8()
                                .px_2()
                                .gap_2()
                                .border_b_1()
                                .border_color(border_color)
                                .child(
                                    Button::new(
                                        "expected-patch-preview",
                                        if expected_patch_preview {
                                            "Edit"
                                        } else {
                                            "Preview"
                                        },
                                    )
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(
                                        |this, _, _window, cx| {
                                            this.toggle_expected_patch_preview(cx);
                                        },
                                    )),
                                )
                                .child(Label::new("Expected Patch").size(LabelSize::Small)),
                        )
                        .child(
                            div()
                                .id("expected-patch")
                                .p_4()
                                .flex_1()
                                .min_h_0()
                                .overflow_scroll()
                                .whitespace_nowrap()
                                .child(if expected_patch_preview {
                                    active_prediction
                                        .expected_diff_editor
                                        .clone()
                                        .into_any_element()
                                } else {
                                    active_prediction.expected_editor.clone().into_any_element()
                                }),
                        ),
                ),
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
                    let modal = cx.entity().downgrade();
                    let failure_mode_menu =
                        ContextMenu::build(window, cx, move |menu, _window, _cx| {
                            FeedbackCompletionProvider::FAILURE_MODES
                                .iter()
                                .fold(menu, |menu, (key, description)| {
                                    let key: SharedString = (*key).into();
                                    let description: SharedString = (*description).into();
                                    let modal = modal.clone();
                                    menu.entry(
                                        format!("{} {}", key, description),
                                        None,
                                        move |window, cx| {
                                            if let Some(modal) = modal.upgrade() {
                                                modal.update(cx, |this, cx| {
                                                    if let Some(active) = &this.active_prediction {
                                                        active.feedback_editor.update(
                                                            cx,
                                                            |editor, cx| {
                                                                editor.set_text(
                                                                    format!("{} {}", key, description),
                                                                    window,
                                                                    cx,
                                                                );
                                                            },
                                                        );
                                                    }
                                                });
                                            }
                                        },
                                    )
                                })
                        });

                    this.child(
                        h_flex()
                            .p_2()
                            .gap_2()
                            .border_y_1()
                            .border_color(border_color)
                            .child(
                                DropdownMenu::new(
                                        "failure-mode-dropdown",
                                        "Issue",
                                        failure_mode_menu,
                                    )
                                    .handle(self.failure_mode_menu_handle.clone())
                                    .style(ui::DropdownStyle::Outlined)
                                    .trigger_size(ButtonSize::Compact),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Icon::new(IconName::Info)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        div().flex_wrap().child(
                                            Label::new(concat!(
                                                "Explain why this completion is good or bad. ",
                                                "If it's negative, describe what you expected instead."
                                            ))
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                        ),
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
                                        .start_icon(Icon::new(IconName::ThumbsDown).size(IconSize::Small))
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
                                        .start_icon(Icon::new(IconName::ThumbsUp).size(IconSize::Small))
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
                                v_flex().child(
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
                    .child({
                        let icons = self.ep_store.read(cx).icons(cx);
                        h_flex()
                            .h_8()
                            .px_2()
                            .justify_between()
                            .border_b_1()
                            .border_color(border_color)
                            .child(Icon::new(icons.base).size(IconSize::Small))
                            .child(
                                Label::new("From most recent to oldest")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            )
                    })
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
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                if !this.failure_mode_menu_handle.is_deployed() {
                    cx.emit(DismissEvent);
                }
            }))
    }
}

impl EventEmitter<DismissEvent> for RatePredictionsModal {}

impl Focusable for RatePredictionsModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for RatePredictionsModal {}

struct FeedbackCompletionProvider;

impl FeedbackCompletionProvider {
    const FAILURE_MODES: &'static [(&'static str, &'static str)] = &[
        ("@location", "Unexpected location"),
        ("@malformed", "Incomplete, cut off, or syntax error"),
        (
            "@deleted",
            "Deleted code that should be kept (use `@reverted` if it undid a recent edit)",
        ),
        ("@style", "Wrong coding style or conventions"),
        ("@repetitive", "Repeated existing code"),
        ("@hallucinated", "Referenced non-existent symbols"),
        ("@formatting", "Wrong indentation or structure"),
        ("@aggressive", "Changed more than expected"),
        ("@conservative", "Too cautious, changed too little"),
        ("@context", "Ignored or misunderstood context"),
        ("@reverted", "Undid recent edits"),
        ("@cursor_position", "Cursor placed in unhelpful position"),
        ("@whitespace", "Unwanted whitespace or newline changes"),
    ];
}

impl editor::CompletionProvider for FeedbackCompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        _trigger: editor::CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> gpui::Task<anyhow::Result<Vec<CompletionResponse>>> {
        let buffer = buffer.read(cx);
        let mut count_back = 0;

        for char in buffer.reversed_chars_at(buffer_position) {
            if char.is_ascii_alphanumeric() || char == '_' || char == '@' {
                count_back += 1;
            } else {
                break;
            }
        }

        let start_anchor = buffer.anchor_before(
            buffer_position
                .to_offset(&buffer)
                .saturating_sub(count_back),
        );

        let replace_range = start_anchor..buffer_position;
        let snapshot = buffer.text_snapshot();
        let query: String = snapshot.text_for_range(replace_range.clone()).collect();

        if !query.starts_with('@') {
            return gpui::Task::ready(Ok(vec![CompletionResponse {
                completions: vec![],
                display_options: CompletionDisplayOptions {
                    dynamic_width: true,
                },
                is_incomplete: false,
            }]));
        }

        let query_lower = query.to_lowercase();

        let completions: Vec<Completion> = Self::FAILURE_MODES
            .iter()
            .filter(|(key, _description)| key.starts_with(&query_lower))
            .map(|(key, description)| Completion {
                replace_range: replace_range.clone(),
                new_text: format!("{} {}", key, description),
                label: CodeLabel::plain(format!("{}: {}", key, description), None),
                documentation: None,
                source: CompletionSource::Custom,
                icon_path: None,
                match_start: None,
                snippet_deduplication_key: None,
                insert_text_mode: None,
                confirm: None,
                group: None,
            })
            .collect();

        gpui::Task::ready(Ok(vec![CompletionResponse {
            completions,
            display_options: CompletionDisplayOptions {
                dynamic_width: true,
            },
            is_incomplete: false,
        }]))
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Entity<Buffer>,
        _position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        _cx: &mut Context<Editor>,
    ) -> bool {
        text.chars()
            .last()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_' || c == '@')
    }
}
