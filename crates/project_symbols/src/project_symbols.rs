use anyhow::anyhow;
use editor::{Bias, Editor, SelectionEffects, scroll::Autoscroll, styled_runs_for_code_label};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, Context, DismissEvent, Entity, HighlightStyle, ParentElement, StyledText, Task, TaskExt,
    TextStyle, WeakEntity, Window, relative,
};
use language::{
    CodeLabel, HighlightId, LanguageName, LanguageServerId, Point, PointUtf16, SymbolKind,
    Unclipped,
    language_settings::LanguageSettings,
};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate, PreviewUpdate};
use project::{Project, ProjectPath, Symbol, WorktreeId, lsp_store::SymbolLocation};
use settings::{ProjectSymbols as ProjectSymbolsSetting, Settings};
use std::{
    cmp::Reverse,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use symbol_index::SymbolSearchResult;
use theme::ActiveTheme;
use theme_settings::ThemeSettings;
use util::ResultExt;
use workspace::{
    Workspace,
    ui::{LabelLike, ListItem, ListItemSpacing, prelude::*},
};

/// Maps a `SymbolKind` to a syntax highlight name for label coloring.
/// Returns `None` for kinds without a natural highlight name.
fn symbol_kind_highlight_name(kind: SymbolKind) -> Option<&'static str> {
    match kind {
        SymbolKind::Function | SymbolKind::Method => Some("function.method"),
        SymbolKind::Constructor => Some("constructor"),
        SymbolKind::Struct | SymbolKind::Class | SymbolKind::Interface | SymbolKind::TypeParameter => {
            Some("type")
        }
        SymbolKind::Enum => Some("enum"),
        SymbolKind::EnumMember => Some("variant"),
        SymbolKind::Field | SymbolKind::Property => Some("property"),
        SymbolKind::Constant => Some("constant"),
        SymbolKind::Variable => Some("variable"),
        SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package => Some("namespace"),
        SymbolKind::Operator => Some("operator"),
        SymbolKind::Event => Some("function"),
        SymbolKind::File
        | SymbolKind::String
        | SymbolKind::Number
        | SymbolKind::Boolean
        | SymbolKind::Array
        | SymbolKind::Object
        | SymbolKind::Key
        | SymbolKind::Null => None,
    }
}

/// Where a match's [`Symbol`] came from; tree-sitter symbols carry the
/// byte-based point used to position the cursor on confirmation.
#[derive(Clone, Copy)]
enum SymbolSource {
    Lsp,
    TreeSitter { row: u32, column: u32 },
}

/// Converts a tree-sitter index result into a [`project::Symbol`] so that the
/// tree-sitter and LSP sources can share the picker's rendering and filtering.
fn to_project_symbol(result: &SymbolSearchResult, cx: &App) -> Option<Symbol> {
    let project_path = ProjectPath::from_worktree_and_path(
        result.symbol.location.worktree_id,
        &result.symbol.location.path,
    )?;
    let name_range = result.symbol.name_range.start as usize..result.symbol.name_range.end as usize;
    let highlight_id = symbol_kind_highlight_name(result.symbol.kind)
        .and_then(|highlight_name| cx.theme().syntax().highlight_id(highlight_name))
        .map(HighlightId::new);

    Some(Symbol {
        // Placeholder values: tree-sitter symbols are never routed through the
        // language server paths that read these fields.
        language_server_name: "tree-sitter".into(),
        source_worktree_id: WorktreeId::from_proto(result.symbol.location.worktree_id),
        source_language_server_id: LanguageServerId(0),
        path: SymbolLocation::InProject(project_path),
        label: CodeLabel::new(
            result.symbol.display_text.to_string(),
            name_range.clone(),
            highlight_id.map(|highlight_id| vec![(name_range, highlight_id)]).unwrap_or_default(),
        ),
        name: result.symbol.name.to_string(),
        kind: result.symbol.kind,
        range: Unclipped(PointUtf16::new(result.symbol.row, result.symbol.column))
            ..Unclipped(PointUtf16::new(result.symbol.row, result.symbol.column)),
        container_name: None,
    })
}

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _: &mut Context<Workspace>| {
            workspace.register_action(
                |workspace, _: &workspace::ToggleProjectSymbols, window, cx| {
                    let project = workspace.project().clone();
                    let handle = cx.entity().downgrade();
                    workspace.toggle_modal(window, cx, move |window, cx| {
                        let delegate = ProjectSymbolsDelegate::new(handle, project.clone());
                        let preview = picker_preview::editor_preview(project, window, cx);
                        Picker::uniform_list_with_preview(delegate, preview, window, cx)
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
    sources: Vec<SymbolSource>,
    visible_match_candidates: Vec<StringMatchCandidate>,
    external_match_candidates: Vec<StringMatchCandidate>,
    show_worktree_root_name: bool,
    matches: Vec<StringMatch>,
    cancel_flag: Option<Arc<AtomicBool>>,
}

impl ProjectSymbolsDelegate {
    fn new(workspace: WeakEntity<Workspace>, project: Entity<Project>) -> Self {
        Self {
            workspace,
            project,
            selected_match_index: 0,
            symbols: Default::default(),
            sources: Default::default(),
            visible_match_candidates: Default::default(),
            external_match_candidates: Default::default(),
            matches: Default::default(),
            show_worktree_root_name: false,
            cancel_flag: None,
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

    /// Replaces the symbol list with the merged LSP and tree-sitter results,
    /// deciding per language which source to keep based on the
    /// `project_symbols` setting.
    fn set_symbols(
        &mut self,
        lsp_symbols: Vec<Symbol>,
        tree_sitter_results: Option<Vec<SymbolSearchResult>>,
        query_filter: &str,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let (symbols, sources) = {
            let project = self.project.read(cx);
            let mut tree_sitter_keep_by_language = std::collections::HashMap::new();
            let mut keep_tree_sitter_for = |language_name: &LanguageName| -> bool {
                *tree_sitter_keep_by_language
                    .entry(language_name.clone())
                    .or_insert_with(|| {
                        let setting =
                            LanguageSettings::resolve(None, Some(language_name), cx).project_symbols;
                        match setting {
                            ProjectSymbolsSetting::TreeSitter => true,
                            ProjectSymbolsSetting::LanguageServer => false,
                            ProjectSymbolsSetting::Auto => {
                                !project.has_lsp_project_symbols(language_name, cx)
                            }
                        }
                    })
            };

            let mut symbols = Vec::new();
            let mut sources = Vec::new();

            for symbol in lsp_symbols {
                let keep_tree_sitter = match &symbol.path {
                    SymbolLocation::InProject(project_path) => project
                        .worktree_for_id(project_path.worktree_id, cx)
                        .and_then(|worktree| {
                            let abs_path = worktree.read(cx).absolutize(&project_path.path);
                            project
                                .languages()
                                .language_for_file_path(&abs_path)
                                .and_then(|language_id| {
                                    project.languages().language_name_for_id(language_id)
                                })
                        })
                        .map(|language_name| keep_tree_sitter_for(&language_name))
                        .unwrap_or(false),
                    // Symbols outside the project have no language association, so
                    // they can only ever come from the language server.
                    SymbolLocation::OutsideProject { .. } => false,
                };
                // Languages served by the tree-sitter index contribute their own
                // symbols below, so drop any language server symbols for them.
                if keep_tree_sitter {
                    continue;
                }
                sources.push(SymbolSource::Lsp);
                symbols.push(symbol);
            }

            if let Some(tree_sitter_results) = tree_sitter_results {
                for result in tree_sitter_results {
                    let Some(project_path) = ProjectPath::from_worktree_and_path(
                        result.symbol.location.worktree_id,
                        &result.symbol.location.path,
                    ) else {
                        continue;
                    };
                    let Some(worktree) = project.worktree_for_id(project_path.worktree_id, cx)
                    else {
                        continue;
                    };
                    let abs_path = worktree.read(cx).absolutize(&project_path.path);
                    let Some(language_id) = project.languages().language_for_file_path(&abs_path)
                    else {
                        continue;
                    };
                    let Some(language_name) =
                        project.languages().language_name_for_id(language_id)
                    else {
                        continue;
                    };
                    if !keep_tree_sitter_for(&language_name) {
                        continue;
                    }
                    if let Some(symbol) = to_project_symbol(&result, cx) {
                        sources.push(SymbolSource::TreeSitter {
                            row: result.symbol.row,
                            column: result.symbol.column,
                        });
                        symbols.push(symbol);
                    }
                }
            }

            (symbols, sources)
        };

        self.symbols = symbols;
        self.sources = sources;

        let project = self.project.read(cx);
        let (visible_match_candidates, external_match_candidates) = self
            .symbols
            .iter()
            .enumerate()
            .map(|(id, symbol)| StringMatchCandidate::new(id, symbol.label.filter_text()))
            .partition(|candidate| {
                if let SymbolLocation::InProject(path) = &self.symbols[candidate.id].path {
                    project
                        .entry_for_path(path, cx)
                        .is_some_and(|e| !e.is_ignored)
                } else {
                    false
                }
            });

        self.visible_match_candidates = visible_match_candidates;
        self.external_match_candidates = external_match_candidates;
        self.filter(query_filter, window, cx);
    }
}

impl PickerDelegate for ProjectSymbolsDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "project symbols"
    }
    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search project symbols...".into()
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_match_index) else {
            return;
        };
        let symbol = self.symbols[mat.candidate_id].clone();
        let source = self.sources[mat.candidate_id];
        let buffer = self.project.update(cx, |project, cx| match source {
            SymbolSource::Lsp => project.open_buffer_for_symbol(&symbol, cx),
            SymbolSource::TreeSitter { .. } => {
                let SymbolLocation::InProject(project_path) = &symbol.path else {
                    return Task::ready(Err(anyhow!("tree-sitter symbol without project path")));
                };
                project.open_buffer(project_path.clone(), cx)
            }
        });
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            let buffer = buffer.await?;
            workspace.update_in(cx, |workspace, window, cx| {
                let position = match source {
                    SymbolSource::Lsp => {
                        let buffer_snapshot = buffer.read(cx).text_snapshot();
                        let start = buffer
                            .read(cx)
                            .clip_point_utf16(symbol.range.start, Bias::Left);
                        buffer_snapshot.point_utf16_to_point(start)
                    }
                    SymbolSource::TreeSitter { row, column } => buffer
                        .read(cx)
                        .clip_point(Point::new(row, column), Bias::Left),
                };
                let pane = if secondary {
                    workspace.adjacent_pane(window, cx)
                } else {
                    workspace.active_pane().clone()
                };

                let editor = workspace.open_project_item::<Editor>(
                    secondary.then_some(pane),
                    buffer,
                    true,
                    true,
                    true,
                    true,
                    window,
                    cx,
                );

                editor.update(cx, |editor, cx| {
                    let multibuffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                    let Some(buffer_snapshot) = multibuffer_snapshot.as_singleton() else {
                        return;
                    };
                    let text_anchor = buffer_snapshot.anchor_before(position);
                    let Some(anchor) = multibuffer_snapshot.anchor_in_buffer(text_anchor)
                    else {
                        return;
                    };
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()),
                        window,
                        cx,
                        |s| s.select_ranges([anchor..anchor]),
                    );
                });
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
        cx.emit(DismissEvent);
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

    fn try_get_preview_data_for_match(&self, cx: &App) -> Option<PreviewUpdate> {
        let candidate_id = self.matches.get(self.selected_match_index)?.candidate_id;
        let source = self.sources[candidate_id];
        match source {
            SymbolSource::Lsp => {
                let symbol = self.symbols.get(candidate_id)?.clone();
                Some(PreviewUpdate::from_symbol(symbol))
            }
            // `PreviewUpdate::from_symbol` opens the buffer through the language
            // server that reported the symbol, which tree-sitter symbols have none
            // of, so preview those from their path without a highlight.
            SymbolSource::TreeSitter { .. } => {
                let SymbolLocation::InProject(project_path) = &self.symbols[candidate_id].path
                else {
                    return None;
                };
                let worktree = self
                    .project
                    .read(cx)
                    .worktree_for_id(project_path.worktree_id, cx)?;
                let abs_path = worktree.read(cx).absolutize(&project_path.path);
                Some(PreviewUpdate::from_path(abs_path))
            }
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        // Cancel any tree-sitter search from a previous query so a slow search
        // can't clobber the results of the current one.
        if let Some(previous_flag) = self.cancel_flag.take() {
            previous_flag.store(true, Ordering::Relaxed);
        }

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

        // Remote projects run their symbol index on the other side and are
        // already covered by the LSP path, so only search the tree-sitter
        // index for local projects.
        let project_is_remote = self.project.read(cx).is_remote();
        let index_snapshot = if project_is_remote {
            None
        } else {
            Some(self.project.update(cx, |project, cx| {
                project
                    .symbol_index(cx)
                    .update(cx, |manager, _cx| manager.snapshot())
            }))
        };
        let cancel_flag = Arc::new(AtomicBool::new(false));
        self.cancel_flag = Some(cancel_flag.clone());
        let executor = cx.background_executor().clone();

        cx.spawn_in(window, async move |this, cx| {
            let lsp_symbols = symbols.await.log_err();
            let tree_sitter_results = match index_snapshot {
                Some(snapshot) => {
                    let results =
                        snapshot.search(&query_filter, 200, cancel_flag.clone(), executor).await;
                    // If the search was cancelled, a newer query has already been
                    // issued and will write its own results; an empty vec from a
                    // cancelled search must not overwrite them.
                    if cancel_flag.load(Ordering::Relaxed) {
                        return;
                    }
                    Some(results)
                }
                None => None,
            };
            this.update_in(cx, |this, window, cx| {
                let delegate = &mut this.delegate;
                delegate.set_symbols(
                    lsp_symbols.unwrap_or_default(),
                    tree_sitter_results,
                    &query_filter,
                    window,
                    cx,
                );
            })
            .log_err();
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
                let mut path = project_path.path.to_rel_path_buf();
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
            .map(|pos| (*pos..label.ceil_char_boundary(pos + 1), highlight_style));

        let highlights = gpui::combine_highlights(custom_highlights, syntax_runs);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .child(
                            LabelLike::new().child(
                                StyledText::new(&label)
                                    .with_default_highlights(&text_style, highlights),
                            ),
                        )
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
    use workspace::MultiWorkspace;

    const RUST_OUTLINE_QUERY: &str = r#"
(function_item
  (visibility_modifier)? @context
  (function_modifiers)? @context
  "fn" @context
  name: (_) @name
  body: (_
    .
    "{" @open
    "}" @close .)) @item

(struct_item
  name: (_) @name) @item

(enum_item
  name: (_) @name) @item
"#;

    fn rust_language() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: (LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            },
            Some(language::tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(RUST_OUTLINE_QUERY)
        .unwrap()
    }

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
                matcher: (LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                })
                .into(),
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
        // Wait for the fake server to finish starting up so it is registered
        // as a running language server.
        cx.run_until_parked();
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

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

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

    #[gpui::test]
    async fn test_project_symbols_renders_utf8_match(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({ "test.rs": "" }))
            .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: (LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                })
                .into(),
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

        let fake_symbols = [symbol("안녕", path!("/dir/test.rs"))];
        let fake_server = fake_servers.next().await.unwrap();
        // Wait for the fake server to finish starting up so it is registered
        // as a running language server.
        cx.run_until_parked();
        fake_server.set_request_handler::<lsp::WorkspaceSymbolRequest, _, _>(
            move |params: lsp::WorkspaceSymbolParams, cx| {
                let executor = cx.background_executor().clone();
                let fake_symbols = fake_symbols.clone();
                async move {
                    let candidates = fake_symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| StringMatchCandidate::new(id, &symbol.name))
                        .collect::<Vec<_>>();
                    let matches = fuzzy::match_strings(
                        &candidates,
                        &params.query,
                        true,
                        true,
                        100,
                        &Default::default(),
                        executor,
                    )
                    .await;

                    Ok(Some(lsp::WorkspaceSymbolResponse::Flat(
                        matches
                            .into_iter()
                            .map(|mat| fake_symbols[mat.candidate_id].clone())
                            .collect(),
                    )))
                }
            },
        );

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let symbols = cx.new_window_entity(|window, cx| {
            Picker::uniform_list(
                ProjectSymbolsDelegate::new(workspace.downgrade(), project.clone()),
                window,
                cx,
            )
        });

        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("안".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            assert_eq!(symbols.delegate.matches.len(), 1);
            assert_eq!(symbols.delegate.matches[0].string, "안녕");
        });

        symbols.update_in(cx, |p, window, cx| {
            assert!(p.delegate.render_match(0, false, window, cx).is_some());
        });
    }

    #[gpui::test]
    async fn test_project_symbols_tree_sitter_fallback(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "test.rs": r#"
fn alpha_function() {}
fn beta_function() {}
struct GammaStruct {}
enum DeltaEnum {}
"#,
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

        // No fake LSP is registered: the tree-sitter index is the only symbol
        // source, which the default "auto" setting should fall back to.
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_language()));

        // Initialize the symbol index and wait for indexing to complete.
        project.update(cx, |project, cx| {
            project.symbol_index(cx);
        });
        cx.run_until_parked();

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let symbols = cx.new_window_entity(|window, cx| {
            Picker::uniform_list(
                ProjectSymbolsDelegate::new(workspace.downgrade(), project.clone()),
                window,
                cx,
            )
        });

        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("alpha".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            let delegate = &symbols.delegate;
            assert_eq!(delegate.matches.len(), 1);
            assert_eq!(delegate.matches[0].string, "alpha_function");
            assert!(matches!(
                delegate.sources[delegate.matches[0].candidate_id],
                SymbolSource::TreeSitter { .. }
            ));
        });

        // Empty query returns nothing.
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            assert_eq!(symbols.delegate.matches.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_project_symbols_no_duplicates_with_lsp(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "test.rs": r#"
fn alpha_function() {}
"#,
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_language()));
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

        project.update(cx, |project, cx| {
            project.symbol_index(cx);
        });
        cx.run_until_parked();

        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/dir/test.rs"), cx)
            })
            .await
            .unwrap();

        // The fake server reports a symbol with the same name as the indexed one
        // to prove only one of the two sources survives the merge.
        let fake_symbols = [symbol("alpha_function", path!("/dir/test.rs"))];
        let fake_server = fake_servers.next().await.unwrap();
        fake_server.set_request_handler::<lsp::WorkspaceSymbolRequest, _, _>(
            move |params: lsp::WorkspaceSymbolParams, cx| {
                let executor = cx.background_executor().clone();
                let fake_symbols = fake_symbols.clone();
                async move {
                    let candidates = fake_symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| StringMatchCandidate::new(id, &symbol.name))
                        .collect::<Vec<_>>();
                    let matches = fuzzy::match_strings(
                        &candidates,
                        &params.query,
                        true,
                        true,
                        100,
                        &Default::default(),
                        executor,
                    )
                    .await;

                    Ok(Some(lsp::WorkspaceSymbolResponse::Flat(
                        matches
                            .into_iter()
                            .map(|mat| fake_symbols[mat.candidate_id].clone())
                            .collect(),
                    )))
                }
            },
        );

        // Wait for the fake server to finish starting up so it is registered
        // as a running language server.
        cx.run_until_parked();
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let symbols = cx.new_window_entity(|window, cx| {
            Picker::uniform_list(
                ProjectSymbolsDelegate::new(workspace.downgrade(), project.clone()),
                window,
                cx,
            )
        });

        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("alpha".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            let delegate = &symbols.delegate;
            assert_eq!(delegate.matches.len(), 1);
            assert!(matches!(
                delegate.sources[delegate.matches[0].candidate_id],
                SymbolSource::Lsp
            ));
        });
    }

    #[gpui::test]
    async fn test_project_symbols_force_tree_sitter(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "test.rs": r#"
fn alpha_function() {}
"#,
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_language()));
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

        project.update(cx, |project, cx| {
            project.symbol_index(cx);
        });
        cx.run_until_parked();

        let _buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/dir/test.rs"), cx)
            })
            .await
            .unwrap();

        let fake_symbols = [symbol("lsp_only_symbol", path!("/dir/test.rs"))];
        let fake_server = fake_servers.next().await.unwrap();
        fake_server.set_request_handler::<lsp::WorkspaceSymbolRequest, _, _>(
            move |params: lsp::WorkspaceSymbolParams, cx| {
                let executor = cx.background_executor().clone();
                let fake_symbols = fake_symbols.clone();
                async move {
                    let candidates = fake_symbols
                        .iter()
                        .enumerate()
                        .map(|(id, symbol)| StringMatchCandidate::new(id, &symbol.name))
                        .collect::<Vec<_>>();
                    let matches = fuzzy::match_strings(
                        &candidates,
                        &params.query,
                        true,
                        true,
                        100,
                        &Default::default(),
                        executor,
                    )
                    .await;

                    Ok(Some(lsp::WorkspaceSymbolResponse::Flat(
                        matches
                            .into_iter()
                            .map(|mat| fake_symbols[mat.candidate_id].clone())
                            .collect(),
                    )))
                }
            },
        );

        // Force the tree-sitter symbol index for Rust, hiding the LSP symbols.
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.project_symbols =
                    Some(settings::ProjectSymbols::TreeSitter);
            });
        });

        let project = project.clone();
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let symbols = cx.new_window_entity(|window, cx| {
            Picker::uniform_list(
                ProjectSymbolsDelegate::new(workspace.downgrade(), project.clone()),
                window,
                cx,
            )
        });

        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("alpha".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            let delegate = &symbols.delegate;
            assert_eq!(delegate.matches.len(), 1);
            assert_eq!(delegate.matches[0].string, "alpha_function");
            assert!(matches!(
                delegate.sources[delegate.matches[0].candidate_id],
                SymbolSource::TreeSitter { .. }
            ));
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
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