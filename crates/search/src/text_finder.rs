use std::{ops::Range, sync::atomic::Ordering};

use db::{
    query,
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use gpui::{
    App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Modifiers, Subscription, Task, WeakEntity, actions,
};
use language::Buffer;
use picker::Picker;

use project::ProjectPath;
use settings::SeedQuerySetting;
use text::Anchor;
use ui::Window;
use workspace::{DismissDecision, ItemHandle, ModalView, Workspace, WorkspaceDb, WorkspaceId};

mod delegate;
mod render;
use delegate::{Delegate, matches_to_multibuffer};
use util::ResultExt as _;

use crate::{ProjectSearchView, SearchOptions, text_finder::delegate::PopulateProjectSearch};

actions!(text_finder, [ToProjectSearch, Fold, Unfold, ToggleFoldAll]);

pub struct TextFinder {
    picker: Entity<Picker<Delegate>>,
    init_modifiers: Option<Modifiers>,
    workspace_id: Option<WorkspaceId>,
    _subscription: Subscription,
}

/// Persists the query and active filters of the just-closed Text Finder in the per-project
/// database, keyed by workspace so the search is restored only for the project it was run in
/// (mirrors JetBrains' per-project find history). The row is removed automatically when its
/// workspace is deleted, via the `ON DELETE CASCADE` foreign key. Workspaces without a database
/// id (not yet persisted) don't participate.
pub struct TextFinderDb(ThreadSafeConnection);

impl Domain for TextFinderDb {
    const NAME: &str = stringify!(TextFinderDb);

    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE text_finder_queries (
            workspace_id INTEGER PRIMARY KEY,
            query TEXT NOT NULL,
            search_options INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE
        ) STRICT;
    )];
}

db::static_connection!(TextFinderDb, [WorkspaceDb]);

impl TextFinderDb {
    query! {
        pub async fn set_last_search(workspace_id: WorkspaceId, query: String, search_options: i64) -> Result<()> {
            INSERT INTO text_finder_queries (workspace_id, query, search_options)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(workspace_id) DO UPDATE SET query = ?2, search_options = ?3
        }
    }

    query! {
        pub fn last_search(workspace_id: WorkspaceId) -> Result<Option<(String, i64)>> {
            SELECT query, search_options
            FROM text_finder_queries
            WHERE workspace_id = ?1
        }
    }
}

/// A query to pre-populate the Text Finder with, plus the search filters to restore alongside
/// it. `options` carries the workspace's last-used filters when there are any persisted; it is
/// `None` only the first time the finder is used in a workspace, leaving the filters at their
/// setting-derived defaults.
pub(crate) struct SearchSeed {
    query: String,
    options: Option<SearchOptions>,
}

fn store_last_search(
    workspace_id: Option<WorkspaceId>,
    query: String,
    options: SearchOptions,
    cx: &App,
) {
    let Some(workspace_id) = workspace_id else {
        return;
    };
    let db = TextFinderDb::global(cx);
    let search_options = options.bits() as i64;
    db::write_and_log(cx, move || async move {
        db.set_last_search(workspace_id, query, search_options)
            .await
    });
}

fn load_last_search(workspace_id: Option<WorkspaceId>, cx: &App) -> Option<SearchSeed> {
    let (query, search_options) = TextFinderDb::global(cx)
        .last_search(workspace_id?)
        .log_err()
        .flatten()?;
    if query.is_empty() {
        return None;
    }
    Some(SearchSeed {
        query,
        options: Some(SearchOptions::from_bits_truncate(search_options as u8)),
    })
}

pub fn init(cx: &mut App) {
    cx.observe_new(TextFinder::register).detach();
}

impl TextFinder {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        pub use zed_actions::text_finder::Toggle;
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            let Some(text_picker) = workspace.active_modal::<Self>(cx) else {
                let seed_query = Self::seed_query(workspace, window, cx);
                Self::open(seed_query, window, cx).detach();
                return;
            };

            text_picker.update(cx, |text_picker, cx| {
                text_picker.init_modifiers = Some(window.modifiers());
                text_picker.picker.update(cx, |picker, cx| {
                    picker.cycle_selection(window, cx);
                });
            })
        });
    }

    pub fn open_from_project_search<T: 'static>(
        project_search_view: Entity<ProjectSearchView>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Task<()> {
        let project_search_item_id = project_search_view.entity_id();
        cx.spawn_in(window, async move |_, cx| {
            let workspace =
                project_search_view.read_with(cx, |view, _| WeakEntity::clone(&view.workspace));
            let delegate = Delegate::new_from_project_search(project_search_view, cx).await;
            workspace
                .update_in(cx, |workspace, window, cx| {
                    remove_project_search_tab(project_search_item_id, workspace, window, cx);
                    let workspace_id = workspace.database_id();
                    workspace.toggle_modal(window, cx, |window, cx| {
                        Self::new(delegate, None, workspace_id, window, cx)
                    });
                })
                .ok();
        })
    }

    /// Transition this text finder into a project search tab, carrying over the
    /// current results (and any in-progress search stream) instead of re-running
    /// the search. Inverse of [`Self::open_from_project_search`].
    fn to_project_search(
        &mut self,
        _: &ToProjectSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let picker = Entity::clone(&self.picker);
        let workspace = self.weak_workspace(cx);

        let connected_task = self.take_search_task(cx);
        let project_search_view = self.project_search_view(cx);
        let query = picker.read(cx).delegate.active_query.clone();
        let search_options = picker.read(cx).delegate.search_options;

        cx.spawn_in(window, async move |this, cx| {
            let search_stream = connected_task.unwrap_or(gpui::Task::ready(None)).await;
            let matches =
                picker.update(cx, |picker, _| std::mem::take(&mut picker.delegate.matches));

            project_search_view
                .update_in(cx, |view, window, cx| {
                    view.adopt_text_finder_state(search_options, query, window, cx);
                })
                .log_err();

            this.update(cx, |_, cx| cx.emit(DismissEvent)).log_err();
            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.add_item_to_active_pane(
                        Box::new(project_search_view.clone()),
                        None,
                        true, // focus item
                        window,
                        cx,
                    );
                })
                .log_err();

            if let PopulateProjectSearch::SupersededByNewSearch =
                matches_to_multibuffer(&project_search_view, &matches, cx).await
            {
                return;
            }

            if let Some(stream) = search_stream {
                project_search_view.update(cx, |view, cx| {
                    view.entity
                        .update(cx, |search, cx| search.hook_up_ongoing_search(stream, cx));
                });
            }
        })
        .detach();
    }

    fn split_left(
        &mut self,
        _: &workspace::pane::SplitLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_in_split(workspace::SplitDirection::Left, window, cx);
    }

    fn split_right(
        &mut self,
        _: &workspace::pane::SplitRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_in_split(workspace::SplitDirection::Right, window, cx);
    }

    fn split_up(
        &mut self,
        _: &workspace::pane::SplitUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_in_split(workspace::SplitDirection::Up, window, cx);
    }

    fn split_down(
        &mut self,
        _: &workspace::pane::SplitDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_in_split(workspace::SplitDirection::Down, window, cx);
    }

    fn fold(&mut self, _: &Fold, _window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.set_selected_group_collapsed(true, cx);
        });
    }

    fn unfold(&mut self, _: &Unfold, _window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.set_selected_group_collapsed(false, cx);
        });
    }

    fn toggle_fold_all(&mut self, _: &ToggleFoldAll, _window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.toggle_all_collapsed(cx);
        });
    }

    fn open_in_split(
        &mut self,
        direction: workspace::SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.open_in_split(direction, window, cx);
        });
    }

    fn weak_workspace(&self, cx: &App) -> WeakEntity<Workspace> {
        let workspace = WeakEntity::clone(
            &self
                .picker
                .read(cx)
                .delegate
                .project_search_view
                .read(cx)
                .workspace,
        );
        workspace
    }

    fn take_search_task(
        &self,
        cx: &mut App,
    ) -> Option<Task<Option<project::SearchResults<project::search::SearchResult>>>> {
        self.picker
            .read(cx)
            .delegate
            .text_finder_turning_into_project_search
            .store(true, Ordering::Relaxed);
        self.picker
            .update(cx, |p, _| p.delegate.in_progress_search.take_connected())
    }

    /// Guess the query the user probably wants for pre-populating the search input.
    fn seed_query(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<SearchSeed> {
        let last_search = load_last_search(workspace.database_id(), cx);
        let options = last_search.as_ref().and_then(|seed| seed.options);

        let query = Self::active_item_query(workspace, window, cx)
            .or_else(|| last_search.map(|seed| seed.query))?;

        Some(SearchSeed { query, options })
    }

    /// The query to seed from the focused or active item, if any.
    ///
    /// The focused pane's item is consulted before the active center pane's, so invoking the
    /// finder from a dock (e.g. with a selection in the terminal) seeds from that item even when
    /// an editor with its own selection is active in the center — the selection the user made
    /// last is the one next to the focus. When the focused item has nothing to offer (say, a
    /// focused terminal without a selection), the center item is tried so an editor selection
    /// still seeds.
    fn active_item_query(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<String> {
        let focused_item = workspace.focused_pane(window, cx).read(cx).active_item();
        let active_item = workspace
            .active_item(cx)
            .filter(|active| match &focused_item {
                Some(focused) => focused.item_id() != active.item_id(),
                None => true,
            });

        focused_item
            .into_iter()
            .chain(active_item)
            .find_map(|item| Self::item_query(workspace, item.as_ref(), window, cx))
    }

    /// The query to seed from one item, if any.
    ///
    /// Only an explicit selection seeds from the item; the bare word under the cursor is
    /// ignored. Confirming a match jumps to (and places the cursor on) it, so seeding from the
    /// cursor on reopen would clobber the search you were in the middle of, whereas a deliberate
    /// selection (e.g. a double-click) is a clear signal to search for that text.
    fn item_query(
        workspace: &mut Workspace,
        item: &dyn ItemHandle,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<String> {
        if let Some(project_search) = item.downcast::<ProjectSearchView>() {
            let query = project_search.read(cx).search_query_text(cx);
            if !query.is_empty() {
                return Some(query);
            }
        }

        if let Some(query) = crate::project_search::buffer_search_query(workspace, item, cx) {
            return Some(query);
        }

        if let Some(searchable_item) = item.to_searchable_item_handle(cx) {
            let query =
                searchable_item.query_suggestion(Some(SeedQuerySetting::Selection), window, cx);
            if !query.is_empty() {
                return Some(query);
            }
        }

        None
    }

    pub(crate) fn open(
        seed_query: Option<SearchSeed>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |workspace, cx| {
            let Ok(delegate_task) = workspace.update_in(cx, |workspace, window, cx| {
                Delegate::new(workspace, window, cx)
            }) else {
                return;
            };

            let delegate = delegate_task.await;
            workspace
                .update_in(cx, |workspace, window, cx| {
                    let workspace_id = workspace.database_id();
                    workspace.toggle_modal(window, cx, |window, cx| {
                        Self::new(delegate, seed_query, workspace_id, window, cx)
                    });
                })
                .ok();
        })
    }

    fn new(
        delegate: Delegate,
        seed_query: Option<SearchSeed>,
        workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project = delegate.project(cx).clone();
        let preview = picker_preview::editor_preview(project, window, cx);
        let picker = cx.new(|cx| Picker::list_with_preview(delegate, preview, window, cx));
        let picker_weak = picker.downgrade();
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, cx| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
            picker.delegate.hook_up_any_ongoing_search(picker_weak, cx);
            if let Some(seed_query) = seed_query {
                // Restore filters before seeding the query so the initial search runs with them.
                if let Some(options) = seed_query.options {
                    picker.delegate.search_options = options;
                }
                picker.set_query(&seed_query.query, window, cx);
                picker.select_query(window, cx);
            }
        });
        let subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            init_modifiers: window.modifiers().modified().then_some(window.modifiers()),
            workspace_id,
            _subscription: subscription,
        }
    }

    fn project_search_view(&self, cx: &mut App) -> Entity<ProjectSearchView> {
        Entity::clone(&self.picker.read(cx).delegate.project_search_view)
    }
}

fn remove_project_search_tab(
    project_search_item_id: gpui::EntityId,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<'_, Workspace>,
) {
    if let Some(pane) = workspace.pane_for_item_id(project_search_item_id) {
        pane.update(cx, |pane, cx| {
            pane.remove_item(project_search_item_id, false, false, window, cx);
        });
    }
}

impl ModalView for TextFinder {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DismissDecision {
        let picker = self.picker.read(cx);
        let query = picker.query(cx);
        if !query.is_empty() {
            let options = picker.delegate.search_options;
            store_last_search(self.workspace_id, query, options, cx);
        }
        DismissDecision::Dismiss(true)
    }
}

impl EventEmitter<DismissEvent> for TextFinder {}

impl Focusable for TextFinder {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

#[derive(Clone)]
pub struct SearchMatch {
    pub path: ProjectPath,
    pub buffer: Entity<Buffer>,
    pub anchor_range: Range<Anchor>,
    pub range: Range<usize>,
    pub match_start_byte_column: u32,
    pub line_number: u32,
}

#[cfg(test)]
mod tests {
    use gpui::{TestAppContext, VisualTestContext};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use workspace::MultiWorkspace;

    use super::*;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            theme_settings::init(theme::LoadThemes::JustBase, cx);

            editor::init(cx);
            crate::init(cx);
        });
    }

    /// Dismissal can be initiated from inside a workspace update: workspace-level
    /// action handlers (e.g. buffer search's `SearchActionsRegistrar`) call
    /// `Workspace::hide_modal` while the workspace entity is leased, which runs
    /// `on_before_dismiss` synchronously under that lease. Reading the workspace
    /// entity there panics with "cannot read workspace::Workspace while it is
    /// already being updated", so this test dismisses the finder the same way.
    #[gpui::test]
    async fn test_dismiss_from_within_workspace_update(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/dir"), json!({"one.rs": "const ONE: usize = 1;"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);

        // Seed a query: the last-search persistence in `on_before_dismiss` (the
        // code path that read the workspace entity) only runs when the query is
        // non-empty, which is the common case in practice since the finder seeds
        // the previous query on open.
        let seed_query = SearchSeed {
            query: "ONE".to_string(),
            options: None,
        };
        workspace
            .update_in(cx, |_, window, cx| {
                TextFinder::open(Some(seed_query), window, cx)
            })
            .await;

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<TextFinder>(cx).is_some());
        });

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(workspace.hide_modal(window, cx));
        });

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<TextFinder>(cx).is_none());
        });
    }
}
