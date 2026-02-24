use std::sync::Arc;

use file_icons::FileIcons;
use gpui::{AnyElement, App, Context, ElementId, IntoElement, WeakEntity, Window};
use project::{Entry, Project, ProjectPath, WorktreeId};
use ui::{
    ButtonLike, ButtonStyle, Color, ContextMenu, ContextMenuEntry, PopoverMenu, prelude::*,
};
use util::rel_path::RelPath;
use workspace::Workspace;

struct FilePathComponent {
    name: SharedString,
    /// The directory whose contents are shown when this segment is clicked.
    ///
    /// For segment `i` in the path, this is the path prefix of the first `i`
    /// components — i.e., the parent directory of the entry that this segment
    /// represents.
    parent_dir: Arc<RelPath>,
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
            path.components().map(|s| s.to_owned()).collect();

        // `ancestors()` produces the path itself, then its parent, grandparent, …, down to "".
        // Reversed, this gives ["", "a", "a/b", …, "a/b/c/file"].
        // The parent_dir for component[i] is the i-th element of this reversed list.
        let ancestors_reversed: Vec<Arc<RelPath>> = {
            let mut v: Vec<Arc<RelPath>> = path.ancestors().map(Arc::from).collect();
            v.reverse();
            v
        };

        let mut components: Vec<FilePathComponent> = component_names
            .into_iter()
            .zip(ancestors_reversed.into_iter())
            .map(|(name, parent_dir)| FilePathComponent {
                name: SharedString::from(name),
                parent_dir,
            })
            .collect();

        if show_worktree_root {
            if let Some(root_name) = root_name {
                components.insert(
                    0,
                    FilePathComponent {
                        name: root_name,
                        parent_dir: Arc::from(RelPath::empty()),
                    },
                );
            }
        }

        Self {
            worktree_id,
            components,
            project,
            workspace,
        }
    }
}

fn build_directory_menu(
    menu: ContextMenu,
    dir_path: &RelPath,
    worktree_id: WorktreeId,
    project: &WeakEntity<Project>,
    workspace: &Option<WeakEntity<Workspace>>,
    _window: &mut Window,
    cx: &mut Context<ContextMenu>,
) -> ContextMenu {
    let Some(project_entity) = project.upgrade() else {
        return menu;
    };
    let Some(worktree) = project_entity.read(cx).worktree_for_id(worktree_id, cx) else {
        return menu;
    };

    let snapshot = worktree.read(cx);
    let mut entries: Vec<Entry> = snapshot.child_entries(dir_path).cloned().collect();

    entries.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.path.cmp(&b.path),
    });

    entries.into_iter().fold(menu, |menu, entry| {
        let Some(name) = entry.path.file_name() else {
            return menu;
        };
        let name = name.to_string();

        if entry.is_dir() {
            let entry_path = entry.path;
            let project = project.clone();
            let workspace = workspace.clone();

            menu.submenu_with_icon(name, IconName::Folder, move |submenu, window, cx| {
                build_directory_menu(
                    submenu,
                    &entry_path,
                    worktree_id,
                    &project,
                    &workspace,
                    window,
                    cx,
                )
            })
        } else {
            let icon_path = FileIcons::get_icon(entry.path.as_std_path(), cx);
            let project_path = ProjectPath {
                worktree_id,
                path: entry.path,
            };
            let workspace = workspace.clone();

            let mut menu_entry = ContextMenuEntry::new(name).handler(move |window, cx| {
                if let Some(workspace) = workspace.as_ref().and_then(|w| w.upgrade()) {
                    workspace.update(cx, |ws: &mut Workspace, cx| {
                        ws.open_path(project_path.clone(), None, true, window, cx)
                            .detach_and_log_err(cx);
                    });
                }
            });

            if let Some(icon) = icon_path {
                menu_entry = menu_entry.custom_icon_path(icon);
            }

            menu.item(menu_entry)
        }
    })
}

impl RenderOnce for FilePathNav {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let worktree_id = self.worktree_id;
        let project = self.project;
        let workspace = self.workspace;

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

            let trigger_id: ElementId =
                format!("file-nav-btn-{}-{}", worktree_id.to_proto(), index).into();
            let menu_id: ElementId =
                format!("file-nav-menu-{}-{}", worktree_id.to_proto(), index).into();

            let segment = PopoverMenu::new(menu_id)
                .anchor(gpui::Corner::TopLeft)
                .menu(move |window, cx| {
                    let project = project.clone();
                    let workspace = workspace.clone();
                    let parent_dir = parent_dir.clone();

                    Some(ContextMenu::build(window, cx, move |menu, window, cx| {
                        build_directory_menu(
                            menu,
                            &parent_dir,
                            worktree_id,
                            &project,
                            &workspace,
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
