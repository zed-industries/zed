mod actions;
mod files;
mod providers;
mod symbols;

use std::sync::Arc;

use actions::ActionProvider;
use editor::{Bias, Editor, SelectionEffects, scroll::Autoscroll};
use files::FileProvider;
use fuzzy::StringMatch;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, SharedString, Styled, Task, WeakEntity, Window, actions, rems,
};
use language::ToPoint;
use picker::{Picker, PickerDelegate};
use providers::{SearchResult, SearchResultCategory};
use symbols::SymbolProvider;
use ui::{
    Divider, IconName, KeyBinding, ListItem, ListItemSpacing, ToggleButtonGroup,
    ToggleButtonGroupStyle, ToggleButtonSimple, prelude::*,
};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(
    search_everywhere,
    [
        /// Toggle the Search Everywhere modal.
        Toggle,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(SearchEverywhere::register).detach();
}

impl ModalView for SearchEverywhere {}

pub struct SearchEverywhere {
    picker: Entity<Picker<SearchEverywhereDelegate>>,
    active_tab: SearchTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchTab {
    #[default]
    All,
    Files,
    Symbols,
    Actions,
}

impl SearchEverywhere {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let Some(previous_focus_handle) = window.focused(cx) else {
            return;
        };

        let project = workspace.project().clone();
        let weak_workspace = cx.entity().downgrade();

        workspace.toggle_modal(window, cx, move |window, cx| {
            SearchEverywhere::new(weak_workspace, project, previous_focus_handle, window, cx)
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<project::Project>,
        previous_focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let weak_search_everywhere = cx.entity().downgrade();

        let delegate = SearchEverywhereDelegate::new(
            weak_search_everywhere,
            workspace,
            project,
            previous_focus_handle,
            window,
            cx,
        );

        // Only start indexing if we don't have cached symbols already
        if !delegate.symbol_provider.has_cached_symbols() {
            delegate.symbol_provider.start_indexing(cx);
        }

        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        Self {
            picker,
            active_tab: SearchTab::All,
        }
    }

    fn set_active_tab(&mut self, tab: SearchTab, window: &mut Window, cx: &mut Context<Self>) {
        self.active_tab = tab;
        self.picker.update(cx, |picker, cx| {
            picker.delegate.active_tab = tab;
            picker.refresh(window, cx);
        });
        cx.notify();
    }
}

impl EventEmitter<DismissEvent> for SearchEverywhere {}

impl Focusable for SearchEverywhere {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for SearchEverywhere {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("SearchEverywhere")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

pub struct SearchEverywhereDelegate {
    search_everywhere: WeakEntity<SearchEverywhere>,
    workspace: WeakEntity<Workspace>,
    project: Entity<project::Project>,
    previous_focus_handle: FocusHandle,
    active_tab: SearchTab,
    matches: Vec<SearchResultMatch>,
    selected_ix: usize,
    is_loading: bool,
    file_provider: FileProvider,
    action_provider: ActionProvider,
    symbol_provider: SymbolProvider,
}

struct SearchResultMatch {
    result: SearchResult,
    string_match: StringMatch,
}

impl SearchEverywhereDelegate {
    fn new(
        search_everywhere: WeakEntity<SearchEverywhere>,
        workspace: WeakEntity<Workspace>,
        project: Entity<project::Project>,
        previous_focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<SearchEverywhere>,
    ) -> Self {
        let file_provider = FileProvider::new(project.clone(), cx);
        let action_provider = ActionProvider::new(window, cx);
        let symbol_provider = SymbolProvider::new(project.clone());

        Self {
            search_everywhere,
            workspace,
            project,
            previous_focus_handle,
            active_tab: SearchTab::All,
            matches: Vec::new(),
            selected_ix: 0,
            is_loading: false,
            file_provider,
            action_provider,
            symbol_provider,
        }
    }

    fn filter_matches_by_tab(&self, matches: Vec<SearchResultMatch>) -> Vec<SearchResultMatch> {
        match self.active_tab {
            SearchTab::All => matches,
            SearchTab::Files => matches
                .into_iter()
                .filter(|m| m.result.category == SearchResultCategory::File)
                .collect(),
            SearchTab::Symbols => matches
                .into_iter()
                .filter(|m| m.result.category == SearchResultCategory::Symbol)
                .collect(),
            SearchTab::Actions => matches
                .into_iter()
                .filter(|m| m.result.category == SearchResultCategory::Action)
                .collect(),
        }
    }
}

impl PickerDelegate for SearchEverywhereDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_ix = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search everywhere...".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        if self.symbol_provider.is_indexing() {
            let progress = self.symbol_provider.indexing_progress_percent();
            Some(SharedString::from(format!(
                "Indexing project... {}%",
                progress
            )))
        } else if self.is_loading {
            Some("Searching...".into())
        } else {
            Some("No matches".into())
        }
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let selected_index = match self.active_tab {
            SearchTab::All => 0,
            SearchTab::Files => 1,
            SearchTab::Symbols => 2,
            SearchTab::Actions => 3,
        };

        let search_everywhere = self.search_everywhere.clone();

        v_flex()
            .child(
                h_flex().px_2().py_1p5().child(
                    ToggleButtonGroup::single_row(
                        "search-everywhere-tabs",
                        [
                            ToggleButtonSimple::new("All", {
                                let search_everywhere = search_everywhere.clone();
                                move |_event, window, cx| {
                                    search_everywhere
                                        .update(cx, |this, cx| {
                                            this.set_active_tab(SearchTab::All, window, cx);
                                        })
                                        .log_err();
                                }
                            }),
                            ToggleButtonSimple::new("Files", {
                                let search_everywhere = search_everywhere.clone();
                                move |_event, window, cx| {
                                    search_everywhere
                                        .update(cx, |this, cx| {
                                            this.set_active_tab(SearchTab::Files, window, cx);
                                        })
                                        .log_err();
                                }
                            }),
                            ToggleButtonSimple::new("Symbols", {
                                let search_everywhere = search_everywhere.clone();
                                move |_event, window, cx| {
                                    search_everywhere
                                        .update(cx, |this, cx| {
                                            this.set_active_tab(SearchTab::Symbols, window, cx);
                                        })
                                        .log_err();
                                }
                            }),
                            ToggleButtonSimple::new("Actions", {
                                move |_event, window, cx| {
                                    search_everywhere
                                        .update(cx, |this, cx| {
                                            this.set_active_tab(SearchTab::Actions, window, cx);
                                        })
                                        .log_err();
                                }
                            }),
                        ],
                    )
                    .style(ToggleButtonGroupStyle::Outlined)
                    .selected_index(selected_index),
                ),
            )
            .child(
                h_flex()
                    .overflow_hidden()
                    .flex_none()
                    .h_9()
                    .px_2p5()
                    .child(editor.clone()),
            )
            .child(Divider::horizontal())
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.is_loading = true;
        cx.notify();

        let file_results = self.file_provider.search(&query, cx);
        let action_results = self.action_provider.search(&query, window, cx);
        let symbol_results = self.symbol_provider.search(&query, cx);

        cx.spawn_in(window, async move |picker, cx| {
            let (file_matches, action_matches, symbol_matches) =
                futures::join!(file_results, action_results, symbol_results);

            let mut all_matches: Vec<SearchResultMatch> = Vec::new();

            for (result, string_match) in file_matches {
                all_matches.push(SearchResultMatch {
                    result,
                    string_match,
                });
            }

            for (result, string_match) in action_matches {
                all_matches.push(SearchResultMatch {
                    result,
                    string_match,
                });
            }

            for (result, string_match) in symbol_matches {
                all_matches.push(SearchResultMatch {
                    result,
                    string_match,
                });
            }

            all_matches.sort_by(|a, b| {
                b.string_match
                    .score
                    .partial_cmp(&a.string_match.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            picker
                .update(cx, |picker, cx| {
                    let filtered = picker.delegate.filter_matches_by_tab(all_matches);
                    picker.delegate.matches = filtered;
                    picker.delegate.selected_ix = 0;
                    picker.delegate.is_loading = false;
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.matches.is_empty() {
            self.dismissed(window, cx);
            return;
        }

        let selected_ix = self.selected_ix;
        if selected_ix >= self.matches.len() {
            self.dismissed(window, cx);
            return;
        }

        let selected_match = &self.matches[selected_ix];

        match selected_match.result.category {
            SearchResultCategory::File => {
                if let Some(path) = &selected_match.result.path {
                    let path = path.clone();
                    if let Some(workspace) = self.workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            workspace
                                .open_path(path, None, true, window, cx)
                                .detach_and_log_err(cx);
                        });
                    }
                    window.focus(&self.previous_focus_handle);
                    self.search_everywhere
                        .update(cx, |_, cx| cx.emit(DismissEvent))
                        .log_err();
                }
            }
            SearchResultCategory::Action => {
                if let Some(action) = &selected_match.result.action {
                    let action = action.boxed_clone();
                    window.focus(&self.previous_focus_handle);
                    self.search_everywhere
                        .update(cx, |_, cx| cx.emit(DismissEvent))
                        .log_err();
                    window.dispatch_action(action, cx);
                }
            }
            SearchResultCategory::Symbol => {
                if let Some(symbol) = &selected_match.result.symbol {
                    let symbol = symbol.clone();
                    let project = self.project.clone();
                    let workspace = self.workspace.clone();
                    let search_everywhere = self.search_everywhere.clone();

                    let buffer = project.update(cx, |project, cx| {
                        project.open_buffer_for_symbol(&symbol, cx)
                    });

                    cx.spawn_in(window, async move |_, cx| {
                        let buffer = buffer.await?;
                        workspace.update_in(cx, |workspace, window, cx| {
                            let position = buffer
                                .read(cx)
                                .clip_point_utf16(symbol.range.start, Bias::Left);

                            let editor = workspace.open_project_item::<Editor>(
                                workspace.active_pane().clone(),
                                buffer,
                                true,
                                true,
                                true,
                                true,
                                window,
                                cx,
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
                        search_everywhere
                            .update(cx, |_, cx| cx.emit(DismissEvent))
                            .log_err();
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                } else if let Some(doc_symbol) = &selected_match.result.document_symbol {
                    let buffer = doc_symbol.buffer.clone();
                    let range = doc_symbol.range.clone();
                    let workspace = self.workspace.clone();
                    let search_everywhere = self.search_everywhere.clone();

                    if let Some(workspace) = workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            // Convert anchor to point using buffer snapshot
                            let point = {
                                let buffer_snapshot = buffer.read(cx).snapshot();
                                range.start.to_point(&buffer_snapshot)
                            };

                            let editor = workspace.open_project_item::<Editor>(
                                workspace.active_pane().clone(),
                                buffer,
                                true,
                                true,
                                true,
                                true,
                                window,
                                cx,
                            );

                            editor.update(cx, |editor, cx| {
                                editor.change_selections(
                                    SelectionEffects::scroll(Autoscroll::center()),
                                    window,
                                    cx,
                                    |s| s.select_ranges([point..point]),
                                );
                            });
                        });
                    }

                    search_everywhere
                        .update(cx, |_, cx| cx.emit(DismissEvent))
                        .log_err();
                }
            }
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        window.focus(&self.previous_focus_handle);
        self.search_everywhere
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let search_match = self.matches.get(ix)?;
        let result = &search_match.result;

        let icon = match result.category {
            SearchResultCategory::File => IconName::File,
            SearchResultCategory::Symbol => IconName::Code,
            SearchResultCategory::Action => IconName::BoltOutlined,
        };

        let keybinding = result
            .action
            .as_ref()
            .map(|action| KeyBinding::for_action_in(&**action, &self.previous_focus_handle, cx));

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot(Icon::new(icon).color(Color::Muted))
                .child(
                    h_flex()
                        .gap_2()
                        .child(Label::new(result.label.clone()))
                        .when_some(result.detail.clone(), |this, detail| {
                            this.child(
                                Label::new(detail)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        }),
                )
                .end_slot(
                    h_flex()
                        .gap_2()
                        .when_some(keybinding, |this, kb| this.child(kb)),
                ),
        )
    }
}
