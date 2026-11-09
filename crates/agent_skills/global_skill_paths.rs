//! Decides which paths count as part of the global skills directory
//! (`~/.agents/skills`), and performs file operations on them. This runs on
//! the machine the skills are on: locally, or on the remote server for a
//! remote project.

use crate::{SKILL_FILE_NAME, global_skills_dir};
use anyhow::{Context as _, Result};
use fs::Fs;
use std::path::{Component, Path, PathBuf};
use util::paths::normalize_path;

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
pub async fn canonicalize_with_ancestors(path: &Path, fs: &dyn Fs) -> Option<PathBuf> {
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

/// Returns the canonicalized global agent skills directory
/// (`~/.agents/skills`).
///
/// Recomputed on every call rather than cached: the underlying
/// `canonicalize_with_ancestors` is a few `stat` syscalls (which the OS
/// page cache already handles), and a process-wide cache would either go
/// stale if the user moved `~/.agents/skills`, or pollute across tests
/// using different `FakeFs` instances.
pub async fn canonical_global_skills_dir(fs: &dyn Fs) -> Option<PathBuf> {
    canonicalize_with_ancestors(&global_skills_dir(), fs).await
}

/// If `path` names `~/.agents/skills` or one of its descendants, return the
/// canonicalized absolute path. Returns `None` for any path that resolves
/// outside the global skills tree, for relative paths that don't start with
/// `~`, or if the skills directory itself can't be canonicalized (fail closed
/// — better to refuse access than to compare against a non-canonical path).
///
/// This is the gate that lets `read_file` / `list_directory` reach into the
/// global skills directory — which lives outside any worktree — without
/// also opening up arbitrary external paths.
pub async fn resolve_global_skill_path(path: &Path, fs: &dyn Fs) -> Option<PathBuf> {
    let normalized_path = resolve_lexical_global_skill_path(path)?;

    // Canonicalize both sides so symlinks can't sneak the path out of the
    // skills tree (and so different but equivalent path representations
    // match). The lexical check above intentionally runs first, so a
    // symlinked `~/.agents/skills` root can't broaden the allowlist to every
    // path under the symlink target. A linked immediate skill directory is
    // allowed separately, but only for paths that stay under that skill target.
    let canonical_path = fs.canonicalize(&normalized_path).await.ok()?;
    let canonical_skills_dir = canonical_global_skills_dir(fs).await?;

    if canonical_path.starts_with(&canonical_skills_dir)
        || is_in_linked_global_skill_dir(
            &normalized_path,
            &canonical_path,
            &canonical_skills_dir,
            fs,
        )
        .await
    {
        Some(canonical_path)
    } else {
        None
    }
}

async fn is_in_linked_global_skill_dir(
    path: &Path,
    canonical_path: &Path,
    canonical_skills_dir: &Path,
    fs: &dyn Fs,
) -> bool {
    let skills_dir = normalize_path(&global_skills_dir());
    let Ok(relative_path) = path.strip_prefix(&skills_dir) else {
        return false;
    };
    let Some(Component::Normal(skill_dir_name)) = relative_path.components().next() else {
        return false;
    };

    let skill_dir = skills_dir.join(skill_dir_name);
    let Ok(canonical_skill_dir) = fs.canonicalize(&skill_dir).await else {
        return false;
    };

    !canonical_skill_dir.starts_with(canonical_skills_dir)
        && canonical_path.starts_with(&canonical_skill_dir)
        && fs.is_file(&skill_dir.join(SKILL_FILE_NAME)).await
}

fn expand_home_prefix(path: &Path) -> Option<PathBuf> {
    if path.is_absolute() {
        return Some(path.to_path_buf());
    }

    let mut components = path.components();
    let first_component = components.next()?;
    if !matches!(first_component, Component::Normal(component) if component == "~") {
        return None;
    }

    let mut expanded = paths::home_dir().clone();
    for component in components {
        match component {
            Component::Normal(component) => expanded.push(component),
            Component::CurDir => {}
            Component::ParentDir => expanded.push(".."),
            Component::Prefix(_) | Component::RootDir => return None,
        }
    }
    Some(expanded)
}

fn expand_and_normalize_absolute_path(path: &Path) -> Option<PathBuf> {
    let expanded_path = expand_home_prefix(path)?;
    let normalized_path = normalize_path(&expanded_path);
    normalized_path.is_absolute().then_some(normalized_path)
}

fn resolve_lexical_global_skill_path(path: &Path) -> Option<PathBuf> {
    let normalized_path = expand_and_normalize_absolute_path(path)?;
    let normalized_skills_dir = normalize_path(&global_skills_dir());

    normalized_path
        .starts_with(&normalized_skills_dir)
        .then_some(normalized_path)
}

/// If `path` names `~/.agents/skills` or one of its descendants, return a
/// canonical absolute path for it. Unlike [`resolve_global_skill_path`], the
/// target path may or may not exist on disk yet — the caller decides whether
/// to read, write, or create it. Returns `None` for any other path, including
/// siblings of the global skills tree or paths that would escape it with `..`
/// or symlinks.
pub async fn resolve_creatable_global_skill_path(path: &Path, fs: &dyn Fs) -> Option<PathBuf> {
    let normalized_path = resolve_lexical_global_skill_path(path)?;
    let canonical_path = canonicalize_with_ancestors(&normalized_path, fs).await?;
    let canonical_skills_dir = canonical_global_skills_dir(fs).await?;

    if canonical_path.starts_with(&canonical_skills_dir) {
        Some(canonical_path)
    } else {
        None
    }
}

/// A path [`locate_global_skill_path`] found to be part of the global skills
/// directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlobalSkillPath {
    /// The canonical absolute path.
    pub path: PathBuf,
    /// Whether this is `~/.agents/skills` itself, which destructive tools
    /// refuse to act on.
    pub is_skills_dir: bool,
    /// Whether the path is in a skill directory that links elsewhere, so on
    /// disk it's outside `~/.agents/skills`. Tools read these, but don't
    /// copy, move, or delete them.
    pub is_linked: bool,
}

impl GlobalSkillPath {
    /// The path, if it's strictly inside `~/.agents/skills` on disk.
    pub fn into_descendant(self) -> Option<PathBuf> {
        (!self.is_skills_dir && !self.is_linked).then_some(self.path)
    }
}

/// Resolves `path` with [`resolve_creatable_global_skill_path`] if it
/// `may_not_exist`, or with [`resolve_global_skill_path`] otherwise.
pub async fn locate_global_skill_path(
    path: &Path,
    fs: &dyn Fs,
    may_not_exist: bool,
) -> Option<GlobalSkillPath> {
    let path = if may_not_exist {
        resolve_creatable_global_skill_path(path, fs).await?
    } else {
        resolve_global_skill_path(path, fs).await?
    };
    let canonical_skills_dir = canonical_global_skills_dir(fs).await?;
    Some(GlobalSkillPath {
        is_skills_dir: path == canonical_skills_dir,
        is_linked: !path.starts_with(&canonical_skills_dir),
        path,
    })
}

/// A change an agent tool makes to files in the global skills directory.
///
/// A path here is one the agent named. It is resolved again where the
/// operation is applied, so nothing outside the skills directory is created
/// or deleted. A copy or move can have its other end outside the skills
/// directory (in the project, say), and that end must be an absolute path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GlobalSkillFileOperation {
    CreateDirectory {
        path: PathBuf,
    },
    Delete {
        path: PathBuf,
    },
    Copy {
        source: PathBuf,
        destination: PathBuf,
    },
    Move {
        source: PathBuf,
        destination: PathBuf,
    },
}

pub async fn apply_global_skill_file_operation(
    fs: &dyn Fs,
    operation: GlobalSkillFileOperation,
) -> Result<()> {
    match operation {
        GlobalSkillFileOperation::CreateDirectory { path } => {
            let path = locate_global_skill_path(&path, fs, true)
                .await
                .with_context(|| not_a_global_skill_path(&path))?
                .path;
            fs.create_dir(&path).await
        }
        GlobalSkillFileOperation::Delete { path } => {
            let path = locate_skill_descendant(&path, fs, false)
                .await
                .with_context(|| not_a_global_skill_path(&path))?;
            let metadata = fs.metadata(&path).await?.context("path not found")?;
            if metadata.is_dir {
                fs.remove_dir(
                    &path,
                    fs::RemoveOptions {
                        recursive: true,
                        ..fs::RemoveOptions::default()
                    },
                )
                .await
            } else {
                fs.remove_file(&path, fs::RemoveOptions::default()).await
            }
        }
        GlobalSkillFileOperation::Copy {
            source,
            destination,
        } => {
            let (source, destination) = locate_copy_or_move(&source, &destination, fs).await?;
            fs::copy_recursive(fs, &source, &destination, fs::CopyOptions::default()).await
        }
        GlobalSkillFileOperation::Move {
            source,
            destination,
        } => {
            let (source, destination) = locate_copy_or_move(&source, &destination, fs).await?;
            fs.rename(
                &source,
                &destination,
                fs::RenameOptions {
                    create_parents: true,
                    ..fs::RenameOptions::default()
                },
            )
            .await
        }
    }
}

fn not_a_global_skill_path(path: &Path) -> String {
    format!("{} is not inside a global skill", path.display())
}

/// Like [`locate_global_skill_path`], but only for a path strictly inside
/// `~/.agents/skills` on disk.
async fn locate_skill_descendant(path: &Path, fs: &dyn Fs, may_not_exist: bool) -> Option<PathBuf> {
    locate_global_skill_path(path, fs, may_not_exist)
        .await?
        .into_descendant()
}

async fn locate_copy_or_move(
    source: &Path,
    destination: &Path,
    fs: &dyn Fs,
) -> Result<(PathBuf, PathBuf)> {
    let skill_source = locate_skill_descendant(source, fs, false).await;
    let skill_destination = locate_skill_descendant(destination, fs, true).await;
    anyhow::ensure!(
        skill_source.is_some() || skill_destination.is_some(),
        "neither {} nor {} is inside a global skill",
        source.display(),
        destination.display()
    );
    let outside_path = |path: &Path| {
        anyhow::ensure!(
            path.is_absolute(),
            "{} is not an absolute path",
            path.display()
        );
        Ok(path.to_path_buf())
    };
    Ok((
        match skill_source {
            Some(source) => source,
            None => outside_path(source)?,
        },
        match skill_destination {
            Some(destination) => destination,
            None => outside_path(destination)?,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;
    use util::path;

    #[gpui::test]
    async fn test_resolve_creatable_global_skill_path_allows_tilde_path(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let input_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("my-skill");
        let expected_path = global_skills_dir().join("my-skill");

        let resolved = resolve_creatable_global_skill_path(&input_path, fs.as_ref())
            .await
            .expect("global skill path should resolve");

        assert_eq!(resolved, expected_path);
    }

    #[gpui::test]
    async fn test_resolve_global_skill_path_allows_tilde_path(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let skill_file = global_skills_dir().join("my-skill").join("SKILL.md");
        fs.insert_tree(
            skill_file
                .parent()
                .expect("skill file should have a parent"),
            json!({ "SKILL.md": "---\nname: my-skill\ndescription: test\n---" }),
        )
        .await;

        let input_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("my-skill")
            .join("SKILL.md");
        let resolved = resolve_global_skill_path(&input_path, fs.as_ref())
            .await
            .expect("global skill file should resolve");

        assert_eq!(resolved, skill_file);
    }

    #[gpui::test]
    async fn test_resolve_global_skill_path_allows_symlinked_skill_dir(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = global_skills_dir();
        fs.insert_tree(
            path!("/external/my-skill"),
            json!({
                "SKILL.md": "---\nname: my-skill\ndescription: test\n---",
                "references": { "guide.md": "details" }
            }),
        )
        .await;
        fs.create_dir(&skills_dir)
            .await
            .expect("global skills directory should be created");
        fs.create_symlink(
            &skills_dir.join("my-skill"),
            PathBuf::from(path!("/external/my-skill")),
        )
        .await
        .expect("skill directory should be symlinked");

        let input_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("my-skill")
            .join("references")
            .join("guide.md");
        let resolved = resolve_global_skill_path(&input_path, fs.as_ref())
            .await
            .expect("symlinked global skill resource should resolve");

        assert_eq!(
            resolved,
            PathBuf::from(path!("/external/my-skill/references/guide.md"))
        );
    }

    #[gpui::test]
    async fn test_resolve_global_skill_path_rejects_escape_from_symlinked_skill_dir(
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = global_skills_dir();
        fs.insert_tree(
            path!("/external/my-skill"),
            json!({
                "SKILL.md": "---\nname: my-skill\ndescription: test\n---",
            }),
        )
        .await;
        fs.insert_tree(path!("/private"), json!({ "secret.txt": "secret" }))
            .await;
        fs.create_symlink(
            &PathBuf::from(path!("/external/my-skill/secret")),
            PathBuf::from(path!("/private")),
        )
        .await
        .expect("nested symlink should be created");
        fs.create_dir(&skills_dir)
            .await
            .expect("global skills directory should be created");
        fs.create_symlink(
            &skills_dir.join("my-skill"),
            PathBuf::from(path!("/external/my-skill")),
        )
        .await
        .expect("skill directory should be symlinked");

        let input_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("my-skill")
            .join("secret")
            .join("secret.txt");

        assert!(
            resolve_global_skill_path(&input_path, fs.as_ref())
                .await
                .is_none(),
            "nested symlinks inside a symlinked skill must not broaden global skill access",
        );
    }

    #[gpui::test]
    async fn test_resolve_creatable_global_skill_path_rejects_other_home_paths(
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        let sibling_path = PathBuf::from("~").join(".agents").join("not-skills");
        let escaped_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("..")
            .join("not-skills");

        assert!(
            resolve_creatable_global_skill_path(&sibling_path, fs.as_ref())
                .await
                .is_none()
        );
        assert!(
            resolve_creatable_global_skill_path(&escaped_path, fs.as_ref())
                .await
                .is_none()
        );
    }

    #[gpui::test]
    async fn test_resolve_creatable_global_skill_path_rejects_symlink_escape(
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = global_skills_dir();
        fs.create_dir(&skills_dir)
            .await
            .expect("global skills directory should be created");
        fs.create_dir(path!("/external").as_ref())
            .await
            .expect("external directory should be created");
        fs.create_symlink(&skills_dir.join("link"), PathBuf::from(path!("/external")))
            .await
            .expect("symlink should be created");

        let escaped_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("link")
            .join("new-dir");

        assert!(
            resolve_creatable_global_skill_path(&escaped_path, fs.as_ref())
                .await
                .is_none()
        );
    }

    #[gpui::test]
    async fn test_global_skill_path_resolvers_reject_absolute_paths_when_skills_dir_is_symlink_to_root(
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(paths::home_dir(), json!({ ".agents": {} }))
            .await;
        fs.insert_tree(path!("/tmp"), json!({ "outside.txt": "outside" }))
            .await;

        let skills_dir = global_skills_dir();
        fs.create_symlink(&skills_dir, PathBuf::from(path!("/")))
            .await
            .expect("global skills directory should be symlinked to root");

        let outside_path = PathBuf::from(path!("/tmp/outside.txt"));
        assert!(
            resolve_global_skill_path(&outside_path, fs.as_ref())
                .await
                .is_none(),
            "existing absolute paths outside the lexical global skills tree should not resolve",
        );
        assert!(
            resolve_creatable_global_skill_path(&outside_path, fs.as_ref())
                .await
                .is_none(),
            "creatable absolute paths outside the lexical global skills tree should not resolve",
        );

        let traversed_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("..")
            .join("outside");
        assert!(
            resolve_creatable_global_skill_path(&traversed_path, fs.as_ref())
                .await
                .is_none(),
            "paths that normalize outside the lexical global skills tree should not resolve",
        );
    }

    #[gpui::test]
    async fn test_global_skill_path_resolvers_reject_absolute_paths_when_skills_dir_is_symlink_to_home(
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            paths::home_dir(),
            json!({
                ".agents": {},
                "outside.txt": "outside",
            }),
        )
        .await;

        let skills_dir = global_skills_dir();
        fs.create_symlink(&skills_dir, paths::home_dir().clone())
            .await
            .expect("global skills directory should be symlinked to home");

        let outside_path = paths::home_dir().join("outside.txt");
        assert!(
            resolve_global_skill_path(&outside_path, fs.as_ref())
                .await
                .is_none(),
            "existing absolute paths outside the lexical global skills tree should not resolve",
        );
        assert!(
            resolve_creatable_global_skill_path(&outside_path, fs.as_ref())
                .await
                .is_none(),
            "creatable absolute paths outside the lexical global skills tree should not resolve",
        );
    }

    #[gpui::test]
    async fn test_file_operations_leave_linked_skill_dirs_alone(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = global_skills_dir();
        fs.insert_tree(&skills_dir, json!({ "real-skill": { "SKILL.md": "real" } }))
            .await;
        fs.insert_tree(
            path!("/elsewhere/linked-skill"),
            json!({ "SKILL.md": "linked" }),
        )
        .await;
        fs.create_symlink(
            &skills_dir.join("linked-skill"),
            PathBuf::from(path!("/elsewhere/linked-skill")),
        )
        .await
        .unwrap();

        let linked_skill_file = PathBuf::from("~/.agents/skills/linked-skill/SKILL.md");
        let located = locate_global_skill_path(&linked_skill_file, fs.as_ref(), false)
            .await
            .expect("a linked skill's files are readable");
        assert!(located.is_linked);

        for path in [
            "~/.agents/skills/linked-skill",
            "~/.agents/skills/linked-skill/SKILL.md",
            "~/.agents/skills",
        ] {
            apply_global_skill_file_operation(
                fs.as_ref(),
                GlobalSkillFileOperation::Delete {
                    path: PathBuf::from(path),
                },
            )
            .await
            .expect_err(path);
        }
        assert!(
            fs.is_file(path!("/elsewhere/linked-skill/SKILL.md").as_ref())
                .await
        );

        apply_global_skill_file_operation(
            fs.as_ref(),
            GlobalSkillFileOperation::Delete {
                path: PathBuf::from("~/.agents/skills/real-skill"),
            },
        )
        .await
        .unwrap();
        assert!(!fs.is_dir(&skills_dir.join("real-skill")).await);
    }
}
