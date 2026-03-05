use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
    sync::Arc,
};

use file_icons::FileIcons;
use gpui::{AnyElement, App, Context, ElementId, IntoElement, WeakEntity, Window};
use project::{Entry, Project, ProjectPath, Worktree, WorktreeId};
use ui::{
    ButtonLike, ButtonStyle, Color, ContextMenu, PopoverMenu, PopoverMenuHandle, prelude::*,
};
use util::rel_path::RelPath;
use workspace::{Workspace, notifications::NotifyTaskExt as _};

struct FilePathComponent {
    name: SharedString,
    /// The directory whose contents are shown when this segment is clicked.
    ///
    /// For segment `i` in the path, this is the path prefix of the first `i`
    /// components — i.e., the parent directory of the entry that this segment
    /// represents.
    parent_dir: Arc<RelPath>,
}

#[derive(Clone)]
struct InlineMenuRow {
    name: SharedString,
    path: Arc<RelPath>,
    depth: usize,
    is_directory: bool,
    is_expanded: bool,
    icon_path: Option<SharedString>,
}

/// A horizontal row of clickable path segments for the editor breadcrumb toolbar.
///
/// Each segment opens a dropdown showing the contents of its parent directory,
/// letting the user navigate the project file hierarchy incrementally without
/// opening the project panel.
#[derive(IntoElement)]
pub struct FilePathNav {
    worktree_id: WorktreeId,
    components: Vec<FilePathComponent>,
    project: WeakEntity<Project>,
    workspace: Option<WeakEntity<Workspace>>,
}

impl FilePathNav {
    pub fn new(
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        show_worktree_root: bool,
        root_name: Option<SharedString>,
        project: WeakEntity<Project>,
        workspace: Option<WeakEntity<Workspace>>,
    ) -> Self {
        let component_names: Vec<String> =
            path.components().map(|component| component.to_owned()).collect();

        // `ancestors()` produces the path itself, then its parent, grandparent, …, down to "".
        // Reversed, this gives ["", "a", "a/b", …, "a/b/c/file"].
        // The parent_dir for component[i] is the i-th element of this reversed list.
        let ancestors_reversed: Vec<Arc<RelPath>> = {
            let mut values: Vec<Arc<RelPath>> = path.ancestors().map(Arc::from).collect();
            values.reverse();
            values
        };

        let mut components: Vec<FilePathComponent> = component_names
            .into_iter()
            .zip(ancestors_reversed)
            .map(|(name, parent_dir)| FilePathComponent {
                name: SharedString::from(name),
                parent_dir,
            })
            .collect();

        if show_worktree_root
            && let Some(root_name) = root_name
        {
            components.insert(
                0,
                FilePathComponent {
                    name: root_name,
                    parent_dir: Arc::from(RelPath::empty()),
                },
            );
        }

        Self {
            worktree_id,
            components,
            project,
            workspace,
        }
    }

    #[cfg(test)]
    pub(crate) fn component_names(&self) -> Vec<&str> {
        self.components.iter().map(|c| c.name.as_ref()).collect()
    }
}

pub(crate) fn open_breadcrumb_file(
    project_path: ProjectPath,
    workspace: &Option<WeakEntity<Workspace>>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.as_ref() else {
        log::error!(
            "Breadcrumb file navigation failed: workspace missing for path {:?}",
            project_path.path
        );
        return;
    };

    if let Err(error) = workspace.update(cx, |workspace, cx| {
        let workspace_handle = workspace.weak_handle();
        workspace
            .open_path(project_path.clone(), None, true, window, cx)
            .detach_and_notify_err(workspace_handle, window, cx);
    }) {
        log::error!(
            "Breadcrumb file navigation failed to update workspace for path {:?}: {error:#}",
            project_path.path
        );
    }
}

fn collect_visible_rows(
    snapshot: &Worktree,
    directory_path: &RelPath,
    depth: usize,
    expanded_dirs: &HashSet<Arc<RelPath>>,
    rows: &mut Vec<InlineMenuRow>,
    cx: &App,
) {
    let mut entries: Vec<Entry> = snapshot.child_entries(directory_path).cloned().collect();
    entries.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.path.cmp(&b.path),
    });

    for entry in entries {
        let Some(name) = entry.path.file_name().map(ToString::to_string) else {
            continue;
        };

        if entry.is_dir() {
            let path = entry.path;
            let is_expanded = expanded_dirs.contains(&path);
            rows.push(InlineMenuRow {
                name: SharedString::from(name),
                icon_path: FileIcons::get_folder_icon(is_expanded, path.as_std_path(), cx),
                path: path.clone(),
                depth,
                is_directory: true,
                is_expanded,
            });
            if is_expanded {
                collect_visible_rows(snapshot, &path, depth + 1, expanded_dirs, rows, cx);
            }
        } else {
            let icon_path = FileIcons::get_icon(entry.path.as_std_path(), cx);
            rows.push(InlineMenuRow {
                name: SharedString::from(name),
                path: entry.path,
                depth,
                is_directory: false,
                is_expanded: false,
                icon_path,
            });
        }
    }
}

fn build_inline_directory_menu(
    menu: ContextMenu,
    parent_dir: &RelPath,
    worktree_id: WorktreeId,
    project: &WeakEntity<Project>,
    workspace: &Option<WeakEntity<Workspace>>,
    expanded_directories: &Rc<RefCell<HashSet<Arc<RelPath>>>>,
    visible_rows: &Rc<RefCell<Vec<InlineMenuRow>>>,
    segment_index: usize,
    segment_handles: &Arc<Vec<PopoverMenuHandle<ContextMenu>>>,
    _window: &mut Window,
    cx: &mut Context<ContextMenu>,
) -> ContextMenu {
    let Some(project_entity) = project.upgrade() else {
        log::error!(
            "Breadcrumb directory menu build failed: project no longer available for worktree {}",
            worktree_id.to_proto()
        );
        return menu;
    };
    let Some(worktree) = project_entity.read(cx).worktree_for_id(worktree_id, cx) else {
        log::error!(
            "Breadcrumb directory menu build failed: missing worktree {}",
            worktree_id.to_proto()
        );
        return menu;
    };

    let expanded_snapshot = expanded_directories.borrow().clone();
    let mut rows: Vec<InlineMenuRow> = Vec::new();
    let snapshot = worktree.read(cx);
    collect_visible_rows(snapshot, parent_dir, 0, &expanded_snapshot, &mut rows, cx);

    *visible_rows.borrow_mut() = rows.clone();

    let expanded_directories_for_child = expanded_directories.clone();
    let visible_rows_for_child = visible_rows.clone();

    let expanded_directories_for_parent = expanded_directories.clone();
    let visible_rows_for_parent = visible_rows.clone();

    let segment_handles_for_next = segment_handles.clone();
    let segment_handles_for_previous = segment_handles.clone();

    let mut menu = menu
        .keep_open_on_confirm(true)
        .key_context("menu FilePathNavMenu")
        .select_child_override(move |menu, window, cx| {
            let Some(selected_index) = menu.selected_index() else {
                return false;
            };
            let Some(selected_row) = visible_rows_for_child.borrow().get(selected_index).cloned()
            else {
                return false;
            };
            if !selected_row.is_directory || selected_row.is_expanded {
                return false;
            }

            expanded_directories_for_child
                .borrow_mut()
                .insert(selected_row.path);
            menu.rebuild(window, cx);
            true
        })
        .select_parent_override(move |menu, window, cx| {
            let Some(selected_index) = menu.selected_index() else {
                return false;
            };
            let Some(selected_row) = visible_rows_for_parent.borrow().get(selected_index).cloned()
            else {
                return false;
            };

            if selected_row.is_directory && selected_row.is_expanded {
                expanded_directories_for_parent
                    .borrow_mut()
                    .remove(&selected_row.path);
                menu.rebuild(window, cx);
                return true;
            }

            let Some(parent_directory) = selected_row.path.parent().map(Arc::from) else {
                return false;
            };
            if expanded_directories_for_parent
                .borrow_mut()
                .remove(&parent_directory)
            {
                menu.rebuild(window, cx);
                return true;
            }

            false
        })
        .select_next_target_handler(move |_menu, window, cx| {
            let segment_count = segment_handles_for_next.len();
            if segment_count <= 1 {
                return false;
            }

            let next_index = (segment_index + 1) % segment_count;
            let handles = segment_handles_for_next.clone();
            window.defer(cx, move |window, cx| {
                handles[segment_index].hide(cx);
                handles[next_index].show(window, cx);
            });
            true
        })
        .select_previous_target_handler(move |_menu, window, cx| {
            let segment_count = segment_handles_for_previous.len();
            if segment_count <= 1 {
                return false;
            }

            let previous_index = if segment_index == 0 {
                segment_count - 1
            } else {
                segment_index - 1
            };
            let handles = segment_handles_for_previous.clone();
            window.defer(cx, move |window, cx| {
                handles[segment_index].hide(cx);
                handles[previous_index].show(window, cx);
            });
            true
        });

    for row in rows {
        let indentation = px((row.depth as f32) * 12.0);

        if row.is_directory {
            let directory_path = row.path;
            let label = row.name.clone();
            let icon_path = row.icon_path.clone();
            let expanded_directories = expanded_directories.clone();
            let is_expanded = row.is_expanded;

            menu = menu.custom_entry(
                move |_window, _cx| {
                    h_flex()
                        .w_full()
                        .pl(indentation)
                        .gap_1p5()
                        .child(
                            Icon::new(if is_expanded {
                                IconName::ChevronDown
                            } else {
                                IconName::ChevronRight
                            })
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                        )
                        .child(
                            icon_path
                                .clone()
                                .map(|path| {
                                    Icon::from_path(path)
                                        .size(IconSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element()
                                })
                                .unwrap_or_else(|| {
                                    Icon::new(IconName::Folder)
                                        .size(IconSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element()
                                }),
                        )
                        .child(Label::new(label.clone()).color(Color::Default).truncate())
                        .into_any_element()
                },
                move |_window, _cx| {
                    let mut expanded = expanded_directories.borrow_mut();
                    if expanded.contains(&directory_path) {
                        expanded.remove(&directory_path);
                    } else {
                        expanded.insert(directory_path.clone());
                    }
                },
            );
        } else {
            let label = row.name.clone();
            let workspace = workspace.clone();
            let project_path = ProjectPath {
                worktree_id,
                path: row.path,
            };
            let icon_path = row.icon_path.clone();

            menu = menu.custom_entry(
                move |_window, _cx| {
                    h_flex()
                        .w_full()
                        .pl(indentation)
                        .gap_1p5()
                        .child(div().w(px(10.)).flex_none())
                        .child(
                            icon_path
                                .clone()
                                .map(|path| {
                                    Icon::from_path(path)
                                        .size(IconSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element()
                                })
                                .unwrap_or_else(|| {
                                    Icon::new(IconName::File)
                                        .size(IconSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element()
                                }),
                        )
                        .child(Label::new(label.clone()).color(Color::Default).truncate())
                        .into_any_element()
                },
                move |window, cx| {
                    open_breadcrumb_file(project_path.clone(), &workspace, window, cx);
                },
            );
        }
    }

    menu
}

impl RenderOnce for FilePathNav {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let worktree_id = self.worktree_id;
        let project = self.project;
        let workspace = self.workspace;

        let segment_handles: Arc<Vec<PopoverMenuHandle<ContextMenu>>> = Arc::new(
            (0..self.components.len())
                .map(|_| PopoverMenuHandle::default())
                .collect(),
        );

        let mut elements: Vec<AnyElement> = Vec::new();

        for (index, component) in self.components.into_iter().enumerate() {
            if index > 0 {
                elements.push(
                    Label::new("/")
                        .color(Color::Placeholder)
                        .into_any_element(),
                );
            }

            let parent_dir = component.parent_dir.clone();
            let project = project.clone();
            let workspace = workspace.clone();
            let segment_handles_for_menu = segment_handles.clone();
            let menu_segment_index = index;
            let expanded_directories: Rc<RefCell<HashSet<Arc<RelPath>>>> =
                Rc::new(RefCell::new(HashSet::default()));
            let visible_rows: Rc<RefCell<Vec<InlineMenuRow>>> = Rc::new(RefCell::new(Vec::new()));

            let trigger_id: ElementId =
                format!("file-nav-btn-{}-{}", worktree_id.to_proto(), index).into();
            let menu_id: ElementId =
                format!("file-nav-menu-{}-{}", worktree_id.to_proto(), index).into();
            let menu_handle = segment_handles[index].clone();

            let segment = PopoverMenu::new(menu_id)
                .with_handle(menu_handle)
                .anchor(gpui::Corner::TopLeft)
                .menu(move |window, cx| {
                    let Some(project_entity) = project.upgrade() else {
                        log::error!(
                            "Breadcrumb menu open failed: project no longer available for worktree {}",
                            worktree_id.to_proto()
                        );
                        return None;
                    };
                    if project_entity.read(cx).worktree_for_id(worktree_id, cx).is_none() {
                        log::error!(
                            "Breadcrumb menu open failed: missing worktree {}",
                            worktree_id.to_proto()
                        );
                        return None;
                    }

                    let project = project.clone();
                    let workspace = workspace.clone();
                    let parent_dir = parent_dir.clone();
                    let expanded_directories = expanded_directories.clone();
                    let visible_rows = visible_rows.clone();
                    let segment_handles_for_menu = segment_handles_for_menu.clone();

                    Some(ContextMenu::build_persistent(window, cx, move |menu, window, cx| {
                        build_inline_directory_menu(
                            menu,
                            &parent_dir,
                            worktree_id,
                            &project,
                            &workspace,
                            &expanded_directories,
                            &visible_rows,
                            menu_segment_index,
                            &segment_handles_for_menu,
                            window,
                            cx,
                        )
                    }))
                })
                .trigger(
                    ButtonLike::new(trigger_id)
                        .child(Label::new(component.name).color(Color::Muted))
                        .style(ButtonStyle::Transparent),
                );

            elements.push(segment.into_any_element());
        }

        h_flex().gap_1().children(elements)
    }
}
