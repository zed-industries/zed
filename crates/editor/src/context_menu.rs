use std::{
    cmp::Reverse,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    uniform_list, AnyElement, BackgroundExecutor, FontWeight, ListSizingBehavior, Model,
    MouseButton, StrikethroughStyle, StyledText, Task, UniformListScrollHandle, WeakView,
};
use language::{Buffer, Documentation};
use multi_buffer::{Anchor, ExcerptId};
use ordered_float::OrderedFloat;
use parking_lot::{Mutex, RwLock};
use project::{CodeAction, Completion, TaskSourceKind};
use settings::Settings;
use task::ResolvedTask;
use ui::{
    div, h_flex, px, ActiveTheme, Color, Div, FluentBuilder, InteractiveElement, IntoElement,
    Label, LabelCommon, LabelSize, ListItem, ParentElement, Pixels, Popover, Selectable,
    SharedString, StatefulInteractiveElement, Styled, StyledExt, ViewContext,
};
use util::ResultExt;
use workspace::Workspace;

use crate::{
    debounced_delay::DebouncedDelay, render_parsed_markdown, split_words,
    styled_runs_for_code_label, CodeActionProvider, CompletionId, CompletionProvider,
    ConfirmCodeAction, ConfirmCompletion, DisplayPoint, DisplayRow, Editor, EditorSettings,
    EditorStyle, ResolvedTasks,
};

pub(super) enum ContextMenu {
    Completions(CompletionsMenu),
    CodeActions(CodeActionsMenu),
}

pub(super) struct RenderedContextMenu {
    pub(super) origin: ContextMenuOrigin,
    pub(super) element: AnyElement,
    pub(super) is_inverted: Arc<AtomicBool>,
}

impl RenderedContextMenu {
    fn invert(&self) {
        self.is_inverted
            .fetch_or(true, std::sync::atomic::Ordering::Release);
    }
}

impl ContextMenu {
    pub(super) fn select_first(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_first(provider, cx),
                ContextMenu::CodeActions(menu) => menu.select_first(cx),
            }
            true
        } else {
            false
        }
    }

    pub(super) fn select_prev(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_prev(provider, cx),
                ContextMenu::CodeActions(menu) => menu.select_prev(cx),
            }
            true
        } else {
            false
        }
    }

    pub(super) fn select_next(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_next(provider, cx),
                ContextMenu::CodeActions(menu) => menu.select_next(cx),
            }
            true
        } else {
            false
        }
    }

    pub(super) fn select_last(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_last(provider, cx),
                ContextMenu::CodeActions(menu) => menu.select_last(cx),
            }
            true
        } else {
            false
        }
    }

    pub(super) fn visible(&self) -> bool {
        match self {
            ContextMenu::Completions(menu) => menu.visible(),
            ContextMenu::CodeActions(menu) => menu.visible(),
        }
    }

    pub(super) fn render(
        &self,
        cursor_position: DisplayPoint,
        style: &EditorStyle,
        max_height: Pixels,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Editor>,
    ) -> RenderedContextMenu {
        match self {
            ContextMenu::Completions(menu) => {
                let (element, is_inverted) = menu.render(style, max_height, workspace, cx);
                RenderedContextMenu {
                    origin: ContextMenuOrigin::EditorPoint(cursor_position),
                    element,
                    is_inverted,
                }
            }
            ContextMenu::CodeActions(menu) => menu.render(cursor_position, style, max_height, cx),
        }
    }
}

pub(crate) enum ContextMenuOrigin {
    EditorPoint(DisplayPoint),
    GutterIndicator(DisplayRow),
}

#[derive(Clone)]
pub(super) struct CompletionsMenu {
    pub(super) id: CompletionId,
    pub(super) sort_completions: bool,
    pub(super) initial_position: Anchor,
    pub(super) buffer: Model<Buffer>,
    pub(super) completions: Arc<RwLock<Box<[Completion]>>>,
    pub(super) match_candidates: Arc<[StringMatchCandidate]>,
    pub(super) matches: Arc<[StringMatch]>,
    pub(super) selected_item: usize,
    pub(super) scroll_handle: UniformListScrollHandle,
    pub(super) selected_completion_documentation_resolve_debounce: Arc<Mutex<DebouncedDelay>>,
}

impl CompletionsMenu {
    fn select_first(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) {
        self.selected_item = 0;
        self.scroll_handle.scroll_to_item(self.selected_item);
        self.attempt_resolve_selected_completion_documentation(provider, cx);
        cx.notify();
    }

    fn select_prev(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
        } else {
            self.selected_item = self.matches.len() - 1;
        }
        self.scroll_handle.scroll_to_item(self.selected_item);
        self.attempt_resolve_selected_completion_documentation(provider, cx);
        cx.notify();
    }

    fn select_next(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) {
        if self.selected_item + 1 < self.matches.len() {
            self.selected_item += 1;
        } else {
            self.selected_item = 0;
        }
        self.scroll_handle.scroll_to_item(self.selected_item);
        self.attempt_resolve_selected_completion_documentation(provider, cx);
        cx.notify();
    }

    fn select_last(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) {
        self.selected_item = self.matches.len() - 1;
        self.scroll_handle.scroll_to_item(self.selected_item);
        self.attempt_resolve_selected_completion_documentation(provider, cx);
        cx.notify();
    }

    pub(super) fn pre_resolve_completion_documentation(
        buffer: Model<Buffer>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        matches: Arc<[StringMatch]>,
        editor: &Editor,
        cx: &mut ViewContext<Editor>,
    ) -> Task<()> {
        let settings = EditorSettings::get_global(cx);
        if !settings.show_completion_documentation {
            return Task::ready(());
        }

        let Some(provider) = editor.completion_provider.as_ref() else {
            return Task::ready(());
        };

        let resolve_task = provider.resolve_completions(
            buffer,
            matches.iter().map(|m| m.candidate_id).collect(),
            completions.clone(),
            cx,
        );

        cx.spawn(move |this, mut cx| async move {
            if let Some(true) = resolve_task.await.log_err() {
                this.update(&mut cx, |_, cx| cx.notify()).ok();
            }
        })
    }

    fn attempt_resolve_selected_completion_documentation(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut ViewContext<Editor>,
    ) {
        let settings = EditorSettings::get_global(cx);
        if !settings.show_completion_documentation {
            return;
        }

        let completion_index = self.matches[self.selected_item].candidate_id;
        let Some(provider) = provider else {
            return;
        };

        let resolve_task = provider.resolve_completions(
            self.buffer.clone(),
            vec![completion_index],
            self.completions.clone(),
            cx,
        );

        let delay_ms =
            EditorSettings::get_global(cx).completion_documentation_secondary_query_debounce;
        let delay = Duration::from_millis(delay_ms);

        self.selected_completion_documentation_resolve_debounce
            .lock()
            .fire_new(delay, cx, |_, cx| {
                cx.spawn(move |this, mut cx| async move {
                    if let Some(true) = resolve_task.await.log_err() {
                        this.update(&mut cx, |_, cx| cx.notify()).ok();
                    }
                })
            });
    }

    fn visible(&self) -> bool {
        !self.matches.is_empty()
    }

    fn render(
        &self,
        style: &EditorStyle,
        max_height: Pixels,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Editor>,
    ) -> (AnyElement, Arc<AtomicBool>) {
        let settings = EditorSettings::get_global(cx);
        let show_completion_documentation = settings.show_completion_documentation;
        let is_inverted = Arc::new(Default::default());
        let widest_completion_ix = self
            .matches
            .iter()
            .enumerate()
            .max_by_key(|(_, mat)| {
                let completions = self.completions.read();
                let completion = &completions[mat.candidate_id];
                let documentation = &completion.documentation;

                let mut len = completion.label.text.chars().count();
                if let Some(Documentation::SingleLine(text)) = documentation {
                    if show_completion_documentation {
                        len += text.chars().count();
                    }
                }

                len
            })
            .map(|(ix, _)| ix);

        let completions = self.completions.clone();
        let matches = self.matches.clone();
        let selected_item = self.selected_item;
        let style = style.clone();

        let multiline_docs = if show_completion_documentation {
            let mat = &self.matches[selected_item];
            let multiline_docs = match &self.completions.read()[mat.candidate_id].documentation {
                Some(Documentation::MultiLinePlainText(text)) => {
                    Some(div().child(SharedString::from(text.clone())))
                }
                Some(Documentation::MultiLineMarkdown(parsed)) if !parsed.text.is_empty() => {
                    Some(div().child(render_parsed_markdown(
                        "completions_markdown",
                        parsed,
                        &style,
                        workspace,
                        cx,
                    )))
                }
                _ => None,
            };
            multiline_docs.map(|div| {
                div.id("multiline_docs")
                    .max_h(max_height)
                    .flex_1()
                    .px_1p5()
                    .py_1()
                    .min_w(px(260.))
                    .max_w(px(640.))
                    .w(px(500.))
                    .overflow_y_scroll()
                    .occlude()
            })
        } else {
            None
        };

        let list = uniform_list(
            cx.view().clone(),
            "completions",
            matches.len(),
            move |_editor, range, cx| {
                let start_ix = range.start;
                let completions_guard = completions.read();

                matches[range]
                    .iter()
                    .enumerate()
                    .map(|(ix, mat)| {
                        let item_ix = start_ix + ix;
                        let candidate_id = mat.candidate_id;
                        let completion = &completions_guard[candidate_id];

                        let documentation = if show_completion_documentation {
                            &completion.documentation
                        } else {
                            &None
                        };

                        let highlights = gpui::combine_highlights(
                            mat.ranges().map(|range| (range, FontWeight::BOLD.into())),
                            styled_runs_for_code_label(&completion.label, &style.syntax).map(
                                |(range, mut highlight)| {
                                    // Ignore font weight for syntax highlighting, as we'll use it
                                    // for fuzzy matches.
                                    highlight.font_weight = None;

                                    if completion.lsp_completion.deprecated.unwrap_or(false) {
                                        highlight.strikethrough = Some(StrikethroughStyle {
                                            thickness: 1.0.into(),
                                            ..Default::default()
                                        });
                                        highlight.color = Some(cx.theme().colors().text_muted);
                                    }

                                    (range, highlight)
                                },
                            ),
                        );
                        let completion_label = StyledText::new(completion.label.text.clone())
                            .with_highlights(&style.text, highlights);
                        let documentation_label =
                            if let Some(Documentation::SingleLine(text)) = documentation {
                                if text.trim().is_empty() {
                                    None
                                } else {
                                    Some(
                                        Label::new(text.clone())
                                            .ml_4()
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                }
                            } else {
                                None
                            };

                        let color_swatch = completion
                            .color()
                            .map(|color| div().size_4().bg(color).rounded_sm());

                        div().min_w(px(220.)).max_w(px(540.)).child(
                            ListItem::new(mat.candidate_id)
                                .inset(true)
                                .selected(item_ix == selected_item)
                                .on_click(cx.listener(move |editor, _event, cx| {
                                    cx.stop_propagation();
                                    if let Some(task) = editor.confirm_completion(
                                        &ConfirmCompletion {
                                            item_ix: Some(item_ix),
                                        },
                                        cx,
                                    ) {
                                        task.detach_and_log_err(cx)
                                    }
                                }))
                                .start_slot::<Div>(color_swatch)
                                .child(h_flex().overflow_hidden().child(completion_label))
                                .end_slot::<Label>(documentation_label),
                        )
                    })
                    .collect()
            },
        )
        .occlude()
        .max_h(max_height)
        .track_scroll(self.scroll_handle.clone())
        .with_width_from_item(widest_completion_ix)
        .with_sizing_behavior(ListSizingBehavior::Infer);

        (
            Popover::new()
                .child(list)
                .when_some(multiline_docs, |popover, multiline_docs| {
                    popover.aside(multiline_docs)
                })
                .into_any_element(),
            is_inverted,
        )
    }

    pub async fn filter(&mut self, query: Option<&str>, executor: BackgroundExecutor) {
        let mut matches = if let Some(query) = query {
            fuzzy::match_strings(
                &self.match_candidates,
                query,
                query.chars().any(|c| c.is_uppercase()),
                100,
                &Default::default(),
                executor,
            )
            .await
        } else {
            self.match_candidates
                .iter()
                .enumerate()
                .map(|(candidate_id, candidate)| StringMatch {
                    candidate_id,
                    score: Default::default(),
                    positions: Default::default(),
                    string: candidate.string.clone(),
                })
                .collect()
        };

        // Remove all candidates where the query's start does not match the start of any word in the candidate
        if let Some(query) = query {
            if let Some(query_start) = query.chars().next() {
                matches.retain(|string_match| {
                    split_words(&string_match.string).any(|word| {
                        // Check that the first codepoint of the word as lowercase matches the first
                        // codepoint of the query as lowercase
                        word.chars()
                            .flat_map(|codepoint| codepoint.to_lowercase())
                            .zip(query_start.to_lowercase())
                            .all(|(word_cp, query_cp)| word_cp == query_cp)
                    })
                });
            }
        }

        let completions = self.completions.read();
        if self.sort_completions {
            matches.sort_unstable_by_key(|mat| {
                // We do want to strike a balance here between what the language server tells us
                // to sort by (the sort_text) and what are "obvious" good matches (i.e. when you type
                // `Creat` and there is a local variable called `CreateComponent`).
                // So what we do is: we bucket all matches into two buckets
                // - Strong matches
                // - Weak matches
                // Strong matches are the ones with a high fuzzy-matcher score (the "obvious" matches)
                // and the Weak matches are the rest.
                //
                // For the strong matches, we sort by our fuzzy-finder score first and for the weak
                // matches, we prefer language-server sort_text first.
                //
                // The thinking behind that: we want to show strong matches first in order of relevance(fuzzy score).
                // Rest of the matches(weak) can be sorted as language-server expects.

                #[derive(PartialEq, Eq, PartialOrd, Ord)]
                enum MatchScore<'a> {
                    Strong {
                        score: Reverse<OrderedFloat<f64>>,
                        sort_text: Option<&'a str>,
                        sort_key: (usize, &'a str),
                    },
                    Weak {
                        sort_text: Option<&'a str>,
                        score: Reverse<OrderedFloat<f64>>,
                        sort_key: (usize, &'a str),
                    },
                }

                let completion = &completions[mat.candidate_id];
                let sort_key = completion.sort_key();
                let sort_text = completion.lsp_completion.sort_text.as_deref();
                let score = Reverse(OrderedFloat(mat.score));

                if mat.score >= 0.2 {
                    MatchScore::Strong {
                        score,
                        sort_text,
                        sort_key,
                    }
                } else {
                    MatchScore::Weak {
                        sort_text,
                        score,
                        sort_key,
                    }
                }
            });
        }

        for mat in &mut matches {
            let completion = &completions[mat.candidate_id];
            mat.string.clone_from(&completion.label.text);
            for position in &mut mat.positions {
                *position += completion.label.filter_range.start;
            }
        }
        drop(completions);

        self.matches = matches.into();
        self.selected_item = 0;
    }
}

pub(super) struct AvailableCodeAction {
    pub(super) excerpt_id: ExcerptId,
    pub(super) action: CodeAction,
    pub(super) provider: Arc<dyn CodeActionProvider>,
}

#[derive(Clone)]
pub(super) struct CodeActionContents {
    pub(super) tasks: Option<Arc<ResolvedTasks>>,
    pub(super) actions: Option<Arc<[AvailableCodeAction]>>,
}

impl CodeActionContents {
    fn len(&self) -> usize {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => actions.len() + tasks.templates.len(),
            (Some(tasks), None) => tasks.templates.len(),
            (None, Some(actions)) => actions.len(),
            (None, None) => 0,
        }
    }

    fn is_empty(&self) -> bool {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => actions.is_empty() && tasks.templates.is_empty(),
            (Some(tasks), None) => tasks.templates.is_empty(),
            (None, Some(actions)) => actions.is_empty(),
            (None, None) => true,
        }
    }

    fn iter(&self) -> impl Iterator<Item = CodeActionsItem> + '_ {
        self.tasks
            .iter()
            .flat_map(|tasks| {
                tasks
                    .templates
                    .iter()
                    .map(|(kind, task)| CodeActionsItem::Task(kind.clone(), task.clone()))
            })
            .chain(self.actions.iter().flat_map(|actions| {
                actions.iter().map(|available| CodeActionsItem::CodeAction {
                    excerpt_id: available.excerpt_id,
                    action: available.action.clone(),
                    provider: available.provider.clone(),
                })
            }))
    }
    pub(super) fn get(&self, index: usize) -> Option<CodeActionsItem> {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => {
                if index < tasks.templates.len() {
                    tasks
                        .templates
                        .get(index)
                        .cloned()
                        .map(|(kind, task)| CodeActionsItem::Task(kind, task))
                } else {
                    actions.get(index - tasks.templates.len()).map(|available| {
                        CodeActionsItem::CodeAction {
                            excerpt_id: available.excerpt_id,
                            action: available.action.clone(),
                            provider: available.provider.clone(),
                        }
                    })
                }
            }
            (Some(tasks), None) => tasks
                .templates
                .get(index)
                .cloned()
                .map(|(kind, task)| CodeActionsItem::Task(kind, task)),
            (None, Some(actions)) => {
                actions
                    .get(index)
                    .map(|available| CodeActionsItem::CodeAction {
                        excerpt_id: available.excerpt_id,
                        action: available.action.clone(),
                        provider: available.provider.clone(),
                    })
            }
            (None, None) => None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub(super) enum CodeActionsItem {
    Task(TaskSourceKind, ResolvedTask),
    CodeAction {
        excerpt_id: ExcerptId,
        action: CodeAction,
        provider: Arc<dyn CodeActionProvider>,
    },
}

impl CodeActionsItem {
    fn as_task(&self) -> Option<&ResolvedTask> {
        let Self::Task(_, task) = self else {
            return None;
        };
        Some(task)
    }
    fn as_code_action(&self) -> Option<&CodeAction> {
        let Self::CodeAction { action, .. } = self else {
            return None;
        };
        Some(action)
    }
    pub(super) fn label(&self) -> String {
        match self {
            Self::CodeAction { action, .. } => action.lsp_action.title.clone(),
            Self::Task(_, task) => task.resolved_label.clone(),
        }
    }
}

pub(crate) struct CodeActionsMenu {
    pub(super) actions: CodeActionContents,
    pub(super) buffer: Model<Buffer>,
    pub(super) selected_item: usize,
    pub(super) scroll_handle: UniformListScrollHandle,
    pub(super) deployed_from_indicator: Option<DisplayRow>,
}

impl CodeActionsMenu {
    fn select_first(&mut self, cx: &mut ViewContext<Editor>) {
        self.selected_item = 0;
        self.scroll_handle.scroll_to_item(self.selected_item);
        cx.notify()
    }

    fn select_prev(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
        } else {
            self.selected_item = self.actions.len() - 1;
        }
        self.scroll_handle.scroll_to_item(self.selected_item);
        cx.notify();
    }

    fn select_next(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item + 1 < self.actions.len() {
            self.selected_item += 1;
        } else {
            self.selected_item = 0;
        }
        self.scroll_handle.scroll_to_item(self.selected_item);
        cx.notify();
    }

    fn select_last(&mut self, cx: &mut ViewContext<Editor>) {
        self.selected_item = self.actions.len() - 1;
        self.scroll_handle.scroll_to_item(self.selected_item);
        cx.notify()
    }

    fn visible(&self) -> bool {
        !self.actions.is_empty()
    }

    fn render(
        &self,
        cursor_position: DisplayPoint,
        _style: &EditorStyle,
        max_height: Pixels,
        cx: &mut ViewContext<Editor>,
    ) -> RenderedContextMenu {
        let actions = self.actions.clone();
        let selected_item = self.selected_item;
        let is_inverted = Arc::new(Default::default());
        let element = uniform_list(
            cx.view().clone(),
            "code_actions_menu",
            self.actions.len(),
            move |_this, range, cx| {
                actions
                    .iter()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .enumerate()
                    .map(|(ix, action)| {
                        let item_ix = range.start + ix;
                        let selected = selected_item == item_ix;
                        let colors = cx.theme().colors();
                        div()
                            .px_1()
                            .rounded_md()
                            .text_color(colors.text)
                            .when(selected, |style| {
                                style
                                    .bg(colors.element_active)
                                    .text_color(colors.text_accent)
                            })
                            .hover(|style| {
                                style
                                    .bg(colors.element_hover)
                                    .text_color(colors.text_accent)
                            })
                            .whitespace_nowrap()
                            .when_some(action.as_code_action(), |this, action| {
                                this.on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |editor, _, cx| {
                                        cx.stop_propagation();
                                        if let Some(task) = editor.confirm_code_action(
                                            &ConfirmCodeAction {
                                                item_ix: Some(item_ix),
                                            },
                                            cx,
                                        ) {
                                            task.detach_and_log_err(cx)
                                        }
                                    }),
                                )
                                // TASK: It would be good to make lsp_action.title a SharedString to avoid allocating here.
                                .child(SharedString::from(action.lsp_action.title.clone()))
                            })
                            .when_some(action.as_task(), |this, task| {
                                this.on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |editor, _, cx| {
                                        cx.stop_propagation();
                                        if let Some(task) = editor.confirm_code_action(
                                            &ConfirmCodeAction {
                                                item_ix: Some(item_ix),
                                            },
                                            cx,
                                        ) {
                                            task.detach_and_log_err(cx)
                                        }
                                    }),
                                )
                                .child(SharedString::from(task.resolved_label.clone()))
                            })
                    })
                    .collect()
            },
        )
        .elevation_1(cx)
        .p_1()
        .max_h(max_height)
        .occlude()
        .track_scroll(self.scroll_handle.clone())
        .with_width_from_item(
            self.actions
                .iter()
                .enumerate()
                .max_by_key(|(_, action)| match action {
                    CodeActionsItem::Task(_, task) => task.resolved_label.chars().count(),
                    CodeActionsItem::CodeAction { action, .. } => {
                        action.lsp_action.title.chars().count()
                    }
                })
                .map(|(ix, _)| ix),
        )
        .with_sizing_behavior(ListSizingBehavior::Infer)
        .into_any_element();

        let origin = if let Some(row) = self.deployed_from_indicator {
            ContextMenuOrigin::GutterIndicator(row)
        } else {
            ContextMenuOrigin::EditorPoint(cursor_position)
        };

        RenderedContextMenu {
            origin,
            element,
            is_inverted,
        }
    }
}
