use std::collections::VecDeque;
pub use std::path::{Path, PathBuf};
pub use std::sync::Arc;

use anyhow::Context;
use fs::Fs;
use gpui::ModelHandle;
use language::{Buffer, Diff};

pub struct Prettier {
    _private: (),
}

pub struct NodeRuntime;

#[derive(Debug)]
pub struct LocateStart {
    pub worktree_root_path: Arc<Path>,
    pub starting_path: Arc<Path>,
}

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

    pub async fn locate(
        starting_path: Option<LocateStart>,
        fs: Arc<dyn Fs>,
    ) -> anyhow::Result<PathBuf> {
        let paths_to_check = match starting_path {
            Some(starting_path) => {
                let worktree_root = starting_path
                    .worktree_root_path
                    .components()
                    .into_iter()
                    .take_while(|path_component| {
                        path_component.as_os_str().to_str() != Some("node_modules")
                    })
                    .collect::<PathBuf>();

                if worktree_root != starting_path.worktree_root_path.as_ref() {
                    vec![worktree_root]
                } else {
                    let (worktree_root_metadata, start_path_metadata) = if starting_path
                        .starting_path
                        .as_ref()
                        == Path::new("")
                    {
                        let worktree_root_data =
                            fs.metadata(&worktree_root).await.with_context(|| {
                                format!(
                                    "FS metadata fetch for worktree root path {worktree_root:?}",
                                )
                            })?;
                        (worktree_root_data.unwrap_or_else(|| {
                            panic!("cannot query prettier for non existing worktree root at {worktree_root_data:?}")
                        }), None)
                    } else {
                        let full_starting_path = worktree_root.join(&starting_path.starting_path);
                        let (worktree_root_data, start_path_data) = futures::try_join!(
                            fs.metadata(&worktree_root),
                            fs.metadata(&full_starting_path),
                        )
                        .with_context(|| {
                            format!("FS metadata fetch for starting path {full_starting_path:?}",)
                        })?;
                        (
                            worktree_root_data.unwrap_or_else(|| {
                                panic!("cannot query prettier for non existing worktree root at {worktree_root_data:?}")
                            }),
                            start_path_data,
                        )
                    };

                    match start_path_metadata {
                        Some(start_path_metadata) => {
                            anyhow::ensure!(worktree_root_metadata.is_dir,
                                "For non-empty start path, worktree root {starting_path:?} should be a directory");
                            anyhow::ensure!(
                                !start_path_metadata.is_dir,
                                "For non-empty start path, it should not be a directory {starting_path:?}"
                            );
                            anyhow::ensure!(
                                !start_path_metadata.is_symlink,
                                "For non-empty start path, it should not be a symlink {starting_path:?}"
                            );

                            let file_to_format = starting_path.starting_path.as_ref();
                            let mut paths_to_check = VecDeque::from(vec![worktree_root.clone()]);
                            let mut current_path = worktree_root;
                            for path_component in file_to_format.components().into_iter() {
                                current_path = current_path.join(path_component);
                                paths_to_check.push_front(current_path.clone());
                                if path_component.as_os_str().to_str() == Some("node_modules") {
                                    break;
                                }
                            }
                            paths_to_check.pop_front(); // last one is the file itself or node_modules, skip it
                            Vec::from(paths_to_check)
                        }
                        None => {
                            anyhow::ensure!(
                                !worktree_root_metadata.is_dir,
                                "For empty start path, worktree root should not be a directory {starting_path:?}"
                            );
                            anyhow::ensure!(
                                !worktree_root_metadata.is_symlink,
                                "For empty start path, worktree root should not be a symlink {starting_path:?}"
                            );
                            worktree_root
                                .parent()
                                .map(|path| vec![path.to_path_buf()])
                                .unwrap_or_default()
                        }
                    }
                }
            }
            None => Vec::new(),
        };

        if dbg!(paths_to_check).is_empty() {
            // TODO kb return the default prettier, how, without state?
        } else {
            // TODO kb now check all paths to check for prettier
        }
        Ok(PathBuf::new())
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
