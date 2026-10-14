//! Protected metadata paths the implementer must not write (ADR-0012).
//!
//! Pure path classifier plus default `bash.patterns` Deny rules. No Seatbelt
//! on the implementer (omp2 owns that). Write/edit auto-allow stays; cedian
//! denies by path at the gate, restores named edit/write targets, and pins
//! Deny patterns for bash commands that name a path into `.git`, enter it,
//! or move git's hooks elsewhere.

use crate::spawn_profile::{BashRule, ToolPolicy};
use std::path::{Component, Path, PathBuf};

/// Workspace-relative metadata dir names (Codex list + ADR-0012).
pub const METADATA_DIR_NAMES: &[&str] = &[".git", ".codex", ".agents", ".aws"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectedKind {
    Git,
    Metadata,
    Session,
}

#[derive(Debug, Clone)]
pub struct ProtectedRoots {
    pub workspace: PathBuf,
    /// Overlay/session roots the agent must not write (CLI `--session-dir`,
    /// app `state_dir/omp`, and optionally `~/.omp` when known).
    pub session_dirs: Vec<PathBuf>,
}

impl ProtectedRoots {
    /// Workspace plus the launch's overlay dir and optional OMP state root.
    pub fn for_launch(workspace: PathBuf, state_dir: &Path, omp_state: Option<PathBuf>) -> Self {
        let mut session_dirs = vec![state_dir.join("omp")];
        if let Some(omp) = omp_state {
            if !session_dirs.iter().any(|d| d == &omp) {
                session_dirs.push(omp);
            }
        }
        Self {
            workspace,
            session_dirs,
        }
    }

    /// Classify `path` as protected, or `None` when it may be written.
    ///
    /// Canonical forms are checked when they resolve; non-canonical forms
    /// are always checked too so a not-yet-created `.git/hooks/x` still
    /// denies. `.gitignore` / `.gitmodules` at the workspace root are files,
    /// not the `.git` directory, and are not protected.
    pub fn classify(&self, path: &Path) -> Option<ProtectedKind> {
        for candidate in path_forms(path) {
            if self
                .session_dirs
                .iter()
                .any(|root| is_under(&candidate, root))
            {
                return Some(ProtectedKind::Session);
            }
            if let Some(kind) = metadata_kind(&candidate, &self.workspace) {
                return Some(kind);
            }
        }
        None
    }
}

/// True when an approval dialog title/body names a protected path or a
/// command targeting one.
pub fn approval_targets_protected(title: &str, roots: &ProtectedRoots) -> bool {
    for token in path_tokens(title) {
        let direct = Path::new(token);
        if roots.classify(direct).is_some() {
            return true;
        }
        if !direct.is_absolute() && roots.classify(&roots.workspace.join(direct)).is_some() {
            return true;
        }
    }
    false
}

/// Default bash.patterns Deny rules (OMP 18.8.6 `tools/bash.ts`: `*` is any
/// text, the rule is anchored, and a Deny fires on the whole command or on
/// any one segment of a compound line). They deny any command that names a
/// path into `.git` (`>.git/x`, `./.git/x` and `a/.git/x` included), one
/// that enters `.git`, and git's `core.hooksPath`, which runs hooks from
/// outside it. Git reads config keys case-insensitively; OMP matches case.
pub fn default_protected_bash_rules() -> Vec<BashRule> {
    [
        "*.git/*",
        "cd .git",
        "cd */.git",
        "pushd .git",
        "pushd */.git",
        "*hooksPath*",
        "*hookspath*",
    ]
    .into_iter()
    .map(|pattern| BashRule {
        pattern: pattern.to_string(),
        approval: ToolPolicy::Deny,
    })
    .collect()
}

fn path_forms(path: &Path) -> Vec<PathBuf> {
    let mut forms = vec![path.to_path_buf()];
    if let Ok(canon) = path.canonicalize() {
        if canon != path {
            forms.push(canon);
        }
        return forms;
    }
    // Not on disk yet: canonicalize the longest existing ancestor and
    // rejoin the rest so `/ws/.git/hooks/new` still matches `/ws/.git`.
    let mut ancestor = path.to_path_buf();
    while let Some(parent) = ancestor.parent() {
        if parent == ancestor {
            break;
        }
        if let Ok(canon) = parent.canonicalize() {
            if let Ok(rest) = path.strip_prefix(parent) {
                let joined = canon.join(rest);
                if !forms.iter().any(|f| f == &joined) {
                    forms.push(joined);
                }
            }
            break;
        }
        ancestor = parent.to_path_buf();
    }
    forms
}

fn is_under(path: &Path, root: &Path) -> bool {
    for p in path_forms(path) {
        for r in path_forms(root) {
            if p == r || p.starts_with(&r) {
                return true;
            }
        }
    }
    path == root || path.starts_with(root)
}

fn metadata_kind(path: &Path, workspace: &Path) -> Option<ProtectedKind> {
    let rel = relative_to(path, workspace)?;
    for component in rel.components() {
        let Component::Normal(name) = component else {
            continue;
        };
        let name = name.to_string_lossy();
        match name.as_ref() {
            ".git" => return Some(ProtectedKind::Git),
            n if METADATA_DIR_NAMES.contains(&n) => return Some(ProtectedKind::Metadata),
            _ => {}
        }
    }
    None
}

fn relative_to(path: &Path, root: &Path) -> Option<PathBuf> {
    for p in path_forms(path) {
        for r in path_forms(root) {
            if let Ok(rel) = p.strip_prefix(&r) {
                return Some(rel.to_path_buf());
            }
        }
    }
    path.strip_prefix(root).ok().map(|r| r.to_path_buf())
}

fn path_tokens(title: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    for line in title.lines() {
        let line = line.trim();
        if let Some(rest) = line
            .strip_prefix("Path: ")
            .or_else(|| line.strip_prefix("path: "))
        {
            tokens.push(rest.trim());
            continue;
        }
        if let Some(rest) = line
            .strip_prefix("Command: ")
            .or_else(|| line.strip_prefix("command: "))
        {
            for part in rest.split_whitespace() {
                let part = part.trim_matches(|c| matches!(c, '"' | '\'' | '`' | ';' | '&' | '|'));
                if part.contains('/') || part.contains(".git") || part.starts_with('.') {
                    tokens.push(part);
                }
            }
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn roots(ws: &Path, session: &Path) -> ProtectedRoots {
        ProtectedRoots {
            workspace: ws.to_path_buf(),
            session_dirs: vec![session.to_path_buf()],
        }
    }

    fn temp(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("cedian-protected-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn classify_git_metadata_session_and_ordinary() {
        let dir = temp("classify");
        let ws = dir.join("ws");
        let session = dir.join("state/omp");
        fs::create_dir_all(ws.join(".git/hooks")).unwrap();
        fs::create_dir_all(ws.join(".codex")).unwrap();
        fs::create_dir_all(ws.join("src")).unwrap();
        fs::create_dir_all(&session).unwrap();
        fs::write(ws.join(".gitignore"), "target/\n").unwrap();
        fs::write(ws.join("src/main.rs"), "fn main() {}\n").unwrap();
        fs::write(ws.join(".git/hooks/pre-commit"), "#!/bin/sh\n").unwrap();
        fs::write(ws.join(".codex/config"), "x\n").unwrap();
        fs::write(session.join("overlay.yml"), "x\n").unwrap();

        let roots = roots(&ws, &session);
        assert_eq!(
            roots.classify(&ws.join(".git/hooks/pre-commit")),
            Some(ProtectedKind::Git)
        );
        assert_eq!(roots.classify(&ws.join("src/main.rs")), None);
        assert_eq!(roots.classify(&ws.join(".gitignore")), None);
        assert_eq!(
            roots.classify(&session.join("overlay.yml")),
            Some(ProtectedKind::Session)
        );
        assert_eq!(
            roots.classify(&ws.join(".codex/config")),
            Some(ProtectedKind::Metadata)
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn approval_title_with_git_path_is_protected() {
        let dir = temp("approval");
        let ws = dir.join("ws");
        let session = dir.join("omp");
        fs::create_dir_all(ws.join(".git/hooks")).unwrap();
        fs::create_dir_all(&session).unwrap();
        let roots = roots(&ws, &session);
        let title = format!("Allow tool: write\nPath: {}/.git/hooks/x", ws.display());
        assert!(approval_targets_protected(&title, &roots));
        assert!(!approval_targets_protected(
            "Allow tool: write\nPath: src/main.rs",
            &roots
        ));
        let _ = fs::remove_dir_all(&dir);
    }

    /// OMP's match for a Deny rule: the glob against the whole command or
    /// against one segment (split at `;`, `&&`, `||`, `|`, `&`, newlines).
    fn omp_denies(command: &str) -> bool {
        fn glob(pattern: &str, text: &str) -> bool {
            let parts: Vec<&str> = pattern.split('*').collect();
            let (first, last) = (parts[0], parts[parts.len() - 1]);
            if parts.len() == 1 {
                return pattern == text;
            }
            if !text.starts_with(first) || !text[first.len()..].ends_with(last) {
                return false;
            }
            let mut rest = &text[first.len()..text.len() - last.len()];
            for part in &parts[1..parts.len() - 1] {
                match rest.find(part) {
                    Some(at) => rest = &rest[at + part.len()..],
                    None => return false,
                }
            }
            true
        }
        let segments = command
            .split(['\n', ';', '|', '&'])
            .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|s| !s.is_empty());
        let whole = command.split_whitespace().collect::<Vec<_>>().join(" ");
        let rules = default_protected_bash_rules();
        assert!(rules.iter().all(|r| r.approval == ToolPolicy::Deny));
        std::iter::once(whole)
            .chain(segments)
            .any(|text| rules.iter().any(|r| glob(&r.pattern, &text)))
    }

    #[test]
    fn default_bash_rules_deny_writes_into_git_and_allow_the_rest() {
        for command in [
            "echo x > .git/hooks/pre-commit",
            "echo x >.git/hooks/pre-commit",
            "cat y > ./.git/config",
            "printf x >> sub/.git/hooks/post-merge",
            "cp evil .git/hooks/pre-commit",
            "tee .git/info/exclude",
            "cd .git && echo x > hooks/pre-commit",
            "cd ./.git; echo x > hooks/pre-commit",
            "pushd sub/.git",
            "git config core.hooksPath /tmp/h",
            "git -c core.hookspath=/tmp/h commit -m x",
        ] {
            assert!(omp_denies(command), "not denied: {command}");
        }
        for command in [
            "git status",
            "git commit -m 'fix'",
            "cat .gitignore",
            "ls .github/workflows",
            "cargo test -p cedian_omp",
        ] {
            assert!(!omp_denies(command), "denied: {command}");
        }
    }
}
