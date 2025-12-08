use std::{
    any::TypeId,
    collections::VecDeque,
    ops::Add,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use client::{Client, UserStore};
use editor::{Editor, PathKey};
use futures::StreamExt as _;
use gpui::{
    Animation, AnimationExt, App, AppContext as _, Context, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement as _, IntoElement as _, ParentElement as _, SharedString,
    Styled as _, Task, TextAlign, Window, actions, div, pulsating_between,
};
use multi_buffer::MultiBuffer;
use project::Project;
use text::OffsetRangeExt;
use ui::{
    ButtonCommon, Clickable, Disableable, FluentBuilder as _, IconButton, IconName,
    StyledTypography as _, h_flex, v_flex,
};

use edit_prediction::{
    ContextRetrievalFinishedDebugEvent, ContextRetrievalStartedDebugEvent, DebugEvent,
    EditPredictionStore,
};
use workspace::Item;

pub struct EditPredictionContextView {
    empty_focus_handle: FocusHandle,
    project: Entity<Project>,
    store: Entity<EditPredictionStore>,
    runs: VecDeque<RetrievalRun>,
    current_ix: usize,
    _update_task: Task<Result<()>>,
}

#[derive(Debug)]
struct RetrievalRun {
    editor: Entity<Editor>,
    started_at: Instant,
    metadata: Vec<(&'static str, SharedString)>,
    finished_at: Option<Instant>,
}

actions!(
    dev,
    [
        /// Go to the previous context retrieval run
        EditPredictionContextGoBack,
        /// Go to the next context retrieval run
        EditPredictionContextGoForward
    ]
);

impl EditPredictionContextView {
    pub fn new(
        project: Entity<Project>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let store = EditPredictionStore::global(client, user_store, cx);

        let mut debug_rx = store.update(cx, |store, _| store.debug_info());
        let _update_task = cx.spawn_in(window, async move |this, cx| {
            while let Some(event) = debug_rx.next().await {
                this.update_in(cx, |this, window, cx| {
                    this.handle_store_event(event, window, cx)
                })?;
            }
            Ok(())
        });

        Self {
            empty_focus_handle: cx.focus_handle(),
            project,
            runs: VecDeque::new(),
            current_ix: 0,
            store,
            _update_task,
        }
    }

    fn handle_store_event(
        &mut self,
        event: DebugEvent,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            DebugEvent::ContextRetrievalStarted(info) => {
                if info.project_entity_id == self.project.entity_id() {
                    self.handle_context_retrieval_started(info, window, cx);
                }
            }
            DebugEvent::ContextRetrievalFinished(info) => {
                if info.project_entity_id == self.project.entity_id() {
                    self.handle_context_retrieval_finished(info, window, cx);
                }
            }
            DebugEvent::EditPredictionRequested(_) => {}
        }
    }

    fn handle_context_retrieval_started(
        &mut self,
        info: ContextRetrievalStartedDebugEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .runs
            .back()
            .is_some_and(|run| run.finished_at.is_none())
        {
            self.runs.pop_back();
        }

        let multibuffer = cx.new(|_| MultiBuffer::new(language::Capability::ReadOnly));
        let editor = cx
            .new(|cx| Editor::for_multibuffer(multibuffer, Some(self.project.clone()), window, cx));

        if self.runs.len() == 32 {
            self.runs.pop_front();
        }

        self.runs.push_back(RetrievalRun {
            editor,
            started_at: info.timestamp,
            finished_at: None,
            metadata: Vec::new(),
        });

        cx.notify();
    }

    fn handle_context_retrieval_finished(
        &mut self,
        info: ContextRetrievalFinishedDebugEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(run) = self.runs.back_mut() else {
            return;
        };

        run.finished_at = Some(info.timestamp);
        run.metadata = info.metadata;

        let project = self.project.clone();
        let related_files = self
            .store
            .read(cx)
            .context_for_project(&self.project, cx)
            .to_vec();

        let editor = run.editor.clone();
        let multibuffer = run.editor.read(cx).buffer().clone();

        if self.current_ix + 2 == self.runs.len() {
            self.current_ix += 1;
        }

        cx.spawn_in(window, async move |this, cx| {
            let mut paths = Vec::new();
            for related_file in related_files {
                let (buffer, point_ranges): (_, Vec<_>) =
                    if let Some(buffer) = related_file.buffer.upgrade() {
                        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

                        (
                            buffer,
                            related_file
                                .excerpts
                                .iter()
                                .map(|excerpt| excerpt.anchor_range.to_point(&snapshot))
                                .collect(),
                        )
                    } else {
                        (
                            project
                                .update(cx, |project, cx| {
                                    project.open_buffer(related_file.path.clone(), cx)
                                })?
                                .await?,
                            related_file
                                .excerpts
                                .iter()
                                .map(|excerpt| excerpt.point_range.clone())
                                .collect(),
                        )
                    };
                cx.update(|_, cx| {
                    let path = PathKey::for_buffer(&buffer, cx);
                    paths.push((path, buffer, point_ranges));
                })?;
            }

            multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.clear(cx);

                for (path, buffer, ranges) in paths {
                    multibuffer.set_excerpts_for_path(path, buffer, ranges, 0, cx);
                }
            })?;

            editor.update_in(cx, |editor, window, cx| {
                editor.move_to_beginning(&Default::default(), window, cx);
            })?;

            this.update(cx, |_, cx| cx.notify())
        })
        .detach();
    }

    fn handle_go_back(
        &mut self,
        _: &EditPredictionContextGoBack,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.current_ix = self.current_ix.saturating_sub(1);
        cx.focus_self(window);
        cx.notify();
    }

    fn handle_go_forward(
        &mut self,
        _: &EditPredictionContextGoForward,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.current_ix = self
            .current_ix
            .add(1)
            .min(self.runs.len().saturating_sub(1));
        cx.focus_self(window);
        cx.notify();
    }

    fn render_informational_footer(
        &self,
        cx: &mut Context<'_, EditPredictionContextView>,
    ) -> ui::Div {
        let run = &self.runs[self.current_ix];
        let new_run_started = self
            .runs
            .back()
            .map_or(false, |latest_run| latest_run.finished_at.is_none());

        h_flex()
            .p_2()
            .w_full()
            .font_buffer(cx)
            .text_xs()
            .border_t_1()
            .gap_2()
            .child(v_flex().h_full().flex_1().child({
                let t0 = run.started_at;
                let mut table = ui::Table::<2>::new().width(ui::px(300.)).no_ui_font();
                for (key, value) in &run.metadata {
                    table = table.row([key.into_any_element(), value.clone().into_any_element()])
                }
                table = table.row([
                    "Total Time".into_any_element(),
                    format!("{} ms", (run.finished_at.unwrap_or(t0) - t0).as_millis())
                        .into_any_element(),
                ]);
                table
            }))
            .child(
                v_flex().h_full().text_align(TextAlign::Right).child(
                    h_flex()
                        .justify_end()
                        .child(
                            IconButton::new("go-back", IconName::ChevronLeft)
                                .disabled(self.current_ix == 0 || self.runs.len() < 2)
                                .tooltip(ui::Tooltip::for_action_title(
                                    "Go to previous run",
                                    &EditPredictionContextGoBack,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.handle_go_back(&EditPredictionContextGoBack, window, cx);
                                })),
                        )
                        .child(
                            div()
                                .child(format!("{}/{}", self.current_ix + 1, self.runs.len()))
                                .map(|this| {
                                    if new_run_started {
                                        this.with_animation(
                                            "pulsating-count",
                                            Animation::new(Duration::from_secs(2))
                                                .repeat()
                                                .with_easing(pulsating_between(0.4, 0.8)),
                                            |label, delta| label.opacity(delta),
                                        )
                                        .into_any_element()
                                    } else {
                                        this.into_any_element()
                                    }
                                }),
                        )
                        .child(
                            IconButton::new("go-forward", IconName::ChevronRight)
                                .disabled(self.current_ix + 1 == self.runs.len())
                                .tooltip(ui::Tooltip::for_action_title(
                                    "Go to next run",
                                    &EditPredictionContextGoBack,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.handle_go_forward(
                                        &EditPredictionContextGoForward,
                                        window,
                                        cx,
                                    );
                                })),
                        ),
                ),
            )
    }
}

impl Focusable for EditPredictionContextView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.runs
            .get(self.current_ix)
            .map(|run| run.editor.read(cx).focus_handle(cx))
            .unwrap_or_else(|| self.empty_focus_handle.clone())
    }
}

impl EventEmitter<()> for EditPredictionContextView {}

impl Item for EditPredictionContextView {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Edit Prediction Context".into()
    }

    fn buffer_kind(&self, _cx: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Multibuffer
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.runs.get(self.current_ix)?.editor.clone().into())
        } else {
            None
        }
    }
}

impl gpui::Render for EditPredictionContextView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        v_flex()
            .key_context("EditPredictionContext")
            .on_action(cx.listener(Self::handle_go_back))
            .on_action(cx.listener(Self::handle_go_forward))
            .size_full()
            .map(|this| {
                if self.runs.is_empty() {
                    this.child(
                        v_flex()
                            .size_full()
                            .justify_center()
                            .items_center()
                            .child("No retrieval runs yet"),
                    )
                } else {
                    this.child(self.runs[self.current_ix].editor.clone())
                        .child(self.render_informational_footer(cx))
                }
            })
    }
}
