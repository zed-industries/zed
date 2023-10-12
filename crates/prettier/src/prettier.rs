use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use client::{proto, Client};
use collections::{HashMap, HashSet};
use fs::Fs;
use gpui::{AsyncAppContext, ModelHandle};
use language::language_settings::language_settings;
use language::proto::deserialize_diff;
use language::{Buffer, BundledFormatter, Diff};
use lsp::request::Request;
use lsp::{LanguageServer, LanguageServerId};
use node_runtime::NodeRuntime;
use serde::{Deserialize, Serialize};
use util::paths::DEFAULT_PRETTIER_DIR;

pub enum Prettier {
    Local(Local),
    Remote(Remote),
    #[cfg(any(test, feature = "test-support"))]
    Test(TestPrettier),
}

pub struct Local {
    worktree_id: Option<usize>,
    default: bool,
    prettier_dir: PathBuf,
    server: Arc<LanguageServer>,
}

pub struct Remote {
    project_id: u64,
    worktree_id: Option<usize>,
    prettier_dir: PathBuf,
    client: Arc<Client>,
}

#[cfg(any(test, feature = "test-support"))]
pub struct TestPrettier {
    worktree_id: Option<usize>,
    prettier_dir: PathBuf,
    default: bool,
}

#[derive(Debug)]
pub struct LocateStart {
    pub worktree_root_path: Arc<Path>,
    pub starting_path: Arc<Path>,
}

pub const PRETTIER_SERVER_FILE: &str = "prettier_server.js";
pub const PRETTIER_SERVER_JS: &str = include_str!("./prettier_server.js");
const PRETTIER_PACKAGE_NAME: &str = "prettier";
const TAILWIND_PRETTIER_PLUGIN_PACKAGE_NAME: &str = "prettier-plugin-tailwindcss";

impl Prettier {
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

    #[cfg(any(test, feature = "test-support"))]
    pub const FORMAT_SUFFIX: &str = "\nformatted by test prettier";

    pub fn remote(
        project_id: u64,
        worktree_id: Option<usize>,
        prettier_dir: PathBuf,
        client: Arc<Client>,
    ) -> Self {
        Self::Remote(Remote {
            project_id,
            worktree_id,
            prettier_dir,
            client,
        })
    }

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
                        path_component.as_os_str().to_string_lossy() != "node_modules"
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
                                if path_component.as_os_str().to_string_lossy() == "node_modules" {
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

    #[cfg(any(test, feature = "test-support"))]
    pub async fn start(
        worktree_id: Option<usize>,
        _: LanguageServerId,
        prettier_dir: PathBuf,
        _: Arc<dyn NodeRuntime>,
        _: AsyncAppContext,
    ) -> anyhow::Result<Self> {
        Ok(
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(TestPrettier {
                worktree_id,
                default: prettier_dir == DEFAULT_PRETTIER_DIR.as_path(),
                prettier_dir,
            }),
        )
    }

    #[cfg(not(any(test, feature = "test-support")))]
    pub async fn start(
        worktree_id: Option<usize>,
        server_id: LanguageServerId,
        prettier_dir: PathBuf,
        node: Arc<dyn NodeRuntime>,
        cx: AsyncAppContext,
    ) -> anyhow::Result<Self> {
        use lsp::LanguageServerBinary;

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
        Ok(Self::Local(Local {
            worktree_id,
            server,
            default: prettier_dir == DEFAULT_PRETTIER_DIR.as_path(),
            prettier_dir,
        }))
    }

    pub async fn invoke(
        &self,
        buffer: Option<&ModelHandle<Buffer>>,
        buffer_path: Option<PathBuf>,
        method: &str,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<Option<Diff>> {
        match method {
            Format::METHOD => self
                .format(
                    buffer.expect("missing buffer for format invocation"),
                    buffer_path,
                    cx,
                )
                .await
                .context("invoke method")
                .map(Some),
            ClearCache::METHOD => {
                self.clear_cache().await.context("invoke method")?;
                Ok(None)
            }
            unknown => anyhow::bail!("Unknown method {unknown}"),
        }
    }

    pub async fn format(
        &self,
        buffer: &ModelHandle<Buffer>,
        buffer_path: Option<PathBuf>,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<Diff> {
        match self {
            Self::Local(local) => {
                let params = buffer.read_with(cx, |buffer, cx| {
                    let buffer_language = buffer.language();
                    let parsers_with_plugins = buffer_language
                        .into_iter()
                        .flat_map(|language| {
                            language
                                .lsp_adapters()
                                .iter()
                                .flat_map(|adapter| adapter.enabled_formatters())
                                .filter_map(|formatter| match formatter {
                                    BundledFormatter::Prettier {
                                        parser_name,
                                        plugin_names,
                                    } => Some((parser_name, plugin_names)),
                                })
                        })
                        .fold(
                            HashMap::default(),
                            |mut parsers_with_plugins, (parser_name, plugins)| {
                                match parser_name {
                                    Some(parser_name) => parsers_with_plugins
                                        .entry(parser_name)
                                        .or_insert_with(HashSet::default)
                                        .extend(plugins),
                                    None => parsers_with_plugins.values_mut().for_each(|existing_plugins| {
                                        existing_plugins.extend(plugins.iter());
                                    }),
                                }
                                parsers_with_plugins
                            },
                        );

                    let selected_parser_with_plugins = parsers_with_plugins.iter().max_by_key(|(_, plugins)| plugins.len());
                    if parsers_with_plugins.len() > 1 {
                        log::warn!("Found multiple parsers with plugins {parsers_with_plugins:?}, will select only one: {selected_parser_with_plugins:?}");
                    }

                    let prettier_node_modules = self.prettier_dir().join("node_modules");
                    anyhow::ensure!(prettier_node_modules.is_dir(), "Prettier node_modules dir does not exist: {prettier_node_modules:?}");
                    let plugin_name_into_path = |plugin_name: &str| {
                        let prettier_plugin_dir = prettier_node_modules.join(plugin_name);
                        for possible_plugin_path in [
                            prettier_plugin_dir.join("dist").join("index.mjs"),
                            prettier_plugin_dir.join("index.mjs"),
                            prettier_plugin_dir.join("plugin.js"),
                            prettier_plugin_dir.join("index.js"),
                            prettier_plugin_dir,
                        ] {
                            if possible_plugin_path.is_file() {
                                return Some(possible_plugin_path);
                            }
                        }
                        None
                    };
                    let (parser, located_plugins) = match selected_parser_with_plugins {
                        Some((parser, plugins)) => {
                            // Tailwind plugin requires being added last
                            // https://github.com/tailwindlabs/prettier-plugin-tailwindcss#compatibility-with-other-prettier-plugins
                            let mut add_tailwind_back = false;

                            let mut plugins = plugins.into_iter().filter(|&&plugin_name| {
                                if plugin_name == TAILWIND_PRETTIER_PLUGIN_PACKAGE_NAME {
                                    add_tailwind_back = true;
                                    false
                                } else {
                                    true
                                }
                            }).map(|plugin_name| (plugin_name, plugin_name_into_path(plugin_name))).collect::<Vec<_>>();
                            if add_tailwind_back {
                                plugins.push((&TAILWIND_PRETTIER_PLUGIN_PACKAGE_NAME, plugin_name_into_path(TAILWIND_PRETTIER_PLUGIN_PACKAGE_NAME)));
                            }
                            (Some(parser.to_string()), plugins)
                        },
                        None => (None, Vec::new()),
                    };

                    let prettier_options = if self.is_default() {
                        let language_settings = language_settings(buffer_language, buffer.file(), cx);
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

                    let plugins = located_plugins.into_iter().filter_map(|(plugin_name, located_plugin_path)| {
                        match located_plugin_path {
                            Some(path) => Some(path),
                            None => {
                                log::error!("Have not found plugin path for {plugin_name:?} inside {prettier_node_modules:?}");
                                None},
                        }
                    }).collect();
                    log::debug!("Formatting file {:?} with prettier, plugins :{plugins:?}, options: {prettier_options:?}", buffer.file().map(|f| f.full_path(cx)));

                    anyhow::Ok(FormatParams {
                        text: buffer.text(),
                        options: FormatOptions {
                            parser,
                            plugins,
                            path: buffer_path,
                            prettier_options,
                        },
                    })
                }).context("prettier params calculation")?;
                let response = local
                    .server
                    .request::<Format>(params)
                    .await
                    .context("prettier format request")?;
                let diff_task = buffer.read_with(cx, |buffer, cx| buffer.diff(response.text, cx));
                Ok(diff_task.await)
            }
            Self::Remote(remote) => buffer
                .read_with(cx, |buffer, _| {
                    remote.client.request(proto::InvokePrettierForBuffer {
                        buffer_id: Some(buffer.remote_id()),
                        worktree_id: self.worktree_id().map(|id| id as u64),
                        method: Format::METHOD.to_string(),
                        project_id: remote.project_id,
                        prettier_path: remote.prettier_dir.to_string_lossy().to_string(),
                    })
                })
                .await
                .context("prettier diff invoke")?
                .diff
                .map(deserialize_diff)
                .context("missing diff after prettier diff invocation"),
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(_) => Ok(buffer
                .read_with(cx, |buffer, cx| {
                    let formatted_text = buffer.text() + Self::FORMAT_SUFFIX;
                    buffer.diff(formatted_text, cx)
                })
                .await),
        }
    }

    pub async fn clear_cache(&self) -> anyhow::Result<()> {
        match self {
            Self::Local(local) => local
                .server
                .request::<ClearCache>(())
                .await
                .context("prettier clear cache"),
            Self::Remote(remote) => remote
                .client
                .request(proto::InvokePrettierForBuffer {
                    buffer_id: None,
                    worktree_id: self.worktree_id().map(|id| id as u64),
                    method: ClearCache::METHOD.to_string(),
                    project_id: remote.project_id,
                    prettier_path: remote.prettier_dir.to_string_lossy().to_string(),
                })
                .await
                .map(|response| {
                    debug_assert!(
                        response.diff.is_none(),
                        "Cleare cache invocation returned diff data"
                    )
                })
                .context("prettier invoke clear cache"),
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(_) => Ok(()),
        }
    }

    pub fn server(&self) -> Option<&Arc<LanguageServer>> {
        match self {
            Self::Local(local) => Some(&local.server),
            Self::Remote(_) => None,
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(_) => None,
        }
    }

    pub fn is_default(&self) -> bool {
        match self {
            Self::Local(local) => local.default,
            Self::Remote(_) => false,
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(test_prettier) => test_prettier.default,
        }
    }

    pub fn prettier_dir(&self) -> &Path {
        match self {
            Self::Local(local) => &local.prettier_dir,
            Self::Remote(remote) => &remote.prettier_dir,
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(test_prettier) => &test_prettier.prettier_dir,
        }
    }

    pub fn worktree_id(&self) -> Option<usize> {
        match self {
            Self::Local(local) => local.worktree_id,
            Self::Remote(remote) => remote.worktree_id,
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(test_prettier) => test_prettier.worktree_id,
        }
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
    plugins: Vec<PathBuf>,
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
