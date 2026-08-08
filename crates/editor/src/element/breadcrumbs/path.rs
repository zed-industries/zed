use super::super::*;
use super::BreadcrumbSegmentTarget;

pub(super) fn breadcrumb_path_prefixes(path: &RelPath) -> Vec<&RelPath> {
    let mut prefixes: Vec<&RelPath> = path
        .ancestors()
        .filter(|prefix| !prefix.is_empty())
        .collect();
    prefixes.reverse();
    prefixes
}

pub(crate) fn breadcrumb_path_segments(
    worktree_id: WorktreeId,
    root_name: &str,
    path: &Arc<RelPath>,
    terminal_buffer_id: Option<BufferId>,
) -> (Vec<HighlightedText>, Vec<Option<BreadcrumbSegmentTarget>>) {
    let mut labels = Vec::new();
    let mut targets = Vec::new();

    labels.push(HighlightedText {
        text: root_name.to_string().into(),
        highlights: vec![],
    });
    targets.push(Some(BreadcrumbSegmentTarget::Directory {
        worktree_id,
        path: RelPath::empty().into_arc(),
    }));

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
                }
            },
        ));
    }

    (labels, targets)
}

pub(crate) const MAX_BREADCRUMB_MENU_ROWS: usize = 200;

pub(super) fn breadcrumb_menu_truncated_label(filter_active: bool) -> String {
    if filter_active {
        format!("Showing first {MAX_BREADCRUMB_MENU_ROWS} matches")
    } else {
        format!("Showing first {MAX_BREADCRUMB_MENU_ROWS} entries")
    }
}

pub(crate) const MAX_UNARY_DIRECTORY_SKIP_DEPTH: usize = 64;

pub(crate) fn single_child_directory(children: &[(Arc<RelPath>, bool)]) -> Option<Arc<RelPath>> {
    match children {
        [(path, true)] => Some(path.clone()),
        _ => None,
    }
}

pub(super) fn breadcrumb_file_icon(path: Option<&RelPath>, cx: &App) -> Option<SharedString> {
    if !BreadcrumbListingSettings::get_global(cx).file_icons {
        return None;
    }
    file_icons::FileIcons::get_icon(path?.as_std_path(), cx)
}

pub(super) fn directory_child_paths(
    worktree: &Entity<project::Worktree>,
    path: &RelPath,
    cx: &App,
) -> Vec<(Arc<RelPath>, bool)> {
    let settings = BreadcrumbListingSettings::get_global(cx);
    worktree
        .read(cx)
        .snapshot()
        .child_entries(path)
        .filter(|entry| !settings.hide_gitignore || !entry.is_ignored)
        .filter(|entry| !settings.hide_hidden || !entry.is_hidden)
        .map(|entry| (entry.path.clone(), entry.is_dir()))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectoryEntryIconSource {
    File,
    Folder,
    Chevron,
    None,
}

pub(crate) fn directory_entry_icon_source(
    is_dir: bool,
    show_file_icons: bool,
    show_folder_icons: bool,
) -> DirectoryEntryIconSource {
    if is_dir {
        if show_folder_icons {
            DirectoryEntryIconSource::Folder
        } else {
            DirectoryEntryIconSource::Chevron
        }
    } else if show_file_icons {
        DirectoryEntryIconSource::File
    } else {
        DirectoryEntryIconSource::None
    }
}

/// Panel listing settings, read directly to avoid a project_panel crate cycle.
#[derive(Clone, Copy, PartialEq, Eq, settings::RegisterSetting)]
pub(super) struct BreadcrumbListingSettings {
    pub(super) sort_mode: settings::ProjectPanelSortMode,
    pub(super) sort_order: settings::ProjectPanelSortOrder,
    pub(super) hide_gitignore: bool,
    pub(super) hide_hidden: bool,
    pub(super) file_icons: bool,
    pub(super) folder_icons: bool,
    pub(super) git_status: bool,
    pub(super) auto_fold_dirs: bool,
}

impl settings::Settings for BreadcrumbListingSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let project_panel = content.project_panel.clone().unwrap();
        let git_status = project_panel.git_status.unwrap()
            && content
                .git
                .as_ref()
                .and_then(|git| git.enabled.as_ref())
                .map(|enabled| enabled.is_git_status_enabled())
                .unwrap_or(true);
        Self {
            sort_mode: project_panel.sort_mode.unwrap(),
            sort_order: project_panel.sort_order.unwrap(),
            hide_gitignore: project_panel.hide_gitignore.unwrap(),
            hide_hidden: project_panel.hide_hidden.unwrap(),
            file_icons: project_panel.file_icons.unwrap(),
            folder_icons: project_panel.folder_icons.unwrap(),
            git_status,
            auto_fold_dirs: project_panel.auto_fold_dirs.unwrap(),
        }
    }
}

#[derive(Clone)]
pub(super) struct BreadcrumbDirectoryEntry {
    pub(super) name: SharedString,
    pub(super) path: Arc<RelPath>,
    pub(super) entry_id: ProjectEntryId,
    pub(super) is_dir: bool,
    pub(super) is_ignored: bool,
    pub(super) git_summary: GitSummary,
}

pub(super) fn breadcrumb_directory_entries(
    project: &Entity<project::Project>,
    worktree: &Entity<project::Worktree>,
    path: &RelPath,
    cx: &App,
) -> Vec<BreadcrumbDirectoryEntry> {
    let settings = BreadcrumbListingSettings::get_global(cx);
    let worktree_snapshot = worktree.read(cx).snapshot();
    let repo_snapshots = project
        .read(cx)
        .git_store()
        .read(cx)
        .display_repo_snapshots(cx);
    project::worktree_child_listing(
        &worktree_snapshot,
        &repo_snapshots,
        path,
        project::WorktreeChildListingOptions {
            sort_mode: settings.sort_mode.into(),
            sort_order: settings.sort_order.into(),
            hide_gitignore: settings.hide_gitignore,
            hide_hidden: settings.hide_hidden,
            git_status_enabled: settings.git_status,
        },
    )
    .into_iter()
    .filter_map(|entry| {
        let name = entry.path.file_name()?.to_string();
        Some(BreadcrumbDirectoryEntry {
            name: name.into(),
            path: entry.path,
            entry_id: entry.id,
            is_dir: entry.is_dir,
            is_ignored: entry.is_ignored,
            git_summary: entry.git_summary,
        })
    })
    .collect()
}

pub(super) fn reveal_directory_in_project_panel(
    workspace: &WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    path: &RelPath,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        return;
    };
    let project = workspace.read(cx).project().clone();
    let Some(entry_id) = project
        .read(cx)
        .entry_for_path(
            &ProjectPath {
                worktree_id,
                path: path.into(),
            },
            cx,
        )
        .map(|entry| entry.id)
    else {
        return;
    };
    project.update(cx, |_, cx| {
        cx.emit(project::Event::RevealInProjectPanel(entry_id));
    });
}
