pub mod blame;
pub mod commit;
mod hosting_provider;
mod remote;
pub mod repository;
pub mod status;

pub use crate::hosting_provider::*;
pub use crate::remote::*;
use anyhow::{Context as _, Result, anyhow};
pub use git2 as libgit;
use gpui::action_with_deprecated_aliases;
use gpui::actions;
use gpui::impl_action_with_deprecated_aliases;
pub use repository::WORK_DIRECTORY_REPO_PATH;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

pub static DOT_GIT: LazyLock<&'static OsStr> = LazyLock::new(|| OsStr::new(".git"));
pub static GITIGNORE: LazyLock<&'static OsStr> = LazyLock::new(|| OsStr::new(".gitignore"));
pub static FSMONITOR_DAEMON: LazyLock<&'static OsStr> =
    LazyLock::new(|| OsStr::new("fsmonitor--daemon"));
pub static LFS_DIR: LazyLock<&'static OsStr> = LazyLock::new(|| OsStr::new("lfs"));
pub static COMMIT_MESSAGE: LazyLock<&'static OsStr> =
    LazyLock::new(|| OsStr::new("COMMIT_EDITMSG"));
pub static INDEX_LOCK: LazyLock<&'static OsStr> = LazyLock::new(|| OsStr::new("index.lock"));

actions!(
    git,
    [
        // per-hunk
        ToggleStaged,
        StageAndNext,
        UnstageAndNext,
        // per-file
        StageFile,
        UnstageFile,
        // repo-wide
        StageAll,
        UnstageAll,
        RestoreTrackedFiles,
        TrashUntrackedFiles,
        Uncommit,
        Push,
        ForcePush,
        Pull,
        Fetch,
        Commit,
        Amend,
        Cancel,
        ExpandCommitEditor,
        GenerateCommitMessage,
        Init,
    ]
);

#[derive(Clone, Debug, Default, PartialEq, Deserialize, JsonSchema)]
pub struct RestoreFile {
    #[serde(default)]
    pub skip_prompt: bool,
}

impl_action_with_deprecated_aliases!(git, RestoreFile, ["editor::RevertFile"]);
action_with_deprecated_aliases!(git, Restore, ["editor::RevertSelectedHunks"]);
action_with_deprecated_aliases!(git, Blame, ["editor::ToggleGitBlame"]);

/// The length of a Git short SHA.
pub const SHORT_SHA_LENGTH: usize = 7;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Oid(libgit::Oid);

impl Oid {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let oid = libgit::Oid::from_bytes(bytes).context("failed to parse bytes into git oid")?;
        Ok(Self(oid))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns this [`Oid`] as a short SHA.
    pub fn display_short(&self) -> String {
        self.to_string().chars().take(SHORT_SHA_LENGTH).collect()
    }
}

impl FromStr for Oid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        libgit::Oid::from_str(s)
            .map_err(|error| anyhow!("failed to parse git oid: {}", error))
            .map(Self)
    }
}

impl fmt::Debug for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Oid>().map_err(serde::de::Error::custom)
    }
}

impl Default for Oid {
    fn default() -> Self {
        Self(libgit::Oid::zero())
    }
}

impl From<Oid> for u32 {
    fn from(oid: Oid) -> Self {
        let bytes = oid.0.as_bytes();
        debug_assert!(bytes.len() > 4);

        let mut u32_bytes: [u8; 4] = [0; 4];
        u32_bytes.copy_from_slice(&bytes[..4]);

        u32::from_ne_bytes(u32_bytes)
    }
}

impl From<Oid> for usize {
    fn from(oid: Oid) -> Self {
        let bytes = oid.0.as_bytes();
        debug_assert!(bytes.len() > 8);

        let mut u64_bytes: [u8; 8] = [0; 8];
        u64_bytes.copy_from_slice(&bytes[..8]);

        u64::from_ne_bytes(u64_bytes) as usize
    }
}
