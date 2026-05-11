//! Paths and constants describing where agent skills live on disk.
//!
//! Skill discovery, parsing, and loading are intentionally *not* in this
//! crate yet — the only thing here is the directory layout, so other code
//! (e.g. the agent's tool-permission machinery) can recognize and protect
//! paths inside the agent skills tree without needing to know anything
//! about the skill format itself.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use util::paths::component_matches_ignore_ascii_case;

/// First segment of the skills directory path: `.agents`.
pub const AGENTS_DIR_NAME: &str = ".agents";

/// Second segment of the skills directory path: `skills`.
pub const SKILLS_DIR_NAME: &str = "skills";

/// The name of a skill definition file, e.g. `<skills_dir>/<skill>/SKILL.md`.
pub const SKILL_FILE_NAME: &str = "SKILL.md";

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
        if matches_pair(prev, curr) {
            return true;
        }
        prev = curr;
    }
    false
}

fn matches_pair(agents: &OsStr, skills: &OsStr) -> bool {
    component_matches_ignore_ascii_case(agents, AGENTS_DIR_NAME)
        && component_matches_ignore_ascii_case(skills, SKILLS_DIR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_positive() {
        assert!(is_agents_skills_path(Path::new(
            "foo/.agents/skills/my-skill/SKILL.md"
        )));
    }

    #[test]
    fn simple_negative() {
        assert!(!is_agents_skills_path(Path::new("foo/bar/baz")));
    }

    #[test]
    fn double_agents() {
        // `foo/.agents/.agents/skills` contains a `.agents/skills` pair at
        // depths 2-3. Any-depth matching catches it; this is intentional, so
        // a `.agents/skills` directory the user wasn't expecting to be
        // touched still prompts for confirmation.
        assert!(is_agents_skills_path(Path::new(
            "foo/.agents/.agents/skills"
        )));
    }

    #[test]
    fn agents_without_skills() {
        assert!(!is_agents_skills_path(Path::new("foo/.agents/other")));
    }

    #[test]
    fn at_start() {
        assert!(is_agents_skills_path(Path::new(".agents/skills")));
    }

    #[test]
    fn trailing_agents() {
        assert!(!is_agents_skills_path(Path::new("foo/.agents")));
    }

    #[test]
    fn deep_match() {
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
    fn absolute() {
        // Absolute paths into a project-local `.agents/skills/` are caught
        // by the same consecutive-component match.
        assert!(is_agents_skills_path(Path::new(
            "/Users/foo/project/.agents/skills/my-skill/SKILL.md"
        )));
        assert!(!is_agents_skills_path(Path::new("/etc/hosts")));
    }

    #[test]
    fn case_insensitive() {
        // Filesystems on macOS/Windows are case-insensitive by default; the
        // classifier must agree.
        assert!(is_agents_skills_path(Path::new(".AGENTS/skills/foo")));
        assert!(is_agents_skills_path(Path::new(".agents/SKILLS/foo")));
        assert!(is_agents_skills_path(Path::new(
            "project/.AGENTS/SKILLS/foo"
        )));
    }
}
