use editor::{
    combine_syntax_and_fuzzy_match_highlights, scroll::autoscroll::Autoscroll,
    styled_runs_for_code_label, Bias, Editor,
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, elements::*, AppContext, ModelHandle, MouseState, Task, ViewContext, WeakViewHandle,
};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate, PickerEvent};
use project::{Project, Symbol};
use settings::Settings;
use std::{borrow::Cow, cmp::Reverse, sync::Arc};
use util::ResultExt;
use workspace::Workspace;

actions!(project_symbols, [Toggle]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(toggle);
    ProjectSymbols::init(cx);
}

fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
    workspace.toggle_modal(cx, |workspace, cx| {
        let project = workspace.project().clone();
        let workspace = cx.weak_handle();
        cx.add_view(|cx| ProjectSymbols::new(ProjectSymbolsDelegate::new(workspace, project), cx))
    });
}

pub type ProjectSymbols = Picker<ProjectSymbolsDelegate>;

pub struct ProjectSymbolsDelegate {
    workspace: WeakViewHandle<Workspace>,
    project: ModelHandle<Project>,
    selected_match_index: usize,
    symbols: Vec<Symbol>,
    visible_match_candidates: Vec<StringMatchCandidate>,
    external_match_candidates: Vec<StringMatchCandidate>,
    show_worktree_root_name: bool,
    matches: Vec<StringMatch>,
}

impl ProjectSymbolsDelegate {
    fn new(workspace: WeakViewHandle<Workspace>, project: ModelHandle<Project>) -> Self {
        Self {
            workspace,
            project,
            selected_match_index: 0,
            symbols: Default::default(),
            visible_match_candidates: Default::default(),
            external_match_candidates: Default::default(),
            matches: Default::default(),
            show_worktree_root_name: false,
        }
    }

    fn filter(&mut self, query: &str, cx: &mut ViewContext<ProjectSymbols>) {
        const MAX_MATCHES: usize = 100;
        let mut visible_matches = cx.background_executor().block(fuzzy::match_strings(
            &self.visible_match_candidates,
            query,
            false,
            MAX_MATCHES,
            &Default::default(),
            cx.background().clone(),
        ));
        let mut external_matches = cx.background_executor().block(fuzzy::match_strings(
            &self.external_match_candidates,
            query,
            false,
            MAX_MATCHES - visible_matches.len(),
            &Default::default(),
            cx.background().clone(),
        ));
        let sort_key_for_match = |mat: &StringMatch| {
            let symbol = &self.symbols[mat.candidate_id];
            (
                Reverse(OrderedFloat(mat.score)),
                &symbol.label.text[symbol.label.filter_range.clone()],
            )
        };

        visible_matches.sort_unstable_by_key(sort_key_for_match);
        external_matches.sort_unstable_by_key(sort_key_for_match);
        let mut matches = visible_matches;
        matches.append(&mut external_matches);

        for mat in &mut matches {
            let symbol = &self.symbols[mat.candidate_id];
            let filter_start = symbol.label.filter_range.start;
            for position in &mut mat.positions {
                *position += filter_start;
            }
        }

        self.matches = matches;
        self.set_selected_index(0, cx);
    }
}

impl PickerDelegate for ProjectSymbolsDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Search project symbols...".into()
    }

    fn confirm(&mut self, cx: &mut ViewContext<ProjectSymbols>) {
        if let Some(symbol) = self
            .matches
            .get(self.selected_match_index)
            .map(|mat| self.symbols[mat.candidate_id].clone())
        {
            let buffer = self.project.update(cx, |project, cx| {
                project.open_buffer_for_symbol(&symbol, cx)
            });
            let symbol = symbol.clone();
            let workspace = self.workspace.clone();
            cx.spawn(|_, mut cx| async move {
                let buffer = buffer.await?;
                workspace.update(&mut cx, |workspace, cx| {
                    let position = buffer
                        .read(cx)
                        .clip_point_utf16(symbol.range.start, Bias::Left);

                    let editor = workspace.open_project_item::<Editor>(buffer, cx);
                    editor.update(cx, |editor, cx| {
                        editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                            s.select_ranges([position..position])
                        });
                    });
                })?;
                Ok::<_, anyhow::Error>(())
            })
            .detach_and_log_err(cx);
            cx.emit(PickerEvent::Dismiss);
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<ProjectSymbols>) {}

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<ProjectSymbols>) {
        self.selected_match_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<ProjectSymbols>) -> Task<()> {
        self.filter(&query, cx);
        self.show_worktree_root_name = self.project.read(cx).visible_worktrees(cx).count() > 1;
        let symbols = self
            .project
            .update(cx, |project, cx| project.symbols(&query, cx));
        cx.spawn(|this, mut cx| async move {
            let symbols = symbols.await.log_err();
            if let Some(symbols) = symbols {
                this.update(&mut cx, |this, cx| {
                    let delegate = this.delegate_mut();
                    let project = delegate.project.read(cx);
                    let (visible_match_candidates, external_match_candidates) = symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| {
                            StringMatchCandidate::new(
                                id,
                                symbol.label.text[symbol.label.filter_range.clone()].to_string(),
                            )
                        })
                        .partition(|candidate| {
                            project
                                .entry_for_path(&symbols[candidate.id].path, cx)
                                .map_or(false, |e| !e.is_ignored)
                        });

                    delegate.visible_match_candidates = visible_match_candidates;
                    delegate.external_match_candidates = external_match_candidates;
                    delegate.symbols = symbols;
                    delegate.filter(&query, cx);
                })
                .log_err();
            }
        })
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> AnyElement<Picker<Self>> {
        let string_match = &self.matches[ix];
        let settings = cx.global::<Settings>();
        let style = &settings.theme.picker.item;
        let current_style = style.style_for(mouse_state, selected);
        let symbol = &self.symbols[string_match.candidate_id];
        let syntax_runs = styled_runs_for_code_label(&symbol.label, &settings.theme.editor.syntax);

        let mut path = symbol.path.path.to_string_lossy();
        if self.show_worktree_root_name {
            let project = self.project.read(cx);
            if let Some(worktree) = project.worktree_for_id(symbol.path.worktree_id, cx) {
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
                    )),
            )
            .with_child(
                // Avoid styling the path differently when it is selected, since
                // the symbol's syntax highlighting doesn't change when selected.
                Label::new(path.to_string(), style.default.label.clone()),
            )
            .contained()
            .with_style(current_style.container)
            .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use gpui::{serde_json::json, TestAppContext};
    use language::{FakeLspAdapter, Language, LanguageConfig};
    use project::FakeFs;
    use settings::SettingsStore;
    use std::{path::Path, sync::Arc};

    #[gpui::test]
    async fn test_project_symbols(cx: &mut TestAppContext) {
        init_test(cx);

        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            None,
        );
        let mut fake_servers = language
            .set_fake_lsp_adapter(Arc::<FakeLspAdapter>::default())
            .await;

        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/dir", json!({ "test.rs": "" })).await;

        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages().add(Arc::new(language)));

        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/dir/test.rs", cx)
            })
            .await
            .unwrap();

        // Set up fake langauge server to return fuzzy matches against
        // a fixed set of symbol names.
        let fake_symbols = [
            symbol("one", "/external"),
            symbol("ton", "/dir/test.rs"),
            symbol("uno", "/dir/test.rs"),
        ];
        let fake_server = fake_servers.next().await.unwrap();
        fake_server.handle_request::<lsp::request::WorkspaceSymbol, _, _>(
            move |params: lsp::WorkspaceSymbolParams, cx| {
                let executor = cx.background();
                let fake_symbols = fake_symbols.clone();
                async move {
                    let candidates = fake_symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| StringMatchCandidate::new(id, symbol.name.clone()))
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
                        matches
                            .into_iter()
                            .map(|mat| fake_symbols[mat.candidate_id].clone())
                            .collect(),
                    ))
                }
            },
        );

        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

        // Create the project symbols view.
        let symbols = cx.add_view(window_id, |cx| {
            ProjectSymbols::new(
                ProjectSymbolsDelegate::new(workspace.downgrade(), project.clone()),
                cx,
            )
        });

        // Spawn multiples updates before the first update completes,
        // such that in the end, there are no matches. Testing for regression:
        // https://github.com/zed-industries/zed/issues/861
        symbols.update(cx, |p, cx| {
            p.update_matches("o".to_string(), cx);
            p.update_matches("on".to_string(), cx);
            p.update_matches("onex".to_string(), cx);
        });

        cx.foreground().run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            assert_eq!(symbols.delegate().matches.len(), 0);
        });

        // Spawn more updates such that in the end, there are matches.
        symbols.update(cx, |p, cx| {
            p.update_matches("one".to_string(), cx);
            p.update_matches("on".to_string(), cx);
        });

        cx.foreground().run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            let delegate = symbols.delegate();
            assert_eq!(delegate.matches.len(), 2);
            assert_eq!(delegate.matches[0].string, "ton");
            assert_eq!(delegate.matches[1].string, "one");
        });

        // Spawn more updates such that in the end, there are again no matches.
        symbols.update(cx, |p, cx| {
            p.update_matches("o".to_string(), cx);
            p.update_matches("".to_string(), cx);
        });

        cx.foreground().run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            assert_eq!(symbols.delegate().matches.len(), 0);
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            cx.set_global(Settings::test(cx));
            cx.set_global(SettingsStore::test(cx));
            language::init(cx);
        });
    }

    fn symbol(name: &str, path: impl AsRef<Path>) -> lsp::SymbolInformation {
        #[allow(deprecated)]
        lsp::SymbolInformation {
            name: name.to_string(),
            kind: lsp::SymbolKind::FUNCTION,
            tags: None,
            deprecated: None,
            container_name: None,
            location: lsp::Location::new(
                lsp::Url::from_file_path(path.as_ref()).unwrap(),
                lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
            ),
        }
    }
}
