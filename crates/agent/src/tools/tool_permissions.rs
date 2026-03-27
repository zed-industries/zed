use crate::{
    Thread, ToolCallEventStream, ToolPermissionContext, ToolPermissionDecision,
    decide_permission_for_path,
};
use anyhow::{Result, anyhow};
use fs::Fs;
use gpui::{App, Entity, Task, WeakEntity};
use project::{Project, ProjectPath};
use settings::Settings;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub enum SensitiveSettingsKind {
    Local,
    Global,
}

/// Result of resolving a path within the project with symlink safety checks.
///
/// See [`resolve_project_path`].
#[derive(Debug, Clone)]
pub enum ResolvedProjectPath {
    /// The path resolves to a location safely within the project boundaries.
    Safe(ProjectPath),
    /// The path resolves through a symlink to a location outside the project.
    /// Agent tools should prompt the user before proceeding with access.
    SymlinkEscape {
        /// The project-relative path (before symlink resolution).
        project_path: ProjectPath,
        /// The canonical (real) filesystem path the symlink points to.
        canonical_target: PathBuf,
    },
}

/// Asynchronously canonicalizes the absolute paths of all worktrees in a
/// project using the provided `Fs`. The returned paths can be passed to
/// [`resolve_project_path`] and related helpers so that they don't need to
/// perform blocking filesystem I/O themselves.
pub async fn canonicalize_worktree_roots<C: gpui::AppContext>(
    project: &Entity<Project>,
    fs: &Arc<dyn Fs>,
    cx: &C,
) -> Vec<PathBuf> {
    let abs_paths: Vec<Arc<Path>> = project.read_with(cx, |project, cx| {
        project
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path())
            .collect()
    });

    let mut canonical_roots = Vec::with_capacity(abs_paths.len());
    for abs_path in &abs_paths {
        match fs.canonicalize(abs_path).await {
            Ok(canonical) => canonical_roots.push(canonical),
            Err(_) => canonical_roots.push(abs_path.to_path_buf()),
        }
    }
    canonical_roots
}

/// Walks up ancestors of `path` to find the deepest one that exists on disk and
/// can be canonicalized, then reattaches the remaining suffix components.
///
/// This is needed for paths where the leaf (or intermediate directories) don't
/// exist yet but an ancestor may be a symlink. For example, when creating
/// `.zed/settings.json` where `.zed` is a symlink to an external directory.
///
/// Note: intermediate directories *can* be symlinks (not just leaf entries),
/// so we must walk the full ancestor chain. For example:
///   `ln -s /external/config /project/.zed`
/// makes `.zed` an intermediate symlink directory.
async fn canonicalize_with_ancestors(path: &Path, fs: &dyn Fs) -> Option<PathBuf> {
    let mut current: Option<&Path> = Some(path);
    let mut suffix_components = Vec::new();
    loop {
        match current {
            Some(ancestor) => match fs.canonicalize(ancestor).await {
                Ok(canonical) => {
                    let mut result = canonical;
                    for component in suffix_components.into_iter().rev() {
                        result.push(component);
                    }
                    return Some(result);
                }
                Err(_) => {
                    if let Some(file_name) = ancestor.file_name() {
                        suffix_components.push(file_name.to_os_string());
                    }
                    current = ancestor.parent();
                }
            },
            None => return None,
        }
    }
}

fn is_within_any_worktree(canonical_path: &Path, canonical_worktree_roots: &[PathBuf]) -> bool {
    canonical_worktree_roots
        .iter()
        .any(|root| canonical_path.starts_with(root))
}

/// Returns the kind of sensitive settings location this path targets, if any:
/// either inside a `.zed/` local-settings directory or inside the global config dir.
pub async fn sensitive_settings_kind(path: &Path, fs: &dyn Fs) -> Option<SensitiveSettingsKind> {
    let local_settings_folder = paths::local_settings_folder_name();
    if path.components().any(|component| {
        component.as_os_str() == <_ as AsRef<OsStr>>::as_ref(&local_settings_folder)
    }) {
        return Some(SensitiveSettingsKind::Local);
    }

    if let Some(canonical_path) = canonicalize_with_ancestors(path, fs).await {
        let config_dir = fs
            .canonicalize(paths::config_dir())
            .await
            .unwrap_or_else(|_| paths::config_dir().to_path_buf());
        if canonical_path.starts_with(&config_dir) {
            return Some(SensitiveSettingsKind::Global);
        }
    }

    None
}

pub async fn is_sensitive_settings_path(path: &Path, fs: &dyn Fs) -> bool {
    sensitive_settings_kind(path, fs).await.is_some()
}

/// Resolves a path within the project, checking for symlink escapes.
///
/// This is the primary entry point for agent tools that need to resolve a
/// user-provided path string into a validated `ProjectPath`. It combines
/// path lookup (`find_project_path`) with symlink safety verification.
///
/// `canonical_worktree_roots` should be obtained from
/// [`canonicalize_worktree_roots`] before calling this function so that no
/// blocking I/O is needed here.
///
/// # Returns
///
/// - `Ok(ResolvedProjectPath::Safe(project_path))` — the path resolves to a
///   location within the project boundaries.
/// - `Ok(ResolvedProjectPath::SymlinkEscape { .. })` — the path resolves
///   through a symlink to a location outside the project. Agent tools should
///   prompt the user before proceeding.
/// - `Err(..)` — the path could not be found in the project or could not be
///   verified. The error message is suitable for returning to the model.
pub fn resolve_project_path(
    project: &Project,
    path: impl AsRef<Path>,
    canonical_worktree_roots: &[PathBuf],
    cx: &App,
) -> Result<ResolvedProjectPath> {
    let path = path.as_ref();
    let project_path = project
        .find_project_path(path, cx)
        .ok_or_else(|| anyhow!("Path {} is not in the project", path.display()))?;

    let worktree = project
        .worktree_for_id(project_path.worktree_id, cx)
        .ok_or_else(|| anyhow!("Could not resolve path {}", path.display()))?;
    let snapshot = worktree.read(cx);

    // Fast path: if the entry exists in the snapshot and is not marked
    // external, we know it's safe (the background scanner already verified).
    if let Some(entry) = snapshot.entry_for_path(&project_path.path) {
        if !entry.is_external {
            return Ok(ResolvedProjectPath::Safe(project_path));
        }

        // Entry is external (set by the worktree scanner when a symlink's
        // canonical target is outside the worktree root). Return the
        // canonical path if the entry has one, otherwise fall through to
        // filesystem-level canonicalization.
        if let Some(canonical) = &entry.canonical_path {
            if is_within_any_worktree(canonical.as_ref(), canonical_worktree_roots) {
                return Ok(ResolvedProjectPath::Safe(project_path));
            }

            return Ok(ResolvedProjectPath::SymlinkEscape {
                project_path,
                canonical_target: canonical.to_path_buf(),
            });
        }
    }

    // For missing/create-mode paths (or external descendants without their own
    // canonical_path), resolve symlink safety through snapshot metadata rather
    // than std::fs canonicalization. This keeps behavior correct for non-local
    // worktrees and in-memory fs backends.
    for ancestor in project_path.path.ancestors() {
        let Some(ancestor_entry) = snapshot.entry_for_path(ancestor) else {
            continue;
        };

        if !ancestor_entry.is_external {
            return Ok(ResolvedProjectPath::Safe(project_path));
        }

        let Some(canonical_ancestor) = ancestor_entry.canonical_path.as_ref() else {
            continue;
        };

        let suffix = project_path.path.strip_prefix(ancestor).map_err(|_| {
            anyhow!(
                "Path {} could not be resolved in the project",
                path.display()
            )
        })?;

        let canonical_target = if suffix.is_empty() {
            canonical_ancestor.to_path_buf()
        } else {
            canonical_ancestor.join(suffix.as_std_path())
        };

        if is_within_any_worktree(&canonical_target, canonical_worktree_roots) {
            return Ok(ResolvedProjectPath::Safe(project_path));
        }

        return Ok(ResolvedProjectPath::SymlinkEscape {
            project_path,
            canonical_target,
        });
    }

    Ok(ResolvedProjectPath::Safe(project_path))
}

/// Prompts the user for permission when a path resolves through a symlink to a
/// location outside the project. This check is an additional gate after
/// settings-based deny decisions: even if a tool is configured as "always allow,"
/// a symlink escape still requires explicit user approval.
pub fn authorize_symlink_access(
    tool_name: &str,
    display_path: &str,
    canonical_target: &Path,
    event_stream: &ToolCallEventStream,
    cx: &mut App,
) -> Task<Result<()>> {
    let title = format!(
        "`{}` points outside the project (symlink to `{}`)",
        display_path,
        canonical_target.display(),
    );

    let context = ToolPermissionContext::symlink_target(
        tool_name,
        vec![canonical_target.display().to_string()],
    );

    event_stream.authorize(title, context, cx)
}

/// Creates a single authorization prompt for multiple symlink escapes.
/// Each escape is a `(display_path, canonical_target)` pair.
///
/// Accepts `&[(&str, PathBuf)]` to match the natural return type of
/// [`detect_symlink_escape`], avoiding intermediate owned-to-borrowed
/// conversions at call sites.
pub fn authorize_symlink_escapes(
    tool_name: &str,
    escapes: &[(&str, PathBuf)],
    event_stream: &ToolCallEventStream,
    cx: &mut App,
) -> Task<Result<()>> {
    debug_assert!(!escapes.is_empty());

    if escapes.len() == 1 {
        return authorize_symlink_access(tool_name, escapes[0].0, &escapes[0].1, event_stream, cx);
    }

    let targets = escapes
        .iter()
        .map(|(path, target)| format!("`{}` → `{}`", path, target.display()))
        .collect::<Vec<_>>()
        .join(" and ");
    let title = format!("{} (symlinks outside project)", targets);

    let context = ToolPermissionContext::symlink_target(
        tool_name,
        escapes
            .iter()
            .map(|(_, target)| target.display().to_string())
            .collect(),
    );

    event_stream.authorize(title, context, cx)
}

/// Checks whether a path escapes the project via symlink, without creating
/// an authorization task. Useful for pre-filtering paths before settings checks.
pub fn path_has_symlink_escape(
    project: &Project,
    path: impl AsRef<Path>,
    canonical_worktree_roots: &[PathBuf],
    cx: &App,
) -> bool {
    matches!(
        resolve_project_path(project, path, canonical_worktree_roots, cx),
        Ok(ResolvedProjectPath::SymlinkEscape { .. })
    )
}

/// Collects symlink escape info for a path without creating an authorization task.
/// Returns `Some((display_path, canonical_target))` if the path escapes via symlink.
pub fn detect_symlink_escape<'a>(
    project: &Project,
    display_path: &'a str,
    canonical_worktree_roots: &[PathBuf],
    cx: &App,
) -> Option<(&'a str, PathBuf)> {
    match resolve_project_path(project, display_path, canonical_worktree_roots, cx).ok()? {
        ResolvedProjectPath::Safe(_) => None,
        ResolvedProjectPath::SymlinkEscape {
            canonical_target, ..
        } => Some((display_path, canonical_target)),
    }
}

/// Collects symlink escape info for two paths (source and destination) and
/// returns any escapes found. This deduplicates the common pattern used by
/// tools that operate on two paths (copy, move).
///
/// Returns a `Vec` of `(display_path, canonical_target)` pairs for paths
/// that escape the project via symlink. The returned vec borrows the display
/// paths from the input strings.
pub fn collect_symlink_escapes<'a>(
    project: &Project,
    source_path: &'a str,
    destination_path: &'a str,
    canonical_worktree_roots: &[PathBuf],
    cx: &App,
) -> Vec<(&'a str, PathBuf)> {
    let mut escapes = Vec::new();
    if let Some(escape) = detect_symlink_escape(project, source_path, canonical_worktree_roots, cx)
    {
        escapes.push(escape);
    }
    if let Some(escape) =
        detect_symlink_escape(project, destination_path, canonical_worktree_roots, cx)
    {
        escapes.push(escape);
    }
    escapes
}

/// Checks authorization for file edits, handling symlink escapes and
/// sensitive settings paths.
///
/// # Authorization precedence
///
/// When a symlink escape is detected, the symlink authorization prompt
/// *replaces* (rather than supplements) the normal tool-permission prompt.
/// This is intentional: the symlink prompt already requires explicit user
/// approval and displays the canonical target, which provides strictly more
/// security-relevant information than the generic tool confirmation. Requiring
/// two sequential prompts for the same operation would degrade UX without
/// meaningfully improving security, since the user must already approve the
/// more specific symlink-escape prompt.
pub fn authorize_file_edit(
    tool_name: &str,
    path: &Path,
    display_description: &str,
    thread: &WeakEntity<Thread>,
    event_stream: &ToolCallEventStream,
    cx: &mut App,
) -> Task<Result<()>> {
    let path_str = path.to_string_lossy();

    let settings = agent_settings::AgentSettings::get_global(cx);
    let decision = decide_permission_for_path(tool_name, &path_str, settings);

    if let ToolPermissionDecision::Deny(reason) = decision {
        return Task::ready(Err(anyhow!("{}", reason)));
    }

    let path_owned = path.to_path_buf();
    let display_description = display_description.to_string();
    let tool_name = tool_name.to_string();
    let thread = thread.clone();
    let event_stream = event_stream.clone();

    // The local settings folder check is synchronous (pure path inspection),
    // so we can handle this common case without spawning.
    let local_settings_folder = paths::local_settings_folder_name();
    let is_local_settings = path.components().any(|component| {
        component.as_os_str() == <_ as AsRef<OsStr>>::as_ref(&local_settings_folder)
    });

    cx.spawn(async move |cx| {
        // Resolve the path and check for symlink escapes.
        let (project_entity, fs) = thread.read_with(cx, |thread, cx| {
            let project = thread.project().clone();
            let fs = project.read(cx).fs().clone();
            (project, fs)
        })?;

        let canonical_roots = canonicalize_worktree_roots(&project_entity, &fs, cx).await;

        let resolved = project_entity.read_with(cx, |project, cx| {
            resolve_project_path(project, &path_owned, &canonical_roots, cx)
        });

        if let Ok(ResolvedProjectPath::SymlinkEscape {
            canonical_target, ..
        }) = &resolved
        {
            let authorize = cx.update(|cx| {
                authorize_symlink_access(
                    &tool_name,
                    &path_owned.to_string_lossy(),
                    canonical_target,
                    &event_stream,
                    cx,
                )
            });
            return authorize.await;
        }

        // Create-mode paths may not resolve yet, so also inspect the parent path
        // for symlink escapes before applying settings-based allow decisions.
        if resolved.is_err() {
            if let Some(parent_path) = path_owned.parent() {
                let parent_resolved = project_entity.read_with(cx, |project, cx| {
                    resolve_project_path(project, parent_path, &canonical_roots, cx)
                });

                if let Ok(ResolvedProjectPath::SymlinkEscape {
                    canonical_target, ..
                }) = &parent_resolved
                {
                    let authorize = cx.update(|cx| {
                        authorize_symlink_access(
                            &tool_name,
                            &path_owned.to_string_lossy(),
                            canonical_target,
                            &event_stream,
                            cx,
                        )
                    });
                    return authorize.await;
                }
            }
        }

        let explicitly_allowed = matches!(decision, ToolPermissionDecision::Allow);

        // Check sensitive settings asynchronously.
        let settings_kind = if is_local_settings {
            Some(SensitiveSettingsKind::Local)
        } else {
            sensitive_settings_kind(&path_owned, fs.as_ref()).await
        };

        let is_sensitive = settings_kind.is_some();
        if explicitly_allowed && !is_sensitive {
            return Ok(());
        }

        match settings_kind {
            Some(SensitiveSettingsKind::Local) => {
                let authorize = cx.update(|cx| {
                    let context = ToolPermissionContext::new(
                        &tool_name,
                        vec![path_owned.to_string_lossy().to_string()],
                    );
                    event_stream.authorize(
                        format!("{} (local settings)", display_description),
                        context,
                        cx,
                    )
                });
                return authorize.await;
            }
            Some(SensitiveSettingsKind::Global) => {
                let authorize = cx.update(|cx| {
                    let context = ToolPermissionContext::new(
                        &tool_name,
                        vec![path_owned.to_string_lossy().to_string()],
                    );
                    event_stream.authorize(
                        format!("{} (settings)", display_description),
                        context,
                        cx,
                    )
                });
                return authorize.await;
            }
            None => {}
        }

        match resolved {
            Ok(_) => Ok(()),
            Err(_) => {
                let authorize = cx.update(|cx| {
                    let context = ToolPermissionContext::new(
                        &tool_name,
                        vec![path_owned.to_string_lossy().to_string()],
                    );
                    event_stream.authorize(&display_description, context, cx)
                });
                authorize.await
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::Fs;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    async fn worktree_roots(
        project: &Entity<Project>,
        fs: &Arc<dyn Fs>,
        cx: &TestAppContext,
    ) -> Vec<PathBuf> {
        let abs_paths: Vec<Arc<Path>> = project.read_with(cx, |project, cx| {
            project
                .worktrees(cx)
                .map(|wt| wt.read(cx).abs_path())
                .collect()
        });

        let mut roots = Vec::with_capacity(abs_paths.len());
        for p in &abs_paths {
            match fs.canonicalize(p).await {
                Ok(c) => roots.push(c),
                Err(_) => roots.push(p.to_path_buf()),
            }
        }
        roots
    }

    #[gpui::test]
    async fn test_resolve_project_path_safe_for_normal_files(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root/project"),
            json!({
                "src": {
                    "main.rs": "fn main() {}",
                    "lib.rs": "pub fn hello() {}"
                },
                "README.md": "# Project"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.run_until_parked();
        let fs_arc: Arc<dyn Fs> = fs;
        let roots = worktree_roots(&project, &fs_arc, cx).await;

        cx.read(|cx| {
            let project = project.read(cx);

            let resolved = resolve_project_path(project, "project/src/main.rs", &roots, cx)
                .expect("should resolve normal file");
            assert!(
                matches!(resolved, ResolvedProjectPath::Safe(_)),
                "normal file should be Safe, got: {:?}",
                resolved
            );

            let resolved = resolve_project_path(project, "project/README.md", &roots, cx)
                .expect("should resolve readme");
            assert!(
                matches!(resolved, ResolvedProjectPath::Safe(_)),
                "readme should be Safe, got: {:?}",
                resolved
            );
        });
    }

    #[gpui::test]
    async fn test_resolve_project_path_detects_symlink_escape(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                },
                "external": {
                    "secret.txt": "top secret"
                }
            }),
        )
        .await;

        fs.create_symlink(path!("/root/project/link").as_ref(), "../external".into())
            .await
            .expect("should create symlink");

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.run_until_parked();
        let fs_arc: Arc<dyn Fs> = fs;
        let roots = worktree_roots(&project, &fs_arc, cx).await;

        cx.read(|cx| {
            let project = project.read(cx);

            let resolved = resolve_project_path(project, "project/link", &roots, cx)
                .expect("should resolve symlink path");
            match &resolved {
                ResolvedProjectPath::SymlinkEscape {
                    canonical_target, ..
                } => {
                    assert_eq!(
                        canonical_target,
                        Path::new(path!("/root/external")),
                        "canonical target should point to external directory"
                    );
                }
                ResolvedProjectPath::Safe(_) => {
                    panic!("symlink escaping project should be detected as SymlinkEscape");
                }
            }
        });
    }

    #[gpui::test]
    async fn test_resolve_project_path_allows_intra_project_symlinks(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root/project"),
            json!({
                "real_dir": {
                    "file.txt": "hello"
                }
            }),
        )
        .await;

        fs.create_symlink(path!("/root/project/link_dir").as_ref(), "real_dir".into())
            .await
            .expect("should create symlink");

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.run_until_parked();
        let fs_arc: Arc<dyn Fs> = fs;
        let roots = worktree_roots(&project, &fs_arc, cx).await;

        cx.read(|cx| {
            let project = project.read(cx);

            let resolved = resolve_project_path(project, "project/link_dir", &roots, cx)
                .expect("should resolve intra-project symlink");
            assert!(
                matches!(resolved, ResolvedProjectPath::Safe(_)),
                "intra-project symlink should be Safe, got: {:?}",
                resolved
            );
        });
    }

    #[gpui::test]
    async fn test_resolve_project_path_missing_child_under_external_symlink(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {},
                "external": {
                    "existing.txt": "hello"
                }
            }),
        )
        .await;

        fs.create_symlink(path!("/root/project/link").as_ref(), "../external".into())
            .await
            .expect("should create symlink");

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.run_until_parked();
        let fs_arc: Arc<dyn Fs> = fs;
        let roots = worktree_roots(&project, &fs_arc, cx).await;

        cx.read(|cx| {
            let project = project.read(cx);

            let resolved = resolve_project_path(project, "project/link/new_dir", &roots, cx)
                .expect("should resolve missing child path under symlink");
            match resolved {
                ResolvedProjectPath::SymlinkEscape {
                    canonical_target, ..
                } => {
                    assert_eq!(
                        canonical_target,
                        Path::new(path!("/root/external/new_dir")),
                        "missing child path should resolve to escaped canonical target",
                    );
                }
                ResolvedProjectPath::Safe(_) => {
                    panic!("missing child under external symlink should be SymlinkEscape");
                }
            }
        });
    }

    #[gpui::test]
    async fn test_resolve_project_path_allows_cross_worktree_symlinks(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "worktree_one": {},
                "worktree_two": {
                    "shared_dir": {
                        "file.txt": "hello"
                    }
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/worktree_one/link_to_worktree_two").as_ref(),
            PathBuf::from("../worktree_two/shared_dir"),
        )
        .await
        .expect("should create symlink");

        let project = Project::test(
            fs.clone(),
            [
                path!("/root/worktree_one").as_ref(),
                path!("/root/worktree_two").as_ref(),
            ],
            cx,
        )
        .await;
        cx.run_until_parked();
        let fs_arc: Arc<dyn Fs> = fs;
        let roots = worktree_roots(&project, &fs_arc, cx).await;

        cx.read(|cx| {
            let project = project.read(cx);

            let resolved =
                resolve_project_path(project, "worktree_one/link_to_worktree_two", &roots, cx)
                    .expect("should resolve cross-worktree symlink");
            assert!(
                matches!(resolved, ResolvedProjectPath::Safe(_)),
                "cross-worktree symlink should be Safe, got: {:?}",
                resolved
            );
        });
    }

    #[gpui::test]
    async fn test_resolve_project_path_missing_child_under_cross_worktree_symlink(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "worktree_one": {},
                "worktree_two": {
                    "shared_dir": {}
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/worktree_one/link_to_worktree_two").as_ref(),
            PathBuf::from("../worktree_two/shared_dir"),
        )
        .await
        .expect("should create symlink");

        let project = Project::test(
            fs.clone(),
            [
                path!("/root/worktree_one").as_ref(),
                path!("/root/worktree_two").as_ref(),
            ],
            cx,
        )
        .await;
        cx.run_until_parked();
        let fs_arc: Arc<dyn Fs> = fs;
        let roots = worktree_roots(&project, &fs_arc, cx).await;

        cx.read(|cx| {
            let project = project.read(cx);

            let resolved = resolve_project_path(
                project,
                "worktree_one/link_to_worktree_two/new_dir",
                &roots,
                cx,
            )
            .expect("should resolve missing child under cross-worktree symlink");
            assert!(
                matches!(resolved, ResolvedProjectPath::Safe(_)),
                "missing child under cross-worktree symlink should be Safe, got: {:?}",
                resolved
            );
        });
    }
}
