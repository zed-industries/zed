use crate::Workspace;
use gpui::{App, AppContext, Entity, Task, WeakEntity};
use itertools::Itertools;
use project::{Entry, Worktree};
use std::path::{Path, PathBuf};
use util::{
    paths::{PathStyle, PathWithPosition, normalize_lexically},
    rel_path::RelPath,
};

#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OpenTargetFoundBy {
    WorktreeExact,
    WorktreeScan,
    BackgroundPathResolution,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BackgroundPathChecks {
    LocalFileSystem,
    ProjectPathResolution,
}

#[derive(Debug, Clone)]
pub enum OpenTarget {
    Worktree(
        PathWithPosition,
        Entry,
        #[cfg(any(test, feature = "test-support"))] OpenTargetFoundBy,
    ),
    Path(
        PathWithPosition,
        bool,
        #[cfg(any(test, feature = "test-support"))] OpenTargetFoundBy,
    ),
}

impl OpenTarget {
    pub fn is_file(&self) -> bool {
        match self {
            OpenTarget::Worktree(_, entry, ..) => entry.is_file(),
            OpenTarget::Path(_, is_dir, ..) => !is_dir,
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            OpenTarget::Worktree(_, entry, ..) => entry.is_dir(),
            OpenTarget::Path(_, is_dir, ..) => *is_dir,
        }
    }

    pub fn path(&self) -> &PathWithPosition {
        match self {
            OpenTarget::Worktree(path, ..) => path,
            OpenTarget::Path(path, ..) => path,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn found_by(&self) -> OpenTargetFoundBy {
        match self {
            OpenTarget::Worktree(.., found_by) => *found_by,
            OpenTarget::Path(.., found_by) => *found_by,
        }
    }
}

pub fn sanitize_path_text(text: &str) -> &str {
    let start = first_unbalanced_open_paren(text).unwrap_or(0);
    let mut sanitized = &text[start..];
    let (open_parens, mut close_parens) =
        sanitized
            .chars()
            .fold((0, 0), |(opens, closes), character| match character {
                '(' => (opens + 1, closes),
                ')' => (opens, closes + 1),
                _ => (opens, closes),
            });

    while let Some(last_char) = sanitized.chars().last() {
        let should_remove = match last_char {
            '.' | ',' | ':' | ';' => true,
            '(' => true,
            ')' if close_parens > open_parens => {
                close_parens -= 1;
                true
            }
            _ => false,
        };

        if should_remove {
            sanitized = &sanitized[..sanitized.len() - last_char.len_utf8()];
        } else {
            break;
        }
    }

    sanitized
}

/// Returns the byte offset just past the first unbalanced `(` in `text`, or
/// `None` if all parentheses are balanced.
pub fn first_unbalanced_open_paren(text: &str) -> Option<usize> {
    let mut balance: i32 = 0;
    let mut first_unmatched = None;
    for (index, character) in text.char_indices() {
        match character {
            '(' => {
                if balance == 0 {
                    first_unmatched = Some(index + character.len_utf8());
                }
                balance += 1;
            }
            ')' => {
                balance -= 1;
                if balance <= 0 {
                    balance = 0;
                    first_unmatched = None;
                }
            }
            _ => {}
        }
    }
    first_unmatched.filter(|_| balance > 0)
}

pub fn possible_open_target(
    workspace: &WeakEntity<Workspace>,
    maybe_path: &str,
    cwd: Option<&Path>,
    cx: &App,
) -> Task<Option<OpenTarget>> {
    possible_open_target_internal(workspace, maybe_path, cwd, cx, None)
}

#[cfg(any(test, feature = "test-support"))]
pub fn possible_open_target_with_fs_checks(
    workspace: &WeakEntity<Workspace>,
    maybe_path: &str,
    cwd: Option<&Path>,
    cx: &App,
    background_path_checks: BackgroundPathChecks,
) -> Task<Option<OpenTarget>> {
    possible_open_target_internal(workspace, maybe_path, cwd, cx, Some(background_path_checks))
}

fn possible_open_target_internal(
    workspace: &WeakEntity<Workspace>,
    maybe_path: &str,
    cwd: Option<&Path>,
    cx: &App,
    background_path_checks: Option<BackgroundPathChecks>,
) -> Task<Option<OpenTarget>> {
    let Some(workspace) = workspace.upgrade() else {
        return Task::ready(None);
    };

    let mut potential_paths = Vec::new();
    let original_path = PathWithPosition::from_path(PathBuf::from(maybe_path));
    let path_with_position = PathWithPosition::parse_str(maybe_path);
    let worktree_candidates = workspace
        .read(cx)
        .worktrees(cx)
        .sorted_by_key(|worktree| {
            let worktree_root = worktree.read(cx).abs_path();
            match cwd.and_then(|cwd| worktree_root.strip_prefix(cwd).ok()) {
                Some(cwd_child) => cwd_child.components().count(),
                None => usize::MAX,
            }
        })
        .collect::<Vec<_>>();

    const GIT_DIFF_PATH_PREFIXES: &[&str] = &["a", "b"];
    for prefix_str in GIT_DIFF_PATH_PREFIXES.iter().chain(std::iter::once(&".")) {
        if let Some(stripped) = original_path.path.strip_prefix(prefix_str).ok() {
            potential_paths.push(PathWithPosition {
                path: stripped.to_owned(),
                row: original_path.row,
                column: original_path.column,
            });
        }
        if let Some(stripped) = path_with_position.path.strip_prefix(prefix_str).ok() {
            potential_paths.push(PathWithPosition {
                path: stripped.to_owned(),
                row: path_with_position.row,
                column: path_with_position.column,
            });
        }
    }

    let insert_both_paths = original_path != path_with_position;
    potential_paths.insert(0, original_path);
    if insert_both_paths {
        potential_paths.insert(1, path_with_position);
    }

    let mut worktree_paths_to_check = Vec::new();
    let mut is_cwd_in_worktree = false;
    let mut open_target = None;
    'worktree_loop: for worktree in &worktree_candidates {
        let worktree_root = worktree.read(cx).abs_path();
        let mut paths_to_check = Vec::with_capacity(potential_paths.len());
        let relative_cwd = cwd
            .and_then(|cwd| cwd.strip_prefix(&worktree_root).ok())
            .and_then(|cwd| RelPath::new(cwd, PathStyle::local()).ok())
            .and_then(|cwd_stripped| {
                (cwd_stripped.as_ref() != RelPath::empty()).then(|| {
                    is_cwd_in_worktree = true;
                    cwd_stripped
                })
            });

        for path_with_position in &potential_paths {
            let path_to_check = if worktree_root.ends_with(&path_with_position.path) {
                let root_path_with_position = PathWithPosition {
                    path: worktree_root.to_path_buf(),
                    row: path_with_position.row,
                    column: path_with_position.column,
                };
                match worktree.read(cx).root_entry() {
                    Some(root_entry) => {
                        open_target = Some(OpenTarget::Worktree(
                            root_path_with_position,
                            root_entry.clone(),
                            #[cfg(any(test, feature = "test-support"))]
                            OpenTargetFoundBy::WorktreeExact,
                        ));
                        break 'worktree_loop;
                    }
                    None => root_path_with_position,
                }
            } else {
                PathWithPosition {
                    path: path_with_position
                        .path
                        .strip_prefix(&worktree_root)
                        .unwrap_or(&path_with_position.path)
                        .to_owned(),
                    row: path_with_position.row,
                    column: path_with_position.column,
                }
            };

            let normalized_path = if path_to_check.path.is_relative() {
                relative_cwd.as_ref().and_then(|relative_cwd| {
                    let joined = relative_cwd
                        .as_ref()
                        .as_std_path()
                        .join(&path_to_check.path);
                    normalize_lexically(&joined).ok().and_then(|path| {
                        RelPath::new(&path, PathStyle::local())
                            .ok()
                            .map(std::borrow::Cow::into_owned)
                    })
                })
            } else {
                None
            };
            let original_path = RelPath::new(&path_to_check.path, PathStyle::local()).ok();

            if !worktree.read(cx).is_single_file()
                && let Some(entry) = normalized_path
                    .as_ref()
                    .and_then(|path| worktree.read(cx).entry_for_path(path))
                    .or_else(|| {
                        original_path
                            .as_ref()
                            .and_then(|path| worktree.read(cx).entry_for_path(path.as_ref()))
                    })
            {
                open_target = Some(OpenTarget::Worktree(
                    PathWithPosition {
                        path: worktree.read(cx).absolutize(&entry.path),
                        row: path_to_check.row,
                        column: path_to_check.column,
                    },
                    entry.clone(),
                    #[cfg(any(test, feature = "test-support"))]
                    OpenTargetFoundBy::WorktreeExact,
                ));
                break 'worktree_loop;
            }

            paths_to_check.push(path_to_check);
        }

        if !paths_to_check.is_empty() {
            worktree_paths_to_check.push((worktree.clone(), paths_to_check));
        }
    }

    if open_target.is_some() {
        if is_cwd_in_worktree {
            return Task::ready(open_target);
        }
    }

    let project = workspace.read(cx).project().clone();
    let background_path_checks = background_path_checks.unwrap_or_else(|| {
        if project.read(cx).is_local() {
            BackgroundPathChecks::LocalFileSystem
        } else {
            BackgroundPathChecks::ProjectPathResolution
        }
    });

    let background_resolution_task = match background_path_checks {
        BackgroundPathChecks::LocalFileSystem => {
            let fs_paths_to_check =
                local_paths_to_check(&potential_paths, cwd, &worktree_candidates, cx);
            let fs = project.read(cx).fs().clone();
            cx.background_spawn(async move {
                for mut path_to_check in fs_paths_to_check {
                    if let Some(fs_path_to_check) = fs.canonicalize(&path_to_check.path).await.ok()
                        && let Some(metadata) = fs.metadata(&fs_path_to_check).await.ok().flatten()
                    {
                        if open_target
                            .as_ref()
                            .map(|open_target| open_target.path().path != fs_path_to_check)
                            .unwrap_or(true)
                        {
                            path_to_check.path = fs_path_to_check;
                            return Some(OpenTarget::Path(
                                path_to_check,
                                metadata.is_dir,
                                #[cfg(any(test, feature = "test-support"))]
                                OpenTargetFoundBy::BackgroundPathResolution,
                            ));
                        }

                        break;
                    }
                }

                open_target
            })
        }
        BackgroundPathChecks::ProjectPathResolution => {
            let paths_to_check = project_paths_to_check(&potential_paths, cwd);
            cx.spawn(async move |cx| {
                for mut path_to_check in paths_to_check {
                    let path = path_to_check.path.to_string_lossy();
                    let resolve_task = project.update(cx, |project, cx| {
                        project.resolve_abs_path(path.as_ref(), cx)
                    });

                    if let Some(resolved_path) = resolve_task.await
                        && let Some(resolved_abs_path) = {
                            let is_dir = resolved_path.is_dir();
                            resolved_path
                                .into_abs_path()
                                .map(|resolved_abs_path| (resolved_abs_path, is_dir))
                        }
                    {
                        let (resolved_abs_path, is_dir) = resolved_abs_path;
                        let resolved_abs_path = PathBuf::from(resolved_abs_path);
                        if open_target
                            .as_ref()
                            .map(|open_target| open_target.path().path != resolved_abs_path)
                            .unwrap_or(true)
                        {
                            path_to_check.path = resolved_abs_path;
                            return Some(OpenTarget::Path(
                                path_to_check,
                                is_dir,
                                #[cfg(any(test, feature = "test-support"))]
                                OpenTargetFoundBy::BackgroundPathResolution,
                            ));
                        }

                        break;
                    }
                }

                open_target
            })
        }
    };

    cx.spawn(async move |cx| {
        background_resolution_task.await.or_else(|| {
            for (worktree, worktree_paths_to_check) in worktree_paths_to_check {
                if let Some(found_entry) =
                    worktree.update(cx, |worktree, _| -> Option<OpenTarget> {
                        let traversal =
                            worktree.traverse_from_path(true, true, false, RelPath::empty());
                        for entry in traversal {
                            if let Some(path_in_worktree) =
                                worktree_paths_to_check.iter().find(|path_to_check| {
                                    RelPath::new(&path_to_check.path, PathStyle::local())
                                        .is_ok_and(|path| entry.path.ends_with(&path))
                                })
                            {
                                return Some(OpenTarget::Worktree(
                                    PathWithPosition {
                                        path: worktree.absolutize(&entry.path),
                                        row: path_in_worktree.row,
                                        column: path_in_worktree.column,
                                    },
                                    entry.clone(),
                                    #[cfg(any(test, feature = "test-support"))]
                                    OpenTargetFoundBy::WorktreeScan,
                                ));
                            }
                        }
                        None
                    })
                {
                    return Some(found_entry);
                }
            }
            None
        })
    })
}

fn local_paths_to_check(
    potential_paths: &[PathWithPosition],
    cwd: Option<&Path>,
    worktree_candidates: &[Entity<Worktree>],
    cx: &App,
) -> Vec<PathWithPosition> {
    cwd.iter()
        .flat_map(|cwd| {
            potential_paths.iter().filter_map(|path_to_check| {
                path_to_check.path.is_relative().then(|| PathWithPosition {
                    path: cwd.join(&path_to_check.path),
                    row: path_to_check.row,
                    column: path_to_check.column,
                })
            })
        })
        .chain(potential_paths.iter().flat_map(|path_to_check| {
            let mut paths_to_check = Vec::new();
            let maybe_path = &path_to_check.path;
            if maybe_path.starts_with("~") {
                if let Some(home_path) =
                    maybe_path
                        .strip_prefix("~")
                        .ok()
                        .and_then(|stripped_maybe_path| {
                            Some(dirs::home_dir()?.join(stripped_maybe_path))
                        })
                {
                    paths_to_check.push(PathWithPosition {
                        path: home_path,
                        row: path_to_check.row,
                        column: path_to_check.column,
                    });
                }
            } else {
                paths_to_check.push(PathWithPosition {
                    path: maybe_path.clone(),
                    row: path_to_check.row,
                    column: path_to_check.column,
                });
                if maybe_path.is_relative() {
                    for worktree in worktree_candidates {
                        if !worktree.read(cx).is_single_file() {
                            paths_to_check.push(PathWithPosition {
                                path: worktree.read(cx).abs_path().join(maybe_path),
                                row: path_to_check.row,
                                column: path_to_check.column,
                            });
                        }
                    }
                }
            }
            paths_to_check
        }))
        .collect()
}

fn project_paths_to_check(
    potential_paths: &[PathWithPosition],
    cwd: Option<&Path>,
) -> Vec<PathWithPosition> {
    cwd.iter()
        .flat_map(|cwd| {
            potential_paths
                .iter()
                .filter_map(|path_to_check| normalize_absolute_candidate(cwd, path_to_check))
        })
        .chain(potential_paths.iter().filter_map(|path_to_check| {
            let maybe_path = &path_to_check.path;
            (maybe_path.starts_with("~") || maybe_path.is_absolute()).then(|| path_to_check.clone())
        }))
        .collect()
}

fn normalize_absolute_candidate(
    cwd: &Path,
    path_to_check: &PathWithPosition,
) -> Option<PathWithPosition> {
    path_to_check.path.is_relative().then(|| {
        normalize_lexically(&cwd.join(&path_to_check.path))
            .ok()
            .map(|path| PathWithPosition {
                path,
                row: path_to_check.row,
                column: path_to_check.column,
            })
    })?
}
