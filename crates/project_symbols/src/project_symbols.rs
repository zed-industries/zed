use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use editor::{Bias, Editor, SelectionEffects, scroll::Autoscroll};
use gpui::{
    App, Context, DismissEvent, Entity, HighlightStyle, ParentElement, StyledText, Task, TaskExt,
    TextStyle, WeakEntity, Window, combine_highlights, relative,
};
use language::{Point, SymbolKind};
use picker::{Picker, PickerDelegate, PreviewUpdate};
use project::{Project, ProjectPath};
use settings::Settings;
use symbol_index::SymbolSearchResult;
use theme::ActiveTheme;
use theme_settings::ThemeSettings;
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
    matches: Vec<SymbolSearchResult>,
    show_worktree_root_name: bool,
    cancel_flag: Option<Arc<AtomicBool>>,
}

impl ProjectSymbolsDelegate {
    fn new(workspace: WeakEntity<Workspace>, project: Entity<Project>) -> Self {
        Self {
            workspace,
            project,
            selected_match_index: 0,
            matches: Vec::new(),
            show_worktree_root_name: false,
            cancel_flag: None,
        }
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
        let Some(result) = self.matches.get(self.selected_match_index).cloned() else {
            return;
        };

        let Some(project_path) = ProjectPath::from_worktree_and_path(
            result.symbol.location.worktree_id,
            &result.symbol.location.path,
        ) else {
            return;
        };

        let buffer = self
            .project
            .update(cx, |project, cx| project.open_buffer(project_path, cx));

        let row = result.symbol.row;
        let column = result.symbol.column;
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |_, cx| {
            let buffer = buffer.await?;
            workspace.update_in(cx, |workspace, window, cx| {
                let position = buffer.read(cx).clip_point(
                    Point::new(row, column),
                    Bias::Left,
                );
                let pane = if secondary {
                    workspace.adjacent_pane(window, cx)
                } else {
                    workspace.active_pane().clone()
                };

                let editor = workspace.open_project_item::<Editor>(
                    pane, buffer, true, true, true, true, window, cx,
                );

                editor.update(cx, |editor, cx| {
                    let multibuffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                    let Some(buffer_snapshot) = multibuffer_snapshot.as_singleton() else {
                        return;
                    };
                    let text_anchor = buffer_snapshot.anchor_before(position);
                    let Some(anchor) = multibuffer_snapshot.anchor_in_buffer(text_anchor) else {
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
        let result = self.matches.get(self.selected_match_index)?;
        let project_path = ProjectPath::from_worktree_and_path(
            result.symbol.location.worktree_id,
            &result.symbol.location.path,
        )?;
        let worktree = self
            .project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)?;
        let abs_path = worktree.read(cx).absolutize(&project_path.path);
        let position = Point::new(result.symbol.row, result.symbol.column);
        Some(PreviewUpdate::from_path_with_position(abs_path, position))
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        // Cancel previous search
        if let Some(flag) = self.cancel_flag.take() {
            flag.store(true, Ordering::Relaxed);
        }

        let cancel_flag = Arc::new(AtomicBool::new(false));
        self.cancel_flag = Some(cancel_flag.clone());

        // Support rust-analyzer's path based symbols feature which
        // allows to search by rust path syntax, in that case we only want
        // to filter names by the last segment
        let query_filter = query
            .rsplit_once("::")
            .map_or(&*query, |(_, suffix)| suffix)
            .to_owned();

        self.show_worktree_root_name = self.project.read(cx).visible_worktrees(cx).count() > 1;

        let snapshot = self.project.update(cx, |project, cx| {
            project
                .symbol_index(cx)
                .update(cx, |manager, _cx| manager.snapshot())
        });

        let executor = cx.background_executor().clone();

        cx.spawn(async move |this, cx| {
            let results = snapshot
                .search(&query_filter, 200, cancel_flag.clone(), executor)
                .await;

            // Guard against cancelled searches overwriting newer results.
            // match_strings_async returns an empty Vec when cancelled, so a
            // cancelled search that completes after a newer one would clobber
            // the correct results.
            if cancel_flag.load(Ordering::Relaxed) {
                return;
            }

            this.update(cx, |this, cx| {
                this.delegate.matches = results;
                this.delegate.selected_match_index = 0;
                cx.notify();
            })
            .ok();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let result = self.matches.get(ix)?;
        let path_style = self.project.read(cx).path_style(cx);

        let project_path = ProjectPath::from_worktree_and_path(
            result.symbol.location.worktree_id,
            &result.symbol.location.path,
        )?;

        let path: Arc<str> = {
            let project = self.project.read(cx);
            let mut path = project_path.path.to_rel_path_buf();
            if self.show_worktree_root_name
                && let Some(worktree) = project.worktree_for_id(project_path.worktree_id, cx)
            {
                path = worktree.read(cx).root_name().join(&path);
            }
            path.display(path_style).into_owned().into()
        };

        let symbol = &result.symbol;
        let display_text = if symbol.context.is_empty() {
            symbol.name.to_string()
        } else {
            format!("{} {}", symbol.context, symbol.name)
        };
        let label = display_text.as_str();
        let line_number = symbol.row + 1;

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

        // Build syntax highlight runs: context in keyword color, name in kind color.
        let syntax_theme = cx.theme().syntax();
        let name_start = if symbol.context.is_empty() {
            0
        } else {
            symbol.context.len() + 1
        };
        let name_end = name_start + symbol.name.len();

        let mut syntax_runs: Vec<(std::ops::Range<usize>, HighlightStyle)> = Vec::new();

        // Context portion (e.g., "fn ", "struct ") styled as keyword.
        if name_start > 0 {
            if let Some(style) = syntax_theme.style_for_name("keyword") {
                syntax_runs.push((0..name_start, style));
            }
        }

        // Name portion styled by symbol kind.
        if let Some(highlight_name) = symbol_kind_highlight_name(symbol.kind)
            && let Some(style) = syntax_theme.style_for_name(highlight_name)
        {
            syntax_runs.push((name_start..name_end, style));
        }

        // Fuzzy match highlight: positions are relative to the candidate string
        // (which is the symbol name), so offset them by name_start.
        let fuzzy_highlight = HighlightStyle {
            background_color: Some(cx.theme().colors().text_accent.alpha(0.3)),
            ..Default::default()
        };
        let fuzzy_highlights = result.positions.iter().map(|pos| {
            let offset_pos = pos + name_start;
            (offset_pos..label.ceil_char_boundary(offset_pos + 1), fuzzy_highlight)
        });

        let highlights = combine_highlights(fuzzy_highlights, syntax_runs.iter().cloned());

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .child(
                            LabelLike::new().child(
                                StyledText::new(label)
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
    use gpui::TestAppContext;
    use language::{Language, LanguageConfig, LanguageMatcher};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::sync::Arc;
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
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(RUST_OUTLINE_QUERY)
        .unwrap()
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

    #[gpui::test]
    async fn test_project_symbols_with_tree_sitter(cx: &mut TestAppContext) {
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

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_language()));

        // Initialize symbol index and wait for indexing to complete
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

        // Search for "alpha"
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("alpha".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            let delegate = &symbols.delegate;
            assert_eq!(delegate.matches.len(), 1);
            assert_eq!(delegate.matches[0].symbol.name.as_ref(), "alpha_function");
        });

        // Search for "function" — should match both functions
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("function".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            let delegate = &symbols.delegate;
            assert_eq!(delegate.matches.len(), 2);
        });

        // Empty query returns nothing
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            assert_eq!(symbols.delegate.matches.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_project_symbols_struct_and_enum(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "test.rs": r#"
struct MyStruct { x: i32 }
enum MyEnum { A, B }
fn my_function() {}
"#,
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_language()));

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

        // Search for "My" — should match struct, enum, and function (case-insensitive)
        symbols.update_in(cx, |p, window, cx| {
            p.update_matches("My".to_string(), window, cx);
        });

        cx.run_until_parked();
        symbols.read_with(cx, |symbols, _| {
            let delegate = &symbols.delegate;
            let names: Vec<&str> = delegate
                .matches
                .iter()
                .map(|m| m.symbol.name.as_ref())
                .collect();
            assert!(names.contains(&"MyStruct"));
            assert!(names.contains(&"MyEnum"));
            assert!(names.contains(&"my_function"));
        });
    }
}
