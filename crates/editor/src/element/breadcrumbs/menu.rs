use super::super::*;
use super::outline::{
    child_outline_indices, flatten_text_for_single_line_display, outline_parents,
    render_outline_item_menu_row, same_symbol_item, sibling_outline_indices,
    top_level_outline_indices,
};
use super::path::{
    BreadcrumbDirectoryEntry, BreadcrumbListingSettings, DirectoryEntryIconSource,
    MAX_BREADCRUMB_MENU_ROWS, MAX_UNARY_DIRECTORY_SKIP_DEPTH, breadcrumb_directory_entries,
    breadcrumb_directory_listing_inputs, breadcrumb_menu_truncated_label, directory_child_paths,
    directory_entry_icon_source, single_child_directory,
};
use crate::EditorEvent;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::Task;
use postage::stream::Stream as _;
use project::git_store::{GitStoreEvent, RepositoryEvent};
use settings::SettingsStore;
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use ui::utils::WithRemSize;

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
pub enum BreadcrumbListing {
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

pub struct BreadcrumbNavigationMenu {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    listing: BreadcrumbListing,
    navigated_path: Option<(WorktreeId, Arc<RelPath>)>,
    symbol_trail: Vec<OutlineItem<Anchor>>,
    active_file_path: Option<Arc<RelPath>>,
    directory_entries: Vec<BreadcrumbDirectoryEntry>,
    /// The listing `directory_entries` were loaded for. While a switch is in flight the rows on
    /// screen still describe the previous directory, and acting on one would pair its path with
    /// the new listing's worktree.
    entries_listing: Option<BreadcrumbListing>,
    /// The load a drill started, while its target is still being expanded. The picker clears the
    /// query as soon as Right is pressed, so without this the rows would repaint - unfiltered -
    /// from the directory being left before the switch even installs the new listing.
    rows_frozen_for_load: Option<u64>,
    all_symbol_items: Vec<OutlineItem<Anchor>>,
    /// Each symbol's parent outline index, cached next to `all_symbol_items` because it depends
    /// only on the outline; recomputing it per publish walked every symbol on each keystroke.
    symbol_parents: Vec<Option<usize>>,
    listed_symbol_indices: Vec<usize>,
    cursor_symbol_ranges: Vec<Range<Anchor>>,
    loading: bool,
    load_epoch: u64,
    load_task: Option<Task<()>>,
    row_refresh_task: Option<Task<()>>,
    selected_index: Option<usize>,
    pending_initial_selection: bool,
    query: String,
    rows: Rc<Vec<BreadcrumbMenuRow>>,
    rows_dirty: bool,
    scroll_to_selection_pending: bool,
    picker: Option<Entity<picker::Picker<BreadcrumbPickerDelegate>>>,
    pressed_outside: bool,
    ranked_matches: Vec<StringMatch>,
    filter_task: Option<Task<()>>,
    filter_cancel: Option<Arc<AtomicBool>>,
    filter_epoch: u64,
    ranked_epoch: u64,
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
    /// Every published row set, in order, so a test can assert what a switch or a reload put on
    /// screen between two settled states, not merely that something was there.
    #[cfg(test)]
    published_row_history: Vec<Vec<SharedString>>,
    _subscriptions: Vec<Subscription>,
    _buffer_subscription: Option<Subscription>,
}

impl BreadcrumbNavigationMenu {
    pub fn new(
        editor: WeakEntity<Editor>,
        workspace: WeakEntity<Workspace>,
        listing: BreadcrumbListing,
        active_file_path: Option<Arc<RelPath>>,
        navigated: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let menu = cx.new(|cx| {
            let navigated_path = match (&listing, navigated) {
                (BreadcrumbListing::Directory { worktree_id, path }, true) => {
                    Some((*worktree_id, path.clone()))
                }
                _ => None,
            };
            Self {
                editor,
                workspace,
                listing,
                navigated_path,
                symbol_trail: Vec::new(),
                active_file_path,
                directory_entries: Vec::new(),
                entries_listing: None,
                rows_frozen_for_load: None,
                all_symbol_items: Vec::new(),
                symbol_parents: Vec::new(),
                listed_symbol_indices: Vec::new(),
                cursor_symbol_ranges: Vec::new(),
                loading: true,
                load_epoch: 0,
                load_task: None,
                row_refresh_task: None,
                selected_index: None,
                pending_initial_selection: true,
                query: String::new(),
                rows: Rc::new(Vec::new()),
                rows_dirty: false,
                scroll_to_selection_pending: false,
                picker: None,
                pressed_outside: false,
                ranked_matches: Vec::new(),
                filter_task: None,
                filter_cancel: None,
                filter_epoch: 0,
                ranked_epoch: 0,
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
                _subscriptions: Vec::new(),
                _buffer_subscription: None,
            }
        });
        menu.update(cx, |this, cx| {
            let delegate = BreadcrumbPickerDelegate::new(
                cx.weak_entity(),
                Self::placeholder_for(&this.listing),
            );
            let picker = cx.new(|cx| {
                let available = window.viewport_size().width / window.rem_size();
                picker::Picker::uniform_list(delegate, window, cx)
                    .popover()
                    .show_scrollbar(true)
                    .initial_width(rems(available.clamp(10., 24.)))
            });
            let picker_focus = picker.focus_handle(cx);
            this._subscriptions.push(cx.on_blur(&picker_focus, window, {
                |this: &mut Self, _, cx| {
                    this.emit_dismiss(cx);
                }
            }));
            this.picker = Some(picker);
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
                    .push(cx.subscribe(&git_store, |this, _, event, cx| {
                        if !matches!(this.listing, BreadcrumbListing::Directory { .. }) {
                            return;
                        }
                        match event {
                            GitStoreEvent::RepositoryUpdated(
                                _,
                                RepositoryEvent::StatusesChanged,
                                _,
                            )
                            | GitStoreEvent::RepositoryAdded
                            | GitStoreEvent::RepositoryRemoved(_)
                            | GitStoreEvent::DiffBaseChanged(_) => this.reload_directory_rows(cx),
                            _ => {}
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
            this.reload_listing(false, window, cx);
            this.focus_menu(window, cx);
        });
        menu
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

    /// `active_file_path` is what the initial selection is resolved against, so it has to be
    /// refreshed on reuse: the editor can have been saved elsewhere since the menu was built.
    pub fn set_listing(
        &mut self,
        listing: BreadcrumbListing,
        active_file_path: Option<Arc<RelPath>>,
        navigated: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keep_previous_rows = matches!(self.listing, BreadcrumbListing::Directory { .. })
            && matches!(listing, BreadcrumbListing::Directory { .. })
            && !self.directory_entries.is_empty();
        self.active_file_path = active_file_path;
        self.pending_restore_path = None;
        self.pending_restore_symbol_range = None;
        if navigated {
            if let BreadcrumbListing::Directory { worktree_id, path } = &listing {
                self.navigated_path = Some((*worktree_id, path.clone()));
            }
        } else if let BreadcrumbListing::Directory { worktree_id, path } = &listing {
            let within_navigated_chain =
                self.navigated_path
                    .as_ref()
                    .is_some_and(|(navigated_worktree, navigated)| {
                        navigated_worktree == worktree_id && navigated.starts_with(path)
                    });
            if !within_navigated_chain {
                self.navigated_path = None;
            }
        } else {
            self.navigated_path = None;
        }
        if matches!(listing, BreadcrumbListing::Directory { .. }) {
            self.symbol_trail.clear();
            self._buffer_subscription = None;
        }
        self.listing = listing;
        self.publish_rows(cx);
        self.refresh_placeholder(window, cx);
        self.clear_filter(window, cx);
        self.pending_initial_selection = true;
        self.selected_index = None;
        self.reload_listing(keep_previous_rows, window, cx);
        self.focus_menu(window, cx);
        cx.notify();
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
    pub fn published_icon_flags(&self, cx: &App) -> Option<(bool, bool)> {
        let picker = self.picker.as_ref()?;
        let delegate = &picker.read(cx).delegate;
        Some((delegate.show_file_icons, delegate.show_folder_icons))
    }

    /// The empty-state text the picker was last handed, so a test can tell "No matches" from
    /// rows that were never replaced.
    #[cfg(test)]
    pub fn published_empty_message(&self, cx: &App) -> SharedString {
        self.picker
            .as_ref()
            .map(|picker| picker.read(cx).delegate.empty_message.clone())
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub fn published_row_labels(&self, cx: &App) -> Vec<SharedString> {
        let Some(picker) = self.picker.as_ref() else {
            return Vec::new();
        };
        picker
            .read(cx)
            .delegate
            .rows
            .iter()
            .map(|row| match row {
                BreadcrumbMenuRow::Directory { entry, .. } => entry.name.clone(),
                BreadcrumbMenuRow::Symbol { item, .. } => item.text.clone(),
            })
            .collect()
    }

    /// Each published symbol row as `(label, parent context)` - the parent name a filter shows
    /// beside a symbol that sits outside the level being browsed.
    #[cfg(test)]
    pub fn published_symbol_contexts(&self, cx: &App) -> Vec<(SharedString, Option<SharedString>)> {
        let Some(picker) = self.picker.as_ref() else {
            return Vec::new();
        };
        picker
            .read(cx)
            .delegate
            .rows
            .iter()
            .filter_map(|row| match row {
                BreadcrumbMenuRow::Symbol { item, context, .. } => {
                    Some((item.text.clone(), context.clone()))
                }
                BreadcrumbMenuRow::Directory { .. } => None,
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

    /// Drains the published rows recorded since the last call, oldest first.
    #[cfg(test)]
    pub fn take_published_row_history(&mut self) -> Vec<Vec<SharedString>> {
        std::mem::take(&mut self.published_row_history)
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
        let menu = cx.new(|cx| Self {
            editor,
            workspace: WeakEntity::new_invalid(),
            listing: BreadcrumbListing::Symbols {
                buffer_id,
                parent: None,
            },
            navigated_path: None,
            symbol_trail: Vec::new(),
            active_file_path: None,
            directory_entries: Vec::new(),
            entries_listing: None,
            rows_frozen_for_load: None,
            symbol_parents: outline_parents(
                &all_items.iter().map(|item| item.depth).collect::<Vec<_>>(),
            ),
            all_symbol_items: all_items,
            listed_symbol_indices: listed_indices,
            cursor_symbol_ranges,
            loading: false,
            load_epoch: 0,
            load_task: None,
            row_refresh_task: None,
            selected_index: None,
            pending_initial_selection: true,
            query: String::new(),
            rows: Rc::new(Vec::new()),
            rows_dirty: false,
            scroll_to_selection_pending: false,
            picker: None,
            pressed_outside: false,
            ranked_matches: Vec::new(),
            filter_task: None,
            filter_cancel: None,
            filter_epoch: 0,
            ranked_epoch: 0,
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
            _subscriptions: Vec::new(),
            _buffer_subscription: None,
        });
        menu.update(cx, |this, cx| {
            let delegate = BreadcrumbPickerDelegate::new(
                cx.weak_entity(),
                Self::placeholder_for(&this.listing),
            );
            let picker = cx.new(|cx| {
                let available = window.viewport_size().width / window.rem_size();
                picker::Picker::uniform_list(delegate, window, cx)
                    .popover()
                    .show_scrollbar(true)
                    .initial_width(rems(available.clamp(10., 24.)))
            });
            this.picker = Some(picker);
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
        self.query = query;
        self.pending_restore_path = None;
        self.pending_restore_symbol_range = None;
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
                picker.update(cx, |picker, cx| picker.set_query("", window, cx));
            }
        });
    }

    pub(super) fn set_selected_row(&mut self, position: usize, cx: &mut Context<Self>) {
        if self.selected_index != Some(position) {
            self.move_selection(Some(position), cx);
            // The picker already scrolls for selections it originates, and deliberately does
            // not for hover. Scrolling again here would drag rows under a resting cursor and
            // retrigger hover on the row that lands beneath it.
            self.scroll_to_selection_pending = false;
        }
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

    /// Reports whether the listing moved, so a typed query survives a drill that goes nowhere.
    pub(super) fn drill_into_selection(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        // Until the rank lands the rows still describe the previous query, and unlike Enter
        // this path has no pending-update contract to wait on.
        if self.rank_pending() {
            return false;
        }
        self.selected_index = Some(index);
        let (listing_before, epoch_before) = (self.listing.clone(), self.load_epoch);
        self.select_child(&menu::SelectChild, window, cx);
        // A directory drill only changes the listing once its load resolves, but it bumps the
        // load epoch immediately; opening a file dismisses instead.
        self.listing != listing_before || self.load_epoch != epoch_before || self.dismiss_emitted
    }

    /// Ungated unlike the drill: the parent comes from the listing, not from rows a rank replaces.
    pub(super) fn step_out_of_listing(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let (listing_before, epoch_before) = (self.listing.clone(), self.load_epoch);
        self.select_parent(&menu::SelectParent, window, cx);
        self.listing != listing_before || self.load_epoch != epoch_before
    }

    /// Deferred and coalesced: delegate callbacks hold the picker's lease, so publishing
    /// inline would try to update it mid-update.
    pub(super) fn publish_rows(&mut self, cx: &mut Context<Self>) {
        if self.rows_dirty {
            return;
        }
        self.rows_dirty = true;
        let menu = cx.weak_entity();
        cx.defer(move |cx| {
            menu.update(cx, |this, cx| {
                this.rows_dirty = false;
                this.publish_rows_now(cx);
            })
            .ok();
        });
    }

    fn publish_rows_now(&mut self, cx: &mut Context<Self>) {
        let Some(picker) = self.picker.clone() else {
            return;
        };
        let settings = *BreadcrumbListingSettings::get_global(cx);
        // A switch keeps what the user was reading, exactly as it was, until the new listing's
        // entries arrive: repainting from the entries being left would show that directory
        // again, and unfiltered, because the switch cleared the query.
        let switching = self.rows_frozen_for_load == Some(self.load_epoch)
            || self
                .entries_listing
                .as_ref()
                .is_some_and(|loaded| loaded != &self.listing);
        if switching && !self.rows.is_empty() {
            return;
        }
        let filter_active = !self.filter_is_empty();
        // The query editor takes the keystroke a hop before the menu hears about it, so until
        // the text and the rank both catch up the rows and the count describe the previous query.
        let visible_query_settled = !filter_active || picker.read(cx).query(cx) == self.query;
        let rank_settled = !filter_active || (!self.rank_pending() && visible_query_settled);
        // Nothing ranked yet for the query on screen: keep the rows the user is reading instead
        // of blanking to "Searching…" and back. Only while the rank is still pending and has a
        // task behind it - a settled rank with no matches has to publish "No matches", and a
        // bumped epoch with no rank behind it must not swallow every later publish. A load is
        // exempt because it replaces the items the rows resolve through.
        if filter_active
            && self.rank_pending()
            && self.filter_task.is_some()
            && !self.loading
            && self.ranked_matches.is_empty()
            && !self.rows.is_empty()
        {
            return;
        }
        let visible = self.visible_row_count();
        let deepest_current = self.deepest_cursor_symbol_range().cloned();
        let is_directory = matches!(self.listing, BreadcrumbListing::Directory { .. });

        let mut rows = Vec::with_capacity(visible);
        for position in 0..visible {
            if is_directory {
                if let Some((entry, match_positions)) = self.directory_row_data(position) {
                    rows.push(BreadcrumbMenuRow::Directory {
                        entry,
                        match_positions,
                    });
                }
            } else if let Some((item, outline_index, match_positions)) =
                self.symbol_row_data(position)
            {
                let is_current = deepest_current
                    .as_ref()
                    .is_some_and(|range| item.range == *range);
                rows.push(BreadcrumbMenuRow::Symbol {
                    item,
                    outline_index,
                    match_positions,
                    is_current,
                    indent: 0,
                    context: None,
                });
            }
        }
        // A filter reaches symbols at any depth, so each row needs to say where it sits:
        // indent relative to the shallowest row, and name the parent for anything that is not
        // part of the level being browsed.
        if filter_active {
            let shallowest = rows
                .iter()
                .filter_map(|row| match row {
                    BreadcrumbMenuRow::Symbol { item, .. } => Some(item.depth),
                    BreadcrumbMenuRow::Directory { .. } => None,
                })
                .min()
                .unwrap_or(0);
            for row in &mut rows {
                if let BreadcrumbMenuRow::Symbol {
                    item,
                    outline_index,
                    indent,
                    context,
                    ..
                } = row
                {
                    *indent = item.depth.saturating_sub(shallowest);
                    // `listed_symbol_indices` is built in outline order, so a binary search
                    // stands in for the linear scan this ran per row, per publish.
                    if self
                        .listed_symbol_indices
                        .binary_search(outline_index)
                        .is_err()
                    {
                        *context = self
                            .symbol_parents
                            .get(*outline_index)
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
        }

        let show_current_column = rows.iter().any(|row| match row {
            BreadcrumbMenuRow::Symbol { is_current, .. } => *is_current,
            BreadcrumbMenuRow::Directory { .. } => false,
        });

        let placeholder = Self::placeholder_for(&self.listing);

        let empty_message: SharedString = if self.loading {
            "Loading…".into()
        } else if filter_active {
            if self.ranked_epoch != self.filter_epoch && self.ranked_matches.is_empty() {
                "Searching…".into()
            } else {
                "No matches".into()
            }
        } else if is_directory {
            "Empty directory".into()
        } else {
            "No symbols".into()
        };

        let truncation_note = self
            .is_display_truncated()
            .then(|| SharedString::from(breadcrumb_menu_truncated_label(filter_active)));

        let match_count_label = (filter_active && rank_settled).then(|| -> SharedString {
            if self.filter_match_truncated {
                format!("{}+ matches", MAX_BREADCRUMB_MENU_ROWS).into()
            } else if self.ranked_matches.len() == 1 {
                "1 match".into()
            } else {
                format!("{} matches", self.ranked_matches.len()).into()
            }
        });

        let selected_index = self
            .selected_index
            .unwrap_or(0)
            .min(rows.len().saturating_sub(1));
        let scroll_to_selection = std::mem::take(&mut self.scroll_to_selection_pending);
        #[cfg(test)]
        self.published_row_history.push(
            rows.iter()
                .map(|row| match row {
                    BreadcrumbMenuRow::Directory { entry, .. } => entry.name.clone(),
                    BreadcrumbMenuRow::Symbol { item, .. } => item.text.clone(),
                })
                .collect(),
        );
        let rows = Rc::new(rows);
        self.rows = rows.clone();
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

    fn emit_bar_changed(&self, cx: &mut Context<Self>) {
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |_, cx| {
                cx.emit(EditorEvent::BreadcrumbsChanged);
            });
        }
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

    fn reload_listing(
        &mut self,
        keep_previous_rows: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.load_epoch = self.load_epoch.wrapping_add(1);
        let epoch = self.load_epoch;
        // A directory-to-directory switch keeps the rows it is leaving on screen until the new
        // listing loads, so navigating never blanks to "Loading…"; the first open and every
        // cross-kind switch still clear, because there is nothing to keep.
        self.loading = true;
        self.rows_frozen_for_load = None;
        if !keep_previous_rows {
            self.directory_entries.clear();
            self.entries_listing = None;
            if matches!(self.listing, BreadcrumbListing::Directory { .. }) {
                self.all_symbol_items.clear();
                self.symbol_parents.clear();
                self.listed_symbol_indices.clear();
            }
            // The candidates index whatever the rows are drawn from, so they go with it. A
            // symbol listing that keeps its outline keeps them: `spawn_symbols_load` re-windows
            // the loaded items without rebuilding either, so clearing here would leave the new
            // level searchable against nothing and answer every query with no matches.
            if self.all_symbol_items.is_empty() {
                self.filter_candidates = Arc::new(Vec::new());
            }
        }
        cx.notify();

        match self.listing.clone() {
            BreadcrumbListing::Directory { worktree_id, path } => {
                self.spawn_directory_load(worktree_id, path, epoch, cx);
            }
            BreadcrumbListing::Symbols { buffer_id, parent } => {
                self.spawn_symbols_load(buffer_id, parent, epoch, window, cx);
            }
        }
    }

    /// Every dismissal path funnels here, and only the first one through emits.
    fn emit_dismiss(&mut self, cx: &mut Context<Self>) {
        if !std::mem::replace(&mut self.dismiss_emitted, true) {
            cx.emit(DismissEvent);
        }
    }

    /// Takes the in-flight loads with it: a listing switch that resolves after the dismiss
    /// would otherwise set a listing whose path is gone.
    fn dismiss_dead_listing(&mut self, cx: &mut Context<Self>) {
        self.load_epoch = self.load_epoch.wrapping_add(1);
        self.load_task = None;
        self.row_refresh_task = None;
        self.emit_dismiss(cx);
    }

    fn reload_directory_rows(&mut self, cx: &mut Context<Self>) {
        let BreadcrumbListing::Directory { worktree_id, path } = &self.listing else {
            return;
        };
        #[cfg(test)]
        {
            self.directory_reload_count = self.directory_reload_count.wrapping_add(1);
        }
        let (worktree_id, path) = (*worktree_id, path.clone());
        let filter_active = !self.filter_is_empty();
        let selected_path = self.selected_index.and_then(|position| {
            if filter_active {
                let match_ = self.ranked_matches.get(position)?;
                self.directory_entries
                    .get(match_.candidate_id)
                    .map(|entry| entry.path.clone())
            } else {
                self.directory_entries
                    .get(position)
                    .map(|entry| entry.path.clone())
            }
        });
        let Some((worktree, project)) = self.worktree(worktree_id, cx).zip(self.project(cx)) else {
            self.directory_entries.clear();
            self.entries_listing = None;
            self.publish_rows(cx);
            cx.notify();
            return;
        };
        let inputs = breadcrumb_directory_listing_inputs(&project, &worktree, cx);
        // Deliberately does not take `load_task` or bump `load_epoch`: a refresh must never
        // cancel a listing switch that is still resolving. It captures the epoch it observed,
        // so a switch starting meanwhile discards this result instead.
        let epoch = self.load_epoch;
        let listing = self.listing.clone();
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
                this.directory_entries = entries;
                this.entries_listing = Some(this.listing.clone());
                this.filter_candidates = Arc::new(candidates);
                // `loading` stays owned by the listing load. Expanding an unscanned directory
                // emits the very entry updates this refresh listens for, so a refresh lands
                // mid-load with a partially scanned worktree; clearing the flag here would let
                // it spend the one-shot initial selection on that partial listing and leave the
                // open file unselected once the full one arrives.
                this.publish_rows(cx);
                this.apply_reloaded_selection(selected_path, cx);
                cx.notify();
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
            let display_count = self.directory_entries.len().min(MAX_BREADCRUMB_MENU_ROWS);
            self.selected_index = self
                .directory_entries
                .iter()
                .take(display_count)
                .position(|entry| entry.path.as_ref() == selected_path.as_ref());
            if self.selected_index.is_none() {
                self.pending_initial_selection = true;
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

    fn spawn_directory_load(
        &mut self,
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        epoch: u64,
        cx: &mut Context<Self>,
    ) {
        let expand_task = self
            .worktree(worktree_id, cx)
            .and_then(|worktree| {
                worktree
                    .read(cx)
                    .entry_for_path(&path)
                    .map(|entry| entry.id)
            })
            .and_then(|entry_id| {
                let project = self.project(cx)?;
                project.update(cx, |project, cx| {
                    project.expand_entry(worktree_id, entry_id, cx)
                })
            });

        self.load_task = Some(cx.spawn(async move |this, cx| {
            if let Some(task) = expand_task {
                task.await.log_err();
            }
            let inputs = this
                .update(cx, |this, cx| {
                    if this.load_epoch != epoch {
                        return None;
                    }
                    let (worktree, project) =
                        this.worktree(worktree_id, cx).zip(this.project(cx))?;
                    Some(breadcrumb_directory_listing_inputs(&project, &worktree, cx))
                })
                .ok()
                .flatten();
            let (entries, candidates) = match inputs {
                Some(inputs) => {
                    let path = path.clone();
                    cx.background_spawn(async move {
                        let entries = breadcrumb_directory_entries(&inputs, &path);
                        let candidates = directory_filter_candidates(&entries);
                        (entries, candidates)
                    })
                    .await
                }
                // Renders identically to a genuinely empty directory, so say so in the log.
                None => {
                    log::warn!("breadcrumb listing found no worktree or project for {path:?}");
                    (Vec::new(), Vec::new())
                }
            };
            this.update(cx, |this, cx| {
                if this.load_epoch != epoch {
                    return;
                }
                this.directory_entries = entries;
                this.entries_listing = Some(this.listing.clone());
                this.filter_candidates = Arc::new(candidates);
                this.loading = false;
                this.publish_rows(cx);
                if this.filter_is_empty() {
                    this.apply_initial_selection_if_needed(cx);
                } else {
                    this.ranked_matches.clear();
                    this.selected_index = None;
                    this.rerank_filter(cx);
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn spawn_symbols_load(
        &mut self,
        buffer_id: BufferId,
        parent: Option<OutlineItem<Anchor>>,
        epoch: u64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.subscribe_listed_buffer(buffer_id, cx);

        if !self.all_symbol_items.is_empty() {
            self.apply_symbol_parent(parent, cx);
            self.loading = false;
            self.apply_initial_selection_if_needed(cx);
            cx.notify();
            self.load_task = None;
            return;
        }

        self.load_task = Some(cx.spawn_in(window, async move |this, cx| {
            let outline_task = this
                .update(cx, |this, cx| {
                    this.editor.upgrade().map(|editor| {
                        editor.update(cx, |editor, cx| editor.buffer_outline_items(buffer_id, cx))
                    })
                })
                .ok()
                .flatten();
            let Some(outline_task) = outline_task else {
                this.update(cx, |this, cx| {
                    this.loading = false;
                    cx.notify();
                })
                .ok();
                return;
            };
            let text_items = outline_task.await;
            this.update_in(cx, |this, window, cx| {
                if this.load_epoch != epoch {
                    return;
                }
                if !this.apply_loaded_outline(buffer_id, &text_items, parent.clone(), cx) {
                    return;
                }
                // A file with no symbols at all has nothing to show, so hand over to the
                // outline picker rather than opening an empty menu.
                if this.all_symbol_items.is_empty()
                    && parent.is_none()
                    && let Some(editor) = this.editor.upgrade()
                    && let Some(callback) = zed_actions::outline::TOGGLE_OUTLINE.get()
                {
                    callback(editor.to_any_view(), window, cx);
                    this.emit_dismiss(cx);
                }
            })
            .ok();
        }));
    }

    fn apply_loaded_outline(
        &mut self,
        buffer_id: BufferId,
        text_items: &[OutlineItem<text::Anchor>],
        parent: Option<OutlineItem<Anchor>>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(editor) = self.editor.upgrade() else {
            self.loading = false;
            cx.notify();
            return false;
        };
        let (all_items, cursor_ranges) = editor.update(cx, |editor, cx| {
            let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            let all_items = editor.map_text_outline_items(text_items, &multi_buffer_snapshot);
            let cursor_ranges = editor
                .outline_symbols_at_cursor
                .as_ref()
                .filter(|(id, _)| *id == buffer_id)
                .map(|(_, ancestors)| ancestors.iter().map(|item| item.range.clone()).collect())
                .unwrap_or_default();
            (all_items, cursor_ranges)
        });
        self.cursor_symbol_ranges = cursor_ranges;
        self.all_symbol_items = all_items;
        // Both derived here, with the outline: every Left/Right below only re-windows the same
        // items, so rebuilding the candidates or the parents there would redo the whole set for
        // nothing - and publish, which runs per keystroke, reads the parents rather than walking
        // the outline itself.
        self.filter_candidates = Arc::new(symbol_filter_candidates(&self.all_symbol_items));
        self.symbol_parents = outline_parents(
            &self
                .all_symbol_items
                .iter()
                .map(|item| item.depth)
                .collect::<Vec<_>>(),
        );
        self.loading = false;
        self.publish_rows(cx);
        self.apply_symbol_parent(parent, cx);
        if self.filter_is_empty() {
            // Nothing ranks an empty query, so the epoch a reload bumped has to be retired
            // here. Left behind it would hold `rank_pending` on for the rest of the menu's
            // life, and the drill reads that as a rank still owed and refuses to move.
            self.ranked_epoch = self.filter_epoch;
            self.apply_initial_selection_if_needed(cx);
        } else {
            self.ranked_matches.clear();
            self.selected_index = None;
            self.rerank_filter(cx);
        }
        self.emit_bar_changed(cx);
        cx.notify();
        true
    }

    fn reload_symbols_from_buffer(
        &mut self,
        buffer_id: BufferId,
        parent: Option<OutlineItem<Anchor>>,
        cx: &mut Context<Self>,
    ) {
        // Latched here, not after the reload: the lines below blank the rows this reads from,
        // so a second edit landing mid-reload would find nothing to name the row with. Only
        // once the user has moved the highlight - before that, `apply_loaded_outline` keeps
        // re-picking the row for the cursor.
        if !self.pending_initial_selection && self.pending_restore_symbol_range.is_none() {
            self.pending_restore_symbol_range = match self.selected_row() {
                Some(BreadcrumbMenuRow::Symbol { item, .. }) => Some(item.range.clone()),
                _ => None,
            };
        }
        // `symbol_trail` is deliberately kept: it is what the bar paints the menu's anchor
        // segment from, and a frame without an anchor dismisses the menu.
        self.all_symbol_items.clear();
        self.symbol_parents.clear();
        self.listed_symbol_indices.clear();
        // Candidates and matches are outline indices into `all_symbol_items`, so keeping them
        // past the clear would publish a match count over rows that can no longer resolve. The
        // directory path gets this for free by swapping its entries and reranking in one
        // synchronous block; this one is split across an await.
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
            let outline_task = this
                .update(cx, |this, cx| {
                    this.editor.upgrade().map(|editor| {
                        editor.update(cx, |editor, cx| editor.buffer_outline_items(buffer_id, cx))
                    })
                })
                .ok()
                .flatten();
            let Some(outline_task) = outline_task else {
                this.update(cx, |this, cx| {
                    this.loading = false;
                    cx.notify();
                })
                .ok();
                return;
            };
            let text_items = outline_task.await;
            this.update(cx, |this, cx| {
                if this.load_epoch != epoch {
                    return;
                }
                if !this.apply_loaded_outline(buffer_id, &text_items, parent, cx) {
                    return;
                }
                this.restore_symbol_selection(cx);
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
        let display_count = self
            .listed_symbol_indices
            .len()
            .min(MAX_BREADCRUMB_MENU_ROWS);
        let all_symbol_items = &self.all_symbol_items;
        self.selected_index = self
            .listed_symbol_indices
            .iter()
            .take(display_count)
            .position(|index| {
                all_symbol_items
                    .get(*index)
                    .is_some_and(|item| item.range == selected_symbol_range)
            });
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

    fn apply_symbol_parent(
        &mut self,
        mut parent: Option<OutlineItem<Anchor>>,
        cx: &mut Context<Self>,
    ) {
        let depths: Vec<usize> = self
            .all_symbol_items
            .iter()
            .map(|item| item.depth)
            .collect();
        let parent_index = parent.as_ref().map(|parent_item| {
            self.all_symbol_items
                .iter()
                .position(|item| item.range == parent_item.range)
        });
        let listed_indices = match parent_index {
            Some(Some(parent_index)) => {
                let children = child_outline_indices(&depths, parent_index);
                if children.is_empty() {
                    sibling_outline_indices(&depths, parent_index)
                } else {
                    children
                }
            }
            // An edit removed the symbol this listing was opened on; an empty listing
            // here would have no keyboard way back out.
            Some(None) => {
                parent = None;
                top_level_outline_indices(&depths)
            }
            None => top_level_outline_indices(&depths),
        };

        let buffer_id = match &self.listing {
            BreadcrumbListing::Symbols { buffer_id, .. } => *buffer_id,
            _ => return,
        };
        self.listing = BreadcrumbListing::Symbols { buffer_id, parent };
        self.listed_symbol_indices = listed_indices;
        // Candidates are the whole outline, unchanged by re-windowing to this parent's level.
        self.rebuild_symbol_trail();
        self.publish_rows(cx);
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
        let depths: Vec<usize> = self
            .all_symbol_items
            .iter()
            .map(|item| item.depth)
            .collect();
        let parents = outline_parents(&depths);
        let Some(mut index) = self
            .all_symbol_items
            .iter()
            .position(|item| item.range == parent.range)
        else {
            self.symbol_trail = vec![parent.clone()];
            return;
        };
        let mut trail = vec![self.all_symbol_items[index].clone()];
        while let Some(parent_index) = parents.get(index).copied().flatten() {
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
            let unranked_selection = match (&self.listing, selected_candidate) {
                (BreadcrumbListing::Symbols { .. }, Some(outline_index)) => self
                    .listed_symbol_indices
                    .iter()
                    .position(|listed| *listed == outline_index),
                (_, candidate) => candidate,
            };
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
            // Ranked in full and truncated below: `match_strings` truncates on its own
            // comparator, which on equal scores keeps the highest candidate ids - the last
            // files of the listing - while the cap promises the first ones.
            let matches = fuzzy::match_strings(
                candidates.as_slice(),
                &query,
                false,
                true,
                usize::MAX,
                &cancel_flag,
                executor,
            )
            .await;
            this.update(cx, |this, cx| {
                if this.filter_epoch != epoch {
                    return;
                }
                this.ranked_epoch = epoch;
                this.filter_match_truncated = matches.len() > MAX_BREADCRUMB_MENU_ROWS;
                let mut matches = matches;
                // Equal scores otherwise come back in reverse listing order, so filtering a
                // directory of item_000..item_199 opened on item_199.
                matches.sort_by(|a, b| {
                    b.score
                        .partial_cmp(&a.score)
                        .unwrap_or(Ordering::Equal)
                        .then(a.candidate_id.cmp(&b.candidate_id))
                });
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
            BreadcrumbListing::Directory { .. } => {
                self.directory_entries.len().min(MAX_BREADCRUMB_MENU_ROWS)
            }
            BreadcrumbListing::Symbols { .. } => self
                .listed_symbol_indices
                .len()
                .min(MAX_BREADCRUMB_MENU_ROWS),
        }
    }

    #[cfg(test)]
    fn visible_row_labels(&self) -> Vec<SharedString> {
        if !self.filter_is_empty() {
            return self
                .ranked_matches
                .iter()
                .take(MAX_BREADCRUMB_MENU_ROWS)
                .map(|match_| match_.string.clone().into())
                .collect();
        }
        match &self.listing {
            BreadcrumbListing::Directory { .. } => self
                .directory_entries
                .iter()
                .take(MAX_BREADCRUMB_MENU_ROWS)
                .map(|entry| entry.name.clone())
                .collect(),
            BreadcrumbListing::Symbols { .. } => self
                .listed_symbol_indices
                .iter()
                .take(MAX_BREADCRUMB_MENU_ROWS)
                .filter_map(|&index| {
                    self.all_symbol_items
                        .get(index)
                        .map(|item| item.text.clone())
                })
                .collect(),
        }
    }

    fn is_display_truncated(&self) -> bool {
        if !self.filter_is_empty() {
            return self.ranked_epoch == self.filter_epoch && self.filter_match_truncated;
        }
        match &self.listing {
            BreadcrumbListing::Directory { .. } => {
                self.directory_entries.len() > MAX_BREADCRUMB_MENU_ROWS
            }
            BreadcrumbListing::Symbols { .. } => {
                self.listed_symbol_indices.len() > MAX_BREADCRUMB_MENU_ROWS
            }
        }
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

    fn initial_selected_index(&self) -> Option<usize> {
        match &self.listing {
            BreadcrumbListing::Directory { .. } => {
                let display_count = self.directory_entries.len().min(MAX_BREADCRUMB_MENU_ROWS);
                let Some(active_path) = self.active_file_path.as_ref() else {
                    return (display_count > 0).then_some(0);
                };
                if let Some(index) = self
                    .directory_entries
                    .iter()
                    .take(display_count)
                    .position(|entry| entry.path.as_ref() == active_path.as_ref())
                {
                    return Some(index);
                }
                if let Some(index) = self
                    .directory_entries
                    .iter()
                    .take(display_count)
                    .position(|entry| entry.is_dir && active_path.starts_with(entry.path.as_ref()))
                {
                    return Some(index);
                }
                (display_count > 0).then_some(0)
            }
            BreadcrumbListing::Symbols { .. } => {
                let display_count = self
                    .listed_symbol_indices
                    .len()
                    .min(MAX_BREADCRUMB_MENU_ROWS);
                if let Some(position) = self
                    .listed_symbol_indices
                    .iter()
                    .take(display_count)
                    .position(|&index| {
                        self.all_symbol_items
                            .get(index)
                            .is_some_and(|item| self.cursor_symbol_ranges.contains(&item.range))
                    })
                {
                    return Some(position);
                }
                Some(0)
            }
        }
    }

    fn move_selection(&mut self, position: Option<usize>, cx: &mut Context<Self>) {
        self.pending_initial_selection = false;
        self.selected_index = position;
        self.scroll_to_selection_pending = true;
        self.publish_rows(cx);
        cx.notify();
    }

    pub(super) fn confirm(
        &mut self,
        _: &menu::Confirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.listing.clone() {
            BreadcrumbListing::Directory { .. } => {
                let Some(entry) = self.selected_directory_entry() else {
                    return;
                };
                self.choose_directory_entry(entry, window, cx);
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
                    self.choose_directory_entry(entry, window, cx);
                } else if self
                    .active_file_path
                    .as_ref()
                    .is_some_and(|path| path.as_ref() == entry.path.as_ref())
                {
                    let Some(editor) = self.editor.upgrade() else {
                        return;
                    };
                    let Some(buffer_id) = editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .map(|buffer| buffer.read(cx).remote_id())
                    else {
                        return;
                    };
                    self.set_listing(
                        BreadcrumbListing::Symbols {
                            buffer_id,
                            parent: None,
                        },
                        self.active_file_path.clone(),
                        false,
                        window,
                        cx,
                    );
                    self.emit_bar_changed(cx);
                } else {
                    self.choose_directory_entry(entry, window, cx);
                }
            }
            BreadcrumbListing::Symbols { .. } => {
                let Some(outline_index) = self.selected_symbol_outline_index() else {
                    return;
                };
                let depths: Vec<usize> = self
                    .all_symbol_items
                    .iter()
                    .map(|item| item.depth)
                    .collect();
                let children = child_outline_indices(&depths, outline_index);
                if children.is_empty() {
                    return;
                }
                let parent = self.all_symbol_items.get(outline_index).cloned();
                let buffer_id = match &self.listing {
                    BreadcrumbListing::Symbols { buffer_id, .. } => *buffer_id,
                    _ => return,
                };
                self.transition_to_symbol_listing(buffer_id, parent, children, window, cx);
            }
        }
    }

    fn transition_to_symbol_listing(
        &mut self,
        buffer_id: BufferId,
        parent: Option<OutlineItem<Anchor>>,
        listed_indices: Vec<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear_filter(window, cx);
        self.pending_initial_selection = true;
        self.selected_index = None;
        self.listing = BreadcrumbListing::Symbols { buffer_id, parent };
        self.listed_symbol_indices = listed_indices;
        // Candidates are the whole outline, unchanged by re-windowing to this parent's level.
        self.rebuild_symbol_trail();
        self.apply_initial_selection_if_needed(cx);
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
        match self.listing.clone() {
            BreadcrumbListing::Directory { worktree_id, path } => {
                let Some(parent) = path.parent() else {
                    return;
                };
                let parent = parent.into_arc();
                self.set_listing(
                    BreadcrumbListing::Directory {
                        worktree_id,
                        path: parent,
                    },
                    self.active_file_path.clone(),
                    true,
                    window,
                    cx,
                );
                self.emit_bar_changed(cx);
            }
            BreadcrumbListing::Symbols { buffer_id, parent } => {
                if parent.is_none() {
                    self.switch_to_file_parent_directory(window, cx);
                    return;
                }
                let Some(&first) = self.listed_symbol_indices.first() else {
                    return;
                };
                let depths: Vec<usize> = self
                    .all_symbol_items
                    .iter()
                    .map(|item| item.depth)
                    .collect();
                let parents = outline_parents(&depths);
                let Some(parent_index) = parents.get(first).copied().flatten() else {
                    let top_level = top_level_outline_indices(&depths);
                    self.transition_to_symbol_listing(buffer_id, None, top_level, window, cx);
                    return;
                };
                let siblings = sibling_outline_indices(&depths, parent_index);
                let new_parent = parents
                    .get(parent_index)
                    .copied()
                    .flatten()
                    .and_then(|index| self.all_symbol_items.get(index).cloned());
                self.transition_to_symbol_listing(buffer_id, new_parent, siblings, window, cx);
            }
        }
    }

    fn switch_to_file_parent_directory(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.editor.upgrade() else {
            return;
        };
        let Some(project_path) = editor.read(cx).active_project_path(cx) else {
            return;
        };
        // A single-file worktree paints no directory segments, so a directory listing here
        // would have nothing to anchor to and the menu would dismiss itself.
        let is_single_file = editor
            .read(cx)
            .project()
            .and_then(|project| {
                project
                    .read(cx)
                    .worktree_for_id(project_path.worktree_id, cx)
            })
            .is_some_and(|worktree| worktree.read(cx).is_single_file());
        if is_single_file {
            return;
        }
        let parent_path = project_path
            .path
            .parent()
            .map(|parent| parent.into_arc())
            .unwrap_or_else(|| RelPath::empty().into_arc());
        self.set_listing(
            BreadcrumbListing::Directory {
                worktree_id: project_path.worktree_id,
                path: parent_path,
            },
            Some(project_path.path),
            false,
            window,
            cx,
        );
        self.emit_bar_changed(cx);
    }

    /// Resolved from the row the picker rendered rather than re-derived from the selection,
    /// so the two index spaces can never disagree silently.
    fn selected_row(&self) -> Option<&BreadcrumbMenuRow> {
        self.rows.get(self.selected_index?)
    }

    fn selected_directory_entry(&self) -> Option<BreadcrumbDirectoryEntry> {
        if self.entries_listing.as_ref() != Some(&self.listing) {
            return None;
        }
        match self.selected_row()? {
            BreadcrumbMenuRow::Directory { entry, .. } => Some(entry.clone()),
            BreadcrumbMenuRow::Symbol { .. } => None,
        }
    }

    fn selected_symbol_item(&self) -> Option<OutlineItem<Anchor>> {
        match self.selected_row()? {
            BreadcrumbMenuRow::Symbol { item, .. } => Some(item.clone()),
            BreadcrumbMenuRow::Directory { .. } => None,
        }
    }

    fn selected_symbol_outline_index(&self) -> Option<usize> {
        match self.selected_row()? {
            BreadcrumbMenuRow::Symbol { outline_index, .. } => Some(*outline_index),
            BreadcrumbMenuRow::Directory { .. } => None,
        }
    }

    fn choose_directory_entry(
        &mut self,
        entry: BreadcrumbDirectoryEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !entry.is_dir {
            self.open_file(entry.path, window, cx);
            return;
        }

        let worktree_id = match &self.listing {
            BreadcrumbListing::Directory { worktree_id, .. } => *worktree_id,
            _ => return,
        };
        let auto_fold_dirs = BreadcrumbListingSettings::get_global(cx).auto_fold_dirs;
        self.load_epoch = self.load_epoch.wrapping_add(1);
        let generation = self.load_epoch;
        self.rows_frozen_for_load = Some(generation);
        let expand_task = self.project(cx).and_then(|project| {
            project.update(cx, |project, cx| {
                project.expand_entry(worktree_id, entry.entry_id, cx)
            })
        });
        self.load_task = Some(cx.spawn_in(window, async move |this, cx| {
            if let Some(task) = expand_task {
                task.await.log_err();
            }

            let mut path = entry.path;
            if auto_fold_dirs {
                for _ in 0..MAX_UNARY_DIRECTORY_SKIP_DEPTH {
                    let still_current = this
                        .update(cx, |this, _| this.load_epoch == generation)
                        .unwrap_or(false);
                    if !still_current {
                        return;
                    }
                    let step = this
                        .update(cx, |this, cx| {
                            let worktree = this.worktree(worktree_id, cx)?;
                            let children = directory_child_paths(&worktree, &path, 2, cx);
                            let child_path = single_child_directory(&children)?;
                            let child_id = worktree
                                .read(cx)
                                .entry_for_path(&child_path)
                                .map(|entry| entry.id)?;
                            let task = this.project(cx).and_then(|project| {
                                project.update(cx, |project, cx| {
                                    project.expand_entry(worktree_id, child_id, cx)
                                })
                            });
                            Some((child_path, task))
                        })
                        .ok()
                        .flatten();
                    let Some((child_path, task)) = step else {
                        break;
                    };
                    if let Some(task) = task {
                        task.await.log_err();
                    }
                    path = child_path;
                }
            }

            this.update_in(cx, |this, window, cx| {
                if this.load_epoch != generation {
                    return;
                }
                // The removal that takes this path names it while the listing we are leaving is
                // still installed, so it classifies as an ordinary reload there and leaves this
                // drill running. The epoch alone cannot tell us the target is gone.
                let target_survives = this.worktree(worktree_id, cx).is_some_and(|worktree| {
                    worktree
                        .read(cx)
                        .entry_for_path(&path)
                        .is_some_and(|entry| entry.is_dir())
                });
                if !target_survives {
                    this.dismiss_dead_listing(cx);
                    return;
                }
                let active_file_path = this.active_file_path.clone();
                this.set_listing(
                    BreadcrumbListing::Directory { worktree_id, path },
                    active_file_path,
                    true,
                    window,
                    cx,
                );
                this.emit_bar_changed(cx);
            })
            .ok();
        }));
    }

    fn open_file(&mut self, path: Arc<RelPath>, window: &mut Window, cx: &mut Context<Self>) {
        let worktree_id = match &self.listing {
            BreadcrumbListing::Directory { worktree_id, .. } => *worktree_id,
            _ => {
                self.emit_dismiss(cx);
                return;
            }
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
    /// which retargets the menu instead of closing it.
    fn dismiss_after_release_outside(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let listing = self.listing.clone();
        cx.defer_in(window, move |this, _window, cx| {
            if this.listing == listing {
                this.emit_dismiss(cx);
            }
        });
    }

    fn deepest_cursor_symbol_range(&self) -> Option<&Range<Anchor>> {
        self.cursor_symbol_ranges.last()
    }

    fn directory_row_data(
        &self,
        position: usize,
    ) -> Option<(BreadcrumbDirectoryEntry, Vec<usize>)> {
        if !self.filter_is_empty() {
            let match_ = self.ranked_matches.get(position)?;
            let entry = self.directory_entries.get(match_.candidate_id)?.clone();
            return Some((entry, match_.positions.clone()));
        }
        let entry = self.directory_entries.get(position)?.clone();
        Some((entry, Vec::new()))
    }

    fn symbol_row_data(&self, position: usize) -> Option<(OutlineItem<Anchor>, usize, Vec<usize>)> {
        if !self.filter_is_empty() {
            let match_ = self.ranked_matches.get(position)?;
            let outline_index = match_.candidate_id;
            let item = self.all_symbol_items.get(outline_index)?.clone();
            return Some((item, outline_index, match_.positions.clone()));
        }
        let outline_index = *self.listed_symbol_indices.get(position)?;
        let item = self.all_symbol_items.get(outline_index)?.clone();
        Some((item, outline_index, Vec::new()))
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

/// The delegate owns these: `PickerDelegate` renders from `&self` while the menu is itself
/// mid-render, so it cannot read the menu back.
#[derive(Clone)]
pub(super) enum BreadcrumbMenuRow {
    Directory {
        entry: BreadcrumbDirectoryEntry,
        match_positions: Vec<usize>,
    },
    Symbol {
        item: OutlineItem<Anchor>,
        outline_index: usize,
        match_positions: Vec<usize>,
        is_current: bool,
        indent: usize,
        /// The containing symbol, shown only when a match came from outside the level being
        /// browsed - otherwise a query like "render" is a column of identical rows.
        context: Option<SharedString>,
    },
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
    rows: Rc<Vec<BreadcrumbMenuRow>>,
    selected_index: usize,
    empty_message: SharedString,
    placeholder: Arc<str>,
    truncation_note: Option<SharedString>,
    match_count_label: Option<SharedString>,
    show_current_column: bool,
    show_file_icons: bool,
    show_folder_icons: bool,
}

impl BreadcrumbPickerDelegate {
    fn new(menu: WeakEntity<BreadcrumbNavigationMenu>, placeholder: Arc<str>) -> Self {
        Self {
            menu,
            rows: Rc::new(Vec::new()),
            selected_index: 0,
            empty_message: "Loading…".into(),
            placeholder,
            truncation_note: None,
            match_count_label: None,
            show_current_column: false,
            show_file_icons: true,
            show_folder_icons: true,
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
        let drilled = self
            .menu
            .update(cx, |menu, cx| menu.drill_into_selection(index, window, cx))
            .ok()?;
        drilled.then(String::new)
    }

    fn select_parent(
        &mut self,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<String> {
        let stepped_out = self
            .menu
            .update(cx, |menu, cx| menu.step_out_of_listing(window, cx))
            .ok()?;
        stepped_out.then(String::new)
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
        let row = self.rows.get(ix)?;
        let content = match row {
            BreadcrumbMenuRow::Directory {
                entry,
                match_positions,
            } => self.render_directory_row(entry, match_positions, selected, cx),
            BreadcrumbMenuRow::Symbol {
                item,
                match_positions,
                is_current,
                indent,
                context,
                ..
            } => {
                let full_name =
                    SharedString::from(flatten_text_for_single_line_display(&item.text));
                let row = render_outline_item_menu_row(
                    item,
                    match_positions,
                    *is_current,
                    self.show_current_column,
                    *indent,
                    context.clone(),
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
            }
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
