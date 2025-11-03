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
    Focusable, ParentElement as _, SharedString, Styled as _, Task, TextAlign, Window, actions,
    pulsating_between,
};
use multi_buffer::MultiBuffer;
use project::Project;
use text::OffsetRangeExt;
use ui::{
    ButtonCommon, Clickable, Color, Disableable, FluentBuilder as _, Icon, IconButton, IconName,
    IconSize, InteractiveElement, IntoElement, ListHeader, ListItem, StyledTypography, div, h_flex,
    v_flex,
};
use workspace::{Item, ItemHandle as _};
use zeta2::{
    Zeta, ZetaContextRetrievalDebugInfo, ZetaContextRetrievalStartedDebugInfo, ZetaDebugInfo,
    ZetaSearchQueryDebugInfo,
};

pub struct Zeta2ContextView {
    empty_focus_handle: FocusHandle,
    project: Entity<Project>,
    zeta: Entity<Zeta>,
    runs: VecDeque<RetrievalRun>,
    current_ix: usize,
    _update_task: Task<Result<()>>,
}

#[derive(Debug)]
struct RetrievalRun {
    editor: Entity<Editor>,
    search_queries: Vec<GlobQueries>,
    started_at: Instant,
    search_results_generated_at: Option<Instant>,
    search_results_executed_at: Option<Instant>,
    search_results_filtered_at: Option<Instant>,
    finished_at: Option<Instant>,
}

#[derive(Debug)]
struct GlobQueries {
    glob: String,
    alternations: Vec<String>,
}

actions!(
    dev,
    [
        /// Go to the previous context retrieval run
        Zeta2ContextGoBack,
        /// Go to the next context retrieval run
        Zeta2ContextGoForward
    ]
);

impl Zeta2ContextView {
    pub fn new(
        project: Entity<Project>,
        client: &Arc<Client>,
        user_store: &Entity<UserStore>,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> Self {
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
            empty_focus_handle: cx.focus_handle(),
            project,
            runs: VecDeque::new(),
            current_ix: 0,
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
            ZetaDebugInfo::SearchResultsFiltered(info) => {
                if info.project == self.project {
                    self.handle_search_results_filtered(info, window, cx);
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
        info: ZetaContextRetrievalStartedDebugInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .runs
            .back()
            .is_some_and(|run| run.search_results_executed_at.is_none())
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
            search_queries: Vec::new(),
            started_at: info.timestamp,
            search_results_generated_at: None,
            search_results_executed_at: None,
            search_results_filtered_at: None,
            finished_at: None,
        });

        cx.notify();
    }

    fn handle_context_retrieval_finished(
        &mut self,
        info: ZetaContextRetrievalDebugInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(run) = self.runs.back_mut() else {
            return;
        };

        run.finished_at = Some(info.timestamp);

        let multibuffer = run.editor.read(cx).buffer().clone();
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

        run.editor.update(cx, |editor, cx| {
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
        let Some(run) = self.runs.back_mut() else {
            return;
        };

        run.search_results_generated_at = Some(info.timestamp);
        run.search_queries = info
            .queries
            .into_iter()
            .map(|query| {
                let mut regex_parser = regex_syntax::ast::parse::Parser::new();

                GlobQueries {
                    glob: query.glob,
                    alternations: match regex_parser.parse(&query.regex) {
                        Ok(regex_syntax::ast::Ast::Alternation(ref alt)) => {
                            alt.asts.iter().map(|ast| ast.to_string()).collect()
                        }
                        _ => vec![query.regex],
                    },
                }
            })
            .collect();
        cx.notify();
    }

    fn handle_search_queries_executed(
        &mut self,
        info: ZetaContextRetrievalDebugInfo,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.current_ix + 2 == self.runs.len() {
            // Switch to latest when the queries are executed
            self.current_ix += 1;
        }

        let Some(run) = self.runs.back_mut() else {
            return;
        };

        run.search_results_executed_at = Some(info.timestamp);
        cx.notify();
    }

    fn handle_search_results_filtered(
        &mut self,
        info: ZetaContextRetrievalDebugInfo,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(run) = self.runs.back_mut() else {
            return;
        };

        run.search_results_filtered_at = Some(info.timestamp);
        cx.notify();
    }

    fn handle_go_back(
        &mut self,
        _: &Zeta2ContextGoBack,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.current_ix = self.current_ix.saturating_sub(1);
        cx.focus_self(window);
        cx.notify();
    }

    fn handle_go_forward(
        &mut self,
        _: &Zeta2ContextGoForward,
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

    fn render_informational_footer(&self, cx: &mut Context<'_, Zeta2ContextView>) -> ui::Div {
        let is_latest = self.runs.len() == self.current_ix + 1;
        let run = &self.runs[self.current_ix];

        h_flex()
            .p_2()
            .w_full()
            .font_buffer(cx)
            .text_xs()
            .border_t_1()
            .gap_2()
            .child(
                v_flex().h_full().flex_1().children(
                    run.search_queries
                        .iter()
                        .enumerate()
                        .flat_map(|(ix, query)| {
                            std::iter::once(ListHeader::new(query.glob.clone()).into_any_element())
                                .chain(query.alternations.iter().enumerate().map(
                                    move |(alt_ix, alt)| {
                                        ListItem::new(ix * 100 + alt_ix)
                                            .start_slot(
                                                Icon::new(IconName::MagnifyingGlass)
                                                    .color(Color::Muted)
                                                    .size(IconSize::Small),
                                            )
                                            .child(alt.clone())
                                            .into_any_element()
                                    },
                                ))
                        }),
                ),
            )
            .child(
                v_flex()
                    .h_full()
                    .text_align(TextAlign::Right)
                    .child(
                        h_flex()
                            .justify_end()
                            .child(
                                IconButton::new("go-back", IconName::ChevronLeft)
                                    .disabled(self.current_ix == 0 || self.runs.len() < 2)
                                    .tooltip(ui::Tooltip::for_action_title(
                                        "Go to previous run",
                                        &Zeta2ContextGoBack,
                                    ))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.handle_go_back(&Zeta2ContextGoBack, window, cx);
                                    })),
                            )
                            .child(
                                div()
                                    .child(format!("{}/{}", self.current_ix + 1, self.runs.len()))
                                    .map(|this| {
                                        if self.runs.back().is_some_and(|back| {
                                            back.search_results_executed_at.is_none()
                                        }) {
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
                                        &Zeta2ContextGoBack,
                                    ))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.handle_go_forward(&Zeta2ContextGoForward, window, cx);
                                    })),
                            ),
                    )
                    .map(|mut div| {
                        let pending_message = |div: ui::Div, msg: &'static str| {
                            if is_latest {
                                return div.child(msg);
                            } else {
                                return div.child("Canceled");
                            }
                        };

                        let t0 = run.started_at;
                        let Some(t1) = run.search_results_generated_at else {
                            return pending_message(div, "Planning search...");
                        };
                        div = div.child(format!("Planned search: {:>5} ms", (t1 - t0).as_millis()));

                        let Some(t2) = run.search_results_executed_at else {
                            return pending_message(div, "Running search...");
                        };
                        div = div.child(format!("Ran search: {:>5} ms", (t2 - t1).as_millis()));

                        let Some(t3) = run.search_results_filtered_at else {
                            return pending_message(div, "Filtering results...");
                        };
                        div =
                            div.child(format!("Filtered results: {:>5} ms", (t3 - t2).as_millis()));

                        let Some(t4) = run.finished_at else {
                            return pending_message(div, "Building excerpts");
                        };
                        div = div
                            .child(format!("Build excerpts: {:>5} µs", (t4 - t3).as_micros()))
                            .child(format!("Total: {:>5} ms", (t4 - t0).as_millis()));
                        div
                    }),
            )
    }
}

impl Focusable for Zeta2ContextView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.runs
            .get(self.current_ix)
            .map(|run| run.editor.read(cx).focus_handle(cx))
            .unwrap_or_else(|| self.empty_focus_handle.clone())
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
            Some(self.runs.get(self.current_ix)?.editor.to_any())
        } else {
            None
        }
    }
}

impl gpui::Render for Zeta2ContextView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        v_flex()
            .key_context("Zeta2Context")
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
