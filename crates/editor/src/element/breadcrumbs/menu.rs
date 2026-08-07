//! Single async-populating breadcrumb navigation menu (directory + symbols).

use super::super::*;
use super::outline::{
    child_outline_indices, flatten_text_for_single_line_display, outline_parents,
    render_outline_item_menu_row, sibling_outline_indices, top_level_outline_indices,
};
use super::path::{
    BreadcrumbDirectoryEntry, BreadcrumbListingSettings, DirectoryEntryIconSource,
    MAX_BREADCRUMB_MENU_ROWS, MAX_UNARY_DIRECTORY_SKIP_DEPTH, breadcrumb_directory_entries,
    breadcrumb_menu_truncated_label, directory_child_paths, directory_entry_icon_source,
    single_child_directory,
};
use crate::EditorEvent;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{ListSizingBehavior, ScrollStrategy, Task, UniformListScrollHandle, uniform_list};
use std::sync::atomic::AtomicBool;
use ui::{ScrollAxes, Scrollbars, WithScrollbar, utils::WithRemSize};

/// What the open breadcrumb menu is currently listing.
#[derive(Clone, Debug)]
pub enum BreadcrumbListing {
    Directory {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
    },
    Symbols {
        buffer_id: BufferId,
        /// `None` = top-level (file segment); `Some` = children/siblings of that item.
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
                        (Some(a), Some(b)) => a.range == b.range && a.depth == b.depth,
                        _ => false,
                    }
            }
            _ => false,
        }
    }
}

impl Eq for BreadcrumbListing {}

/// One filterable, keyboard-driven navigation session for the breadcrumb strip.
pub struct BreadcrumbNavigationMenu {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    listing: BreadcrumbListing,
    /// When set, the strip shows this path instead of the open file (user drilled directories).
    navigated_path: Option<(WorktreeId, Arc<RelPath>)>,
    /// Symbol ancestors shown on the bar while browsing symbols.
    symbol_trail: Vec<OutlineItem<Anchor>>,
    /// Open file path used for directory-row preselection / git trail highlight.
    active_file_path: Option<Arc<RelPath>>,
    /// Full directory listing for the current path (uncapped; display caps after ranking).
    directory_entries: Vec<BreadcrumbDirectoryEntry>,
    /// Full outline tree (multibuffer anchors) for symbol Left/Right without recompute.
    all_symbol_items: Vec<OutlineItem<Anchor>>,
    /// Indices into `all_symbol_items` currently listed as rows (before filter ranking).
    listed_symbol_indices: Vec<usize>,
    /// Ranges of symbols the cursor is inside (checkmark column).
    cursor_symbol_ranges: Vec<Range<Anchor>>,
    loading: bool,
    /// Bumps on each load/listing change so older async results are discarded.
    load_epoch: u64,
    load_task: Option<Task<()>>,
    selected_index: Option<usize>,
    pending_initial_selection: bool,
    filter_query: String,
    /// Ranked matches over the full entry set when `filter_query` is non-empty.
    ranked_matches: Vec<StringMatch>,
    filter_task: Option<Task<()>>,
    filter_epoch: u64,
    /// Fuzzy candidates for the current rows, rebuilt on listing changes — not per keystroke.
    filter_candidates: Arc<Vec<StringMatchCandidate>>,
    scroll_handle: UniformListScrollHandle,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
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
            let focus_handle = cx.focus_handle();
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
                all_symbol_items: Vec::new(),
                listed_symbol_indices: Vec::new(),
                cursor_symbol_ranges: Vec::new(),
                loading: true,
                load_epoch: 0,
                load_task: None,
                selected_index: None,
                pending_initial_selection: true,
                filter_query: String::new(),
                ranked_matches: Vec::new(),
                filter_task: None,
                filter_epoch: 0,
                filter_candidates: Arc::new(Vec::new()),
                scroll_handle: UniformListScrollHandle::new(),
                focus_handle,
                _subscriptions: Vec::new(),
            }
        });
        menu.update(cx, |this, cx| {
            let focus_handle = this.focus_handle.clone();
            this._subscriptions.push(cx.on_blur(&focus_handle, window, {
                |this: &mut Self, window, cx| {
                    if !this.focus_handle.contains_focused(window, cx) {
                        cx.emit(DismissEvent);
                    }
                }
            }));
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
                        let listing_path = listing_path.clone();
                        let should_reload = match event {
                            // Reload only when a direct child of the listed directory changed;
                            // unrelated saves elsewhere in the worktree must not re-sort the menu.
                            project::Event::WorktreeUpdatedEntries(worktree_id, updates) => {
                                worktree_id == listing_worktree
                                    && updates.iter().any(|(path, _, _)| {
                                        path.parent()
                                            .is_some_and(|parent| parent == listing_path.as_ref())
                                            || path.as_ref() == listing_path.as_ref()
                                    })
                            }
                            project::Event::WorktreeUpdatedRootRepoCommonDir(worktree_id)
                            | project::Event::WorktreeRemoved(worktree_id) => {
                                worktree_id == listing_worktree
                            }
                            _ => false,
                        };
                        if should_reload {
                            this.reload_directory_rows(cx);
                        }
                    }));
            }
            this.reload_listing(window, cx);
            this.focus_menu(window, cx);
        });
        menu
    }

    pub fn listing(&self) -> &BreadcrumbListing {
        &self.listing
    }

    /// Path the strip should show when the user has navigated away from the open file.
    pub fn navigated_path(&self) -> Option<(WorktreeId, Arc<RelPath>)> {
        self.navigated_path.clone()
    }

    /// Whether the bar is showing a navigated directory path rather than the open file.
    pub fn is_navigated(&self) -> bool {
        self.navigated_path.is_some()
    }

    /// Symbol ancestors for the strip while a symbols listing is open.
    pub fn symbol_trail(&self) -> &[OutlineItem<Anchor>] {
        &self.symbol_trail
    }

    /// Switch contents in place (no close/reopen). Resets filter and reloads async.
    pub fn set_listing(
        &mut self,
        listing: BreadcrumbListing,
        navigated: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if navigated {
            if let BreadcrumbListing::Directory { worktree_id, path } = &listing {
                self.navigated_path = Some((*worktree_id, path.clone()));
            }
        } else if let BreadcrumbListing::Directory { worktree_id, path } = &listing {
            // A non-drill open keeps the navigated bar only while browsing segments of that
            // navigated chain; opening anything outside it returns to the real file path.
            let within_navigated_chain =
                self.navigated_path
                    .as_ref()
                    .is_some_and(|(navigated_worktree, navigated)| {
                        navigated_worktree == worktree_id && navigated.starts_with(path)
                    });
            if !within_navigated_chain {
                self.navigated_path = None;
            }
        }
        if matches!(listing, BreadcrumbListing::Directory { .. }) {
            self.symbol_trail.clear();
        }
        self.listing = listing;
        self.clear_filter();
        self.pending_initial_selection = true;
        self.selected_index = None;
        self.reload_listing(window, cx);
        self.focus_menu(window, cx);
        self.emit_bar_changed(cx);
        cx.notify();
    }

    #[cfg(test)]
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    #[cfg(test)]
    pub fn filter(&self) -> &str {
        &self.filter_query
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

    #[cfg(test)]
    pub fn listed_labels(&self) -> Vec<SharedString> {
        self.entry_names()
    }

    #[cfg(test)]
    pub fn clear_filter_for_test(&mut self, cx: &mut Context<Self>) {
        self.clear_filter();
        self.rerank_filter(cx);
    }

    #[cfg(test)]
    pub fn apply_initial_selection_for_test(&mut self, cx: &mut Context<Self>) {
        self.pending_initial_selection = true;
        self.apply_initial_selection_if_needed(cx);
    }

    /// Test helper: drive directory choose without going through keyboard confirm.
    #[cfg(test)]
    pub fn choose_directory_entry_for_test(
        &mut self,
        path: Arc<RelPath>,
        entry_id: ProjectEntryId,
        is_dir: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.choose_directory_entry(
            BreadcrumbDirectoryEntry {
                name: path
                    .file_name()
                    .unwrap_or_else(|| path.as_unix_str())
                    .to_string()
                    .into(),
                path,
                entry_id,
                is_dir,
                is_ignored: false,
                git_summary: GitSummary::UNCHANGED,
            },
            window,
            cx,
        );
    }

    /// Test helper: symbols menu with a preloaded outline tree (skips async load).
    #[cfg(test)]
    pub fn new_with_symbols_for_test(
        editor: WeakEntity<Editor>,
        buffer_id: BufferId,
        all_items: Vec<OutlineItem<Anchor>>,
        listed_indices: Vec<usize>,
        cursor_symbol_ranges: Vec<Range<Anchor>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let menu = cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            Self {
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
                all_symbol_items: all_items,
                listed_symbol_indices: listed_indices,
                cursor_symbol_ranges,
                loading: false,
                load_epoch: 0,
                load_task: None,
                selected_index: None,
                pending_initial_selection: true,
                filter_query: String::new(),
                ranked_matches: Vec::new(),
                filter_task: None,
                filter_epoch: 0,
                filter_candidates: Arc::new(Vec::new()),
                scroll_handle: UniformListScrollHandle::new(),
                focus_handle,
                _subscriptions: Vec::new(),
            }
        });
        menu
    }

    fn focus_menu(&self, window: &mut Window, cx: &mut Context<Self>) {
        let _ = cx;
        window.focus(&self.focus_handle, cx);
    }

    fn clear_filter(&mut self) {
        self.filter_query.clear();
        self.ranked_matches.clear();
        self.filter_epoch = self.filter_epoch.wrapping_add(1);
        self.filter_task = None;
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

    fn reload_listing(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.load_epoch = self.load_epoch.wrapping_add(1);
        let epoch = self.load_epoch;
        self.loading = true;
        self.directory_entries.clear();
        self.filter_candidates = Arc::new(Vec::new());
        // Keep the symbol tree across parent-only listing changes so Left/Right stays instant.
        if matches!(self.listing, BreadcrumbListing::Directory { .. }) {
            self.all_symbol_items.clear();
            self.listed_symbol_indices.clear();
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

    /// Recompute stored directory rows from the current worktree snapshot (no expand, no
    /// load_epoch bump — in-flight drills must not be cancelled by worktree events).
    fn reload_directory_rows(&mut self, cx: &mut Context<Self>) {
        let BreadcrumbListing::Directory { worktree_id, path } = &self.listing else {
            return;
        };
        let Some((worktree, project)) = self.worktree(*worktree_id, cx).zip(self.project(cx))
        else {
            self.directory_entries.clear();
            cx.notify();
            return;
        };
        self.directory_entries = breadcrumb_directory_entries(&project, &worktree, path, cx);
        self.rebuild_filter_candidates();
        self.loading = false;
        if !self.filter_query.is_empty() {
            // Old match indices point into the replaced entry vec; confirming against them
            // would resolve the wrong row until the async rerank lands.
            self.ranked_matches.clear();
            self.selected_index = None;
            self.rerank_filter(cx);
        } else {
            let visible = self.visible_row_count();
            if let Some(position) = self.selected_index
                && position >= visible
            {
                self.selected_index = visible.checked_sub(1);
            }
            self.apply_initial_selection_if_needed(cx);
        }
        cx.notify();
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
            this.update(cx, |this, cx| {
                if this.load_epoch != epoch {
                    return;
                }
                let entries = this
                    .worktree(worktree_id, cx)
                    .zip(this.project(cx))
                    .map(|(worktree, project)| {
                        breadcrumb_directory_entries(&project, &worktree, &path, cx)
                    })
                    .unwrap_or_default();
                this.directory_entries = entries;
                this.rebuild_filter_candidates();
                this.loading = false;
                this.apply_initial_selection_if_needed(cx);
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
        if !self.all_symbol_items.is_empty() {
            self.apply_symbol_parent(parent, cx);
            self.loading = false;
            self.apply_initial_selection_if_needed(cx);
            cx.notify();
            self.load_task = None;
            return;
        }

        // Start the outline task inside the spawned future so we never nest
        // `editor.update` under an outer `editor.update` (segment click / action).
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
                let Some(editor) = this.editor.upgrade() else {
                    this.loading = false;
                    cx.notify();
                    return;
                };
                let (all_items, cursor_ranges) = editor.update(cx, |editor, cx| {
                    let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                    let all_items =
                        editor.map_text_outline_items(&text_items, &multi_buffer_snapshot);
                    let cursor_ranges = editor
                        .outline_symbols_at_cursor
                        .as_ref()
                        .filter(|(id, _)| *id == buffer_id)
                        .map(|(_, ancestors)| {
                            ancestors.iter().map(|item| item.range.clone()).collect()
                        })
                        .unwrap_or_default();
                    (all_items, cursor_ranges)
                });

                this.cursor_symbol_ranges = cursor_ranges;
                this.all_symbol_items = all_items;
                this.loading = false;

                if this.all_symbol_items.is_empty() {
                    if parent.is_none() {
                        if let Some(callback) = zed_actions::outline::TOGGLE_OUTLINE.get() {
                            callback(editor.to_any_view(), window, cx);
                        }
                        cx.emit(DismissEvent);
                        return;
                    }
                }

                this.apply_symbol_parent(parent, cx);
                this.apply_initial_selection_if_needed(cx);
                this.emit_bar_changed(cx);
                cx.notify();
            })
            .ok();
        }));
    }

    fn apply_symbol_parent(&mut self, parent: Option<OutlineItem<Anchor>>, cx: &mut Context<Self>) {
        let depths: Vec<usize> = self
            .all_symbol_items
            .iter()
            .map(|item| item.depth)
            .collect();
        let listed_indices = if let Some(parent_item) = parent.as_ref() {
            let Some(parent_index) = self
                .all_symbol_items
                .iter()
                .position(|item| item.range == parent_item.range)
            else {
                self.listed_symbol_indices = vec![];
                self.ranked_matches.clear();
                self.rebuild_filter_candidates();
                let buffer_id = match &self.listing {
                    BreadcrumbListing::Symbols { buffer_id, .. } => *buffer_id,
                    _ => return,
                };
                self.listing = BreadcrumbListing::Symbols { buffer_id, parent };
                self.rebuild_symbol_trail(cx);
                return;
            };
            let children = child_outline_indices(&depths, parent_index);
            if children.is_empty() {
                sibling_outline_indices(&depths, parent_index)
            } else {
                children
            }
        } else {
            top_level_outline_indices(&depths)
        };

        let buffer_id = match &self.listing {
            BreadcrumbListing::Symbols { buffer_id, .. } => *buffer_id,
            _ => return,
        };
        self.listing = BreadcrumbListing::Symbols { buffer_id, parent };
        self.listed_symbol_indices = listed_indices;
        self.rebuild_filter_candidates();
        self.rebuild_symbol_trail(cx);
    }

    fn rebuild_symbol_trail(&mut self, _cx: &mut Context<Self>) {
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

    /// Speed-search input: printable keys append, backspace pops one grapheme, cmd/ctrl-v
    /// pastes. Returns whether the keystroke edited the query.
    fn handle_filter_key(&mut self, event: &gpui::KeyDownEvent, cx: &mut Context<Self>) -> bool {
        use unicode_segmentation::UnicodeSegmentation as _;
        let keystroke = &event.keystroke;
        let paste_modifier = if cfg!(target_os = "macos") {
            keystroke.modifiers.platform
        } else {
            keystroke.modifiers.control
        };
        if paste_modifier && keystroke.key == "v" {
            if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                let sanitized: String = text.chars().filter(|ch| !ch.is_control()).collect();
                if !sanitized.is_empty() {
                    self.filter_query.push_str(&sanitized);
                    self.apply_filter_edit(cx);
                    return true;
                }
            }
            return false;
        }
        if keystroke.modifiers.control
            || keystroke.modifiers.alt
            || keystroke.modifiers.platform
            || keystroke.modifiers.function
        {
            return false;
        }
        if keystroke.key == "backspace" {
            let Some((offset, _)) = self.filter_query.grapheme_indices(true).next_back() else {
                return false;
            };
            self.filter_query.truncate(offset);
            self.apply_filter_edit(cx);
            return true;
        }
        if let Some(key_char) = keystroke
            .key_char
            .as_ref()
            .filter(|key_char| !key_char.chars().any(char::is_control))
        {
            self.filter_query.push_str(key_char);
            self.apply_filter_edit(cx);
            return true;
        }
        false
    }

    fn apply_filter_edit(&mut self, cx: &mut Context<Self>) {
        self.pending_initial_selection = false;
        self.rerank_filter(cx);
        cx.notify();
    }

    fn rerank_filter(&mut self, cx: &mut Context<Self>) {
        self.filter_epoch = self.filter_epoch.wrapping_add(1);
        let epoch = self.filter_epoch;
        let query = self.filter_query.clone();

        if query.is_empty() {
            self.ranked_matches.clear();
            self.filter_task = None;
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
            cx.notify();
            return;
        }

        let candidates = self.filter_candidates.clone();
        let executor = cx.background_executor().clone();
        self.filter_task = Some(cx.spawn(async move |this, cx| {
            let cancel_flag = AtomicBool::new(false);
            let matches = fuzzy::match_strings(
                candidates.as_slice(),
                &query,
                false,
                true,
                MAX_BREADCRUMB_MENU_ROWS,
                &cancel_flag,
                executor,
            )
            .await;
            this.update(cx, |this, cx| {
                if this.filter_epoch != epoch {
                    return;
                }
                this.ranked_matches = matches;
                this.selected_index = (!this.ranked_matches.is_empty()).then_some(0);
                if let Some(0) = this.selected_index {
                    this.scroll_handle
                        .scroll_to_item(0, ScrollStrategy::Nearest);
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn rebuild_filter_candidates(&mut self) {
        let candidates: Vec<StringMatchCandidate> = match &self.listing {
            BreadcrumbListing::Directory { .. } => self
                .directory_entries
                .iter()
                .enumerate()
                .map(|(index, entry)| StringMatchCandidate::new(index, entry.name.as_ref()))
                .collect(),
            BreadcrumbListing::Symbols { .. } => self
                .listed_symbol_indices
                .iter()
                .enumerate()
                .filter_map(|(position, &outline_index)| {
                    let text = self.all_symbol_items.get(outline_index)?.text.as_ref();
                    Some(StringMatchCandidate::new(position, text))
                })
                .collect(),
        };
        self.filter_candidates = Arc::new(candidates);
    }

    fn visible_row_count(&self) -> usize {
        if !self.filter_query.is_empty() {
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
        if !self.filter_query.is_empty() {
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

    /// Whether the uncapped listing exceeds the display cap (note only when unfiltered).
    fn is_display_truncated(&self) -> bool {
        if !self.filter_query.is_empty() {
            return false;
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
        if !self.pending_initial_selection || !self.filter_query.is_empty() || self.loading {
            return;
        }
        let visible = self.visible_row_count();
        if visible == 0 {
            return;
        }
        self.pending_initial_selection = false;
        self.selected_index = self.initial_selected_index();
        if let Some(position) = self.selected_index {
            self.scroll_handle
                .scroll_to_item(position, ScrollStrategy::Nearest);
        }
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
        if let Some(position) = position {
            self.scroll_handle
                .scroll_to_item(position, ScrollStrategy::Nearest);
        }
        cx.notify();
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let visible = self.visible_row_count();
        if visible == 0 {
            return;
        }
        let next = self
            .selected_index
            .map(|position| (position + 1) % visible)
            .unwrap_or(0);
        self.move_selection(Some(next), cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let visible = self.visible_row_count();
        if visible == 0 {
            return;
        }
        let previous = self
            .selected_index
            .map(|position| {
                if position == 0 {
                    visible - 1
                } else {
                    position - 1
                }
            })
            .unwrap_or(visible - 1);
        self.move_selection(Some(previous), cx);
    }

    fn select_first(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        if self.visible_row_count() == 0 {
            return;
        }
        self.move_selection(Some(0), cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        let visible = self.visible_row_count();
        if visible == 0 {
            return;
        }
        self.move_selection(Some(visible - 1), cx);
    }

    pub fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
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

    pub(crate) fn select_child(
        &mut self,
        _: &menu::SelectChild,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.listing.clone() {
            BreadcrumbListing::Directory { .. } => {
                let Some(entry) = self.selected_directory_entry() else {
                    return;
                };
                if entry.is_dir {
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
                self.clear_filter();
                self.pending_initial_selection = true;
                self.selected_index = None;
                self.listing = BreadcrumbListing::Symbols { buffer_id, parent };
                self.listed_symbol_indices = children;
                self.rebuild_filter_candidates();
                self.rebuild_symbol_trail(cx);
                self.apply_initial_selection_if_needed(cx);
                self.emit_bar_changed(cx);
                cx.notify();
            }
        }
    }

    pub(crate) fn select_parent(
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
                    true,
                    window,
                    cx,
                );
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
                    // Parent of listed rows is the file top-level.
                    self.clear_filter();
                    self.pending_initial_selection = true;
                    self.selected_index = None;
                    self.listing = BreadcrumbListing::Symbols {
                        buffer_id,
                        parent: None,
                    };
                    self.listed_symbol_indices = top_level_outline_indices(&depths);
                    self.rebuild_filter_candidates();
                    self.symbol_trail.clear();
                    self.apply_initial_selection_if_needed(cx);
                    self.emit_bar_changed(cx);
                    cx.notify();
                    return;
                };
                let siblings = sibling_outline_indices(&depths, parent_index);
                let new_parent = parents
                    .get(parent_index)
                    .copied()
                    .flatten()
                    .and_then(|index| self.all_symbol_items.get(index).cloned());
                self.clear_filter();
                self.pending_initial_selection = true;
                self.selected_index = None;
                self.listing = BreadcrumbListing::Symbols {
                    buffer_id,
                    parent: new_parent,
                };
                self.listed_symbol_indices = siblings;
                self.rebuild_filter_candidates();
                self.rebuild_symbol_trail(cx);
                self.apply_initial_selection_if_needed(cx);
                self.emit_bar_changed(cx);
                cx.notify();
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
        let parent_path = project_path
            .path
            .parent()
            .map(|parent| parent.into_arc())
            .unwrap_or_else(|| RelPath::empty().into_arc());
        self.active_file_path = Some(project_path.path.clone());
        self.set_listing(
            BreadcrumbListing::Directory {
                worktree_id: project_path.worktree_id,
                path: parent_path,
            },
            false,
            window,
            cx,
        );
    }

    fn selected_directory_entry(&self) -> Option<BreadcrumbDirectoryEntry> {
        let BreadcrumbListing::Directory { .. } = &self.listing else {
            return None;
        };
        let position = self.selected_index?;
        if !self.filter_query.is_empty() {
            let match_ = self.ranked_matches.get(position)?;
            return self.directory_entries.get(match_.candidate_id).cloned();
        }
        self.directory_entries.get(position).cloned()
    }

    fn selected_symbol_item(&self) -> Option<OutlineItem<Anchor>> {
        let outline_index = self.selected_symbol_outline_index()?;
        self.all_symbol_items.get(outline_index).cloned()
    }

    fn selected_symbol_outline_index(&self) -> Option<usize> {
        let BreadcrumbListing::Symbols { .. } = &self.listing else {
            return None;
        };
        let position = self.selected_index?;
        if !self.filter_query.is_empty() {
            let match_ = self.ranked_matches.get(position)?;
            let listed_position = match_.candidate_id;
            return self.listed_symbol_indices.get(listed_position).copied();
        }
        self.listed_symbol_indices.get(position).copied()
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
                            let children = directory_child_paths(&worktree, &path, cx);
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
                this.set_listing(
                    BreadcrumbListing::Directory { worktree_id, path },
                    true,
                    window,
                    cx,
                );
            })
            .ok();
        }));
    }

    fn open_file(&mut self, path: Arc<RelPath>, window: &mut Window, cx: &mut Context<Self>) {
        let worktree_id = match &self.listing {
            BreadcrumbListing::Directory { worktree_id, .. } => *worktree_id,
            _ => {
                cx.emit(DismissEvent);
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
        cx.emit(DismissEvent);
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
        cx.emit(DismissEvent);
    }

    fn render_directory_entry(
        &self,
        position: usize,
        entry: BreadcrumbDirectoryEntry,
        match_positions: &[usize],
        show_file_icons: bool,
        show_folder_icons: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_open_ancestor = entry.is_dir
            && self
                .active_file_path
                .as_ref()
                .is_some_and(|active_path| active_path.starts_with(&entry.path));
        // Current-file trail must not paint a second selection highlight.
        let is_selected = self.selected_index == Some(position);

        let icon_path =
            match directory_entry_icon_source(entry.is_dir, show_file_icons, show_folder_icons) {
                DirectoryEntryIconSource::File => {
                    file_icons::FileIcons::get_icon(entry.path.as_std_path(), cx)
                }
                DirectoryEntryIconSource::Folder => file_icons::FileIcons::get_folder_icon(
                    is_open_ancestor,
                    entry.path.as_std_path(),
                    cx,
                ),
                DirectoryEntryIconSource::Chevron => {
                    file_icons::FileIcons::get_chevron_icon(false, cx)
                }
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

        // Always use git-aware colors: when git_status is off, summaries are UNCHANGED and
        // ignored-dimming still applies (same as the project panel).
        let label_color = crate::items::entry_git_aware_label_color(
            entry.git_summary,
            entry.is_ignored,
            is_selected,
        );

        let entry_for_click = entry.clone();
        let label = if match_positions.is_empty() {
            Label::new(entry.name.clone())
                .color(label_color)
                .into_any_element()
        } else {
            ui::HighlightedLabel::new(entry.name.clone(), match_positions.to_vec())
                .color(label_color)
                .into_any_element()
        };

        ListItem::new(SharedString::from(format!(
            "breadcrumb-directory-entry-{}",
            entry.name
        )))
        .inset(true)
        .toggle_state(is_selected)
        .start_slot(icon)
        .child(label)
        .on_click(cx.listener(move |this, _, window, cx| {
            this.choose_directory_entry(entry_for_click.clone(), window, cx);
        }))
    }

    /// Range of the deepest cursor symbol (checkmark only on that row, not ancestors).
    fn deepest_cursor_symbol_range(&self) -> Option<&Range<Anchor>> {
        self.cursor_symbol_ranges.last()
    }

    fn directory_row_data(
        &self,
        position: usize,
    ) -> Option<(BreadcrumbDirectoryEntry, Vec<usize>)> {
        if !self.filter_query.is_empty() {
            let match_ = self.ranked_matches.get(position)?;
            let entry = self.directory_entries.get(match_.candidate_id)?.clone();
            return Some((entry, match_.positions.clone()));
        }
        let entry = self.directory_entries.get(position)?.clone();
        Some((entry, Vec::new()))
    }

    fn symbol_row_data(&self, position: usize) -> Option<(OutlineItem<Anchor>, Vec<usize>)> {
        if !self.filter_query.is_empty() {
            let match_ = self.ranked_matches.get(position)?;
            let listed_position = match_.candidate_id;
            let outline_index = *self.listed_symbol_indices.get(listed_position)?;
            let item = self.all_symbol_items.get(outline_index)?.clone();
            return Some((item, match_.positions.clone()));
        }
        let outline_index = *self.listed_symbol_indices.get(position)?;
        let item = self.all_symbol_items.get(outline_index)?.clone();
        Some((item, Vec::new()))
    }

    fn render_filter_row(&self, cx: &Context<Self>) -> impl IntoElement {
        let content: gpui::AnyElement = if self.filter_query.is_empty() {
            Label::new("Type to filter…")
                .color(Color::Placeholder)
                .into_any_element()
        } else {
            Label::new(self.filter_query.clone()).into_any_element()
        };
        h_flex()
            .px_2()
            .py_1()
            .gap_2()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                Icon::new(IconName::MagnifyingGlass)
                    .color(Color::Muted)
                    .size(IconSize::Small),
            )
            .child(content)
    }
}

impl gpui::Focusable for BreadcrumbNavigationMenu {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for BreadcrumbNavigationMenu {}

impl Render for BreadcrumbNavigationMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let visible = self.visible_row_count();
        let listing_settings = BreadcrumbListingSettings::get_global(cx);
        let show_file_icons = listing_settings.file_icons;
        let show_folder_icons = listing_settings.folder_icons;
        let is_directory = matches!(self.listing, BreadcrumbListing::Directory { .. });
        let deepest_current = self.deepest_cursor_symbol_range().cloned();
        let show_current_column = match &self.listing {
            BreadcrumbListing::Directory { .. } => false,
            BreadcrumbListing::Symbols { .. } => {
                if self.loading {
                    false
                } else if !self.filter_query.is_empty() {
                    self.ranked_matches.iter().any(|match_| {
                        self.listed_symbol_indices
                            .get(match_.candidate_id)
                            .and_then(|&outline_index| self.all_symbol_items.get(outline_index))
                            .is_some_and(|item| {
                                deepest_current
                                    .as_ref()
                                    .is_some_and(|range| item.range == *range)
                            })
                    })
                } else {
                    self.listed_symbol_indices.iter().any(|&outline_index| {
                        self.all_symbol_items
                            .get(outline_index)
                            .is_some_and(|item| {
                                deepest_current
                                    .as_ref()
                                    .is_some_and(|range| item.range == *range)
                            })
                    })
                }
            }
        };

        let empty_message = if self.loading {
            "Loading…"
        } else if !self.filter_query.is_empty() {
            "No matches"
        } else if is_directory {
            "Empty directory"
        } else {
            "No symbols"
        };

        let truncated = self.is_display_truncated();
        let theme_settings = theme::theme_settings(cx);
        let ui_font_size = theme_settings.ui_font_size(cx);
        let ui_font_family = theme_settings.ui_font(cx).family.clone();
        let list_count = if self.loading { 0 } else { visible };

        let rows_list = uniform_list(
            "breadcrumb-navigation-menu-rows",
            list_count,
            cx.processor(move |this, range: Range<usize>, window, cx| {
                range
                    .map(|position| {
                        if is_directory {
                            let Some((entry, match_positions)) = this.directory_row_data(position)
                            else {
                                return div().into_any_element();
                            };
                            this.render_directory_entry(
                                position,
                                entry,
                                &match_positions,
                                show_file_icons,
                                show_folder_icons,
                                cx,
                            )
                            .into_any_element()
                        } else {
                            let Some((item, match_positions)) = this.symbol_row_data(position)
                            else {
                                return div().into_any_element();
                            };
                            let is_current = deepest_current
                                .as_ref()
                                .is_some_and(|range| item.range == *range);
                            let is_selected = this.selected_index == Some(position);
                            let row = if match_positions.is_empty() {
                                render_outline_item_menu_row(
                                    &item,
                                    is_current,
                                    show_current_column,
                                    window,
                                    cx,
                                )
                            } else {
                                h_flex()
                                    .gap_1p5()
                                    .when(is_current, |this| {
                                        this.child(
                                            Icon::new(IconName::Check)
                                                .color(Color::Accent)
                                                .size(IconSize::Small),
                                        )
                                    })
                                    .when(!is_current && show_current_column, |this| {
                                        this.child(div().size(IconSize::Small.rems()))
                                    })
                                    .child(
                                        ui::HighlightedLabel::new(
                                            flatten_text_for_single_line_display(&item.text),
                                            match_positions,
                                        )
                                        .color(Color::Default),
                                    )
                                    .into_any_element()
                            };
                            ListItem::new(position)
                                .inset(true)
                                .toggle_state(is_selected)
                                .child(row)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.navigate_to_symbol(&item, window, cx);
                                }))
                                .into_any_element()
                        }
                    })
                    .collect()
            }),
        )
        .with_sizing_behavior(ListSizingBehavior::Infer)
        .track_scroll(&self.scroll_handle)
        .flex_grow(1.)
        .max_h(px(400.));

        WithRemSize::new(ui_font_size)
            .font_family(ui_font_family)
            .elevation_2(cx)
            .child(
                v_flex()
                    .id("breadcrumb-navigation-menu")
                    .track_focus(&self.focus_handle)
                    .key_context("menu")
                    .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
                    .on_action(cx.listener(Self::select_next))
                    .on_action(cx.listener(Self::select_previous))
                    .on_action(cx.listener(Self::select_first))
                    .on_action(cx.listener(Self::select_last))
                    .on_action(cx.listener(Self::confirm))
                    .on_action(cx.listener(Self::select_child))
                    .on_action(cx.listener(Self::select_parent))
                    .on_key_down(cx.listener(|this, event, _, cx| {
                        if this.handle_filter_key(event, cx) {
                            cx.stop_propagation();
                        }
                    }))
                    .on_mouse_down_out(cx.listener(|_, _: &MouseDownEvent, _, cx| {
                        // Do not stop propagation: a double-click on a segment must both dismiss
                        // this menu and deliver both clicks to the segment (reveal on count 2).
                        cx.emit(DismissEvent);
                    }))
                    .min_w(px(200.))
                    .child(self.render_filter_row(cx))
                    .child(
                        v_flex()
                            .id("breadcrumb-navigation-menu-list")
                            .relative()
                            // Vertical inset so the selected-row highlight clears rounded corners.
                            .py(DynamicSpacing::Base04.rems(cx))
                            .max_h(px(400.))
                            .min_h_0()
                            .overflow_hidden()
                            .when(list_count == 0, |this| {
                                this.child(
                                    h_flex().px_2().py_1().child(
                                        Label::new(empty_message)
                                            .color(Color::Muted)
                                            .size(LabelSize::Small),
                                    ),
                                )
                            })
                            .when(list_count > 0, |this| {
                                this.child(rows_list).custom_scrollbars(
                                    Scrollbars::new(ScrollAxes::Vertical)
                                        .tracked_scroll_handle(&self.scroll_handle),
                                    window,
                                    cx,
                                )
                            }),
                    )
                    .when(truncated, |this| {
                        this.child(
                            Label::new(breadcrumb_menu_truncated_label())
                                .color(Color::Muted)
                                .size(LabelSize::Small)
                                .mx_2()
                                .mb_1(),
                        )
                    }),
            )
    }
}
