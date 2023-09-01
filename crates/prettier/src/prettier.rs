pub use std::path::{Path, PathBuf};
pub use std::sync::Arc;

use fs::Fs;
use gpui::ModelHandle;
use language::{Buffer, Diff};

pub struct Prettier {
    _private: (),
}

type NodeRuntime = ();

impl Prettier {
    // This was taken from the prettier-vscode extension.
    pub const CONFIG_FILE_NAMES: &'static [&'static str] = &[
        ".prettierrc",
        ".prettierrc.json",
        ".prettierrc.json5",
        ".prettierrc.yaml",
        ".prettierrc.yml",
        ".prettierrc.toml",
        ".prettierrc.js",
        ".prettierrc.cjs",
        "package.json",
        "prettier.config.js",
        "prettier.config.cjs",
        ".editorconfig",
    ];

    pub async fn locate(starting_path: Option<&Path>, fs: Arc<dyn Fs>) -> PathBuf {
        todo!()
    }

    pub async fn start(prettier_path: &Path, node: Arc<NodeRuntime>) -> anyhow::Result<Self> {
        todo!()
    }

    pub async fn format(&self, buffer: &ModelHandle<Buffer>) -> anyhow::Result<Diff> {
        todo!()
    }

    pub async fn clear_cache(&self) -> anyhow::Result<()> {
        todo!()
    }
}
