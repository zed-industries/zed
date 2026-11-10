//! Policy types.
//!
//! [`SandboxPolicy`] is what callers write: rules can carry [`Handler`]s,
//! which are closures in the caller's process. [`WirePolicy`] is what the
//! supervisor enforces: the same rules with each handler replaced by an id
//! the caller keeps a table of.

use std::collections::BTreeSet;
use std::fmt;
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::Responder;

/// What a sandbox may do.
///
/// Each resource has its own ordered rule list. Within a list, for each
/// permission, the last rule that matches and sets that permission decides;
/// a permission no matching rule sets is denied. Order between lists means
/// nothing.
#[derive(Clone, Debug, Default)]
pub struct SandboxPolicy {
    /// Filesystem access, by host path.
    ///
    /// - Read means opening a file for its contents. Listing directories,
    ///   `stat`, `readlink` and `execve` are never restricted.
    /// - Write means any modification: creating, deleting, renaming,
    ///   linking, opening for writing, `chmod`, `truncate`, timestamps.
    pub fs: Vec<FsRule>,
}

impl SandboxPolicy {
    /// Every path readable and writable.
    pub fn unrestricted() -> Self {
        Self {
            fs: vec![FsRule {
                path: PathPattern::root(),
                read: Some(Access::Allow),
                write: Some(Access::Allow),
            }],
        }
    }
}

/// A rule of any resource kind, for handlers that add rules
/// ([`Decision::remember`]).
#[derive(Clone, Debug)]
pub enum Rule {
    Fs(FsRule),
}

#[derive(Clone, Debug)]
pub struct FsRule {
    pub path: PathPattern,
    /// `None`: this rule has no opinion, and earlier rules still apply.
    pub read: Option<Access>,
    pub write: Option<Access>,
}

impl FsRule {
    pub fn new(path: impl Into<String>) -> Result<Self> {
        Ok(Self {
            path: PathPattern::new(path)?,
            read: None,
            write: None,
        })
    }

    pub fn read(mut self, access: Access) -> Self {
        self.read = Some(access);
        self
    }

    pub fn write(mut self, access: Access) -> Self {
        self.write = Some(access);
        self
    }
}

#[derive(Clone, Debug)]
pub enum Access {
    Allow,
    Deny,
    /// Defer the decision to `Handler`, each time. The syscall blocks until
    /// it responds.
    Ask(Handler),
}

/// Decides requests for access that a rule set to [`Access::Ask`].
///
/// Called on a background thread shared by every sandbox in the context, so
/// it must return promptly: answer through the [`Responder`], from any
/// thread, whenever the decision is made. A panicking handler denies.
///
/// Requests to one handler in one sandbox are asked one at a time. While one
/// is open, an identical request shares its decision, and other requests
/// wait and are then re-checked against the policy (including any rules the
/// decision remembered) before being asked.
#[derive(Clone)]
pub struct Handler(pub(crate) Arc<dyn Fn(AccessRequest, Responder) + Send + Sync>);

impl Handler {
    pub fn new(handler: impl Fn(AccessRequest, Responder) + Send + Sync + 'static) -> Self {
        Self(Arc::new(handler))
    }
}

impl fmt::Debug for Handler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Handler")
    }
}

/// What one blocked syscall needs. A single decision covers all of it.
#[derive(Clone, Debug)]
pub struct AccessRequest {
    pub items: Vec<RequestedAccess>,
    /// The thread that made the syscall.
    pub pid: i32,
    pub executable: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestedAccess {
    Fs {
        permission: Permission,
        path: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
}

/// A handler's answer.
#[derive(Clone, Debug, Default)]
pub struct Decision {
    pub allow: bool,
    /// Rules appended to the sandbox's live policy before the syscall is
    /// answered, so later requests they cover don't ask again. They last
    /// until the sandbox is dropped or its policy is replaced.
    pub remember: Vec<Rule>,
}

impl Decision {
    pub fn allow() -> Self {
        Self {
            allow: true,
            remember: Vec::new(),
        }
    }

    pub fn deny() -> Self {
        Self::default()
    }
}

pub(crate) type HandlerId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WireAccess {
    Allow,
    Deny,
    Ask(HandlerId),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum WireRule {
    Fs(WireFsRule),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct WireFsRule {
    pub path: PathPattern,
    pub read: Option<WireAccess>,
    pub write: Option<WireAccess>,
}

impl WireFsRule {
    fn sets(&self, permission: Permission) -> Option<WireAccess> {
        match permission {
            Permission::Read => self.read,
            Permission::Write => self.write,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct WirePolicy {
    pub fs: Vec<WireFsRule>,
}

impl WirePolicy {
    pub fn add(&mut self, rule: WireRule) {
        match rule {
            WireRule::Fs(rule) => self.fs.push(rule),
        }
    }

    pub fn fs_access(&self, path: &Path, permission: Permission) -> WireAccess {
        self.fs
            .iter()
            .rev()
            .filter_map(|rule| Some((rule, rule.sets(permission)?)))
            .find(|(rule, _)| rule.path.matches(path))
            .map_or(WireAccess::Deny, |(_, access)| access)
    }

    /// Whether every rule about `permission` treats paths beneath `a` exactly
    /// as it treats the same relative paths beneath `b`. If so, moving a
    /// directory from `a` to `b` can't change `permission` for anything in it.
    pub fn same_beneath(&self, a: &Path, b: &Path, permission: Permission) -> bool {
        self.fs
            .iter()
            .filter(|rule| rule.sets(permission).is_some())
            .all(|rule| rule.path.state_after(a) == rule.path.state_after(b))
    }

    /// Whether giving the object at `from` the additional or new name `to`
    /// would let `permission` reach it where it previously couldn't.
    pub fn widens(&self, from: &Path, to: &Path, permission: Permission) -> bool {
        self.fs_access(from, permission) != WireAccess::Allow
            && self.fs_access(to, permission) == WireAccess::Allow
    }
}

impl Rule {
    pub(crate) fn to_wire(&self, register: &mut impl FnMut(&Handler) -> HandlerId) -> WireRule {
        match self {
            Rule::Fs(rule) => WireRule::Fs(rule.to_wire(register)),
        }
    }
}

impl FsRule {
    pub(crate) fn to_wire(&self, register: &mut impl FnMut(&Handler) -> HandlerId) -> WireFsRule {
        let mut convert = |access: &Option<Access>| {
            access.as_ref().map(|access| match access {
                Access::Allow => WireAccess::Allow,
                Access::Deny => WireAccess::Deny,
                Access::Ask(handler) => WireAccess::Ask(register(handler)),
            })
        };
        WireFsRule {
            path: self.path.clone(),
            read: convert(&self.read),
            write: convert(&self.write),
        }
    }
}

/// An absolute host path pattern. A pattern matches a path and everything
/// beneath it. Components may contain `*` (any run of characters other than
/// `/`) and `?` (one character), and a whole component of `**` matches any
/// number of components.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PathPattern {
    source: String,
    segments: Vec<Segment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Segment {
    Literal(Vec<u8>),
    Wildcard(Vec<u8>),
    AnyDepth,
}

/// Where matching stands after consuming some leading path components.
#[derive(Debug, PartialEq, Eq)]
enum MatchState {
    /// The components so far match, so everything beneath them does too.
    Matched,
    /// Segment positions that could still lead to a match (empty: none can).
    Pending(BTreeSet<usize>),
}

impl PathPattern {
    pub fn new(source: impl Into<String>) -> Result<Self> {
        let source = source.into();
        if !source.starts_with('/') {
            bail!("path pattern `{source}` must be absolute");
        }
        let mut segments = Vec::new();
        for component in source.split('/').filter(|component| !component.is_empty()) {
            segments.push(match component {
                "." | ".." => bail!("path pattern `{source}` must not contain `.` or `..`"),
                "**" => Segment::AnyDepth,
                _ if component.contains("**") => {
                    bail!("`**` must be a whole component in path pattern `{source}`")
                }
                _ if component.contains(['*', '?']) => {
                    Segment::Wildcard(component.as_bytes().to_vec())
                }
                _ => Segment::Literal(component.as_bytes().to_vec()),
            });
        }
        Ok(Self { source, segments })
    }

    /// A pattern matching exactly `path` and everything beneath it.
    pub fn literal(path: &Path) -> Result<Self> {
        let Some(source) = path.to_str() else {
            bail!("path {} is not valid UTF-8", path.display());
        };
        if source.contains(['*', '?']) {
            bail!("path {source} contains wildcard characters");
        }
        Self::new(source)
    }

    fn root() -> Self {
        Self {
            source: "/".to_string(),
            segments: Vec::new(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.source
    }

    /// Whether `path` (absolute and normalized) is matched by this pattern or
    /// lies beneath a match.
    pub fn matches(&self, path: &Path) -> bool {
        self.state_after(path) == MatchState::Matched
    }

    fn state_after(&self, path: &Path) -> MatchState {
        let mut components = Vec::new();
        for component in path.components() {
            match component {
                Component::RootDir => {}
                Component::Normal(name) => components.push(name.as_bytes()),
                Component::Prefix(_) | Component::CurDir | Component::ParentDir => {
                    return MatchState::Pending(BTreeSet::new());
                }
            }
        }

        let end = self.segments.len();
        let mut positions = self.closure(BTreeSet::from([0]));
        for component in components {
            if positions.contains(&end) {
                return MatchState::Matched;
            }
            let mut next = BTreeSet::new();
            for &position in &positions {
                match &self.segments[position] {
                    Segment::AnyDepth => {
                        next.insert(position);
                    }
                    Segment::Literal(name) => {
                        if component == name.as_slice() {
                            next.insert(position + 1);
                        }
                    }
                    Segment::Wildcard(pattern) => {
                        if wildcard_matches(pattern, component) {
                            next.insert(position + 1);
                        }
                    }
                }
            }
            positions = self.closure(next);
        }
        if positions.contains(&end) {
            MatchState::Matched
        } else {
            MatchState::Pending(positions)
        }
    }

    /// Add the positions reachable without consuming a component (`**` may
    /// match zero components).
    fn closure(&self, mut positions: BTreeSet<usize>) -> BTreeSet<usize> {
        let mut stack: Vec<usize> = positions.iter().copied().collect();
        while let Some(position) = stack.pop() {
            if matches!(self.segments.get(position), Some(Segment::AnyDepth))
                && positions.insert(position + 1)
            {
                stack.push(position + 1);
            }
        }
        positions
    }
}

impl TryFrom<String> for PathPattern {
    type Error = anyhow::Error;

    fn try_from(source: String) -> Result<Self> {
        Self::new(source)
    }
}

impl From<PathPattern> for String {
    fn from(pattern: PathPattern) -> Self {
        pattern.source
    }
}

fn wildcard_matches(pattern: &[u8], text: &[u8]) -> bool {
    match pattern.split_first() {
        None => text.is_empty(),
        Some((b'*', rest)) => (0..=text.len()).any(|skip| wildcard_matches(rest, &text[skip..])),
        Some((b'?', rest)) => !text.is_empty() && wildcard_matches(rest, &text[1..]),
        Some((byte, rest)) => text.first() == Some(byte) && wildcard_matches(rest, &text[1..]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Access::{Allow, Deny};
    use Permission::*;

    fn pattern(source: &str) -> PathPattern {
        PathPattern::new(source).unwrap()
    }

    fn rule(source: &str) -> FsRule {
        FsRule::new(source).unwrap()
    }

    /// Convert as `Sandbox` does, numbering handlers in order of appearance.
    fn wire(rules: Vec<FsRule>) -> WirePolicy {
        let mut next = 0;
        let mut register = |_: &Handler| {
            next += 1;
            next - 1
        };
        WirePolicy {
            fs: rules
                .iter()
                .map(|rule| rule.to_wire(&mut register))
                .collect(),
        }
    }

    #[test]
    fn test_pattern_matching() {
        assert!(pattern("/").matches(Path::new("/anything/at/all")));
        assert!(pattern("/proj").matches(Path::new("/proj")));
        assert!(pattern("/proj").matches(Path::new("/proj/src/main.rs")));
        assert!(!pattern("/proj").matches(Path::new("/project")));
        assert!(!pattern("/proj").matches(Path::new("/")));
        assert!(pattern("/proj/*.env").matches(Path::new("/proj/prod.env")));
        assert!(pattern("/proj/*.env").matches(Path::new("/proj/.env")));
        assert!(!pattern("/proj/*.env").matches(Path::new("/proj/sub/prod.env")));
        assert!(pattern("/**/*.env").matches(Path::new("/a/b/c.env")));
        assert!(pattern("/**/*.env").matches(Path::new("/c.env")));
        assert!(pattern("/proj/**/.git").matches(Path::new("/proj/.git/config")));
        assert!(pattern("/proj/**/.git").matches(Path::new("/proj/a/b/.git")));
        assert!(!pattern("/proj/**/.git").matches(Path::new("/proj/a/b/git")));
        assert!(pattern("/a/?").matches(Path::new("/a/b")));
        assert!(!pattern("/a/?").matches(Path::new("/a/bc")));
        assert!(!pattern("/proj").matches(Path::new("/proj/../etc")));
    }

    #[test]
    fn test_pattern_validation() {
        assert!(PathPattern::new("relative").is_err());
        assert!(PathPattern::new("/a/../b").is_err());
        assert!(PathPattern::new("/a/b**").is_err());
        assert!(PathPattern::literal(Path::new("/a/b*")).is_err());
    }

    #[test]
    fn test_last_matching_rule_wins_per_permission() {
        let handler = Handler::new(|_, _| {});
        let policy = wire(vec![
            rule("/").read(Allow),
            rule("/proj").write(Allow),
            rule("/proj/.git").write(Deny),
            rule("/**/*.env").read(Deny),
            rule("/proj/ask").write(Access::Ask(handler)),
        ]);
        let access = |path: &str, permission| policy.fs_access(Path::new(path), permission);
        assert_eq!(access("/etc/passwd", Read), WireAccess::Allow);
        assert_eq!(access("/etc/passwd", Write), WireAccess::Deny);
        assert_eq!(access("/proj/src", Write), WireAccess::Allow);
        assert_eq!(access("/proj/.git/config", Write), WireAccess::Deny);
        assert_eq!(access("/proj/.env", Read), WireAccess::Deny);
        // The `.env` rule says nothing about writes.
        assert_eq!(access("/proj/.env", Write), WireAccess::Allow);
        assert_eq!(access("/proj/ask/x", Write), WireAccess::Ask(0));
        assert_eq!(
            WirePolicy::default().fs_access(Path::new("/x"), Read),
            WireAccess::Deny
        );
        assert_eq!(
            wire(SandboxPolicy::unrestricted().fs).fs_access(Path::new("/x"), Write),
            WireAccess::Allow
        );
    }

    #[test]
    fn test_directory_moves() {
        let policy = wire(vec![
            rule("/").read(Allow),
            rule("/proj").write(Allow),
            rule("/proj/.git").write(Deny),
            rule("/**/*.env").read(Deny),
            rule("/home/*/secret").read(Deny),
        ]);
        let same = |a: &str, b: &str, permission| {
            policy.same_beneath(Path::new(a), Path::new(b), permission)
        };
        // `**/*.env` doesn't care where a directory lives.
        assert!(same("/proj/a", "/proj/b", Read));
        assert!(same("/proj/a", "/proj/b", Write));
        // Moving the project would move `.git` out from under its rule.
        assert!(!same("/proj", "/proj2", Write));
        // Contents named `secret` are only denied directly under `/home/*`.
        assert!(!same("/home", "/tmp", Read));
        assert!(same("/home/u/a", "/tmp/a", Read));
        assert!(!same("/home/u", "/tmp/u", Read));
    }
}
