pub mod blame;
pub mod commit;
mod hosting_provider;
mod remote;
pub mod repository;
pub mod stash;
pub mod status;

pub use crate::hosting_provider::*;
pub use crate::remote::*;
use anyhow::Result;
use gpui::{Action, actions};
pub use repository::RemoteCommandOutput;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

pub const DOT_GIT: &str = ".git";
pub const GITIGNORE: &str = ".gitignore";
pub const FSMONITOR_DAEMON: &str = "fsmonitor--daemon";
pub const LFS_DIR: &str = "lfs";
pub const COMMIT_MESSAGE: &str = "COMMIT_EDITMSG";
pub const INDEX_LOCK: &str = "index.lock";
pub const REPO_EXCLUDE: &str = "info/exclude";

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
        /// Shows the git history for the current file.
        FileHistory,
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
        /// Generates a commit message using AI.
        GenerateCommitMessage,
        /// Initializes a new git repository.
        Init,
        /// Opens all modified files in the editor.
        OpenModifiedFiles,
        /// Clones a repository.
        Clone,
        /// Adds a file to .gitignore.
        AddToGitignore,
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

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Oid([u8; 20]);

impl Oid {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let bytes: [u8; 20] = bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("expected 20 bytes for git oid, got {}", bytes.len()))?;
        Ok(Self(bytes))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        let mut bytes = [0u8; 20];
        rng.fill(&mut bytes);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.0 == [0u8; 20]
    }

    /// Returns this [`Oid`] as a short SHA.
    pub fn display_short(&self) -> String {
        hex::encode(self.0)[..SHORT_SHA_LENGTH].to_string()
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
        anyhow::ensure!(
            !s.is_empty() && s.len() <= 40,
            "invalid hex length {} for git oid",
            s.len()
        );
        let mut padded = [b'0'; 40];
        padded[..s.len()].copy_from_slice(s.as_bytes());
        let mut bytes = [0u8; 20];
        hex::decode_to_slice(&padded, &mut bytes)?;
        Ok(Self(bytes))
    }
}

impl fmt::Debug for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(self.0))
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.0))
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
        u32_bytes.copy_from_slice(&oid.0[..4]);
        u32::from_ne_bytes(u32_bytes)
    }
}

impl From<Oid> for usize {
    fn from(oid: Oid) -> Self {
        let mut u64_bytes = [0u8; 8];
        u64_bytes.copy_from_slice(&oid.0[..8]);
        u64::from_ne_bytes(u64_bytes) as usize
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
