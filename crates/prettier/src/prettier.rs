use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use collections::{HashMap, HashSet};
use fs::Fs;
use gpui::{AsyncAppContext, ModelHandle};
use language::language_settings::language_settings;
use language::{Buffer, Diff};
use lsp::{LanguageServer, LanguageServerId};
use node_runtime::NodeRuntime;
use serde::{Deserialize, Serialize};
use util::paths::{PathMatcher, DEFAULT_PRETTIER_DIR};

pub enum Prettier {
    Real(RealPrettier),
    #[cfg(any(test, feature = "test-support"))]
    Test(TestPrettier),
}

pub struct RealPrettier {
    default: bool,
    prettier_dir: PathBuf,
    server: Arc<LanguageServer>,
}

#[cfg(any(test, feature = "test-support"))]
pub struct TestPrettier {
    prettier_dir: PathBuf,
    default: bool,
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

    pub async fn locate_prettier_installation(
        fs: &dyn Fs,
        installed_prettiers: &HashSet<PathBuf>,
        locate_from: &Path,
    ) -> anyhow::Result<Option<PathBuf>> {
        let mut path_to_check = locate_from
            .components()
            .take_while(|component| !is_node_modules(component))
            .collect::<PathBuf>();
        let path_to_check_metadata = fs
            .metadata(&path_to_check)
            .await
            .with_context(|| format!("failed to get metadata for initial path {path_to_check:?}"))?
            .with_context(|| format!("empty metadata for initial path {path_to_check:?}"))?;
        if !path_to_check_metadata.is_dir {
            path_to_check.pop();
        }

        let mut project_path_with_prettier_dependency = None;
        loop {
            if installed_prettiers.contains(&path_to_check) {
                log::debug!("Found prettier path {path_to_check:?} in installed prettiers");
                return Ok(Some(path_to_check));
            } else if let Some(package_json_contents) =
                read_package_json(fs, &path_to_check).await?
            {
                if has_prettier_in_package_json(&package_json_contents) {
                    if has_prettier_in_node_modules(fs, &path_to_check).await? {
                        log::debug!("Found prettier path {path_to_check:?} in both package.json and node_modules");
                        return Ok(Some(path_to_check));
                    } else if project_path_with_prettier_dependency.is_none() {
                        project_path_with_prettier_dependency = Some(path_to_check.clone());
                    }
                } else {
                    match package_json_contents.get("workspaces") {
                        Some(serde_json::Value::Array(workspaces)) => {
                            match &project_path_with_prettier_dependency {
                                Some(project_path_with_prettier_dependency) => {
                                    let subproject_path = project_path_with_prettier_dependency.strip_prefix(&path_to_check).expect("traversing path parents, should be able to strip prefix");
                                    if workspaces.iter().filter_map(|value| {
                                        if let serde_json::Value::String(s) = value {
                                            Some(s.clone())
                                        } else {
                                            log::warn!("Skipping non-string 'workspaces' value: {value:?}");
                                            None
                                        }
                                    }).any(|workspace_definition| {
                                        if let Some(path_matcher) = PathMatcher::new(&workspace_definition).ok() {
                                            path_matcher.is_match(subproject_path)
                                        } else {
                                            workspace_definition == subproject_path.to_string_lossy()
                                        }
                                    }) {
                                        anyhow::ensure!(has_prettier_in_node_modules(fs, &path_to_check).await?, "Found prettier path {path_to_check:?} in the workspace root for project in {project_path_with_prettier_dependency:?}, but it's not installed into workspace root's node_modules");
                                        log::info!("Found prettier path {path_to_check:?} in the workspace root for project in {project_path_with_prettier_dependency:?}");
                                        return Ok(Some(path_to_check));
                                    } else {
                                        log::warn!("Skipping path {path_to_check:?} that has prettier in its 'node_modules' subdirectory, but is not included in its package.json workspaces {workspaces:?}");
                                    }
                                }
                                None => {
                                    log::warn!("Skipping path {path_to_check:?} that has prettier in its 'node_modules' subdirectory, but has no prettier in its package.json");
                                }
                            }
                        },
                        Some(unknown) => log::error!("Failed to parse workspaces for {path_to_check:?} from package.json, got {unknown:?}. Skipping."),
                        None => log::warn!("Skipping path {path_to_check:?} that has no prettier dependency and no workspaces section in its package.json"),
                    }
                }
            }

            if !path_to_check.pop() {
                match project_path_with_prettier_dependency {
                    Some(closest_prettier_discovered) => {
                        anyhow::bail!("No prettier found in node_modules for ancestors of {locate_from:?}, but discovered prettier package.json dependency in {closest_prettier_discovered:?}")
                    }
                    None => {
                        log::debug!("Found no prettier in ancestors of {locate_from:?}");
                        return Ok(None);
                    }
                }
            }
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn start(
        _: LanguageServerId,
        prettier_dir: PathBuf,
        _: Arc<dyn NodeRuntime>,
        _: AsyncAppContext,
    ) -> anyhow::Result<Self> {
        Ok(Self::Test(TestPrettier {
            default: prettier_dir == DEFAULT_PRETTIER_DIR.as_path(),
            prettier_dir,
        }))
    }

    #[cfg(not(any(test, feature = "test-support")))]
    pub async fn start(
        server_id: LanguageServerId,
        prettier_dir: PathBuf,
        node: Arc<dyn NodeRuntime>,
        cx: AsyncAppContext,
    ) -> anyhow::Result<Self> {
        use lsp::LanguageServerBinary;

        let background = cx.background();
        anyhow::ensure!(
            prettier_dir.is_dir(),
            "Prettier dir {prettier_dir:?} is not a directory"
        );
        let prettier_server = DEFAULT_PRETTIER_DIR.join(PRETTIER_SERVER_FILE);
        anyhow::ensure!(
            prettier_server.is_file(),
            "no prettier server package found at {prettier_server:?}"
        );

        let node_path = background
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
        let server = background
            .spawn(server.initialize(None))
            .await
            .context("prettier server initialization")?;
        Ok(Self::Real(RealPrettier {
            server,
            default: prettier_dir == DEFAULT_PRETTIER_DIR.as_path(),
            prettier_dir,
        }))
    }

    pub async fn format(
        &self,
        buffer: &ModelHandle<Buffer>,
        buffer_path: Option<PathBuf>,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<Diff> {
        match self {
            Self::Real(local) => {
                let params = buffer.read_with(cx, |buffer, cx| {
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
                    anyhow::ensure!(prettier_node_modules.is_dir(), "Prettier node_modules dir does not exist: {prettier_node_modules:?}");
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
            #[cfg(any(test, feature = "test-support"))]
            Self::Test(_) => Ok(buffer
                .read_with(cx, |buffer, cx| {
                    let formatted_text = buffer.text() + FORMAT_SUFFIX;
                    buffer.diff(formatted_text, cx)
                })
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
}

async fn has_prettier_in_node_modules(fs: &dyn Fs, path: &Path) -> anyhow::Result<bool> {
    let possible_node_modules_location = path.join("node_modules").join(PRETTIER_PACKAGE_NAME);
    if let Some(node_modules_location_metadata) = fs
        .metadata(&possible_node_modules_location)
        .await
        .with_context(|| format!("fetching metadata for {possible_node_modules_location:?}"))?
    {
        return Ok(node_modules_location_metadata.is_dir);
    }
    Ok(false)
}

async fn read_package_json(
    fs: &dyn Fs,
    path: &Path,
) -> anyhow::Result<Option<HashMap<String, serde_json::Value>>> {
    let possible_package_json = path.join("package.json");
    if let Some(package_json_metadata) = fs
        .metadata(&possible_package_json)
        .await
        .with_context(|| format!("fetching metadata for package json {possible_package_json:?}"))?
    {
        if !package_json_metadata.is_dir && !package_json_metadata.is_symlink {
            let package_json_contents = fs
                .load(&possible_package_json)
                .await
                .with_context(|| format!("reading {possible_package_json:?} file contents"))?;
            return serde_json::from_str::<HashMap<String, serde_json::Value>>(
                &package_json_contents,
            )
            .map(Some)
            .with_context(|| format!("parsing {possible_package_json:?} file contents"));
        }
    }
    Ok(None)
}

fn has_prettier_in_package_json(
    package_json_contents: &HashMap<String, serde_json::Value>,
) -> bool {
    if let Some(serde_json::Value::Object(o)) = package_json_contents.get("dependencies") {
        if o.contains_key(PRETTIER_PACKAGE_NAME) {
            return true;
        }
    }
    if let Some(serde_json::Value::Object(o)) = package_json_contents.get("devDependencies") {
        if o.contains_key(PRETTIER_PACKAGE_NAME) {
            return true;
        }
    }
    false
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

#[cfg(test)]
mod tests {
    use fs::FakeFs;
    use serde_json::json;

    use super::*;

    #[gpui::test]
    async fn test_prettier_lookup_finds_nothing(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
                ".config": {
                    "zed": {
                        "settings.json": r#"{ "formatter": "auto" }"#,
                    },
                },
                "work": {
                    "project": {
                        "src": {
                            "index.js": "// index.js file contents",
                        },
                        "node_modules": {
                            "expect": {
                                "build": {
                                    "print.js": "// print.js file contents",
                                },
                                "package.json": r#"{
                                    "devDependencies": {
                                        "prettier": "2.5.1"
                                    }
                                }"#,
                            },
                            "prettier": {
                                "index.js": "// Dummy prettier package file",
                            },
                        },
                        "package.json": r#"{}"#
                    },
                }
            }),
        )
        .await;

        assert!(
            Prettier::locate_prettier_installation(
                fs.as_ref(),
                &HashSet::default(),
                Path::new("/root/.config/zed/settings.json"),
            )
            .await
            .unwrap()
            .is_none(),
            "Should successfully find no prettier for path hierarchy without it"
        );
        assert!(
            Prettier::locate_prettier_installation(
                fs.as_ref(),
                &HashSet::default(),
                Path::new("/root/work/project/src/index.js")
            )
            .await
            .unwrap()
            .is_none(),
            "Should successfully find no prettier for path hierarchy that has node_modules with prettier, but no package.json mentions of it"
        );
        assert!(
            Prettier::locate_prettier_installation(
                fs.as_ref(),
                &HashSet::default(),
                Path::new("/root/work/project/node_modules/expect/build/print.js")
            )
            .await
            .unwrap()
            .is_none(),
            "Even though it has package.json with prettier in it and no prettier on node_modules along the path, nothing should fail since declared inside node_modules"
        );
    }

    #[gpui::test]
    async fn test_prettier_lookup_in_simple_npm_projects(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
                "web_blog": {
                    "node_modules": {
                        "prettier": {
                            "index.js": "// Dummy prettier package file",
                        },
                        "expect": {
                            "build": {
                                "print.js": "// print.js file contents",
                            },
                            "package.json": r#"{
                                "devDependencies": {
                                    "prettier": "2.5.1"
                                }
                            }"#,
                        },
                    },
                    "pages": {
                        "[slug].tsx": "// [slug].tsx file contents",
                    },
                    "package.json": r#"{
                        "devDependencies": {
                            "prettier": "2.3.0"
                        },
                        "prettier": {
                            "semi": false,
                            "printWidth": 80,
                            "htmlWhitespaceSensitivity": "strict",
                            "tabWidth": 4
                        }
                    }"#
                }
            }),
        )
        .await;

        assert_eq!(
            Prettier::locate_prettier_installation(
                fs.as_ref(),
                &HashSet::default(),
                Path::new("/root/web_blog/pages/[slug].tsx")
            )
            .await
            .unwrap(),
            Some(PathBuf::from("/root/web_blog")),
            "Should find a preinstalled prettier in the project root"
        );
        assert_eq!(
            Prettier::locate_prettier_installation(
                fs.as_ref(),
                &HashSet::default(),
                Path::new("/root/web_blog/node_modules/expect/build/print.js")
            )
            .await
            .unwrap(),
            Some(PathBuf::from("/root/web_blog")),
            "Should find a preinstalled prettier in the project root even for node_modules files"
        );
    }

    #[gpui::test]
    async fn test_prettier_lookup_for_not_installed(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
                "work": {
                    "web_blog": {
                        "pages": {
                            "[slug].tsx": "// [slug].tsx file contents",
                        },
                        "package.json": r#"{
                            "devDependencies": {
                                "prettier": "2.3.0"
                            },
                            "prettier": {
                                "semi": false,
                                "printWidth": 80,
                                "htmlWhitespaceSensitivity": "strict",
                                "tabWidth": 4
                            }
                        }"#
                    }
                }
            }),
        )
        .await;

        let path = "/root/work/web_blog/node_modules/pages/[slug].tsx";
        match Prettier::locate_prettier_installation(
            fs.as_ref(),
            &HashSet::default(),
            Path::new(path)
        )
        .await {
            Ok(path) => panic!("Expected to fail for prettier in package.json but not in node_modules found, but got path {path:?}"),
            Err(e) => {
                let message = e.to_string();
                assert!(message.contains(path), "Error message should mention which start file was used for location");
                assert!(message.contains("/root/work/web_blog"), "Error message should mention potential candidates without prettier node_modules contents");
            },
        };

        assert_eq!(
            Prettier::locate_prettier_installation(
                fs.as_ref(),
                &HashSet::from_iter(
                    [PathBuf::from("/root"), PathBuf::from("/root/work")].into_iter()
                ),
                Path::new("/root/work/web_blog/node_modules/pages/[slug].tsx")
            )
            .await
            .unwrap(),
            Some(PathBuf::from("/root/work")),
            "Should return first cached value found without path checks"
        );
    }

    #[gpui::test]
    async fn test_prettier_lookup_in_npm_workspaces(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
                "work": {
                    "full-stack-foundations": {
                        "exercises": {
                            "03.loading": {
                                "01.problem.loader": {
                                    "app": {
                                        "routes": {
                                            "users+": {
                                                "$username_+": {
                                                    "notes.tsx": "// notes.tsx file contents",
                                                },
                                            },
                                        },
                                    },
                                    "node_modules": {},
                                    "package.json": r#"{
                                        "devDependencies": {
                                            "prettier": "^3.0.3"
                                        }
                                    }"#
                                },
                            },
                        },
                        "package.json": r#"{
                            "workspaces": ["exercises/*/*", "examples/*"]
                        }"#,
                        "node_modules": {
                            "prettier": {
                                "index.js": "// Dummy prettier package file",
                            },
                        },
                    },
                }
            }),
        )
        .await;

        assert_eq!(
            Prettier::locate_prettier_installation(
                fs.as_ref(),
                &HashSet::default(),
                Path::new("/root/work/full-stack-foundations/exercises/03.loading/01.problem.loader/app/routes/users+/$username_+/notes.tsx"),
            ).await.unwrap(),
            Some(PathBuf::from("/root/work/full-stack-foundations")),
            "Should ascend to the multi-workspace root and find the prettier there",
        );
    }

    #[gpui::test]
    async fn test_prettier_lookup_in_npm_workspaces_for_not_installed(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root",
            json!({
                "work": {
                    "full-stack-foundations": {
                        "exercises": {
                            "03.loading": {
                                "01.problem.loader": {
                                    "app": {
                                        "routes": {
                                            "users+": {
                                                "$username_+": {
                                                    "notes.tsx": "// notes.tsx file contents",
                                                },
                                            },
                                        },
                                    },
                                    "node_modules": {},
                                    "package.json": r#"{
                                        "devDependencies": {
                                            "prettier": "^3.0.3"
                                        }
                                    }"#
                                },
                            },
                        },
                        "package.json": r#"{
                            "workspaces": ["exercises/*/*", "examples/*"]
                        }"#,
                    },
                }
            }),
        )
        .await;

        match Prettier::locate_prettier_installation(
            fs.as_ref(),
            &HashSet::default(),
            Path::new("/root/work/full-stack-foundations/exercises/03.loading/01.problem.loader/app/routes/users+/$username_+/notes.tsx")
        )
        .await {
            Ok(path) => panic!("Expected to fail for prettier in package.json but not in node_modules found, but got path {path:?}"),
            Err(e) => {
                let message = e.to_string();
                assert!(message.contains("/root/work/full-stack-foundations/exercises/03.loading/01.problem.loader"), "Error message should mention which project had prettier defined");
                assert!(message.contains("/root/work/full-stack-foundations"), "Error message should mention potential candidates without prettier node_modules contents");
            },
        };
    }
}

fn is_node_modules(path_component: &std::path::Component<'_>) -> bool {
    path_component.as_os_str().to_string_lossy() == "node_modules"
}
