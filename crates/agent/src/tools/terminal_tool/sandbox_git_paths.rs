use fs::Fs;
use gpui::App;
use project::Project;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub(crate) struct SandboxGitPathCandidates {
    pub(crate) writable_paths: Vec<PathBuf>,
    pub(crate) git_paths: Vec<PathBuf>,
    repositories: Vec<SandboxGitRepositoryPaths>,
}

struct SandboxGitRepositoryPaths {
    work_directory_abs_path: PathBuf,
    dot_git_abs_path: PathBuf,
    repository_dir_abs_path: PathBuf,
    common_dir_abs_path: PathBuf,
}

pub(crate) struct SandboxGitPaths {
    pub(crate) writable_paths: Vec<PathBuf>,
    pub(crate) git_dirs: Vec<PathBuf>,
    pub(crate) allow_git_access: bool,
}

impl SandboxGitPathCandidates {
    pub(crate) fn cache_key_repositories(&self) -> Vec<(PathBuf, PathBuf, PathBuf, PathBuf)> {
        let mut repositories = self
            .repositories
            .iter()
            .map(|repository| {
                (
                    repository.work_directory_abs_path.clone(),
                    repository.dot_git_abs_path.clone(),
                    repository.repository_dir_abs_path.clone(),
                    repository.common_dir_abs_path.clone(),
                )
            })
            .collect::<Vec<_>>();
        repositories.sort();
        repositories
    }

    pub(crate) fn from_project(project: &Project, cx: &App) -> Self {
        let mut candidates = Self::default();

        for worktree in project.worktrees(cx) {
            let worktree = worktree.read(cx);
            let worktree_abs_path = worktree.abs_path();
            candidates
                .writable_paths
                .push(worktree_abs_path.to_path_buf());
            // Protect `<worktree>/.git` even when it doesn't exist yet, so a command
            // can't `git init` and then write to the freshly created metadata.
            candidates.git_paths.push(worktree_abs_path.join(".git"));

            // `Worktree` derefs to `Snapshot`; read the field directly instead of
            // cloning the whole snapshot just for this path.
            if let Some(root_repo_common_dir) = worktree.root_repo_common_dir() {
                candidates
                    .git_paths
                    .push(root_repo_common_dir.to_path_buf());
            }
        }

        // `Repository` derefs to `RepositorySnapshot`, so read the few path fields
        // directly rather than cloning the entire snapshot (which carries the
        // per-path status tree) for each repository.
        for repository in project.git_store().read(cx).repositories().values() {
            let repository = repository.read(cx);
            let repository_paths = SandboxGitRepositoryPaths {
                work_directory_abs_path: repository.work_directory_abs_path.to_path_buf(),
                dot_git_abs_path: repository.dot_git_abs_path.to_path_buf(),
                repository_dir_abs_path: repository.repository_dir_abs_path.to_path_buf(),
                common_dir_abs_path: repository.common_dir_abs_path.to_path_buf(),
            };
            candidates
                .git_paths
                .push(repository_paths.dot_git_abs_path.clone());
            candidates
                .git_paths
                .push(repository_paths.repository_dir_abs_path.clone());
            candidates
                .git_paths
                .push(repository_paths.common_dir_abs_path.clone());
            candidates.repositories.push(repository_paths);
        }

        candidates.git_paths.sort();
        candidates.git_paths.dedup();
        candidates.writable_paths.sort();
        candidates.writable_paths.dedup();

        candidates
    }
}

pub(crate) async fn sandbox_git_paths(
    candidates: SandboxGitPathCandidates,
    fs: &dyn Fs,
    allow_git_access: bool,
) -> SandboxGitPaths {
    let mut writable_paths = candidates.writable_paths;
    let mut git_dirs = candidates.git_paths;

    let mut allow_verified_git_access = false;
    if allow_git_access {
        let mut verified_git_paths = Vec::new();
        for repository in candidates.repositories {
            verified_git_paths.extend(verified_sandbox_git_paths(repository, fs).await);
        }
        verified_git_paths.sort();
        verified_git_paths.dedup();

        // A Git path inside a writable worktree root is already writable, so
        // granting it can never escalate access beyond the project. A non-repo
        // `.git` placeholder there (a plain folder opened alongside a repo, or a
        // not-yet-initialized repo) would never appear in `verified_git_paths`,
        // so requiring it to verify would wrongly deny the whole grant. Only
        // paths that fall *outside* every writable root can leak access to
        // unrelated metadata, so those are the only ones that must verify.
        let mut all_external_git_paths_verified = true;
        for path in &git_dirs {
            if path_is_within_any(path, &writable_paths) {
                continue;
            }
            let Some(normalized_path) = normalize_sandbox_git_path(path, fs).await else {
                log::warn!(
                    "Denying requested agent terminal Git metadata access because external Git metadata path `{}` could not be normalized",
                    path.display()
                );
                all_external_git_paths_verified = false;
                break;
            };
            if verified_git_paths.binary_search(&normalized_path).is_err() {
                log::warn!(
                    "Denying requested agent terminal Git metadata access because external Git metadata path `{}` (normalized to `{}`) was not verified from project repository metadata",
                    path.display(),
                    normalized_path.display()
                );
                all_external_git_paths_verified = false;
                break;
            }
        }

        // The current sandbox policy can make one Git directory set either all
        // writable or all protected. Only grant Git access when every external
        // candidate still verifies; otherwise keep protecting the original
        // candidate set. The granted set is the verified paths only, so even
        // when the grant proceeds, unverified `.git` metadata never becomes
        // writable.
        if all_external_git_paths_verified {
            git_dirs = verified_git_paths;
            allow_verified_git_access = true;
        }
    }

    git_dirs.sort();
    git_dirs.dedup();
    writable_paths.sort();
    writable_paths.dedup();

    SandboxGitPaths {
        writable_paths,
        git_dirs,
        allow_git_access: allow_verified_git_access,
    }
}

async fn verified_sandbox_git_paths(
    repository: SandboxGitRepositoryPaths,
    fs: &dyn Fs,
) -> Vec<PathBuf> {
    macro_rules! deny {
        ($($arg:tt)*) => {{
            log::debug!(
                "Denying agent terminal Git metadata access for repository `{}` (dot_git: `{}`, repository_dir: `{}`, common_dir: `{}`): {}",
                repository.work_directory_abs_path.display(),
                repository.dot_git_abs_path.display(),
                repository.repository_dir_abs_path.display(),
                repository.common_dir_abs_path.display(),
                format_args!($($arg)*)
            );
            return Vec::new();
        }};
    }

    let Some(dot_git_abs_path) = normalize_sandbox_git_path(&repository.dot_git_abs_path, fs).await
    else {
        deny!(
            "could not normalize .git path `{}`",
            repository.dot_git_abs_path.display()
        );
    };
    let Some(repository_dir_abs_path) =
        normalize_sandbox_git_path(&repository.repository_dir_abs_path, fs).await
    else {
        deny!(
            "could not normalize repository dir `{}`",
            repository.repository_dir_abs_path.display()
        );
    };
    let Some(common_dir_abs_path) =
        normalize_sandbox_git_path(&repository.common_dir_abs_path, fs).await
    else {
        deny!(
            "could not normalize common dir `{}`",
            repository.common_dir_abs_path.display()
        );
    };

    let dot_git_metadata = match fs.metadata(&repository.dot_git_abs_path).await {
        Ok(Some(metadata)) => metadata,
        Ok(None) => deny!(
            ".git path `{}` does not exist",
            repository.dot_git_abs_path.display()
        ),
        Err(error) => deny!(
            "failed to read metadata for .git path `{}`: {error}",
            repository.dot_git_abs_path.display()
        ),
    };
    if dot_git_metadata.is_symlink {
        deny!(
            ".git path `{}` is a symlink",
            repository.dot_git_abs_path.display()
        );
    }

    if dot_git_metadata.is_dir {
        if dot_git_abs_path != repository_dir_abs_path {
            deny!(
                "directory .git path `{}` normalized to `{}`, which does not match repository dir `{}` normalized to `{}`",
                repository.dot_git_abs_path.display(),
                dot_git_abs_path.display(),
                repository.repository_dir_abs_path.display(),
                repository_dir_abs_path.display()
            );
        }

        if repository_dir_abs_path == common_dir_abs_path {
            return vec![
                dot_git_abs_path,
                repository_dir_abs_path,
                common_dir_abs_path,
            ];
        }

        let Some(common_dir) = read_commondir_path(&repository_dir_abs_path, fs).await else {
            deny!(
                "repository dir `{}` did not contain a readable commondir pointing at expected common dir `{}`",
                repository_dir_abs_path.display(),
                common_dir_abs_path.display()
            );
        };
        if common_dir == common_dir_abs_path {
            return vec![
                dot_git_abs_path,
                repository_dir_abs_path,
                common_dir_abs_path,
            ];
        }
        deny!(
            "repository dir `{}` commondir resolved to `{}`, expected `{}`",
            repository_dir_abs_path.display(),
            common_dir.display(),
            common_dir_abs_path.display()
        );
    }

    let Some(expected_dot_git_abs_path) =
        normalize_sandbox_git_path(repository.work_directory_abs_path.join(".git"), fs).await
    else {
        deny!(
            "could not normalize expected worktree .git path `{}`",
            repository.work_directory_abs_path.join(".git").display()
        );
    };
    if dot_git_abs_path != expected_dot_git_abs_path {
        deny!(
            ".git path `{}` normalized to `{}`, expected worktree .git path `{}`",
            repository.dot_git_abs_path.display(),
            dot_git_abs_path.display(),
            expected_dot_git_abs_path.display()
        );
    }

    let Some(stated_repository_dir) = read_gitfile_path(&repository.dot_git_abs_path, fs).await
    else {
        deny!(
            "gitfile `{}` did not resolve to a readable, non-symlink repository dir",
            repository.dot_git_abs_path.display()
        );
    };

    if stated_repository_dir != repository_dir_abs_path {
        deny!(
            "gitfile `{}` resolved to repository dir `{}`, expected `{}`",
            repository.dot_git_abs_path.display(),
            stated_repository_dir.display(),
            repository_dir_abs_path.display()
        );
    }

    let Some(common_dir) = read_commondir_path(&stated_repository_dir, fs).await else {
        if repository_dir_abs_path == common_dir_abs_path
            && gitdir_belongs_to_submodule_worktree(
                &repository_dir_abs_path,
                &repository.work_directory_abs_path,
                fs,
            )
            .await
        {
            return vec![dot_git_abs_path, repository_dir_abs_path];
        }
        deny!(
            "repository dir `{}` has no verified commondir and did not verify as a submodule gitdir for worktree `{}`",
            repository_dir_abs_path.display(),
            repository.work_directory_abs_path.display()
        );
    };

    if common_dir != common_dir_abs_path {
        deny!(
            "repository dir `{}` commondir resolved to `{}`, expected `{}`",
            stated_repository_dir.display(),
            common_dir.display(),
            common_dir_abs_path.display()
        );
    }

    if repository_dir_abs_path != common_dir_abs_path
        && !linked_worktree_points_back(
            &common_dir_abs_path,
            &repository_dir_abs_path,
            &dot_git_abs_path,
            &repository.work_directory_abs_path,
            fs,
        )
        .await
    {
        deny!(
            "linked worktree repository dir `{}` did not point back to .git path `{}` and worktree `{}` under common dir `{}`",
            repository_dir_abs_path.display(),
            dot_git_abs_path.display(),
            repository.work_directory_abs_path.display(),
            common_dir_abs_path.display()
        );
    }

    vec![
        dot_git_abs_path,
        repository_dir_abs_path,
        common_dir_abs_path,
    ]
}

async fn read_gitfile_path(dot_git_abs_path: &Path, fs: &dyn Fs) -> Option<PathBuf> {
    let contents = match fs.load(dot_git_abs_path).await {
        Ok(contents) => contents,
        Err(error) => {
            log::debug!(
                "Could not verify Git metadata path: failed to read gitfile `{}`: {error}",
                dot_git_abs_path.display()
            );
            return None;
        }
    };
    let Some(gitdir) = contents.strip_prefix("gitdir:") else {
        log::debug!(
            "Could not verify Git metadata path: gitfile `{}` does not start with `gitdir:`",
            dot_git_abs_path.display()
        );
        return None;
    };
    let gitdir = Path::new(gitdir.trim());
    let Some(dot_git_parent) = dot_git_abs_path.parent() else {
        log::debug!(
            "Could not verify Git metadata path: gitfile `{}` has no parent directory",
            dot_git_abs_path.display()
        );
        return None;
    };
    let path = if gitdir.is_absolute() {
        gitdir.to_path_buf()
    } else {
        dot_git_parent.join(gitdir)
    };
    match fs.metadata(&path).await {
        Ok(Some(metadata)) if metadata.is_symlink => {
            log::debug!(
                "Could not verify Git metadata path: gitfile `{}` points to symlinked gitdir `{}`",
                dot_git_abs_path.display(),
                path.display()
            );
            return None;
        }
        Ok(_) => {}
        Err(error) => {
            log::debug!(
                "Could not check whether gitfile `{}` points to a symlink at `{}`: {error}",
                dot_git_abs_path.display(),
                path.display()
            );
        }
    }
    let normalized_path = normalize_sandbox_git_path(&path, fs).await;
    if normalized_path.is_none() {
        log::debug!(
            "Could not verify Git metadata path: gitfile `{}` points to gitdir `{}` that could not be normalized",
            dot_git_abs_path.display(),
            path.display()
        );
    }
    normalized_path
}

async fn read_commondir_path(repository_dir_abs_path: &Path, fs: &dyn Fs) -> Option<PathBuf> {
    let commondir_abs_path = repository_dir_abs_path.join("commondir");
    let commondir_contents = match fs.load(&commondir_abs_path).await {
        Ok(contents) => contents,
        Err(error) => {
            log::debug!(
                "Could not verify Git metadata path: failed to read commondir file `{}`: {error}",
                commondir_abs_path.display()
            );
            return None;
        }
    };
    let commondir_path = Path::new(commondir_contents.trim());
    let path = if commondir_path.is_absolute() {
        commondir_path.to_path_buf()
    } else {
        repository_dir_abs_path.join(commondir_path)
    };
    let normalized_path = normalize_sandbox_git_path(&path, fs).await;
    if normalized_path.is_none() {
        log::debug!(
            "Could not verify Git metadata path: commondir file `{}` points to `{}` which could not be normalized",
            commondir_abs_path.display(),
            path.display()
        );
    }
    normalized_path
}

async fn linked_worktree_points_back(
    common_dir_abs_path: &Path,
    repository_dir_abs_path: &Path,
    dot_git_abs_path: &Path,
    work_directory_abs_path: &Path,
    fs: &dyn Fs,
) -> bool {
    let expected_repository_parent = common_dir_abs_path.join("worktrees");
    if repository_dir_abs_path.parent() != Some(expected_repository_parent.as_path()) {
        log::debug!(
            "Could not verify linked worktree Git metadata: repository dir `{}` is not under expected worktrees dir `{}`",
            repository_dir_abs_path.display(),
            expected_repository_parent.display()
        );
        return false;
    }

    match fs.metadata(repository_dir_abs_path).await {
        Ok(Some(metadata)) if metadata.is_dir && !metadata.is_symlink => {}
        Ok(Some(metadata)) => {
            log::debug!(
                "Could not verify linked worktree Git metadata: repository dir `{}` has invalid metadata (is_dir: {}, is_symlink: {})",
                repository_dir_abs_path.display(),
                metadata.is_dir,
                metadata.is_symlink
            );
            return false;
        }
        Ok(None) => {
            log::debug!(
                "Could not verify linked worktree Git metadata: repository dir `{}` does not exist",
                repository_dir_abs_path.display()
            );
            return false;
        }
        Err(error) => {
            log::debug!(
                "Could not verify linked worktree Git metadata: failed to read metadata for repository dir `{}`: {error}",
                repository_dir_abs_path.display()
            );
            return false;
        }
    }

    let expected_dot_git_abs_path = work_directory_abs_path.join(".git");
    let Some(expected_dot_git_abs_path) =
        normalize_sandbox_git_path(&expected_dot_git_abs_path, fs).await
    else {
        log::debug!(
            "Could not verify linked worktree Git metadata: expected .git path `{}` could not be normalized",
            expected_dot_git_abs_path.display()
        );
        return false;
    };
    if dot_git_abs_path != expected_dot_git_abs_path {
        log::debug!(
            "Could not verify linked worktree Git metadata: .git path `{}` does not match expected worktree .git path `{}`",
            dot_git_abs_path.display(),
            expected_dot_git_abs_path.display()
        );
        return false;
    }

    let Some(listed_dot_git_path) = read_listed_worktree_gitdir(repository_dir_abs_path, fs).await
    else {
        return false;
    };
    if listed_dot_git_path != dot_git_abs_path {
        log::debug!(
            "Could not verify linked worktree Git metadata: repository dir `{}` lists .git path `{}`, expected `{}`",
            repository_dir_abs_path.display(),
            listed_dot_git_path.display(),
            dot_git_abs_path.display()
        );
        return false;
    }

    true
}

async fn read_listed_worktree_gitdir(worktree_entry_dir: &Path, fs: &dyn Fs) -> Option<PathBuf> {
    let gitdir_abs_path = worktree_entry_dir.join("gitdir");
    let gitdir_contents = match fs.load(&gitdir_abs_path).await {
        Ok(contents) => contents,
        Err(error) => {
            log::debug!(
                "Could not verify linked worktree Git metadata: failed to read worktree gitdir file `{}`: {error}",
                gitdir_abs_path.display()
            );
            return None;
        }
    };
    let gitdir_path = Path::new(gitdir_contents.trim());
    let path = if gitdir_path.is_absolute() {
        gitdir_path.to_path_buf()
    } else {
        worktree_entry_dir.join(gitdir_path)
    };
    let normalized_path = normalize_sandbox_git_path(&path, fs).await;
    if normalized_path.is_none() {
        log::debug!(
            "Could not verify linked worktree Git metadata: worktree gitdir file `{}` points to `{}` which could not be normalized",
            gitdir_abs_path.display(),
            path.display()
        );
    }
    normalized_path
}

async fn gitdir_belongs_to_submodule_worktree(
    repository_dir_abs_path: &Path,
    work_directory_abs_path: &Path,
    fs: &dyn Fs,
) -> bool {
    let Some(work_directory_abs_path) =
        normalize_sandbox_git_path(work_directory_abs_path, fs).await
    else {
        log::debug!(
            "Could not verify submodule Git metadata: worktree path `{}` could not be normalized",
            work_directory_abs_path.display()
        );
        return false;
    };

    let Some(core_worktree) = read_core_worktree(repository_dir_abs_path, fs).await else {
        return false;
    };
    if core_worktree != work_directory_abs_path {
        log::debug!(
            "Could not verify submodule Git metadata: repository dir `{}` has core.worktree `{}`, expected `{}`",
            repository_dir_abs_path.display(),
            core_worktree.display(),
            work_directory_abs_path.display()
        );
        return false;
    }

    true
}

async fn read_core_worktree(repository_dir_abs_path: &Path, fs: &dyn Fs) -> Option<PathBuf> {
    let config_abs_path = repository_dir_abs_path.join("config");
    let config = match fs.load(&config_abs_path).await {
        Ok(config) => config,
        Err(error) => {
            log::debug!(
                "Could not verify submodule Git metadata: failed to read config `{}`: {error}",
                config_abs_path.display()
            );
            return None;
        }
    };
    let Some(core_worktree) = parse_core_worktree(&config) else {
        log::debug!(
            "Could not verify submodule Git metadata: config `{}` did not contain exactly one supported core.worktree value",
            config_abs_path.display()
        );
        return None;
    };
    let path = Path::new(&core_worktree);
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        repository_dir_abs_path.join(path)
    };
    let normalized_path = normalize_sandbox_git_path(&path, fs).await;
    if normalized_path.is_none() {
        log::debug!(
            "Could not verify submodule Git metadata: core.worktree value `{}` from config `{}` resolved to `{}` which could not be normalized",
            core_worktree,
            config_abs_path.display(),
            path.display()
        );
    }
    normalized_path
}

fn parse_core_worktree(config: &str) -> Option<String> {
    let mut in_core_section = false;
    let mut core_worktree = None;

    for raw_line in config.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if line.ends_with('\\') {
            return None;
        }

        if line.starts_with('[') {
            if !line.ends_with(']') {
                return None;
            }
            let section = line[1..line.len() - 1].trim();
            if section.to_lowercase().starts_with("include") {
                return None;
            }
            in_core_section = section.eq_ignore_ascii_case("core");
            continue;
        }

        if !in_core_section {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if !key.trim().eq_ignore_ascii_case("worktree") {
            continue;
        }
        if core_worktree.is_some() {
            return None;
        }
        core_worktree = Some(parse_git_config_path_value(value.trim())?);
    }

    core_worktree
}

fn parse_git_config_path_value(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }

    if !value.starts_with('"') {
        if value.contains('"') || value.starts_with('~') {
            return None;
        }
        return Some(value.to_string());
    }

    let mut chars = value.chars();
    chars.next()?;
    let mut parsed = String::new();
    let mut escaped = false;
    let mut closed = false;
    while let Some(character) = chars.next() {
        if escaped {
            match character {
                '"' | '\\' => parsed.push(character),
                _ => return None,
            }
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == '"' {
            closed = true;
            break;
        } else {
            parsed.push(character);
        }
    }

    if escaped || !closed {
        return None;
    }

    let remaining = &value[value.len() - chars.as_str().len()..];
    if !remaining.trim().is_empty() {
        return None;
    }

    if parsed.is_empty() || parsed.starts_with('~') {
        return None;
    }

    Some(parsed)
}

fn path_is_within_any(path: &Path, roots: &[PathBuf]) -> bool {
    // `Path::starts_with` matches whole components, so `/projectX` is not
    // treated as being within `/project`.
    roots.iter().any(|root| path.starts_with(root))
}

async fn normalize_sandbox_git_path(path: impl AsRef<Path>, fs: &dyn Fs) -> Option<PathBuf> {
    if let Ok(path) = fs.canonicalize(path.as_ref()).await {
        Some(path)
    } else {
        util::paths::normalize_lexically(path.as_ref()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::Fs;

    #[gpui::test]
    async fn test_sandbox_paths_protect_git_paths_until_git_access_is_allowed(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/main_repo",
            serde_json::json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;
        fs.add_linked_worktree_for_repo(
            Path::new("/main_repo/.git"),
            false,
            git::repository::Worktree {
                path: PathBuf::from("/linked_worktree"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;
        fs.write(Path::new("/linked_worktree/file.txt"), b"content")
            .await
            .expect("linked worktree file should be written");

        let project = project::Project::test(fs.clone(), [Path::new("/linked_worktree")], cx).await;
        let candidates =
            cx.update(|cx| SandboxGitPathCandidates::from_project(project.read(cx), cx));
        let paths_without_git_access = sandbox_git_paths(candidates, fs.as_ref(), false).await;

        assert!(
            paths_without_git_access
                .writable_paths
                .contains(&PathBuf::from("/linked_worktree"))
        );
        assert!(
            paths_without_git_access
                .git_dirs
                .contains(&PathBuf::from("/linked_worktree/.git"))
        );
        assert!(
            !paths_without_git_access
                .git_dirs
                .contains(&PathBuf::from("/linked_worktree/.gitignore"))
        );
        assert!(
            paths_without_git_access
                .git_dirs
                .contains(&PathBuf::from("/main_repo/.git"))
        );
        assert!(
            paths_without_git_access
                .git_dirs
                .contains(&PathBuf::from("/main_repo/.git/worktrees/feature"))
        );

        let candidates =
            cx.update(|cx| SandboxGitPathCandidates::from_project(project.read(cx), cx));
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/linked_worktree"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/linked_worktree/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/main_repo/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/main_repo/.git/worktrees/feature"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_grant_git_access_when_non_git_folder_is_present(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/repo",
            serde_json::json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;
        // A plain folder opened alongside the repo. Its `<root>/.git` placeholder
        // never corresponds to a repository, so it must not block the grant for
        // the real repo.
        fs.insert_tree("/notes", serde_json::json!({ "todo.txt": "hi" }))
            .await;

        let project =
            project::Project::test(fs.clone(), [Path::new("/repo"), Path::new("/notes")], cx).await;
        let candidates =
            cx.update(|cx| SandboxGitPathCandidates::from_project(project.read(cx), cx));
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/repo/.git"))
        );
        assert!(
            paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/notes"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_allow_submodule_gitdir_without_commondir(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/super",
            serde_json::json!({
                ".git": {
                    "modules": {
                        "sub": {
                            "HEAD": "ref: refs/heads/main",
                            "config": "[core]\n\trepositoryformatversion = 0\n\tworktree = ../../../sub\n"
                        }
                    }
                },
                "sub": {
                    ".git": "gitdir: ../.git/modules/sub",
                    "file.txt": "content"
                }
            }),
        )
        .await;

        let project = project::Project::test(fs.clone(), [Path::new("/super/sub")], cx).await;
        let candidates =
            cx.update(|cx| SandboxGitPathCandidates::from_project(project.read(cx), cx));
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/super/sub"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/super/sub/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/super/.git/modules/sub"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_do_not_grant_submodule_gitdir_without_back_reference(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/super",
            serde_json::json!({
                ".git": {
                    "modules": {
                        "sub": {
                            "HEAD": "ref: refs/heads/main",
                            "config": "[core]\n\trepositoryformatversion = 0\n"
                        }
                    }
                },
                "sub": {
                    ".git": "gitdir: ../.git/modules/sub",
                    "file.txt": "content"
                }
            }),
        )
        .await;

        let project = project::Project::test(fs.clone(), [Path::new("/super/sub")], cx).await;
        let candidates =
            cx.update(|cx| SandboxGitPathCandidates::from_project(project.read(cx), cx));
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(!paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/super/sub/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/super/.git/modules/sub"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/super/.git/modules/sub"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_do_not_grant_submodule_gitfile_to_unrelated_gitdir(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            serde_json::json!({
                "sub": {
                    ".git": "gitdir: /other_repo/.git",
                    "file.txt": "content"
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/other_repo",
            serde_json::json!({
                ".git": {
                    "HEAD": "ref: refs/heads/main",
                    "config": "[core]\n\trepositoryformatversion = 0\n"
                }
            }),
        )
        .await;

        let project = project::Project::test(fs.clone(), [Path::new("/project/sub")], cx).await;
        let candidates =
            cx.update(|cx| SandboxGitPathCandidates::from_project(project.read(cx), cx));
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(!paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/project/sub/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/other_repo/.git"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/other_repo/.git"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_do_not_follow_gitfile_changed_after_scan(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/main_repo",
            serde_json::json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;
        fs.add_linked_worktree_for_repo(
            Path::new("/main_repo/.git"),
            false,
            git::repository::Worktree {
                path: PathBuf::from("/linked_worktree"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;
        fs.write(Path::new("/linked_worktree/file.txt"), b"content")
            .await
            .expect("linked worktree file should be written");
        fs.insert_tree(
            "/other_repo",
            serde_json::json!({
                ".git": {
                    "worktrees": {
                        "other": {
                            "HEAD": "ref: refs/heads/other",
                            "commondir": "/other_repo/.git",
                            "gitdir": "/other_worktree/.git"
                        }
                    }
                }
            }),
        )
        .await;

        let project = project::Project::test(fs.clone(), [Path::new("/linked_worktree")], cx).await;
        fs.write(
            Path::new("/linked_worktree/.git"),
            b"gitdir: /other_repo/.git/worktrees/other",
        )
        .await
        .expect("mutated gitfile should be written");

        let candidates =
            cx.update(|cx| SandboxGitPathCandidates::from_project(project.read(cx), cx));
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(!paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/linked_worktree/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/main_repo/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/main_repo/.git/worktrees/feature"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/other_repo/.git"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/other_repo/.git/worktrees/other"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_do_not_grant_unverified_worktree_gitdir(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/main_repo",
            serde_json::json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;
        fs.add_linked_worktree_for_repo(
            Path::new("/main_repo/.git"),
            false,
            git::repository::Worktree {
                path: PathBuf::from("/linked_worktree"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;
        fs.insert_tree(
            "/other_repo",
            serde_json::json!({
                ".git": {
                    "worktrees": {
                        "other": {
                            "HEAD": "ref: refs/heads/other",
                            "commondir": "/other_repo/.git",
                            "gitdir": "/other_worktree/.git"
                        }
                    }
                }
            }),
        )
        .await;
        fs.write(
            Path::new("/linked_worktree/.git"),
            b"gitdir: /other_repo/.git/worktrees/other",
        )
        .await
        .expect("malicious gitfile should be written");

        let project = project::Project::test(fs.clone(), [Path::new("/linked_worktree")], cx).await;
        let candidates =
            cx.update(|cx| SandboxGitPathCandidates::from_project(project.read(cx), cx));
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(!paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/linked_worktree"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/other_repo/.git"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/other_repo/.git/worktrees/other"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/linked_worktree/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/other_repo/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/other_repo/.git/worktrees/other"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_do_not_grant_symlinked_dot_git(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            serde_json::json!({
                "file.txt": "content",
            }),
        )
        .await;
        fs.insert_tree(
            "/other_repo",
            serde_json::json!({
                ".git": {}
            }),
        )
        .await;
        fs.insert_symlink(
            Path::new("/project/.git"),
            PathBuf::from("/other_repo/.git"),
        )
        .await;

        let candidates = SandboxGitPathCandidates {
            writable_paths: vec![PathBuf::from("/project")],
            git_paths: vec![
                PathBuf::from("/project/.git"),
                PathBuf::from("/other_repo/.git"),
            ],
            repositories: vec![SandboxGitRepositoryPaths {
                work_directory_abs_path: PathBuf::from("/project"),
                dot_git_abs_path: PathBuf::from("/project/.git"),
                repository_dir_abs_path: PathBuf::from("/other_repo/.git"),
                common_dir_abs_path: PathBuf::from("/other_repo/.git"),
            }],
        };
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(!paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/project/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/other_repo/.git"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/other_repo/.git"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_do_not_grant_symlinked_dot_git_file(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            serde_json::json!({
                "file.txt": "content",
            }),
        )
        .await;
        fs.insert_tree(
            "/other_repo",
            serde_json::json!({
                "gitfile": "gitdir: /other_repo/.git",
                ".git": {
                    "HEAD": "ref: refs/heads/main",
                    "config": "[core]\n\trepositoryformatversion = 0\n\tworktree = /project\n"
                }
            }),
        )
        .await;
        fs.insert_symlink(
            Path::new("/project/.git"),
            PathBuf::from("/other_repo/gitfile"),
        )
        .await;

        let candidates = SandboxGitPathCandidates {
            writable_paths: vec![PathBuf::from("/project")],
            git_paths: vec![
                PathBuf::from("/project/.git"),
                PathBuf::from("/other_repo/.git"),
            ],
            repositories: vec![SandboxGitRepositoryPaths {
                work_directory_abs_path: PathBuf::from("/project"),
                dot_git_abs_path: PathBuf::from("/project/.git"),
                repository_dir_abs_path: PathBuf::from("/other_repo/.git"),
                common_dir_abs_path: PathBuf::from("/other_repo/.git"),
            }],
        };
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(!paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/project/.git"))
        );
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/other_repo/.git"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/other_repo/.git"))
        );
    }

    #[gpui::test]
    async fn test_sandbox_paths_do_not_grant_gitfile_to_symlinked_gitdir(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            serde_json::json!({
                ".git": "gitdir: /other_repo/gitdir_link",
                "file.txt": "content",
            }),
        )
        .await;
        fs.insert_tree(
            "/other_repo",
            serde_json::json!({
                ".git": {
                    "HEAD": "ref: refs/heads/main",
                    "config": "[core]\n\trepositoryformatversion = 0\n\tworktree = /project\n"
                }
            }),
        )
        .await;
        fs.insert_symlink(
            Path::new("/other_repo/gitdir_link"),
            PathBuf::from("/other_repo/.git"),
        )
        .await;

        let candidates = SandboxGitPathCandidates {
            writable_paths: vec![PathBuf::from("/project")],
            git_paths: vec![
                PathBuf::from("/project/.git"),
                PathBuf::from("/other_repo/gitdir_link"),
                PathBuf::from("/other_repo/.git"),
            ],
            repositories: vec![SandboxGitRepositoryPaths {
                work_directory_abs_path: PathBuf::from("/project"),
                dot_git_abs_path: PathBuf::from("/project/.git"),
                repository_dir_abs_path: PathBuf::from("/other_repo/gitdir_link"),
                common_dir_abs_path: PathBuf::from("/other_repo/gitdir_link"),
            }],
        };
        let paths_with_git_access = sandbox_git_paths(candidates, fs.as_ref(), true).await;

        assert!(!paths_with_git_access.allow_git_access);
        assert!(
            paths_with_git_access
                .git_dirs
                .contains(&PathBuf::from("/other_repo/gitdir_link"))
        );
        assert!(
            !paths_with_git_access
                .writable_paths
                .contains(&PathBuf::from("/other_repo/.git"))
        );
    }

    #[test]
    fn test_parse_core_worktree_accepts_simple_and_quoted_values() {
        assert_eq!(
            parse_core_worktree("[core]\n\tworktree = ../../../sub\n"),
            Some("../../../sub".to_string())
        );
        assert_eq!(
            parse_core_worktree("[core]\n\tworktree = \"../../../sub with spaces\"\n"),
            Some("../../../sub with spaces".to_string())
        );
        assert_eq!(
            parse_core_worktree("[core]\n\tworktree = \"C:/Users/Test/project/sub\"\n"),
            Some("C:/Users/Test/project/sub".to_string())
        );
        assert_eq!(
            parse_core_worktree("[core]\n\tworktree = \"C:\\\\Users\\\\Test\\\\project\\\\sub\"\n"),
            Some("C:\\Users\\Test\\project\\sub".to_string())
        );
    }

    #[test]
    fn test_parse_core_worktree_rejects_ambiguous_or_unsupported_config() {
        assert_eq!(parse_core_worktree("[core]\n\tworktree =\n"), None);
        assert_eq!(
            parse_core_worktree("[core]\n\tworktree = ../../../sub\n\tworktree = ../../../other\n"),
            None
        );
        assert_eq!(parse_core_worktree("worktree = ../../../sub\n"), None);
        assert_eq!(
            parse_core_worktree(
                "[include]\n\tpath = ../config\n[core]\n\tworktree = ../../../sub\n"
            ),
            None
        );
        assert_eq!(
            parse_core_worktree("[core]\n\tworktree = \"../../../sub\" trailing\n"),
            None
        );
        assert_eq!(
            parse_core_worktree("[core]\n\tworktree = \"../../../sub\n"),
            None
        );
        assert_eq!(
            parse_core_worktree("[core]\n\tworktree = ../../../sub\\\n"),
            None
        );
        assert_eq!(parse_core_worktree("[core]\n\tworktree = ~/sub\n"), None);
    }
}
