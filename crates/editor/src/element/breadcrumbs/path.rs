use super::*;

use language::DiagnosticSeverity;
use project::{Project, ProjectPath};
use settings::ShowDiagnostics;

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

/// Mirrors `project_panel`'s ordering, visibility and icon settings so the dropdown agrees with
/// the panel. Read from `SettingsContent` independently rather than through `project_panel`'s
/// resolved settings, since `project_panel` depends on `editor` and the reverse would be
/// circular.
#[derive(Clone, Copy, settings::RegisterSetting)]
pub struct BreadcrumbDirectoryListingSettings {
    pub sort_mode: settings::ProjectPanelSortMode,
    pub sort_order: settings::ProjectPanelSortOrder,
    pub hide_gitignore: bool,
    pub hide_hidden: bool,
    pub file_icons: bool,
    pub folder_icons: bool,
    pub git_status: bool,
    pub show_diagnostics: ShowDiagnostics,
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
            git_status: project_panel.git_status.unwrap(),
            show_diagnostics: project_panel.show_diagnostics.unwrap(),
        }
    }
}

/// Mirrors `ProjectPanel::update_diagnostics`'s own filtering (see
/// `crates/project_panel/src/project_panel.rs`): errors always count, warnings only count when
/// the setting is `all`, and the setting can turn diagnostics off entirely. Reads the aggregated
/// per-path summary rather than walking every diagnostic, since this runs on every breadcrumb
/// render.
pub fn breadcrumb_diagnostic_severity(
    project: &Project,
    project_path: &ProjectPath,
    show_diagnostics: ShowDiagnostics,
    cx: &App,
) -> Option<DiagnosticSeverity> {
    if show_diagnostics == ShowDiagnostics::Off {
        return None;
    }
    let summary = project.diagnostic_summary_for_path(project_path, cx);
    if summary.error_count > 0 {
        Some(DiagnosticSeverity::ERROR)
    } else if show_diagnostics == ShowDiagnostics::All && summary.warning_count > 0 {
        Some(DiagnosticSeverity::WARNING)
    } else {
        None
    }
}

/// A single row in a breadcrumb directory dropdown: one of `path`'s direct children, sorted the
/// way the project panel orders siblings (see [`BreadcrumbDirectoryListingSettings`]).
pub struct BreadcrumbDirectoryEntry {
    pub name: SharedString,
    pub path: Arc<RelPath>,
    pub is_dir: bool,
    pub is_ignored: bool,
    pub git_summary: GitSummary,
    /// Only ever set for files: mirroring the panel's ancestor-propagated severity for
    /// directories would mean scanning every diagnostic summary per row, which this listing's
    /// render path can't afford.
    pub diagnostic_severity: Option<DiagnosticSeverity>,
}

/// Lists `path`'s direct children, filtered the way the project panel filters gitignored and
/// hidden entries.
pub fn breadcrumb_directory_entries(
    project: &Entity<Project>,
    worktree: &Entity<project::Worktree>,
    path: &RelPath,
    cx: &App,
) -> Vec<BreadcrumbDirectoryEntry> {
    let settings = BreadcrumbDirectoryListingSettings::get_global(cx);
    let worktree_snapshot = worktree.read(cx).snapshot();
    let worktree_id = worktree_snapshot.id();
    let project_ref = project.read(cx);
    let repo_snapshots = project_ref.git_store().read(cx).display_repo_snapshots(cx);
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
            let diagnostic_severity = (!entry.is_dir())
                .then(|| {
                    breadcrumb_diagnostic_severity(
                        project_ref,
                        &ProjectPath {
                            worktree_id,
                            path: entry.path.clone(),
                        },
                        settings.show_diagnostics,
                        cx,
                    )
                })
                .flatten();
            Some(BreadcrumbDirectoryEntry {
                name: name.into(),
                path: entry.path.clone(),
                is_dir: entry.is_dir(),
                is_ignored: entry.is_ignored,
                git_summary: entry.git_summary,
                diagnostic_severity,
            })
        })
        .collect()
}

/// Whether the leading segment offers navigation at all: `false` for a buffer with no project
/// path, and for a single-file worktree, which has no tree to browse.
pub(crate) fn breadcrumb_path_is_navigable(
    has_project_path: bool,
    worktree_is_single_file: Option<bool>,
) -> bool {
    has_project_path && !worktree_is_single_file.unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

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

    #[gpui::test]
    async fn test_breadcrumb_directory_entries_honors_show_diagnostics_setting(
        cx: &mut TestAppContext,
    ) {
        use crate::editor_tests::init_test;
        use language::{Diagnostic, DiagnosticEntry, DiagnosticSourceKind};
        use lsp::{DiagnosticSeverity as LspDiagnosticSeverity, LanguageServerId};
        use project::{FakeFs, Project};
        use serde_json::json;
        use settings::SettingsStore;
        use std::path::Path;
        use text::{PointUtf16, Unclipped};
        use util::path;

        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "error.txt": "",
                "warning.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        cx.run_until_parked();

        let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
        lsp_store.update(cx, |lsp_store, cx| {
            let diagnostic = |severity, message: &str| DiagnosticEntry {
                range: Unclipped(PointUtf16::new(0, 0))..Unclipped(PointUtf16::new(0, 1)),
                diagnostic: Diagnostic {
                    severity,
                    is_primary: true,
                    message: message.to_string(),
                    source_kind: DiagnosticSourceKind::Pushed,
                    ..Diagnostic::default()
                },
            };
            lsp_store
                .update_diagnostic_entries(
                    LanguageServerId(0),
                    Path::new(path!("/root/error.txt")).to_owned(),
                    None,
                    None,
                    vec![diagnostic(LspDiagnosticSeverity::ERROR, "error")],
                    cx,
                )
                .unwrap();
            lsp_store
                .update_diagnostic_entries(
                    LanguageServerId(0),
                    Path::new(path!("/root/warning.txt")).to_owned(),
                    None,
                    None,
                    vec![diagnostic(LspDiagnosticSeverity::WARNING, "warning")],
                    cx,
                )
                .unwrap();
        });
        cx.run_until_parked();

        // Default (`all`, matching the project panel's own default): both severities surface.
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert_eq!(
            entries
                .iter()
                .find(|entry| entry.name.as_ref() == "error.txt")
                .and_then(|entry| entry.diagnostic_severity),
            Some(DiagnosticSeverity::ERROR),
        );
        assert_eq!(
            entries
                .iter()
                .find(|entry| entry.name.as_ref() == "warning.txt")
                .and_then(|entry| entry.diagnostic_severity),
            Some(DiagnosticSeverity::WARNING),
        );

        // `errors`: warnings drop out, errors still surface — same filtering the panel applies
        // in `ProjectPanel::update_diagnostics`.
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project_panel
                        .get_or_insert_default()
                        .show_diagnostics = Some(settings::ShowDiagnostics::Errors);
                });
            });
        });
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert_eq!(
            entries
                .iter()
                .find(|entry| entry.name.as_ref() == "error.txt")
                .and_then(|entry| entry.diagnostic_severity),
            Some(DiagnosticSeverity::ERROR),
            "errors still surface under `errors`",
        );
        assert_eq!(
            entries
                .iter()
                .find(|entry| entry.name.as_ref() == "warning.txt")
                .and_then(|entry| entry.diagnostic_severity),
            None,
            "warnings are filtered out under `errors`",
        );

        // `off`: the setting suppresses diagnostics entirely, not just warnings.
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project_panel
                        .get_or_insert_default()
                        .show_diagnostics = Some(settings::ShowDiagnostics::Off);
                });
            });
        });
        let entries =
            cx.update(|cx| breadcrumb_directory_entries(&project, &worktree, RelPath::empty(), cx));
        assert!(
            entries
                .iter()
                .all(|entry| entry.diagnostic_severity.is_none()),
            "`off` suppresses diagnostics entirely",
        );
    }
}
