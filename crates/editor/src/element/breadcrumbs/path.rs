use super::super::*;
use super::BreadcrumbSegmentTarget;
use git::status::GitSummary;
use project::{ProjectEntryId, ProjectPath, WorktreeId};
use util::rel_path::RelPath;

pub(super) fn breadcrumb_path_prefixes(path: &RelPath) -> Vec<&RelPath> {
    let mut prefixes: Vec<&RelPath> = path
        .ancestors()
        .filter(|prefix| !prefix.is_empty())
        .collect();
    prefixes.reverse();
    prefixes
}

pub(super) fn breadcrumb_path_segments(
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

pub(super) const MAX_BREADCRUMB_MENU_ROWS: usize = 200;

pub(super) const MAX_UNARY_DIRECTORY_SKIP_DEPTH: usize = 64;

pub(super) fn single_child_directory(children: &[(Arc<RelPath>, bool)]) -> Option<Arc<RelPath>> {
    match children {
        [(path, true)] => Some(path.clone()),
        _ => None,
    }
}

/// Gated on the tab family for the same reason as the git colour: the bar describes the open
/// file the way a tab does, and it stands in for the prefix icon, which follows
/// `tabs.file_icons` too. The panel family stays with the menu's rows.
pub(super) fn breadcrumb_file_icon(path: Option<&RelPath>, cx: &App) -> Option<SharedString> {
    if !workspace::ItemSettings::get_global(cx).file_icons {
        return None;
    }
    file_icons::FileIcons::get_icon(path?.as_std_path(), cx)
}

/// Callers only ever ask whether there is exactly one child, and the auto-fold walk asks that
/// once per level, so the traversal stops early instead of listing the whole directory. Every
/// child counts, hidden or ignored, as in the project panel's own fold check: a directory it
/// shows unfolded must not be folded here.
pub(super) fn directory_child_paths(
    worktree: &Entity<project::Worktree>,
    path: &RelPath,
    limit: usize,
    cx: &App,
) -> Vec<(Arc<RelPath>, bool)> {
    worktree
        .read(cx)
        .snapshot()
        .child_entries(path)
        .take(limit)
        .map(|entry| (entry.path.clone(), entry.is_dir()))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DirectoryEntryIconSource {
    File,
    Folder,
    Chevron,
    None,
}

pub(super) fn directory_entry_icon_source(
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

/// Read from the settings store directly: project_panel depends on editor, so the reverse
/// dependency would be a cycle. These govern the menu's rows, which are a directory listing;
/// the bar's own icon and git colour follow the tab family instead.
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
            // The menu's rows have no disclosure to paint, so `Chevron` maps to the row
            // chevron and both icon-bearing modes map to the folder icon.
            folder_icons: project_panel.folder_indicator.unwrap().shows_icon(),
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

#[derive(Clone, Copy, Debug)]
pub(super) struct WorktreeChildListingOptions {
    sort_mode: util::paths::SortMode,
    sort_order: util::paths::SortOrder,
    hide_gitignore: bool,
    hide_hidden: bool,
    git_status_enabled: bool,
}

pub(super) struct BreadcrumbDirectoryListingInputs {
    worktree_snapshot: project::WorktreeSnapshot,
    /// Empty when git status is off, so the listing skips the per-child repository walk.
    repo_snapshots: collections::HashMap<
        project::git_store::RepositoryId,
        project::git_store::RepositorySnapshot,
    >,
    options: WorktreeChildListingOptions,
}

/// Snapshots everything the listing needs, so the traversal, git walk and sort can run off the
/// foreground thread.
pub(super) fn breadcrumb_directory_listing_inputs(
    project: &Entity<project::Project>,
    worktree: &Entity<project::Worktree>,
    cx: &App,
) -> BreadcrumbDirectoryListingInputs {
    let settings = BreadcrumbListingSettings::get_global(cx);
    BreadcrumbDirectoryListingInputs {
        worktree_snapshot: worktree.read(cx).snapshot(),
        repo_snapshots: if settings.git_status {
            project
                .read(cx)
                .git_store()
                .read(cx)
                .display_repo_snapshots(cx)
        } else {
            collections::HashMap::default()
        },
        options: WorktreeChildListingOptions {
            sort_mode: settings.sort_mode.into(),
            sort_order: settings.sort_order.into(),
            hide_gitignore: settings.hide_gitignore,
            hide_hidden: settings.hide_hidden,
            git_status_enabled: settings.git_status,
        },
    }
}

fn directory_entry(
    entry: &project::Entry,
    git_summary: GitSummary,
) -> Option<BreadcrumbDirectoryEntry> {
    Some(BreadcrumbDirectoryEntry {
        name: SharedString::new(entry.path.file_name()?),
        path: entry.path.clone(),
        entry_id: entry.id,
        is_dir: entry.is_dir(),
        is_ignored: entry.is_ignored,
        git_summary,
    })
}

pub(super) fn breadcrumb_directory_entries(
    inputs: &BreadcrumbDirectoryListingInputs,
    path: &RelPath,
) -> Vec<BreadcrumbDirectoryEntry> {
    let options = inputs.options;
    let snapshot = &inputs.worktree_snapshot;
    // Inside an ignored or hidden directory every child is ignored or hidden too. The panel
    // cannot show such a directory at all with the matching setting on, so there is no parity to
    // keep, and hiding its children would list a directory the user is standing in as empty.
    let listed = snapshot.entry_for_path(path);
    let hide_ignored = options.hide_gitignore && !listed.is_some_and(|entry| entry.is_ignored);
    let hide_hidden = options.hide_hidden && !listed.is_some_and(|entry| entry.is_hidden);
    let shown = |entry: &project::Entry| {
        !(hide_ignored && entry.is_ignored) && !(hide_hidden && entry.is_hidden)
    };
    let mut entries: Vec<BreadcrumbDirectoryEntry> = if options.git_status_enabled {
        project::ChildEntriesGitIter::new(&inputs.repo_snapshots, snapshot, path)
            .filter(|entry| shown(entry))
            .filter_map(|entry| directory_entry(&entry, entry.git_summary))
            .collect()
    } else {
        snapshot
            .child_entries(path)
            .filter(|entry| shown(entry))
            .filter_map(|entry| directory_entry(entry, GitSummary::UNCHANGED))
            .collect()
    };
    // Siblings share every component but the last, so comparing names orders them exactly as
    // comparing their full paths does, without walking the shared prefix on every comparison.
    entries.sort_by(|a, b| {
        util::paths::compare_rel_paths_by(
            (a.path.last_n_components(1).unwrap_or(&a.path), !a.is_dir),
            (b.path.last_n_components(1).unwrap_or(&b.path), !b.is_dir),
            options.sort_mode,
            options.sort_order,
        )
    });
    entries
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
