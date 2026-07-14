pub mod blame;
pub mod commit;
mod hosting_provider;
mod remote;
pub mod repository;
pub mod stash;
pub mod status;

pub use crate::hosting_provider::*;
pub use crate::remote::*;
use anyhow::{Context as _, Result};
use gpui::{Action, actions};
pub use repository::RemoteCommandOutput;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fmt::{self, Write as _};
use std::path::PathBuf;
use std::str::FromStr;

pub const DOT_GIT: &str = ".git";
pub const GITIGNORE: &str = ".gitignore";
pub const FSMONITOR_DAEMON: &str = "fsmonitor--daemon";
pub const LFS_DIR: &str = "lfs";
pub const HEAD: &str = "HEAD";
pub const OBJECTS_DIR: &str = "objects";
pub const REFS_DIR: &str = "refs";
pub const REFTABLE_DIR: &str = "reftable";
pub const HOOKS_DIR: &str = "hooks";
pub const LOGS_DIR: &str = "logs";
pub const LOGS_REF_STASH: &str = "logs/refs/stash";
pub const REBASE_MERGE_DIR: &str = "rebase-merge";
pub const REBASE_APPLY_DIR: &str = "rebase-apply";
pub const SEQUENCER_DIR: &str = "sequencer";
pub const COMMIT_MESSAGE: &str = "COMMIT_EDITMSG";
pub const FETCH_HEAD: &str = "FETCH_HEAD";
pub const ORIG_HEAD: &str = "ORIG_HEAD";
pub const BISECT_LOG: &str = "BISECT_LOG";
pub const GC_PID: &str = "gc.pid";
pub const INFO_DIR: &str = "info";
pub const REPO_EXCLUDE: &str = "info/exclude";
pub const COMMONDIR: &str = "commondir";

actions!(
    git,
    [
        // per-hunk
        /// Toggles the staged state of the hunk or status entry at cursor.
        ToggleStaged,
        /// Stage status entries between an anchor entry and the cursor.
        StageRange,
        /// Stages the current hunk and moves to the next one.
        StageAndNext,
        /// Unstages the current hunk and moves to the next one.
        UnstageAndNext,
        /// Restores the selected hunks to their original state.
        #[action(deprecated_aliases = ["editor::RevertSelectedHunks"])]
        Restore,
        /// Restores the selected hunks to their original state and moves to the
        /// next one.
        RestoreAndNext,
        // per-file
        /// Shows git blame information for the current file.
        #[action(deprecated_aliases = ["editor::ToggleGitBlame"])]
        Blame,
        /// Shows the git history for the selected file, folder, or project.
        FileHistory,
        /// Opens the selected file in the editor without a diff view.
        ViewFile,
        /// Stages the current file.
        StageFile,
        /// Unstages the current file.
        UnstageFile,
        // repo-wide
        /// Stages all changes in the repository.
        StageAll,
        /// Unstages all changes in the repository.
        UnstageAll,
        /// Stashes all changes in the repository, including untracked files.
        StashAll,
        /// Pops the most recent stash.
        StashPop,
        /// Apply the most recent stash.
        StashApply,
        /// Restores all tracked files to their last committed state.
        RestoreTrackedFiles,
        /// Moves all untracked files to trash.
        TrashUntrackedFiles,
        /// Undoes the last commit, keeping changes in the working directory.
        Uncommit,
        /// Pushes commits to the remote repository.
        Push,
        /// Pushes commits to a specific remote branch.
        PushTo,
        /// Force pushes commits to the remote repository.
        ForcePush,
        /// Pulls changes from the remote repository.
        Pull,
        /// Pulls changes from the remote repository with rebase.
        PullRebase,
        /// Fetches changes from the remote repository.
        Fetch,
        /// Fetches changes from a specific remote.
        FetchFrom,
        /// Creates a new commit with staged changes.
        Commit,
        /// Amends the last commit with staged changes.
        Amend,
        /// Enable the --signoff option.
        Signoff,
        /// Cancels the current git operation.
        Cancel,
        /// Expands the commit message editor.
        ExpandCommitEditor,
        /// Toggles whether the commit message editor fills all the available
        /// vertical space within the git panel.
        ToggleFillCommitEditor,
        /// Generates a commit message using AI.
        GenerateCommitMessage,
        /// Initializes a new git repository.
        Init,
        /// Opens all modified files in the editor.
        OpenModifiedFiles,
        /// Opens the current file in a solo diff view.
        OpenFileDiff,
        /// Clones a repository.
        Clone,
        ViewCommit,
        /// Adds a file to .gitignore.
        AddToGitignore,
        /// Adds a file to the repository's .git/info/exclude.
        AddToGitInfoExclude,
        /// Copies the current branch name to the clipboard.
        CopyBranchName,
    ]
);

/// Renames a git branch.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = git)]
#[serde(deny_unknown_fields)]
pub struct RenameBranch {
    /// The branch to rename.
    ///
    /// Default: the current branch.
    #[serde(default)]
    pub branch: Option<String>,
}

/// Restores a file to its last committed state, discarding local changes.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = git, deprecated_aliases = ["editor::RevertFile"])]
#[serde(deny_unknown_fields)]
pub struct RestoreFile {
    #[serde(default)]
    pub skip_prompt: bool,
}

/// The length of a Git short SHA.
pub const SHORT_SHA_LENGTH: usize = 7;

const SHA1_BYTE_LENGTH: usize = 20;
const SHA256_BYTE_LENGTH: usize = 32;
const SHA1_HEX_LENGTH: usize = SHA1_BYTE_LENGTH * 2;
const SHA256_HEX_LENGTH: usize = SHA256_BYTE_LENGTH * 2;
const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Whether the raw contents of a regular `.git/HEAD` file denote a valid git head,
/// mirroring git's `validate_headref` (setup.c) for the non-symlink case:
///
/// * a symbolic ref `ref:` (at byte zero, no leading whitespace) whose target —
///   after skipping only space, tab, LF, CR (git's `sane isspace`) — begins with
///   `refs/`. git does not validate the refname further, so neither do we; and
/// * otherwise a detached object id, accepted when the contents begin with at
///   least 40 ASCII hex digits. git parses "any supported algorithm, longest
///   first" and never checks the trailing byte, so 40 hex (SHA-1), 64 hex
///   (SHA-256), and any longer/junk-suffixed hex prefix all pass; this keeps
///   git's forward compatibility with future, longer hashes. Fewer than 40 hex
///   is rejected. `Oid::from_str` is deliberately not used here — it accepts
///   abbreviated (<40) ids, which git's HEAD check does not.
///
/// An empty or leftover `.git` has no valid `HEAD`, so this returns `false`.
/// Operates on bytes so a non-UTF-8 suffix after an otherwise-valid prefix (which
/// git accepts) is not spuriously rejected.
pub fn regular_head_contents_are_valid(contents: &[u8]) -> bool {
    if let Some(mut target) = contents.strip_prefix(b"ref:") {
        while let [first, rest @ ..] = target
            && matches!(*first, b' ' | b'\t' | b'\n' | b'\r')
        {
            target = rest;
        }
        return target.starts_with(b"refs/");
    }
    contents
        .get(..SHA1_HEX_LENGTH)
        .is_some_and(|prefix| prefix.iter().all(u8::is_ascii_hexdigit))
}

/// Whether a symlink `HEAD`'s raw link text denotes a valid git head. git's
/// `validate_headref` inspects the symlink target *without following it* and
/// accepts it iff the link text begins with `refs/` (an unborn or packed-only
/// target is legal). Pass the raw `readlink` result; it is never resolved.
pub fn head_symlink_target_is_valid(target: &OsStr) -> bool {
    target.as_encoded_bytes().starts_with(b"refs/")
}

/// Parses a `.git` *gitfile* (the `.git` of a linked worktree or a separate
/// git-dir checkout), returning the referenced git directory path. Mirrors git's
/// `read_gitfile_gently`: the contents must begin with the literal `gitdir: `
/// (including the single space); only trailing CR/LF is stripped, so any further
/// spaces or tabs are part of the path. The returned path may be relative — the
/// caller resolves it against the gitfile's parent. Non-UTF-8 paths are preserved
/// on Unix.
pub fn parse_gitfile(contents: &[u8]) -> Result<PathBuf> {
    let path = contents
        .strip_prefix(b"gitdir: ")
        .context("gitfile must begin with `gitdir: `")?;
    let path = strip_trailing_crlf(path);
    anyhow::ensure!(!path.is_empty(), "gitfile has an empty gitdir path");
    Ok(bytes_to_path(path))
}

/// Parses a git `commondir` file, returning the referenced common directory path.
/// Mirrors git's `get_common_dir_noenv`: a zero-byte file is invalid, but after
/// stripping trailing CR/LF the remainder may be empty — git treats that as an empty
/// relative path, which the caller resolves back to the repository directory. The
/// returned path may be relative.
pub fn parse_commondir(contents: &[u8]) -> Result<PathBuf> {
    anyhow::ensure!(!contents.is_empty(), "commondir file is empty");
    Ok(bytes_to_path(strip_trailing_crlf(contents)))
}

fn strip_trailing_crlf(mut bytes: &[u8]) -> &[u8] {
    while let [rest @ .., last] = bytes
        && matches!(*last, b'\n' | b'\r')
    {
        bytes = rest;
    }
    bytes
}

fn bytes_to_path(bytes: &[u8]) -> PathBuf {
    // Unix: raw bytes; Windows: WTF-8. Lossy conversion is only reachable for
    // genuinely malformed metadata, which resolution then rejects downstream.
    <PathBuf as util::paths::PathExt>::try_from_bytes(bytes)
        .unwrap_or_else(|_| PathBuf::from(String::from_utf8_lossy(bytes).into_owned()))
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Oid {
    bytes: [u8; SHA256_BYTE_LENGTH],
    format: OidFormat,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum OidFormat {
    Sha1,
    Sha256,
}

impl OidFormat {
    fn byte_len(self) -> usize {
        match self {
            Self::Sha1 => SHA1_BYTE_LENGTH,
            Self::Sha256 => SHA256_BYTE_LENGTH,
        }
    }

    fn hex_len(self) -> usize {
        match self {
            Self::Sha1 => SHA1_HEX_LENGTH,
            Self::Sha256 => SHA256_HEX_LENGTH,
        }
    }
}

impl Oid {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let format = match bytes.len() {
            SHA1_BYTE_LENGTH => OidFormat::Sha1,
            SHA256_BYTE_LENGTH => OidFormat::Sha256,
            len => {
                anyhow::bail!(
                    "invalid git oid byte length: expected {SHA1_BYTE_LENGTH} for SHA-1 or {SHA256_BYTE_LENGTH} for SHA-256, got {len}"
                );
            }
        };

        let mut oid_bytes = [0u8; SHA256_BYTE_LENGTH];
        oid_bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(Self {
            bytes: oid_bytes,
            format,
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        let mut bytes = [0u8; SHA256_BYTE_LENGTH];
        rng.fill(&mut bytes[..SHA1_BYTE_LENGTH]);
        Self {
            bytes,
            format: OidFormat::Sha1,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.format.byte_len()]
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|byte| *byte == 0)
    }

    /// Returns this [`Oid`] as a short SHA.
    pub fn display_short(&self) -> String {
        self.hex_string(SHORT_SHA_LENGTH)
    }

    fn hex_string(&self, len: usize) -> String {
        let mut string = String::with_capacity(len);
        for index in 0..len {
            string.push(self.hex_digit(index));
        }
        string
    }

    fn write_hex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for index in 0..self.format.hex_len() {
            f.write_char(self.hex_digit(index))?;
        }
        Ok(())
    }

    #[inline(always)]
    fn hex_digit(&self, index: usize) -> char {
        debug_assert!(index < self.format.hex_len());
        let byte = self.as_bytes()[index / 2];
        let nibble = if index & 1 == 0 {
            byte >> 4
        } else {
            byte & 0x0f
        };
        char::from(HEX_DIGITS[nibble as usize])
    }
}

impl TryFrom<&str> for Oid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Oid::from_str(value)
    }
}

impl FromStr for Oid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let format = match s.len() {
            1..=SHA1_HEX_LENGTH => OidFormat::Sha1,
            SHA256_HEX_LENGTH => OidFormat::Sha256,
            len => {
                anyhow::bail!(
                    "invalid git oid hex length: expected 1..={SHA1_HEX_LENGTH} for SHA-1 or {SHA256_HEX_LENGTH} for SHA-256, got {len}"
                );
            }
        };

        let mut bytes = [0u8; SHA256_BYTE_LENGTH];
        for (index, byte) in s.bytes().enumerate() {
            let digit = decode_hex_digit(byte)
                .ok_or_else(|| anyhow::anyhow!("invalid hex digit at byte {index} for git oid"))?;
            if index % 2 == 0 {
                bytes[index / 2] = digit << 4;
            } else {
                bytes[index / 2] |= digit;
            }
        }

        Ok(Self { bytes, format })
    }
}

fn decode_hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

impl fmt::Debug for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_hex(f)
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.hex_string(self.format.hex_len()))
    }
}

impl<'de> Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Oid>().map_err(serde::de::Error::custom)
    }
}

impl From<Oid> for u32 {
    fn from(oid: Oid) -> Self {
        let mut u32_bytes = [0u8; 4];
        u32_bytes.copy_from_slice(&oid.as_bytes()[..4]);
        u32::from_ne_bytes(u32_bytes)
    }
}

impl From<Oid> for usize {
    fn from(oid: Oid) -> Self {
        let mut u64_bytes = [0u8; 8];
        u64_bytes.copy_from_slice(&oid.as_bytes()[..8]);
        u64::from_ne_bytes(u64_bytes) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_head_contents() {
        let valid = |head: &str| regular_head_contents_are_valid(head.as_bytes());

        // Symbolic refs must point under `refs/`, mirroring git's `validate_headref`.
        assert!(valid("ref: refs/heads/main\n"));
        assert!(valid("ref: refs/heads/main")); // no trailing newline
        assert!(valid("ref: refs/heads/feature/x\r\n")); // CRLF
        assert!(valid("ref:refs/heads/main")); // no space after `ref:`
        assert!(valid("ref:\trefs/heads/main")); // tab after `ref:`
        assert!(valid("ref: refs/heads/.invalid")); // reftable stub HEAD (git creates this)
        assert!(!valid("ref: HEAD")); // not under `refs/`
        assert!(!valid("ref: ../evil")); // not under `refs/`
        assert!(!valid("ref: ")); // empty target
        assert!(!valid("ref:")); // empty target
        // git skips only ASCII space/tab/LF/CR after `ref:`, not other whitespace.
        assert!(!valid("ref:\x0brefs/heads/main")); // vertical tab
        assert!(!valid("ref:\x0crefs/heads/main")); // form feed
        assert!(!valid("ref:\u{a0}refs/heads/main")); // NBSP

        // Detached HEADs are accepted when the contents begin with >= 40 hex.
        let sha1 = "0123456789abcdef0123456789abcdef01234567";
        let sha256 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert_eq!(sha1.len(), SHA1_HEX_LENGTH);
        assert_eq!(sha256.len(), SHA256_HEX_LENGTH);
        assert!(valid(sha1));
        assert!(valid(&format!("{sha1}\n")));
        assert!(valid(sha256));
        // git parses a >= 40-hex prefix and ignores the suffix (forward-hash rule).
        assert!(valid(&format!("{sha1}garbage"))); // 40 hex + junk
        assert!(valid(&"a".repeat(50))); // 41..63 hex
        assert!(valid(&"a".repeat(65))); // > 64 hex
        // A non-UTF-8 suffix after a valid 40-hex prefix is still accepted.
        let mut bytes = sha1.as_bytes().to_vec();
        bytes.push(0xff);
        assert!(regular_head_contents_are_valid(&bytes));

        // Junk / empty / truncated / non-hex is not a valid HEAD.
        assert!(!valid(""));
        assert!(!valid("   "));
        assert!(!valid("garbage"));
        assert!(!valid("0123456789abcdef")); // too short for any oid
        assert!(!valid(&"a".repeat(39))); // 39 hex, one short of SHA-1
        assert!(!valid(&"z".repeat(SHA1_HEX_LENGTH))); // right length, non-hex
        assert!(!valid(&format!(" {sha1}"))); // leading whitespace
    }

    #[test]
    fn validates_head_symlink_target() {
        assert!(head_symlink_target_is_valid(OsStr::new("refs/heads/main")));
        assert!(head_symlink_target_is_valid(OsStr::new(
            "refs/heads/unborn"
        ))); // target need not exist
        assert!(!head_symlink_target_is_valid(OsStr::new("ORIG_HEAD")));
        assert!(!head_symlink_target_is_valid(OsStr::new("HEAD.saved")));
        assert!(!head_symlink_target_is_valid(OsStr::new(
            "../refs/heads/main"
        )));
    }

    #[test]
    fn parses_gitfile_and_commondir() {
        // Requires the literal `gitdir: ` prefix (with the space), like git.
        assert_eq!(
            parse_gitfile(b"gitdir: /repo/.git/worktrees/wt\n").unwrap(),
            PathBuf::from("/repo/.git/worktrees/wt"),
        );
        assert_eq!(
            parse_gitfile(b"gitdir: ../relative\r\n").unwrap(),
            PathBuf::from("../relative"),
        );
        // Spaces beyond the required one are part of the path (git strips only CR/LF).
        assert_eq!(
            parse_gitfile(b"gitdir: /has spaces/x\n").unwrap(),
            PathBuf::from("/has spaces/x"),
        );
        assert!(parse_gitfile(b"gitdir:/no-space").is_err()); // git rejects a missing space
        assert!(parse_gitfile(b"gitdir: \n").is_err()); // empty path
        assert!(parse_gitfile(b"not a gitfile").is_err());

        assert_eq!(
            parse_commondir(b"/repo/.git\n").unwrap(),
            PathBuf::from("/repo/.git"),
        );
        assert_eq!(
            parse_commondir(b"../..\r\n").unwrap(),
            PathBuf::from("../..")
        );
        assert!(parse_commondir(b"").is_err()); // a zero-byte file is invalid
        // git accepts newline-only content as an empty relative path (→ repository dir).
        assert_eq!(parse_commondir(b"\n").unwrap(), PathBuf::new());
    }

    #[test]
    fn parses_short_sha1_oid_by_padding_trailing_nibbles() {
        let oid = "abc1234".parse::<Oid>().expect("failed to parse oid");

        assert_eq!(oid.as_bytes().len(), SHA1_BYTE_LENGTH);
        assert_eq!(oid.display_short(), "abc1234");
        assert_eq!(oid.to_string(), "abc1234000000000000000000000000000000000");
    }

    #[test]
    fn parses_full_sha256_oid() {
        let sha = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let oid = sha.parse::<Oid>().expect("failed to parse oid");

        assert_eq!(oid.as_bytes().len(), SHA256_BYTE_LENGTH);
        assert_eq!(oid.to_string(), sha);
    }

    #[test]
    fn rejects_invalid_oid_lengths() {
        assert!("".parse::<Oid>().is_err());
        assert!("a".repeat(SHA1_HEX_LENGTH + 1).parse::<Oid>().is_err());
        assert!("a".repeat(SHA256_HEX_LENGTH - 1).parse::<Oid>().is_err());
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum RunHook {
    PreCommit,
}

impl RunHook {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PreCommit => "pre-commit",
        }
    }

    pub fn to_proto(&self) -> i32 {
        *self as i32
    }

    pub fn from_proto(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::PreCommit),
            _ => None,
        }
    }
}
