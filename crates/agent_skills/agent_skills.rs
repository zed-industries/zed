use fs::Fs;
use futures::StreamExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use util::paths::component_matches_ignore_ascii_case;

/// First segment of the skills directory path: `.agents`.
pub const AGENTS_DIR_NAME: &str = ".agents";

/// Second segment of the skills directory path: `skills`.
pub const SKILLS_DIR_NAME: &str = "skills";

/// Opaque identifier for the project scope a skill was loaded from.
///
/// `agent_skills` is a leaf crate and intentionally does not depend on
/// `worktree`. Callers (e.g. the `agent` crate) construct these from
/// `worktree::WorktreeId::to_usize()` and recover the original ID via
/// `worktree::WorktreeId::from_usize()` when needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillScopeId(pub usize);

/// Cap on concurrent filesystem operations during skill discovery and loading.
/// Without this bound, a `.agents/skills` directory containing thousands of
/// entries would fan out an equally large number of concurrent OS-level I/O
/// operations, potentially exhausting file descriptors or stalling the app.
const SKILL_IO_CONCURRENCY: usize = 16;

/// The name of the skill definition file
pub const SKILL_FILE_NAME: &str = "SKILL.md";

/// A skill discovered on disk. This PR only records paths; the
/// `SKILL.md` contents (name, description, body) are parsed in a
/// follow-up PR.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Skill {
    pub source: SkillSource,
    /// Absolute path to the skill directory (e.g. `~/.agents/skills/foo`).
    pub directory_path: PathBuf,
    /// Absolute path to the `SKILL.md` file inside the skill directory.
    pub skill_file_path: PathBuf,
}

/// Indicates where a skill was loaded from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillSource {
    /// From ~/.agents/skills/
    Global,
    /// From {project}/.agents/skills/
    ProjectLocal {
        worktree_id: SkillScopeId,
        worktree_root_name: Arc<str>,
    },
}

pub async fn load_skills_from_directory(
    fs: &Arc<dyn Fs>,
    directory: &Path,
    source: SkillSource,
) -> Vec<Skill> {
    if !fs.is_dir(directory).await {
        return Vec::new();
    }

    let skill_files = find_skill_files(fs, directory).await;

    let mut skills: Vec<Skill> = skill_files
        .into_iter()
        .filter_map(|skill_file_path| {
            let directory_path = skill_file_path.parent()?.to_path_buf();
            Some(Skill {
                source: source.clone(),
                directory_path,
                skill_file_path,
            })
        })
        .collect();

    // Sort by path for deterministic ordering — `fs.read_dir` order is
    // filesystem-dependent.
    skills.sort_by(|a, b| a.skill_file_path.cmp(&b.skill_file_path));

    skills
}

/// Find every `<skills_root>/<name>/SKILL.md` directly under `directory`.
///
/// Discovery is intentionally one level deep: a skill is the immediate
/// child directory of the skills root, and `SKILL.md` is the file that
/// names it. See `crates/agent_skills/README.md` for why we don't recurse.
async fn find_skill_files(fs: &Arc<dyn Fs>, directory: &Path) -> Vec<PathBuf> {
    let Ok(mut entries) = fs.read_dir(directory).await else {
        return Vec::new();
    };

    let mut entry_paths = Vec::new();
    while let Some(entry) = entries.next().await {
        if let Ok(entry_path) = entry {
            entry_paths.push(entry_path);
        }
    }

    futures::stream::iter(entry_paths)
        .map(|entry_path| {
            let fs = fs.clone();
            async move {
                let Ok(Some(metadata)) = fs.metadata(&entry_path).await else {
                    return None;
                };
                if !metadata.is_dir {
                    return None;
                }
                let skill_file = entry_path.join(SKILL_FILE_NAME);
                fs.is_file(&skill_file).await.then_some(skill_file)
            }
        })
        .buffer_unordered(SKILL_IO_CONCURRENCY)
        .filter_map(|x| async move { x })
        .collect()
        .await
}

/// Returns the global skills directory: `~/.agents/skills`.
///
/// Other agents (e.g. Claude Code) already write skill files into this
/// location, so a Zed installation may have skills here even before the
/// rest of Zed's skills support ships.
///
/// In test builds, `paths::home_dir()` is hardcoded to a fixed path
/// (e.g. `/Users/zed`), so all tests using this function operate on the
/// same simulated home directory. Each test should use its own `FakeFs`
/// instance to keep skill setups from leaking across tests.
pub fn global_skills_dir() -> PathBuf {
    paths::home_dir()
        .join(AGENTS_DIR_NAME)
        .join(SKILLS_DIR_NAME)
}

/// Project-local skills live at this path relative to a worktree root,
/// e.g. `<worktree>/.agents/skills/<skill>/SKILL.md`.
pub fn project_skills_relative_path() -> &'static str {
    ".agents/skills"
}

/// Returns `true` if `path` looks like it points into an agent skills
/// directory — i.e. it contains `AGENTS_DIR_NAME` immediately followed by
/// `SKILLS_DIR_NAME` as two consecutive path components, anywhere in the
/// path. Comparison is case-insensitive so it agrees with classifiers
/// that canonicalize against `~/.agents/skills` on case-insensitive
/// filesystems (macOS/Windows by default).
///
/// The path arriving here can be any of:
///
///   1. Bare relative-to-worktree-root: `.agents/skills/...`
///   2. Worktree-name prefixed:         `<worktree>/.agents/skills/...`
///   3. Absolute:                       `/path/to/worktree/.agents/skills/...`
///
/// Any-depth matching has a known cost: a `.agents/skills` directory
/// nested inside vendored sources (e.g. `vendor/x/.agents/skills/...`)
/// would also be flagged. We accept that as the safer-failing direction —
/// an extra confirmation prompt for a vendored file is annoying, while
/// silently letting the agent overwrite a `.agents/skills` tree the user
/// didn't expect to be touched is unsafe.
pub fn is_agents_skills_path(path: &Path) -> bool {
    let mut components = path.components().map(|c| c.as_os_str());
    let Some(mut prev) = components.next() else {
        return false;
    };
    for curr in components {
        if component_matches_ignore_ascii_case(prev, AGENTS_DIR_NAME)
            && component_matches_ignore_ascii_case(curr, SKILLS_DIR_NAME)
        {
            return true;
        }
        prev = curr;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_load_skills_from_empty_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(Path::new("/skills")).await.unwrap();

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_load_single_skill(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(Path::new("/skills/my-skill")).await.unwrap();
        fs.insert_file(
            Path::new("/skills/my-skill/SKILL.md"),
            b"placeholder".to_vec(),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        assert_eq!(
            results,
            vec![Skill {
                source: SkillSource::Global,
                directory_path: PathBuf::from("/skills/my-skill"),
                skill_file_path: PathBuf::from("/skills/my-skill/SKILL.md"),
            }]
        );
    }

    #[gpui::test]
    async fn test_load_nested_skills(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(Path::new("/skills/skill-one")).await.unwrap();
        fs.insert_file(
            Path::new("/skills/skill-one/SKILL.md"),
            b"placeholder".to_vec(),
        )
        .await;
        fs.create_dir(Path::new("/skills/skill-two")).await.unwrap();
        fs.insert_file(
            Path::new("/skills/skill-two/SKILL.md"),
            b"placeholder".to_vec(),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        let paths: Vec<PathBuf> = results.iter().map(|s| s.skill_file_path.clone()).collect();
        assert_eq!(
            paths,
            vec![
                PathBuf::from("/skills/skill-one/SKILL.md"),
                PathBuf::from("/skills/skill-two/SKILL.md"),
            ]
        );
    }

    #[gpui::test]
    async fn test_load_skills_returns_results_sorted_by_path(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        for name in ["charlie", "alpha", "bravo", "delta"] {
            let dir = PathBuf::from("/skills").join(name);
            fs.create_dir(&dir).await.unwrap();
            fs.insert_file(&dir.join("SKILL.md"), b"placeholder".to_vec())
                .await;
        }

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        let paths: Vec<PathBuf> = results.iter().map(|s| s.skill_file_path.clone()).collect();

        let mut expected = paths.clone();
        expected.sort();
        assert_eq!(paths, expected);
    }

    #[gpui::test]
    async fn test_load_ignores_non_skill_files(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(Path::new("/skills/my-skill")).await.unwrap();
        fs.insert_file(
            Path::new("/skills/my-skill/SKILL.md"),
            b"placeholder".to_vec(),
        )
        .await;
        fs.insert_file(
            Path::new("/skills/not-a-skill.txt"),
            b"This is not a skill".to_vec(),
        )
        .await;
        fs.create_dir(Path::new("/skills/some-dir")).await.unwrap();
        fs.insert_file(
            Path::new("/skills/some-dir/other-file.md"),
            b"Not a SKILL.md".to_vec(),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].skill_file_path,
            PathBuf::from("/skills/my-skill/SKILL.md")
        );
    }

    #[gpui::test]
    async fn test_load_from_nonexistent_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/nonexistent"),
            SkillSource::Global,
        )
        .await;

        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_nested_skill_md_inside_skill_resources_is_not_loaded(cx: &mut TestAppContext) {
        // We only look at immediate children of the skills root, so a
        // `SKILL.md` nested inside a skill's resources directory cannot
        // accidentally be picked up as a separate skill.
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(Path::new("/skills/outer")).await.unwrap();
        fs.insert_file(Path::new("/skills/outer/SKILL.md"), b"placeholder".to_vec())
            .await;
        fs.create_dir(Path::new("/skills/outer/references"))
            .await
            .unwrap();
        fs.insert_file(
            Path::new("/skills/outer/references/SKILL.md"),
            b"placeholder".to_vec(),
        )
        .await;

        let results = load_skills_from_directory(
            &(fs as Arc<dyn Fs>),
            Path::new("/skills"),
            SkillSource::Global,
        )
        .await;

        let paths: Vec<PathBuf> = results.iter().map(|s| s.skill_file_path.clone()).collect();
        assert_eq!(paths, vec![PathBuf::from("/skills/outer/SKILL.md")]);
    }

    #[test]
    fn is_agents_skills_path_simple_positive() {
        assert!(is_agents_skills_path(Path::new(
            "foo/.agents/skills/my-skill/SKILL.md"
        )));
    }

    #[test]
    fn is_agents_skills_path_simple_negative() {
        assert!(!is_agents_skills_path(Path::new("foo/bar/baz")));
    }

    #[test]
    fn is_agents_skills_path_double_agents() {
        // `foo/.agents/.agents/skills` contains a `.agents/skills` pair at
        // depths 2-3. Any-depth matching catches it; this is intentional, so
        // a `.agents/skills` directory the user wasn't expecting to be
        // touched still prompts for confirmation.
        assert!(is_agents_skills_path(Path::new(
            "foo/.agents/.agents/skills"
        )));
    }

    #[test]
    fn is_agents_skills_path_agents_without_skills() {
        assert!(!is_agents_skills_path(Path::new("foo/.agents/other")));
    }

    #[test]
    fn is_agents_skills_path_at_start() {
        assert!(is_agents_skills_path(Path::new(".agents/skills")));
    }

    #[test]
    fn is_agents_skills_path_trailing_agents() {
        assert!(!is_agents_skills_path(Path::new("foo/.agents")));
    }

    #[test]
    fn is_agents_skills_path_deep_match() {
        // Any-depth matching: nested `.agents/skills` directories — e.g.
        // inside vendored sources — are flagged too. We prefer the extra
        // prompt over silently letting the agent edit something named
        // `.agents/skills`.
        assert!(is_agents_skills_path(Path::new("a/b/.agents/skills/x.txt")));
        assert!(is_agents_skills_path(Path::new(
            "some/random/place/.agents/skills/foo"
        )));
    }

    #[test]
    fn is_agents_skills_path_absolute() {
        // Absolute paths into a project-local `.agents/skills/` are caught
        // by the same consecutive-component match.
        assert!(is_agents_skills_path(Path::new(
            "/Users/foo/project/.agents/skills/my-skill/SKILL.md"
        )));
        assert!(!is_agents_skills_path(Path::new("/etc/hosts")));
    }

    #[test]
    fn is_agents_skills_path_case_insensitive() {
        // Filesystems on macOS/Windows are case-insensitive by default; the
        // classifier must agree.
        assert!(is_agents_skills_path(Path::new(".AGENTS/skills/foo")));
        assert!(is_agents_skills_path(Path::new(".agents/SKILLS/foo")));
        assert!(is_agents_skills_path(Path::new(
            "project/.AGENTS/SKILLS/foo"
        )));
    }
}
