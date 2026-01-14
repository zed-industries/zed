use editor::{Bias, Editor, SelectionEffects, scroll::Autoscroll, styled_runs_for_code_label};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, Context, DismissEvent, Entity, HighlightStyle, ParentElement, StyledText, Task, TextStyle,
    WeakEntity, Window, relative, rems,
};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use project::{Project, Symbol, lsp_store::SymbolLocation};
use settings::Settings;
use std::{cmp::Reverse, sync::Arc};
use theme::{ActiveTheme, ThemeSettings};
use util::ResultExt;
use workspace::{
    Workspace,
    ui::{LabelLike, ListItem, ListItemSpacing, prelude::*},
};

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _: &mut Context<Workspace>| {
            workspace.register_action(
                |workspace, _: &workspace::ToggleProjectSymbols, window, cx| {
                    let project = workspace.project().clone();
                    let handle = cx.entity().downgrade();
                    workspace.toggle_modal(window, cx, move |window, cx| {
                        let delegate = ProjectSymbolsDelegate::new(handle, project);
                        Picker::uniform_list(delegate, window, cx).width(rems(34.))
                    })
                },
            );
        },
    )
    .detach();
}

pub type ProjectSymbols = Entity<Picker<ProjectSymbolsDelegate>>;

pub struct ProjectSymbolsDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    selected_match_index: usize,
    symbols: Vec<Symbol>,
    visible_match_candidates: Vec<StringMatchCandidate>,
    external_match_candidates: Vec<StringMatchCandidate>,
    show_worktree_root_name: bool,
    matches: Vec<StringMatch>,
}

impl ProjectSymbolsDelegate {
    fn new(workspace: WeakEntity<Workspace>, project: Entity<Project>) -> Self {
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

    // Note if you make changes to this, also change `agent_ui::completion_provider::search_symbols`
    fn filter(&mut self, query: &str, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        const MAX_MATCHES: usize = 100;
        let mut visible_matches = cx.foreground_executor().block_on(fuzzy::match_strings(
            &self.visible_match_candidates,
            query,
            false,
            true,
            MAX_MATCHES,
            &Default::default(),
            cx.background_executor().clone(),
        ));
        let mut external_matches = cx.foreground_executor().block_on(fuzzy::match_strings(
            &self.external_match_candidates,
            query,
            false,
            true,
            MAX_MATCHES - visible_matches.len().min(MAX_MATCHES),
            &Default::default(),
            cx.background_executor().clone(),
        ));
        let sort_key_for_match = |mat: &StringMatch| {
            let symbol = &self.symbols[mat.candidate_id];
            (Reverse(OrderedFloat(mat.score)), symbol.label.filter_text())
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
        self.set_selected_index(0, window, cx);
    }
}

impl PickerDelegate for ProjectSymbolsDelegate {
    type ListItem = ListItem;
    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search project symbols...".into()
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
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
            cx.spawn_in(window, async move |_, cx| {
                let buffer = buffer.await?;
                workspace.update_in(cx, |workspace, window, cx| {
                    let position = buffer
                        .read(cx)
                        .clip_point_utf16(symbol.range.start, Bias::Left);
                    let pane = if secondary {
                        workspace.adjacent_pane(window, cx)
                    } else {
                        workspace.active_pane().clone()
                    };

                    let editor = workspace.open_project_item::<Editor>(
                        pane, buffer, true, true, true, true, window, cx,
                    );

                    editor.update(cx, |editor, cx| {
                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::center()),
                            window,
                            cx,
                            |s| s.select_ranges([position..position]),
                        );
                    });
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_match_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        // Try to support rust-analyzer's path based symbols feature which
        // allows to search by rust path syntax, in that case we only want to
        // filter names by the last segment
        // Ideally this was a first class LSP feature (rich queries)
        let query_filter = query
            .rsplit_once("::")
            .map_or(&*query, |(_, suffix)| suffix)
            .to_owned();
        self.filter(&query_filter, window, cx);
        self.show_worktree_root_name = self.project.read(cx).visible_worktrees(cx).count() > 1;
        let symbols = self
            .project
            .update(cx, |project, cx| project.symbols(&query, cx));
        cx.spawn_in(window, async move |this, cx| {
            let symbols = symbols.await.log_err();
            if let Some(symbols) = symbols {
                this.update_in(cx, |this, window, cx| {
                    let delegate = &mut this.delegate;
                    let project = delegate.project.read(cx);
                    let (visible_match_candidates, external_match_candidates) = symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| {
                            StringMatchCandidate::new(id, symbol.label.filter_text())
                        })
                        .partition(|candidate| {
                            if let SymbolLocation::InProject(path) = &symbols[candidate.id].path {
                                project
                                    .entry_for_path(path, cx)
                                    .is_some_and(|e| !e.is_ignored)
                            } else {
                                false
                            }
                        });

                    delegate.visible_match_candidates = visible_match_candidates;
                    delegate.external_match_candidates = external_match_candidates;
                    delegate.symbols = symbols;
                    delegate.filter(&query_filter, window, cx);
                })
                .log_err();
            }
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let path_style = self.project.read(cx).path_style(cx);
        let string_match = &self.matches.get(ix)?;
        let symbol = &self.symbols.get(string_match.candidate_id)?;
        let theme = cx.theme();
        let local_player = theme.players().local();
        let syntax_runs = styled_runs_for_code_label(&symbol.label, theme.syntax(), &local_player);

        let path = match &symbol.path {
            SymbolLocation::InProject(project_path) => {
                let project = self.project.read(cx);
                let mut path = project_path.path.clone();
                if self.show_worktree_root_name
                    && let Some(worktree) = project.worktree_for_id(project_path.worktree_id, cx)
                {
                    path = worktree.read(cx).root_name().join(&path);
                }
                path.display(path_style).into_owned().into()
            }
            SymbolLocation::OutsideProject {
                abs_path,
                signature: _,
            } => abs_path.to_string_lossy(),
        };
        let label = symbol.label.text.clone();
        let line_number = symbol.range.start.0.row + 1;
        let path = path.into_owned();

        let settings = ThemeSettings::get_global(cx);

        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(1.),
            ..Default::default()
        };

        let highlight_style = HighlightStyle {
            background_color: Some(cx.theme().colors().text_accent.alpha(0.3)),
            ..Default::default()
        };
        let custom_highlights = string_match
            .positions
            .iter()
            .map(|pos| (*pos..pos + 1, highlight_style));

        let highlights = gpui::combine_highlights(custom_highlights, syntax_runs);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .child(LabelLike::new().child(
                            StyledText::new(label).with_default_highlights(&text_style, highlights),
                        ))
                        .child(
                            h_flex()
                                .child(Label::new(path).size(LabelSize::Small).color(Color::Muted))
                                .child(
                                    Label::new(format!(":{}", line_number))
                                        .size(LabelSize::Small)
                                        .color(Color::Placeholder),
                                ),
                        ),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use gpui::{TestAppContext, VisualContext};
    use language::{FakeLspAdapter, Language, LanguageConfig, LanguageMatcher};
    use lsp::OneOf;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{path::Path, sync::Arc};
    use util::path;

    #[gpui::test]
    async fn test_project_symbols(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({ "test.rs": "" }))
            .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )));
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    workspace_symbol_provider: Some(OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/dir/test.rs"), cx)
            })
            .await
            .unwrap();

        // Set up fake language server to return fuzzy matches against
        // a fixed set of symbol names.
        let fake_symbols = [
            symbol("one", path!("/external")),
            symbol("ton", path!("/dir/test.rs")),
            symbol("uno", path!("/dir/test.rs")),
        ];
        let fake_server = fake_servers.next().await.unwrap();
        fake_server.set_request_handler::<lsp::WorkspaceSymbolRequest, _, _>(
            move |params: lsp::WorkspaceSymbolParams, cx| {
                let executor = cx.background_executor().clone();
                let fake_symbols = fake_symbols.clone();
                async move {
                    let (query, prefixed) = match params.query.strip_prefix("dir::") {
                        Some(query) => (query, true),
                        None => (&*params.query, false),
                    };
                    let candidates = fake_symbols
                        .iter()
                        .enumerate()
                        .filter(|(_, symbol)| {
                            !prefixed || symbol.location.uri.path().contains("dir")
                        })
                        .map(|(id, symbol)| StringMatchCandidate::new(id, &symbol.name))
                        .collect::<Vec<_>>();
                    let matches = if query.is_empty() {
                        Vec::new()
                    } else {
                        fuzzy::match_strings(
                            &candidates,
                            &query,
                            true,
                            true,
                            100,
                            &Default::default(),
                            executor.clone(),
                        )
                        .await
                    };

                    Ok(Some(lsp::WorkspaceSymbolResponse::Flat(
                        matches
                            .into_iter()
                            .map(|mat| fake_symbols[mat.candidate_id].clone())
                            .collect(),
                    )))
                }
            },
        );

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // Create the project symbols view.
        let symbols = cx.new_window_entity(|window, cx| {
            Picker::uniform_list(
                ProjectSymbolsDelegate::new(workspace.downgrade(), project.clone()),
                window,
                cx,
            )
        });

        // Spawn multiples updates before the first update completes,
        // such that in the end, there are no matches. Testing for regression:
        // https://github.com/zed-industries/zed/issues/861
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("o".to_string(), window, cx);
            p.update_matches("on".to_string(), window, cx);
            p.update_matches("onex".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            assert_eq!(symbols.delegate.matches.len(), 0);
        });

        // Spawn more updates such that in the end, there are matches.
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("one".to_string(), window, cx);
            p.update_matches("on".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            let delegate = &symbols.delegate;
            assert_eq!(delegate.matches.len(), 2);
            assert_eq!(delegate.matches[0].string, "ton");
            assert_eq!(delegate.matches[1].string, "one");
        });

        // Spawn more updates such that in the end, there are again no matches.
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("o".to_string(), window, cx);
            p.update_matches("".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            assert_eq!(symbols.delegate.matches.len(), 0);
        });

        // Check that rust-analyzer path style symbols work
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("dir::to".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            assert_eq!(symbols.delegate.matches.len(), 1);
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            release_channel::init(semver::Version::new(0, 0, 0), cx);
            editor::init(cx);
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
                lsp::Uri::from_file_path(path.as_ref()).unwrap(),
                lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
            ),
        }
    }
}
