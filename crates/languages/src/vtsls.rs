use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use gpui::AsyncApp;
use language::{
    LanguageName, LspAdapter, LspAdapterDelegate, LspInstaller, PromptResponseContext, Toolchain,
};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerName, Uri};
use node_runtime::{NodeRuntime, VersionStrategy};
use project::{Fs, lsp_store::language_server_settings};
use regex::Regex;
use semver::Version;
use serde_json::Value;
use serde_json::json;
use settings::update_settings_file;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};
use util::{ResultExt, maybe, merge_json_value_into};

const ACTION_ALWAYS: &str = "Always";
const ACTION_NEVER: &str = "Never";
const UPDATE_IMPORTS_MESSAGE_PATTERN: &str = "Update imports for";
const VTSLS_SERVER_NAME: &str = "vtsls";

fn typescript_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct VtslsLspAdapter {
    node: NodeRuntime,
    fs: Arc<dyn Fs>,
}

impl VtslsLspAdapter {
    const PACKAGE_NAME: &'static str = "@vtsls/language-server";
    const SERVER_PATH: &'static str = "node_modules/@vtsls/language-server/bin/vtsls.js";

    const TYPESCRIPT_PACKAGE_NAME: &'static str = "typescript";
    const TYPESCRIPT_TSDK_PATH: &'static str = "node_modules/typescript/lib";
    const TYPESCRIPT_YARN_TSDK_PATH: &'static str = ".yarn/sdks/typescript/lib";

    pub fn new(node: NodeRuntime, fs: Arc<dyn Fs>) -> Self {
        VtslsLspAdapter { node, fs }
    }

    async fn tsdk_path(&self, adapter: &Arc<dyn LspAdapterDelegate>) -> Option<&'static str> {
        let yarn_sdk = adapter
            .worktree_root_path()
            .join(Self::TYPESCRIPT_YARN_TSDK_PATH);

        let tsdk_path = if self.fs.is_dir(&yarn_sdk).await {
            Self::TYPESCRIPT_YARN_TSDK_PATH
        } else {
            Self::TYPESCRIPT_TSDK_PATH
        };

        if self
            .fs
            .is_dir(&adapter.worktree_root_path().join(tsdk_path))
            .await
        {
            Some(tsdk_path)
        } else {
            None
        }
    }

    pub fn enhance_diagnostic_message(message: &str) -> Option<String> {
        static SINGLE_WORD_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"'([^\s']*)'").expect("Failed to create REGEX"));

        static MULTI_WORD_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"'([^']+\s+[^']*)'").expect("Failed to create REGEX"));

        let first = SINGLE_WORD_REGEX.replace_all(message, "`$1`").to_string();
        let second = MULTI_WORD_REGEX
            .replace_all(&first, "\n```typescript\n$1\n```\n")
            .to_string();
        Some(second)
    }
}

pub struct TypeScriptVersions {
    typescript_version: Version,
    server_version: Version,
}

const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("vtsls");

impl LspInstaller for VtslsLspAdapter {
    type BinaryVersion = TypeScriptVersions;

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<Self::BinaryVersion> {
        Ok(TypeScriptVersions {
            typescript_version: self.node.npm_package_latest_version("typescript").await?,
            server_version: self
                .node
                .npm_package_latest_version("@vtsls/language-server")
                .await?,
        })
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let env = delegate.shell_env().await;
        let path = delegate.which(SERVER_NAME.as_ref()).await?;
        Some(LanguageServerBinary {
            path: path.clone(),
            arguments: typescript_server_binary_arguments(&path),
            env: Some(env),
        })
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Self::BinaryVersion,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let server_path = container_dir.join(Self::SERVER_PATH);

        let typescript_version = latest_version.typescript_version.to_string();
        let server_version = latest_version.server_version.to_string();

        let mut packages_to_install = Vec::new();

        if self
            .node
            .should_install_npm_package(
                Self::PACKAGE_NAME,
                &server_path,
                &container_dir,
                VersionStrategy::Latest(&latest_version.server_version),
            )
            .await
        {
            packages_to_install.push((Self::PACKAGE_NAME, server_version.as_str()));
        }

        if self
            .node
            .should_install_npm_package(
                Self::TYPESCRIPT_PACKAGE_NAME,
                &container_dir.join(Self::TYPESCRIPT_TSDK_PATH),
                &container_dir,
                VersionStrategy::Latest(&latest_version.typescript_version),
            )
            .await
        {
            packages_to_install.push((Self::TYPESCRIPT_PACKAGE_NAME, typescript_version.as_str()));
        }

        self.node
            .npm_install_packages(&container_dir, &packages_to_install)
            .await?;

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
        get_cached_ts_server_binary(container_dir, &self.node).await
    }
}

#[async_trait(?Send)]
impl LspAdapter for VtslsLspAdapter {
    fn name(&self) -> LanguageServerName {
        SERVER_NAME
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
        let label_len = item.label.len();
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

        let text = if let Some(description) = item
            .label_details
            .as_ref()
            .and_then(|label_details| label_details.description.as_ref())
        {
            format!("{} {}", item.label, description)
        } else if let Some(detail) = &item.detail {
            format!("{} {}", item.label, detail)
        } else {
            item.label.clone()
        };
        Some(language::CodeLabel::filtered(
            text,
            label_len,
            item.filter_text.as_deref(),
            vec![(0..label_len, highlight_id)],
        ))
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: Option<Uri>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let tsdk_path = self.tsdk_path(delegate).await;
        let config = serde_json::json!({
            "tsdk": tsdk_path,
            "suggest": {
                "completeFunctionCalls": true
            },
            "inlayHints": {
                "parameterNames": {
                    "enabled": "all",
                    "suppressWhenArgumentMatchesName": false
                },
                "parameterTypes": {
                    "enabled": true
                },
                "variableTypes": {
                    "enabled": true,
                    "suppressWhenTypeMatchesName": false
                },
                "propertyDeclarationTypes": {
                    "enabled": true
                },
                "functionLikeReturnTypes": {
                    "enabled": true
                },
                "enumMemberValues": {
                    "enabled": true
                }
            },
            "tsserver": {
                "maxTsServerMemory": 8092
            },
        });

        let mut default_workspace_configuration = serde_json::json!({
            "typescript": config,
            "javascript": config,
            "vtsls": {
                "experimental": {
                    "completion": {
                        "enableServerSideFuzzyMatch": true,
                        "entriesLimit": 5000,
                    }
                },
               "autoUseWorkspaceTsdk": true
            }
        });

        let override_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
        })?;

        if let Some(override_options) = override_options {
            merge_json_value_into(override_options, &mut default_workspace_configuration)
        }

        Ok(default_workspace_configuration)
    }

    fn diagnostic_message_to_markdown(&self, message: &str) -> Option<String> {
        VtslsLspAdapter::enhance_diagnostic_message(message)
    }

    fn language_ids(&self) -> HashMap<LanguageName, String> {
        HashMap::from_iter([
            (LanguageName::new_static("TypeScript"), "typescript".into()),
            (LanguageName::new_static("JavaScript"), "javascript".into()),
            (LanguageName::new_static("TSX"), "typescriptreact".into()),
        ])
    }

    fn process_prompt_response(&self, context: &PromptResponseContext, cx: &mut AsyncApp) {
        let selected_title = context.selected_action.title.as_str();
        let is_preference_response =
            selected_title == ACTION_ALWAYS || selected_title == ACTION_NEVER;
        if !is_preference_response {
            return;
        }

        if context.message.contains(UPDATE_IMPORTS_MESSAGE_PATTERN) {
            let setting_value = match selected_title {
                ACTION_ALWAYS => "always",
                ACTION_NEVER => "never",
                _ => return,
            };

            let settings = json!({
                "typescript": {
                    "updateImportsOnFileMove": {
                        "enabled": setting_value
                    }
                },
                "javascript": {
                    "updateImportsOnFileMove": {
                        "enabled": setting_value
                    }
                }
            });

            let _ = cx.update(|cx| {
                update_settings_file(self.fs.clone(), cx, move |content, _| {
                    let lsp_settings = content
                        .project
                        .lsp
                        .0
                        .entry(VTSLS_SERVER_NAME.into())
                        .or_default();

                    if let Some(existing) = &mut lsp_settings.settings {
                        merge_json_value_into(settings, existing);
                    } else {
                        lsp_settings.settings = Some(settings);
                    }
                });
            });
        }
    }
}

async fn get_cached_ts_server_binary(
    container_dir: PathBuf,
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
        let server_path = container_dir.join(VtslsLspAdapter::SERVER_PATH);
        anyhow::ensure!(
            server_path.exists(),
            "missing executable in directory {container_dir:?}"
        );
        Ok(LanguageServerBinary {
            path: node.binary_path().await?,
            env: None,
            arguments: typescript_server_binary_arguments(&server_path),
        })
    })
    .await
    .log_err()
}

#[cfg(test)]
mod tests {
    use crate::vtsls::VtslsLspAdapter;

    #[test]
    fn test_diagnostic_message_to_markdown() {
        // Leaves simple messages unchanged
        let message = "The expected type comes from the return type of this signature.";

        let expected = "The expected type comes from the return type of this signature.";

        assert_eq!(
            VtslsLspAdapter::enhance_diagnostic_message(message).expect("Should be some"),
            expected
        );

        // Parses both multi-word and single-word correctly
        let message = "Property 'baz' is missing in type '{ foo: string; bar: string; }' but required in type 'User'.";

        let expected = "Property `baz` is missing in type \n```typescript\n{ foo: string; bar: string; }\n```\n but required in type `User`.";

        assert_eq!(
            VtslsLspAdapter::enhance_diagnostic_message(message).expect("Should be some"),
            expected
        );

        // Parses multi-and-single word in any order, and ignores existing newlines
        let message = "Type '() => { foo: string; bar: string; }' is not assignable to type 'GetUserFunction'.\n  Property 'baz' is missing in type '{ foo: string; bar: string; }' but required in type 'User'.";

        let expected = "Type \n```typescript\n() => { foo: string; bar: string; }\n```\n is not assignable to type `GetUserFunction`.\n  Property `baz` is missing in type \n```typescript\n{ foo: string; bar: string; }\n```\n but required in type `User`.";

        assert_eq!(
            VtslsLspAdapter::enhance_diagnostic_message(message).expect("Should be some"),
            expected
        );
    }
}
