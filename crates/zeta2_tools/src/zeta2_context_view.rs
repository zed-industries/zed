use std::{any::TypeId, sync::Arc, time::Instant};

use anyhow::Result;
use client::{Client, UserStore};
use editor::{Editor, PathKey};
use futures::StreamExt as _;
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable,
    ParentElement as _, SharedString, Styled as _, Task, TextAlign, Window,
};
use multi_buffer::MultiBuffer;
use project::Project;
use text::OffsetRangeExt;
use ui::{
    Color, FluentBuilder as _, Icon, IconName, IconSize, ListItem, StyledTypography, h_flex, v_flex,
};
use workspace::{Item, ItemHandle as _};
use zeta2::{
    SearchToolQuery, Zeta, ZetaContextRetrievalDebugInfo, ZetaDebugInfo, ZetaSearchQueryDebugInfo,
};

pub struct Zeta2ContextView {
    project: Entity<Project>,
    editor: Entity<Editor>,
    search_queries: Vec<SearchToolQuery>,
    zeta: Entity<Zeta>,
    context_retrieval_started_at: Instant,
    search_results_generated_at: Option<Instant>,
    search_results_executed_at: Option<Instant>,
    context_retrieval_finished_at: Option<Instant>,
    _update_task: Task<Result<()>>,
}

impl Zeta2ContextView {
    pub fn new(
        project: Entity<Project>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|_| MultiBuffer::new(language::Capability::ReadOnly));
        let editor =
            cx.new(|cx| Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx));
        let zeta = Zeta::global(client, user_store, cx);

        let mut debug_rx = zeta.update(cx, |zeta, _| zeta.debug_info());
        let _update_task = cx.spawn_in(window, async move |this, cx| {
            while let Some(event) = debug_rx.next().await {
                this.update_in(cx, |this, window, cx| {
                    this.handle_zeta_event(event, window, cx)
                })?;
            }
            Ok(())
        });

        Self {
            project,
            editor,
            search_queries: Vec::new(),
            context_retrieval_started_at: Instant::now(),
            search_results_generated_at: None,
            search_results_executed_at: None,
            context_retrieval_finished_at: None,
            zeta,
            _update_task,
        }
    }

    fn handle_zeta_event(
        &mut self,
        event: ZetaDebugInfo,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ZetaDebugInfo::ContextRetrievalStarted(info) => {
                if info.project == self.project {
                    self.handle_context_retrieval_started(info, window, cx);
                }
            }
            ZetaDebugInfo::SearchQueriesGenerated(info) => {
                if info.project == self.project {
                    self.handle_search_queries_generated(info, window, cx);
                }
            }
            ZetaDebugInfo::SearchQueriesExecuted(info) => {
                if info.project == self.project {
                    self.handle_search_queries_executed(info, window, cx);
                }
            }
            ZetaDebugInfo::ContextRetrievalFinished(info) => {
                if info.project == self.project {
                    self.handle_context_retrieval_finished(info, window, cx);
                }
            }
            ZetaDebugInfo::EditPredicted(_) => {}
        }
    }

    fn handle_context_retrieval_started(
        &mut self,
        info: ZetaContextRetrievalDebugInfo,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_retrieval_started_at = info.timestamp;
        self.search_results_generated_at.take();
        self.search_results_executed_at.take();
        self.context_retrieval_finished_at.take();
        self.search_queries.clear();
        cx.notify();
    }

    fn handle_context_retrieval_finished(
        &mut self,
        info: ZetaContextRetrievalDebugInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_retrieval_finished_at = Some(info.timestamp);

        let multibuffer = self.editor.read(cx).buffer().clone();
        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.clear(cx);

            let context = self.zeta.read(cx).context_for_project(&self.project);
            let mut paths = Vec::new();
            for (buffer, ranges) in context {
                let path = PathKey::for_buffer(&buffer, cx);
                let snapshot = buffer.read(cx).snapshot();
                let ranges = ranges
                    .iter()
                    .map(|range| range.to_point(&snapshot))
                    .collect::<Vec<_>>();
                paths.push((path, buffer, ranges));
            }

            for (path, buffer, ranges) in paths {
                multibuffer.set_excerpts_for_path(path, buffer, ranges, 0, cx);
            }
        });

        self.editor.update(cx, |editor, cx| {
            editor.move_to_beginning(&Default::default(), window, cx);
        });

        cx.notify();
    }

    fn handle_search_queries_generated(
        &mut self,
        info: ZetaSearchQueryDebugInfo,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.search_results_generated_at = Some(info.timestamp);
        self.search_queries = info.queries;
        cx.notify();
    }

    fn handle_search_queries_executed(
        &mut self,
        info: ZetaContextRetrievalDebugInfo,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.search_results_executed_at = Some(info.timestamp);
        cx.notify();
    }

    fn render_informational_footer(&mut self, cx: &mut Context<'_, Zeta2ContextView>) -> ui::Div {
        h_flex()
            .w_full()
            .font_buffer(cx)
            .text_xs()
            .border_t_1()
            .child(
                v_flex()
                    .h_full()
                    .flex_1()
                    .children(self.search_queries.iter().enumerate().map(|(ix, query)| {
                        ListItem::new(ix)
                            .start_slot(
                                Icon::new(IconName::MagnifyingGlass)
                                    .color(Color::Muted)
                                    .size(IconSize::Small),
                            )
                            .child(query.regex.clone())
                    })),
            )
            .child(
                v_flex()
                    .h_full()
                    .pr_2()
                    .text_align(TextAlign::Right)
                    .map(|mut div| {
                        let t0 = self.context_retrieval_started_at;
                        let Some(t1) = self.search_results_generated_at else {
                            return div.child("Planning search...");
                        };
                        div = div.child(format!("Planned search: {:>5} ms", (t1 - t0).as_millis()));

                        let Some(t2) = self.search_results_executed_at else {
                            return div.child("Running search...");
                        };
                        div = div.child(format!("Ran search: {:>5} ms", (t2 - t1).as_millis()));

                        let Some(t3) = self.context_retrieval_finished_at else {
                            return div.child("Filtering results...");
                        };
                        div.child(format!("Filtered results: {:>5} ms", (t3 - t2).as_millis()))
                    }),
            )
    }
}

impl Focusable for Zeta2ContextView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

impl EventEmitter<()> for Zeta2ContextView {}

impl Item for Zeta2ContextView {
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
    ) -> Option<gpui::AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }
}

impl gpui::Render for Zeta2ContextView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        v_flex()
            .size_full()
            .child(self.editor.clone())
            .child(self.render_informational_footer(cx))
    }
}
