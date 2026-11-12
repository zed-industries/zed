use anyhow::{Context as _, Result, bail};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
use github_releases::{GitHubLspBinaryVersion, latest_github_release};
use gpui::{App, AsyncApp, Entity, Task};
use language::{
    Buffer, BufferSnapshot, ContextProvider, LanguageName, LanguageRegistry, LocalFile as _,
    LspAdapter, LspAdapterDelegate, LspInstaller, ResolvedRunnable, RunnableMatchCapture,
    RunnableResolver, Toolchain,
};
use lsp::{LanguageServerBinary, LanguageServerName, Uri};
use node_runtime::{NodeRuntime, VersionStrategy};
use project::lsp_store::language_server_settings;
use semver::Version;
use serde_json::{Value, json};
use settings::SettingsLocation;
use smol::{
    fs::{self},
    io::BufReader,
};
use std::{
    borrow::Cow,
    env::consts,
    ffi::OsString,
    future::Future,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};
use task::{TagCondition, TaskTemplate, TaskTemplates, VariableName};
use util::{
    ResultExt, archive::extract_zip, fs::remove_matching, maybe, merge_json_value_into,
    paths::PathStyle, rel_path::RelPath, union_json_value_into,
};

use crate::PackageJsonData;

const SERVER_PATH: &str =
    "node_modules/vscode-langservers-extracted/bin/vscode-json-language-server";

pub(crate) struct JsonTaskProvider;

/// The runnable capture holding a script name, exposed to tasks as `ZED_CUSTOM_script`.
const SCRIPT_CAPTURE: &str = "script";

struct JsonScriptRunnableResolver;

impl RunnableResolver for JsonScriptRunnableResolver {
    fn resolve(
        &self,
        local_captures: &[RunnableMatchCapture],
        _: &[RunnableMatchCapture],
        buffer: &BufferSnapshot,
    ) -> Option<ResolvedRunnable> {
        let run_capture = local_captures.iter().find(|capture| capture.is_run())?;
        // The capture spans the quoted key, so escape sequences are decoded
        // the same way as when the task provider parses the file.
        let quoted_name = buffer
            .text_for_range(run_capture.range())
            .collect::<String>();
        let script_name = serde_json_lenient::from_str::<String>(&quoted_name).ok()?;
        if script_name.is_empty() {
            return None;
        }
        Some(ResolvedRunnable {
            run_range: run_capture.range(),
            extra_captures: smallvec::smallvec![(SCRIPT_CAPTURE.to_owned(), script_name)],
        })
    }
}

impl ContextProvider for JsonTaskProvider {
    fn associated_tasks(
        &self,
        buffer: Option<Entity<Buffer>>,
        cx: &App,
    ) -> gpui::Task<Option<TaskTemplates>> {
        let file = buffer.as_ref().and_then(|buffer| buffer.read(cx).file());
        let Some(file) = project::File::from_dyn(file).cloned() else {
            return Task::ready(None);
        };

        let is_package_json = RelPath::from_unix_str("package.json")
            .is_ok_and(|file_name| file.path.ends_with(file_name));
        let is_composer_json = RelPath::from_unix_str("composer.json")
            .is_ok_and(|file_name| file.path.ends_with(file_name));

        if !is_package_json && !is_composer_json {
            return Task::ready(None);
        }

        // Scripts are run by npm or composer against the saved manifest, so only its scripts are listed:
        // an unsaved rename or an incomplete edit in the buffer must not change what can be run.
        // Runnables still come from the buffer and bind by the decoded script name,
        // so a script renamed without saving gets no runnable until the manifest is saved.
        cx.spawn(async move |cx| {
            let contents = file
                .worktree
                .update(cx, |worktree, cx| worktree.load_file(&file.path, cx))
                .await
                .ok()?;
            let path: Arc<Path> = cx.update(|cx| file.abs_path(cx)).as_path().into();
            let contents = contents.text.to_string();
            let task_templates = if is_package_json {
                package_json_task_templates(path, &contents)?
            } else {
                composer_json_task_templates(&contents)?
            };
            Some(TaskTemplates(task_templates))
        })
    }

    fn runnable_resolver(&self) -> Option<Arc<dyn RunnableResolver>> {
        Some(Arc::new(JsonScriptRunnableResolver))
    }
}

fn package_json_task_templates(path: Arc<Path>, contents: &str) -> Option<Vec<TaskTemplate>> {
    let package_json =
        serde_json_lenient::from_str::<HashMap<String, serde_json_lenient::Value>>(contents)
            .ok()?;
    let package_json = PackageJsonData::new(path, package_json);
    let command = package_json.package_manager.unwrap_or("npm");
    Some(
        package_json
            .scripts
            .into_iter()
            .map(|(_, script_name)| TaskTemplate {
                label: format!("package.json > {}", escape_task_variables(&script_name)),
                command: command.to_owned(),
                args: vec!["run".into(), escape_task_variables(&script_name)],
                cwd: Some(VariableName::Dirname.template_value()),
                tags: vec!["package-script".into()],
                tag_condition: Some(script_tag_condition(script_name)),
                ..TaskTemplate::default()
            })
            .collect(),
    )
}

fn composer_json_task_templates(contents: &str) -> Option<Vec<TaskTemplate>> {
    let composer_json = serde_json_lenient::Value::from_str(contents).ok()?;
    let scripts = composer_json.get("scripts")?.as_object()?;
    Some(
        scripts
            .keys()
            .map(|script_name| TaskTemplate {
                label: format!("composer.json > {}", escape_task_variables(script_name)),
                command: "composer".to_owned(),
                args: vec![
                    "-d".into(),
                    "$ZED_DIRNAME".into(),
                    escape_task_variables(script_name),
                ],
                tags: vec!["composer-script".into()],
                tag_condition: Some(script_tag_condition(script_name.clone())),
                ..TaskTemplate::default()
            })
            .collect(),
    )
}

/// Binds a script's template only to the runnable of that script, see `JsonScriptRunnableResolver`.
fn script_tag_condition(script_name: String) -> TagCondition {
    TagCondition {
        variable: VariableName::Custom(SCRIPT_CAPTURE.into()),
        value: script_name,
    }
}

/// Script names are arbitrary JSON keys, so `$` in them must not start a task variable.
pub(crate) fn escape_task_variables(text: &str) -> String {
    text.replace('$', "$$")
}

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct JsonLspAdapter {
    languages: Arc<LanguageRegistry>,
    node: NodeRuntime,
}

impl JsonLspAdapter {
    const PACKAGE_NAME: &str = "vscode-langservers-extracted";

    pub fn new(languages: Arc<LanguageRegistry>, node: NodeRuntime) -> Self {
        Self { languages, node }
    }
}

impl LspInstaller for JsonLspAdapter {
    type BinaryVersion = Version;

    async fn fetch_latest_server_version(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<Self::BinaryVersion> {
        self.node
            .npm_package_latest_version(Self::PACKAGE_NAME)
            .await
    }

    async fn check_if_user_installed(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate
            .which("vscode-json-language-server".as_ref())
            .await?;
        let env = delegate.shell_env().await;

        Some(LanguageServerBinary {
            path,
            env: Some(env),
            arguments: vec!["--stdio".into()],
        })
    }

    fn check_if_version_installed(
        &self,
        version: &Self::BinaryVersion,
        container_dir: &PathBuf,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Option<LanguageServerBinary>> + use<> {
        let node = self.node.clone();
        let version = version.clone();
        let container_dir = container_dir.clone();

        async move {
            let server_path = container_dir.join(SERVER_PATH);

            let should_install_language_server = node
                .should_install_npm_package(
                    Self::PACKAGE_NAME,
                    &server_path,
                    &container_dir,
                    VersionStrategy::Latest(&version),
                )
                .await;

            if should_install_language_server {
                None
            } else {
                Some(LanguageServerBinary {
                    path: node.binary_path().await.ok()?,
                    env: None,
                    arguments: server_binary_arguments(&server_path),
                })
            }
        }
    }

    fn fetch_server_binary(
        &self,
        _latest_version: Self::BinaryVersion,
        container_dir: PathBuf,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Result<LanguageServerBinary>> + use<> {
        let node = self.node.clone();

        async move {
            let server_path = container_dir.join(SERVER_PATH);

            node.npm_install_latest_packages(&container_dir, &[Self::PACKAGE_NAME])
                .await?;

            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: server_binary_arguments(&server_path),
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &self.node).await
    }
}

#[async_trait(?Send)]
impl LspAdapter for JsonLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("json-language-server".into())
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
        _: &mut AsyncApp,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(json!({
            "provideFormatter": true
        })))
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        requested_uri: Option<Uri>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let requested_path = requested_uri.as_ref().and_then(|uri| {
            (uri.scheme() == "file")
                .then(|| uri.to_file_path().ok())
                .flatten()
        });
        let path_in_worktree = requested_path
            .as_ref()
            .and_then(|abs_path| {
                let rel_path = abs_path.strip_prefix(delegate.worktree_root_path()).ok()?;
                RelPath::new(rel_path, PathStyle::local()).ok()
            })
            .unwrap_or_else(|| Cow::Borrowed(RelPath::empty()));
        let mut config = cx.update(|cx| {
            let schemas = json_schema_store::all_schema_file_associations(
                &self.languages,
                Some(SettingsLocation {
                    worktree_id: delegate.worktree_id(),
                    path: path_in_worktree.as_ref(),
                }),
                cx,
            );

            // This can be viewed via `dev: open language server logs` -> `json-language-server` ->
            // `Server Info`
            serde_json::json!({
                "json": {
                    "format": {
                        "enable": true,
                    },
                    "validate": {
                        "enable": true,
                    },
                    "schemas": schemas
                }
            })
        });

        if let Some(proxy_settings) = cx.update(|cx| {
            json_schema_proxy_settings(cx.http_client().proxy().map(ToString::to_string))
        }) {
            merge_json_value_into(proxy_settings, &mut config);
        }

        let project_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &self.name(), cx)
                .and_then(|s| worktree_root(delegate, s.settings.clone()))
        });

        if let Some(override_options) = project_options {
            union_json_value_into(override_options, &mut config);
        }

        Ok(config)
    }

    fn language_ids(&self) -> HashMap<LanguageName, String> {
        [
            (LanguageName::new_static("JSON"), "json".into()),
            (LanguageName::new_static("JSONC"), "jsonc".into()),
        ]
        .into_iter()
        .collect()
    }

    fn is_primary_zed_json_schema_adapter(&self) -> bool {
        true
    }

    async fn process_completions(&self, items: &mut [lsp::CompletionItem]) {
        unquote_filter_texts(items);
    }
}

/// json-language-server quotes `filterText` (`"status_bar"`), or the label of items
/// without one (`"dark"`), but the completion query never includes the opening
/// quote, which skews fuzzy match scores.
fn unquote_filter_texts(items: &mut [lsp::CompletionItem]) {
    for item in items {
        let filter_text = item.filter_text.as_deref().unwrap_or(&item.label);
        if filter_text.starts_with('"')
            && let Ok(unquoted) = serde_json::from_str::<String>(filter_text)
        {
            item.filter_text = Some(unquoted);
        }
    }
}

fn worktree_root(delegate: &Arc<dyn LspAdapterDelegate>, settings: Option<Value>) -> Option<Value> {
    let Some(Value::Object(mut settings_map)) = settings else {
        return settings;
    };

    let Some(Value::Object(json_config)) = settings_map.get_mut("json") else {
        return Some(Value::Object(settings_map));
    };

    let Some(Value::Array(schemas)) = json_config.get_mut("schemas") else {
        return Some(Value::Object(settings_map));
    };

    for schema in schemas.iter_mut() {
        let Value::Object(schema_map) = schema else {
            continue;
        };
        let Some(Value::String(url)) = schema_map.get_mut("url") else {
            continue;
        };

        if !url.starts_with(".") && !url.starts_with("~") {
            continue;
        }

        *url = delegate
            .resolve_relative_path(url.clone().into())
            .to_string_lossy()
            .into_owned();
    }

    Some(Value::Object(settings_map))
}

fn json_schema_proxy_settings(proxy: Option<String>) -> Option<Value> {
    proxy.map(|proxy| {
        json!({
            "http": {
                "proxy": proxy,
            }
        })
    })
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
        let server_path = container_dir.join(SERVER_PATH);
        anyhow::ensure!(
            server_path.exists(),
            "missing executable in directory {server_path:?}"
        );
        Ok(LanguageServerBinary {
            path: node.binary_path().await?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    })
    .await
    .log_err()
}

#[cfg(test)]
mod tests {
    use std::{ops::Range, path::Path, sync::Arc};

    use gpui::{AppContext as _, Entity, TestAppContext};
    use language::{Buffer, Language};
    use serde_json::json;

    use task::{TaskContext, TaskTemplate, TaskVariables, VariableName};

    use super::{
        JsonTaskProvider, SCRIPT_CAPTURE, composer_json_task_templates, json_schema_proxy_settings,
        package_json_task_templates, unquote_filter_texts,
    };

    #[test]
    fn test_unquote_filter_texts() {
        let filter_texts = [
            Some("\"status_bar\""),
            Some("\"key with \\\"escaped\\\" quotes\""),
            Some("true"),
            Some("{}"),
            Some("\"unterminated"),
            None,
        ];
        let mut items = filter_texts
            .iter()
            .map(|filter_text| lsp::CompletionItem {
                filter_text: filter_text.map(ToString::to_string),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        unquote_filter_texts(&mut items);

        assert_eq!(
            items
                .iter()
                .map(|item| item.filter_text.as_deref())
                .collect::<Vec<_>>(),
            [
                Some("status_bar"),
                Some("key with \"escaped\" quotes"),
                Some("true"),
                Some("{}"),
                Some("\"unterminated"),
                None,
            ]
        );
    }

    #[test]
    fn test_unquote_filter_texts_from_labels() {
        let items = [
            ("\"dark\"", None),
            ("\"value with \\\"escaped\\\" quotes\"", None),
            ("true", None),
            ("{}", None),
            ("\"truncated value...", None),
            ("\"dark\"", Some("custom")),
        ];
        let mut items = items
            .iter()
            .map(|(label, filter_text)| lsp::CompletionItem {
                label: label.to_string(),
                filter_text: filter_text.map(ToString::to_string),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        unquote_filter_texts(&mut items);

        assert_eq!(
            items
                .iter()
                .map(|item| item.filter_text.as_deref())
                .collect::<Vec<_>>(),
            [
                Some("dark"),
                Some("value with \"escaped\" quotes"),
                None,
                None,
                None,
                Some("custom"),
            ]
        );
    }

    #[test]
    fn test_json_schema_proxy_settings_includes_proxy() {
        assert_eq!(
            json_schema_proxy_settings(Some("http://proxy.example:8080".to_string())),
            Some(json!({
                "http": {
                    "proxy": "http://proxy.example:8080",
                }
            }))
        );
    }

    #[test]
    fn test_json_schema_proxy_settings_ignores_missing_proxy() {
        assert_eq!(json_schema_proxy_settings(None), None);
    }

    fn json_language() -> Arc<Language> {
        let language = crate::language("json", tree_sitter_json::LANGUAGE.into());
        Arc::new(
            Arc::try_unwrap(language)
                .unwrap()
                .with_context_provider(Some(Arc::new(JsonTaskProvider))),
        )
    }

    fn script_runnables(
        buffer: &Entity<Buffer>,
        range: Range<usize>,
        cx: &mut TestAppContext,
    ) -> Vec<(String, Option<String>)> {
        buffer.read_with(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot
                .runnable_ranges(range)
                .map(|runnable| {
                    (
                        snapshot
                            .text_for_range(runnable.run_range)
                            .collect::<String>(),
                        runnable.extra_captures.get(SCRIPT_CAPTURE).cloned(),
                    )
                })
                .collect()
        })
    }

    #[gpui::test]
    fn test_script_runnables_capture_decoded_names(cx: &mut TestAppContext) {
        let package_json = r#"{
        "name": "tasks-repro",
        "scripts": {
          "dev": "vite",
          "t\u0065st": "vitest",
          "$ZED_FILE": "echo",
          "": "echo empty"
        },
        "config": {
          "build": "not a script"
        }
      }"#;
        let buffer =
            cx.new(|cx| Buffer::local(package_json, cx).with_language(json_language(), cx));
        cx.executor().run_until_parked();

        assert_eq!(
            script_runnables(&buffer, 0..package_json.len(), cx),
            vec![
                ("\"dev\"".to_string(), Some("dev".to_string())),
                ("\"t\\u0065st\"".to_string(), Some("test".to_string())),
                ("\"$ZED_FILE\"".to_string(), Some("$ZED_FILE".to_string())),
            ],
            "every script key should be runnable with its JSON-decoded name captured, \
                  empty and non-script keys should not be runnable"
        );

        let dev_value_offset = package_json
            .find("\"vite\"")
            .expect("missing the dev script value");

        assert_eq!(
            script_runnables(&buffer, dev_value_offset..dev_value_offset, cx),
            vec![("\"dev\"".to_string(), Some("dev".to_string()))],
            "a position on a script's value should belong to that script's runnable only"
        );
    }

    fn script_context(script_name: Option<&str>) -> TaskContext {
        TaskContext {
            task_variables: TaskVariables::from_iter(
                [
                    (VariableName::File, "/project/package.json".to_owned()),
                    (VariableName::Dirname, "/project".to_owned()),
                ]
                .into_iter()
                .chain(script_name.map(|script_name| {
                    (
                        VariableName::Custom(SCRIPT_CAPTURE.into()),
                        script_name.to_owned(),
                    )
                })),
            ),
            ..TaskContext::default()
        }
    }

    fn bound_template<'a>(
        templates: &'a [TaskTemplate],
        tag: &str,
        context: &TaskContext,
    ) -> &'a TaskTemplate {
        let bound_templates = templates
            .iter()
            .filter(|template| template.binds_to_tag(tag, &context.task_variables))
            .collect::<Vec<_>>();
        match bound_templates.as_slice() {
            [template] => template,
            _ => panic!("expected exactly one template bound to {tag}, got {bound_templates:?}"),
        }
    }

    #[test]
    fn test_package_json_script_templates() {
        let templates = package_json_task_templates(
            Path::new("/project/package.json").into(),
            r#"{
                      "scripts": {
                          "dev": "vite",
                          "t\u0065st": "vitest",
                          "$ZED_FILE": "echo"
                      }
                  }"#,
        )
        .expect("package.json should be parsed");

        for script_name in ["dev", "test", "$ZED_FILE"] {
            let context = script_context(Some(script_name));
            let template = bound_template(&templates, "package-script", &context);
            let task = template
                .resolve_task("test", &context)
                .expect("script template should resolve");
            assert_eq!(task.resolved_label, format!("package.json > {script_name}"));
            assert_eq!(task.resolved.command.as_deref(), Some("npm"));
            assert_eq!(
                task.resolved.args,
                vec!["run".to_string(), script_name.to_string()],
                "script names should be passed literally, without task variable substitution"
            );
        }

        let outside_scripts = script_context(None);
        assert!(
            templates.iter().all(|template| !template
                .binds_to_tag("package-script", &outside_scripts.task_variables)),
            "no script template should bind outside of the scripts"
        );
        assert_eq!(
            templates
                .iter()
                .filter_map(|template| template.resolve_task("test", &outside_scripts))
                .map(|task| task.resolved_label)
                .collect::<Vec<_>>(),
            vec![
                "package.json > $ZED_FILE".to_string(),
                "package.json > dev".to_string(),
                "package.json > test".to_string(),
            ],
            "every script should still be listed once, wherever the cursor is"
        );
    }

    #[test]
    fn test_composer_json_script_templates() {
        let templates = composer_json_task_templates(
            r#"{ "scripts": { "lint": "phpcs", "$ZED_FILE": "echo" } }"#,
        )
        .expect("composer.json should be parsed");

        for script_name in ["lint", "$ZED_FILE"] {
            let context = script_context(Some(script_name));
            let template = bound_template(&templates, "composer-script", &context);
            let task = template
                .resolve_task("test", &context)
                .expect("script template should resolve");
            assert_eq!(
                task.resolved_label,
                format!("composer.json > {script_name}")
            );
            assert_eq!(
                task.resolved.args,
                vec![
                    "-d".to_string(),
                    "/project".to_string(),
                    script_name.to_string()
                ],
            );
        }

        assert!(
            templates.iter().all(|template| !template.binds_to_tag(
                "package-script",
                &script_context(Some("lint")).task_variables
            )),
            "composer scripts should not bind to the package.json tag"
        );
    }
}

pub struct NodeVersionAdapter;

impl NodeVersionAdapter {
    const SERVER_NAME: LanguageServerName =
        LanguageServerName::new_static("package-version-server");
}

impl LspInstaller for NodeVersionAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;

    async fn fetch_latest_server_version(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<GitHubLspBinaryVersion> {
        let release = latest_github_release(
            "zed-industries/package-version-server",
            true,
            false,
            delegate.http_client(),
        )
        .await?;
        let os = match consts::OS {
            "macos" => "apple-darwin",
            "linux" => "unknown-linux-gnu",
            "windows" => "pc-windows-msvc",
            other => bail!("Running on unsupported os: {other}"),
        };
        let suffix = if consts::OS == "windows" {
            ".zip"
        } else {
            ".tar.gz"
        };
        let asset_name = format!("{}-{}-{os}{suffix}", Self::SERVER_NAME, consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;
        Ok(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
            digest: asset.digest.clone(),
        })
    }

    async fn check_if_user_installed(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
        Some(LanguageServerBinary {
            path,
            env: None,
            arguments: Default::default(),
        })
    }

    fn fetch_server_binary(
        &self,
        latest_version: GitHubLspBinaryVersion,
        container_dir: PathBuf,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Result<LanguageServerBinary>> + use<> {
        let delegate = delegate.clone();

        async move {
            let version = &latest_version;
            let destination_path = container_dir.join(format!(
                "{}-{}{}",
                Self::SERVER_NAME,
                version.name,
                std::env::consts::EXE_SUFFIX
            ));
            let destination_container_path =
                container_dir.join(format!("{}-{}-tmp", Self::SERVER_NAME, version.name));
            if fs::metadata(&destination_path).await.is_err() {
                let mut response = delegate
                    .http_client()
                    .get(&version.url, Default::default(), true)
                    .await
                    .context("downloading release")?;
                if version.url.ends_with(".zip") {
                    extract_zip(&destination_container_path, response.body_mut()).await?;
                } else if version.url.ends_with(".tar.gz") {
                    let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                    let archive = Archive::new(decompressed_bytes);
                    archive.unpack(&destination_container_path).await?;
                }

                fs::copy(
                    destination_container_path.join(format!(
                        "{}{}",
                        Self::SERVER_NAME,
                        std::env::consts::EXE_SUFFIX
                    )),
                    &destination_path,
                )
                .await?;
                remove_matching(&container_dir, |entry| entry != destination_path).await;
            }
            Ok(LanguageServerBinary {
                path: destination_path,
                env: None,
                arguments: Default::default(),
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_version_server_binary(container_dir).await
    }
}

#[async_trait(?Send)]
impl LspAdapter for NodeVersionAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }
}

async fn get_cached_version_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    maybe!(async {
        let mut last = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            last = Some(entry?.path());
        }

        anyhow::Ok(LanguageServerBinary {
            path: last.context("no cached binary")?,
            env: None,
            arguments: Default::default(),
        })
    })
    .await
    .log_err()
}
