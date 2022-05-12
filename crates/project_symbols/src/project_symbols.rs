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
    pending_update: Task<()>,
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
        Self {
            project,
            picker: cx.add_view(|cx| Picker::new(handle, cx)),
            selected_match_index: 0,
            symbols: Default::default(),
            match_candidates: Default::default(),
            matches: Default::default(),
            show_worktree_root_name: false,
            pending_update: Task::ready(()),
        }
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        workspace.toggle_modal(cx, |workspace, cx| {
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
            cx.background_executor().block(fuzzy::match_strings(
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
        self.pending_update = cx.spawn_weak(|this, mut cx| async move {
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
        });
        Task::ready(())
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> ElementBox {
        let string_match = &self.matches[ix];
        let settings = cx.global::<Settings>();
        let style = &settings.theme.picker.item;
        let current_style = style.style_for(mouse_state, selected);
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
                Text::new(symbol.label.text.clone(), current_style.label.text.clone())
                    .with_soft_wrap(false)
                    .with_highlights(combine_syntax_and_fuzzy_match_highlights(
                        &symbol.label.text,
                        current_style.label.text.clone().into(),
                        syntax_runs,
                        &string_match.positions,
                    ))
                    .boxed(),
            )
            .with_child(
                // Avoid styling the path differently when it is selected, since
                // the symbol's syntax highlighting doesn't change when selected.
                Label::new(path.to_string(), style.default.label.clone()).boxed(),
            )
            .contained()
            .with_style(current_style.container)
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use gpui::{serde_json::json, TestAppContext};
    use language::{FakeLspAdapter, Language, LanguageConfig};
    use project::FakeFs;
    use std::sync::Arc;

    #[gpui::test]
    async fn test_project_symbols(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        cx.update(|cx| cx.set_global(Settings::test(cx)));

        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            None,
        );
        let mut fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter::default());

        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/dir", json!({ "test.rs": "" })).await;

        let project = Project::test(fs.clone(), ["/dir"], cx).await;
        project.update(cx, |project, _| project.languages().add(Arc::new(language)));

        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/dir/test.rs", cx)
            })
            .await
            .unwrap();

        // Set up fake langauge server to return fuzzy matches against
        // a fixed set of symbol names.
        let fake_symbol_names = ["one", "ton", "uno"];
        let fake_server = fake_servers.next().await.unwrap();
        fake_server.handle_request::<lsp::request::WorkspaceSymbol, _, _>(
            move |params: lsp::WorkspaceSymbolParams, cx| {
                let executor = cx.background();
                async move {
                    let candidates = fake_symbol_names
                        .into_iter()
                        .map(|name| StringMatchCandidate::new(0, name.into()))
                        .collect::<Vec<_>>();
                    let matches = if params.query.is_empty() {
                        Vec::new()
                    } else {
                        fuzzy::match_strings(
                            &candidates,
                            &params.query,
                            true,
                            100,
                            &Default::default(),
                            executor.clone(),
                        )
                        .await
                    };

                    Ok(Some(
                        matches.into_iter().map(|mat| symbol(&mat.string)).collect(),
                    ))
                }
            },
        );

        // Create the project symbols view.
        let (_, symbols_view) = cx.add_window(|cx| ProjectSymbolsView::new(project.clone(), cx));
        let picker = symbols_view.read_with(cx, |symbols_view, _| symbols_view.picker.clone());

        // Spawn multiples updates before the first update completes,
        // such that in the end, there are no matches. Testing for regression:
        // https://github.com/zed-industries/zed/issues/861
        picker.update(cx, |p, cx| {
            p.update_matches("o".to_string(), cx);
            p.update_matches("on".to_string(), cx);
            p.update_matches("onex".to_string(), cx);
        });

        cx.foreground().run_until_parked();
        symbols_view.read_with(cx, |symbols_view, _| {
            assert_eq!(symbols_view.matches.len(), 0);
        });

        // Spawn more updates such that in the end, there are matches.
        picker.update(cx, |p, cx| {
            p.update_matches("one".to_string(), cx);
            p.update_matches("on".to_string(), cx);
        });

        cx.foreground().run_until_parked();
        symbols_view.read_with(cx, |symbols_view, _| {
            assert_eq!(symbols_view.matches.len(), 2);
            assert_eq!(symbols_view.matches[0].string, "one");
            assert_eq!(symbols_view.matches[1].string, "ton");
        });

        // Spawn more updates such that in the end, there are again no matches.
        picker.update(cx, |p, cx| {
            p.update_matches("o".to_string(), cx);
            p.update_matches("".to_string(), cx);
        });

        cx.foreground().run_until_parked();
        symbols_view.read_with(cx, |symbols_view, _| {
            assert_eq!(symbols_view.matches.len(), 0);
        });
    }

    fn symbol(name: &str) -> lsp::SymbolInformation {
        #[allow(deprecated)]
        lsp::SymbolInformation {
            name: name.to_string(),
            kind: lsp::SymbolKind::FUNCTION,
            tags: None,
            deprecated: None,
            container_name: None,
            location: lsp::Location::new(
                lsp::Url::from_file_path("/a/b").unwrap(),
                lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
            ),
        }
    }
}
