use editor::{
    combine_syntax_and_fuzzy_match_highlights, styled_runs_for_code_label, Editor, EditorSettings,
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    action,
    elements::*,
    keymap::{self, Binding},
    AppContext, Axis, Entity, ModelHandle, MutableAppContext, RenderContext, Task, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use ordered_float::OrderedFloat;
use postage::watch;
use project::{Project, ProjectSymbol};
use std::{
    cmp::{self, Reverse},
    sync::Arc,
};
use util::ResultExt;
use workspace::{
    menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrev},
    Settings, Workspace,
};

action!(Toggle);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-t", Toggle, None),
        Binding::new("escape", Toggle, Some("ProjectSymbolsView")),
    ]);
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
    settings: watch::Receiver<Settings>,
    selected_match_index: usize,
    list_state: UniformListState,
    symbols: Vec<ProjectSymbol>,
    match_candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    pending_symbols_task: Task<Option<()>>,
    query_editor: ViewHandle<Editor>,
}

pub enum Event {
    Dismissed,
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

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        let settings = self.settings.borrow();

        Flex::new(Axis::Vertical)
            .with_child(
                Container::new(ChildView::new(&self.query_editor).boxed())
                    .with_style(settings.theme.selector.input_editor.container)
                    .boxed(),
            )
            .with_child(Flexible::new(1.0, false, self.render_matches()).boxed())
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
    fn new(
        project: ModelHandle<Project>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            style: settings.theme.selector.input_editor.as_editor(),
                            tab_size: settings.tab_size,
                            soft_wrap: editor::SoftWrap::None,
                        }
                    })
                },
                cx,
            )
        });
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();
        let mut this = Self {
            handle: cx.weak_handle(),
            project,
            settings,
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
            let symbols = cx.add_view(|cx| Self::new(project, workspace.settings.clone(), cx));
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
        cx.emit(Event::Dismissed);
    }

    fn update_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.filter(cx);
        let query = self.query_editor.read(cx).text(cx);
        let symbols = self
            .project
            .update(cx, |project, cx| project.symbols(&query, cx));
        self.pending_symbols_task = cx.spawn_weak(|this, mut cx| async move {
            let symbols = symbols
                .await
                .log_err()?
                .into_values()
                .flatten()
                .collect::<Vec<_>>();
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

    fn render_matches(&self) -> ElementBox {
        if self.matches.is_empty() {
            let settings = self.settings.borrow();
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
                items.extend(
                    view.matches[range]
                        .iter()
                        .enumerate()
                        .map(move |(ix, m)| view.render_match(m, start + ix)),
                );
            },
        );

        Container::new(list.boxed())
            .with_margin_top(6.0)
            .named("matches")
    }

    fn render_match(&self, string_match: &StringMatch, index: usize) -> ElementBox {
        let settings = self.settings.borrow();
        let style = if index == self.selected_match_index {
            &settings.theme.selector.active_item
        } else {
            &settings.theme.selector.item
        };
        let symbol = &self.symbols[string_match.candidate_id];
        let syntax_runs = styled_runs_for_code_label(
            &symbol.label,
            style.label.text.color,
            &settings.theme.editor.syntax,
        );

        Text::new(symbol.label.text.clone(), style.label.text.clone())
            .with_soft_wrap(false)
            .with_highlights(combine_syntax_and_fuzzy_match_highlights(
                &symbol.label.text,
                style.label.text.clone().into(),
                syntax_runs,
                &string_match.positions,
            ))
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
            editor::Event::Edited => self.update_matches(cx),
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
        }
    }
}
