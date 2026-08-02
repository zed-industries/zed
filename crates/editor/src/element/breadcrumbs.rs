//! Breadcrumb path and symbol navigation: turns the bar's segments into clickable dropdowns,
//! sharing the project panel's ordering and gitignore treatment rather than reimplementing them.

use super::*;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::Task;
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::Project;
use ui::{HighlightedLabel, ListItemSpacing};

/// The parent of each item is the nearest preceding entry with a smaller depth rather than
/// `depth - 1`, because tree-sitter outlines can jump depth unevenly.
fn outline_parents(depths: &[usize]) -> Vec<Option<usize>> {
    let mut parents = Vec::with_capacity(depths.len());
    let mut ancestor_stack: Vec<(usize, usize)> = Vec::new();
    for (index, &depth) in depths.iter().enumerate() {
        while ancestor_stack
            .last()
            .is_some_and(|&(ancestor_depth, _)| ancestor_depth >= depth)
        {
            ancestor_stack.pop();
        }
        parents.push(ancestor_stack.last().map(|&(_, parent_index)| parent_index));
        ancestor_stack.push((depth, index));
    }
    parents
}

/// Items at `target_index`'s depth sharing its nearest shallower ancestor, itself included.
pub(crate) fn sibling_outline_indices(depths: &[usize], target_index: usize) -> Vec<usize> {
    if target_index >= depths.len() {
        return Vec::new();
    }

    let parents = outline_parents(depths);
    let target_parent = parents[target_index];
    parents
        .iter()
        .enumerate()
        .filter_map(|(index, &parent)| (parent == target_parent).then_some(index))
        .collect()
}

/// The items directly inside `target_index`, one level deeper.
pub(crate) fn child_outline_indices(depths: &[usize], target_index: usize) -> Vec<usize> {
    if target_index >= depths.len() {
        return Vec::new();
    }

    let parents = outline_parents(depths);
    parents
        .iter()
        .enumerate()
        .filter_map(|(index, &parent)| (parent == Some(target_index)).then_some(index))
        .collect()
}

/// Indices of the top-level items — those with no parent. The breadcrumb's leading path
/// segment stands in for the tree's implicit root, so it lists these.
pub(crate) fn top_level_outline_indices(depths: &[usize]) -> Vec<usize> {
    let parents = outline_parents(depths);
    parents
        .iter()
        .enumerate()
        .filter_map(|(index, &parent)| parent.is_none().then_some(index))
        .collect()
}

/// What a segment's dropdown drills into.
#[derive(Clone, Debug)]
pub(crate) enum BreadcrumbSegmentTarget {
    /// Lists document symbols: `item: None` is the file segment and lists top-level symbols,
    /// `Some` lists that item's children.
    Symbol {
        buffer_id: BufferId,
        item: Option<OutlineItem<Anchor>>,
    },
    /// Lists `path`'s contents. `active_path` is the same at every ancestor, so a listing at any
    /// depth can mark the trail towards it.
    Directory {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        active_path: Option<Arc<RelPath>>,
        /// Whether this segment's own dropdown is open, which draws it as the active one.
        is_active_segment: bool,
    },
}

/// Splits `path` into ancestor prefixes, root first: `a/b/c.rs` becomes `[a, a/b, a/b/c.rs]`.
fn breadcrumb_path_prefixes(path: &RelPath) -> Vec<&RelPath> {
    let mut prefixes: Vec<&RelPath> = path
        .ancestors()
        .filter(|prefix| !prefix.is_empty())
        .collect();
    prefixes.reverse();
    prefixes
}

/// Builds the leading path segments, root first. The root is included so top-level directories
/// stay reachable, since no other segment lists them.
pub(crate) fn breadcrumb_path_segments(
    worktree_id: WorktreeId,
    root_name: &str,
    path: &Arc<RelPath>,
    active_path: Option<Arc<RelPath>>,
    terminal_buffer_id: Option<BufferId>,
    active_segment: Option<&RelPath>,
) -> (Vec<HighlightedText>, Vec<Option<BreadcrumbSegmentTarget>>) {
    let mut labels = vec![HighlightedText {
        text: root_name.to_string().into(),
        highlights: vec![],
    }];
    let mut targets = vec![Some(BreadcrumbSegmentTarget::Directory {
        worktree_id,
        path: RelPath::empty().into_arc(),
        active_path: active_path.clone(),
        is_active_segment: active_segment == Some(RelPath::empty()),
    })];

    let prefixes = breadcrumb_path_prefixes(path);
    let last_prefix_index = prefixes.len().saturating_sub(1);
    for (prefix_index, prefix) in prefixes.iter().copied().enumerate() {
        let name = prefix.file_name().unwrap_or_else(|| prefix.as_unix_str());
        labels.push(HighlightedText {
            text: name.to_string().into(),
            highlights: vec![],
        });
        targets.push(Some(
            if prefix_index == last_prefix_index
                && let Some(buffer_id) = terminal_buffer_id
            {
                BreadcrumbSegmentTarget::Symbol {
                    buffer_id,
                    item: None,
                }
            } else {
                BreadcrumbSegmentTarget::Directory {
                    worktree_id,
                    path: prefix.into_arc(),
                    active_path: active_path.clone(),
                    is_active_segment: active_segment == Some(prefix),
                }
            },
        ));
    }

    (labels, targets)
}

/// Flattens `text` to a single display line. The replacement must be the same UTF-8 length as the
/// newline, since highlight ranges are byte offsets into the unflattened text.
fn flatten_text_for_single_line_display(text: &str) -> String {
    const LINE_BREAK: char = '\n';
    const REPLACEMENT: &str = " ";
    debug_assert_eq!(
        LINE_BREAK.len_utf8(),
        REPLACEMENT.len(),
        "replacing {LINE_BREAK:?} with {REPLACEMENT:?} would shift byte-offset highlight ranges"
    );
    text.replace(LINE_BREAK, REPLACEMENT)
}

/// The symbols a breadcrumb segment can move to, filtered by the picker's query.
pub(crate) struct BreadcrumbSymbolDelegate {
    editor: WeakEntity<Editor>,
    items: Vec<OutlineItem<Anchor>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    /// The segment's own symbol, so the row standing for it reads as the current one.
    current_range: Option<Range<Anchor>>,
}

pub(crate) type BreadcrumbSymbolPicker = Picker<BreadcrumbSymbolDelegate>;

impl BreadcrumbSymbolDelegate {
    fn picker(
        editor: WeakEntity<Editor>,
        items: Vec<OutlineItem<Anchor>>,
        current_range: Option<Range<Anchor>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<BreadcrumbSymbolPicker> {
        cx.new(|cx| {
            let selected_index = current_range
                .as_ref()
                .and_then(|range| items.iter().position(|item| &item.range == range))
                .unwrap_or(0);
            let delegate = Self {
                editor,
                items,
                matches: Vec::new(),
                selected_index,
                current_range,
            };
            Picker::uniform_list(delegate, window, cx)
                .popover()
                .initial_width(rems(18.))
        })
    }

    /// Whether any listed symbol is the segment's own. If none is, the checkmark column is left
    /// out rather than indenting every row for a mark that never appears.
    fn shows_current_marker(&self) -> bool {
        self.current_range
            .as_ref()
            .is_some_and(|range| self.items.iter().any(|item| &item.range == range))
    }

    fn item_at(&self, index: usize) -> Option<&OutlineItem<Anchor>> {
        self.items.get(self.matches.get(index)?.candidate_id)
    }
}

impl PickerDelegate for BreadcrumbSymbolDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "breadcrumb symbol picker"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) {
        self.selected_index = index;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search symbols…".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::End
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) -> Task<()> {
        let candidates = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                StringMatchCandidate::new(index, &flatten_text_for_single_line_display(&item.text))
            })
            .collect::<Vec<_>>();

        if query.is_empty() {
            self.matches = candidates
                .into_iter()
                .map(|candidate| StringMatch {
                    candidate_id: candidate.id,
                    string: candidate.string,
                    positions: Vec::new(),
                    score: 0.,
                })
                .collect();
            self.selected_index = self
                .current_range
                .as_ref()
                .and_then(|range| self.items.iter().position(|item| &item.range == range))
                .unwrap_or(0);
            cx.notify();
            return Task::ready(());
        }

        let executor = cx.background_executor().clone();
        cx.spawn(async move |picker, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                MAX_BREADCRUMB_MENU_ENTRIES,
                &Default::default(),
                executor,
            )
            .await;
            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .ok();
        })
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) {
        let Some(item) = self.item_at(self.selected_index).cloned() else {
            return;
        };
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_to_outline_item(&item, window, cx);
            });
        }
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<BreadcrumbSymbolPicker>) {}

    /// Rendered with the symbol's own syntax highlighting, the way the outline picker and panel
    /// draw it. The fuzzy match positions are left off, since the two highlight sets would
    /// fight.
    fn render_match(
        &self,
        index: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) -> Option<Self::ListItem> {
        let item = self.item_at(index)?;
        let is_current = self.current_range.as_ref() == Some(&item.range);

        let mut text_style = window.text_style();
        text_style.color = Color::Default.color(cx);

        Some(
            ListItem::new(SharedString::from(format!(
                "breadcrumb-symbol-entry-{index}"
            )))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .when(self.shows_current_marker(), |this| {
                this.start_slot(div().flex_none().size(IconSize::Small.rems()).when(
                    is_current,
                    |this| {
                        this.child(
                            Icon::new(IconName::Check)
                                .color(Color::Accent)
                                .size(IconSize::Small),
                        )
                    },
                ))
            })
            .child(
                div().text_ui(cx).child(
                    StyledText::new(flatten_text_for_single_line_display(&item.text))
                        .with_default_highlights(&text_style, item.highlight_ranges.clone()),
                ),
            )
            .into_any_element(),
        )
    }
}

/// A segment whose dropdown drills into the outline: `target`'s children, else its siblings, else
/// the buffer's top-level symbols.
fn render_breadcrumb_symbol_segment(
    editor: WeakEntity<Editor>,
    buffer_id: BufferId,
    target: Option<OutlineItem<Anchor>>,
    label: gpui::AnyElement,
    index: usize,
) -> gpui::AnyElement {
    // `ButtonLike` wraps its click handler in `cx.stop_propagation()`, which is what keeps this
    // click from also reaching the outline toggle behind the popover.
    let trigger = ButtonLike::new(("breadcrumb-symbol", index))
        .style(ButtonStyle::Transparent)
        .size(ButtonSize::None)
        .height(rems_from_px(22.).into())
        .child(label);

    PopoverMenu::new(("breadcrumb-symbol-menu", index))
        .trigger(trigger)
        .menu(move |window, cx| {
            let editor_entity = editor.upgrade()?;
            let menu_items =
                editor_entity
                    .read(cx)
                    .breadcrumb_symbol_menu_items(buffer_id, target.as_ref(), cx);
            // Nothing to drill into, so fall through to the outline picker rather than flashing an
            // empty popover.
            if menu_items.is_empty() {
                if let Some(callback) = zed_actions::outline::TOGGLE_OUTLINE.get() {
                    callback(editor_entity.to_any_view(), window, cx);
                }
                return None;
            }
            Some(BreadcrumbSymbolDelegate::picker(
                editor.clone(),
                menu_items,
                target.as_ref().map(|item| item.range.clone()),
                window,
                cx,
            ))
        })
        .into_any_element()
}

/// Caps how many entries one dropdown lists.
pub(crate) const MAX_BREADCRUMB_MENU_ENTRIES: usize = 200;

/// Bounds how far [`descend_single_child_directories`] walks, guarding against a pathologically
/// deep chain or a symlink cycle.
const MAX_BREADCRUMB_DESCENT_DEPTH: usize = 64;

/// Walks down through directories that hold exactly one child directory, stopping one short of a
/// file so the user still opens it themselves.
///
fn descend_single_child_directories(
    start: Arc<RelPath>,
    mut child_entries: impl FnMut(&RelPath) -> Vec<(Arc<RelPath>, bool)>,
) -> Arc<RelPath> {
    let mut current = start;
    for _ in 0..MAX_BREADCRUMB_DESCENT_DEPTH {
        let children = child_entries(&current);
        let [(only_child_path, only_child_is_dir)] = children.as_slice() else {
            return current;
        };
        if !only_child_is_dir {
            return current;
        }
        current = only_child_path.clone();
    }
    current
}

fn breadcrumb_directory_children(
    worktree: &Entity<project::Worktree>,
    path: &RelPath,
    cx: &App,
) -> Vec<(Arc<RelPath>, bool)> {
    worktree
        .read(cx)
        .snapshot()
        .child_entries(path)
        .map(|entry| (entry.path.clone(), entry.is_dir()))
        .collect()
}

/// Which icon a listing row uses. With folder icons off a directory falls back to a chevron
/// rather than to nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BreadcrumbEntryIconSource {
    File,
    Folder,
    Chevron,
    None,
}

fn breadcrumb_entry_icon_source(
    is_dir: bool,
    show_file_icons: bool,
    show_folder_icons: bool,
) -> BreadcrumbEntryIconSource {
    if is_dir {
        if show_folder_icons {
            BreadcrumbEntryIconSource::Folder
        } else {
            BreadcrumbEntryIconSource::Chevron
        }
    } else if show_file_icons {
        BreadcrumbEntryIconSource::File
    } else {
        BreadcrumbEntryIconSource::None
    }
}

/// Mirrors `project_panel`'s ordering, visibility and icon settings so the dropdown agrees with
/// the panel. Read from `SettingsContent` independently rather than through `project_panel`'s
/// resolved settings, since `project_panel` depends on `editor` and the reverse would be
/// circular.
#[derive(Clone, Copy, settings::RegisterSetting)]
struct BreadcrumbDirectoryListingSettings {
    sort_mode: settings::ProjectPanelSortMode,
    sort_order: settings::ProjectPanelSortOrder,
    hide_gitignore: bool,
    hide_hidden: bool,
    file_icons: bool,
    folder_icons: bool,
}

impl settings::Settings for BreadcrumbDirectoryListingSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let project_panel = content.project_panel.clone().unwrap();
        Self {
            sort_mode: project_panel.sort_mode.unwrap(),
            sort_order: project_panel.sort_order.unwrap(),
            hide_gitignore: project_panel.hide_gitignore.unwrap(),
            hide_hidden: project_panel.hide_hidden.unwrap(),
            file_icons: project_panel.file_icons.unwrap(),
            folder_icons: project_panel.folder_icons.unwrap(),
        }
    }
}

/// A single row in a breadcrumb directory dropdown: one of `path`'s direct children, sorted the
/// way the project panel orders siblings (see [`BreadcrumbDirectoryListingSettings`]).
struct BreadcrumbDirectoryEntry {
    name: SharedString,
    path: Arc<RelPath>,
    is_dir: bool,
    is_ignored: bool,
    git_summary: GitSummary,
}

/// Lists `path`'s direct children, filtered the way the project panel filters gitignored and
/// hidden entries.
fn breadcrumb_directory_entries(
    project: &Entity<Project>,
    worktree: &Entity<project::Worktree>,
    path: &RelPath,
    cx: &App,
) -> Vec<BreadcrumbDirectoryEntry> {
    let settings = BreadcrumbDirectoryListingSettings::get_global(cx);
    let worktree_snapshot = worktree.read(cx).snapshot();
    let repo_snapshots = project
        .read(cx)
        .git_store()
        .read(cx)
        .display_repo_snapshots(cx);
    let mut entries = project::git_store::git_traversal::ChildEntriesGitIter::new(
        &repo_snapshots,
        &worktree_snapshot,
        path,
    )
    .filter(|entry| !settings.hide_gitignore || !entry.is_ignored)
    .filter(|entry| !settings.hide_hidden || !entry.is_hidden)
    .map(|entry| entry.to_owned())
    .collect::<Vec<_>>();
    entries.sort_by(|a, b| {
        util::paths::compare_rel_paths_by(
            (&*a.path, a.is_file()),
            (&*b.path, b.is_file()),
            settings.sort_mode.into(),
            settings.sort_order.into(),
        )
    });

    entries
        .into_iter()
        .filter_map(|entry| {
            let name = entry.path.file_name()?.to_string();
            Some(BreadcrumbDirectoryEntry {
                name: name.into(),
                path: entry.path.clone(),
                is_dir: entry.is_dir(),
                is_ignored: entry.is_ignored,
                git_summary: entry.git_summary,
            })
        })
        .collect()
}

/// The directory dropdown's contents. Choosing a directory navigates the bar into it; choosing a
/// file opens it.
pub(crate) struct BreadcrumbDirectoryDelegate {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    current_path: Arc<RelPath>,
    /// The open file's own path, so the row leading to it reads as the current one.
    active_path: Option<Arc<RelPath>>,
    entries: Vec<BreadcrumbDirectoryEntry>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    /// Whether any row draws an icon: reserving the column when every icon is off would indent
    /// the list for nothing.
    show_icons: bool,
    _expand_task: gpui::Task<()>,
}

pub(crate) type BreadcrumbDirectoryPicker = Picker<BreadcrumbDirectoryDelegate>;

impl BreadcrumbDirectoryDelegate {
    fn picker(
        editor: WeakEntity<Editor>,
        workspace: WeakEntity<Workspace>,
        worktree_id: WorktreeId,
        current_path: Arc<RelPath>,
        active_path: Option<Arc<RelPath>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<BreadcrumbDirectoryPicker> {
        cx.new(|cx| {
            let delegate = Self {
                editor,
                workspace,
                worktree_id,
                current_path,
                active_path,
                entries: Vec::new(),
                matches: Vec::new(),
                selected_index: 0,
                show_icons: false,
                _expand_task: gpui::Task::ready(()),
            };
            let mut picker = Picker::uniform_list(delegate, window, cx)
                .popover()
                // Narrower than the picker default, which is sized for modals: this lists file
                // names beside their own segment.
                .initial_width(rems(15.));
            picker.delegate.reload_entries(cx);
            picker.delegate.expand_current_path(window, cx);
            picker.delegate.select_active_path();
            picker
        })
    }

    fn project(&self, cx: &App) -> Option<Entity<Project>> {
        Some(self.workspace.upgrade()?.read(cx).project().clone())
    }

    fn worktree(&self, cx: &App) -> Option<Entity<project::Worktree>> {
        self.project(cx)?
            .read(cx)
            .worktree_for_id(self.worktree_id, cx)
    }

    fn reload_entries(&mut self, cx: &App) {
        let (Some(project), Some(worktree)) = (self.project(cx), self.worktree(cx)) else {
            self.entries = Vec::new();
            return;
        };
        self.entries = breadcrumb_directory_entries(&project, &worktree, &self.current_path, cx);

        let settings = BreadcrumbDirectoryListingSettings::get_global(cx);
        self.show_icons = self.entries.iter().any(|entry| {
            breadcrumb_entry_icon_source(entry.is_dir, settings.file_icons, settings.folder_icons)
                != BreadcrumbEntryIconSource::None
        });
    }

    /// Starts on the row leading to the open file, so the dropdown opens where the user already is
    /// rather than at the top of an unrelated directory.
    fn select_active_path(&mut self) {
        let Some(active_path) = self.active_path.as_ref() else {
            return;
        };
        self.selected_index = self
            .matches
            .iter()
            .position(|entry_match| {
                self.entries
                    .get(entry_match.candidate_id)
                    .is_some_and(|entry| active_path.starts_with(&entry.path))
            })
            .unwrap_or(0);
    }

    /// Triggers the same worktree scan the project panel makes when a directory is expanded.
    /// Gitignored directories are never scanned proactively, so without this the dropdown lists
    /// nothing.
    fn expand_current_path(
        &mut self,
        window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) {
        let Some(project) = self.project(cx) else {
            return;
        };
        let Some(entry_id) = self
            .worktree(cx)
            .and_then(|worktree| worktree.read(cx).entry_for_path(&self.current_path))
            .map(|entry| entry.id)
        else {
            return;
        };
        let Some(expand) = project.update(cx, |project, cx| {
            project.expand_entry(self.worktree_id, entry_id, cx)
        }) else {
            return;
        };

        self._expand_task = cx.spawn_in(window, async move |picker, cx| {
            expand.await.log_err();
            picker
                .update_in(cx, |picker, window, cx| picker.refresh(window, cx))
                .ok();
        });
    }

    fn entry_at(&self, index: usize) -> Option<&BreadcrumbDirectoryEntry> {
        self.entries.get(self.matches.get(index)?.candidate_id)
    }
}

impl PickerDelegate for BreadcrumbDirectoryDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "breadcrumb directory picker"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) {
        self.selected_index = index;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search this folder…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some(if self.entries.is_empty() {
            "Empty directory".into()
        } else {
            "No matches".into()
        })
    }

    fn editor_position(&self) -> PickerEditorPosition {
        // Below the list, so a folder with three entries still reads as a menu rather than as a
        // search dialog that happens to have results.
        PickerEditorPosition::End
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) -> Task<()> {
        // Re-read rather than filtered from the previous listing: a pending scan or an edit
        // elsewhere may have changed what this directory holds.
        self.reload_entries(cx);
        let candidates = self
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| StringMatchCandidate::new(index, &entry.name))
            .collect::<Vec<_>>();

        if query.is_empty() {
            self.matches = candidates
                .into_iter()
                .map(|candidate| StringMatch {
                    candidate_id: candidate.id,
                    string: candidate.string,
                    positions: Vec::new(),
                    score: 0.,
                })
                .collect();
            self.select_active_path();
            cx.notify();
            return Task::ready(());
        }

        let executor = cx.background_executor().clone();
        cx.spawn(async move |picker, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                MAX_BREADCRUMB_MENU_ENTRIES,
                &Default::default(),
                executor,
            )
            .await;
            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .ok();
        })
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) {
        let Some(entry) = self.entry_at(self.selected_index) else {
            return;
        };
        let entry_path = entry.path.clone();

        if !entry.is_dir {
            if let Some(workspace) = self.workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .open_path(
                            ProjectPath {
                                worktree_id: self.worktree_id,
                                path: entry_path,
                            },
                            None,
                            true,
                            window,
                            cx,
                        )
                        .detach_and_log_err(cx);
                });
            }
            cx.emit(DismissEvent);
            return;
        }

        // Descending happens on confirm rather than on open: a segment's own dropdown lists its
        // children verbatim, and only choosing a row walks through single-child directories.
        let Some(worktree) = self.worktree(cx) else {
            return;
        };
        let resolved_path = descend_single_child_directories(entry_path, |path| {
            breadcrumb_directory_children(&worktree, path, cx)
        });

        // `current_path` isn't updated in place: the popover is dismissed and reopened under the
        // resolved directory's own segment.
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_breadcrumb_to(self.worktree_id, resolved_path, window, cx);
            });
        }
    }

    /// Returns `None` because stepping into a directory re-anchors the dropdown under that
    /// directory's own segment, rather than the picker swapping its query.
    fn select_child(
        &mut self,
        window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) -> Option<String> {
        if self.entry_at(self.selected_index)?.is_dir {
            self.confirm(false, window, cx);
        }
        None
    }

    fn select_parent(
        &mut self,
        window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) -> Option<String> {
        let parent = self.current_path.parent()?.into_arc();
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_breadcrumb_to(self.worktree_id, parent, window, cx);
            });
        }
        None
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<BreadcrumbDirectoryPicker>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) -> Option<Self::ListItem> {
        let entry = self.entry_at(index)?;
        let listing_settings = BreadcrumbDirectoryListingSettings::get_global(cx);

        let leads_to_active_path = entry.is_dir
            && self
                .active_path
                .as_ref()
                .is_some_and(|active_path| active_path.starts_with(&entry.path));
        let is_active_file =
            !entry.is_dir && self.active_path.as_deref() == Some(entry.path.as_ref());

        let icon_path = match breadcrumb_entry_icon_source(
            entry.is_dir,
            listing_settings.file_icons,
            listing_settings.folder_icons,
        ) {
            BreadcrumbEntryIconSource::File => {
                file_icons::FileIcons::get_icon(entry.path.as_std_path(), cx)
            }
            BreadcrumbEntryIconSource::Folder => file_icons::FileIcons::get_folder_icon(
                leads_to_active_path,
                entry.path.as_std_path(),
                cx,
            ),
            BreadcrumbEntryIconSource::Chevron => {
                // These rows aren't expandable in place — choosing a directory navigates into it
                // rather than expanding it inline — so there's no expanded state to reflect.
                file_icons::FileIcons::get_chevron_icon(false, cx)
            }
            BreadcrumbEntryIconSource::None => None,
        };
        let icon = icon_path.map(Icon::from_path).map(|icon| {
            icon.color(Color::Muted)
                .size(IconSize::Small)
                .into_any_element()
        });

        // The project panel's own mapping, so an entry reads the same colour in both places.
        let label_color = crate::items::entry_git_aware_label_color(
            entry.git_summary,
            entry.is_ignored,
            is_active_file,
        );

        Some(
            ListItem::new(SharedString::from(format!(
                "breadcrumb-directory-entry-{index}"
            )))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .when(self.show_icons, |this| {
                this.start_slot(
                    div()
                        .flex_none()
                        .size(IconSize::Small.rems())
                        .children(icon),
                )
            })
            .child(
                HighlightedLabel::new(entry.name.clone(), self.matches[index].positions.clone())
                    .color(label_color),
            )
            .into_any_element(),
        )
    }
}

/// A path segment whose dropdown lists `path`'s direct children, directories first then
/// alphabetical. Opening it only marks the segment active; single-child chains are resolved only
/// once a row is chosen.
fn render_breadcrumb_directory_segment(
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    path: Arc<RelPath>,
    active_path: Option<Arc<RelPath>>,
    is_active_segment: bool,
    shared_popover_handle: PopoverMenuHandle<BreadcrumbDirectoryPicker>,
    label: gpui::AnyElement,
    index: usize,
) -> gpui::AnyElement {
    let trigger = ButtonLike::new(("breadcrumb-directory", index))
        .style(ButtonStyle::Transparent)
        .size(ButtonSize::None)
        .height(rems_from_px(22.).into())
        .child(label);

    // Only the active segment's popover carries the shared handle `Editor::navigate_breadcrumb_to`
    // reopens through; the rest get a throwaway one.
    let popover_handle = if is_active_segment {
        shared_popover_handle
    } else {
        PopoverMenuHandle::default()
    };

    PopoverMenu::new(("breadcrumb-directory-menu", index))
        .with_handle(popover_handle)
        .trigger(trigger)
        .menu(move |window, cx| {
            let workspace_entity = workspace.upgrade()?;
            workspace_entity
                .read(cx)
                .project()
                .read(cx)
                .worktree_for_id(worktree_id, cx)?;

            if let Some(editor_entity) = editor.upgrade() {
                editor_entity.update(cx, |editor, cx| {
                    editor.open_breadcrumb_navigation(worktree_id, path.clone(), cx);
                });
            }

            let picker = BreadcrumbDirectoryDelegate::picker(
                editor.clone(),
                workspace.clone(),
                worktree_id,
                path.clone(),
                active_path.clone(),
                window,
                cx,
            );
            if let Some(editor_entity) = editor.upgrade() {
                editor_entity.update(cx, |editor, cx| {
                    editor.watch_breadcrumb_dismissal(&picker, worktree_id, path.clone(), cx);
                });
            }
            Some(picker)
        })
        .into_any_element()
}

/// Where a segment sits in [`plan_breadcrumb_layout`]'s drop order when the bar can't fit
/// everything.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BreadcrumbSegmentKind {
    /// The leading project-root segment — kept only once every [`Middle`](Self::Middle) segment
    /// is gone, since it's the one segment whose dropdown reaches top-level entries.
    Root,
    /// A directory component strictly between the root (or the start of the trail) and the file
    /// segment — dropped first, since the "⋯" is exactly what stands in for these.
    Middle,
    /// The open file, or the directory navigated into, dropped only once every `Root` and
    /// `Middle` segment is gone.
    File,
    /// An ancestor symbol, outermost first. The last one is nearest the cursor and never dropped.
    Symbol,
}

/// Assigns each segment its kind purely from its position relative to `file_segment_index`.
pub(crate) fn classify_breadcrumb_segment_kinds(
    segment_count: usize,
    file_segment_index: usize,
    has_root_segment: bool,
) -> Vec<BreadcrumbSegmentKind> {
    (0..segment_count)
        .map(|index| match index.cmp(&file_segment_index) {
            Ordering::Greater => BreadcrumbSegmentKind::Symbol,
            Ordering::Equal => BreadcrumbSegmentKind::File,
            Ordering::Less if has_root_segment && index == 0 => BreadcrumbSegmentKind::Root,
            Ordering::Less => BreadcrumbSegmentKind::Middle,
        })
        .collect()
}

/// Aligns `symbol_segments` 1:1 with `segments`, replacing it wholesale if the lengths disagree:
/// later steps assume equal length and would otherwise panic in `Vec::splice`.
fn align_symbol_segments(
    segments: &[HighlightedText],
    symbol_segments: Vec<Option<BreadcrumbSegmentTarget>>,
) -> Vec<Option<BreadcrumbSegmentTarget>> {
    if symbol_segments.len() == segments.len() {
        symbol_segments
    } else {
        vec![None; segments.len()]
    }
}

/// A safety net against a pathologically deep path. Ordinary breadcrumbs never approach it; the
/// width comparison in `plan_breadcrumb_layout` is what actually fires.
const MAX_BREADCRUMB_SEGMENTS_HARD_CAP: usize = 64;

/// Trims a pathologically long run of `Middle` segments to a bounded prefix and suffix before the
/// width-based planner sees it. The run is always contiguous, so this is a single splice that
/// never touches `Root`, `File` or `Symbol`.
fn hard_cap_breadcrumb_middle_segments(
    mut segments: Vec<HighlightedText>,
    mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>>,
    mut kinds: Vec<BreadcrumbSegmentKind>,
    mut file_segment_index: usize,
) -> (
    Vec<HighlightedText>,
    Vec<Option<BreadcrumbSegmentTarget>>,
    Vec<BreadcrumbSegmentKind>,
    usize,
) {
    let middle_start = kinds
        .iter()
        .position(|kind| *kind == BreadcrumbSegmentKind::Middle);
    let middle_end = kinds
        .iter()
        .rposition(|kind| *kind == BreadcrumbSegmentKind::Middle)
        .map(|index| index + 1);
    let (Some(middle_start), Some(middle_end)) = (middle_start, middle_end) else {
        return (segments, symbol_segments, kinds, file_segment_index);
    };
    if middle_end - middle_start <= MAX_BREADCRUMB_SEGMENTS_HARD_CAP {
        return (segments, symbol_segments, kinds, file_segment_index);
    }

    let half = MAX_BREADCRUMB_SEGMENTS_HARD_CAP / 2;
    let splice_start = middle_start + half;
    let splice_end = middle_end - half;

    segments.splice(
        splice_start..splice_end,
        Some(HighlightedText {
            text: "⋯".into(),
            highlights: vec![],
        }),
    );
    symbol_segments.splice(splice_start..splice_end, Some(None));
    kinds.splice(
        splice_start..splice_end,
        Some(BreadcrumbSegmentKind::Middle),
    );

    // `File` always follows every `Middle` segment, so this splice can only shift its index left.
    file_segment_index -= (splice_end - splice_start) - 1;

    (segments, symbol_segments, kinds, file_segment_index)
}

/// What [`plan_breadcrumb_layout`] decided: `visible` and `ellipses` together partition
/// `0..segment_count`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BreadcrumbLayoutPlan {
    pub(crate) visible: Vec<usize>,
    pub(crate) ellipses: Vec<Range<usize>>,
}

fn total_breadcrumb_layout_width(
    widths: &[Pixels],
    dropped: &[bool],
    ellipsis_width: Pixels,
) -> Pixels {
    let mut total = Pixels::ZERO;
    let mut in_dropped_run = false;
    for (index, &is_dropped) in dropped.iter().enumerate() {
        if is_dropped {
            if !in_dropped_run {
                total += ellipsis_width;
                in_dropped_run = true;
            }
        } else {
            total += widths[index];
            in_dropped_run = false;
        }
    }
    total
}

fn breadcrumb_layout_plan_from_dropped(dropped: &[bool]) -> BreadcrumbLayoutPlan {
    let mut visible = Vec::new();
    let mut ellipses = Vec::new();
    let mut run_start = None;
    for (index, &is_dropped) in dropped.iter().enumerate() {
        if is_dropped {
            run_start.get_or_insert(index);
        } else {
            if let Some(start) = run_start.take() {
                ellipses.push(start..index);
            }
            visible.push(index);
        }
    }
    if let Some(start) = run_start {
        ellipses.push(start..dropped.len());
    }
    BreadcrumbLayoutPlan { visible, ellipses }
}

/// Drops segments cheapest first until the row fits: every `Middle`, then `Root`, then `File`,
/// then `Symbol` outermost first. The last segment is never a candidate, so the bar never empties.
/// A pure function of the measured widths, so it is testable without a `Window`.
pub(crate) fn plan_breadcrumb_layout(
    widths: &[Pixels],
    kinds: &[BreadcrumbSegmentKind],
    ellipsis_width: Pixels,
    available_width: Pixels,
) -> BreadcrumbLayoutPlan {
    debug_assert_eq!(widths.len(), kinds.len());
    let segment_count = widths.len();
    if segment_count == 0 {
        return BreadcrumbLayoutPlan {
            visible: Vec::new(),
            ellipses: Vec::new(),
        };
    }

    let mut dropped = vec![false; segment_count];
    if total_breadcrumb_layout_width(widths, &dropped, ellipsis_width) <= available_width {
        return breadcrumb_layout_plan_from_dropped(&dropped);
    }

    let last_index = segment_count - 1;
    let mut drop_order = Vec::with_capacity(segment_count - 1);
    for kind in [
        BreadcrumbSegmentKind::Middle,
        BreadcrumbSegmentKind::Root,
        BreadcrumbSegmentKind::File,
        BreadcrumbSegmentKind::Symbol,
    ] {
        drop_order.extend(
            kinds
                .iter()
                .enumerate()
                .filter(|(index, segment_kind)| *index != last_index && **segment_kind == kind)
                .map(|(index, _)| index),
        );
    }

    for index in drop_order {
        dropped[index] = true;
        if total_breadcrumb_layout_width(widths, &dropped, ellipsis_width) <= available_width {
            break;
        }
    }

    breadcrumb_layout_plan_from_dropped(&dropped)
}

/// Whether the leading segment offers navigation at all: `false` for a buffer with no project
/// path, and for a single-file worktree, which has no tree to browse.
fn breadcrumb_path_is_navigable(
    has_project_path: bool,
    worktree_is_single_file: Option<bool>,
) -> bool {
    has_project_path && !worktree_is_single_file.unwrap_or(false)
}

/// One segment, resolved ahead of the render pass so the element never reaches back into
/// `Editor` state.
struct PreparedBreadcrumbSegment {
    kind: BreadcrumbSegmentKind,
    label: HighlightedText,
    target: Option<BreadcrumbSegmentTarget>,
    /// Whether this is the dirty file's own segment. Precomputed because the `'static`
    /// `BreadcrumbsRow` can't hold `active_item` or `TabBarSettings`.
    dirty_filename_style: bool,
    /// Icon before the segment's name, which is what tells the file from the directories leading
    /// to it.
    icon: Option<SharedString>,
    /// Text colour: the path stays muted so the file it leads to reads as the subject.
    label_color: Color,
}

/// Per-segment slot width, measured once per render. `shape_line` is cached by text and font, so
/// this is a handful of lookups rather than a reshape. Counting one arrow per segment
/// overestimates slightly, which can only make the row collapse earlier than needed, never later.
struct BreadcrumbSegmentMetrics {
    widths: Vec<Pixels>,
    ellipsis_width: Pixels,
}

/// Runs describing how `render_segment` will actually paint `segment`'s label. The bold file name
/// `apply_dirty_filename_style` adds is wider than the plain style, so measuring everything at the
/// base weight would plan the row narrower than it gets painted and let it overflow.
fn segment_text_runs(
    segment: &PreparedBreadcrumbSegment,
    text: &str,
    text_style: &gpui::TextStyle,
) -> Vec<gpui::TextRun> {
    let Some(filename_offset) = segment
        .dirty_filename_style
        .then(|| dirty_filename_offset(&segment.label))
        .flatten()
    else {
        return vec![text_style.to_run(text.len())];
    };

    let mut bold_style = text_style.clone();
    bold_style.font_weight = FontWeight::BOLD;
    if filename_offset == 0 {
        return vec![bold_style.to_run(text.len())];
    }
    vec![
        text_style.to_run(filename_offset),
        bold_style.to_run(text.len() - filename_offset),
    ]
}

fn breadcrumb_layout_plan_width(
    widths: &[Pixels],
    plan: &BreadcrumbLayoutPlan,
    ellipsis_width: Pixels,
) -> Pixels {
    let mut dropped = vec![false; widths.len()];
    for range in &plan.ellipses {
        for index in range.clone() {
            dropped[index] = true;
        }
    }
    total_breadcrumb_layout_width(widths, &dropped, ellipsis_width)
}

/// A custom `Element` rather than an `h_flex` because how many segments fit can only be decided
/// once GPUI hands back the row's real width: measured layout in `request_layout`, real children
/// built in `prepaint`, the same pattern `UniformList` uses.
struct BreadcrumbsRow {
    segments: Vec<PreparedBreadcrumbSegment>,
    editor: Option<WeakEntity<Editor>>,
}

/// Names the per-segment hover group, so the highlight lands on the label and not on the
/// separator after it.
const BREADCRUMB_SEGMENT_GROUP: &str = "breadcrumb-segment";

/// Horizontal padding around a segment's label, inside its hover highlight.
const BREADCRUMB_LABEL_PADDING: Pixels = px(4.);

/// Matches the project panel's own entry icons, so the two read as the same tree.
const BREADCRUMB_ICON_SIZE: IconSize = IconSize::Small;

/// Only the file's segment gets an icon. Directories get none, and symbols name code rather than
/// an entry in the tree.
fn breadcrumb_segment_icon(
    target: &Option<BreadcrumbSegmentTarget>,
    file_path: Option<&RelPath>,
    cx: &App,
) -> Option<SharedString> {
    if !BreadcrumbDirectoryListingSettings::get_global(cx).file_icons {
        return None;
    }
    match target {
        Some(BreadcrumbSegmentTarget::Symbol { item: None, .. }) => {
            file_icons::FileIcons::get_icon(file_path?.as_std_path(), cx)
        }
        _ => None,
    }
}

fn breadcrumb_separator_width(window: &Window) -> Pixels {
    IconSize::XSmall.rems().to_pixels(window.rem_size())
}

impl BreadcrumbsRow {
    /// The UI font rather than the buffer font: the bar reads as chrome, not as code.
    fn effective_text_style(&self, window: &Window) -> gpui::TextStyle {
        window.text_style()
    }

    fn measure(&self, window: &mut Window) -> BreadcrumbSegmentMetrics {
        let text_style = self.effective_text_style(window);
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let gap = window.rem_size() * 0.25;

        let arrow_width = breadcrumb_separator_width(window);

        let ellipsis_run = text_style.to_run("⋯".len());
        let ellipsis_label_width = window
            .text_system()
            .shape_line("⋯".into(), font_size, &[ellipsis_run], None)
            .width();
        let ellipsis_width =
            ellipsis_label_width + BREADCRUMB_LABEL_PADDING * 2. + arrow_width + gap * 2.;

        let widths = self
            .segments
            .iter()
            .map(|segment| {
                let text = flatten_text_for_single_line_display(&segment.label.text);
                let runs = segment_text_runs(segment, &text, &text_style);
                let label_width = window
                    .text_system()
                    .shape_line(text.into(), font_size, &runs, None)
                    .width();
                let icon_width = if segment.icon.is_some() {
                    BREADCRUMB_ICON_SIZE.rems().to_pixels(window.rem_size()) + gap
                } else {
                    Pixels::ZERO
                };
                icon_width + label_width + BREADCRUMB_LABEL_PADDING * 2. + arrow_width + gap * 2.
            })
            .collect();

        BreadcrumbSegmentMetrics {
            widths,
            ellipsis_width,
        }
    }

    /// Positions are in the final rendered sequence rather than the raw segment index, since
    /// that's the sequence whose last edge has nothing to point at.
    fn with_separator(
        &self,
        position: usize,
        last_position: usize,
        content: gpui::AnyElement,
        interactive: bool,
        cx: &App,
    ) -> gpui::AnyElement {
        // Only the label is painted on hover. The separator stays clickable, belonging to the
        // segment on its left, but isn't part of that segment's name.
        let label = div()
            .px(BREADCRUMB_LABEL_PADDING)
            .rounded_sm()
            // Multi buffer excerpt headers render the same trail as plain text, with no dropdowns
            // to open, so lighting it up on hover would advertise a click that does nothing.
            .when(interactive, |this| {
                this.group_hover(BREADCRUMB_SEGMENT_GROUP, |style| {
                    style.bg(cx.theme().colors().ghost_element_hover)
                })
            })
            .child(content);

        if position == last_position {
            return label.into_any_element();
        }
        h_flex()
            .gap_1()
            .child(label)
            .child(
                // Nudged down a pixel: breadcrumb text is mostly lowercase, whose visual centre
                // sits below the geometric one a centred chevron lands on.
                div().relative().top(px(2.)).child(
                    Icon::new(IconName::ChevronRight)
                        .size(IconSize::XSmall)
                        .color(Color::Placeholder),
                ),
            )
            .into_any_element()
    }

    fn wrap_segment(&self, element: gpui::AnyElement) -> gpui::AnyElement {
        div()
            .group(BREADCRUMB_SEGMENT_GROUP)
            .child(element)
            .into_any_element()
    }

    fn render_segment(
        &self,
        index: usize,
        position: usize,
        last_position: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let segment = &self.segments[index];
        let mut text_style = self.effective_text_style(window);
        text_style.color = segment.label_color.color(cx);

        let text = if segment.dirty_filename_style
            && let Some(styled_element) =
                apply_dirty_filename_style(&segment.label, &text_style, cx)
        {
            styled_element
        } else {
            StyledText::new(flatten_text_for_single_line_display(&segment.label.text))
                .with_default_highlights(&text_style, segment.label.highlights.clone())
                .into_any()
        };

        let content = match &segment.icon {
            Some(icon) => h_flex()
                .gap_1()
                .child(
                    // The same optical nudge the separator chevron gets.
                    div().relative().top(px(2.)).child(
                        Icon::from_path(icon.clone())
                            .color(Color::Muted)
                            .size(BREADCRUMB_ICON_SIZE),
                    ),
                )
                .child(text)
                .into_any_element(),
            None => text,
        };
        let interactive = segment.target.is_some() && self.editor.is_some();
        let label = self.with_separator(position, last_position, content, interactive, cx);

        let element = match (segment.target.clone(), self.editor.clone()) {
            (Some(BreadcrumbSegmentTarget::Symbol { buffer_id, item }), Some(editor)) => {
                render_breadcrumb_symbol_segment(editor, buffer_id, item, label, index)
            }
            (
                Some(BreadcrumbSegmentTarget::Directory {
                    worktree_id,
                    path,
                    active_path,
                    is_active_segment,
                }),
                Some(editor),
            ) => {
                let Some(upgraded_editor) = editor.upgrade() else {
                    return label;
                };
                let Some(workspace) = upgraded_editor
                    .read(cx)
                    .workspace()
                    .map(|workspace| workspace.downgrade())
                else {
                    return label;
                };
                let shared_popover_handle = upgraded_editor.read(cx).breadcrumb_popover_handle();
                render_breadcrumb_directory_segment(
                    editor,
                    workspace,
                    worktree_id,
                    path,
                    active_path,
                    is_active_segment,
                    shared_popover_handle,
                    label,
                    index,
                )
            }
            _ => return label,
        };
        self.wrap_segment(element)
    }

    /// The inert "⋯" standing for a collapsed run: no popover of its own, since everything it
    /// hides is reachable by widening the window or through the segments beside it.
    fn render_ellipsis(&self, position: usize, last_position: usize, cx: &App) -> gpui::AnyElement {
        let content = Label::new("⋯").color(Color::Placeholder).into_any_element();
        self.with_separator(position, last_position, content, false, cx)
    }
}

struct BreadcrumbsRowPrepaintState {
    children: Vec<gpui::AnyElement>,
}

impl gpui::IntoElement for BreadcrumbsRow {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for BreadcrumbsRow {
    type RequestLayoutState = BreadcrumbSegmentMetrics;
    type PrepaintState = BreadcrumbsRowPrepaintState;

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
        _cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let metrics = self.measure(window);
        let natural_width = metrics
            .widths
            .iter()
            .fold(Pixels::ZERO, |total, width| total + *width);
        let line_height = window.text_style().line_height_in_pixels(window.rem_size());

        let widths = metrics.widths.clone();
        let ellipsis_width = metrics.ellipsis_width;
        let kinds: Vec<BreadcrumbSegmentKind> = self.segments.iter().map(|s| s.kind).collect();

        // A flex item's automatic minimum size is its min-content size, so answering `MinContent`
        // with the whole trail would stop the parent ever offering less. The row can always fall
        // back to one segment plus an ellipsis.
        let mut style = Style::default();
        style.min_size.width = px(0.).into();

        let layout_id = window.request_measured_layout(
            style,
            move |known_dimensions, available_space, _window, _cx| {
                let width = known_dimensions
                    .width
                    .unwrap_or(match available_space.width {
                        AvailableSpace::Definite(available_width) => {
                            let plan = plan_breadcrumb_layout(
                                &widths,
                                &kinds,
                                ellipsis_width,
                                available_width,
                            );
                            breadcrumb_layout_plan_width(&widths, &plan, ellipsis_width)
                        }
                        AvailableSpace::MinContent => widths
                            .last()
                            .copied()
                            .unwrap_or(ellipsis_width)
                            .max(ellipsis_width),
                        AvailableSpace::MaxContent => natural_width,
                    });
                let height = known_dimensions.height.unwrap_or(line_height);
                size(width, height)
            },
        );

        (layout_id, metrics)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        metrics: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let kinds: Vec<BreadcrumbSegmentKind> = self.segments.iter().map(|s| s.kind).collect();
        let plan = plan_breadcrumb_layout(
            &metrics.widths,
            &kinds,
            metrics.ellipsis_width,
            bounds.size.width,
        );

        enum FinalItem {
            Segment(usize),
            Ellipsis,
        }

        let segment_count = kinds.len();
        let mut sequence = Vec::with_capacity(plan.visible.len() + plan.ellipses.len());
        let mut index = 0;
        while index < segment_count {
            if let Some(range) = plan.ellipses.iter().find(|range| range.start == index) {
                sequence.push(FinalItem::Ellipsis);
                index = range.end;
            } else {
                sequence.push(FinalItem::Segment(index));
                index += 1;
            }
        }

        let last_position = sequence.len().saturating_sub(1);
        let gap = window.rem_size() * 0.25;
        let mut x = bounds.origin.x;
        let mut children = Vec::with_capacity(sequence.len());
        for (position, item) in sequence.into_iter().enumerate() {
            let mut element = match item {
                FinalItem::Segment(index) => {
                    self.render_segment(index, position, last_position, window, cx)
                }
                FinalItem::Ellipsis => self.render_ellipsis(position, last_position, cx),
            };
            let available_space = size(
                AvailableSpace::MaxContent,
                AvailableSpace::Definite(bounds.size.height),
            );
            let element_size = element.layout_as_root(available_space, window, cx);
            element.prepaint_at(point(x, bounds.origin.y), window, cx);
            x += element_size.width + gap;
            children.push(element);
        }

        // Every segment has registered its popover handle by now, which is what a pending
        // re-anchor waits for.
        if let Some(editor) = self.editor.as_ref().and_then(WeakEntity::upgrade)
            && editor.read(cx).breadcrumb_pending_reanchor()
        {
            editor.update(cx, |editor, cx| {
                editor.reanchor_breadcrumb_popover(window, cx);
            });
        }

        BreadcrumbsRowPrepaintState { children }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        for child in &mut prepaint.children {
            child.paint(window, cx);
        }
    }
}

pub fn render_breadcrumb_text(
    mut segments: Vec<HighlightedText>,
    prefix: Option<gpui::AnyElement>,
    active_item: &dyn ItemHandle,
    multibuffer_header: bool,
    cx: &App,
) -> gpui::AnyElement {
    // min_w_0 because a flex item's minimum size defaults to its content's, which would stop
    // `BreadcrumbsRow` from ever being told to collapse.
    let element = h_flex().flex_grow_1().min_w_0().text_ui(cx);

    let editor = active_item
        .downcast::<Editor>()
        .map(|editor| editor.downgrade());

    // Aligned 1:1 with `segments` once the path splitting below runs. The buffer id comes from
    // the singleton rather than `outline_symbols_at_cursor`, so the path segment still gets a
    // menu when the cursor sits outside any symbol.
    let mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>> = Vec::new();
    // Stays 0 whenever the path splitting below doesn't run.
    let mut file_segment_index = 0usize;
    // Whether path splitting inserted a leading root segment, so
    // `classify_breadcrumb_segment_kinds` can tell it from an ordinary `Middle` component.
    let mut has_root_segment = false;
    // The buffer whose outline the segment dropdowns will need, so hovering the bar can start
    // fetching it before any of them is opened.
    let mut outline_buffer_id = None;
    let mut file_path_for_icon: Option<Arc<RelPath>> = None;
    let mut file_status = None;

    if !multibuffer_header
        && let Some(editor_entity) = editor.as_ref().and_then(WeakEntity::upgrade)
    {
        let editor_ref = editor_entity.read(cx);
        if let Some(buffer) = editor_ref.buffer().read(cx).as_singleton() {
            let buffer_id = buffer.read(cx).remote_id();
            outline_buffer_id = Some(buffer_id);
            let mut path_split = false;

            // The real open file's path, independent of any navigation below: it is both the
            // fallback bar and the `active_path` submenus keep highlighting towards.
            let real_project_path = active_item.project_path(cx);
            file_path_for_icon = real_project_path
                .as_ref()
                .map(|project_path| project_path.path.clone());
            file_status = editor_ref
                .project()
                .zip(real_project_path.as_ref())
                .and_then(|(project, project_path)| {
                    project.read(cx).project_path_git_status(project_path, cx)
                });
            // Set once a directory row is chosen (see `Editor::navigate_breadcrumb_to`); while
            // set, the bar shows that directory's path instead of the file's.
            let navigation = editor_ref.breadcrumb_navigation().cloned();
            let navigated = navigation
                .as_ref()
                .is_some_and(|navigation| navigation.navigated);
            let active_segment = navigation
                .as_ref()
                .map(|navigation| navigation.active_path.clone());

            let is_navigable = breadcrumb_path_is_navigable(
                real_project_path.is_some(),
                real_project_path.as_ref().and_then(|project_path| {
                    editor_ref
                        .project()
                        .and_then(|project| {
                            project
                                .read(cx)
                                .worktree_for_id(project_path.worktree_id, cx)
                        })
                        .map(|worktree| worktree.read(cx).is_single_file())
                }),
            );

            // The root segment is added unconditionally so sibling top-level directories stay
            // reachable from the root, not only from the file's own path. It can't double up with
            // the root `breadcrumbs_inner` already names, because this branch splices that
            // segment away wholesale.
            if is_navigable
                && !segments.is_empty()
                && let Some(project) = editor_ref.project()
            {
                let split = if let Some(navigation) = navigation
                    .as_ref()
                    .filter(|navigation| navigation.navigated)
                {
                    project
                        .read(cx)
                        .worktree_for_id(navigation.worktree_id, cx)
                        .map(|worktree| {
                            breadcrumb_path_segments(
                                navigation.worktree_id,
                                worktree.read(cx).root_name_str(),
                                &navigation.active_path,
                                real_project_path.as_ref().map(|path| path.path.clone()),
                                None,
                                active_segment.as_deref(),
                            )
                        })
                } else if let Some(project_path) = real_project_path.as_ref()
                    && let Some(worktree) = project
                        .read(cx)
                        .worktree_for_id(project_path.worktree_id, cx)
                {
                    Some(breadcrumb_path_segments(
                        project_path.worktree_id,
                        worktree.read(cx).root_name_str(),
                        &project_path.path,
                        Some(project_path.path.clone()),
                        Some(buffer_id),
                        active_segment.as_deref(),
                    ))
                } else {
                    None
                };

                if let Some((path_labels, path_targets)) = split {
                    file_segment_index = path_labels.len() - 1;
                    let replace_range = if navigated { 0..segments.len() } else { 0..1 };
                    segments.splice(replace_range, path_labels);
                    symbol_segments = path_targets;
                    path_split = true;
                    has_root_segment = true;
                }
            }

            if !path_split && is_navigable {
                symbol_segments.push(Some(BreadcrumbSegmentTarget::Symbol {
                    buffer_id,
                    item: None,
                }));
            } else if !path_split {
                symbol_segments.push(None);
            }

            if !navigated {
                let ancestors = editor_ref
                    .outline_symbols_at_cursor
                    .as_ref()
                    .filter(|(id, _)| *id == buffer_id)
                    .map(|(_, ancestors)| ancestors.as_slice())
                    .unwrap_or_default();
                symbol_segments.extend(ancestors.iter().cloned().map(|item| {
                    Some(BreadcrumbSegmentTarget::Symbol {
                        buffer_id,
                        item: Some(item),
                    })
                }));
            }
        }
    }

    let symbol_segments = align_symbol_segments(&segments, symbol_segments);
    let kinds =
        classify_breadcrumb_segment_kinds(segments.len(), file_segment_index, has_root_segment);
    let (segments, symbol_segments, kinds, file_segment_index) =
        hard_cap_breadcrumb_middle_segments(segments, symbol_segments, kinds, file_segment_index);

    let apply_dirty_filename_style =
        !workspace::TabBarSettings::get_global(cx).show && active_item.is_dirty(cx);

    let prepared_segments = segments
        .into_iter()
        .zip(symbol_segments)
        .zip(kinds)
        .enumerate()
        .map(|(index, ((label, target), kind))| {
            let icon = breadcrumb_segment_icon(&target, file_path_for_icon.as_deref(), cx);
            let label_color = if kind == BreadcrumbSegmentKind::File {
                crate::element::file_status_label_color(file_status)
            } else {
                Color::Muted
            };
            PreparedBreadcrumbSegment {
                kind,
                label,
                target,
                dirty_filename_style: apply_dirty_filename_style && index == file_segment_index,
                icon,
                label_color,
            }
        })
        .collect();

    let row = BreadcrumbsRow {
        segments: prepared_segments,
        editor: editor.clone(),
    };

    let breadcrumbs_stack = div()
        .min_w_0()
        .when(multibuffer_header, |this| {
            this.pl_2()
                .border_l_1()
                .border_color(cx.theme().colors().border.opacity(0.6))
        })
        .child(row)
        .into_any_element();

    let breadcrumbs = if let Some(prefix) = prefix {
        h_flex()
            .min_w_0()
            .gap_1p5()
            .child(prefix)
            .child(breadcrumbs_stack)
            .into_any_element()
    } else {
        breadcrumbs_stack
    };

    let has_project_path = active_item.project_path(cx).is_some();

    match editor {
        Some(editor) => element
            .id("breadcrumb_container")
            .when_some(outline_buffer_id, |this, buffer_id| {
                let editor = editor.clone();
                this.on_hover(move |hovered, _, cx| {
                    if *hovered {
                        editor
                            .update(cx, |editor, cx| {
                                editor.prefetch_breadcrumb_outline(buffer_id, cx)
                            })
                            .ok();
                    }
                })
            })
            // A plain row rather than a `ButtonLike`: `ButtonLike` renders `flex_none`, which would
            // stop the bar from ever being told to shrink.
            .child(
                h_flex()
                    .h(rems_from_px(22.))
                    .px_1()
                    .min_w_0()
                    .child(breadcrumbs)
                    .when(!multibuffer_header && has_project_path, |this| {
                        this.on_mouse_down(gpui::MouseButton::Right, {
                            let editor = editor.clone();
                            move |_, _, cx| {
                                if let Some(abs_path) = editor.upgrade().and_then(|editor| {
                                    editor.update(cx, |editor, cx| editor.target_file_abs_path(cx))
                                }) && let Some(path_str) = abs_path.to_str()
                                {
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        path_str.to_string(),
                                    ));
                                }
                            }
                        })
                    }),
            )
            .into_any_element(),
        None => element
            .h(rems_from_px(22.)) // Match the height and padding of the `ButtonLike` in the other arm.
            .pl_1()
            .child(breadcrumbs)
            .into_any_element(),
    }
}

/// Byte offset where the file name starts in a path label, shared between painting and measuring
/// so the two can't drift apart.
fn dirty_filename_offset(segment: &HighlightedText) -> Option<usize> {
    let filename = std::path::Path::new(segment.text.as_ref()).file_name()?;
    segment.text.rfind(filename.to_string_lossy().as_ref())
}

fn apply_dirty_filename_style(
    segment: &HighlightedText,
    text_style: &gpui::TextStyle,
    cx: &App,
) -> Option<gpui::AnyElement> {
    let text = flatten_text_for_single_line_display(&segment.text);

    let filename_position = dirty_filename_offset(segment)?;

    let bold_weight = FontWeight::BOLD;
    let default_color = Color::Default.color(cx);

    if filename_position == 0 {
        let mut filename_style = text_style.clone();
        filename_style.font_weight = bold_weight;
        filename_style.color = default_color;

        return Some(
            StyledText::new(text)
                .with_default_highlights(&filename_style, [])
                .into_any(),
        );
    }

    let highlight_style = gpui::HighlightStyle {
        font_weight: Some(bold_weight),
        color: Some(default_color),
        ..Default::default()
    };

    let highlight = vec![(filename_position..text.len(), highlight_style)];
    Some(
        StyledText::new(text)
            .with_default_highlights(text_style, highlight)
            .into_any(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Selects the row for `path` in an open directory picker and confirms it, standing in for a
    /// click on that row.
    #[cfg(test)]
    fn confirm_breadcrumb_row(
        picker: &Entity<BreadcrumbDirectoryPicker>,
        path: &str,
        cx: &mut VisualTestContext,
    ) {
        use util::rel_path::rel_path;
        picker.update_in(cx, |picker, window, cx| {
            let index = picker
                .delegate
                .matches
                .iter()
                .position(|entry_match| {
                    picker.delegate.entries[entry_match.candidate_id]
                        .path
                        .as_ref()
                        == rel_path(path)
                })
                .expect("row is listed");
            picker.delegate.selected_index = index;
            picker.delegate.confirm(false, window, cx);
        });
    }

    use crate::MultiBuffer;
    use gpui::{TestAppContext, VisualTestContext};

    #[test]
    fn test_breadcrumb_path_is_navigable() {
        // Untitled/unsaved buffer: no project path at all.
        assert!(!breadcrumb_path_is_navigable(false, None));
        assert!(!breadcrumb_path_is_navigable(false, Some(false)));

        // File opened outside any real worktree — Zed represents it as a single-file worktree.
        assert!(!breadcrumb_path_is_navigable(true, Some(true)));

        // Ordinary file inside a real worktree.
        assert!(breadcrumb_path_is_navigable(true, Some(false)));

        // Worktree couldn't be resolved (e.g. removed mid-session): preserves the prior
        // fallback-to-symbols behavior rather than assuming non-navigable.
        assert!(breadcrumb_path_is_navigable(true, None));
    }

    #[test]
    fn test_flatten_text_for_single_line_display_preserves_byte_offsets() {
        // Byte-offset highlight ranges computed against `original` must stay valid against the
        // flattened result — verify by locating the same substring by offset in both strings.
        let original = "fn outer() {\n    inner()\n}";
        let flattened = flatten_text_for_single_line_display(original);

        assert_eq!(flattened, "fn outer() {     inner() }");
        assert_eq!(flattened.len(), original.len());

        let inner_offset = original.find("inner").unwrap();
        assert_eq!(
            &flattened[inner_offset..inner_offset + "inner".len()],
            "inner",
        );
    }

    #[test]
    fn test_breadcrumb_entry_icon_source() {
        assert_eq!(
            breadcrumb_entry_icon_source(true, true, true),
            BreadcrumbEntryIconSource::Folder
        );
        assert_eq!(
            breadcrumb_entry_icon_source(true, false, true),
            BreadcrumbEntryIconSource::Folder
        );
        assert_eq!(
            breadcrumb_entry_icon_source(true, true, false),
            BreadcrumbEntryIconSource::Chevron
        );
        assert_eq!(
            breadcrumb_entry_icon_source(true, false, false),
            BreadcrumbEntryIconSource::Chevron
        );
        assert_eq!(
            breadcrumb_entry_icon_source(false, true, true),
            BreadcrumbEntryIconSource::File
        );
        assert_eq!(
            breadcrumb_entry_icon_source(false, true, false),
            BreadcrumbEntryIconSource::File
        );
        assert_eq!(
            breadcrumb_entry_icon_source(false, false, true),
            BreadcrumbEntryIconSource::None
        );
        assert_eq!(
            breadcrumb_entry_icon_source(false, false, false),
            BreadcrumbEntryIconSource::None
        );
    }

    #[test]
    fn test_sibling_outline_indices_top_level() {
        // struct A; struct B; struct C; — all depth 0, no parent.
        let depths = [0, 0, 0];
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 1), vec![0, 1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![0, 1, 2]);
    }

    #[test]
    fn test_sibling_outline_indices_nested() {
        // `impl A { fn one; fn two }` then `impl B { fn three }`, i.e. [0, 1, 1, 0, 1].
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(sibling_outline_indices(&depths, 1), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 4), vec![4]);
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 3]);
        assert_eq!(sibling_outline_indices(&depths, 3), vec![0, 3]);
    }

    #[test]
    fn test_sibling_outline_indices_uneven_depths() {
        // Tree-sitter outlines can jump straight from depth 0 to depth 2; the parent of a
        // depth-2 item is the nearest preceding shallower item, not a nonexistent depth-1 one.
        let depths = [0, 2, 2, 0];
        assert_eq!(sibling_outline_indices(&depths, 1), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 3]);
    }

    #[test]
    fn test_sibling_outline_indices_single_item() {
        let depths = [0];
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0]);
    }

    #[test]
    fn test_sibling_outline_indices_out_of_bounds() {
        let depths = [0, 0];
        assert_eq!(sibling_outline_indices(&depths, 5), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_top_level() {
        // struct A; struct B; struct C; — all depth 0, none has children.
        let depths = [0, 0, 0];
        assert_eq!(child_outline_indices(&depths, 0), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 1), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 2), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_nested() {
        // `impl A { fn one; fn two }` then `impl B { fn three }`, i.e. [0, 1, 1, 0, 1].
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(child_outline_indices(&depths, 0), vec![1, 2]);
        assert_eq!(child_outline_indices(&depths, 3), vec![4]);
        // Leaf items have no children.
        assert_eq!(child_outline_indices(&depths, 1), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 2), Vec::<usize>::new());
        assert_eq!(child_outline_indices(&depths, 4), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_uneven_depths() {
        // The depth-2 fields are still direct children of the depth-0 struct even with no
        // depth-1 item between them — parenthood follows the nearest shallower item.
        let depths = [0, 2, 2, 0];
        assert_eq!(child_outline_indices(&depths, 0), vec![1, 2]);
        assert_eq!(child_outline_indices(&depths, 3), Vec::<usize>::new());
    }

    #[test]
    fn test_child_outline_indices_out_of_bounds() {
        let depths = [0, 0];
        assert_eq!(child_outline_indices(&depths, 5), Vec::<usize>::new());
    }

    #[test]
    fn test_top_level_outline_indices() {
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(top_level_outline_indices(&depths), vec![0, 3]);

        let depths_uneven = [0, 2, 2, 0];
        assert_eq!(top_level_outline_indices(&depths_uneven), vec![0, 3]);

        let depths_empty: [usize; 0] = [];
        assert_eq!(
            top_level_outline_indices(&depths_empty),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn test_breadcrumb_path_prefixes_nested() {
        use util::rel_path::rel_path;

        assert_eq!(
            breadcrumb_path_prefixes(rel_path("a/b/c.rs")),
            vec![rel_path("a"), rel_path("a/b"), rel_path("a/b/c.rs")]
        );
    }

    #[test]
    fn test_breadcrumb_path_prefixes_top_level_file() {
        use util::rel_path::rel_path;

        assert_eq!(
            breadcrumb_path_prefixes(rel_path("file.rs")),
            vec![rel_path("file.rs")]
        );
    }

    #[test]
    fn test_breadcrumb_path_prefixes_empty() {
        assert_eq!(
            breadcrumb_path_prefixes(RelPath::empty()),
            Vec::<&RelPath>::new()
        );
    }

    #[test]
    fn test_breadcrumb_path_segments_nested() {
        use util::rel_path::rel_path;

        let worktree_id = WorktreeId::from_usize(0);
        let buffer_id = BufferId::new(1).unwrap();
        let path = rel_path("src/main/kotlin/Foo.kt").into_arc();

        let (labels, targets) = breadcrumb_path_segments(
            worktree_id,
            "my-project",
            &path,
            Some(path.clone()),
            Some(buffer_id),
            None,
        );

        assert_eq!(
            labels.iter().map(|l| l.text.as_ref()).collect::<Vec<_>>(),
            vec!["my-project", "src", "main", "kotlin", "Foo.kt"]
        );
        assert_eq!(targets.len(), labels.len());

        match targets[0].as_ref().unwrap() {
            BreadcrumbSegmentTarget::Directory {
                worktree_id: id,
                path,
                active_path,
                is_active_segment,
            } => {
                assert_eq!(*id, worktree_id);
                assert_eq!(path.as_unix_str(), "");
                assert_eq!(
                    active_path.as_deref(),
                    Some(rel_path("src/main/kotlin/Foo.kt"))
                );
                assert!(!is_active_segment);
            }
            other => panic!("expected root directory target, got {other:?}"),
        }

        for (index, expected_dir) in ["src", "src/main", "src/main/kotlin"]
            .into_iter()
            .enumerate()
        {
            match targets[index + 1].as_ref().unwrap() {
                BreadcrumbSegmentTarget::Directory { path, .. } => {
                    assert_eq!(path.as_unix_str(), expected_dir);
                }
                other => panic!("expected directory target, got {other:?}"),
            }
        }

        match targets.last().unwrap().as_ref().unwrap() {
            BreadcrumbSegmentTarget::Symbol {
                buffer_id: id,
                item,
            } => {
                assert_eq!(*id, buffer_id);
                assert!(item.is_none());
            }
            other => panic!("expected symbol target for the file segment, got {other:?}"),
        }
    }

    #[test]
    fn test_breadcrumb_path_segments_top_level_file() {
        use util::rel_path::rel_path;

        let worktree_id = WorktreeId::from_usize(0);
        let buffer_id = BufferId::new(1).unwrap();
        let path = rel_path("Foo.kt").into_arc();

        let (labels, targets) = breadcrumb_path_segments(
            worktree_id,
            "my-project",
            &path,
            Some(path.clone()),
            Some(buffer_id),
            None,
        );

        assert_eq!(
            labels.iter().map(|l| l.text.as_ref()).collect::<Vec<_>>(),
            vec!["my-project", "Foo.kt"]
        );
        assert!(matches!(
            targets[0].as_ref().unwrap(),
            BreadcrumbSegmentTarget::Directory { .. }
        ));
        assert!(matches!(
            targets[1].as_ref().unwrap(),
            BreadcrumbSegmentTarget::Symbol { item: None, .. }
        ));
    }

    #[test]
    fn test_breadcrumb_path_segments_navigated_directory_marks_active_segment() {
        use util::rel_path::rel_path;

        let worktree_id = WorktreeId::from_usize(0);
        let path = rel_path("src/main").into_arc();

        let (labels, targets) = breadcrumb_path_segments(
            worktree_id,
            "ihavenever",
            &path,
            None,
            None,
            Some(rel_path("src/main")),
        );

        assert_eq!(
            labels.iter().map(|l| l.text.as_ref()).collect::<Vec<_>>(),
            vec!["ihavenever", "src", "main"]
        );

        // The terminal segment is a directory target — not a `Symbol` target — because a
        // navigated bar's last segment is a directory the user browsed to, not the open file.
        let active_flags: Vec<bool> = targets
            .iter()
            .map(|target| match target.as_ref().unwrap() {
                BreadcrumbSegmentTarget::Directory {
                    is_active_segment, ..
                } => *is_active_segment,
                BreadcrumbSegmentTarget::Symbol { .. } => {
                    panic!("navigated directory path should have no symbol target")
                }
            })
            .collect();
        assert_eq!(active_flags, vec![false, false, true]);
    }

    #[test]
    fn test_breadcrumb_path_segments_drill_down_includes_root_and_lists_own_children() {
        use util::rel_path::rel_path;

        let worktree_id = WorktreeId::from_usize(0);
        let path = rel_path("src/main/Foo.kt").into_arc();

        let (labels, targets) = breadcrumb_path_segments(
            worktree_id,
            "my-project",
            &path,
            Some(path.clone()),
            None,
            None,
        );

        // The leading project-root segment is present — it's the only way to reach top-level
        // siblings in this mode.
        assert_eq!(
            labels.iter().map(|l| l.text.as_ref()).collect::<Vec<_>>(),
            vec!["my-project", "src", "main", "Foo.kt"]
        );

        // Clicking a segment lists its own children: `src`'s dropdown target is `src` itself,
        // `src/main`'s is `src/main` itself.
        let list_paths: Vec<String> = targets
            .iter()
            .map(|target| match target.as_ref().unwrap() {
                BreadcrumbSegmentTarget::Directory { path, .. } => path.as_unix_str().to_string(),
                BreadcrumbSegmentTarget::Symbol { .. } => "<symbol>".to_string(),
            })
            .collect();
        assert_eq!(list_paths, vec!["", "src", "src/main", "src/main/Foo.kt"]);
    }

    #[test]
    fn test_descend_single_child_directories_stops_at_fork() {
        use util::rel_path::rel_path;

        let tree: collections::HashMap<&str, Vec<(&str, bool)>> =
            collections::HashMap::from_iter([
                ("a", vec![("a/b", true)]),
                ("a/b", vec![("a/b/c", true), ("a/b/d", true)]),
            ]);

        let result = descend_single_child_directories(rel_path("a").into_arc(), |path| {
            tree.get(path.as_unix_str())
                .into_iter()
                .flatten()
                .map(|(child, is_dir)| (rel_path(child).into_arc(), *is_dir))
                .collect()
        });

        assert_eq!(result, rel_path("a/b").into_arc());
    }

    #[test]
    fn test_descend_single_child_directories_stops_short_of_lone_file() {
        use util::rel_path::rel_path;

        let tree: collections::HashMap<&str, Vec<(&str, bool)>> = collections::HashMap::from_iter(
            [("repository", vec![("repository/Repositories.kt", false)])],
        );

        let result = descend_single_child_directories(rel_path("repository").into_arc(), |path| {
            tree.get(path.as_unix_str())
                .into_iter()
                .flatten()
                .map(|(child, is_dir)| (rel_path(child).into_arc(), *is_dir))
                .collect()
        });

        // Stops at `repository` rather than descending into the file it alone contains — the
        // user still clicks `Repositories.kt` themselves.
        assert_eq!(result, rel_path("repository").into_arc());
    }

    #[test]
    fn test_descend_single_child_directories_caps_depth() {
        use util::rel_path::rel_path;

        // Each directory has exactly one child, forever — simulates a symlink cycle or a
        // pathologically deep chain. The cap must stop the walk rather than looping forever.
        let result = descend_single_child_directories(rel_path("a").into_arc(), |path| {
            vec![(
                rel_path(&format!("{}/x", path.as_unix_str())).into_arc(),
                true,
            )]
        });

        // The walk must terminate rather than loop forever; the cap bounds how many `/x` segments
        // it can add on top of the starting `a`.
        assert_eq!(
            result.as_unix_str().matches('/').count(),
            MAX_BREADCRUMB_DESCENT_DEPTH
        );
    }

    #[test]
    fn test_descend_single_child_directories_stops_at_empty_directory() {
        use util::rel_path::rel_path;

        let result = descend_single_child_directories(rel_path("empty").into_arc(), |_| Vec::new());

        assert_eq!(result, rel_path("empty").into_arc());
    }

    #[test]
    fn test_align_symbol_segments_realigns_divergent_lengths() {
        let segments: Vec<HighlightedText> = (0..3)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        // Shorter than `segments` on purpose: models a navigation whose worktree failed to resolve.
        let symbol_segments = vec![Some(BreadcrumbSegmentTarget::Symbol {
            buffer_id: BufferId::new(1).unwrap(),
            item: None,
        })];

        let symbol_segments = align_symbol_segments(&segments, symbol_segments);

        assert_eq!(symbol_segments.len(), 3);
        assert!(symbol_segments.iter().all(Option::is_none));
    }

    #[test]
    fn test_classify_breadcrumb_segment_kinds() {
        // Root, two directory components, file, two ancestor symbols.
        let kinds = classify_breadcrumb_segment_kinds(6, 3, true);
        assert_eq!(
            kinds,
            vec![
                BreadcrumbSegmentKind::Root,
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::File,
                BreadcrumbSegmentKind::Symbol,
                BreadcrumbSegmentKind::Symbol,
            ]
        );

        // No root segment (`Siblings` mode, or the path wasn't split at all): the first segment
        // is `Middle`, not `Root`.
        let kinds = classify_breadcrumb_segment_kinds(3, 1, false);
        assert_eq!(
            kinds,
            vec![
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::File,
                BreadcrumbSegmentKind::Symbol,
            ]
        );

        // The path wasn't split (unsaved buffer): a single `File` segment, `file_segment_index`
        // stays `0`.
        let kinds = classify_breadcrumb_segment_kinds(1, 0, false);
        assert_eq!(kinds, vec![BreadcrumbSegmentKind::File]);
    }

    /// Without `align_symbol_segments`, a short `symbol_segments` (from a worktree that failed to
    /// resolve) panics: splice ranges below are computed from `segments.len()` alone.
    #[test]
    fn test_hard_cap_breadcrumb_middle_segments_does_not_panic_on_divergent_symbol_segments() {
        let segments: Vec<HighlightedText> = (0..100)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        // Only one entry — far shorter than `segments` — modeling the divergence described above.
        let symbol_segments = vec![Some(BreadcrumbSegmentTarget::Symbol {
            buffer_id: BufferId::new(1).unwrap(),
            item: None,
        })];
        let symbol_segments = align_symbol_segments(&segments, symbol_segments);
        assert_eq!(symbol_segments.len(), segments.len());

        // A root segment, 98 middle components, then the file at the end.
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), 99, true);

        let (segments, symbol_segments, kinds, file_segment_index) =
            hard_cap_breadcrumb_middle_segments(segments, symbol_segments, kinds, 99);

        // Root (1) + hard-capped middle (32 kept prefix + 1 "⋯" + 32 kept suffix = 65) + file (1)
        // = 67.
        assert_eq!(segments.len(), 67);
        assert_eq!(symbol_segments.len(), segments.len());
        assert_eq!(kinds.len(), segments.len());
        assert_eq!(file_segment_index, 66);
        assert_eq!(kinds[file_segment_index], BreadcrumbSegmentKind::File);
    }

    #[test]
    fn test_hard_cap_breadcrumb_middle_segments_leaves_ordinary_input_untouched() {
        let segments: Vec<HighlightedText> = (0..6)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![None; segments.len()];
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), 3, true);

        let (segments, symbol_segments, kinds, file_segment_index) =
            hard_cap_breadcrumb_middle_segments(segments, symbol_segments, kinds, 3);

        assert_eq!(segments.len(), 6);
        assert_eq!(symbol_segments.len(), 6);
        assert_eq!(kinds.len(), 6);
        assert_eq!(file_segment_index, 3);
    }

    /// Widths (in pixels) for a synthetic six-segment trail modeling the report's own example:
    /// root, four middle directory components, the file, then two ancestor symbols (outermost
    /// first, cursor-nearest last) — `root, a, b, c, d, file.kt, Class, fun method`.
    fn sample_breadcrumb_widths_and_kinds() -> (Vec<Pixels>, Vec<BreadcrumbSegmentKind>) {
        use BreadcrumbSegmentKind::*;
        let widths = vec![
            px(60.),  // root
            px(30.),  // a
            px(30.),  // b
            px(30.),  // c
            px(30.),  // d
            px(80.),  // file.kt
            px(90.),  // Class
            px(120.), // fun method
        ];
        let kinds = vec![Root, Middle, Middle, Middle, Middle, File, Symbol, Symbol];
        (widths, kinds)
    }

    #[test]
    fn test_plan_breadcrumb_layout_everything_fits() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();
        let total: Pixels = widths.iter().fold(Pixels::ZERO, |sum, w| sum + *w);

        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), total);

        assert_eq!(plan.visible, (0..widths.len()).collect::<Vec<_>>());
        assert!(plan.ellipses.is_empty());
    }

    #[test]
    fn test_plan_breadcrumb_layout_drops_middle_before_root_before_file_before_outer_symbols() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();

        // Narrow enough that all 4 middle components must go (dropping only 3 still leaves it too
        // wide), but root, file, and both symbols still fit once all 4 are gone.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(380.));
        assert_eq!(plan.visible, vec![0, 5, 6, 7]);
        assert_eq!(plan.ellipses, vec![1..5]);

        // Narrow enough that root has to go too, but file and both symbols still fit.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(340.));
        assert_eq!(plan.visible, vec![5, 6, 7]);
        assert_eq!(plan.ellipses, vec![0..5]);

        // Narrower still: file goes too, leaving just the symbol chain.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(230.));
        assert_eq!(plan.visible, vec![6, 7]);
        assert_eq!(plan.ellipses, vec![0..6]);

        // Narrower yet: the outer symbol goes too, leaving only the innermost — never dropped.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(140.));
        assert_eq!(plan.visible, vec![7]);
        assert_eq!(plan.ellipses, vec![0..7]);
    }

    #[test]
    fn test_plan_breadcrumb_layout_degenerate_case_always_keeps_the_last_segment() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();

        // Not even the innermost symbol alone fits — still renders it rather than nothing.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(1.));
        assert_eq!(plan.visible, vec![7]);
        assert_eq!(plan.ellipses, vec![0..7]);
    }

    #[test]
    fn test_plan_breadcrumb_layout_single_segment_never_collapses() {
        let plan =
            plan_breadcrumb_layout(&[px(500.)], &[BreadcrumbSegmentKind::File], px(20.), px(1.));
        assert_eq!(plan.visible, vec![0]);
        assert!(plan.ellipses.is_empty());
    }

    #[test]
    fn test_plan_breadcrumb_layout_empty_input() {
        let plan = plan_breadcrumb_layout(&[], &[], px(20.), px(500.));
        assert!(plan.visible.is_empty());
        assert!(plan.ellipses.is_empty());
    }

    /// Without the fix, choosing a directory row panics: `choose` runs while the browser entity
    /// is leased by `cx.listener`, and re-anchoring the popover synchronously updates that same
    /// leased entity again ("cannot update ... while it is already being updated").
    #[gpui::test]
    async fn test_choosing_breadcrumb_directory_row_does_not_double_lease_browser(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::cell::RefCell;
        use util::path;
        use workspace::Workspace;

        // `PopoverMenu` only wires itself up during a real layout pass, so this needs an honest
        // `Render` mounted as a window root rather than a bare drawn element. The `Editor` is
        // created inside this same window too, since `on_next_frame` calls from re-anchoring
        // would otherwise get stuck on a window nothing here drains.
        struct Harness {
            handle: PopoverMenuHandle<BreadcrumbDirectoryPicker>,
            editor: Entity<Editor>,
            workspace: WeakEntity<Workspace>,
            worktree_id: WorktreeId,
            captured_browser: Rc<RefCell<Option<Entity<BreadcrumbDirectoryPicker>>>>,
        }

        impl Render for Harness {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                let editor = self.editor.downgrade();
                let workspace = self.workspace.clone();
                let worktree_id = self.worktree_id;
                let captured_browser = self.captured_browser.clone();
                PopoverMenu::new("test-breadcrumb-directory-menu")
                    .with_handle(self.handle.clone())
                    .trigger(ButtonLike::new("trigger").child(div()))
                    .menu(move |window, cx| {
                        // Marking the segment active must happen in the same opening sequence
                        // that creates the browser below, not as a separate step — a separate
                        // call would dismiss the already-open browser via `handle` first.
                        if let Some(editor_entity) = editor.upgrade() {
                            editor_entity.update(cx, |editor, cx| {
                                editor.open_breadcrumb_navigation(
                                    worktree_id,
                                    RelPath::empty().into(),
                                    cx,
                                );
                            });
                        }
                        let browser = BreadcrumbDirectoryDelegate::picker(
                            editor.clone(),
                            workspace.clone(),
                            worktree_id,
                            RelPath::empty().into(),
                            None,
                            window,
                            cx,
                        );
                        *captured_browser.borrow_mut() = Some(browser.clone());
                        Some(browser)
                    })
            }
        }

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "dir_a": {
                    "child1.txt": "",
                    "child2.txt": "",
                },
                "file.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let captured_browser: Rc<RefCell<Option<Entity<BreadcrumbDirectoryPicker>>>> =
            Rc::default();

        let harness_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            // The real handle `Editor::navigate_breadcrumb_to` re-anchors, exactly like
            // `render_breadcrumb_directory_segment` uses for the active segment — not a fresh
            // handle of the test's own, which `navigate_breadcrumb_to` would have no way to reach.
            let handle = editor.read(cx).breadcrumb_popover_handle();
            Harness {
                handle,
                editor,
                workspace: workspace.downgrade(),
                worktree_id,
                captured_browser: captured_browser.clone(),
            }
        });
        let editor = harness_window
            .read_with(cx, |harness, _| harness.editor.clone())
            .unwrap();
        let handle = harness_window
            .read_with(cx, |harness, _| harness.handle.clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*harness_window, cx);

        // Wires `handle` up to the popover's state, like a real breadcrumb bar render pass does.
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });

        // Opening through `handle.show` makes `browser` the same entity `handle` holds internally
        // — required for the repro, since the panic is about reaching back into a leased entity.
        cx.update(|window, cx| handle.show(window, cx));
        let browser = captured_browser.borrow().clone().expect("popover opened");
        assert!(handle.is_deployed());
        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation().is_some(),
                "opening the popover marked this segment active"
            );
        });

        // Choosing while `browser` is still leased by this `update` call is the actual repro.
        confirm_breadcrumb_row(&browser, "dir_a", cx);

        editor.read_with(cx, |editor, _| {
            let navigation = editor
                .breadcrumb_navigation()
                .expect("navigate_breadcrumb_to set a session");
            assert_eq!(navigation.active_path.as_unix_str(), "dir_a");
            assert!(navigation.navigated);
            assert!(
                editor.breadcrumb_reanchoring,
                "re-anchor is still in flight — the popover isn't back open yet"
            );
        });
        assert!(
            !handle.is_deployed(),
            "the pre-navigation popover was dismissed synchronously by the defer"
        );

        // Re-anchoring hides then re-shows the popover once the new active segment lays out; by
        // then `browser` is no longer leased.
        cx.update(|window, cx| {
            editor.update(cx, |editor, cx| {
                editor.reanchor_breadcrumb_popover(window, cx);
            });
        });

        editor.read_with(cx, |editor, _| {
            assert!(
                !editor.breadcrumb_reanchoring,
                "re-anchor finishes within a few frames"
            );
        });
        assert!(
            handle.is_deployed(),
            "the popover reopened under the resolved directory's own segment"
        );
    }

    /// The whole flow driven by `menu::` actions rather than by simulated keystrokes: move the
    /// selection, submit it, and end up inside the chosen directory.
    #[gpui::test]
    async fn test_breadcrumb_directory_picker_navigates_from_the_keyboard(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha": { "one.txt": "", "two.txt": "" },
                "beta": { "three.txt": "" },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        struct Harness {
            picker: Entity<BreadcrumbDirectoryPicker>,
            editor: Entity<Editor>,
        }
        impl Render for Harness {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                self.picker.clone()
            }
        }

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let harness_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let picker = BreadcrumbDirectoryDelegate::picker(
                editor.downgrade(),
                workspace.downgrade(),
                worktree_id,
                RelPath::empty().into(),
                None,
                window,
                cx,
            );
            Harness { picker, editor }
        });
        let (picker, editor) = harness_window
            .read_with(cx, |harness, _| {
                (harness.picker.clone(), harness.editor.clone())
            })
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*harness_window, cx);
        cx.run_until_parked();

        picker.update_in(cx, |picker, window, cx| {
            window.focus(&picker.focus_handle(cx), cx);
        });
        cx.run_until_parked();

        // The listing opens on `alpha`; one step down lands on `beta`.
        cx.dispatch_action(menu::SelectNext);
        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .entry_at(picker.delegate.selected_index)
                    .map(|entry| entry.name.as_ref()),
                Some("beta"),
            );
        });

        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();

        editor.read_with(cx, |editor, _| {
            let navigation = editor
                .breadcrumb_navigation()
                .expect("confirming a directory row navigates the bar into it");
            assert_eq!(navigation.active_path.as_unix_str(), "beta");
            assert!(navigation.navigated);
        });
    }

    /// Worktrees never scan gitignored directories proactively, so without the expansion call a
    /// dropdown that only reads the snapshot lists nothing. One level per dropdown opened.
    #[gpui::test]
    async fn test_breadcrumb_directory_browser_expands_nested_gitignored_directories(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::{path, rel_path::rel_path};
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".gitignore": "ignored_dir\n",
                "ignored_dir": { "nested": { "file.txt": "" } },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
        cx.run_until_parked();

        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(&project, &worktree, rel_path("ignored_dir"), cx)
        });
        assert!(
            entries.is_empty(),
            "nothing under a gitignored directory is scanned until something asks for it"
        );

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let open_dropdown_at = |path: &'static str, cx: &mut VisualTestContext| {
            let browser = editor_window
                .update(cx, |_, window, cx| {
                    BreadcrumbDirectoryDelegate::picker(
                        editor.downgrade(),
                        workspace.downgrade(),
                        worktree_id,
                        rel_path(path).into_arc(),
                        None,
                        window,
                        cx,
                    )
                })
                .unwrap();
            cx.run_until_parked();
            let entries = cx.update(|_, cx| {
                breadcrumb_directory_entries(&project, &worktree, rel_path(path), cx)
            });
            drop(browser);
            entries
                .into_iter()
                .map(|entry| entry.name.to_string())
                .collect::<Vec<_>>()
        };

        assert_eq!(
            open_dropdown_at("ignored_dir", cx),
            vec!["nested".to_string()],
            "opening the dropdown scans one level into the gitignored directory"
        );
        assert_eq!(
            open_dropdown_at("ignored_dir/nested", cx),
            vec!["file.txt".to_string()],
            "and the level below it once that one is opened too"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_browser_choose_descends_single_child_directories(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use crate::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;
        use workspace::Workspace;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a": { "b": { "c.txt": "" } },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        // `a`'s only child is `b`, whose only child is the file `c.txt` — a single-child chain —
        // so choosing `a` descends straight to `a/b`, stopping short of the file.
        let browser = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbDirectoryDelegate::picker(
                    editor.downgrade(),
                    workspace.downgrade(),
                    worktree_id,
                    RelPath::empty().into(),
                    None,
                    window,
                    cx,
                )
            })
            .unwrap();
        confirm_breadcrumb_row(&browser, "a", cx);
        editor.read_with(cx, |editor, _| {
            assert_eq!(
                editor
                    .breadcrumb_navigation()
                    .expect("navigate_breadcrumb_to set a session")
                    .active_path
                    .as_unix_str(),
                "a/b",
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_sorts_like_project_panel(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "Apple": { "leaf.txt": "" },
                "banana.txt": "",
                "Cherry.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        cx.run_until_parked();

        // Default settings match the project panel's own default sort.
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec!["Apple", "banana.txt", "Cherry.txt"],
        );

        // Reusing `compare_rel_paths_by` means our ordering tracks `project_panel.sort_mode`/
        // `sort_order` the same way the panel's does.
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    let project_panel = settings.project_panel.get_or_insert_default();
                    project_panel.sort_mode = Some(settings::ProjectPanelSortMode::FilesFirst);
                    project_panel.sort_order = Some(settings::ProjectPanelSortOrder::Unicode);
                });
            });
        });
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec!["Cherry.txt", "banana.txt", "Apple"],
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_honors_hide_gitignore_setting(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".gitignore": "ignored.txt",
                "kept.txt": "",
                "ignored.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        cx.run_until_parked();

        // `hide_gitignore` defaults to `false`: shown dimmed rather than hidden, like the panel.
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        let ignored_entry = entries
            .iter()
            .find(|entry| entry.name.as_ref() == "ignored.txt")
            .expect("gitignored entry is shown, not hidden, by default");
        assert!(ignored_entry.is_ignored);

        // Same setting the project panel reads — keeps the two views in agreement.
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project_panel
                        .get_or_insert_default()
                        .hide_gitignore = Some(true);
                });
            });
        });
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert!(
            !entries
                .iter()
                .any(|entry| entry.name.as_ref() == "ignored.txt"),
            "hide_gitignore should drop the ignored entry entirely, not just dim it",
        );
        assert!(
            entries
                .iter()
                .any(|entry| entry.name.as_ref() == "kept.txt"),
            "non-ignored entries stay listed"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_honors_hide_hidden_setting(cx: &mut TestAppContext) {
        use crate::editor_tests::init_test;
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use util::path;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".hidden": "",
                "kept.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        cx.run_until_parked();

        // `hide_hidden` defaults to `false`, matching the project panel.
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert!(
            entries.iter().any(|entry| entry.name.as_ref() == ".hidden"),
            "hidden entry is shown by default"
        );

        // Same setting the project panel reads — keeps the two views in agreement.
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project_panel.get_or_insert_default().hide_hidden = Some(true);
                });
            });
        });
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert!(
            !entries.iter().any(|entry| entry.name.as_ref() == ".hidden"),
            "hide_hidden should drop the hidden entry entirely, not just dim it",
        );
        assert!(
            entries
                .iter()
                .any(|entry| entry.name.as_ref() == "kept.txt"),
            "non-hidden entries stay listed"
        );
    }
}
