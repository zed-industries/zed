use editor::{
    combine_syntax_and_fuzzy_match_highlights, styled_runs_for_code_label, Autoscroll, Bias, Editor,
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, elements::*, keymap, AppContext, Axis, Entity, ModelHandle, MutableAppContext,
    RenderContext, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use ordered_float::OrderedFloat;
use project::{Project, Symbol};
use settings::Settings;
use std::{
    borrow::Cow,
    cmp::{self, Reverse},
};
use util::ResultExt;
use workspace::{
    menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrev},
    Workspace,
};

actions!(project_symbols, [Toggle]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ProjectSymbolsView::toggle);
    cx.add_action(ProjectSymbolsView::confirm);
    cx.add_action(ProjectSymbolsView::select_prev);
    cx.add_action(ProjectSymbolsView::select_next);
    cx.add_action(ProjectSymbolsView::select_first);
    cx.add_action(ProjectSymbolsView::select_last);
}

pub struct ProjectSymbolsView {
    handle: WeakViewHandle<Self>,
    project: ModelHandle<Project>,
    selected_match_index: usize,
    list_state: UniformListState,
    symbols: Vec<Symbol>,
    match_candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    pending_symbols_task: Task<Option<()>>,
    query_editor: ViewHandle<Editor>,
}

pub enum Event {
    Dismissed,
    Selected(Symbol),
}

impl Entity for ProjectSymbolsView {
    type Event = Event;
}

impl View for ProjectSymbolsView {
    fn ui_name() -> &'static str {
        "ProjectSymbolsView"
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let settings = cx.global::<Settings>();
        Flex::new(Axis::Vertical)
            .with_child(
                Container::new(ChildView::new(&self.query_editor).boxed())
                    .with_style(settings.theme.selector.input_editor.container)
                    .boxed(),
            )
            .with_child(
                FlexItem::new(self.render_matches(cx))
                    .flex(1., false)
                    .boxed(),
            )
            .contained()
            .with_style(settings.theme.selector.container)
            .constrained()
            .with_max_width(500.0)
            .with_max_height(420.0)
            .aligned()
            .top()
            .named("project symbols view")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }
}

impl ProjectSymbolsView {
    fn new(project: ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(Some(|theme| theme.selector.input_editor.clone()), cx)
        });
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();
        let mut this = Self {
            handle: cx.weak_handle(),
            project,
            selected_match_index: 0,
            list_state: Default::default(),
            symbols: Default::default(),
            match_candidates: Default::default(),
            matches: Default::default(),
            pending_symbols_task: Task::ready(None),
            query_editor,
        };
        this.update_matches(cx);
        this
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        workspace.toggle_modal(cx, |cx, workspace| {
            let project = workspace.project().clone();
            let symbols = cx.add_view(|cx| Self::new(project, cx));
            cx.subscribe(&symbols, Self::on_event).detach();
            symbols
        });
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if self.selected_match_index > 0 {
            self.select(self.selected_match_index - 1, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if self.selected_match_index + 1 < self.matches.len() {
            self.select(self.selected_match_index + 1, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        self.select(0, cx);
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        self.select(self.matches.len().saturating_sub(1), cx);
    }

    fn select(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        self.selected_match_index = index;
        self.list_state.scroll_to(ScrollTarget::Show(index));
        cx.notify();
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(symbol) = self
            .matches
            .get(self.selected_match_index)
            .map(|mat| self.symbols[mat.candidate_id].clone())
        {
            cx.emit(Event::Selected(symbol));
        }
    }

    fn update_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.filter(cx);
        let query = self.query_editor.read(cx).text(cx);
        let symbols = self
            .project
            .update(cx, |project, cx| project.symbols(&query, cx));
        self.pending_symbols_task = cx.spawn_weak(|this, mut cx| async move {
            let symbols = symbols.await.log_err()?;
            if let Some(this) = this.upgrade(&cx) {
                this.update(&mut cx, |this, cx| {
                    this.match_candidates = symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| {
                            StringMatchCandidate::new(
                                id,
                                symbol.label.text[symbol.label.filter_range.clone()].to_string(),
                            )
                        })
                        .collect();
                    this.symbols = symbols;
                    this.filter(cx);
                });
            }
            None
        });
    }

    fn filter(&mut self, cx: &mut ViewContext<Self>) {
        let query = self.query_editor.read(cx).text(cx);
        let mut matches = if query.is_empty() {
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
        } else {
            smol::block_on(fuzzy::match_strings(
                &self.match_candidates,
                &query,
                false,
                100,
                &Default::default(),
                cx.background().clone(),
            ))
        };

        matches.sort_unstable_by_key(|mat| {
            let label = &self.symbols[mat.candidate_id].label;
            (
                Reverse(OrderedFloat(mat.score)),
                &label.text[label.filter_range.clone()],
            )
        });

        for mat in &mut matches {
            let filter_start = self.symbols[mat.candidate_id].label.filter_range.start;
            for position in &mut mat.positions {
                *position += filter_start;
            }
        }

        self.matches = matches;
        self.select_first(&SelectFirst, cx);
        cx.notify();
    }

    fn render_matches(&self, cx: &AppContext) -> ElementBox {
        if self.matches.is_empty() {
            let settings = cx.global::<Settings>();
            return Container::new(
                Label::new(
                    "No matches".into(),
                    settings.theme.selector.empty.label.clone(),
                )
                .boxed(),
            )
            .with_style(settings.theme.selector.empty.container)
            .named("empty matches");
        }

        let handle = self.handle.clone();
        let list = UniformList::new(
            self.list_state.clone(),
            self.matches.len(),
            move |mut range, items, cx| {
                let cx = cx.as_ref();
                let view = handle.upgrade(cx).unwrap();
                let view = view.read(cx);
                let start = range.start;
                range.end = cmp::min(range.end, view.matches.len());

                let show_worktree_root_name =
                    view.project.read(cx).visible_worktrees(cx).count() > 1;
                items.extend(view.matches[range].iter().enumerate().map(move |(ix, m)| {
                    view.render_match(m, start + ix, show_worktree_root_name, cx)
                }));
            },
        );

        Container::new(list.boxed())
            .with_margin_top(6.0)
            .named("matches")
    }

    fn render_match(
        &self,
        string_match: &StringMatch,
        index: usize,
        show_worktree_root_name: bool,
        cx: &AppContext,
    ) -> ElementBox {
        let settings = cx.global::<Settings>();
        let style = if index == self.selected_match_index {
            &settings.theme.selector.active_item
        } else {
            &settings.theme.selector.item
        };
        let symbol = &self.symbols[string_match.candidate_id];
        let syntax_runs = styled_runs_for_code_label(&symbol.label, &settings.theme.editor.syntax);

        let mut path = symbol.path.to_string_lossy();
        if show_worktree_root_name {
            let project = self.project.read(cx);
            if let Some(worktree) = project.worktree_for_id(symbol.worktree_id, cx) {
                path = Cow::Owned(format!(
                    "{}{}{}",
                    worktree.read(cx).root_name(),
                    std::path::MAIN_SEPARATOR,
                    path.as_ref()
                ));
            }
        }

        Flex::column()
            .with_child(
                Text::new(symbol.label.text.clone(), style.label.text.clone())
                    .with_soft_wrap(false)
                    .with_highlights(combine_syntax_and_fuzzy_match_highlights(
                        &symbol.label.text,
                        style.label.text.clone().into(),
                        syntax_runs,
                        &string_match.positions,
                    ))
                    .boxed(),
            )
            .with_child(
                // Avoid styling the path differently when it is selected, since
                // the symbol's syntax highlighting doesn't change when selected.
                Label::new(path.to_string(), settings.theme.selector.item.label.clone()).boxed(),
            )
            .contained()
            .with_style(style.container)
            .boxed()
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Blurred => cx.emit(Event::Dismissed),
            editor::Event::BufferEdited { .. } => self.update_matches(cx),
            _ => {}
        }
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<Self>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => workspace.dismiss_modal(cx),
            Event::Selected(symbol) => {
                let buffer = workspace
                    .project()
                    .update(cx, |project, cx| project.open_buffer_for_symbol(symbol, cx));

                let symbol = symbol.clone();
                cx.spawn(|workspace, mut cx| async move {
                    let buffer = buffer.await?;
                    workspace.update(&mut cx, |workspace, cx| {
                        let position = buffer
                            .read(cx)
                            .clip_point_utf16(symbol.range.start, Bias::Left);

                        let editor = workspace.open_project_item::<Editor>(buffer, cx);
                        editor.update(cx, |editor, cx| {
                            editor.select_ranges(
                                [position..position],
                                Some(Autoscroll::Center),
                                cx,
                            );
                        });
                    });
                    Ok::<_, anyhow::Error>(())
                })
                .detach_and_log_err(cx);
                workspace.dismiss_modal(cx);
            }
        }
    }
}
