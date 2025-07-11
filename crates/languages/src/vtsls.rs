use anyhow::Result;
use async_trait::async_trait;
use collections::HashMap;
use gpui::AsyncApp;
use language::{LanguageToolchainStore, LspAdapter, LspAdapterDelegate};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerName};
use node_runtime::NodeRuntime;
use project::{Fs, lsp_store::language_server_settings};
use serde_json::Value;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{ResultExt, maybe, merge_json_value_into};

fn typescript_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct VtslsLspAdapter {
    node: NodeRuntime,
}

impl VtslsLspAdapter {
    const PACKAGE_NAME: &'static str = "@vtsls/language-server";
    const SERVER_PATH: &'static str = "node_modules/@vtsls/language-server/bin/vtsls.js";

    const TYPESCRIPT_PACKAGE_NAME: &'static str = "typescript";
    const TYPESCRIPT_TSDK_PATH: &'static str = "node_modules/typescript/lib";

    pub fn new(node: NodeRuntime) -> Self {
        VtslsLspAdapter { node }
    }

    async fn tsdk_path(fs: &dyn Fs, adapter: &Arc<dyn LspAdapterDelegate>) -> Option<&'static str> {
        let is_yarn = adapter
            .read_text_file(PathBuf::from(".yarn/sdks/typescript/lib/typescript.js"))
            .await
            .is_ok();

        let tsdk_path = if is_yarn {
            ".yarn/sdks/typescript/lib"
        } else {
            Self::TYPESCRIPT_TSDK_PATH
        };

        if fs
            .is_dir(&adapter.worktree_root_path().join(tsdk_path))
            .await
        {
            Some(tsdk_path)
        } else {
            None
        }
    }
}

struct TypeScriptVersions {
    typescript_version: String,
    server_version: String,
}

const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("vtsls");

#[async_trait(?Send)]
impl LspAdapter for VtslsLspAdapter {
    fn name(&self) -> LanguageServerName {
        SERVER_NAME.clone()
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(TypeScriptVersions {
            typescript_version: self.node.npm_package_latest_version("typescript").await?,
            server_version: self
                .node
                .npm_package_latest_version("@vtsls/language-server")
                .await?,
        }) as Box<_>)
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Arc<dyn LanguageToolchainStore>,
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
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::SERVER_PATH);

        let mut packages_to_install = Vec::new();

        if self
            .node
            .should_install_npm_package(
                Self::PACKAGE_NAME,
                &server_path,
                &container_dir,
                &latest_version.server_version,
            )
            .await
        {
            packages_to_install.push((Self::PACKAGE_NAME, latest_version.server_version.as_str()));
        }

        if self
            .node
            .should_install_npm_package(
                Self::TYPESCRIPT_PACKAGE_NAME,
                &container_dir.join(Self::TYPESCRIPT_TSDK_PATH),
                &container_dir,
                &latest_version.typescript_version,
            )
            .await
        {
            packages_to_install.push((
                Self::TYPESCRIPT_PACKAGE_NAME,
                latest_version.typescript_version.as_str(),
            ));
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
        let filter_range = item
            .filter_text
            .as_deref()
            .and_then(|filter| text.find(filter).map(|ix| ix..ix + filter.len()))
            .unwrap_or(0..len);
        Some(language::CodeLabel {
            text,
            runs: vec![(0..len, highlight_id)],
            filter_range,
        })
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        fs: &dyn Fs,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let tsdk_path = Self::tsdk_path(fs, delegate).await;
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

    fn language_ids(&self) -> HashMap<String, String> {
        HashMap::from_iter([
            ("TypeScript".into(), "typescript".into()),
            ("JavaScript".into(), "javascript".into()),
            ("TSX".into(), "typescriptreact".into()),
        ])
    }

    fn diagnostic_message_to_markdown(&self, message: &str) -> Option<String> {
        use regex::{Captures, Regex};

        // Helper functions for formatting
        let format_type_block = |prefix: &str, content: &str| -> String {
            if prefix.is_empty() {
                if content.len() > 50 || content.contains('\n') || content.contains('`') {
                    format!("\n```typescript\n{}\n```\n", content)
                } else {
                    format!("`{}`", content)
                }
            } else {
                format!("{} `{}`", prefix, content)
            }
        };

        let format_typescript_block =
            |content: &str| -> String { format!("\n\n```typescript\n{}\n```\n", content) };

        let format_simple_type_block = |content: &str| -> String { format!("`{}`", content) };

        let unstyle_code_block = |content: &str| -> String { format!("`{}`", content) };

        let mut result = message.to_string();

        // Format 'key' with "value"
        let re = Regex::new(r#"(\w+)(\s+)'(.+?)'(\s+)with(\s+)"(.+?)""#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!(
                    "{}{}`{}`{} with `\"{}\"`",
                    &caps[1], &caps[2], &caps[3], &caps[4], &caps[6]
                )
            })
            .to_string();

        // Format "key"
        let re = Regex::new(r#"(\s)'"(.*?)"'(\s|:|.|$)"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!("{}`\"{}\"`{}", &caps[1], &caps[2], &caps[3])
            })
            .to_string();

        // Format declare module snippet
        let re = Regex::new(r#"['"](declare module )['"](.*)['""];['"']"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format_typescript_block(&format!("{} \"{}\"", &caps[1], &caps[2]))
            })
            .to_string();

        // Format missing props error
        let re = Regex::new(r#"(is missing the following properties from type\s?)'(.*)': ([^:]+)"#)
            .unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                let props: Vec<&str> = caps[3].split(", ").filter(|s| !s.is_empty()).collect();
                let props_html = props
                    .iter()
                    .map(|prop| format!("<li>{}</li>", prop))
                    .collect::<Vec<_>>()
                    .join("");
                format!("{}`{}`: <ul>{}</ul>", &caps[1], &caps[2], props_html)
            })
            .to_string();

        // Format type pairs
        let re = Regex::new(r#"(?i)(types) ['"](.*?)['"] and ['"](.*?)['"][.]?"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!("{} `{}` and `{}`", &caps[1], &caps[2], &caps[3])
            })
            .to_string();

        // Format type annotation options
        let re = Regex::new(r#"(?i)type annotation must be ['"](.*?)['"] or ['"](.*?)['"][.]?"#)
            .unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!("type annotation must be `{}` or `{}`", &caps[1], &caps[2])
            })
            .to_string();

        // Format overload
        let re = Regex::new(r#"(?i)(Overload \d of \d), ['"](.*?)['"], "#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!("{}, `{}`, ", &caps[1], &caps[2])
            })
            .to_string();

        // Format simple strings
        let re = Regex::new(r#"^['"]"[^"]*"['"]$"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| format_typescript_block(&caps[0]))
            .to_string();

        // Replace module 'x' by module "x" for ts error #2307
        let re = Regex::new(r#"(?i)(module )'([^"]*?)'"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!("{}\"{}\"", &caps[1], &caps[2])
            })
            .to_string();

        // Format string types
        let re = Regex::new(r#"(?i)(module|file|file name|imported via) ['""](.*?)['""]"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format_type_block(&caps[1], &format!("\"{}\"", &caps[2]))
            })
            .to_string();

        // Format types
        let re = Regex::new(r#"(?i)(type|type alias|interface|module|file|file name|class|method's|subtype of constraint) ['"](.*?)['"]"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format_type_block(&caps[1], &caps[2])
            })
            .to_string();

        // Format reversed types
        let re = Regex::new(r#"(?i)(.*)['"]([^>]*)['"] (type|interface|return type|file|module|is (not )?assignable)"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!("{}`{}` {}", &caps[1], &caps[2], &caps[3])
            })
            .to_string();

        // Format simple types that didn't captured before
        let re = Regex::new(
            r#"['"]((void|null|undefined|any|boolean|string|number|bigint|symbol)(\[\])?)['"']"#,
        )
        .unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format_simple_type_block(&caps[1])
            })
            .to_string();

        // Format some typescript keywords
        let re = Regex::new(r#"['"](import|export|require|in|continue|break|let|false|true|const|new|throw|await|for await|[0-9]+)( ?.*?)['"]"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format_typescript_block(&format!("{}{}", &caps[1], &caps[2]))
            })
            .to_string();

        // Format return values
        let re = Regex::new(r#"(?i)(return|operator) ['"](.*?)['"']"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!("{} {}", &caps[1], format_typescript_block(&caps[2]))
            })
            .to_string();

        // Format regular code blocks
        let re = Regex::new(r#"(\W|^)'([^'"]*?)'(\W|$)"#).unwrap();
        result = re
            .replace_all(&result, |caps: &Captures| {
                format!("{}{}{}", &caps[1], unstyle_code_block(&caps[2]), &caps[3])
            })
            .to_string();

        Some(result)
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
