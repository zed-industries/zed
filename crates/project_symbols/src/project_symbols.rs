use editor::{
    combine_syntax_and_fuzzy_match_highlights, styled_runs_for_code_label, Autoscroll, Bias, Editor,
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, elements::*, AppContext, Entity, ModelHandle, MutableAppContext, RenderContext, Task,
    View, ViewContext, ViewHandle,
};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use project::{Project, Symbol};
use settings::Settings;
use std::{borrow::Cow, cmp::Reverse};
use util::ResultExt;
use workspace::Workspace;

actions!(project_symbols, [Toggle]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ProjectSymbolsView::toggle);
    Picker::<ProjectSymbolsView>::init(cx);
}

pub struct ProjectSymbolsView {
    picker: ViewHandle<Picker<Self>>,
    project: ModelHandle<Project>,
    selected_match_index: usize,
    symbols: Vec<Symbol>,
    match_candidates: Vec<StringMatchCandidate>,
    show_worktree_root_name: bool,
    matches: Vec<StringMatch>,
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

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.picker.clone()).boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.picker);
    }
}

impl ProjectSymbolsView {
    fn new(project: ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        let handle = cx.weak_handle();
        let picker = cx.add_view(|cx| Picker::new(handle, cx));
        let mut this = Self {
            picker,
            project,
            selected_match_index: 0,
            symbols: Default::default(),
            match_candidates: Default::default(),
            matches: Default::default(),
            show_worktree_root_name: false,
        };
        this.update_matches(String::new(), cx).detach();
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

    fn filter(&mut self, query: &str, cx: &mut ViewContext<Self>) {
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
                query,
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
        self.set_selected_index(0, cx);
        cx.notify();
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

impl PickerDelegate for ProjectSymbolsView {
    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(symbol) = self
            .matches
            .get(self.selected_match_index)
            .map(|mat| self.symbols[mat.candidate_id].clone())
        {
            cx.emit(Event::Selected(symbol));
        }
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Self>) {
        self.selected_match_index = ix;
        cx.notify();
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> Task<()> {
        self.filter(&query, cx);
        self.show_worktree_root_name = self.project.read(cx).visible_worktrees(cx).count() > 1;
        let symbols = self
            .project
            .update(cx, |project, cx| project.symbols(&query, cx));
        cx.spawn_weak(|this, mut cx| async move {
            let symbols = symbols.await.log_err();
            if let Some(this) = this.upgrade(&cx) {
                if let Some(symbols) = symbols {
                    this.update(&mut cx, |this, cx| {
                        this.match_candidates = symbols
                            .iter()
                            .enumerate()
                            .map(|(id, symbol)| {
                                StringMatchCandidate::new(
                                    id,
                                    symbol.label.text[symbol.label.filter_range.clone()]
                                        .to_string(),
                                )
                            })
                            .collect();
                        this.symbols = symbols;
                        this.filter(&query, cx);
                    });
                }
            }
        })
    }

    fn render_match(&self, ix: usize, selected: bool, cx: &AppContext) -> ElementBox {
        let string_match = &self.matches[ix];
        let settings = cx.global::<Settings>();
        let style = if selected {
            &settings.theme.selector.active_item
        } else {
            &settings.theme.selector.item
        };
        let symbol = &self.symbols[string_match.candidate_id];
        let syntax_runs = styled_runs_for_code_label(&symbol.label, &settings.theme.editor.syntax);

        let mut path = symbol.path.to_string_lossy();
        if self.show_worktree_root_name {
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
}
