use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use fs::Fs;
use gpui::{AsyncAppContext, ModelHandle, Task};
use language::{Buffer, BundledFormatter, Diff};
use lsp::{LanguageServer, LanguageServerBinary, LanguageServerId};
use node_runtime::NodeRuntime;
use serde::{Deserialize, Serialize};
use util::paths::DEFAULT_PRETTIER_DIR;

pub struct Prettier {
    server: Arc<LanguageServer>,
}

#[derive(Debug)]
pub struct LocateStart {
    pub worktree_root_path: Arc<Path>,
    pub starting_path: Arc<Path>,
}

pub const PRETTIER_SERVER_FILE: &str = "prettier_server.js";
pub const PRETTIER_SERVER_JS: &str = include_str!("./prettier_server.js");
const PRETTIER_PACKAGE_NAME: &str = "prettier";

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
        let paths_to_check = match starting_path.as_ref() {
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

        match find_closest_prettier_dir(paths_to_check, fs.as_ref())
            .await
            .with_context(|| format!("finding prettier starting with {starting_path:?}"))?
        {
            Some(prettier_dir) => Ok(prettier_dir),
            None => Ok(util::paths::DEFAULT_PRETTIER_DIR.to_path_buf()),
        }
    }

    pub fn start(
        prettier_dir: PathBuf,
        node: Arc<dyn NodeRuntime>,
        cx: AsyncAppContext,
    ) -> Task<anyhow::Result<Self>> {
        cx.spawn(|cx| async move {
            anyhow::ensure!(
                prettier_dir.is_dir(),
                "Prettier dir {prettier_dir:?} is not a directory"
            );
            let prettier_server = DEFAULT_PRETTIER_DIR.join(PRETTIER_SERVER_FILE);
            anyhow::ensure!(
                prettier_server.is_file(),
                "no prettier server package found at {prettier_server:?}"
            );

            let node_path = node.binary_path().await?;
            let server = LanguageServer::new(
                LanguageServerId(0),
                LanguageServerBinary {
                    path: node_path,
                    arguments: vec![prettier_server.into(), prettier_dir.into()],
                },
                Path::new("/"),
                None,
                cx,
            )
            .context("prettier server creation")?;
            let server = server
                .initialize(None)
                .await
                .context("prettier server initialization")?;
            Ok(Self { server })
        })
    }

    pub async fn format(
        &self,
        buffer: &ModelHandle<Buffer>,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<Diff> {
        let params = buffer.read_with(cx, |buffer, cx| {
            let path = buffer
                .file()
                .map(|file| file.full_path(cx))
                .map(|path| path.to_path_buf());
            let parser = buffer.language().and_then(|language| {
                language
                    .lsp_adapters()
                    .iter()
                    .flat_map(|adapter| adapter.enabled_formatters())
                    .find_map(|formatter| match formatter {
                        BundledFormatter::Prettier { parser_name, .. } => {
                            Some(parser_name.to_string())
                        }
                    })
            });
            PrettierFormatParams {
                text: buffer.text(),
                options: FormatOptions { parser, path },
            }
        });
        let response = self
            .server
            .request::<PrettierFormat>(params)
            .await
            .context("prettier format request")?;
        let diff_task = buffer.read_with(cx, |buffer, cx| buffer.diff(response.text, cx));
        Ok(diff_task.await)
    }

    pub async fn clear_cache(&self) -> anyhow::Result<()> {
        todo!()
    }
}

async fn find_closest_prettier_dir(
    paths_to_check: Vec<PathBuf>,
    fs: &dyn Fs,
) -> anyhow::Result<Option<PathBuf>> {
    for path in paths_to_check {
        let possible_package_json = path.join("package.json");
        if let Some(package_json_metadata) = fs
            .metadata(&possible_package_json)
            .await
            .with_context(|| format!("Fetching metadata for {possible_package_json:?}"))?
        {
            if !package_json_metadata.is_dir && !package_json_metadata.is_symlink {
                let package_json_contents = fs
                    .load(&possible_package_json)
                    .await
                    .with_context(|| format!("reading {possible_package_json:?} file contents"))?;
                if let Ok(json_contents) = serde_json::from_str::<HashMap<String, serde_json::Value>>(
                    &package_json_contents,
                ) {
                    if let Some(serde_json::Value::Object(o)) = json_contents.get("dependencies") {
                        if o.contains_key(PRETTIER_PACKAGE_NAME) {
                            return Ok(Some(path));
                        }
                    }
                    if let Some(serde_json::Value::Object(o)) = json_contents.get("devDependencies")
                    {
                        if o.contains_key(PRETTIER_PACKAGE_NAME) {
                            return Ok(Some(path));
                        }
                    }
                }
            }
        }

        let possible_node_modules_location = path.join("node_modules").join(PRETTIER_PACKAGE_NAME);
        if let Some(node_modules_location_metadata) = fs
            .metadata(&possible_node_modules_location)
            .await
            .with_context(|| format!("fetching metadata for {possible_node_modules_location:?}"))?
        {
            if node_modules_location_metadata.is_dir {
                return Ok(Some(path));
            }
        }
    }
    Ok(None)
}

enum PrettierFormat {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrettierFormatParams {
    text: String,
    options: FormatOptions,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormatOptions {
    parser: Option<String>,
    path: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrettierFormatResult {
    text: String,
}

impl lsp::request::Request for PrettierFormat {
    type Params = PrettierFormatParams;
    type Result = PrettierFormatResult;
    const METHOD: &'static str = "prettier/format";
}
