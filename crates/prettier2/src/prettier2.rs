use anyhow::Context;
use collections::HashMap;
use fs::Fs;
use gpui::{AsyncAppContext, Model};
use language::{language_settings::language_settings, Buffer, Diff};
use lsp::{LanguageServer, LanguageServerId};
use node_runtime::NodeRuntime;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::paths::DEFAULT_PRETTIER_DIR;

pub enum Prettier {
    Real(RealPrettier),
    #[cfg(any(test, feature = "test-support"))]
    Test(TestPrettier),
}

pub struct RealPrettier {
    worktree_id: Option<usize>,
    default: bool,
    prettier_dir: PathBuf,
    server: Arc<LanguageServer>,
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

#[cfg(any(test, feature = "test-support"))]
pub const FORMAT_SUFFIX: &str = "\nformatted by test prettier";

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

    pub async fn locate(
        starting_path: Option<LocateStart>,
        fs: Arc<dyn Fs>,
    ) -> anyhow::Result<PathBuf> {
        fn is_node_modules(path_component: &std::path::Component<'_>) -> bool {
            path_component.as_os_str().to_string_lossy() == "node_modules"
        }

        let paths_to_check = match starting_path.as_ref() {
            Some(starting_path) => {
                let worktree_root = starting_path
                    .worktree_root_path
                    .components()
                    .into_iter()
                    .take_while(|path_component| !is_node_modules(path_component))
                    .collect::<PathBuf>();
                if worktree_root != starting_path.worktree_root_path.as_ref() {
                    vec![worktree_root]
                } else {
                    if starting_path.starting_path.as_ref() == Path::new("") {
                        worktree_root
                            .parent()
                            .map(|path| vec![path.to_path_buf()])
                            .unwrap_or_default()
                    } else {
                        let file_to_format = starting_path.starting_path.as_ref();
                        let mut paths_to_check = VecDeque::new();
                        let mut current_path = worktree_root;
                        for path_component in file_to_format.components().into_iter() {
                            let new_path = current_path.join(path_component);
                            let old_path = std::mem::replace(&mut current_path, new_path);
                            paths_to_check.push_front(old_path);
                            if is_node_modules(&path_component) {
                                break;
                            }
                        }
                        Vec::from(paths_to_check)
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

        let executor = cx.background_executor().clone();
        anyhow::ensure!(
            prettier_dir.is_dir(),
            "Prettier dir {prettier_dir:?} is not a directory"
        );
        let prettier_server = DEFAULT_PRETTIER_DIR.join(PRETTIER_SERVER_FILE);
        anyhow::ensure!(
            prettier_server.is_file(),
            "no prettier server package found at {prettier_server:?}"
        );

        let node_path = executor
            .spawn(async move { node.binary_path().await })
            .await?;
        let server = LanguageServer::new(
            Arc::new(parking_lot::Mutex::new(None)),
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
        let server = executor
            .spawn(server.initialize(None))
            .await
            .context("prettier server initialization")?;
        Ok(Self::Real(RealPrettier {
            worktree_id,
            server,
            default: prettier_dir == DEFAULT_PRETTIER_DIR.as_path(),
            prettier_dir,
        }))
    }

    pub async fn format(
        &self,
        buffer: &Model<Buffer>,
        buffer_path: Option<PathBuf>,
        cx: &mut AsyncAppContext,
    ) -> anyhow::Result<Diff> {
        match self {
            Self::Real(local) => {
                let params = buffer
                    .update(cx, |buffer, cx| {
                        let buffer_language = buffer.language();
                        let parser_with_plugins = buffer_language.and_then(|l| {
                            let prettier_parser = l.prettier_parser_name()?;
                            let mut prettier_plugins = l
                                .lsp_adapters()
                                .iter()
                                .flat_map(|adapter| adapter.prettier_plugins())
                                .collect::<Vec<_>>();
                            prettier_plugins.dedup();
                            Some((prettier_parser, prettier_plugins))
                        });

                        let prettier_node_modules = self.prettier_dir().join("node_modules");
                        anyhow::ensure!(
                            prettier_node_modules.is_dir(),
                            "Prettier node_modules dir does not exist: {prettier_node_modules:?}"
                        );
                        let plugin_name_into_path = |plugin_name: &str| {
                            let prettier_plugin_dir = prettier_node_modules.join(plugin_name);
                            for possible_plugin_path in [
                                prettier_plugin_dir.join("dist").join("index.mjs"),
                                prettier_plugin_dir.join("dist").join("index.js"),
                                prettier_plugin_dir.join("dist").join("plugin.js"),
                                prettier_plugin_dir.join("index.mjs"),
                                prettier_plugin_dir.join("index.js"),
                                prettier_plugin_dir.join("plugin.js"),
                                prettier_plugin_dir,
                            ] {
                                if possible_plugin_path.is_file() {
                                    return Some(possible_plugin_path);
                                }
                            }
                            None
                        };
                        let (parser, located_plugins) = match parser_with_plugins {
                            Some((parser, plugins)) => {
                                // Tailwind plugin requires being added last
                                // https://github.com/tailwindlabs/prettier-plugin-tailwindcss#compatibility-with-other-prettier-plugins
                                let mut add_tailwind_back = false;

                                let mut plugins = plugins
                                    .into_iter()
                                    .filter(|&&plugin_name| {
                                        if plugin_name == TAILWIND_PRETTIER_PLUGIN_PACKAGE_NAME {
                                            add_tailwind_back = true;
                                            false
                                        } else {
                                            true
                                        }
                                    })
                                    .map(|plugin_name| {
                                        (plugin_name, plugin_name_into_path(plugin_name))
                                    })
                                    .collect::<Vec<_>>();
                                if add_tailwind_back {
                                    plugins.push((
                                        &TAILWIND_PRETTIER_PLUGIN_PACKAGE_NAME,
                                        plugin_name_into_path(
                                            TAILWIND_PRETTIER_PLUGIN_PACKAGE_NAME,
                                        ),
                                    ));
                                }
                                (Some(parser.to_string()), plugins)
                            }
                            None => (None, Vec::new()),
                        };

                        let prettier_options = if self.is_default() {
                            let language_settings =
                                language_settings(buffer_language, buffer.file(), cx);
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

                        let plugins = located_plugins
                            .into_iter()
                            .filter_map(|(plugin_name, located_plugin_path)| {
                                match located_plugin_path {
                                    Some(path) => Some(path),
                                    None => {
                                        log::error!(
                                            "Have not found plugin path for {:?} inside {:?}",
                                            plugin_name,
                                            prettier_node_modules
                                        );
                                        None
                                    }
                                }
                            })
                            .collect();
                        log::debug!(
                            "Formatting file {:?} with prettier, plugins :{:?}, options: {:?}",
                            plugins,
                            prettier_options,
                            buffer.file().map(|f| f.full_path(cx))
                        );

                        anyhow::Ok(FormatParams {
                            text: buffer.text(),
                            options: FormatOptions {
                                parser,
                                plugins,
                                path: buffer_path,
                                prettier_options,
                            },
                        })
                    })?
                    .context("prettier params calculation")?;
                let response = local
                    .server
                    .request::<Format>(params)
                    .await
                    .context("prettier format request")?;
                let diff_task = buffer.update(cx, |buffer, cx| buffer.diff(response.text, cx))?;
                Ok(diff_task.await)
            }
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(_) => Ok(buffer
                .update(cx, |buffer, cx| {
                    let formatted_text = buffer.text() + FORMAT_SUFFIX;
                    buffer.diff(formatted_text, cx)
                })?
                .await),
        }
    }

    pub async fn clear_cache(&self) -> anyhow::Result<()> {
        match self {
            Self::Real(local) => local
                .server
                .request::<ClearCache>(())
                .await
                .context("prettier clear cache"),
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(_) => Ok(()),
        }
    }

    pub fn server(&self) -> Option<&Arc<LanguageServer>> {
        match self {
            Self::Real(local) => Some(&local.server),
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(_) => None,
        }
    }

    pub fn is_default(&self) -> bool {
        match self {
            Self::Real(local) => local.default,
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(test_prettier) => test_prettier.default,
        }
    }

    pub fn prettier_dir(&self) -> &Path {
        match self {
            Self::Real(local) => &local.prettier_dir,
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(test_prettier) => &test_prettier.prettier_dir,
        }
    }

    pub fn worktree_id(&self) -> Option<usize> {
        match self {
            Self::Real(local) => local.worktree_id,
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
