use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use collections::HashMap;
use gpui::AsyncAppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lazy_static::lazy_static;
use lsp::{CodeActionKind, LanguageServerBinary};
use node_runtime::NodeRuntime;
use project::project_settings::ProjectSettings;
use regex::Regex;
use rope::Rope;
use serde_json::{json, Value};
use settings::Settings;
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    any::Any,
    borrow::Cow,
    ffi::OsString,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{
    fs::remove_matching,
    github::{build_tarball_url, GitHubLspBinaryVersion},
    maybe, ResultExt,
};

fn typescript_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

fn eslint_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct TypeScriptLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl TypeScriptLspAdapter {
    const OLD_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";
    const NEW_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.mjs";

    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        TypeScriptLspAdapter { node }
    }
}

struct TypeScriptVersions {
    typescript_version: String,
    server_version: String,
}

#[async_trait(?Send)]
impl LspAdapter for TypeScriptLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("typescript-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(TypeScriptVersions {
            typescript_version: self.node.npm_package_latest_version("typescript").await?,
            server_version: self
                .node
                .npm_package_latest_version("typescript-language-server")
                .await?,
        }) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::NEW_SERVER_PATH);
        let package_name = "typescript";

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                package_name,
                &server_path,
                &container_dir,
                latest_version.typescript_version.as_str(),
            )
            .await;

        if should_install_language_server {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[
                        (package_name, latest_version.typescript_version.as_str()),
                        (
                            "typescript-language-server",
                            latest_version.server_version.as_str(),
                        ),
                    ],
                )
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: typescript_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_ts_server_binary(container_dir, &*self.node).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_ts_server_binary(container_dir, &*self.node).await
    }

    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::QUICKFIX,
            CodeActionKind::REFACTOR,
            CodeActionKind::REFACTOR_EXTRACT,
            CodeActionKind::SOURCE,
        ])
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        use lsp::CompletionItemKind as Kind;
        let len = item.label.len();
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            Kind::CLASS | Kind::INTERFACE | Kind::ENUM => grammar.highlight_id_for_name("type"),
            Kind::CONSTRUCTOR => grammar.highlight_id_for_name("type"),
            Kind::CONSTANT => grammar.highlight_id_for_name("constant"),
            Kind::FUNCTION | Kind::METHOD => grammar.highlight_id_for_name("function"),
            Kind::PROPERTY | Kind::FIELD => grammar.highlight_id_for_name("property"),
            Kind::VARIABLE => grammar.highlight_id_for_name("variable"),
            _ => None,
        }?;

        let text = match &item.detail {
            Some(detail) => format!("{} {}", item.label, detail),
            None => item.label.clone(),
        };

        Some(language::CodeLabel {
            text,
            runs: vec![(0..len, highlight_id)],
            filter_range: 0..len,
        })
    }

    async fn label_for_resolved_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let (label, range, import) = get_details_for_completion(completion)?;

        let source = Rope::from(label.clone());
        let runs = language.highlight_text(&source, 0..label.len());

        let text = match import {
            Some(import) => format!("{} {}", label, import),
            None => label,
        };
        Some(language::CodeLabel {
            text,
            runs,
            filter_range: range,
        })
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(json!({
            "provideFormatter": true,
            "tsserver": {
                "path": "node_modules/typescript/lib",
            },
            "preferences": {
                "includeInlayParameterNameHints": "all",
                "includeInlayParameterNameHintsWhenArgumentMatchesName": true,
                "includeInlayFunctionParameterTypeHints": true,
                "includeInlayVariableTypeHints": true,
                "includeInlayVariableTypeHintsWhenTypeMatchesName": true,
                "includeInlayPropertyDeclarationTypeHints": true,
                "includeInlayFunctionLikeReturnTypeHints": true,
                "includeInlayEnumMemberValueHints": true,
            }
        })))
    }

    fn language_ids(&self) -> HashMap<String, String> {
        HashMap::from_iter([
            ("TypeScript".into(), "typescript".into()),
            ("JavaScript".into(), "javascript".into()),
            ("TSX".into(), "typescriptreact".into()),
        ])
    }
}

async fn get_cached_ts_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
        let old_server_path = container_dir.join(TypeScriptLspAdapter::OLD_SERVER_PATH);
        let new_server_path = container_dir.join(TypeScriptLspAdapter::NEW_SERVER_PATH);
        if new_server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: typescript_server_binary_arguments(&new_server_path),
            })
        } else if old_server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: typescript_server_binary_arguments(&old_server_path),
            })
        } else {
            Err(anyhow!(
                "missing executable in directory {:?}",
                container_dir
            ))
        }
    })
    .await
    .log_err()
}

pub struct EsLintLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl EsLintLspAdapter {
    const CURRENT_VERSION: &'static str = "release/2.4.4";

    const SERVER_PATH: &'static str = "vscode-eslint/server/out/eslintServer.js";
    const SERVER_NAME: &'static str = "eslint";

    const FLAT_CONFIG_FILE_NAMES: &'static [&'static str] =
        &["eslint.config.js", "eslint.config.mjs", "eslint.config.cjs"];

    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        EsLintLspAdapter { node }
    }
}

#[async_trait(?Send)]
impl LspAdapter for EsLintLspAdapter {
    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let workspace_root = delegate.worktree_root_path();

        let eslint_user_settings = cx.update(|cx| {
            ProjectSettings::get_global(cx)
                .lsp
                .get(Self::SERVER_NAME)
                .and_then(|s| s.settings.clone())
                .unwrap_or_default()
        })?;

        let mut code_action_on_save = json!({
            // We enable this, but without also configuring `code_actions_on_format`
            // in the Zed configuration, it doesn't have an effect.
            "enable": true,
        });

        if let Some(code_action_settings) = eslint_user_settings
            .get("codeActionOnSave")
            .and_then(|settings| settings.as_object())
        {
            if let Some(enable) = code_action_settings.get("enable") {
                code_action_on_save["enable"] = enable.clone();
            }
            if let Some(mode) = code_action_settings.get("mode") {
                code_action_on_save["mode"] = mode.clone();
            }
            if let Some(rules) = code_action_settings.get("rules") {
                code_action_on_save["rules"] = rules.clone();
            }
        }

        let problems = eslint_user_settings
            .get("problems")
            .cloned()
            .unwrap_or_else(|| json!({}));

        let node_path = eslint_user_settings.get("nodePath").unwrap_or(&Value::Null);
        let use_flat_config = Self::FLAT_CONFIG_FILE_NAMES
            .iter()
            .any(|file| workspace_root.join(file).is_file());

        Ok(json!({
            "": {
                "validate": "on",
                "rulesCustomizations": [],
                "run": "onType",
                "nodePath": node_path,
                "workingDirectory": {"mode": "auto"},
                "workspaceFolder": {
                    "uri": workspace_root,
                    "name": workspace_root.file_name()
                        .unwrap_or_else(|| workspace_root.as_os_str()),
                },
                "problems": problems,
                "codeActionOnSave": code_action_on_save,
                "experimental": {
                    "useFlatConfig": use_flat_config,
                },
            }
        }))
    }

    fn name(&self) -> LanguageServerName {
        LanguageServerName(Self::SERVER_NAME.into())
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let url = build_tarball_url("microsoft/vscode-eslint", Self::CURRENT_VERSION)?;

        Ok(Box::new(GitHubLspBinaryVersion {
            name: Self::CURRENT_VERSION.into(),
            url,
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = container_dir.join(format!("vscode-eslint-{}", version.name));
        let server_path = destination_path.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            remove_matching(&container_dir, |entry| entry != destination_path).await;

            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(&destination_path).await?;

            let mut dir = fs::read_dir(&destination_path).await?;
            let first = dir.next().await.ok_or(anyhow!("missing first file"))??;
            let repo_root = destination_path.join("vscode-eslint");
            fs::rename(first.path(), &repo_root).await?;

            self.node
                .run_npm_subcommand(Some(&repo_root), "install", &[])
                .await?;

            self.node
                .run_npm_subcommand(Some(&repo_root), "run-script", &["compile"])
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_eslint_server_binary(container_dir, &*self.node).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_eslint_server_binary(container_dir, &*self.node).await
    }
}

async fn get_cached_eslint_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
        // This is unfortunate but we don't know what the version is to build a path directly
        let mut dir = fs::read_dir(&container_dir).await?;
        let first = dir.next().await.ok_or(anyhow!("missing first file"))??;
        if !first.file_type().await?.is_dir() {
            return Err(anyhow!("First entry is not a directory"));
        }
        let server_path = first.path().join(EsLintLspAdapter::SERVER_PATH);

        Ok(LanguageServerBinary {
            path: node.binary_path().await?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    })
    .await
    .log_err()
}

fn get_details_for_completion(
    completion: &lsp::CompletionItem,
) -> Option<(String, Range<usize>, Option<String>)> {
    println!("completion: {:?}\n", completion);
    let kind = completion.kind?;
    let mut scan = completion.detail.as_ref()?.as_str();
    let import_text = "Auto import from '";
    let import = if scan.starts_with(import_text) {
        let slice = &scan[import_text.len()..];
        let import_end = slice.find('\'');
        let import = match import_end {
            Some(end) => Some(slice[..end].to_string()),
            None => return None,
        };
        if let Some(offset) = scan.find('\n') {
            scan = &scan[(offset + 1)..];
        } else {
            return None;
        }
        import
    } else {
        None
    };

    if scan.starts_with("namespace") {
        if let Some(offset) = scan.find('\n') {
            scan = &scan[offset..];
        } else {
            return None;
        }
    }

    let interface = "interface ";
    let constant = "const ";
    let type_keyword = "type ";
    let (label, range) = match kind {
        lsp::CompletionItemKind::CLASS => {
            let class = "class ";
            let ctor = "constructor ";
            if scan.starts_with(ctor) {
                scan = &scan[ctor.len()..];
                let name_end = scan.find(|c| (c == '(') || (c == '<'))? + 4;
                Some((Cow::from(format!("new {}", &scan)), 4..name_end))
            } else if scan.starts_with(type_keyword) {
                let name_end = scan[type_keyword.len()..].find(|c| (c == ' ') || (c == '<'))?
                    + type_keyword.len();
                Some((Cow::from(scan), type_keyword.len()..name_end))
            } else if scan.starts_with(class) {
                let name_end =
                    scan[class.len()..].find(|c| (c == '\n') || (c == '<'))? + class.len();
                let label = scan[name_end..]
                    .find('\n')
                    .map(|i| i + name_end)
                    .map(|label_end| &scan[..label_end])
                    .unwrap_or(scan);
                Some((Cow::from(label), class.len()..name_end))
            } else {
                None
            }
        }
        lsp::CompletionItemKind::METHOD => {
            let method = "(method) ";
            if !scan.starts_with(method) {
                return None;
            }
            scan = &scan[method.len()..];
            scan = &scan[scan.find('.')? + 1..];
            let name_end = scan.find(|c| (c == '(') || (c == '<'))?;
            Some((Cow::from(scan), 0..name_end))
        }
        lsp::CompletionItemKind::FUNCTION => {
            let func = "function ";
            if scan.starts_with(interface) {
                None
            } else if scan.starts_with(func) {
                scan = &scan[func.len()..];
                let name_end = scan.find(|c| (c == '(') || (c == '<'))?;
                Some((Cow::from(scan), 0..name_end))
            } else {
                None
            }
        }
        lsp::CompletionItemKind::VARIABLE => {
            let var = "var ";
            let let_keyword = "let ";
            let alias = "(alias) ";
            let new = "new ";

            if scan.starts_with(alias) {
                scan = &scan[alias.len()..];
            }

            if scan.starts_with(interface) {
                let name_end =
                    scan[interface.len()..].find(|c| (c == ' ') || (c == '<'))? + interface.len();
                let label = scan[name_end..]
                    .rfind('}')
                    .map(|i| i + name_end)
                    .map(|label_end| &scan[..label_end])
                    .unwrap_or(scan);
                Some((Cow::from(label), 0..name_end))
            } else if scan.starts_with(type_keyword) {
                let name_end = scan[type_keyword.len()..].find(|c| (c == ' ') || (c == '<'))?
                    + type_keyword.len();
                let label = scan[name_end..]
                    .rfind('}')
                    .map(|i| i + name_end + 1)
                    .map(|label_end| &scan[..label_end])
                    .unwrap_or(scan);
                Some((Cow::from(label), type_keyword.len()..name_end))
            } else if scan.starts_with(new) {
                let name_end = scan[new.len()..].find(|c| (c == '(') || (c == '<'))? + new.len();
                let label = scan[name_end..]
                    .find('\n')
                    .map(|i| i + name_end)
                    .map(|label_end| &scan[..label_end])
                    .unwrap_or(scan);
                Some((Cow::from(label), new.len()..name_end))
            } else if scan.starts_with(constant) {
                let name_end = scan[constant.len()..].find(':')? + constant.len();
                Some((Cow::from(scan), 0..name_end))
            } else if scan.starts_with(var) {
                let name_end = scan[var.len()..].find(':')? + var.len();
                Some((Cow::from(scan), 0..name_end))
            } else if scan.starts_with(let_keyword) {
                let name_end = scan[let_keyword.len()..].find(':')? + let_keyword.len();
                Some((Cow::from(scan), 0..name_end))
            } else {
                None
            }
        }
        lsp::CompletionItemKind::CONSTANT => {
            scan = &scan[constant.len()..];
            let name_end = scan.find(':')?;
            Some((Cow::from(scan), 0..name_end))
        }
        lsp::CompletionItemKind::PROPERTY => {
            let property = "(property) ";
            if !scan.starts_with(property) {
                return None;
            }
            scan = &scan[property.len()..];
            scan = &scan[scan.find('.')? + 1..];
            let name_end = scan.find(':')?;
            Some((Cow::from(scan), 0..name_end))
        }
        lsp::CompletionItemKind::FIELD => {
            let property = "(property) ";
            if !scan.starts_with(property) {
                return None;
            }
            scan = &scan[property.len()..];
            scan = &scan[scan.find('.')? + 1..];
            let name_end = scan.find(':')?;
            Some((Cow::from(scan), 0..name_end))
        }
        lsp::CompletionItemKind::CONSTRUCTOR => None,
        lsp::CompletionItemKind::INTERFACE => {
            if scan.starts_with(interface) {
                let name_end =
                    scan[interface.len()..].find(|c| (c == ' ') || (c == '<'))? + interface.len();
                let label = scan[name_end..]
                    .rfind('}')
                    .map(|i| i + name_end + 1)
                    .map(|label_end| &scan[..label_end])
                    .unwrap_or(scan);
                Some((Cow::from(label), interface.len()..name_end))
            } else {
                None
            }
        }
        lsp::CompletionItemKind::ENUM => {
            let enum_text = "enum ";
            if scan.starts_with(enum_text) {
                scan = &scan[enum_text.len()..];
                let name_end = scan.find(' ')?;
                Some((Cow::from(scan), 0..name_end))
            } else {
                None
            }
        }
        _ => None,
    }?;

    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\s*\n)+\s*").unwrap();
    }
    Some((
        REGEX.replace_all(label.as_ref(), " ").into_owned(),
        range,
        import,
    ))
}

#[cfg(test)]
mod tests {
    use crate::typescript::get_details_for_completion;
    use gpui::{Context, TestAppContext};
    use lsp::{CompletionItem, CompletionItemKind};
    use unindent::Unindent;

    #[gpui::test]
    fn test_get_completion_details() {
        let completion = CompletionItem {
            label: "foo".to_string(),
            detail: Some("var foo: string".to_string()),
            kind: Some(CompletionItemKind::VARIABLE),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "foo: string");
        assert_eq!(range, 0..3);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "foo".to_string(),
            detail: Some("let foo: string".to_string()),
            kind: Some(CompletionItemKind::VARIABLE),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "foo: string");
        assert_eq!(range, 0..3);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "foo".to_string(),
            detail: Some("function foo()".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "foo()");
        assert_eq!(range, 0..3);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "Foo".to_string(),
            detail: Some("interface Foo {}".to_string()),
            kind: Some(CompletionItemKind::INTERFACE),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "interface Foo {}");
        assert_eq!(range, 10..13);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "Foo".to_string(),
            detail: Some("enum Foo {}".to_string()),
            kind: Some(CompletionItemKind::ENUM),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "Foo {}");
        assert_eq!(range, 0..3);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "foo".to_string(),
            detail: Some("const foo: string".to_string()),
            kind: Some(CompletionItemKind::CONSTANT),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "foo: string");
        assert_eq!(range, 0..3);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "Hello".to_string(),
            detail: Some("constructor Hello(): Hello".to_string()),
            kind: Some(CompletionItemKind::CLASS),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "new Hello(): Hello");
        assert_eq!(range, 4..9);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "lchmodSync".to_string(),
            detail: Some(
                "Auto import from 'fs'\nfunction lchmodSync(path: PathLike, mode: Mode): void"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "lchmodSync(path: PathLike, mode: Mode): void");
        assert_eq!(range, 0..10);
        assert_eq!(import, Some("fs".to_string()));

        let completion = CompletionItem {
          label: "moduleFunctionDocs".to_string(),
          detail:Some(        "Auto import from 'function-module'\nfunction moduleFunctionDocs(param1: string, param2: number, param3: ModuleClass): [string, number]".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(
            label,
            "moduleFunctionDocs(param1: string, param2: number, param3: ModuleClass): [string, number]"
        );
        assert_eq!(range, 0..18);
        assert_eq!(import, Some("function-module".to_string()));

        let completion = CompletionItem {
            label: "localConst".to_string(),
            detail: Some("const localConst: \"\"".to_string()),
            kind: Some(CompletionItemKind::VARIABLE),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "localConst: \"\"");
        assert_eq!(range, 0..10);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "ModuleGenericClass".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Auto import from 'class-module'\nconstructor ModuleGenericClass<T = any>(hi: T): ModuleGenericClass<T>".to_string()),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(
            label,
            "new ModuleGenericClass<T = any>(hi: T): ModuleGenericClass<T>"
        );
        assert_eq!(range, 4..22);
        assert_eq!(import, Some("class-module".to_string()));

        let completion = CompletionItem {
            label: "ModuleClass".to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            detail: Some(
                "(alias) new ModuleClass(hi: string): ModuleClass\nimport ModuleClass".to_string(),
            ),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "new ModuleClass(hi: string): ModuleClass");
        assert_eq!(range, 4..15);
        assert_eq!(import, None);

        let completion = CompletionItem {
          label: "Mock".to_string(),
          kind: Some(CompletionItemKind::VARIABLE),
          detail: Some("Auto import from 'node:test'\n(alias) type Mock<F extends Function> = F & {\n    mock: MockFunctionContext<F>;\n}\nexport Mock".to_string()),
          ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(
            label,
            "type Mock<F extends Function> = F & { mock: MockFunctionContext<F>; }"
        );
        assert_eq!(range, 5..9);
        assert_eq!(import, Some("node:test".to_string()));

        let completion = CompletionItem {
              label: "ModuleGenericClass".to_string(),
              kind: Some(CompletionItemKind::CLASS),
              detail: Some("Auto import from 'class-module'\nconstructor ModuleGenericClass<T = any>(hi: T): ModuleGenericClass<T>".to_string()),
              ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(
            label,
            "new ModuleGenericClass<T = any>(hi: T): ModuleGenericClass<T>"
        );
        assert_eq!(range, 4..22);
        assert_eq!(import, Some("class-module".to_string()));

        let completion = CompletionItem {
            label: "member".to_string(),
            kind: Some(CompletionItemKind::FIELD),
            detail: Some("(property) ModuleClass.member: string".to_string()),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "member: string");
        assert_eq!(range, 0..6);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "method".to_string(),
            kind: Some(CompletionItemKind::METHOD),
            detail: Some("(method) ModuleClass.method(hi: string): void".to_string()),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "method(hi: string): void");
        assert_eq!(range, 0..6);
        assert_eq!(import, None);

        let completion = CompletionItem {
            label: "Module".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some(
                "Auto import from 'module'\nclass Module\ninterface Module\nnamespace Module"
                    .to_string(),
            ),
            ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "class Module");
        assert_eq!(range, 6..12);
        assert_eq!(import, Some("module".to_string()));

        let completion = CompletionItem {
          label: "ModuleGenericType".to_string(),
          kind: Some(CompletionItemKind::CLASS),
          detail: Some("Auto import from 'type-module'\ntype ModuleGenericType<T = string> = {\n    hi: T;\n}".to_string()),
          ..Default::default()
        };
        let (label, range, import) = get_details_for_completion(&completion).unwrap();
        assert_eq!(label, "type ModuleGenericType<T = string> = { hi: T; }");
        assert_eq!(range, 5..22);
        assert_eq!(import, Some("type-module".to_string()));
    }

    #[gpui::test]
    async fn test_outline(cx: &mut TestAppContext) {
        let language = crate::language("typescript", tree_sitter_typescript::language_typescript());

        let text = r#"
            function a() {
              // local variables are omitted
              let a1 = 1;
              // all functions are included
              async function a2() {}
            }
            // top-level variables are included
            let b: C
            function getB() {}
            // exported variables are included
            export const d = e;
        "#
        .unindent();

        let buffer =
            cx.new_model(|cx| language::Buffer::local(text, cx).with_language(language, cx));
        let outline = buffer.update(cx, |buffer, _| buffer.snapshot().outline(None).unwrap());
        assert_eq!(
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>(),
            &[
                ("function a()", 0),
                ("async function a2()", 1),
                ("let b", 0),
                ("function getB()", 0),
                ("const d", 0),
            ]
        );
    }
}
