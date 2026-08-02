//! Breadcrumb path/symbol navigation: turns the breadcrumb bar's segments into clickable
//! dropdown targets, sharing the project panel's ordering and gitignore treatment (see
//! `BreadcrumbDirectoryListingSettings`) rather than reimplementing them.

use super::*;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::Task;
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::Project;
use ui::{HighlightedLabel, ListItemSpacing};

/// Computes, for each item in a flat pre-order outline (given as `depths`), the index of its
/// parent — the nearest preceding item with a smaller depth (`None` for top-level items).
/// "Nearest preceding item with a smaller depth" is used rather than `depth - 1`, because
/// tree-sitter outlines can have uneven depth jumps (e.g. going straight from depth 0 to depth
/// 2). `sibling_outline_indices`, `child_outline_indices` and `top_level_outline_indices` all
/// derive from this single pass so they can't disagree about what "parent" means.
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

/// Indices of `target_index`'s siblings — items at the same depth that share its nearest
/// shallower ancestor (`target_index` included in the result).
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

/// Indices of the items directly nested inside `target_index` — one level deeper, using the
/// same "nearest shallower ancestor" notion of parenthood as `sibling_outline_indices`. A
/// node's siblings are its parent's children, which is why the breadcrumb dropdown prefers
/// this (drilling down) and only falls back to `sibling_outline_indices` when a node has no
/// children.
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

/// What a single breadcrumb segment's dropdown drills into, computed alongside `segments` in
/// [`render_breadcrumb_text`] so the render pass for each segment (which only has `&App`) can
/// stay ignorant of where the data came from.
#[derive(Clone, Debug)]
pub(crate) enum BreadcrumbSegmentTarget {
    /// A path or symbol segment whose dropdown lists document symbols: `item.is_none()` for the
    /// file segment itself (lists the buffer's top-level symbols), `Some` for an ancestor symbol
    /// segment (lists its children, falling back to its siblings).
    Symbol {
        buffer_id: BufferId,
        item: Option<OutlineItem<Anchor>>,
    },
    /// A directory segment whose dropdown lists `path`'s contents. `active_path` is the open
    /// buffer's path — the same value at every directory in its ancestor chain — so a listing at
    /// any depth can mark the entry the breadcrumb passes through.
    Directory {
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        active_path: Option<Arc<RelPath>>,
        /// Whether this segment's dropdown is the one currently open, so its render can draw the
        /// box IntelliJ's navigation bar shows around such a segment.
        is_active_segment: bool,
    },
}

/// Splits `path` into its component prefixes, root first and `path` itself last — e.g.
/// `a/b/c.rs` becomes `[a, a/b, a/b/c.rs]`. Used to turn the breadcrumb's leading path segment
/// into one clickable component per path element (each directory, then the file), mirroring
/// IntelliJ's navigation bar. Empty for the empty path (a worktree root, which is never itself a
/// buffer's path).
fn breadcrumb_path_prefixes(path: &RelPath) -> Vec<&RelPath> {
    let mut prefixes: Vec<&RelPath> = path
        .ancestors()
        .filter(|prefix| !prefix.is_empty())
        .collect();
    prefixes.reverse();
    prefixes
}

/// Builds the breadcrumb's leading path segments: the worktree root, then one per path component,
/// each listing its own children.
///
/// The root segment is what makes top-level directories reachable, since no other segment lists
/// them. `terminal_buffer_id` is `Some` when `path` is the open file, whose segment lists document
/// symbols instead of directory contents.
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

/// Flattens `text` to a single display line. The highlight ranges that travel with breadcrumb text
/// are byte offsets into the unflattened string, so the replacement has to be the same length as
/// what it replaces or every range past the first newline shifts.
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

/// The list a breadcrumb symbol segment drops down: the symbols the segment can move to, filtered
/// by the query typed into the picker's search field.
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

    /// Whether any listed symbol is the segment's own, i.e. whether the checkmark column will ever
    /// be filled. Symbol menus usually have no current row, and reserving the column regardless
    /// indents every row for a checkmark that never appears.
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

    /// Rendered the way the outline picker and outline panel render outline items: with the
    /// symbol's own syntax highlighting, so `fun resolveEnv` reads as code and the symbol's kind
    /// is legible from keyword coloring rather than a flat label. The fuzzy match positions are
    /// deliberately not drawn on top of it, since the two highlight sets would fight.
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

/// Renders a single breadcrumb segment as a clickable element that opens a dropdown drilling into
/// the outline: `target`'s children if it has any, else its siblings, else — for the leading path
/// segment, `target: None` — the buffer's top-level symbols.
fn render_breadcrumb_symbol_segment(
    editor: WeakEntity<Editor>,
    buffer_id: BufferId,
    target: Option<OutlineItem<Anchor>>,
    label: gpui::AnyElement,
    index: usize,
) -> gpui::AnyElement {
    // `PopoverMenu::trigger` installs its own click handler, and `ButtonLike` wraps whichever
    // handler it ends up with in `cx.stop_propagation()`. That is what keeps a click here from
    // also reaching the enclosing "toggle outline view" button and opening the outline picker
    // behind the popover.
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
            // With nothing to drill into — a buffer whose language has no outline, or no grammar
            // at all — fall through to the outline picker the whole breadcrumb bar used to open,
            // instead of flashing an empty popover or swallowing the click.
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

/// Caps how many entries a breadcrumb dropdown lists — both a directory dropdown's children
/// (`Worktree` entries already scanned and held in memory, so this bounds layout cost, not I/O)
/// and a symbol dropdown's siblings/children (see `Editor::breadcrumb_symbol_menu_items`), which a
/// generated file's flat top-level listing can make just as large. Exists to keep one popover
/// legible and its layout cheap.
pub(crate) const MAX_BREADCRUMB_MENU_ENTRIES: usize = 200;

/// Caps how many single-child directories [`descend_single_child_directories`] will walk through
/// before giving up and treating the current directory as the fork. Guards against pathological
/// depth and against looping forever if a worktree ever contained a symlink cycle.
const MAX_BREADCRUMB_DESCENT_DEPTH: usize = 64;

/// Walks down from `start` through directories that have exactly one child directory, the way
/// IntelliJ's navigation bar skips straight past them instead of making the user click through a
/// chain with no alternative — e.g. a `com/example/app/` chain holding nothing but the next single
/// subdirectory collapses straight to the first one with an actual choice (zero or multiple
/// children) to show. Stops there, and also stops one directory short of descending into a file:
/// when the only remaining child is a file, that file is left as the sole entry of the directory
/// returned, so the user still has to click it themselves rather than having it opened for them —
/// scrolling through a chain of single-item directories is done on the user's behalf, opening a
/// file is not.
///
/// `child_entries` is injected rather than reading a worktree directly so this stays pure and
/// unit-testable; production callers back it with a real worktree snapshot.
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

/// Backs [`descend_single_child_directories`] with a live worktree: every direct child of `path`,
/// as `(path, is_dir)` pairs.
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

/// Which icon source a breadcrumb directory dropdown row should draw from, given
/// `project_panel.file_icons`/`project_panel.folder_icons` (see
/// [`BreadcrumbDirectoryListingSettings`]) and whether the row is a directory. Mirrors the
/// project/git/outline panels: turning folder icons off falls back to a
/// chevron rather than hiding the icon slot entirely (see
/// `crates/project_panel/src/project_panel.rs`'s `EntryKind` icon match), while turning file
/// icons off just leaves files with no icon. Factored out as pure selection logic — with no
/// `cx`/icon-theme lookup — so it's testable without a GPUI context.
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

/// Mirrors a subset of `project_panel`'s settings — ordering (`sort_mode`, `sort_order`),
/// visibility (`hide_gitignore`, `hide_hidden`) and icon display (`file_icons`, `folder_icons`)
/// — so the breadcrumb dropdown's listing agrees with the panel's, including when the user
/// changes those settings — see `cmp_worktree_entries` and the `hide_gitignore`/`hide_hidden`
/// uses in `crates/project_panel/src/project_panel.rs`.
///
/// This can't just call into `project_panel::ProjectPanelSettings` and reuse its already-resolved
/// fields: `project_panel` depends on `editor` (for `entry_git_aware_label_color`, reused below),
/// so `editor` depending back on `project_panel` for this would be circular. Reading the same
/// `project_panel` section of `SettingsContent` a second time, independently, keeps this dropdown
/// in sync with the panel without inverting that dependency.
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

/// Lists `path`'s direct children for a breadcrumb directory dropdown. Gitignored and hidden
/// entries are dropped when `project_panel.hide_gitignore`/`hide_hidden` are set, matching the
/// project panel, and each entry carries the git summary the panel colors rows by.
fn breadcrumb_directory_entries(
    project: &Entity<Project>,
    worktree: &Entity<project::Worktree>,
    path: &RelPath,
    cx: &App,
) -> Vec<BreadcrumbDirectoryEntry> {
    let settings = BreadcrumbDirectoryListingSettings::get_global(cx);
    let worktree_snapshot = worktree.read(cx).snapshot();
    // The panel's own scoped traversal (`project_panel.rs`'s sibling lookup uses the same one),
    // which walks just this directory's children rather than the whole worktree.
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

/// The list a breadcrumb directory segment drops down: `current_path`'s direct children, filtered
/// by the query typed into the picker's search field. Choosing a directory navigates the bar into
/// it and reopens the dropdown under its own segment (see [`Editor::navigate_breadcrumb_to`]),
/// mirroring IntelliJ's navigation bar popup, which has no submenu tree either. Choosing a file
/// opens it.
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
    /// Whether any row will draw an icon. Rows reserve the icon's width to stay aligned with each
    /// other, but reserving it when the settings turn every icon off just indents the whole list
    /// for nothing.
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
                // names next to the segment it drops from, not search results across a project.
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

    /// Asks the worktree to scan this directory's children, the same call the project panel makes
    /// when a directory is expanded. Gitignored directories are never scanned proactively, so
    /// without this their dropdown lists nothing at all.
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
        // Re-read rather than filter the previous listing: a pending `expand_current_path` scan,
        // or an edit elsewhere, may have changed what this directory holds.
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

        // Descending here rather than when the dropdown opens: a segment's own dropdown lists its
        // children verbatim, and only choosing a row walks through a chain of single-child
        // directories. The walk stops at a directory whose only child is a file, leaving the open
        // to the user's click on that file.
        let Some(worktree) = self.worktree(cx) else {
            return;
        };
        let resolved_path = descend_single_child_directories(entry_path, |path| {
            breadcrumb_directory_children(&worktree, path, cx)
        });

        // Doesn't update `current_path` in place: the popover is about to be dismissed and
        // reopened under the resolved directory's own segment, with a picker of its own.
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_breadcrumb_to(self.worktree_id, resolved_path, window, cx);
            });
        }
    }

    /// Stepping into the selected directory without leaving the keyboard. Returns `None` because
    /// the bar re-anchors the dropdown under the directory's own segment instead of the picker
    /// swapping its own query, which is what the `Some(query)` contract is for.
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

        // The project panel's own mapping, so a modified or untracked entry reads the same in both
        // places. Only the label is colored; the panel leaves icons muted too.
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

/// Renders a single breadcrumb path segment as a clickable element that opens a dropdown listing
/// `path`'s direct children verbatim — subdirectories and files, directories first then
/// alphabetical — so the whole project tree is reachable from the breadcrumb bar the way
/// IntelliJ's navigation bar reaches it, without switching to the project panel. Opening the
/// dropdown only marks this segment active (see [`Editor::open_breadcrumb_navigation`]); it does
/// not itself skip through single-child directories or otherwise change the bar — that happens
/// only once a row is chosen, inside [`BreadcrumbDirectoryDelegate::confirm`], which is also what
/// replaces the bar with the resolved directory (see [`Editor::navigate_breadcrumb_to`]).
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

    // The active segment's `PopoverMenu` carries the handle `Editor::navigate_breadcrumb_to`
    // reopens the dropdown through once the bar re-renders with it marked active; every other
    // segment gets its own throwaway handle, same as a plain independently-openable dropdown.
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

/// Where a breadcrumb segment sits in [`plan_breadcrumb_layout`]'s drop-priority order when the
/// bar doesn't have room to show everything. Assigned by [`classify_breadcrumb_segment_kinds`]
/// purely from a segment's position relative to `file_segment_index`; the actual survive/drop
/// decision lives entirely in `plan_breadcrumb_layout`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BreadcrumbSegmentKind {
    /// The leading project-root segment — kept only once every [`Middle`](Self::Middle) segment
    /// is gone, since it's the one segment whose dropdown reaches top-level entries.
    Root,
    /// A directory component strictly between the root (or the start of the trail) and the file
    /// segment — dropped first, since the "⋯" is exactly what stands in for these.
    Middle,
    /// The trail's endpoint segment: the open file itself, or — while breadcrumb navigation has
    /// drilled into a directory — that directory. Dropped only once every [`Root`](Self::Root)
    /// and [`Middle`](Self::Middle) segment is already gone.
    File,
    /// An ancestor-symbol segment following the file, ordered outermost (shallowest) first and
    /// innermost (nearest the cursor) last. `plan_breadcrumb_layout` never drops the last one —
    /// it's what the user is actually reading — and drops earlier ones first when it must drop
    /// any symbol at all.
    Symbol,
}

/// Assigns each of `segment_count` breadcrumb segments its [`BreadcrumbSegmentKind`], purely from
/// position: everything before `file_segment_index` is [`Root`](BreadcrumbSegmentKind::Root)
/// (only index `0`, and only when `has_root_segment`) or [`Middle`](BreadcrumbSegmentKind::Middle);
/// `file_segment_index` itself is [`File`](BreadcrumbSegmentKind::File); everything after is
/// [`Symbol`](BreadcrumbSegmentKind::Symbol). Relies on the segment order `render_breadcrumb_text`
/// already establishes — root, then path components, then the file, then ancestor symbols
/// nearest-cursor-last (see `breadcrumb_path_segments` and `Editor::breadcrumbs_inner`).
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

/// Aligns `symbol_segments` 1:1 with `segments`, discarding it (replacing wholesale with `None`s
/// the same length as `segments`) if the lengths disagree.
///
/// `symbol_segments` is built by a caller that tracks navigation state independently of
/// `segments` (see `render_breadcrumb_text`'s comment on why the two can diverge — e.g. a
/// navigated worktree that no longer resolves), so the two vectors aren't guaranteed to start out
/// the same length. Every later step — hard-capping, width measurement, applying the layout plan —
/// indexes both vectors by the same original index and computes splice ranges purely from
/// `segments.len()`; running any of them against a shorter `symbol_segments` would make
/// `Vec::splice` panic. Aligning first makes the invariant hold structurally instead of by caller
/// convention, and keeps it testable without a live rendered editor.
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

/// Safety net against pathological input — a path many thousands of components deep — bounding
/// the cost of [`plan_breadcrumb_layout`], which re-sums the whole row for every segment it
/// considers dropping. Ordinary breadcrumbs, even deeply nested real-world ones, stay far under
/// this; it's `plan_breadcrumb_layout`'s width comparison that actually fires in normal use, not
/// this cap.
const MAX_BREADCRUMB_SEGMENTS_HARD_CAP: usize = 64;

/// Pre-trims a pathologically long `Middle` run (see [`MAX_BREADCRUMB_SEGMENTS_HARD_CAP`]) down to
/// a bounded prefix and suffix before the width-based planner ever sees it, the same shape the old
/// count-based collapse used. `Middle` segments are always contiguous — the span between the root
/// (or the start of the trail) and the file segment — so this is a single splice, and it never
/// touches `Root`, `File`, or `Symbol` segments, which `plan_breadcrumb_layout`'s priority order
/// already keeps until last.
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

    // `File` always follows every `Middle` segment (see `classify_breadcrumb_segment_kinds`), so
    // this splice can only ever shift it left, never swallow it.
    file_segment_index -= (splice_end - splice_start) - 1;

    (segments, symbol_segments, kinds, file_segment_index)
}

/// The result of [`plan_breadcrumb_layout`]: which original segment indices survive, and which
/// contiguous runs of indices collapse into a single "⋯" each. `visible` and `ellipses` together
/// partition `0..segment_count` — every index is in exactly one of them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BreadcrumbLayoutPlan {
    pub(crate) visible: Vec<usize>,
    pub(crate) ellipses: Vec<Range<usize>>,
}

/// Sums the rendered width of a candidate layout: visible segments at their measured width, each
/// maximal run of dropped segments collapsed to a single `ellipsis_width`. The same model
/// `plan_breadcrumb_layout` searches over.
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

/// Groups a `dropped` bitmap into the [`BreadcrumbLayoutPlan`] shape: consecutive dropped indices
/// become one collapsed range each.
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

/// Decides which breadcrumb segments survive when `widths` (each already inclusive of that
/// segment's own separator/gap — see `render_breadcrumb_text`'s width measurement) don't fit in
/// `available_width`, and where the resulting "⋯" placeholders go. A pure function of measured
/// widths and segment kinds — no `Window` involved — so it's unit-testable independent of any
/// GPUI layout; `render_breadcrumb_text` is the only caller that has to do the (impure) measuring.
///
/// Drops segments one at a time, cheapest-to-lose first, stopping as soon as the remainder fits:
/// every [`Middle`](BreadcrumbSegmentKind::Middle) segment, then
/// [`Root`](BreadcrumbSegmentKind::Root), then [`File`](BreadcrumbSegmentKind::File), then each
/// [`Symbol`](BreadcrumbSegmentKind::Symbol) from outermost to innermost — exactly the priority
/// order the feature calls for. The very last segment is excluded from every one of those groups,
/// so it is never a drop candidate at all; that is what makes the degenerate case graceful — even
/// when nothing else fits, that one segment still renders instead of the bar going empty.
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

/// Whether the breadcrumb bar's leading path/file segment should offer any navigation
/// (directory-splitting into a dropdown, or the whole-buffer symbol-listing fallback) at all.
///
/// `false` for two cases the maintainer review on #60282 called out by name:
/// - `has_project_path: false` — an untitled/unsaved buffer, which has no location in a project
///   for a dropdown to browse from.
/// - `worktree_is_single_file: Some(true)` — a file opened outside any real worktree, which Zed
///   represents as a worktree scoped to that one file (see [`project::Worktree::is_single_file`]);
///   there's no directory tree to browse and no sibling to reach.
///
/// `worktree_is_single_file: None` (the worktree couldn't be resolved at all, e.g. removed
/// mid-session) is treated as navigable, preserving the prior fallback-to-symbols behavior for
/// that unrelated edge case rather than conflating "can't check" with "confirmed not navigable".
fn breadcrumb_path_is_navigable(
    has_project_path: bool,
    worktree_is_single_file: Option<bool>,
) -> bool {
    has_project_path && !worktree_is_single_file.unwrap_or(false)
}

/// One breadcrumb segment with everything [`BreadcrumbsRow`] needs to measure it and, if it
/// survives [`plan_breadcrumb_layout`], render it — resolved ahead of time by
/// `render_breadcrumb_text` so the element itself never has to reach back into `Editor`/
/// `Workspace`/`ItemHandle` state during layout.
struct PreparedBreadcrumbSegment {
    kind: BreadcrumbSegmentKind,
    label: HighlightedText,
    target: Option<BreadcrumbSegmentTarget>,
    /// Whether this is the dirty/unsaved file's own segment and should render through
    /// `apply_dirty_filename_style` instead of its plain label — precomputed because that check
    /// needs `active_item` and `workspace::TabBarSettings`, neither of which a `'static` GPUI
    /// element like `BreadcrumbsRow` can hold onto.
    dirty_filename_style: bool,
    /// The icon shown before this segment's name, mirroring the project panel's file and folder
    /// icons. This is what tells the file apart from the directories leading to it, the way
    /// IntelliJ's navigation bar does.
    icon: Option<SharedString>,
    /// Colour for the segment's own text. The path stays muted so the file it leads to reads as
    /// the subject rather than as one more directory, and the file carries its git status, the
    /// same mapping multi buffer headers use for file names.
    label_color: Color,
}

/// Per-segment "slot" width [`BreadcrumbsRow`] plans against: the segment's own label plus one
/// arrow and the gaps around it, all measured once per render in `request_layout` via the
/// window's text system. `shape_line` is cached by text/font (see `TextSystem::shape_line`), so
/// this is a handful of cache lookups per render — not a full reshape — and it only happens when
/// GPUI actually re-renders the breadcrumb bar (e.g. the cursor moves), never on every frame.
///
/// Deliberately uniform regardless of where a segment ends up in the final `with_separator`
/// sequence: a row of `n` rendered items has at most `n - 1` arrows, so summing "one arrow per
/// segment" overestimates the true row width by about one arrow's worth. That bias only ever
/// makes [`plan_breadcrumb_layout`] collapse slightly earlier than the bare minimum, never later —
/// it can't be the reason something overflows.
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

/// Sums `plan`'s rendered width the same way `plan_breadcrumb_layout` modeled it internally,
/// re-expressed from the plan's `visible`/`ellipses` shape instead of a `dropped` bitmap.
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

/// Renders the breadcrumb bar's segment trail, deciding — from the width GPUI actually offers it,
/// not a fixed segment count — how many segments to show before collapsing the rest into a single
/// "⋯" (see [`plan_breadcrumb_layout`]). A custom [`gpui::Element`] rather than a plain `h_flex`
/// because that decision can only be made once the surrounding flex layout has resolved how much
/// horizontal space this row gets, which — like `List`/`UniformList` in `gpui` — means requesting
/// a *measured* layout (`Window::request_measured_layout`) sized purely from text metrics, then,
/// once `prepaint` hands back the authoritative final `bounds`, building the real interactive
/// segment elements only for whichever ones the plan keeps and laying them out by hand
/// (`AnyElement::layout_as_root` / `prepaint_at`, the same pattern `UniformList` uses for its
/// visible rows) rather than through a normal taffy child tree.
struct BreadcrumbsRow {
    segments: Vec<PreparedBreadcrumbSegment>,
    editor: Option<WeakEntity<Editor>>,
}

/// Names the per-segment hover group, so the highlight can be painted on the segment's label alone
/// while the separator after it stays unpainted despite being inside the same click target.
const BREADCRUMB_SEGMENT_GROUP: &str = "breadcrumb-segment";

/// Horizontal padding around a segment's label, inside its hover highlight.
const BREADCRUMB_LABEL_PADDING: Pixels = px(4.);

/// Matches the project panel's own entry icons, so the two read as the same tree.
const BREADCRUMB_ICON_SIZE: IconSize = IconSize::Small;

/// The icon for one breadcrumb segment: only the segment standing for the open file gets one,
/// which is what sets the file apart from the directories leading to it. Directories get none —
/// IntelliJ's navigation bar shows no folder icons either, and a row of them reads as noise rather
/// than as information. Symbol segments get none because they name code, not an entry in the tree.
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
    /// The bar reads as part of the UI chrome rather than as code, so it uses the UI font — the
    /// same choice IntelliJ's navigation bar makes — rather than the buffer font the breadcrumb
    /// text is measured in elsewhere.
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
        // Only the label is painted on hover. The separator stays clickable — it belongs to the
        // segment on its left, which is the one a click drills into — but it isn't part of that
        // segment's name, so highlighting it reads as though it were.
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
                // Nudged down by a pixel: the chevron is centered in its own box, but breadcrumb
                // text is almost all lowercase, whose visual center sits below the line's
                // geometric one, so a geometrically centered chevron reads as raised beside it.
                div().relative().top(px(2.)).child(
                    Icon::new(IconName::ChevronRight)
                        .size(IconSize::XSmall)
                        .color(Color::Placeholder),
                ),
            )
            .into_any_element()
    }

    /// Hosts the hover group [`Self::with_separator`]'s label reacts to, wrapped around the whole
    /// segment so hovering anywhere in its click target — separator included — lights the label.
    fn wrap_segment(&self, element: gpui::AnyElement) -> gpui::AnyElement {
        div()
            .group(BREADCRUMB_SEGMENT_GROUP)
            .child(element)
            .into_any_element()
    }

    /// Builds the actual clickable element for segment `index`, matching the pre-width-aware
    /// code's per-segment rendering exactly, just moved here so it only ever runs for segments
    /// `plan_breadcrumb_layout` actually kept.
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
                    // Nudged for the same reason as the separator: centering against the line box
                    // reads high beside mostly-lowercase text.
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

    /// The "⋯" placeholder for a collapsed run. Deliberately inert (plain `Label`, no popover
    /// trigger): the whole point of width-aware collapse is that it fires far less often than the
    /// old count-based version did, and every segment it stands in for is still reachable by
    /// widening the window (or the segments to either side of it, which remain fully clickable) —
    /// making it list the hidden components itself would mean giving it its own popover machinery
    /// (a `PopoverMenu` plus a listing widget) for a rarely-hit case, which isn't worth the added
    /// layout-logic complexity here.
    fn render_ellipsis(&self, position: usize, last_position: usize, cx: &App) -> gpui::AnyElement {
        let content = Label::new("⋯").color(Color::Placeholder).into_any_element();
        self.with_separator(position, last_position, content, false, cx)
    }
}

/// [`gpui::Element::PrepaintState`] for [`BreadcrumbsRow`]: the already-laid-out child elements
/// (segments and "⋯" placeholders, in final left-to-right order), ready for `paint` to just walk
/// and paint — all the layout decisions happened in `prepaint`, which is the only place the row's
/// authoritative final width is known.
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
        // with the full trail would tell the layout this row can never be narrower than its text —
        // the parent would stop offering it less, and `plan_breadcrumb_layout` would never see a
        // width worth collapsing for. The row can always fall back to one segment plus an ellipsis,
        // and pins `min_size` to match rather than leaving it to that automatic minimum.
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

        // Every segment has now registered whatever popover handle it owns, which is exactly what
        // a pending re-anchor was waiting for.
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
    // `min_w_0` so the toolbar's width actually reaches `BreadcrumbsRow`: a flex item defaults to
    // a minimum size of its content, which would let the row report the width it wants rather than
    // the width it has, and `plan_breadcrumb_layout` would never collapse anything.
    let element = h_flex().flex_grow_1().min_w_0().text_ui(cx);

    let editor = active_item
        .downcast::<Editor>()
        .map(|editor| editor.downgrade());

    // Segment data aligned 1:1 with `segments` once the path-splitting below runs: the leading
    // path segment is split into one directory segment per path component plus a final file
    // segment (buffer id paired with `None` — no ancestor item, so its dropdown lists top-level
    // symbols instead), and each subsequent ancestor symbol segment gets the buffer id paired
    // with its own item. Empty unless we can resolve a live singleton-buffer editor, since
    // that's exactly the precondition `Editor::breadcrumbs_inner` uses to include symbols in
    // `segments`. The buffer id comes from the singleton directly rather than
    // `outline_symbols_at_cursor` so the path segment still gets a menu when the cursor sits
    // outside any symbol (an empty ancestor chain).
    let mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>> = Vec::new();
    // Which final segment is "the file", for the dirty-filename styling below. Stays 0 —
    // matching the pre-split behavior — whenever the path-splitting below doesn't run (a
    // multibuffer header, or a buffer with no project path, e.g. unsaved).
    let mut file_segment_index = 0usize;
    // Whether the path-splitting below inserted a leading root segment at index `0`
    // (`DrillDown` mode only — see `breadcrumb_path_segments`'s doc comment), so
    // `classify_breadcrumb_segment_kinds` below can tell that segment apart from an ordinary
    // `Middle` directory component.
    let mut has_root_segment = false;
    // The buffer whose outline the segment dropdowns will need, so hovering the bar can start
    // fetching it before any of them is opened.
    let mut outline_buffer_id = None;
    // The open file's own path, for the file segment's icon. Only that segment shows one; the
    // directory segments carry their own paths in their targets.
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

            // The real open file's path, independent of any breadcrumb navigation below — used
            // both as the fallback bar (when nothing is navigated) and, while navigated, as the
            // `active_path` that submenus still highlight their way towards, so browsing
            // elsewhere in the tree doesn't lose the trail back to the file actually open.
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
            // Set once a directory row has been chosen inside an open dropdown (see
            // `Editor::navigate_breadcrumb_to`); while set, the bar shows that directory's own
            // path instead of the open file's, with no symbol segments.
            let navigation = editor_ref.breadcrumb_navigation().cloned();
            let navigated = navigation
                .as_ref()
                .is_some_and(|navigation| navigation.navigated);
            let active_segment = navigation
                .as_ref()
                .map(|navigation| navigation.active_path.clone());

            // A buffer with no project path at all (never saved) has no directory tree to
            // browse and no location in a project for the leading segment to represent, so it
            // must stay plain text rather than degrading to a whole-buffer symbol dropdown.
            // Likewise a path that resolves into a worktree created just to hold one file opened
            // outside any real project (see `Worktree::is_single_file`) has no siblings and no
            // real root to split into — IntelliJ's navigation bar and VS Code's breadcrumbs both
            // leave such files unclickable rather than offering a dropdown with nothing useful
            // in it. A worktree that can't be resolved at all (removed mid-session) is treated
            // as navigable, preserving the prior fallback-to-symbols behavior for that edge case.
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

            // Splitting the path requires knowing which worktree to name its root and list its
            // top-level entries from; falls back to the single unsplit path segment
            // `render_breadcrumb_text`'s caller already built otherwise (e.g. an unsaved buffer).
            // The root segment is added unconditionally — even for a single-worktree project —
            // so sibling top-level directories are reachable the way IntelliJ's navigation bar
            // reaches them, starting at the project root rather than the file's own path.
            // `Editor::breadcrumbs_inner` separately bakes the root name into its own single
            // unsplit segment when more than one worktree is visible (via `resolve_file_path`'s
            // `include_root`); that doesn't double up here because this whole branch replaces
            // that segment's text wholesale via the `splice` below rather than reusing it.
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
                // Not navigable (see `is_navigable`'s doc comment): leave this segment with no
                // target at all so `render_breadcrumb_text` renders it as plain, unclickable
                // text instead of wrapping it in a popover trigger.
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

    // Precomputed here rather than inside `BreadcrumbsRow`: the dirty-filename check needs
    // `active_item` and `TabBarSettings`, and `BreadcrumbsRow` is a `'static` GPUI element that
    // can't hold a borrow of the former. Whether this ends up actually applying to a rendered
    // segment is still decided per-frame, by `plan_breadcrumb_layout` — if the file segment gets
    // collapsed into a "⋯" under extreme width pressure, this flag on it simply never gets read.
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
            // A plain row rather than a `ButtonLike`: the whole bar is no longer clickable as one
            // unit (every segment carries its own dropdown), and `ButtonLike` renders `flex_none`,
            // which would stop the bar from ever being narrower than its content and so keep
            // `BreadcrumbsRow` from being told to collapse.
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

/// Byte offset at which the file name starts inside a path segment's label, or `None` when the
/// label isn't a path. Shared between painting the bold file name and measuring it, so the two
/// can't drift apart.
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
        // The whole point of `flatten_text_for_single_line_display` is that byte-offset
        // highlight ranges computed against `original` stay valid against its return value —
        // verify that directly rather than just trusting the debug-assert, by locating the same
        // substring by byte offset in both strings.
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
        // impl A {         // 0
        //     fn one() {}  // 1
        //     fn two() {}  // 1
        // }
        // impl B {         // 0
        //     fn three() {}// 1
        // }
        let depths = [0, 1, 1, 0, 1];
        assert_eq!(sibling_outline_indices(&depths, 1), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 2), vec![1, 2]);
        assert_eq!(sibling_outline_indices(&depths, 4), vec![4]);
        assert_eq!(sibling_outline_indices(&depths, 0), vec![0, 3]);
        assert_eq!(sibling_outline_indices(&depths, 3), vec![0, 3]);
    }

    #[test]
    fn test_sibling_outline_indices_uneven_depths() {
        // Tree-sitter outlines can jump straight from depth 0 to depth 2 (e.g. a struct
        // whose fields are one nesting level "deeper" than a typical impl body). The parent
        // of a depth-2 item here should be the nearest preceding shallower item (depth 0),
        // not a nonexistent depth-1 item.
        // struct Foo {  // 0
        //     bar: u32, // 2
        //     baz: u32, // 2
        // }
        // struct Qux;   // 0
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
        // impl A {         // 0
        //     fn one() {}  // 1
        //     fn two() {}  // 1
        // }
        // impl B {         // 0
        //     fn three() {}// 1
        // }
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
        // struct Foo {  // 0
        //     bar: u32, // 2
        //     baz: u32, // 2
        // }
        // struct Qux;   // 0
        //
        // The depth-2 fields are still direct children of the depth-0 struct, even though
        // there's no depth-1 item between them — parenthood follows the nearest preceding
        // shallower item, not `depth - 1`.
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

        // Stops at `repository` itself rather than descending into the file it alone contains —
        // the resolved directory's listing still has to show `Repositories.kt` as a row the user
        // clicks themselves, not open it on their behalf.
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
        // Divergent on purpose, and shorter than `segments`: models `symbol_segments` built from a
        // navigation whose worktree failed to resolve (see `render_breadcrumb_text`'s comment on
        // `symbol_segments`).
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

    /// Regression test for the `symbol_segments.splice()` panic described in the branch review:
    /// with the cursor deep in a symbol chain and a navigated worktree that fails to resolve,
    /// `symbol_segments` ends up far shorter than `segments` (see `render_breadcrumb_text`'s
    /// comment on `symbol_segments` for how the two are built independently). Every splice below —
    /// hard-capping, and later applying `plan_breadcrumb_layout`'s plan — computes its range purely
    /// from `segments.len()`; applying that same range to the short `symbol_segments` panics unless
    /// `align_symbol_segments` runs first.
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

        // Narrow enough that all 4 middle components must go (dropping only 3 of them leaves
        // root(60) + ⋯(20) + d(30) + file(80) + symbols(90+120) = 400, still too wide), but root,
        // file, and both symbols still fit once all 4 are gone: root(60) + ⋯(20) + file(80) +
        // symbols(90+120) = 370.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(380.));
        assert_eq!(plan.visible, vec![0, 5, 6, 7]);
        assert_eq!(plan.ellipses, vec![1..5]);

        // Narrow enough that root has to go too, but file and both symbols still fit: ⋯(20) +
        // file(80) + symbols(90+120) = 310.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(340.));
        assert_eq!(plan.visible, vec![5, 6, 7]);
        assert_eq!(plan.ellipses, vec![0..5]);

        // Narrower still: file goes as well, leaving just the symbol chain — "only symbols fit".
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(230.));
        assert_eq!(plan.visible, vec![6, 7]);
        assert_eq!(plan.ellipses, vec![0..6]);

        // Narrower yet: the outer symbol goes too, leaving only the innermost — the one segment
        // `plan_breadcrumb_layout` never drops.
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

    /// Regression test for the double-lease panic fixed on `Editor::navigate_breadcrumb_to`:
    /// choosing a directory row runs `BreadcrumbDirectoryBrowser::choose` inside a `cx.listener`
    /// on that browser entity (see `render_entry`'s `on_click`), i.e. the entity is leased for
    /// the duration of the call. `choose` reaches `navigate_breadcrumb_to`, which re-anchors the
    /// active segment's shared `PopoverMenuHandle` — and that handle's `menu` is the very browser
    /// entity being chosen from, since a row can only be chosen from the active segment's own
    /// dropdown. Calling `PopoverMenuHandle::hide`/`show` synchronously there updates that same
    /// entity a second time and panics (`entity_map.rs`: "cannot update ... while it is already
    /// being updated"). This wires up a real `PopoverMenu`/`PopoverMenuHandle` exactly as
    /// `render_breadcrumb_directory_segment` does — including calling `open_breadcrumb_navigation`
    /// from inside the same `.menu()` builder that creates the browser, not as a separate step —
    /// opens it through `handle.show` to get the *same* `BreadcrumbDirectoryBrowser` entity
    /// `handle`'s internal state holds, and then drives `choose` on that entity through
    /// `Entity::update_in` the same way `cx.listener` does. Verified by temporarily reverting
    /// `navigate_breadcrumb_to` to its pre-fix, synchronous `handle.hide`/`show`: this test then
    /// panics with exactly "cannot update editor::element::BreadcrumbDirectoryBrowser while it is
    /// already being updated", and passes again once the fix is restored.
    ///
    /// Not covered: real mouse-event dispatch through the popover's rendered rows (this calls
    /// `choose` directly rather than clicking a laid-out `ListItem`), and the visual outcome of
    /// the re-anchor (this only checks that it completes and lands in the expected state, not
    /// pixel positions).
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

        // A real `PopoverMenu`/`PopoverMenuHandle` only wires itself up during an actual layout
        // pass of a rendered view (see `PopoverMenu::request_layout`), which itself needs a
        // `current_view` on the window's render stack — so this has to be an honest `Render`
        // mounted as a window's root, not a bare element drawn via `VisualTestContext::draw`.
        //
        // The `Editor` is created here too, inside the harness's own window, rather than in a
        // window of its own: `Context::defer_in`'s `ensure_window` only *fills in* an entity's
        // window association if it doesn't already have one, so an `Editor` native to a different
        // window would keep re-anchoring's `on_next_frame` calls stuck on that other window,
        // where nothing in this test ever drains them.
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
                        // Mirrors `render_breadcrumb_directory_segment`'s `.menu()` builder
                        // exactly: `open_breadcrumb_navigation` marks this segment active as
                        // part of the *same* opening sequence that creates the browser below,
                        // not a separate step afterward. Calling it separately, after the
                        // popover was already open, would itself dismiss that already-open
                        // browser through `handle` (see `open_breadcrumb_navigation`'s own
                        // `hide` call) before `choose` ever ran — which would silently defeat
                        // the reproduction this test exists for, by leaving `handle` pointing at
                        // nothing by the time `navigate_breadcrumb_to` tries to reach it.
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

        // Lays out the harness once so `PopoverMenu::request_layout` wires `handle`'s state up to
        // the popover's own `Rc<RefCell<Option<Entity<...>>>>`, exactly like a real breadcrumb
        // bar render pass does before any dropdown ever opens.
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });

        // Opening the popover through `handle.show` (rather than constructing a
        // `BreadcrumbDirectoryBrowser` directly) is what makes `browser` below the exact same
        // entity `handle`'s internal state holds — the crux of reproducing the bug, since the
        // panic is specifically about `navigate_breadcrumb_to` reaching back into the entity
        // that's currently leased.
        cx.update(|window, cx| handle.show(window, cx));
        let browser = captured_browser.borrow().clone().expect("popover opened");
        assert!(handle.is_deployed());
        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation().is_some(),
                "opening the popover marked this segment active"
            );
        });

        // The actual reproduction: choosing a directory row while the browser entity is leased
        // by this very `update` call. `dir_a` has two children, so `descend_single_child_directories`
        // resolves it immediately and `choose` calls straight into `navigate_breadcrumb_to` —
        // still inside this closure's lease. Pre-fix, this panicked.
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

        // Completing the re-anchor (see `navigate_breadcrumb_to`) must not panic either: it calls
        // `handle.hide`, then `handle.show` once the bar has laid the new active segment out — and
        // by then `browser` is no longer leased, but a fresh browser now backs the handle. Standing
        // in for that layout here, since this test drives the handle directly rather than rendering
        // a real bar.
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

    #[gpui::test]
    /// Worktrees never scan gitignored directories proactively, so a dropdown that only reads the
    /// snapshot shows them as empty. Opening one has to trigger the same scan the project panel
    /// triggers when a directory is expanded — one level per dropdown, so reaching a file nested
    /// two levels inside `.gitignore`d territory takes two of them.
    /// The reviewer asked for the whole flow driven by `menu::` actions rather than by simulated
    /// keystrokes: move the selection, submit it, and end up somewhere new.
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
        // so choosing `a` descends straight to `a/b`, the directory that stops short of the file
        // (see `descend_single_child_directories`).
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

        // Default settings (`sort_mode: directories_first`, `sort_order: default`) match the
        // project panel's own default: the directory first, then files in case-insensitive
        // natural order.
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert_eq!(
            entries.iter().map(|e| e.name.as_ref()).collect::<Vec<_>>(),
            vec!["Apple", "banana.txt", "Cherry.txt"],
        );

        // Reusing `util::paths::compare_rel_paths_by` (see `BreadcrumbDirectoryListingSettings`)
        // means changing `project_panel.sort_mode`/`sort_order` changes our ordering exactly the
        // way it changes the panel's: files first, compared by raw Unicode codepoint — so the
        // uppercase `Cherry.txt` sorts before lowercase `banana.txt`, and the directory moves
        // last.
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

        // `hide_gitignore` defaults to `false`: the ignored entry is still listed, matching
        // `entry.is_ignored` on `worktree::Entry` so the caller can still color it, mirroring the
        // project panel's default of showing gitignored entries dimmed rather than hidden.
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        let ignored_entry = entries
            .iter()
            .find(|entry| entry.name.as_ref() == "ignored.txt")
            .expect("gitignored entry is shown, not hidden, by default");
        assert!(ignored_entry.is_ignored);

        // Setting `project_panel.hide_gitignore` — the same setting the panel itself reads —
        // removes it from the listing entirely, keeping the two views in agreement about what
        // exists (see `BreadcrumbDirectoryListingSettings`'s doc comment).
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

        // `hide_hidden` defaults to `false`: the dotfile is still listed, matching the project
        // panel's default of showing hidden entries.
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert!(
            entries.iter().any(|entry| entry.name.as_ref() == ".hidden"),
            "hidden entry is shown by default"
        );

        // Setting `project_panel.hide_hidden` — the same setting the panel itself reads — removes
        // it from the listing entirely, keeping the two views in agreement about what exists (see
        // `BreadcrumbDirectoryListingSettings`'s doc comment).
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
