use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use collections::HashMap;
use fs::Fs;
use gpui::{AsyncAppContext, ModelHandle};
use language::language_settings::language_settings;
use language::{Buffer, BundledFormatter, Diff};
use lsp::{LanguageServer, LanguageServerBinary, LanguageServerId};
use node_runtime::NodeRuntime;
use serde::{Deserialize, Serialize};
use util::paths::DEFAULT_PRETTIER_DIR;

pub struct Prettier {
    worktree_id: Option<usize>,
    default: bool,
    prettier_dir: PathBuf,
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
            None => Ok(DEFAULT_PRETTIER_DIR.to_path_buf()),
        }
    }

    pub async fn start(
        worktree_id: Option<usize>,
        server_id: LanguageServerId,
        prettier_dir: PathBuf,
        node: Arc<dyn NodeRuntime>,
        cx: AsyncAppContext,
    ) -> anyhow::Result<Self> {
        let backgroud = cx.background();
        anyhow::ensure!(
            prettier_dir.is_dir(),
            "Prettier dir {prettier_dir:?} is not a directory"
        );
        let prettier_server = DEFAULT_PRETTIER_DIR.join(PRETTIER_SERVER_FILE);
        anyhow::ensure!(
            prettier_server.is_file(),
            "no prettier server package found at {prettier_server:?}"
        );

        let node_path = backgroud
            .spawn(async move { node.binary_path().await })
            .await?;
        let server = LanguageServer::new(
            server_id,
            LanguageServerBinary {
                path: node_path,
                arguments: vec![prettier_server.into(), prettier_dir.as_path().into()],
            },
            Path::new("/"),
            None,
            cx,
        )
        .context("prettier server creation")?;
        let server = backgroud
            .spawn(server.initialize(None))
            .await
            .context("prettier server initialization")?;
        Ok(Self {
            worktree_id,
            server,
            default: prettier_dir == DEFAULT_PRETTIER_DIR.as_path(),
            prettier_dir,
        })
    }

    pub async fn format(
        &self,
        buffer: &ModelHandle<Buffer>,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<Diff> {
        let params = buffer.read_with(cx, |buffer, cx| {
            let buffer_file = buffer.file();
            let buffer_language = buffer.language();
            let path = buffer_file
                .map(|file| file.full_path(cx))
                .map(|path| path.to_path_buf());
            let parser = buffer_language.and_then(|language| {
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

            let prettier_options = if self.default {
                let language_settings = language_settings(buffer_language, buffer_file, cx);
                let mut options = language_settings.prettier.clone();
                if !options.contains_key("tabWidth") {
                    options.insert(
                        "tabWidth".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(
                            language_settings.tab_size.get(),
                        )),
                    );
                }
                if !options.contains_key("printWidth") {
                    options.insert(
                        "printWidth".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(
                            language_settings.preferred_line_length,
                        )),
                    );
                }
                Some(options)
            } else {
                None
            };

            FormatParams {
                text: buffer.text(),
                options: FormatOptions {
                    parser,
                    // TODO kb is not absolute now
                    path,
                    prettier_options,
                },
            }
        });
        let response = self
            .server
            .request::<Format>(params)
            .await
            .context("prettier format request")?;
        let diff_task = buffer.read_with(cx, |buffer, cx| buffer.diff(response.text, cx));
        Ok(diff_task.await)
    }

    pub async fn clear_cache(&self) -> anyhow::Result<()> {
        self.server
            .request::<ClearCache>(())
            .await
            .context("prettier clear cache")
    }

    pub fn server(&self) -> &Arc<LanguageServer> {
        &self.server
    }

    pub fn is_default(&self) -> bool {
        self.default
    }

    pub fn prettier_dir(&self) -> &Path {
        &self.prettier_dir
    }

    pub fn worktree_id(&self) -> Option<usize> {
        self.worktree_id
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

enum Format {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormatParams {
    text: String,
    options: FormatOptions,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormatOptions {
    parser: Option<String>,
    #[serde(rename = "filepath")]
    path: Option<PathBuf>,
    prettier_options: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormatResult {
    text: String,
}

impl lsp::request::Request for Format {
    type Params = FormatParams;
    type Result = FormatResult;
    const METHOD: &'static str = "prettier/format";
}

enum ClearCache {}

impl lsp::request::Request for ClearCache {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "prettier/clear_cache";
}
