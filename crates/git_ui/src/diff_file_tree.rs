use crate::{diff_multibuffer::PathTarget, git_status_icon};
use collections::{HashMap, HashSet};
use file_icons::FileIcons;
use git::{repository::RepoPath, status::FileStatus};
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, Render, ScrollStrategy, SharedString,
    Subscription, UniformListScrollHandle, uniform_list,
};
use project::git_store::diff_buffer_list::{BranchDiffEvent, DiffBufferList};
use std::collections::BTreeMap;
use theme::ActiveTheme;
use ui::{CommonAnimationExt as _, prelude::*};

const TREE_INDENT: f32 = 12.0;

/// A sidebar tree showing only the files changed in a diff, in the style of
/// PhpStorm's diff viewer: directories with a single child are compacted into
/// one row, and selecting a file navigates the adjacent diff editor to it.
pub struct DiffFileTree {
    branch_diff: Entity<DiffBufferList>,
    entries: Vec<TreeEntry>,
    all_files: Vec<(RepoPath, FileStatus)>,
    collapsed_dirs: HashSet<RepoPath>,
    selected_index: Option<usize>,
    open_file: Option<(RepoPath, FileStatus)>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    _subscription: Subscription,
}

#[derive(Debug, Clone)]
enum TreeEntry {
    Directory {
        path: RepoPath,
        name: SharedString,
        depth: usize,
        expanded: bool,
    },
    File {
        repo_path: RepoPath,
        status: FileStatus,
        depth: usize,
    },
}

pub enum DiffFileTreeEvent {
    OpenEntry {
        repo_path: RepoPath,
        status: FileStatus,
        target: PathTarget,
    },
}

impl EventEmitter<DiffFileTreeEvent> for DiffFileTree {}

#[derive(Default)]
struct TreeNode {
    name: SharedString,
    path: Option<RepoPath>,
    children: BTreeMap<SharedString, TreeNode>,
    files: Vec<(RepoPath, FileStatus)>,
}

impl DiffFileTree {
    pub fn new(branch_diff: Entity<DiffBufferList>, cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe(&branch_diff, |this, _, event, cx| match event {
            BranchDiffEvent::FileListChanged | BranchDiffEvent::DiffBaseChanged => {
                this.rebuild_entries(cx);
            }
        });
        let mut this = Self {
            branch_diff,
            entries: Vec::new(),
            all_files: Vec::new(),
            collapsed_dirs: HashSet::default(),
            selected_index: None,
            open_file: None,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
            _subscription: subscription,
        };
        this.rebuild_entries(cx);
        this
    }

    fn rebuild_entries(&mut self, cx: &mut Context<Self>) {
        let mut root = TreeNode::default();
        let mut statuses_by_file = HashMap::default();
        if let Some(statuses) = self.branch_diff.read(cx).statuses_by_path() {
            for entry in statuses.iter() {
                statuses_by_file.insert(entry.repo_path.clone(), entry.status);
                let components: Vec<&str> = entry.repo_path.components().collect();
                let Some((_file_name, parents)) = components.split_last() else {
                    continue;
                };
                let mut current = &mut root;
                let mut current_path = String::new();
                let mut inserted = true;
                for component in parents {
                    if !current_path.is_empty() {
                        current_path.push('/');
                    }
                    current_path.push_str(component);
                    let Ok(dir_path) = RepoPath::new(&current_path) else {
                        inserted = false;
                        break;
                    };
                    let component = SharedString::from(component.to_string());
                    current = current
                        .children
                        .entry(component.clone())
                        .or_insert_with(|| TreeNode {
                            name: component,
                            path: Some(dir_path),
                            ..Default::default()
                        });
                }
                if inserted {
                    current.files.push((entry.repo_path.clone(), entry.status));
                }
            }
        }

        let mut entries = Vec::new();
        Self::flatten(&root, 0, &self.collapsed_dirs, &mut entries);
        self.entries = entries;
        let mut all_files = Vec::new();
        Self::collect_files(&root, &mut all_files);
        self.all_files = all_files;

        // Keep the open file across refreshes (its status may have changed);
        // if it is no longer part of the diff, fall back to the first file so
        // the adjacent editor never lingers on a file that left the change set.
        let open_file = self
            .open_file
            .take()
            .and_then(|(path, _)| Some((path.clone(), *statuses_by_file.get(&path)?)));
        match open_file {
            Some((path, status)) => {
                self.selected_index = self.index_of_file(&path);
                self.open_file = Some((path, status));
            }
            None => {
                let first_file =
                    self.entries
                        .iter()
                        .enumerate()
                        .find_map(|(index, entry)| match entry {
                            TreeEntry::File {
                                repo_path, status, ..
                            } => Some((index, repo_path.clone(), *status)),
                            TreeEntry::Directory { .. } => None,
                        });
                if let Some((index, repo_path, status)) = first_file {
                    self.selected_index = Some(index);
                    self.open_file = Some((repo_path.clone(), status));
                    cx.emit(DiffFileTreeEvent::OpenEntry {
                        repo_path,
                        status,
                        target: PathTarget::Start,
                    });
                } else {
                    self.selected_index = None;
                }
            }
        }
        cx.notify();
    }

    fn flatten(
        node: &TreeNode,
        depth: usize,
        collapsed_dirs: &HashSet<RepoPath>,
        out: &mut Vec<TreeEntry>,
    ) {
        for child in node.children.values() {
            let (terminal, name) = Self::compact_directory_chain(child);
            let Some(path) = terminal.path.clone().or_else(|| child.path.clone()) else {
                continue;
            };
            let expanded = !collapsed_dirs.contains(&path);
            out.push(TreeEntry::Directory {
                path,
                name,
                depth,
                expanded,
            });
            if expanded {
                Self::flatten(terminal, depth + 1, collapsed_dirs, out);
            }
        }
        for (repo_path, status) in &node.files {
            out.push(TreeEntry::File {
                repo_path: repo_path.clone(),
                status: *status,
                depth,
            });
        }
    }

    /// Collects every file in display order, ignoring collapse state, so that
    /// next/previous-file navigation can reach files inside collapsed folders.
    fn collect_files(node: &TreeNode, out: &mut Vec<(RepoPath, FileStatus)>) {
        for child in node.children.values() {
            Self::collect_files(child, out);
        }
        for file in &node.files {
            out.push(file.clone());
        }
    }

    /// Collapses chains of directories that contain a single subdirectory and
    /// no files into a single `a/b/c` row, as PhpStorm and the git panel do.
    fn compact_directory_chain(mut node: &TreeNode) -> (&TreeNode, SharedString) {
        let mut parts = vec![node.name.clone()];
        while node.files.is_empty() && node.children.len() == 1 {
            let Some(child) = node.children.values().next() else {
                break;
            };
            if child.path.is_none() {
                break;
            }
            parts.push(child.name.clone());
            node = child;
        }
        (node, SharedString::from(parts.join("/")))
    }

    /// The file currently shown in the adjacent diff editor. Unlike
    /// `selected_index` (the keyboard cursor, which can rest on a directory),
    /// this survives collapsing the directory that contains it.
    pub fn open_file(&self) -> Option<&(RepoPath, FileStatus)> {
        self.open_file.as_ref()
    }

    fn index_of_file(&self, path: &RepoPath) -> Option<usize> {
        self.entries.iter().position(
            |entry| matches!(entry, TreeEntry::File { repo_path, .. } if repo_path == path),
        )
    }

    /// Highlights `path` in the tree, expanding collapsed ancestors so the
    /// entry is visible. Used to follow the diff editor's active file.
    pub fn set_active_path(&mut self, path: Option<RepoPath>, cx: &mut Context<Self>) {
        let Some(path) = path else {
            return;
        };
        if self
            .open_file
            .as_ref()
            .is_some_and(|(open_path, _)| open_path == &path)
        {
            return;
        }
        let Some(status) = self.branch_diff.read(cx).status_for_path(&path, cx) else {
            return;
        };
        self.open_file = Some((path.clone(), status));
        let collapsed_ancestors = self
            .collapsed_dirs
            .iter()
            .any(|directory| path.starts_with(directory));
        if collapsed_ancestors {
            self.collapsed_dirs
                .retain(|directory| !path.starts_with(directory));
            self.rebuild_entries(cx);
        }
        self.selected_index = self.index_of_file(&path);
        if let Some(index) = self.selected_index {
            self.scroll_handle
                .scroll_to_item(index, ScrollStrategy::Center);
        }
        cx.notify();
    }

    fn toggle_expanded(&mut self, path: &RepoPath, cx: &mut Context<Self>) {
        if !self.collapsed_dirs.remove(path) {
            self.collapsed_dirs.insert(path.clone());
        }
        self.rebuild_entries(cx);
    }

    fn open_entry(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.get(index) else {
            return;
        };
        self.selected_index = Some(index);
        match entry {
            TreeEntry::Directory { path, .. } => {
                let path = path.clone();
                self.toggle_expanded(&path, cx);
            }
            TreeEntry::File {
                repo_path, status, ..
            } => {
                let repo_path = repo_path.clone();
                let status = *status;
                self.open_file = Some((repo_path.clone(), status));
                cx.emit(DiffFileTreeEvent::OpenEntry {
                    repo_path,
                    status,
                    target: PathTarget::Start,
                });
            }
        }
        cx.notify();
    }

    /// Opens the file after the currently open one, wrapping to the first.
    pub fn open_next_file(&mut self, cx: &mut Context<Self>) -> bool {
        self.open_file_at_offset(1, cx)
    }

    /// Opens the file before the currently open one, wrapping to the last, and
    /// asks the diff editor to land on its last hunk.
    pub fn open_previous_file(&mut self, cx: &mut Context<Self>) -> bool {
        self.open_file_at_offset(-1, cx)
    }

    fn open_file_at_offset(&mut self, offset: isize, cx: &mut Context<Self>) -> bool {
        let file_count = self.all_files.len() as isize;
        if file_count == 0 {
            return false;
        }
        let current_index = self.open_file.as_ref().and_then(|(open_path, _)| {
            self.all_files
                .iter()
                .position(|(path, _)| path == open_path)
        });
        let index = match current_index {
            Some(index) => (index as isize + offset).rem_euclid(file_count) as usize,
            None if offset >= 0 => 0,
            None => (file_count - 1) as usize,
        };
        let Some((path, status)) = self.all_files.get(index).cloned() else {
            return false;
        };
        self.open_file = Some((path.clone(), status));
        if self
            .collapsed_dirs
            .iter()
            .any(|directory| path.starts_with(directory))
        {
            self.collapsed_dirs
                .retain(|directory| !path.starts_with(directory));
            self.rebuild_entries(cx);
        }
        self.selected_index = self.index_of_file(&path);
        if let Some(index) = self.selected_index {
            self.scroll_handle
                .scroll_to_item(index, ScrollStrategy::Center);
        }
        let target = if offset >= 0 {
            PathTarget::Start
        } else {
            PathTarget::End
        };
        cx.emit(DiffFileTreeEvent::OpenEntry {
            repo_path: path,
            status,
            target,
        });
        cx.notify();
        true
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        if self.entries.is_empty() {
            return;
        }
        let index = self
            .selected_index
            .map_or(0, |index| (index + 1).min(self.entries.len() - 1));
        self.selected_index = Some(index);
        self.scroll_handle
            .scroll_to_item(index, ScrollStrategy::Center);
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.entries.is_empty() {
            return;
        }
        let index = self
            .selected_index
            .map_or(0, |index| index.saturating_sub(1));
        self.selected_index = Some(index);
        self.scroll_handle
            .scroll_to_item(index, ScrollStrategy::Center);
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_index {
            self.open_entry(index, cx);
        }
    }

    fn render_entry(&self, index: usize, cx: &Context<Self>) -> AnyElement {
        let Some(entry) = self.entries.get(index) else {
            return gpui::Empty.into_any_element();
        };
        let selected = self.selected_index == Some(index);
        let colors = cx.theme().colors();
        let base = h_flex()
            .h(rems(1.5))
            .w_full()
            .min_w_0()
            .px_1()
            .gap_1()
            .cursor_pointer()
            .when(selected, |this| this.bg(colors.element_selected))
            .when(!selected, |this| {
                this.hover(|style| style.bg(colors.ghost_element_hover))
            });

        match entry {
            TreeEntry::Directory {
                path,
                name,
                depth,
                expanded,
            } => {
                let folder_icon = FileIcons::get_folder_icon(*expanded, path.as_std_path(), cx)
                    .map(Icon::from_path)
                    .unwrap_or_else(|| {
                        Icon::new(if *expanded {
                            IconName::FolderOpen
                        } else {
                            IconName::Folder
                        })
                    });
                base.id(("diff-tree-dir", index))
                    .pl(px(*depth as f32 * TREE_INDENT + 4.0))
                    .child(
                        Icon::new(if *expanded {
                            IconName::ChevronDown
                        } else {
                            IconName::ChevronRight
                        })
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                    )
                    .child(folder_icon.size(IconSize::Small).color(Color::Muted))
                    .child(
                        Label::new(name.clone())
                            .color(Color::Muted)
                            .single_line()
                            .truncate(),
                    )
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.open_entry(index, cx);
                    }))
                    .into_any_element()
            }
            TreeEntry::File {
                repo_path,
                status,
                depth,
            } => {
                let file_name: SharedString = repo_path
                    .file_name()
                    .map(|name| name.to_owned())
                    .unwrap_or_else(|| repo_path.as_unix_str().to_owned())
                    .into();
                let file_icon = FileIcons::get_icon(repo_path.as_std_path(), cx)
                    .map(Icon::from_path)
                    .unwrap_or_else(|| Icon::new(IconName::File));
                let label_color = if status.is_conflicted() {
                    Color::VersionControlConflict
                } else if status.is_created() {
                    Color::VersionControlAdded
                } else if status.is_deleted() {
                    Color::Disabled
                } else if status.is_modified() {
                    Color::VersionControlModified
                } else {
                    Color::Default
                };
                let status = *status;
                base.id(("diff-tree-file", index))
                    .pl(px(*depth as f32 * TREE_INDENT + 4.0))
                    .child(file_icon.size(IconSize::Small).color(Color::Muted))
                    .child(
                        Label::new(file_name)
                            .color(label_color)
                            .when(status.is_deleted(), Label::strikethrough)
                            .single_line()
                            .truncate(),
                    )
                    .child(div().flex_1())
                    .child(git_status_icon(status))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.focus_handle.focus(window, cx);
                        this.open_entry(index, cx);
                    }))
                    .into_any_element()
            }
        }
    }
}

impl Focusable for DiffFileTree {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DiffFileTree {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entry_count = self.entries.len();
        let is_loading = self.branch_diff.read(cx).is_tree_base_loading();

        v_flex()
            .key_context("DiffFileTree")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::confirm))
            .when(entry_count == 0, |this| {
                this.items_center().justify_center().child(if is_loading {
                    Icon::new(IconName::LoadCircle)
                        .color(Color::Muted)
                        .with_rotate_animation(3)
                        .into_any_element()
                } else {
                    Label::new("No changed files")
                        .color(Color::Muted)
                        .into_any_element()
                })
            })
            .when(entry_count > 0, |this| {
                this.child(
                    uniform_list(
                        "diff-file-tree",
                        entry_count,
                        cx.processor(|this, range: std::ops::Range<usize>, _window, cx| {
                            range.map(|index| this.render_entry(index, cx)).collect()
                        }),
                    )
                    .size_full()
                    .track_scroll(&self.scroll_handle),
                )
            })
    }
}
