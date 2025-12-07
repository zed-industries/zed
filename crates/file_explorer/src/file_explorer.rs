#[cfg(test)]
mod file_explorer_tests;

use file_icons::FileIcons;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    KeyContext, ParentElement, Render, Styled, Task, WeakEntity, Window, actions, px,
};
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectPath, WorktreeId};
use search::ToggleIncludeIgnored;
use settings::Settings;
use std::{path::Path, sync::Arc};
use ui::{
    Button, ContextMenu, HighlightedLabel, Icon, IconButton, IconName, IconSize, Indicator,
    KeyBinding, Label, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, TintColor,
    Tooltip, prelude::*,
};
use util::{ResultExt, paths::PathStyle, rel_path::RelPath};
use workspace::{
    ModalView, SplitDirection, Workspace, item::PreviewTabsSettings, notifications::NotifyResultExt,
    pane,
};
use worktree::Entry;

actions!(
    file_explorer,
    [
        /// Opens/closes the file explorer modal.
        Toggle,
        /// Navigates to the parent directory.
        NavigateToParent,
        /// Toggles the filter options menu.
        ToggleFilterMenu,
        /// Toggles the split direction menu.
        ToggleSplitMenu,
    ]
);

impl ModalView for FileExplorer {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        let submenu_focused = self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .filter_popover_menu_handle
                .is_focused(window, cx)
                || picker
                    .delegate
                    .split_popover_menu_handle
                    .is_focused(window, cx)
        });
        workspace::DismissDecision::Dismiss(!submenu_focused)
    }
}

pub struct FileExplorer {
    picker: Entity<Picker<FileExplorerDelegate>>,
    picker_focus_handle: FocusHandle,
}

pub fn init(cx: &mut App) {
    cx.observe_new(FileExplorer::register).detach();
}

impl FileExplorer {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            if workspace.active_modal::<Self>(cx).is_some() {
                return;
            }
            Self::open(workspace, window, cx);
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        worktree_id: Option<WorktreeId>,
        current_path: Arc<RelPath>,
        initial_selected_path: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let file_explorer = cx.entity().downgrade();

        let delegate = FileExplorerDelegate::new(
            file_explorer,
            workspace,
            project,
            worktree_id,
            current_path,
            initial_selected_path,
            cx,
        );

        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, cx| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
            picker.delegate.load_entries(cx);
        });

        Self {
            picker,
            picker_focus_handle,
        }
    }

    fn open(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let project = workspace.project().clone();

        let active_item = workspace.active_item(cx);

        // Try to use the active item's worktree if it's visible
        let from_active_item = active_item.and_then(|item| {
            let project_path = item.project_path(cx)?;
            // Verify the worktree is visible (not a temporary worktree for external files)
            let worktree_is_visible = project
                .read(cx)
                .visible_worktrees(cx)
                .any(|wt| wt.read(cx).id() == project_path.worktree_id);
            if !worktree_is_visible {
                return None;
            }
            let parent = project_path
                .path
                .parent()
                .map(|p| p.to_owned().into())
                .unwrap_or_else(|| RelPath::empty().to_owned().into());
            let file_name = project_path.path.file_name().map(|s| s.to_string());
            Some((Some(project_path.worktree_id), parent, file_name))
        });

        let (worktree_id, current_path, initial_selected_path) =
            if let Some(result) = from_active_item {
                result
            } else {
                // No active item with a valid worktree
                let mut visible_worktrees = project.read(cx).visible_worktrees(cx);
                let first_worktree = visible_worktrees.next();
                let has_multiple = visible_worktrees.next().is_some();

                if has_multiple {
                    // Multiple worktrees, show worktree selection
                    (None, RelPath::empty().to_owned().into(), None)
                } else if let Some(worktree) = first_worktree {
                    // Single worktree, navigate to its root
                    let worktree = worktree.read(cx);
                    (
                        Some(worktree.id()),
                        RelPath::empty().to_owned().into(),
                        None,
                    )
                } else {
                    // No worktrees at all
                    return;
                }
            };

        let weak_workspace = cx.entity().downgrade();

        workspace.toggle_modal(window, cx, |window, cx| {
            FileExplorer::new(
                weak_workspace,
                project,
                worktree_id,
                current_path,
                initial_selected_path,
                window,
                cx,
            )
        });
    }

    fn handle_navigate_to_parent(
        &mut self,
        _: &NavigateToParent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.navigate_to_parent(window, cx);
        });
    }

    fn handle_split_toggle_menu(
        &mut self,
        _: &ToggleSplitMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.split_popover_menu_handle.toggle(window, cx);
        });
    }

    fn handle_filter_toggle_menu(
        &mut self,
        _: &ToggleFilterMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .filter_popover_menu_handle
                .toggle(window, cx);
        });
    }

    fn handle_toggle_ignored(
        &mut self,
        _: &ToggleIncludeIgnored,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.include_ignored = match picker.delegate.include_ignored {
                Some(true) => None,
                Some(false) => Some(true),
                None => Some(true),
            };
            picker.delegate.load_entries(cx);
            picker.refresh(window, cx);
        });
    }

    fn go_to_file_split_left(
        &mut self,
        _: &pane::SplitLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Left, window, cx)
    }

    fn go_to_file_split_right(
        &mut self,
        _: &pane::SplitRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Right, window, cx)
    }

    fn go_to_file_split_up(
        &mut self,
        _: &pane::SplitUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Up, window, cx)
    }

    fn go_to_file_split_down(
        &mut self,
        _: &pane::SplitDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Down, window, cx)
    }

    fn go_to_file_split_inner(
        &mut self,
        split_direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let result = self.picker.update(cx, |picker, cx| {
            let delegate = &picker.delegate;
            let Some(string_match) = delegate.matches.get(delegate.selected_index) else {
                return None;
            };
            let entry = delegate.all_entries[string_match.candidate_id].clone();

            let allow_preview =
                PreviewTabsSettings::get_global(cx).enable_preview_from_file_explorer;

            match entry {
                FileExplorerEntry::ParentDirectory
                | FileExplorerEntry::AllWorktrees
                | FileExplorerEntry::Worktree(..) => None,
                FileExplorerEntry::Entry(e) => {
                    if e.is_dir() {
                        None
                    } else {
                        let worktree_id = delegate.worktree_id?;
                        Some((
                            ProjectPath {
                                worktree_id,
                                path: e.path,
                            },
                            delegate.workspace.upgrade(),
                            allow_preview,
                        ))
                    }
                }
            }
        });

        let Some((project_path, Some(workspace), allow_preview)) = result else {
            return;
        };

        let open_task = workspace.update(cx, move |workspace, cx| {
            workspace.split_path_preview(
                project_path,
                allow_preview,
                Some(split_direction),
                window,
                cx,
            )
        });
        open_task.detach_and_log_err(cx);
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for FileExplorer {}

impl Focusable for FileExplorer {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.picker_focus_handle.clone()
    }
}

impl Render for FileExplorer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let key_context = self.picker.read(cx).delegate.key_context(window, cx);

        v_flex()
            .key_context(key_context)
            .w(rems(34.))
            .on_action(cx.listener(Self::handle_navigate_to_parent))
            .on_action(cx.listener(Self::handle_filter_toggle_menu))
            .on_action(cx.listener(Self::handle_toggle_ignored))
            .on_action(cx.listener(Self::handle_split_toggle_menu))
            .on_action(cx.listener(Self::go_to_file_split_left))
            .on_action(cx.listener(Self::go_to_file_split_right))
            .on_action(cx.listener(Self::go_to_file_split_up))
            .on_action(cx.listener(Self::go_to_file_split_down))
            .child(self.picker.clone())
    }
}

#[derive(Debug, Clone)]
enum FileExplorerEntry {
    ParentDirectory,
    AllWorktrees,
    Worktree(WorktreeId, Arc<RelPath>),
    Entry(Entry),
}

impl FileExplorerEntry {
    fn display_name(&self) -> String {
        match self {
            FileExplorerEntry::ParentDirectory => "..".to_string(),
            FileExplorerEntry::AllWorktrees => "All Worktrees".to_string(),
            FileExplorerEntry::Worktree(_, name) => name.as_ref().as_unix_str().to_string(),
            FileExplorerEntry::Entry(e) => e
                .path
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| ".".to_string()),
        }
    }
}

pub struct FileExplorerDelegate {
    file_explorer: WeakEntity<FileExplorer>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    worktree_id: Option<WorktreeId>,
    current_path: Arc<RelPath>,
    all_entries: Vec<FileExplorerEntry>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    include_ignored: Option<bool>,
    filter_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    split_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    /// The path of the file to initially select (if any)
    initial_selected_path: Option<String>,
}

impl FileExplorerDelegate {
    fn new(
        file_explorer: WeakEntity<FileExplorer>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        worktree_id: Option<WorktreeId>,
        current_path: Arc<RelPath>,
        initial_selected_path: Option<String>,
        cx: &mut Context<FileExplorer>,
    ) -> Self {
        Self {
            file_explorer,
            workspace,
            project,
            worktree_id,
            current_path,
            all_entries: Vec::new(),
            matches: Vec::new(),
            selected_index: 0,
            include_ignored: None,
            filter_popover_menu_handle: PopoverMenuHandle::default(),
            split_popover_menu_handle: PopoverMenuHandle::default(),
            focus_handle: cx.focus_handle(),
            initial_selected_path,
        }
    }

    fn load_entries(&mut self, cx: &App) {
        self.all_entries.clear();

        let project = self.project.read(cx);

        // If no worktree_id, show worktree selection
        let Some(worktree_id) = self.worktree_id else {
            let mut worktrees: Vec<_> = project
                .visible_worktrees(cx)
                .map(|wt| {
                    let wt = wt.read(cx);
                    FileExplorerEntry::Worktree(wt.id(), wt.root_name().into())
                })
                .collect();
            worktrees.sort_by_key(|a| a.display_name());
            self.all_entries = worktrees;
            self.matches = self.create_matches_from_entries();
            self.selected_index = 0;
            return;
        };

        let Some(worktree) = project.worktree_for_id(worktree_id, cx) else {
            return;
        };
        let worktree = worktree.read(cx);

        // Show parent directory if we're in a subdirectory
        // Or show "All Worktrees" if at root with multiple worktrees
        let at_worktree_root = self.current_path.as_ref().as_unix_str().is_empty();
        let has_multiple_worktrees = project.visible_worktrees(cx).count() > 1;

        if !at_worktree_root {
            self.all_entries.push(FileExplorerEntry::ParentDirectory);
        } else if has_multiple_worktrees {
            self.all_entries.push(FileExplorerEntry::AllWorktrees);
        }

        let mut dirs: Vec<Entry> = Vec::new();
        let mut files: Vec<Entry> = Vec::new();

        let include_ignored = self
            .include_ignored
            .unwrap_or_else(|| worktree.root_entry().is_some_and(|entry| entry.is_ignored));

        for entry in worktree.child_entries(&self.current_path) {
            if !include_ignored && entry.is_ignored {
                continue;
            }

            if entry.is_dir() {
                dirs.push(entry.clone());
            } else {
                files.push(entry.clone());
            }
        }

        dirs.sort_by(|a, b| a.path.cmp(&b.path));
        files.sort_by(|a, b| a.path.cmp(&b.path));

        for dir in dirs {
            self.all_entries.push(FileExplorerEntry::Entry(dir));
        }
        for file in files {
            self.all_entries.push(FileExplorerEntry::Entry(file));
        }

        self.matches = self.create_matches_from_entries();

        self.selected_index = self
            .initial_selected_path
            .take()
            .and_then(|target_path| {
                self.matches.iter().position(|m| {
                    match &self.all_entries[m.candidate_id] {
                        FileExplorerEntry::ParentDirectory
                        | FileExplorerEntry::AllWorktrees
                        | FileExplorerEntry::Worktree(..) => false,
                        FileExplorerEntry::Entry(e) => {
                            e.path.file_name().map(|s| s.to_string()).as_deref()
                                == Some(&target_path)
                        }
                    }
                })
            })
            .unwrap_or(0);
    }

    fn create_matches_from_entries(&self) -> Vec<StringMatch> {
        self.all_entries
            .iter()
            .enumerate()
            .map(|(index, entry)| StringMatch {
                candidate_id: index,
                string: entry.display_name(),
                positions: Vec::new(),
                score: 0.0,
            })
            .collect()
    }

    fn navigate_to_parent(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(parent) = self.current_path.parent() {
            // Navigate up within the current worktree
            self.current_path = parent.to_owned().into();
            self.load_entries(cx);
            cx.notify();

            cx.defer_in(window, |picker, window, cx| {
                picker.set_query("", window, cx);
                picker.refresh_placeholder(window, cx);
                picker.refresh(window, cx);
            });
        } else if self.worktree_id.is_some() {
            // At worktree root, navigate to worktree selection if multiple worktrees
            let has_multiple_worktrees = self.project.read(cx).visible_worktrees(cx).count() > 1;
            if has_multiple_worktrees {
                self.worktree_id = None;
                self.current_path = RelPath::empty().to_owned().into();
                self.load_entries(cx);
                cx.notify();

                cx.defer_in(window, |picker, window, cx| {
                    picker.set_query("", window, cx);
                    picker.refresh_placeholder(window, cx);
                    picker.refresh(window, cx);
                });
            }
        }
    }

    fn navigate_to_entry(
        &mut self,
        entry: &FileExplorerEntry,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        match entry {
            FileExplorerEntry::Worktree(id, _) => {
                self.worktree_id = Some(*id);
                self.current_path = RelPath::empty().to_owned().into();
                self.load_entries(cx);

                cx.notify();
                cx.defer_in(window, |picker, window, cx| {
                    picker.set_query("", window, cx);
                    picker.refresh_placeholder(window, cx);
                    picker.refresh(window, cx);
                });
            }
            FileExplorerEntry::Entry(e) => {
                self.current_path = e.path.clone();
                self.load_entries(cx);

                cx.notify();
                cx.defer_in(window, |picker, window, cx| {
                    picker.set_query("", window, cx);
                    picker.refresh_placeholder(window, cx);
                    picker.refresh(window, cx);
                });
            }
            FileExplorerEntry::ParentDirectory | FileExplorerEntry::AllWorktrees => {}
        }
    }

    fn display_path(&self, cx: &App) -> String {
        let Some(worktree_id) = self.worktree_id else {
            return "Worktrees".to_string();
        };

        let worktree_name = self
            .project
            .read(cx)
            .worktree_for_id(worktree_id, cx)
            .map(|wt| {
                wt.read(cx)
                    .root_name()
                    .display(PathStyle::local())
                    .into_owned()
            })
            .unwrap_or_else(|| ".".to_string());

        let path_str = self.current_path.as_ref().as_unix_str();
        if path_str.is_empty() {
            worktree_name
        } else {
            format!("{}/{}", worktree_name, path_str)
        }
    }

    fn key_context(&self, window: &Window, cx: &App) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("FileExplorer");

        if self.filter_popover_menu_handle.is_focused(window, cx) {
            key_context.add("filter_menu_open");
        }

        if self.split_popover_menu_handle.is_focused(window, cx) {
            key_context.add("split_menu_open");
        }

        key_context
    }
}

impl PickerDelegate for FileExplorerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, cx: &mut App) -> Arc<str> {
        format!("Filter {}", self.display_path(cx)).into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        if self
            .matches
            .first()
            .is_some_and(|m| matches!(self.all_entries[m.candidate_id], FileExplorerEntry::ParentDirectory))
        {
            vec![0]
        } else {
            Vec::new()
        }
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.matches = self.create_matches_from_entries();
            if self.selected_index >= self.matches.len() {
                self.selected_index = self.matches.len().saturating_sub(1);
            }
            cx.notify();
            Task::ready(())
        } else {
            let candidates: Vec<StringMatchCandidate> = self
                .all_entries
                .iter()
                .enumerate()
                .map(|(id, entry)| StringMatchCandidate::new(id, &entry.display_name()))
                .collect();

            let executor = cx.background_executor().clone();
            cx.spawn_in(window, async move |picker, cx| {
                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    executor,
                )
                .await;

                picker
                    .update(cx, |picker, cx| {
                        picker.delegate.matches = matches;
                        if picker.delegate.selected_index >= picker.delegate.matches.len() {
                            picker.delegate.selected_index =
                                picker.delegate.matches.len().saturating_sub(1);
                        }
                        cx.notify();
                    })
                    .ok();
            })
        }
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(string_match) = self.matches.get(self.selected_index) else {
            return;
        };
        let entry = self.all_entries[string_match.candidate_id].clone();

        match entry {
            FileExplorerEntry::ParentDirectory | FileExplorerEntry::AllWorktrees => {
                self.navigate_to_parent(window, cx);
            }
            FileExplorerEntry::Worktree(..) => {
                self.navigate_to_entry(&entry, window, cx);
            }
            FileExplorerEntry::Entry(ref e) if e.is_dir() => {
                self.navigate_to_entry(&entry, window, cx);
            }
            FileExplorerEntry::Entry(e) => {
                let Some(worktree_id) = self.worktree_id else {
                    return;
                };
                let project_path = ProjectPath {
                    worktree_id,
                    path: e.path,
                };

                if let Some(workspace) = self.workspace.upgrade() {
                    let allow_preview =
                        PreviewTabsSettings::get_global(cx).enable_preview_from_file_explorer;

                    let open_task = workspace.update(cx, |workspace, cx| {
                        if secondary {
                            workspace.split_path_preview(
                                project_path,
                                allow_preview,
                                None,
                                window,
                                cx,
                            )
                        } else {
                            workspace.open_path_preview(
                                project_path,
                                None,
                                true,
                                allow_preview,
                                true,
                                window,
                                cx,
                            )
                        }
                    });

                    let file_explorer = self.file_explorer.clone();
                    cx.spawn_in(window, async move |_, mut cx| {
                        open_task.await.notify_async_err(&mut cx);
                        file_explorer.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                    })
                    .detach();
                }
            }
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.file_explorer
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
        let string_match = self.matches.get(ix)?;
        let entry = &self.all_entries[string_match.candidate_id];

        let (icon_name, file_icon) = match entry {
            FileExplorerEntry::ParentDirectory => (Some(IconName::Folder), None),
            FileExplorerEntry::AllWorktrees => (Some(IconName::FileTree), None),
            FileExplorerEntry::Worktree(..) => (Some(IconName::FileTree), None),
            FileExplorerEntry::Entry(e) => {
                if e.is_dir() {
                    (Some(IconName::Folder), None)
                } else {
                    let name = e.path.file_name().unwrap_or("");
                    let icon = FileIcons::get_icon(Path::new(name), cx)
                        .map(|path| Icon::from_path(path).color(Color::Muted));
                    (None, icon)
                }
            }
        };

        let is_dir = match entry {
            FileExplorerEntry::ParentDirectory => true,
            FileExplorerEntry::AllWorktrees => false,
            FileExplorerEntry::Worktree(..) => false,
            FileExplorerEntry::Entry(e) => e.is_dir(),
        };

        let start_icon = file_icon.or_else(|| icon_name.map(|n| Icon::new(n).color(Color::Muted)));

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .start_slot::<Icon>(start_icon)
                .inset(true)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .child(HighlightedLabel::new(
                            entry.display_name(),
                            string_match.positions.clone(),
                        ))
                        .when(is_dir, |this| this.child(Label::new("/"))),
                ),
        )
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();
        let include_ignored = self.include_ignored;

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .justify_between()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    PopoverMenu::new("filter-menu-popover")
                        .with_handle(self.filter_popover_menu_handle.clone())
                        .attach(gpui::Corner::BottomRight)
                        .anchor(gpui::Corner::BottomLeft)
                        .offset(gpui::Point {
                            x: px(1.0),
                            y: px(1.0),
                        })
                        .trigger_with_tooltip(
                            IconButton::new("filter-trigger", IconName::Sliders)
                                .icon_size(IconSize::Small)
                                .toggle_state(include_ignored.unwrap_or(false))
                                .when(include_ignored.is_some(), |this| {
                                    this.indicator(Indicator::dot().color(Color::Info))
                                }),
                            {
                                let focus_handle = focus_handle.clone();
                                move |_window, cx| {
                                    Tooltip::for_action_in(
                                        "Filter Options",
                                        &ToggleFilterMenu,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            },
                        )
                        .menu({
                            let focus_handle = focus_handle.clone();

                            move |window, cx| {
                                Some(ContextMenu::build(window, cx, {
                                    let focus_handle = focus_handle.clone();
                                    move |menu, _, _| {
                                        menu.context(focus_handle.clone())
                                            .header("Filter Options")
                                            .toggleable_entry(
                                                "Include Ignored Files",
                                                include_ignored.unwrap_or(false),
                                                ui::IconPosition::End,
                                                Some(ToggleIncludeIgnored.boxed_clone()),
                                                move |window, cx| {
                                                    window.focus(&focus_handle);
                                                    window.dispatch_action(
                                                        ToggleIncludeIgnored.boxed_clone(),
                                                        cx,
                                                    );
                                                },
                                            )
                                    }
                                }))
                            }
                        }),
                )
                .child(
                    h_flex()
                        .gap_0p5()
                        .child(
                            PopoverMenu::new("split-menu-popover")
                                .with_handle(self.split_popover_menu_handle.clone())
                                .attach(gpui::Corner::BottomRight)
                                .anchor(gpui::Corner::BottomLeft)
                                .offset(gpui::Point {
                                    x: px(1.0),
                                    y: px(1.0),
                                })
                                .trigger(
                                    ui::ButtonLike::new("split-trigger")
                                        .child(Label::new("Split…"))
                                        .selected_style(ui::ButtonStyle::Tinted(TintColor::Accent))
                                        .child(KeyBinding::for_action_in(
                                            &ToggleSplitMenu,
                                            &focus_handle,
                                            cx,
                                        )),
                                )
                                .menu({
                                    let focus_handle = focus_handle.clone();

                                    move |window, cx| {
                                        Some(ContextMenu::build(window, cx, {
                                            let focus_handle = focus_handle.clone();
                                            move |menu, _, _| {
                                                menu.context(focus_handle)
                                                    .action(
                                                        "Split Left",
                                                        pane::SplitLeft.boxed_clone(),
                                                    )
                                                    .action(
                                                        "Split Right",
                                                        pane::SplitRight.boxed_clone(),
                                                    )
                                                    .action("Split Up", pane::SplitUp.boxed_clone())
                                                    .action(
                                                        "Split Down",
                                                        pane::SplitDown.boxed_clone(),
                                                    )
                                            }
                                        }))
                                    }
                                }),
                        )
                        .child(
                            Button::new("open-selection", "Open")
                                .key_binding(KeyBinding::for_action_in(
                                    &menu::Confirm,
                                    &focus_handle,
                                    cx,
                                ))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                                }),
                        ),
                )
                .into_any(),
        )
    }
}
