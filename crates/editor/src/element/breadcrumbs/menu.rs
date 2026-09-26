use super::super::*;
use super::outline::{
    child_outline_indices, flatten_text_for_single_line_display, outline_parents,
    render_outline_item_menu_row, same_symbol_item, sibling_outline_indices,
    top_level_outline_indices,
};
use super::path::{
    BreadcrumbDirectoryEntry, BreadcrumbListingSettings, DirectoryEntryIconSource,
    MAX_BREADCRUMB_MENU_ROWS, MAX_UNARY_DIRECTORY_SKIP_DEPTH, breadcrumb_directory_entries,
    breadcrumb_directory_listing_inputs, directory_child_paths, directory_entry_icon_source,
    single_child_directory,
};
use crate::EditorEvent;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{AsyncApp, DismissEvent, EventEmitter, FocusHandle, Subscription, Task};
use language::OutlineItem;
use postage::stream::Stream as _;
use project::git_store::{GitStore, GitStoreEvent, RepositoryEvent, RepositoryId};
use project::{ProjectPath, WorktreeId};
use settings::SettingsStore;
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use ui::ListItem;
use ui::utils::WithRemSize;
use util::rel_path::RelPath;

/// Ordered so a batch of updates folds with `max`: one update that may have taken the listing
/// with it outranks any number of ordinary changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum ListingPathImpact {
    Ignore,
    Reload,
    Dead,
}

/// One `String` and `CharBag` per entry, so a large directory is that many allocations; built
/// off the foreground with the entries rather than on the main thread after each reload.
fn directory_filter_candidates(entries: &[BreadcrumbDirectoryEntry]) -> Vec<StringMatchCandidate> {
    entries
        .iter()
        .enumerate()
        .map(|(index, entry)| StringMatchCandidate::new(index, entry.name.as_ref()))
        .collect()
}

/// Every symbol in the buffer, keyed by outline index, so a filter reaches a symbol at any
/// depth rather than only the level being browsed. The set depends only on the outline, so it is
/// built once at load, not on the Left/Right that only re-window it.
fn symbol_filter_candidates(items: &[OutlineItem<Anchor>]) -> Vec<StringMatchCandidate> {
    items
        .iter()
        .enumerate()
        .map(|(outline_index, item)| StringMatchCandidate::new(outline_index, item.text.as_ref()))
        .collect()
}

/// A worktree update names the path that changed and never the path it moved to, so renaming or
/// deleting the listed directory - or any ancestor of it - arrives as an update at that path.
/// The shape of the update cannot tell a removal from an ordinary change, so both route to
/// `Dead`, where looking the path up settles it.
pub(super) fn listing_path_impact(updated: &RelPath, listing: &RelPath) -> ListingPathImpact {
    if updated == listing || listing.is_descendant_of(updated) {
        ListingPathImpact::Dead
    } else if updated.parent() == Some(listing) {
        ListingPathImpact::Reload
    } else {
        ListingPathImpact::Ignore
    }
}

#[derive(Clone, Debug)]
pub(crate) enum BreadcrumbListing {
    Directory {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
    },
    Symbols {
        buffer_id: BufferId,
        parent: Option<OutlineItem<Anchor>>,
    },
}

impl PartialEq for BreadcrumbListing {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Directory {
                    worktree_id: a_id,
                    path: a_path,
                },
                Self::Directory {
                    worktree_id: b_id,
                    path: b_path,
                },
            ) => a_id == b_id && a_path == b_path,
            (
                Self::Symbols {
                    buffer_id: a_id,
                    parent: a_parent,
                },
                Self::Symbols {
                    buffer_id: b_id,
                    parent: b_parent,
                },
            ) => {
                a_id == b_id
                    && match (a_parent, b_parent) {
                        (None, None) => true,
                        (Some(a), Some(b)) => same_symbol_item(a, b),
                        _ => false,
                    }
            }
            _ => false,
        }
    }
}

impl Eq for BreadcrumbListing {}
/// Which row a switch lands on. The listing's own rule - the open file, the caret's symbol, the
/// first row - applies when the hinted row is not in the new listing.
#[derive(Clone, Debug)]
enum SelectionHint {
    Initial,
    Path(Arc<RelPath>),
    Symbol(Range<Anchor>),
}

/// What a symbols switch does when the file turns out to have no symbols.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WithoutSymbols {
    /// The drill from the open file's row: its siblings are the rows already on screen.
    Stay,
    /// A segment click: list the file's siblings, or close when they are what is already listed,
    /// as any click on the rows already on screen does.
    ListSiblings,
}

#[derive(Clone, Debug)]
struct SwitchRequest {
    target: BreadcrumbListing,
    active_file_path: Option<Arc<RelPath>>,
    navigated: bool,
    selection: SelectionHint,
    auto_fold: bool,
    without_symbols: WithoutSymbols,
    /// Set when one switch hands over to another, so text typed during the first leg still counts
    /// as typed during the switch.
    query_at_start: Option<String>,
}

impl SwitchRequest {
    fn new(target: BreadcrumbListing, active_file_path: Option<Arc<RelPath>>) -> Self {
        Self {
            target,
            active_file_path,
            navigated: false,
            selection: SelectionHint::Initial,
            auto_fold: false,
            without_symbols: WithoutSymbols::ListSiblings,
            query_at_start: None,
        }
    }
}

/// A switch still resolving. The listing on screen - rows and anchor alike - stays installed until
/// the target's data is in hand, and then everything moves in one update: moving the anchor first
/// hangs the old rows under the new segment, and blanking them flashes "Loading…".
struct PendingSwitch {
    generation: u64,
    target: BreadcrumbListing,
    query_at_start: String,
    /// Left presses on a symbol level whose outline is still being fetched; the level above it is
    /// only known once the outline lands.
    step_outs: usize,
}

/// What Left or Right did, so the picker knows whether the key was used and whether the query
/// still describes the rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ListingStep {
    Moved,
    Resolving,
    Stayed,
}

enum ResolvedListing {
    Directory {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        entries: Vec<BreadcrumbDirectoryEntry>,
        candidates: Vec<StringMatchCandidate>,
        /// The worktree's scan the entries were listed from.
        scan_id: usize,
    },
    Symbols {
        buffer_id: BufferId,
        parent: Option<OutlineItem<Anchor>>,
        /// `None` keeps the outline the menu already holds for this buffer.
        outline: Option<LoadedOutline>,
    },
}

struct LoadedOutline {
    items: Vec<OutlineItem<Anchor>>,
    depths: Vec<usize>,
    parents: Vec<Option<usize>>,
    candidates: Vec<StringMatchCandidate>,
    cursor_ranges: Vec<Range<Anchor>>,
    /// The buffer's version the outline was fetched at.
    version: Option<clock::Global>,
}

/// Fetches the outline and derives everything the menu reads from it off the main thread.
async fn load_outline(
    menu: WeakEntity<BreadcrumbNavigationMenu>,
    buffer_id: BufferId,
    cx: &mut AsyncApp,
) -> Option<LoadedOutline> {
    let (outline_task, version) = menu
        .update(cx, |menu, cx| {
            menu.editor.upgrade().map(|editor| {
                editor.update(cx, |editor, cx| {
                    let version = editor
                        .buffer()
                        .read(cx)
                        .buffer(buffer_id)
                        .map(|buffer| buffer.read(cx).version());
                    (editor.buffer_outline_items(buffer_id, cx), version)
                })
            })
        })
        .ok()
        .flatten()?;
    let text_items = outline_task.await;
    let (snapshot, cursor_ranges) = menu
        .update(cx, |menu, cx| {
            let editor = menu.editor.upgrade()?;
            let editor = editor.read(cx);
            let cursor_ranges = editor
                .outline_symbols_at_cursor
                .as_ref()
                .filter(|(id, _)| *id == buffer_id)
                .map(|(_, ancestors)| ancestors.iter().map(|item| item.range.clone()).collect())
                .unwrap_or_default();
            Some((editor.buffer().read(cx).snapshot(cx), cursor_ranges))
        })
        .ok()
        .flatten()?;
    Some(
        cx.background_spawn(async move {
            let items =
                crate::document_symbols::text_outline_items_to_multibuffer(&text_items, &snapshot);
            let depths: Vec<usize> = items.iter().map(|item| item.depth).collect();
            let parents = outline_parents(&depths);
            let candidates = symbol_filter_candidates(&items);
            LoadedOutline {
                items,
                depths,
                parents,
                candidates,
                cursor_ranges,
                version,
            }
        })
        .await,
    )
}

/// The open file's siblings: its parent directory, and the file's own path within it. `None`
/// where there is no directory to list, a single-file worktree or an untitled buffer.
pub(crate) fn file_parent_directory(
    editor: &Editor,
    cx: &App,
) -> Option<(WorktreeId, Arc<RelPath>, Arc<RelPath>)> {
    let project_path = editor.active_project_path(cx)?;
    // A single-file worktree paints no directory segments, so a directory listing there would
    // have nothing to anchor to.
    let is_single_file = editor
        .project()
        .and_then(|project| {
            project
                .read(cx)
                .worktree_for_id(project_path.worktree_id, cx)
        })
        .is_some_and(|worktree| worktree.read(cx).is_single_file());
    if is_single_file {
        return None;
    }
    let parent_path = project_path
        .path
        .parent()
        .map(|parent| parent.into_arc())
        .unwrap_or_else(|| RelPath::empty().into_arc());
    Some((project_path.worktree_id, parent_path, project_path.path))
}

pub(crate) struct BreadcrumbNavigationMenu {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    listing: BreadcrumbListing,
    navigated_path: Option<(WorktreeId, Arc<RelPath>)>,
    symbol_trail: Vec<OutlineItem<Anchor>>,
    active_file_path: Option<Arc<RelPath>>,
    directory_entries: Arc<Vec<BreadcrumbDirectoryEntry>>,
    all_symbol_items: Arc<Vec<OutlineItem<Anchor>>>,
    /// Derived once per outline: Left, Right, publishing and the toggle check all read them, and
    /// publishing runs per keystroke.
    symbol_depths: Vec<usize>,
    symbol_parents: Vec<Option<usize>>,
    listed_symbol_indices: Vec<usize>,
    cursor_symbol_ranges: Vec<Range<Anchor>>,
    loading: bool,
    load_epoch: u64,
    load_task: Option<Task<()>>,
    pending_switch: Option<PendingSwitch>,
    switch_generation: u64,
    switch_task: Option<Task<()>>,
    row_refresh_task: Option<Task<()>>,
    refresh_in_flight: bool,
    refresh_queued: bool,
    selected_index: Option<usize>,
    pending_initial_selection: bool,
    query: String,
    rows: BreadcrumbMenuRows,
    /// What `rows` were built for; a switch moves `listing` before its first publish.
    rows_listing: Option<BreadcrumbListing>,
    rows_dirty: bool,
    scroll_to_selection_pending: bool,
    picker: Option<Entity<picker::Picker<BreadcrumbPickerDelegate>>>,
    pressed_outside: bool,
    ranked_matches: Vec<StringMatch>,
    filter_task: Option<Task<()>>,
    filter_cancel: Option<Arc<AtomicBool>>,
    filter_epoch: u64,
    ranked_epoch: u64,
    /// The row under the highlight when the query went from empty to non-empty, restored when it
    /// comes back to empty unless the user moved the highlight under the filter meanwhile: typing
    /// to look and then erasing has to leave them where they were, or every abandoned search
    /// dumps them at the top of the list.
    pre_filter_selection: Option<PreFilterSelection>,
    filter_selection_touched: bool,
    /// The row that was arrowed to when a filesystem event forced a reload, restored by path
    /// once the rank that reload triggered lands. Ranked positions do not survive the rebuild.
    pending_restore_path: Option<Arc<RelPath>>,
    /// The symbol that was highlighted when a buffer edit forced a reload, restored by anchor
    /// range once that reload lands. Latched rather than read back off the rows, because the
    /// reload blanks them: a second edit arriving while the first is still in flight would
    /// otherwise find nothing left to restore. Anchors survive edits elsewhere in the buffer,
    /// so the range is what still identifies the row across one.
    pending_restore_symbol_range: Option<Range<Anchor>>,
    /// Held while a rank is in flight; see [`FilterSettled`].
    filter_settled: FilterSettled,
    filter_match_truncated: bool,
    /// Escape alone reaches the delegate's `dismissed`, the picker's own event, and the blur
    /// that follows the teardown; the listeners get exactly one event.
    dismiss_emitted: bool,
    filter_candidates: Arc<Vec<StringMatchCandidate>>,
    last_listing_settings: BreadcrumbListingSettings,
    #[cfg(test)]
    directory_reload_count: usize,
    /// Every published row set with the listing it was published for, in order, so a test can
    /// assert what a switch or a reload put on screen between two settled states.
    #[cfg(test)]
    published_row_history: Vec<Vec<SharedString>>,
    #[cfg(test)]
    published_listing_history: Vec<BreadcrumbListing>,
    #[cfg(test)]
    published_empty_message_history: Vec<SharedString>,
    #[cfg(test)]
    published_match_count_history: Vec<Option<SharedString>>,
    _subscriptions: Vec<Subscription>,
    _buffer_subscription: Option<Subscription>,
}

impl BreadcrumbNavigationMenu {
    pub fn new(
        editor: WeakEntity<Editor>,
        workspace: WeakEntity<Workspace>,
        listing: BreadcrumbListing,
        active_file_path: Option<Arc<RelPath>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let menu = cx.new(|cx| {
            Self::with_listing(
                editor,
                workspace,
                listing.clone(),
                active_file_path.clone(),
                cx,
            )
        });
        menu.update(cx, |this, cx| {
            this.attach_picker(window, cx);
            if let Some(project) = this.project(cx) {
                this._subscriptions
                    .push(cx.subscribe(&project, |this, _, event, cx| {
                        let BreadcrumbListing::Directory {
                            worktree_id: listing_worktree,
                            path: listing_path,
                        } = &this.listing
                        else {
                            return;
                        };
                        let (listing_worktree, listing_path) =
                            (*listing_worktree, listing_path.clone());
                        let impact = match event {
                            project::Event::WorktreeUpdatedEntries(worktree_id, updates)
                                if *worktree_id == listing_worktree =>
                            {
                                updates.iter().fold(
                                    ListingPathImpact::Ignore,
                                    |impact, (path, _, _)| {
                                        impact.max(listing_path_impact(path, &listing_path))
                                    },
                                )
                            }
                            project::Event::WorktreeUpdatedRootRepoCommonDir(worktree_id)
                                if *worktree_id == listing_worktree =>
                            {
                                ListingPathImpact::Reload
                            }
                            project::Event::WorktreeRemoved(worktree_id)
                                if *worktree_id == listing_worktree =>
                            {
                                ListingPathImpact::Dead
                            }
                            _ => ListingPathImpact::Ignore,
                        };
                        match impact {
                            ListingPathImpact::Ignore => {}
                            ListingPathImpact::Reload => this.reload_directory_rows(cx),
                            ListingPathImpact::Dead => {
                                // Metadata on the listing or an ancestor lands here too, so
                                // only a path that is gone - or no longer a directory - takes
                                // the listing with it.
                                let listing_survives =
                                    this.worktree(listing_worktree, cx).is_some_and(|worktree| {
                                        worktree
                                            .read(cx)
                                            .entry_for_path(&listing_path)
                                            .is_some_and(|entry| entry.is_dir())
                                    });
                                if listing_survives {
                                    this.reload_directory_rows(cx);
                                } else {
                                    this.dismiss_dead_listing(cx);
                                }
                            }
                        }
                    }));
            }
            if let Some(project) = this.project(cx) {
                // Directory rows carry a git summary aggregated over the whole subtree, and a
                // change below the immediate children never reaches the worktree subscription
                // above - nor does an index-only change, which touches no path at all.
                let git_store = project.read(cx).git_store().clone();
                this._subscriptions
                    .push(cx.subscribe(&git_store, |this, git_store, event, cx| {
                        let affects_listing = match event {
                            GitStoreEvent::RepositoryUpdated(
                                repository_id,
                                RepositoryEvent::StatusesChanged,
                                _,
                            ) => this.listing_is_in_repository(&git_store, repository_id, cx),
                            GitStoreEvent::DiffBaseChanged(repository_id) => {
                                repository_id.as_ref().is_none_or(|repository_id| {
                                    this.listing_is_in_repository(&git_store, repository_id, cx)
                                })
                            }
                            GitStoreEvent::RepositoryAdded
                            | GitStoreEvent::RepositoryRemoved(_) => true,
                            _ => false,
                        };
                        if affects_listing {
                            this.reload_directory_rows(cx);
                        }
                    }));
            }
            this._subscriptions
                .push(cx.observe_global::<SettingsStore>(|this, cx| {
                    if !matches!(this.listing, BreadcrumbListing::Directory { .. }) {
                        return;
                    }
                    let settings = *BreadcrumbListingSettings::get_global(cx);
                    if settings == this.last_listing_settings {
                        return;
                    }
                    this.last_listing_settings = settings;
                    this.reload_directory_rows(cx);
                }));
            this.start_switch(SwitchRequest::new(listing, active_file_path), window, cx);
            this.focus_menu(window, cx);
        });
        menu
    }

    /// The menu in its loading state: `listing` is what the strip anchors to until the first
    /// switch installs its rows.
    fn with_listing(
        editor: WeakEntity<Editor>,
        workspace: WeakEntity<Workspace>,
        listing: BreadcrumbListing,
        active_file_path: Option<Arc<RelPath>>,
        cx: &App,
    ) -> Self {
        Self {
            editor,
            workspace,
            listing,
            navigated_path: None,
            symbol_trail: Vec::new(),
            active_file_path,
            directory_entries: Arc::default(),
            all_symbol_items: Arc::default(),
            symbol_depths: Vec::new(),
            symbol_parents: Vec::new(),
            listed_symbol_indices: Vec::new(),
            cursor_symbol_ranges: Vec::new(),
            loading: true,
            load_epoch: 0,
            load_task: None,
            pending_switch: None,
            switch_generation: 0,
            switch_task: None,
            row_refresh_task: None,
            refresh_in_flight: false,
            refresh_queued: false,
            selected_index: None,
            pending_initial_selection: true,
            query: String::new(),
            rows: BreadcrumbMenuRows::default(),
            rows_listing: None,
            rows_dirty: false,
            scroll_to_selection_pending: false,
            picker: None,
            pressed_outside: false,
            ranked_matches: Vec::new(),
            filter_task: None,
            filter_cancel: None,
            filter_epoch: 0,
            ranked_epoch: 0,
            pre_filter_selection: None,
            filter_selection_touched: false,
            pending_restore_path: None,
            pending_restore_symbol_range: None,
            filter_settled: FilterSettled::default(),
            filter_match_truncated: false,
            dismiss_emitted: false,
            filter_candidates: Arc::new(Vec::new()),
            last_listing_settings: *BreadcrumbListingSettings::get_global(cx),
            #[cfg(test)]
            directory_reload_count: 0,
            #[cfg(test)]
            published_row_history: Vec::new(),
            #[cfg(test)]
            published_listing_history: Vec::new(),
            #[cfg(test)]
            published_empty_message_history: Vec::new(),
            #[cfg(test)]
            published_match_count_history: Vec::new(),
            _subscriptions: Vec::new(),
            _buffer_subscription: None,
        }
    }

    fn attach_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let delegate =
            BreadcrumbPickerDelegate::new(cx.weak_entity(), Self::placeholder_for(&self.listing));
        let picker = cx.new(|cx| {
            let available = window.viewport_size().width / window.rem_size();
            picker::Picker::uniform_list(delegate, window, cx)
                .popover()
                .show_scrollbar(true)
                .initial_width(rems(available.clamp(10., 24.)))
        });
        let picker_focus = picker.focus_handle(cx);
        self._subscriptions.push(cx.on_blur(&picker_focus, window, {
            |this: &mut Self, _, cx| {
                this.emit_dismiss(cx);
            }
        }));
        self.picker = Some(picker);
    }

    pub fn listing(&self) -> &BreadcrumbListing {
        &self.listing
    }

    pub fn navigated_path(&self) -> Option<(WorktreeId, Arc<RelPath>)> {
        self.navigated_path.clone()
    }

    pub fn symbol_trail(&self) -> &[OutlineItem<Anchor>] {
        &self.symbol_trail
    }

    /// Switches the menu to `listing` once its rows are in hand. `active_file_path` is what the
    /// initial selection is resolved against, so it is refreshed on every switch: the editor can
    /// have been saved elsewhere since the menu was built.
    pub fn set_listing(
        &mut self,
        listing: BreadcrumbListing,
        active_file_path: Option<Arc<RelPath>>,
        navigated: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let request = SwitchRequest {
            navigated,
            ..SwitchRequest::new(listing, active_file_path)
        };
        self.start_switch(request, window, cx);
    }

    #[cfg(test)]
    pub fn symbol_restore_pending(&self) -> bool {
        self.pending_restore_symbol_range.is_some()
    }

    #[cfg(test)]
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    #[cfg(test)]
    pub fn filter(&self) -> String {
        self.filter_query().to_string()
    }

    pub(super) fn rank_pending(&self) -> bool {
        self.ranked_epoch != self.filter_epoch
    }

    /// `None` once the rank has landed and its rows are published, so a caller that gets
    /// nothing back is free to act on what the picker is showing.
    pub(super) fn filter_settled(&self) -> Option<postage::barrier::Receiver> {
        self.filter_settled.receiver()
    }

    #[cfg(test)]
    pub fn entry_names(&self) -> Vec<SharedString> {
        match &self.listing {
            BreadcrumbListing::Directory { .. } => self
                .directory_entries
                .iter()
                .map(|entry| entry.name.clone())
                .collect(),
            BreadcrumbListing::Symbols { .. } => self
                .listed_symbol_indices
                .iter()
                .filter_map(|&index| {
                    self.all_symbol_items
                        .get(index)
                        .map(|item| item.text.clone())
                })
                .collect(),
        }
    }

    #[cfg(test)]
    pub fn filtered_entry_names(&self) -> Vec<SharedString> {
        self.visible_row_labels()
    }

    /// What the picker is rendering, which is what the user can act on. It diverges from the
    /// menu's own state whenever a mutation forgets to publish.
    #[cfg(test)]
    pub fn published_symbol_items(&self, cx: &App) -> Vec<OutlineItem<Anchor>> {
        let Some(picker) = self.picker.as_ref() else {
            return Vec::new();
        };
        let rows = &picker.read(cx).delegate.rows;
        (0..rows.len())
            .filter_map(|index| rows.symbol(index).map(|(item, _)| item.clone()))
            .collect()
    }

    #[cfg(test)]
    pub fn published_icon_flags(&self, cx: &App) -> Option<(bool, bool)> {
        let picker = self.picker.as_ref()?;
        let delegate = &picker.read(cx).delegate;
        Some((delegate.show_file_icons, delegate.show_folder_icons))
    }

    /// Tells "No matches" from rows that were never replaced.
    #[cfg(test)]
    pub fn published_empty_message(&self, cx: &App) -> SharedString {
        self.picker
            .as_ref()
            .map(|picker| picker.read(cx).delegate.empty_message.clone())
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub fn published_row_labels(&self, cx: &App) -> Vec<SharedString> {
        self.picker
            .as_ref()
            .map(|picker| picker.read(cx).delegate.rows.labels())
            .unwrap_or_default()
    }

    /// Pairs each symbol row with the parent name a filter shows beside an out-of-level match.
    #[cfg(test)]
    pub fn published_symbol_contexts(&self, cx: &App) -> Vec<(SharedString, Option<SharedString>)> {
        let Some(picker) = self.picker.as_ref() else {
            return Vec::new();
        };
        let rows = &picker.read(cx).delegate.rows;
        (0..rows.len())
            .filter_map(|index| {
                rows.symbol(index)
                    .map(|(item, row)| (item.text.clone(), row.context.clone()))
            })
            .collect()
    }

    /// Unlike `clear_filter_for_test`, keeps the ranked matches until `rerank_filter` consumes
    /// them - the path a user takes when they backspace a query away.
    #[cfg(test)]
    pub fn set_filter_query_for_test(&mut self, query: &str, cx: &mut Context<Self>) {
        self.set_filter_query(query.to_string(), cx);
    }

    #[cfg(test)]
    pub fn clear_filter_for_test(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.clear_filter(window, cx);
        self.rerank_filter(cx);
    }

    #[cfg(test)]
    pub fn directory_reload_count_for_test(&self) -> usize {
        self.directory_reload_count
    }

    #[cfg(test)]
    pub fn take_published_row_history(&mut self) -> Vec<Vec<SharedString>> {
        std::mem::take(&mut self.published_row_history)
    }

    #[cfg(test)]
    pub fn take_published_listing_history(&mut self) -> Vec<BreadcrumbListing> {
        std::mem::take(&mut self.published_listing_history)
    }

    /// The events that refresh a listing arrive in bursts the test harness cannot time, so a
    /// test fires the refresh the way they would.
    #[cfg(test)]
    pub fn reload_directory_rows_for_test(&mut self, cx: &mut Context<Self>) {
        self.reload_directory_rows(cx);
    }

    #[cfg(test)]
    pub fn take_published_empty_message_history(&mut self) -> Vec<SharedString> {
        std::mem::take(&mut self.published_empty_message_history)
    }

    #[cfg(test)]
    pub fn take_published_match_count_history(&mut self) -> Vec<Option<SharedString>> {
        std::mem::take(&mut self.published_match_count_history)
    }

    /// The tail of a directory refresh, with the `selected_path` a real refresh would have
    /// derived at its start. Split out so a test can land two refreshes inside the window a
    /// rank leaves open, which no amount of parking can time reliably.
    #[cfg(test)]
    pub fn apply_reloaded_selection_for_test(
        &mut self,
        selected_path: Option<Arc<RelPath>>,
        cx: &mut Context<Self>,
    ) {
        self.apply_reloaded_selection(selected_path, cx);
    }

    #[cfg(test)]
    pub fn apply_initial_selection_for_test(&mut self, cx: &mut Context<Self>) {
        self.pending_initial_selection = true;
        self.apply_initial_selection_if_needed(cx);
    }

    #[cfg(test)]
    pub fn new_with_symbols_for_test(
        editor: WeakEntity<Editor>,
        buffer_id: BufferId,
        all_items: Vec<OutlineItem<Anchor>>,
        listed_indices: Vec<usize>,
        cursor_symbol_ranges: Vec<Range<Anchor>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let menu = cx.new(|cx| {
            let mut menu = Self::with_listing(
                editor,
                WeakEntity::new_invalid(),
                BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: None,
                },
                None,
                cx,
            );
            menu.symbol_depths = all_items.iter().map(|item| item.depth).collect();
            menu.symbol_parents = outline_parents(&menu.symbol_depths);
            menu.all_symbol_items = Arc::new(all_items);
            menu.listed_symbol_indices = listed_indices;
            menu.cursor_symbol_ranges = cursor_symbol_ranges;
            menu.loading = false;
            menu
        });
        menu.update(cx, |this, cx| {
            this.attach_picker(window, cx);
            // The real constructors reach this through the listing load; without it a menu
            // built straight from items has no filter candidates and matches nothing.
            this.rebuild_filter_candidates();
            this.publish_rows(cx);
        });
        menu
    }

    fn focus_menu(&self, window: &mut Window, cx: &mut Context<Self>) {
        // Deferred: reached from delegate callbacks that hold the picker's lease.
        let Some(picker) = self.picker.clone() else {
            return;
        };
        cx.defer_in(window, move |_, window, cx| {
            window.focus(&picker.focus_handle(cx), cx);
        });
    }

    /// The picker copies the placeholder into its query editor when the head is built, so a
    /// listing that changes kind has to push the new one through `refresh_placeholder`.
    fn refresh_placeholder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(picker) = self.picker.clone() else {
            return;
        };
        let placeholder = Self::placeholder_for(&self.listing);
        cx.defer_in(window, move |_, window, cx| {
            picker.update(cx, |picker, cx| {
                picker.delegate.placeholder = placeholder;
                picker.refresh_placeholder(window, cx);
            });
        });
    }

    fn placeholder_for(listing: &BreadcrumbListing) -> Arc<str> {
        match listing {
            BreadcrumbListing::Directory { .. } => "Search this directory…".into(),
            BreadcrumbListing::Symbols { .. } => "Search these symbols…".into(),
        }
    }

    fn filter_query(&self) -> &str {
        &self.query
    }

    fn filter_is_empty(&self) -> bool {
        self.query.is_empty()
    }

    pub(super) fn set_filter_query(&mut self, query: String, cx: &mut Context<Self>) {
        if self.query == query {
            return;
        }
        let query_started = self.query.is_empty() && !query.is_empty();
        self.query = query;
        self.pending_restore_path = None;
        self.pending_restore_symbol_range = None;
        if query_started {
            self.pre_filter_selection = self
                .selected_directory_entry()
                .map(|entry| PreFilterSelection::Directory(entry.path))
                .or_else(|| {
                    self.selected_symbol_item()
                        .map(|item| PreFilterSelection::Symbol(item.range))
                });
            self.filter_selection_touched = false;
        }
        if !self.filter_is_empty() {
            self.pending_initial_selection = false;
            // A new query lands on its best match. The index is only kept across a rank the
            // user arrowed through while it was in flight, which sets it again after this.
            self.selected_index = None;
        }
        self.rerank_filter(cx);
        // `selected_index` and the row lookups address the ranked matches the moment the
        // query is non-empty, so the delegate cannot keep serving the old rows.
        self.publish_rows(cx);
        cx.notify();
    }

    fn clear_filter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.clear_filter_shown_as(None, window, cx);
    }

    /// With `shown` set, the query box is emptied only while it still reads that text: anything
    /// typed since is meant for what comes next, and the picker delivers it on its own. An empty
    /// `shown` leaves the box alone, since there is nothing of it to take back.
    fn clear_filter_shown_as(
        &mut self,
        shown: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.query.clear();
        self.ranked_matches.clear();
        self.filter_match_truncated = false;
        self.filter_epoch = self.filter_epoch.wrapping_add(1);
        self.pending_restore_symbol_range = None;
        // Nothing reranks after a listing change - the cleared query short-circuits
        // `set_filter_query` - so leaving the epochs apart would make `rank_pending` true
        // forever and swallow every later drill.
        self.ranked_epoch = self.filter_epoch;
        if let Some(cancel) = self.filter_cancel.take() {
            cancel.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        self.filter_task = None;
        let picker = self.picker.clone();
        // Deferred for the same reason as `publish_rows`: callers reach here with the picker
        // leased. Dropping the rank task drops the settle it owed, and the picker waits on that
        // barrier before it will confirm anything - so republish first, then release it.
        cx.defer_in(window, move |this, window, cx| {
            this.publish_rows_now(cx);
            this.filter_settled.settle();
            if let Some(picker) = picker {
                picker.update(cx, |picker, cx| {
                    if shown
                        .as_ref()
                        .is_none_or(|shown| !shown.is_empty() && picker.query(cx) == *shown)
                    {
                        picker.set_query("", window, cx);
                    }
                });
            }
        });
    }

    pub(super) fn set_selected_row(&mut self, position: usize, cx: &mut Context<Self>) {
        if self.selected_index == Some(position) {
            return;
        }
        self.pending_initial_selection = false;
        if !self.filter_is_empty() {
            self.filter_selection_touched = true;
        }
        self.selected_index = Some(position);
        // Nothing asks for a scroll here: the picker already scrolls for selections it
        // originates, and deliberately does not for hover. Scrolling would drag rows under a
        // resting cursor and retrigger hover on the row that lands beneath it.
        self.publish_selection(cx);
        cx.notify();
    }

    /// By index: the picker selects and confirms in one call, while the menu's copy of the
    /// selection lands a cycle later.
    pub(super) fn confirm_row(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected_index = Some(index);
        self.confirm(&menu::Confirm, window, cx);
    }

    /// Reports whether the listing on screen changed, so a typed query survives a drill that
    /// goes nowhere. A drill that has to resolve first keeps the query: it is cleared with the
    /// rows it described, when the switch installs.
    pub(super) fn drill_into_selection(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ListingStep {
        // Until the rank lands the rows still describe the previous query, and unlike Enter
        // this path has no pending-update contract to wait on.
        if self.rank_pending() {
            return ListingStep::Stayed;
        }
        self.selected_index = Some(index);
        self.step_listing(
            |this, window, cx| this.select_child(&menu::SelectChild, window, cx),
            window,
            cx,
        )
    }

    /// Ungated unlike the drill: the parent comes from the listing, not from rows a rank replaces.
    pub(super) fn step_out_of_listing(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ListingStep {
        self.step_listing(
            |this, window, cx| this.select_parent(&menu::SelectParent, window, cx),
            window,
            cx,
        )
    }

    fn step_listing(
        &mut self,
        step: impl FnOnce(&mut Self, &mut Window, &mut Context<Self>),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ListingStep {
        let listing_before = self.listing.clone();
        let pending_before = self
            .pending_switch
            .as_ref()
            .map(|pending| (pending.generation, pending.step_outs));
        step(self, window, cx);
        let pending_after = self
            .pending_switch
            .as_ref()
            .map(|pending| (pending.generation, pending.step_outs));
        if self.listing != listing_before || self.dismiss_emitted {
            ListingStep::Moved
        } else if pending_after.is_some() && pending_after != pending_before {
            ListingStep::Resolving
        } else {
            ListingStep::Stayed
        }
    }

    /// A selection change on its own. The rows are the objects the delegate already holds, and
    /// rebuilding them clones every one - up to `MAX_BREADCRUMB_MENU_ROWS`, each with its
    /// anchors and highlight ranges - which hovering does once per row it crosses. Deferred for
    /// the same reason as `publish_rows`.
    fn publish_selection(&mut self, cx: &mut Context<Self>) {
        let menu = cx.weak_entity();
        cx.defer(move |cx| {
            menu.update(cx, |this, cx| {
                let Some(picker) = this.picker.clone() else {
                    return;
                };
                let selected_index = this
                    .selected_index
                    .unwrap_or(0)
                    .min(this.rows.len().saturating_sub(1));
                let scroll_to_selection = std::mem::take(&mut this.scroll_to_selection_pending);
                picker.update(cx, |picker, cx| {
                    picker.delegate.selected_index = selected_index;
                    if scroll_to_selection {
                        picker.scroll_to_selected_index();
                    }
                    cx.notify();
                });
            })
            .ok();
        });
    }

    /// Deferred and coalesced: delegate callbacks hold the picker's lease, so publishing inline
    /// would try to update it mid-update.
    pub(super) fn publish_rows(&mut self, cx: &mut Context<Self>) {
        if self.rows_dirty {
            return;
        }
        self.rows_dirty = true;
        let menu = cx.weak_entity();
        cx.defer(move |cx| {
            menu.update(cx, |this, cx| {
                // A direct publish since then already put this state on screen.
                if this.rows_dirty {
                    this.publish_rows_now(cx);
                }
            })
            .ok();
        });
    }

    fn publish_rows_now(&mut self, cx: &mut Context<Self>) {
        self.rows_dirty = false;
        let Some(picker) = self.picker.clone() else {
            return;
        };
        let settings = *BreadcrumbListingSettings::get_global(cx);
        let filter_active = !self.filter_is_empty();
        // The query editor takes the keystroke a hop before the menu hears about it, so until
        // the text and the rank both catch up the rows and the count describe the previous query.
        let visible_query_settled = !filter_active || picker.read(cx).query(cx) == self.query;
        let rank_settled = !filter_active || (!self.rank_pending() && visible_query_settled);
        // Nothing ranked yet for the query on screen: keep the rows the user is reading instead
        // of blanking to "Searching…" and back. Only while the rank is still pending and has a
        // task behind it - a settled rank with no matches has to publish "No matches", and a
        // bumped epoch with no rank behind it must not swallow every later publish. A load is
        // exempt because it replaces the items the rows resolve through, and so is a switch:
        // rows kept from the listing it left would hang under the segment it moved to.
        if filter_active
            && self.rank_pending()
            && self.filter_task.is_some()
            && !self.loading
            && self.ranked_matches.is_empty()
            && self.rows.len() > 0
            && self.rows_listing.as_ref() == Some(&self.listing)
        {
            return;
        }
        let rows = self.build_rows(filter_active);
        let show_current_column = rows.shows_current_column();
        let placeholder = Self::placeholder_for(&self.listing);
        let is_directory = matches!(self.listing, BreadcrumbListing::Directory { .. });
        let (shown_empty_message, shown_truncation_note, shown_match_count) = {
            let delegate = &picker.read(cx).delegate;
            (
                delegate.empty_message.clone(),
                delegate.truncation_note.clone(),
                delegate.match_count_label.clone(),
            )
        };

        let empty_message: SharedString = if self.loading {
            LOADING_MESSAGE.into()
        } else if filter_active {
            if self.ranked_epoch != self.filter_epoch && self.ranked_matches.is_empty() {
                // Every keystroke bumps the filter epoch before its rank runs, so this branch is
                // reached once per letter. Downgrading a verdict the user is already reading
                // back to a provisional one makes the empty state strobe while they type; keep
                // what is on screen and let the settled rank be the thing that changes it.
                if shown_empty_message.as_ref() == NO_MATCHES_MESSAGE {
                    shown_empty_message
                } else {
                    SEARCHING_MESSAGE.into()
                }
            } else {
                NO_MATCHES_MESSAGE.into()
            }
        } else if is_directory {
            "Empty directory".into()
        } else {
            "No symbols".into()
        };

        // Both sit in the popup's chrome, so blanking them between a keystroke and its rank
        // blinks them out once per letter; hold the last settled ones until the rank lands.
        let truncation_note = if !filter_active {
            None
        } else if rank_settled {
            self.filter_match_truncated.then(|| {
                SharedString::from(format!("Showing first {MAX_BREADCRUMB_MENU_ROWS} matches"))
            })
        } else {
            shown_truncation_note
        };
        let match_count_label: Option<SharedString> = if !filter_active || self.loading {
            None
        } else if rank_settled {
            Some(if self.filter_match_truncated {
                format!("{}+ matches", MAX_BREADCRUMB_MENU_ROWS).into()
            } else if self.ranked_matches.len() == 1 {
                "1 match".into()
            } else {
                format!("{} matches", self.ranked_matches.len()).into()
            })
        } else {
            shown_match_count
        };

        let selected_index = self
            .selected_index
            .unwrap_or(0)
            .min(rows.len().saturating_sub(1));
        let scroll_to_selection = std::mem::take(&mut self.scroll_to_selection_pending);
        #[cfg(test)]
        {
            self.published_empty_message_history
                .push(empty_message.clone());
            self.published_match_count_history
                .push(match_count_label.clone());
            self.published_row_history.push(rows.labels());
            self.published_listing_history.push(self.listing.clone());
        }
        self.rows = rows.clone();
        self.rows_listing = Some(self.listing.clone());
        picker.update(cx, |picker, cx| {
            let delegate = &mut picker.delegate;
            delegate.rows = rows;
            delegate.selected_index = selected_index;
            delegate.empty_message = empty_message;
            delegate.placeholder = placeholder;
            delegate.truncation_note = truncation_note;
            delegate.match_count_label = match_count_label;
            delegate.show_current_column = show_current_column;
            delegate.show_file_icons = settings.file_icons;
            delegate.show_folder_icons = settings.folder_icons;
            if scroll_to_selection {
                picker.scroll_to_selected_index();
            }
            cx.notify();
        });
    }

    /// The rows share the menu's entries and outline instead of copying them, so an unfiltered
    /// listing publishes in constant time however long it is; only ranked rows are built.
    fn build_rows(&self, filter_active: bool) -> BreadcrumbMenuRows {
        match &self.listing {
            BreadcrumbListing::Directory { .. } => BreadcrumbMenuRows::Directory {
                entries: self.directory_entries.clone(),
                matches: filter_active.then(|| {
                    Rc::new(
                        self.ranked_matches
                            .iter()
                            .take(MAX_BREADCRUMB_MENU_ROWS)
                            .filter(|match_| match_.candidate_id < self.directory_entries.len())
                            .map(|match_| (match_.candidate_id, match_.positions.clone()))
                            .collect(),
                    )
                }),
            },
            BreadcrumbListing::Symbols { .. } => {
                let item_count = self.all_symbol_items.len();
                let mut rows: Vec<SymbolRow> = if filter_active {
                    self.ranked_matches
                        .iter()
                        .take(MAX_BREADCRUMB_MENU_ROWS)
                        .filter(|match_| match_.candidate_id < item_count)
                        .map(|match_| SymbolRow {
                            outline_index: match_.candidate_id,
                            match_positions: match_.positions.clone(),
                            indent: 0,
                            context: None,
                        })
                        .collect()
                } else {
                    self.listed_symbol_indices
                        .iter()
                        .filter(|&&outline_index| outline_index < item_count)
                        .map(|&outline_index| SymbolRow {
                            outline_index,
                            match_positions: Vec::new(),
                            indent: 0,
                            context: None,
                        })
                        .collect()
                };
                // A filter reaches symbols at any depth, so each row needs to say where it sits:
                // indent relative to the shallowest row, and name the parent for anything that is
                // not part of the level being browsed.
                if filter_active {
                    let shallowest = rows
                        .iter()
                        .filter_map(|row| self.symbol_depths.get(row.outline_index).copied())
                        .min()
                        .unwrap_or(0);
                    for row in &mut rows {
                        let depth = self
                            .symbol_depths
                            .get(row.outline_index)
                            .copied()
                            .unwrap_or(0);
                        row.indent = depth.saturating_sub(shallowest);
                        if self
                            .listed_symbol_indices
                            .binary_search(&row.outline_index)
                            .is_err()
                        {
                            row.context = self
                                .symbol_parents
                                .get(row.outline_index)
                                .copied()
                                .flatten()
                                .and_then(|parent| self.all_symbol_items.get(parent))
                                .map(|parent| {
                                    SharedString::from(flatten_text_for_single_line_display(
                                        &parent.text,
                                    ))
                                });
                        }
                    }
                }
                BreadcrumbMenuRows::Symbols {
                    items: self.all_symbol_items.clone(),
                    rows: Rc::new(rows),
                    current: self.deepest_cursor_symbol_range().cloned(),
                }
            }
        }
    }

    fn emit_bar_changed(&self, cx: &mut Context<Self>) {
        let editor = self.editor.clone();
        // Deferred: a segment click switches listings while it holds the editor.
        cx.defer(move |cx| {
            editor
                .update(cx, |_, cx| cx.emit(EditorEvent::BreadcrumbsChanged))
                .ok();
        });
    }

    fn worktree(&self, worktree_id: WorktreeId, cx: &App) -> Option<Entity<project::Worktree>> {
        let workspace = self.workspace.upgrade()?;
        workspace
            .read(cx)
            .project()
            .read(cx)
            .worktree_for_id(worktree_id, cx)
    }

    fn project(&self, cx: &App) -> Option<Entity<project::Project>> {
        Some(self.workspace.upgrade()?.read(cx).project().clone())
    }

    /// Every dismissal path funnels here, and only the first one through emits.
    fn emit_dismiss(&mut self, cx: &mut Context<Self>) {
        if !std::mem::replace(&mut self.dismiss_emitted, true) {
            cx.emit(DismissEvent);
        }
    }

    /// Takes the in-flight work with it: a switch that resolves after the dismiss would otherwise
    /// install a listing whose path is gone.
    fn dismiss_dead_listing(&mut self, cx: &mut Context<Self>) {
        self.load_epoch = self.load_epoch.wrapping_add(1);
        self.load_task = None;
        self.row_refresh_task = None;
        self.pending_switch = None;
        self.emit_dismiss(cx);
    }

    /// Whether a repository's status change can touch the git summaries of the listed rows: the
    /// listing sits inside it, or it sits inside the listing.
    fn listing_is_in_repository(
        &self,
        git_store: &Entity<GitStore>,
        repository_id: &RepositoryId,
        cx: &App,
    ) -> bool {
        let BreadcrumbListing::Directory { worktree_id, path } = &self.listing else {
            return false;
        };
        let Some(worktree) = self.worktree(*worktree_id, cx) else {
            return false;
        };
        let listing_path = worktree.read(cx).absolutize(path);
        git_store
            .read(cx)
            .repositories()
            .get(repository_id)
            .is_some_and(|repository| {
                let work_directory = &repository.read(cx).work_directory_abs_path;
                listing_path.starts_with(work_directory.as_ref())
                    || work_directory.starts_with(&listing_path)
            })
    }

    fn switch_is_current(&self, generation: u64) -> bool {
        self.pending_switch
            .as_ref()
            .is_some_and(|pending| pending.generation == generation)
    }

    fn holds_outline_for(&self, buffer_id: BufferId) -> bool {
        !self.all_symbol_items.is_empty()
            && matches!(
                self.listing,
                BreadcrumbListing::Symbols { buffer_id: listed, .. } if listed == buffer_id
            )
    }

    /// Only a directory the worktree has not loaded - an ignored one, say - needs expanding. For a
    /// loaded one the request is a rescan whose diff walks the whole subtree and re-announces it,
    /// which lists the directory a second time.
    fn expand_directory(
        &self,
        worktree_id: WorktreeId,
        path: &RelPath,
        cx: &mut Context<Self>,
    ) -> Option<Task<anyhow::Result<()>>> {
        let worktree = self.worktree(worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_path(path)?;
        if entry.kind == project::EntryKind::Dir {
            return None;
        }
        let entry_id = entry.id;
        let project = self.project(cx)?;
        project.update(cx, |project, cx| {
            project.expand_entry(worktree_id, entry_id, cx)
        })
    }

    /// Every listing change goes through here: the target is resolved - expanded, listed or
    /// outlined - before anything on screen moves, and a newer switch supersedes an older one.
    fn start_switch(
        &mut self,
        request: SwitchRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.switch_generation = self.switch_generation.wrapping_add(1);
        let generation = self.switch_generation;
        self.pending_switch = Some(PendingSwitch {
            generation,
            target: request.target.clone(),
            query_at_start: request
                .query_at_start
                .clone()
                .unwrap_or_else(|| self.query.clone()),
            step_outs: 0,
        });
        match request.target.clone() {
            BreadcrumbListing::Symbols { buffer_id, parent }
                if self.holds_outline_for(buffer_id) =>
            {
                self.switch_task = None;
                let resolved = ResolvedListing::Symbols {
                    buffer_id,
                    parent,
                    outline: None,
                };
                self.install(generation, resolved, request, window, cx);
            }
            BreadcrumbListing::Symbols { buffer_id, parent } => {
                self.switch_task = Some(cx.spawn_in(window, async move |this, cx| {
                    let Some(outline) = load_outline(this.clone(), buffer_id, cx).await else {
                        return;
                    };
                    this.update_in(cx, |this, window, cx| {
                        this.finish_symbols_switch(
                            generation, buffer_id, parent, outline, request, window, cx,
                        );
                    })
                    .ok();
                }));
            }
            BreadcrumbListing::Directory { worktree_id, path } => {
                let expand_task = self.expand_directory(worktree_id, &path, cx);
                self.switch_task = Some(cx.spawn_in(window, async move |this, cx| {
                    if let Some(task) = expand_task {
                        task.await.log_err();
                    }
                    let mut path = path;
                    if request.auto_fold {
                        for _ in 0..MAX_UNARY_DIRECTORY_SKIP_DEPTH {
                            let step = this
                                .update(cx, |this, cx| {
                                    if !this.switch_is_current(generation) {
                                        return None;
                                    }
                                    let worktree = this.worktree(worktree_id, cx)?;
                                    let child_path = single_child_directory(
                                        &directory_child_paths(&worktree, &path, 2, cx),
                                    )?;
                                    let expand =
                                        this.expand_directory(worktree_id, &child_path, cx);
                                    Some((child_path, expand))
                                })
                                .ok()
                                .flatten();
                            let Some((child_path, expand)) = step else {
                                break;
                            };
                            if let Some(task) = expand {
                                task.await.log_err();
                            }
                            path = child_path;
                        }
                    }
                    // `None` when superseded; `Some(None)` when the worktree itself is gone.
                    let inputs = this
                        .update(cx, |this, cx| {
                            this.switch_is_current(generation).then(|| {
                                this.worktree(worktree_id, cx).zip(this.project(cx)).map(
                                    |(worktree, project)| {
                                        (
                                            breadcrumb_directory_listing_inputs(
                                                &project, &worktree, cx,
                                            ),
                                            worktree.read(cx).scan_id(),
                                        )
                                    },
                                )
                            })
                        })
                        .ok()
                        .flatten();
                    let Some(inputs) = inputs else {
                        return;
                    };
                    let Some((inputs, scan_id)) = inputs else {
                        this.update(cx, |this, cx| {
                            if this.switch_is_current(generation) {
                                this.dismiss_dead_listing(cx);
                            }
                        })
                        .ok();
                        return;
                    };
                    let listing_path = path.clone();
                    let (entries, candidates) = cx
                        .background_spawn(async move {
                            let entries = breadcrumb_directory_entries(&inputs, &listing_path);
                            let candidates = directory_filter_candidates(&entries);
                            (entries, candidates)
                        })
                        .await;
                    this.update_in(cx, |this, window, cx| {
                        let resolved = ResolvedListing::Directory {
                            worktree_id,
                            path,
                            entries,
                            candidates,
                            scan_id,
                        };
                        this.install(generation, resolved, request, window, cx);
                    })
                    .ok();
                }));
            }
        }
    }

    fn finish_symbols_switch(
        &mut self,
        generation: u64,
        buffer_id: BufferId,
        parent: Option<OutlineItem<Anchor>>,
        outline: LoadedOutline,
        request: SwitchRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.switch_is_current(generation) {
            return;
        }
        if outline.items.is_empty() {
            // A file with no symbols has no level of its own. From the file's own row that
            // leaves the siblings already on screen; from its segment it lists them - the rule a
            // childless symbol follows - or closes when they are what is already listed.
            if request.without_symbols == WithoutSymbols::Stay {
                self.pending_switch = None;
                return;
            }
            let siblings = self
                .editor
                .upgrade()
                .and_then(|editor| file_parent_directory(editor.read(cx), cx));
            if let Some((worktree_id, parent_path, file_path)) = siblings {
                let siblings = BreadcrumbListing::Directory {
                    worktree_id,
                    path: parent_path,
                };
                if siblings == self.listing {
                    self.pending_switch = None;
                    self.emit_dismiss(cx);
                } else {
                    let request = SwitchRequest {
                        selection: SelectionHint::Path(file_path),
                        query_at_start: self
                            .pending_switch
                            .as_ref()
                            .map(|pending| pending.query_at_start.clone()),
                        ..SwitchRequest::new(siblings, request.active_file_path)
                    };
                    self.start_switch(request, window, cx);
                }
                return;
            }
        }
        let resolved = ResolvedListing::Symbols {
            buffer_id,
            parent,
            outline: Some(outline),
        };
        self.install(generation, resolved, request, window, cx);
    }

    fn install(
        &mut self,
        generation: u64,
        resolved: ResolvedListing,
        request: SwitchRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.switch_is_current(generation) {
            return;
        }
        let Some(pending) = self.pending_switch.take() else {
            return;
        };
        // Reloads still aimed at the listing being replaced must not land on the new one.
        self.load_epoch = self.load_epoch.wrapping_add(1);
        self.load_task = None;
        self.row_refresh_task = None;
        self.refresh_in_flight = false;
        self.refresh_queued = false;
        // Changes that land while a switch resolves are judged against the listing being left, so
        // the switch checks for them itself: whatever it read is installed, then refreshed.
        let changed_since_read = match resolved {
            ResolvedListing::Directory {
                worktree_id,
                path,
                entries,
                candidates,
                scan_id,
            } => {
                // The removal that takes this path can name it while the listing being left is
                // still installed, so the switch itself has to check the target is still there.
                let target = self.worktree(worktree_id, cx).and_then(|worktree| {
                    let worktree = worktree.read(cx);
                    worktree
                        .entry_for_path(&path)
                        .is_some_and(|entry| entry.is_dir())
                        .then(|| worktree.scan_id())
                });
                let Some(current_scan_id) = target else {
                    self.dismiss_dead_listing(cx);
                    return;
                };
                if request.navigated {
                    self.navigated_path = Some((worktree_id, path.clone()));
                } else {
                    let within_navigated_chain = self.navigated_path.as_ref().is_some_and(
                        |(navigated_worktree, navigated)| {
                            *navigated_worktree == worktree_id && navigated.starts_with(&path)
                        },
                    );
                    if !within_navigated_chain {
                        self.navigated_path = None;
                    }
                }
                self.symbol_trail.clear();
                self._buffer_subscription = None;
                self.all_symbol_items = Arc::default();
                self.symbol_depths.clear();
                self.symbol_parents.clear();
                self.listed_symbol_indices.clear();
                self.cursor_symbol_ranges.clear();
                self.directory_entries = Arc::new(entries);
                self.filter_candidates = Arc::new(candidates);
                self.listing = BreadcrumbListing::Directory { worktree_id, path };
                current_scan_id != scan_id
            }
            ResolvedListing::Symbols {
                buffer_id,
                parent,
                outline,
            } => {
                // Only a fetched outline carries a version, so the editor is read here only on
                // the asynchronous path, never under a segment click's lease on it.
                let fetched_version = outline.as_ref().and_then(|outline| outline.version.clone());
                if let Some(outline) = outline {
                    self.store_outline(outline);
                }
                self.navigated_path = None;
                self.directory_entries = Arc::default();
                self.listing = BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: parent.clone(),
                };
                self.apply_symbol_parent(parent);
                self.subscribe_listed_buffer(buffer_id, cx);
                fetched_version.is_some_and(|fetched_version| {
                    self.editor
                        .upgrade()
                        .and_then(|editor| editor.read(cx).buffer().read(cx).buffer(buffer_id))
                        .is_some_and(|buffer| {
                            buffer.read(cx).version().changed_since(&fetched_version)
                        })
                })
            }
        };
        self.loading = false;
        self.active_file_path = request.active_file_path;
        self.pending_restore_path = None;
        self.pending_restore_symbol_range = None;
        self.pre_filter_selection = None;
        self.filter_selection_touched = false;
        if self.query != pending.query_at_start && !self.query.is_empty() {
            // Typed while the switch was resolving: the text was meant for the listing arriving
            // now, so it filters these rows rather than being thrown away with the old ones.
            // Erasing it comes back to the row this listing would have opened on.
            self.pre_filter_selection = self
                .hinted_position(&request.selection)
                .or_else(|| self.initial_selected_index())
                .and_then(|position| self.pre_filter_selection_at(position));
            self.ranked_matches.clear();
            self.selected_index = None;
            self.pending_initial_selection = false;
            self.rerank_filter(cx);
        } else {
            // A query from before the switch described the rows being left, and one erased
            // meanwhile leaves nothing to filter, so the filter restarts on these rows - which
            // also retires the epoch of any reload this switch cancelled. Text typed after the
            // switch started may not have reached the menu yet, so the box keeps what it reads.
            self.clear_filter_shown_as(Some(pending.query_at_start), window, cx);
            self.selected_index = self
                .hinted_position(&request.selection)
                .or_else(|| self.initial_selected_index());
            self.pending_initial_selection = self.selected_index.is_none();
        }
        self.scroll_to_selection_pending = true;
        self.publish_rows(cx);
        self.refresh_placeholder(window, cx);
        self.focus_menu(window, cx);
        self.emit_bar_changed(cx);
        cx.notify();
        for _ in 0..pending.step_outs {
            self.select_parent(&menu::SelectParent, window, cx);
        }
        if changed_since_read {
            // Deferred behind the publish above, so the refresh keeps the row this install chose.
            let menu = cx.weak_entity();
            cx.defer(move |cx| {
                menu.update(cx, |this, cx| this.refresh_listing(cx)).ok();
            });
        }
    }

    fn refresh_listing(&mut self, cx: &mut Context<Self>) {
        match self.listing.clone() {
            BreadcrumbListing::Directory { .. } => self.reload_directory_rows(cx),
            BreadcrumbListing::Symbols { buffer_id, parent } => {
                self.reload_symbols_from_buffer(buffer_id, parent, cx)
            }
        }
    }

    fn hinted_position(&self, hint: &SelectionHint) -> Option<usize> {
        match (hint, &self.listing) {
            (SelectionHint::Path(path), BreadcrumbListing::Directory { .. }) => self
                .directory_entries
                .iter()
                .position(|entry| entry.path.as_ref() == path.as_ref()),
            (SelectionHint::Symbol(range), BreadcrumbListing::Symbols { .. }) => {
                self.listed_symbol_indices.iter().position(|&index| {
                    self.all_symbol_items
                        .get(index)
                        .is_some_and(|item| item.range == *range)
                })
            }
            _ => None,
        }
    }

    /// Refreshes the listing on screen after a worktree, git or settings change. At most one runs
    /// at a time; changes arriving meanwhile queue one more pass instead of stacking rebuilds.
    fn reload_directory_rows(&mut self, cx: &mut Context<Self>) {
        let BreadcrumbListing::Directory { worktree_id, path } = &self.listing else {
            return;
        };
        if self.refresh_in_flight {
            self.refresh_queued = true;
            return;
        }
        #[cfg(test)]
        {
            self.directory_reload_count = self.directory_reload_count.wrapping_add(1);
        }
        let (worktree_id, path) = (*worktree_id, path.clone());
        let Some((worktree, project)) = self.worktree(worktree_id, cx).zip(self.project(cx)) else {
            self.directory_entries = Arc::default();
            self.publish_rows(cx);
            cx.notify();
            return;
        };
        let inputs = breadcrumb_directory_listing_inputs(&project, &worktree, cx);
        // Tied to the listing on screen: a switch installing meanwhile discards it, and it never
        // cancels a switch that is still resolving.
        let epoch = self.load_epoch;
        let listing = self.listing.clone();
        self.refresh_in_flight = true;
        self.row_refresh_task = Some(cx.spawn(async move |this, cx| {
            let (entries, candidates) = cx
                .background_spawn(async move {
                    let entries = breadcrumb_directory_entries(&inputs, &path);
                    let candidates = directory_filter_candidates(&entries);
                    (entries, candidates)
                })
                .await;
            this.update(cx, |this, cx| {
                if this.load_epoch != epoch || this.listing != listing {
                    return;
                }
                this.refresh_in_flight = false;
                // The row the user is on now, not when the refresh started: arrowing while it ran
                // must not be undone when it lands.
                let selected_path = this.selected_directory_entry().map(|entry| entry.path);
                this.directory_entries = Arc::new(entries);
                this.filter_candidates = Arc::new(candidates);
                this.publish_rows(cx);
                this.apply_reloaded_selection(selected_path, cx);
                cx.notify();
                if std::mem::take(&mut this.refresh_queued) {
                    this.reload_directory_rows(cx);
                }
            })
            .ok();
        }));
    }

    fn apply_reloaded_selection(
        &mut self,
        selected_path: Option<Arc<RelPath>>,
        cx: &mut Context<Self>,
    ) {
        if !self.filter_is_empty() {
            self.ranked_matches.clear();
            self.selected_index = None;
            // Latched, not overwritten: a second refresh landing before the rank consumes the latch
            // sees `selected_index` already blanked above, so it would carry a `None` in and drop the
            // user's row to the top match.
            if selected_path.is_some() {
                self.pending_restore_path = selected_path;
            }
            self.rerank_filter(cx);
        } else if let Some(selected_path) = selected_path {
            self.selected_index = self
                .directory_entries
                .iter()
                .position(|entry| entry.path.as_ref() == selected_path.as_ref());
            if self.selected_index.is_none() {
                self.pending_initial_selection = true;
            } else {
                self.scroll_to_selection_pending = true;
            }
            self.apply_initial_selection_if_needed(cx);
        } else {
            let visible = self.visible_row_count();
            if let Some(position) = self.selected_index
                && position >= visible
            {
                self.selected_index = visible.checked_sub(1);
            }
            self.apply_initial_selection_if_needed(cx);
        }
    }

    fn store_outline(&mut self, outline: LoadedOutline) {
        self.all_symbol_items = Arc::new(outline.items);
        self.symbol_depths = outline.depths;
        self.symbol_parents = outline.parents;
        self.filter_candidates = Arc::new(outline.candidates);
        self.cursor_symbol_ranges = outline.cursor_ranges;
    }

    fn reload_symbols_from_buffer(
        &mut self,
        buffer_id: BufferId,
        parent: Option<OutlineItem<Anchor>>,
        cx: &mut Context<Self>,
    ) {
        // Latched here, not after the reload: the lines below blank the rows this reads from,
        // so a second edit landing mid-reload would find nothing to name the row with. Only
        // once the user has moved the highlight - before that, the reload keeps re-picking the
        // row for the cursor.
        if !self.pending_initial_selection && self.pending_restore_symbol_range.is_none() {
            self.pending_restore_symbol_range = self.selected_symbol_item().map(|item| item.range);
        }
        // `symbol_trail` is deliberately kept: it is what the bar paints the menu's anchor
        // segment from, and a frame without an anchor dismisses the menu.
        self.all_symbol_items = Arc::default();
        self.symbol_depths.clear();
        self.symbol_parents.clear();
        self.listed_symbol_indices.clear();
        // Candidates and matches are outline indices into `all_symbol_items`, so keeping them
        // past the clear would publish a match count over rows that can no longer resolve.
        self.filter_candidates = Arc::new(Vec::new());
        self.ranked_matches.clear();
        self.filter_match_truncated = false;
        self.selected_index = None;
        if let Some(cancel) = self.filter_cancel.take() {
            cancel.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        self.filter_task = None;
        self.filter_epoch = self.filter_epoch.wrapping_add(1);
        self.load_epoch = self.load_epoch.wrapping_add(1);
        let epoch = self.load_epoch;
        self.loading = true;
        // Rows carry their own item, so leaving the old ones up would let a click act on a
        // symbol this buffer no longer has. Publishing here empties them until the reload lands.
        self.publish_rows(cx);
        self.load_task = Some(cx.spawn(async move |this, cx| {
            let outline = load_outline(this.clone(), buffer_id, cx).await;
            this.update(cx, |this, cx| {
                if this.load_epoch != epoch {
                    return;
                }
                this.loading = false;
                if let Some(outline) = outline {
                    this.store_outline(outline);
                }
                this.apply_symbol_parent(parent);
                if this.filter_is_empty() {
                    // Nothing ranks an empty query, so the epoch the reload bumped has to be
                    // retired here, or the drill reads a rank still owed and refuses to move.
                    this.ranked_epoch = this.filter_epoch;
                    this.apply_initial_selection_if_needed(cx);
                } else {
                    this.ranked_matches.clear();
                    this.selected_index = None;
                    this.rerank_filter(cx);
                }
                this.restore_symbol_selection(cx);
                this.publish_rows(cx);
                this.emit_bar_changed(cx);
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    fn restore_symbol_selection(&mut self, cx: &mut Context<Self>) {
        // Taken before the filter check: a reload that lands under a query has its selection
        // re-derived by the rank, so holding the latch back would carry this level's row into
        // whatever level the user drills to next.
        let Some(selected_symbol_range) = self.pending_restore_symbol_range.take() else {
            return;
        };
        if !self.filter_is_empty() {
            return;
        }
        self.selected_index = self.hinted_position(&SelectionHint::Symbol(selected_symbol_range));
        if self.selected_index.is_none() {
            // The edit took the symbol the user was on with it, so fall back to the rule that
            // picks a row when the menu first opens rather than leaving nothing highlighted.
            self.pending_initial_selection = true;
            self.apply_initial_selection_if_needed(cx);
        } else {
            self.scroll_to_selection_pending = true;
        }
        self.publish_rows(cx);
        cx.notify();
    }

    fn subscribe_listed_buffer(&mut self, buffer_id: BufferId, cx: &mut Context<Self>) {
        self._buffer_subscription = None;
        let menu = cx.weak_entity();
        let editor = self.editor.clone();
        cx.defer(move |cx| {
            let Some(buffer) = editor
                .upgrade()
                .and_then(|editor| editor.read(cx).buffer().read(cx).buffer(buffer_id))
            else {
                return;
            };
            menu.update(cx, |this, cx| {
                if !matches!(
                    this.listing,
                    BreadcrumbListing::Symbols {
                        buffer_id: listing_buffer,
                        ..
                    } if listing_buffer == buffer_id
                ) {
                    return;
                }
                this._buffer_subscription =
                    Some(cx.subscribe(&buffer, move |this, _, event, cx| {
                        if !matches!(
                            event,
                            language::BufferEvent::Edited { .. } | language::BufferEvent::Reloaded
                        ) {
                            return;
                        }
                        let BreadcrumbListing::Symbols {
                            buffer_id: listing_buffer,
                            parent,
                        } = this.listing.clone()
                        else {
                            return;
                        };
                        if listing_buffer != buffer_id {
                            return;
                        }
                        this.reload_symbols_from_buffer(buffer_id, parent, cx);
                    }));
            })
            .ok();
        });
    }

    /// Whether opening `listing` would put exactly the rows already on screen back on screen.
    /// A childless symbol lists its siblings, which is also what its parent lists, so two
    /// segments can name the same rows - and a click on the other one is then a second click on
    /// the same thing, which closes rather than re-anchors.
    pub fn lists_same_rows_as(&self, listing: &BreadcrumbListing) -> bool {
        if &self.listing == listing {
            return true;
        }
        match (&self.listing, listing) {
            (
                BreadcrumbListing::Symbols {
                    buffer_id: open_buffer,
                    ..
                },
                BreadcrumbListing::Symbols { buffer_id, parent },
            ) => {
                if open_buffer != buffer_id || self.all_symbol_items.is_empty() {
                    return false;
                }
                self.level_indices(parent.as_ref()) == self.listed_symbol_indices
            }
            _ => false,
        }
    }

    /// The rows a symbols listing shows for `parent`: its children, or its siblings when it has
    /// none; the top level for no parent, or for one this outline no longer has.
    fn level_indices(&self, parent: Option<&OutlineItem<Anchor>>) -> Vec<usize> {
        match parent.and_then(|parent| {
            self.all_symbol_items
                .iter()
                .position(|item| same_symbol_item(item, parent))
        }) {
            Some(parent_index) => level_outline_indices(&self.symbol_depths, parent_index),
            None => top_level_outline_indices(&self.symbol_depths),
        }
    }

    fn apply_symbol_parent(&mut self, mut parent: Option<OutlineItem<Anchor>>) {
        // An edit removed the symbol this listing was opened on; an empty listing here would
        // have no keyboard way back out.
        if parent.as_ref().is_some_and(|parent| {
            !self
                .all_symbol_items
                .iter()
                .any(|item| same_symbol_item(item, parent))
        }) {
            parent = None;
        }
        self.listed_symbol_indices = self.level_indices(parent.as_ref());
        if let BreadcrumbListing::Symbols { buffer_id, .. } = self.listing {
            self.listing = BreadcrumbListing::Symbols { buffer_id, parent };
        }
        self.rebuild_symbol_trail();
    }

    fn rebuild_symbol_trail(&mut self) {
        let BreadcrumbListing::Symbols {
            parent: Some(parent),
            ..
        } = &self.listing
        else {
            self.symbol_trail.clear();
            return;
        };
        let Some(mut index) = self
            .all_symbol_items
            .iter()
            .position(|item| same_symbol_item(item, parent))
        else {
            self.symbol_trail.clear();
            return;
        };
        let mut trail = vec![self.all_symbol_items[index].clone()];
        while let Some(parent_index) = self.symbol_parents.get(index).copied().flatten() {
            trail.push(self.all_symbol_items[parent_index].clone());
            index = parent_index;
        }
        trail.reverse();
        self.symbol_trail = trail;
    }

    fn rerank_filter(&mut self, cx: &mut Context<Self>) {
        if let Some(cancel) = self.filter_cancel.take() {
            cancel.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        self.filter_epoch = self.filter_epoch.wrapping_add(1);
        let epoch = self.filter_epoch;
        let query = self.filter_query().to_string();

        if query.is_empty() {
            // `selected_index` changes what it addresses when the query clears, so carry the
            // selection over by identity.
            let selected_candidate = self
                .selected_index
                .and_then(|position| self.ranked_matches.get(position))
                .map(|match_| match_.candidate_id);
            // For symbols the candidate is an outline index, which only addresses a row if the
            // symbol belongs to the level being browsed; a match from elsewhere has no row.
            let carried_selection = match (&self.listing, selected_candidate) {
                (BreadcrumbListing::Symbols { .. }, Some(outline_index)) => self
                    .listed_symbol_indices
                    .iter()
                    .position(|listed| *listed == outline_index),
                (_, candidate) => candidate,
            };
            // A search the user typed and erased without arrowing is abandoned, so the row
            // they left comes back; a search they arrowed through is a choice, so that stays.
            let restored_selection = if self.filter_selection_touched {
                None
            } else {
                self.pre_filter_selection
                    .as_ref()
                    .and_then(|selection| self.position_of_pre_filter_selection(selection))
            };
            let unranked_selection = restored_selection.or(carried_selection);
            self.pre_filter_selection = None;
            self.filter_selection_touched = false;
            // The old value addresses the ranked matches that are about to be dropped, so
            // keeping it would silently highlight whichever sibling sits at that index.
            if selected_candidate.is_some() && unranked_selection.is_none() {
                self.selected_index = None;
                self.pending_initial_selection = true;
            }
            self.ranked_matches.clear();
            self.filter_match_truncated = false;
            self.filter_task = None;
            self.ranked_epoch = epoch;
            if let Some(position) = unranked_selection {
                self.selected_index = Some(position);
            }
            if self.selected_index.is_none() {
                self.apply_initial_selection_if_needed(cx);
            } else {
                let visible = self.visible_row_count();
                if let Some(position) = self.selected_index
                    && position >= visible
                {
                    self.selected_index = visible.checked_sub(1);
                }
            }
            // Same promise as the ranked branch: settling says the delegate's rows already
            // describe this query, so the publish cannot be left to the effect queue.
            self.publish_rows_now(cx);
            self.filter_settled.settle();
            cx.notify();
            return;
        }

        let candidates = self.filter_candidates.clone();
        let executor = cx.background_executor().clone();
        let cancel_flag = Arc::new(AtomicBool::new(false));
        self.filter_cancel = Some(cancel_flag.clone());
        self.filter_settled.arm();
        self.filter_task = Some(cx.spawn(async move |this, cx| {
            // On the background executor, not just the match itself: `match_strings` gathers
            // and sorts its results after the parallel part, and with the cap lifted below that
            // is the whole matching set rather than a screenful.
            let matches = cx
                .background_spawn(async move {
                    // Ranked in full: `match_strings` truncates on its own comparator, which on
                    // equal scores keeps the highest candidate ids - the last files of the
                    // listing - while the cap promises the first ones. The sort and the cut
                    // stay on this thread too; over a large directory they are the cost.
                    let mut matches = fuzzy::match_strings(
                        candidates.as_slice(),
                        &query,
                        false,
                        true,
                        usize::MAX,
                        &cancel_flag,
                        executor,
                    )
                    .await;
                    // Equal scores otherwise come back in reverse listing order, so filtering a
                    // directory of item_000..item_199 opened on item_199. Only the rows shown are
                    // sorted; one past the cap is what tells the footer the list was cut.
                    let by_rank = |a: &StringMatch, b: &StringMatch| {
                        b.score
                            .partial_cmp(&a.score)
                            .unwrap_or(Ordering::Equal)
                            .then(a.candidate_id.cmp(&b.candidate_id))
                    };
                    let kept = MAX_BREADCRUMB_MENU_ROWS + 1;
                    if matches.len() > kept {
                        matches.select_nth_unstable_by(kept - 1, by_rank);
                        matches.truncate(kept);
                    }
                    matches.sort_by(by_rank);
                    matches
                })
                .await;
            this.update(cx, |this, cx| {
                if this.filter_epoch != epoch {
                    return;
                }
                this.ranked_epoch = epoch;
                this.filter_match_truncated = matches.len() > MAX_BREADCRUMB_MENU_ROWS;
                this.ranked_matches = matches.into_iter().take(MAX_BREADCRUMB_MENU_ROWS).collect();
                // Restored by path, never by rank position: a reload rebuilds
                // `directory_entries`, so the candidate ids the old positions addressed now
                // mean different files.
                let restored = this.pending_restore_path.take().and_then(|path| {
                    this.ranked_matches.iter().position(|match_| {
                        this.directory_entries
                            .get(match_.candidate_id)
                            .is_some_and(|entry| entry.path.as_ref() == path.as_ref())
                    })
                });
                // Otherwise kept if it still addresses a row: the user can arrow through
                // results while the rank is in flight, and a new query clears it so that case
                // still lands on the best match.
                this.selected_index = restored.or(match this.selected_index {
                    Some(index) if index < this.ranked_matches.len() => Some(index),
                    _ => (!this.ranked_matches.is_empty()).then_some(0),
                });
                this.scroll_to_selection_pending = true;
                // Published inline rather than deferred: the picker treats this task's
                // completion as "rows are final", and a deferred publish would land a cycle
                // after that promise.
                this.publish_rows_now(cx);
                this.filter_settled.settle();
                cx.notify();
            })
            .ok();
        }));
    }

    /// The production paths build candidates with the data they load - directories off the
    /// foreground, symbols once per outline - so this stays only for the test constructor that
    /// installs a listing straight from items.
    #[cfg(test)]
    fn rebuild_filter_candidates(&mut self) {
        let candidates = match &self.listing {
            BreadcrumbListing::Directory { .. } => {
                directory_filter_candidates(&self.directory_entries)
            }
            BreadcrumbListing::Symbols { .. } => symbol_filter_candidates(&self.all_symbol_items),
        };
        self.filter_candidates = Arc::new(candidates);
    }

    fn visible_row_count(&self) -> usize {
        if !self.filter_is_empty() {
            return self.ranked_matches.len().min(MAX_BREADCRUMB_MENU_ROWS);
        }
        match &self.listing {
            BreadcrumbListing::Directory { .. } => self.directory_entries.len(),
            BreadcrumbListing::Symbols { .. } => self.listed_symbol_indices.len(),
        }
    }

    #[cfg(test)]
    fn visible_row_labels(&self) -> Vec<SharedString> {
        self.build_rows(!self.filter_is_empty()).labels()
    }

    fn apply_initial_selection_if_needed(&mut self, cx: &mut Context<Self>) {
        if !self.pending_initial_selection || !self.filter_is_empty() || self.loading {
            return;
        }
        let visible = self.visible_row_count();
        if visible == 0 {
            return;
        }
        self.pending_initial_selection = false;
        self.selected_index = self.initial_selected_index();
        self.scroll_to_selection_pending = true;
        self.publish_rows(cx);
        cx.notify();
    }

    fn position_of_pre_filter_selection(&self, selection: &PreFilterSelection) -> Option<usize> {
        match selection {
            PreFilterSelection::Directory(path) => {
                self.hinted_position(&SelectionHint::Path(path.clone()))
            }
            PreFilterSelection::Symbol(range) => {
                self.hinted_position(&SelectionHint::Symbol(range.clone()))
            }
        }
    }

    /// The unfiltered row at `position`, by identity.
    fn pre_filter_selection_at(&self, position: usize) -> Option<PreFilterSelection> {
        match &self.listing {
            BreadcrumbListing::Directory { .. } => self
                .directory_entries
                .get(position)
                .map(|entry| PreFilterSelection::Directory(entry.path.clone())),
            BreadcrumbListing::Symbols { .. } => self
                .listed_symbol_indices
                .get(position)
                .and_then(|&index| self.all_symbol_items.get(index))
                .map(|item| PreFilterSelection::Symbol(item.range.clone())),
        }
    }

    fn initial_selected_index(&self) -> Option<usize> {
        match &self.listing {
            BreadcrumbListing::Directory { .. } => {
                let first = (!self.directory_entries.is_empty()).then_some(0);
                let Some(active_path) = self.active_file_path.as_ref() else {
                    return first;
                };
                self.directory_entries
                    .iter()
                    .position(|entry| entry.path.as_ref() == active_path.as_ref())
                    .or_else(|| {
                        self.directory_entries.iter().position(|entry| {
                            entry.is_dir && active_path.starts_with(entry.path.as_ref())
                        })
                    })
                    .or(first)
            }
            BreadcrumbListing::Symbols { .. } => self
                .listed_symbol_indices
                .iter()
                .position(|&index| {
                    self.all_symbol_items
                        .get(index)
                        .is_some_and(|item| self.cursor_symbol_ranges.contains(&item.range))
                })
                .or((!self.listed_symbol_indices.is_empty()).then_some(0)),
        }
    }

    pub(super) fn confirm(
        &mut self,
        _: &menu::Confirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &self.listing {
            BreadcrumbListing::Directory { .. } => {
                let Some(entry) = self.selected_directory_entry() else {
                    return;
                };
                if entry.is_dir {
                    self.drill_into_directory(entry, window, cx);
                } else {
                    self.open_file(entry.path, window, cx);
                }
            }
            BreadcrumbListing::Symbols { .. } => {
                let Some(item) = self.selected_symbol_item() else {
                    return;
                };
                self.navigate_to_symbol(&item, window, cx);
            }
        }
    }

    fn select_child(&mut self, _: &menu::SelectChild, window: &mut Window, cx: &mut Context<Self>) {
        match self.listing.clone() {
            BreadcrumbListing::Directory { .. } => {
                let Some(entry) = self.selected_directory_entry() else {
                    return;
                };
                if entry.is_dir {
                    self.drill_into_directory(entry, window, cx);
                } else if self
                    .active_file_path
                    .as_ref()
                    .is_some_and(|path| path.as_ref() == entry.path.as_ref())
                {
                    let Some(buffer_id) = self.editor_buffer_id(cx) else {
                        return;
                    };
                    let request = SwitchRequest {
                        without_symbols: WithoutSymbols::Stay,
                        ..SwitchRequest::new(
                            BreadcrumbListing::Symbols {
                                buffer_id,
                                parent: None,
                            },
                            self.active_file_path.clone(),
                        )
                    };
                    self.start_switch(request, window, cx);
                } else {
                    self.open_file(entry.path, window, cx);
                }
            }
            BreadcrumbListing::Symbols { buffer_id, .. } => {
                let Some(outline_index) = self.selected_symbol_outline_index() else {
                    return;
                };
                let children = child_outline_indices(&self.symbol_depths, outline_index);
                if children.is_empty() {
                    return;
                }
                let parent = self.all_symbol_items.get(outline_index).cloned();
                self.transition_to_symbol_listing(
                    buffer_id,
                    parent,
                    children,
                    SelectionHint::Initial,
                    window,
                    cx,
                );
            }
        }
    }

    fn drill_into_directory(
        &mut self,
        entry: BreadcrumbDirectoryEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let BreadcrumbListing::Directory { worktree_id, .. } = self.listing else {
            return;
        };
        let request = SwitchRequest {
            navigated: true,
            auto_fold: BreadcrumbListingSettings::get_global(cx).auto_fold_dirs,
            ..SwitchRequest::new(
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: entry.path,
                },
                self.active_file_path.clone(),
            )
        };
        self.start_switch(request, window, cx);
    }

    /// Re-windows the outline the menu holds, so it installs at once. The level on screen is
    /// what the key acted on, so a switch still resolving is superseded.
    fn transition_to_symbol_listing(
        &mut self,
        buffer_id: BufferId,
        parent: Option<OutlineItem<Anchor>>,
        listed_indices: Vec<usize>,
        selection: SelectionHint,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pending_switch = None;
        self.switch_task = None;
        self.clear_filter(window, cx);
        self.listing = BreadcrumbListing::Symbols { buffer_id, parent };
        self.listed_symbol_indices = listed_indices;
        self.rebuild_symbol_trail();
        self.selected_index = self
            .hinted_position(&selection)
            .or_else(|| self.initial_selected_index());
        self.pending_initial_selection = self.selected_index.is_none();
        self.scroll_to_selection_pending = true;
        self.publish_rows(cx);
        self.emit_bar_changed(cx);
        cx.notify();
    }

    fn select_parent(
        &mut self,
        _: &menu::SelectParent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(pending) = self.pending_switch.as_mut()
            && matches!(
                pending.target,
                BreadcrumbListing::Symbols {
                    parent: Some(_),
                    ..
                }
            )
        {
            // The level above a symbol is read off the outline that switch is still fetching.
            pending.step_outs += 1;
            return;
        }
        // Chains from a switch still resolving, so Left twice climbs two levels even when the
        // first has not landed yet.
        let base = self
            .pending_switch
            .as_ref()
            .map(|pending| pending.target.clone())
            .unwrap_or_else(|| self.listing.clone());
        match base {
            BreadcrumbListing::Directory { worktree_id, path } => {
                let Some(parent) = path.parent() else {
                    return;
                };
                let request = SwitchRequest {
                    navigated: true,
                    selection: SelectionHint::Path(path.clone()),
                    ..SwitchRequest::new(
                        BreadcrumbListing::Directory {
                            worktree_id,
                            path: parent.into_arc(),
                        },
                        self.active_file_path.clone(),
                    )
                };
                self.start_switch(request, window, cx);
            }
            BreadcrumbListing::Symbols { parent: None, .. } => {
                self.step_out_to_file_directory(window, cx);
            }
            BreadcrumbListing::Symbols {
                buffer_id,
                parent: Some(_),
            } => {
                if self.holds_outline_for(buffer_id) {
                    self.step_out_of_symbol_level(buffer_id, window, cx);
                }
            }
        }
    }

    fn step_out_of_symbol_level(
        &mut self,
        buffer_id: BufferId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(&first) = self.listed_symbol_indices.first() else {
            return;
        };
        match self.symbol_parents.get(first).copied().flatten() {
            // The rows are the top level already - a childless top-level symbol lists its
            // siblings - so the level above them is the file's directory.
            None => self.step_out_to_file_directory(window, cx),
            Some(parent_index) => {
                let siblings = sibling_outline_indices(&self.symbol_depths, parent_index);
                let new_parent = self
                    .symbol_parents
                    .get(parent_index)
                    .copied()
                    .flatten()
                    .and_then(|index| self.all_symbol_items.get(index).cloned());
                let selection = self
                    .all_symbol_items
                    .get(parent_index)
                    .map(|item| SelectionHint::Symbol(item.range.clone()))
                    .unwrap_or(SelectionHint::Initial);
                self.transition_to_symbol_listing(
                    buffer_id, new_parent, siblings, selection, window, cx,
                );
            }
        }
    }

    fn step_out_to_file_directory(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((worktree_id, parent_path, file_path)) = self
            .editor
            .upgrade()
            .and_then(|editor| file_parent_directory(editor.read(cx), cx))
        else {
            return;
        };
        let request = SwitchRequest {
            selection: SelectionHint::Path(file_path),
            ..SwitchRequest::new(
                BreadcrumbListing::Directory {
                    worktree_id,
                    path: parent_path,
                },
                self.active_file_path.clone(),
            )
        };
        self.start_switch(request, window, cx);
    }

    fn editor_buffer_id(&self, cx: &App) -> Option<BufferId> {
        let editor = self.editor.upgrade()?;
        let buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
        Some(buffer.read(cx).remote_id())
    }

    fn selected_directory_entry(&self) -> Option<BreadcrumbDirectoryEntry> {
        self.rows
            .directory_entry(self.selected_index?)
            .map(|(entry, _)| entry.clone())
    }

    fn selected_symbol_item(&self) -> Option<OutlineItem<Anchor>> {
        self.rows
            .symbol(self.selected_index?)
            .map(|(item, _)| item.clone())
    }

    fn selected_symbol_outline_index(&self) -> Option<usize> {
        self.rows
            .symbol(self.selected_index?)
            .map(|(_, row)| row.outline_index)
    }

    fn open_file(&mut self, path: Arc<RelPath>, window: &mut Window, cx: &mut Context<Self>) {
        let BreadcrumbListing::Directory { worktree_id, .. } = self.listing else {
            return;
        };
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace
                    .open_path(ProjectPath { worktree_id, path }, None, true, window, cx)
                    .detach_and_log_err(cx);
            });
        }
        self.emit_dismiss(cx);
    }

    fn navigate_to_symbol(
        &mut self,
        item: &OutlineItem<Anchor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_to_outline_item(item, window, cx);
            });
        }
        self.emit_dismiss(cx);
    }

    /// Deferred and re-checked: a press outside can be a click on another breadcrumb segment,
    /// which retargets the menu - at once, or through a switch that is still resolving - instead
    /// of closing it.
    fn dismiss_after_release_outside(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let listing = self.listing.clone();
        let generation = self.switch_generation;
        cx.defer_in(window, move |this, _window, cx| {
            if this.listing == listing && this.switch_generation == generation {
                this.emit_dismiss(cx);
            }
        });
    }

    fn deepest_cursor_symbol_range(&self) -> Option<&Range<Anchor>> {
        self.cursor_symbol_ranges.last()
    }
}

const LOADING_MESSAGE: &str = "Loading…";
const SEARCHING_MESSAGE: &str = "Searching…";
const NO_MATCHES_MESSAGE: &str = "No matches";

/// A symbol segment lists its children, and falls back to its siblings only when it has none.
fn level_outline_indices(depths: &[usize], parent_index: usize) -> Vec<usize> {
    let children = child_outline_indices(depths, parent_index);
    if children.is_empty() {
        sibling_outline_indices(depths, parent_index)
    } else {
        children
    }
}

const MAX_ROW_LABEL_CHARS_WITHOUT_TOOLTIP: usize = 24;

fn row_label_needs_tooltip(label: &str) -> bool {
    label.chars().count() > MAX_ROW_LABEL_CHARS_WITHOUT_TOOLTIP
}

impl gpui::Focusable for BreadcrumbNavigationMenu {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.picker {
            Some(picker) => picker.focus_handle(cx),
            None => cx.focus_handle(),
        }
    }
}

impl EventEmitter<DismissEvent> for BreadcrumbNavigationMenu {}

/// Judges the press and the release against the same geometry. GPUI's `on_mouse_down_out`
/// tests `contains` while `on_mouse_up_out` tests `is_hovered`, and the two disagree over the
/// picker's scrollbar: its thumb blocks hit testing, so a release on the thumb reads as outside
/// the popup and closes it in the middle of a drag.
struct OutsideClickBoundary {
    child: gpui::AnyElement,
    menu: WeakEntity<BreadcrumbNavigationMenu>,
}

impl gpui::IntoElement for OutsideClickBoundary {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for OutsideClickBoundary {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);

        let menu = self.menu.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase != DispatchPhase::Capture {
                return;
            }
            let pressed_outside = matches!(
                event.button,
                MouseButton::Left | MouseButton::Right | MouseButton::Middle
            ) && !bounds.contains(&window.mouse_position());
            menu.update(cx, |menu, _| menu.pressed_outside = pressed_outside)
                .ok();
        });

        let menu = self.menu.clone();
        window.on_mouse_event(move |_: &MouseUpEvent, phase, window, cx| {
            if phase != DispatchPhase::Capture {
                return;
            }
            // Cleared on every release, so an arm can never be judged by a later press.
            let was_pressed_outside = menu
                .update(cx, |menu, _| std::mem::take(&mut menu.pressed_outside))
                .unwrap_or(false);
            if was_pressed_outside && !bounds.contains(&window.mouse_position()) {
                menu.update(cx, |menu, cx| {
                    menu.dismiss_after_release_outside(window, cx)
                })
                .ok();
            }
        });
    }
}

impl Render for BreadcrumbNavigationMenu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(picker) = self.picker.clone() else {
            return div().into_any_element();
        };
        let theme_settings = theme::theme_settings(cx);
        let ui_font_size = theme_settings.ui_font_size(cx);
        let ui_font_family = theme_settings.ui_font(cx).family.clone();
        WithRemSize::new(ui_font_size)
            .font_family(ui_font_family)
            .occlude()
            .child(OutsideClickBoundary {
                child: div()
                    .id("breadcrumb-navigation-menu")
                    .debug_selector(|| "breadcrumb-navigation-menu".into())
                    .key_context("BreadcrumbNavigationMenu")
                    .child(picker)
                    .into_any_element(),
                menu: cx.entity().downgrade(),
            })
            .into_any_element()
    }
}
#[derive(Clone)]
enum PreFilterSelection {
    Directory(Arc<RelPath>),
    Symbol(Range<Anchor>),
}
/// What the picker renders. It shares the menu's entries and outline rather than copying them,
/// and `PickerDelegate` renders from `&self` while the menu is itself mid-render, so the delegate
/// holds its own handle to them instead of reading the menu back.
#[derive(Clone)]
pub(super) enum BreadcrumbMenuRows {
    Directory {
        entries: Arc<Vec<BreadcrumbDirectoryEntry>>,
        /// The ranked matches as `(entry index, match positions)` while a query is active.
        matches: Option<Rc<Vec<(usize, Vec<usize>)>>>,
    },
    Symbols {
        items: Arc<Vec<OutlineItem<Anchor>>>,
        rows: Rc<Vec<SymbolRow>>,
        current: Option<Range<Anchor>>,
    },
}

pub(super) struct SymbolRow {
    outline_index: usize,
    match_positions: Vec<usize>,
    indent: usize,
    /// The containing symbol, shown only when a match came from outside the level being
    /// browsed - otherwise a query like "render" is a column of identical rows.
    context: Option<SharedString>,
}

impl Default for BreadcrumbMenuRows {
    fn default() -> Self {
        Self::Directory {
            entries: Arc::default(),
            matches: None,
        }
    }
}

impl BreadcrumbMenuRows {
    fn len(&self) -> usize {
        match self {
            Self::Directory { entries, matches } => matches
                .as_ref()
                .map_or(entries.len(), |matches| matches.len()),
            Self::Symbols { rows, .. } => rows.len(),
        }
    }

    fn directory_entry(&self, index: usize) -> Option<(&BreadcrumbDirectoryEntry, &[usize])> {
        let Self::Directory { entries, matches } = self else {
            return None;
        };
        match matches {
            Some(matches) => {
                let (entry_index, positions) = matches.get(index)?;
                Some((entries.get(*entry_index)?, positions.as_slice()))
            }
            None => Some((entries.get(index)?, &[])),
        }
    }

    fn symbol(&self, index: usize) -> Option<(&OutlineItem<Anchor>, &SymbolRow)> {
        let Self::Symbols { items, rows, .. } = self else {
            return None;
        };
        let row = rows.get(index)?;
        Some((items.get(row.outline_index)?, row))
    }

    fn is_current(&self, item: &OutlineItem<Anchor>) -> bool {
        matches!(self, Self::Symbols { current: Some(current), .. } if item.range == *current)
    }

    fn shows_current_column(&self) -> bool {
        match self {
            Self::Symbols {
                items,
                rows,
                current: Some(current),
            } => rows.iter().any(|row| {
                items
                    .get(row.outline_index)
                    .is_some_and(|item| item.range == *current)
            }),
            _ => false,
        }
    }

    #[cfg(test)]
    fn labels(&self) -> Vec<SharedString> {
        (0..self.len())
            .filter_map(|index| {
                self.directory_entry(index)
                    .map(|(entry, _)| entry.name.clone())
                    .or_else(|| self.symbol(index).map(|(item, _)| item.text.clone()))
            })
            .collect()
    }
}

/// A rank in flight. The delegate's `update_matches` task awaits it so the picker's notion of
/// "the update finished" spans the rank and the publish that follows it, not just the handoff.
#[derive(Clone, Default)]
pub(super) struct FilterSettled(
    Rc<RefCell<Option<(postage::barrier::Sender, postage::barrier::Receiver)>>>,
);

impl FilterSettled {
    fn arm(&self) {
        // A superseding rank reuses the outstanding barrier. Overwriting it drops the sender,
        // which resolves a receiver already handed out and releases the picker's pending update
        // before any rows have been published for the replacement query.
        let mut slot = self.0.borrow_mut();
        if slot.is_none() {
            *slot = Some(postage::barrier::channel());
        }
    }

    #[cfg(test)]
    pub(super) fn arm_for_test(&self) {
        self.arm();
    }

    pub(super) fn settle(&self) {
        self.0.borrow_mut().take();
    }

    pub(super) fn receiver(&self) -> Option<postage::barrier::Receiver> {
        self.0
            .borrow()
            .as_ref()
            .map(|(_, receiver)| receiver.clone())
    }
}
pub(super) struct BreadcrumbPickerDelegate {
    menu: WeakEntity<BreadcrumbNavigationMenu>,
    rows: BreadcrumbMenuRows,
    selected_index: usize,
    empty_message: SharedString,
    placeholder: Arc<str>,
    truncation_note: Option<SharedString>,
    match_count_label: Option<SharedString>,
    show_current_column: bool,
    show_file_icons: bool,
    show_folder_icons: bool,
    /// The text in the query box, which reaches the menu a task later.
    query: String,
}

impl BreadcrumbPickerDelegate {
    fn new(menu: WeakEntity<BreadcrumbNavigationMenu>, placeholder: Arc<str>) -> Self {
        Self {
            menu,
            rows: BreadcrumbMenuRows::default(),
            selected_index: 0,
            empty_message: LOADING_MESSAGE.into(),
            placeholder,
            truncation_note: None,
            match_count_label: None,
            show_current_column: false,
            show_file_icons: true,
            show_folder_icons: true,
            query: String::new(),
        }
    }

    /// The query box's text after Left or Right. A switch that still has to resolve keeps it:
    /// returning something at all stops the key from also moving the caret in the box.
    fn query_after_step(&self, step: ListingStep) -> Option<String> {
        match step {
            ListingStep::Moved => Some(String::new()),
            ListingStep::Resolving => Some(self.query.clone()),
            ListingStep::Stayed => None,
        }
    }

    fn render_directory_row(
        &self,
        entry: &BreadcrumbDirectoryEntry,
        match_positions: &[usize],
        selected: bool,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let icon_path = match directory_entry_icon_source(
            entry.is_dir,
            self.show_file_icons,
            self.show_folder_icons,
        ) {
            DirectoryEntryIconSource::File => {
                file_icons::FileIcons::get_icon(entry.path.as_std_path(), cx)
            }
            DirectoryEntryIconSource::Folder => {
                file_icons::FileIcons::get_folder_icon(false, entry.path.as_std_path(), cx)
            }
            DirectoryEntryIconSource::Chevron => file_icons::FileIcons::get_chevron_icon(false, cx),
            DirectoryEntryIconSource::None => None,
        };
        let icon = icon_path
            .map(Icon::from_path)
            .map(|icon| {
                icon.color(Color::Muted)
                    .size(IconSize::Small)
                    .into_any_element()
            })
            .unwrap_or_else(|| div().size(IconSize::Small.rems()).into_any_element());
        let label_color = crate::items::entry_git_aware_label_color(
            entry.git_summary,
            entry.is_ignored,
            selected,
        );
        let label = if match_positions.is_empty() {
            Label::new(entry.name.clone())
                .color(label_color)
                .truncate_middle()
                .into_any_element()
        } else {
            ui::HighlightedLabel::new(entry.name.clone(), match_positions.to_vec())
                .color(label_color)
                .truncate_middle()
                .into_any_element()
        };
        let full_name = entry.name.clone();
        h_flex()
            .id(("breadcrumb-directory-row", entry.entry_id.to_usize()))
            .gap_1p5()
            .min_w_0()
            .child(icon)
            .child(label)
            .when(row_label_needs_tooltip(&full_name), |this| {
                this.tooltip(move |_window, cx| Tooltip::simple(full_name.clone(), cx))
            })
            .into_any_element()
    }
}

impl picker::PickerDelegate for BreadcrumbPickerDelegate {
    type ListItem = gpui::AnyElement;

    fn name() -> &'static str {
        "BreadcrumbNavigationMenu"
    }

    fn match_count(&self) -> usize {
        self.rows.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_index = ix;
        let menu = self.menu.clone();
        cx.defer(move |cx| {
            menu.update(cx, |menu, cx| menu.set_selected_row(ix, cx))
                .ok();
        });
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.placeholder.clone()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some(self.empty_message.clone())
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Task<()> {
        self.query = query.clone();
        let menu = self.menu.clone();
        cx.spawn(async move |_, cx| {
            // Handing the query over only starts the rank. The picker treats this task as the
            // whole update, so it has to span the rank and the publish that follows it -
            // otherwise Enter lands while the rows still describe the previous query.
            let settled = menu
                .update(cx, |menu, cx| {
                    menu.set_filter_query(query, cx);
                    menu.filter_settled()
                })
                .ok()
                .flatten();
            if let Some(mut settled) = settled {
                settled.recv().await;
            }
        })
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        let index = self.selected_index;
        self.menu
            .update(cx, |menu, cx| menu.confirm_row(index, window, cx))
            .ok();
    }

    fn select_child(
        &mut self,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<String> {
        let index = self.selected_index;
        let step = self
            .menu
            .update(cx, |menu, cx| menu.drill_into_selection(index, window, cx))
            .ok()?;
        self.query_after_step(step)
    }

    fn select_parent(
        &mut self,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<String> {
        let step = self
            .menu
            .update(cx, |menu, cx| menu.step_out_of_listing(window, cx))
            .ok()?;
        self.query_after_step(step)
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        self.menu.update(cx, |menu, cx| menu.emit_dismiss(cx)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let content = if let Some((entry, match_positions)) = self.rows.directory_entry(ix) {
            self.render_directory_row(entry, match_positions, selected, cx)
        } else {
            let (item, row) = self.rows.symbol(ix)?;
            let full_name = SharedString::from(flatten_text_for_single_line_display(&item.text));
            let row = render_outline_item_menu_row(
                item,
                &row.match_positions,
                self.rows.is_current(item),
                self.show_current_column,
                row.indent,
                row.context.clone(),
                window,
                cx,
            );
            h_flex()
                .id(("breadcrumb-symbol-row", ix))
                .min_w_0()
                .child(row)
                .when(row_label_needs_tooltip(&full_name), |this| {
                    this.tooltip(move |_window, cx| Tooltip::simple(full_name.clone(), cx))
                })
                .into_any_element()
        };
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ui::ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(content)
                .into_any_element(),
        )
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        // Nothing on screen otherwise says the arrow keys walk the tree, and stepping in and
        // out is the whole point of the menu.
        let focus = window.focused(cx)?;
        let hint = |action: &dyn gpui::Action, label: &'static str| {
            h_flex()
                .gap_1()
                .child(ui::KeyBinding::for_action_in(action, &focus, cx))
                .child(Label::new(label).color(Color::Muted).size(LabelSize::Small))
        };
        let keys = h_flex()
            .gap_2()
            .child(hint(&menu::SelectParent, "Out"))
            .child(hint(&menu::SelectChild, "In"))
            .child(hint(&menu::Confirm, "Open"));

        Some(
            v_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .when_some(self.truncation_note.clone(), |this, note| {
                    this.child(Label::new(note).color(Color::Muted).size(LabelSize::Small))
                })
                .child(keys)
                .into_any_element(),
        )
    }

    fn searchbar_trailer(
        &self,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        let label = self.match_count_label.clone()?;
        Some(
            Label::new(label)
                .color(Color::Muted)
                .size(LabelSize::Small)
                .into_any_element(),
        )
    }
}
