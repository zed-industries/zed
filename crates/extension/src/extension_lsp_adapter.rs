use crate::wasm_host::{
    wit::{self, LanguageServerConfig},
    WasmExtension, WasmHost,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::{Future, FutureExt};
use gpui::AsyncAppContext;
use language::{
    CodeLabel, HighlightId, Language, LanguageServerName, LspAdapter, LspAdapterDelegate,
};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerBinaryOptions};
use serde::Serialize;
use serde_json::Value;
use std::ops::Range;
use std::{any::Any, path::PathBuf, pin::Pin, sync::Arc};
use util::{maybe, ResultExt};
use wasmtime_wasi::WasiView as _;

pub struct ExtensionLspAdapter {
    pub(crate) extension: WasmExtension,
    pub(crate) language_server_id: LanguageServerName,
    pub(crate) config: LanguageServerConfig,
    pub(crate) host: Arc<WasmHost>,
}

#[async_trait(?Send)]
impl LspAdapter for ExtensionLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(self.config.name.clone().into())
    }

    fn get_language_server_command<'a>(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        _: LanguageServerBinaryOptions,
        _: futures::lock::MutexGuard<'a, Option<LanguageServerBinary>>,
        _: &'a mut AsyncAppContext,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<LanguageServerBinary>>>> {
        async move {
            let command = self
                .extension
                .call({
                    let this = self.clone();
                    |extension, store| {
                        async move {
                            let resource = store.data_mut().table().push(delegate)?;
                            let command = extension
                                .call_language_server_command(
                                    store,
                                    &this.language_server_id,
                                    &this.config,
                                    resource,
                                )
                                .await?
                                .map_err(|e| anyhow!("{}", e))?;
                            anyhow::Ok(command)
                        }
                        .boxed()
                    }
                })
                .await?;

            let path = self
                .host
                .path_from_extension(&self.extension.manifest.id, command.command.as_ref());

            // TODO: This should now be done via the `zed::make_file_executable` function in
            // Zed extension API, but we're leaving these existing usages in place temporarily
            // to avoid any compatibility issues between Zed and the extension versions.
            //
            // We can remove once the following extension versions no longer see any use:
            // - toml@0.0.2
            // - zig@0.0.1
            if ["toml", "zig"].contains(&self.extension.manifest.id.as_ref())
                && path.starts_with(&self.host.work_dir)
            {
                #[cfg(not(windows))]
                {
                    use std::fs::{self, Permissions};
                    use std::os::unix::fs::PermissionsExt;

                    fs::set_permissions(&path, Permissions::from_mode(0o755))
                        .context("failed to set file permissions")?;
                }
            }

            Ok(LanguageServerBinary {
                path,
                arguments: command.args.into_iter().map(|arg| arg.into()).collect(),
                env: Some(command.env.into_iter().collect()),
            })
        }
        .boxed_local()
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        unreachable!("get_language_server_command is overridden")
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        unreachable!("get_language_server_command is overridden")
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        unreachable!("get_language_server_command is overridden")
    }

    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        let code_action_kinds = self
            .extension
            .manifest
            .language_servers
            .get(&self.language_server_id)
            .and_then(|server| server.code_action_kinds.clone());

        code_action_kinds.or(Some(vec![
            CodeActionKind::EMPTY,
            CodeActionKind::QUICKFIX,
            CodeActionKind::REFACTOR,
            CodeActionKind::REFACTOR_EXTRACT,
            CodeActionKind::SOURCE,
        ]))
    }

    fn language_ids(&self) -> HashMap<String, String> {
        // TODO: The language IDs can be provided via the language server options
        // in `extension.toml now but we're leaving these existing usages in place temporarily
        // to avoid any compatibility issues between Zed and the extension versions.
        //
        // We can remove once the following extension versions no longer see any use:
        // - php@0.0.1
        if self.extension.manifest.id.as_ref() == "php" {
            return HashMap::from_iter([("PHP".into(), "php".into())]);
        }

        self.extension
            .manifest
            .language_servers
            .get(&LanguageServerName(self.config.name.clone().into()))
            .map(|server| server.language_ids.clone())
            .unwrap_or_default()
    }

    async fn initialization_options(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let delegate = delegate.clone();
        let json_options = self
            .extension
            .call({
                let this = self.clone();
                |extension, store| {
                    async move {
                        let resource = store.data_mut().table().push(delegate)?;
                        let options = extension
                            .call_language_server_initialization_options(
                                store,
                                &this.language_server_id,
                                &this.config,
                                resource,
                            )
                            .await?
                            .map_err(|e| anyhow!("{}", e))?;
                        anyhow::Ok(options)
                    }
                    .boxed()
                }
            })
            .await?;
        Ok(if let Some(json_options) = json_options {
            serde_json::from_str(&json_options).with_context(|| {
                format!("failed to parse initialization_options from extension: {json_options}")
            })?
        } else {
            None
        })
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let delegate = delegate.clone();
        let json_options: Option<String> = self
            .extension
            .call({
                let this = self.clone();
                |extension, store| {
                    async move {
                        let resource = store.data_mut().table().push(delegate)?;
                        let options = extension
                            .call_language_server_workspace_configuration(
                                store,
                                &this.language_server_id,
                                resource,
                            )
                            .await?
                            .map_err(|e| anyhow!("{}", e))?;
                        anyhow::Ok(options)
                    }
                    .boxed()
                }
            })
            .await?;
        Ok(if let Some(json_options) = json_options {
            serde_json::from_str(&json_options).with_context(|| {
                format!("failed to parse initialization_options from extension: {json_options}")
            })?
        } else {
            serde_json::json!({})
        })
    }

    async fn labels_for_completions(
        self: Arc<Self>,
        completions: &[lsp::CompletionItem],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        let completions = completions
            .iter()
            .map(|completion| wit::Completion::from(completion.clone()))
            .collect::<Vec<_>>();

        let labels = self
            .extension
            .call({
                let this = self.clone();
                |extension, store| {
                    async move {
                        extension
                            .call_labels_for_completions(
                                store,
                                &this.language_server_id,
                                completions,
                            )
                            .await?
                            .map_err(|e| anyhow!("{}", e))
                    }
                    .boxed()
                }
            })
            .await?;

        Ok(labels_from_wit(labels, language))
    }

    async fn labels_for_symbols(
        self: Arc<Self>,
        symbols: &[(String, lsp::SymbolKind)],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        let symbols = symbols
            .iter()
            .cloned()
            .map(|(name, kind)| wit::Symbol {
                name,
                kind: kind.into(),
            })
            .collect::<Vec<_>>();

        let labels = self
            .extension
            .call({
                let this = self.clone();
                |extension, store| {
                    async move {
                        extension
                            .call_labels_for_symbols(store, &this.language_server_id, symbols)
                            .await?
                            .map_err(|e| anyhow!("{}", e))
                    }
                    .boxed()
                }
            })
            .await?;

        Ok(labels_from_wit(labels, language))
    }
}

fn labels_from_wit(
    labels: Vec<Option<wit::CodeLabel>>,
    language: &Arc<Language>,
) -> Vec<Option<CodeLabel>> {
    labels
        .into_iter()
        .map(|label| {
            let label = label?;
            let runs = if label.code.is_empty() {
                Vec::new()
            } else {
                language.highlight_text(&label.code.as_str().into(), 0..label.code.len())
            };
            build_code_label(&label, &runs, language)
        })
        .collect()
}

fn build_code_label(
    label: &wit::CodeLabel,
    parsed_runs: &[(Range<usize>, HighlightId)],
    language: &Arc<Language>,
) -> Option<CodeLabel> {
    let mut text = String::new();
    let mut runs = vec![];

    for span in &label.spans {
        match span {
            wit::CodeLabelSpan::CodeRange(range) => {
                let range = Range::from(*range);
                let code_span = &label.code.get(range.clone())?;
                let mut input_ix = range.start;
                let mut output_ix = text.len();
                for (run_range, id) in parsed_runs {
                    if run_range.start >= range.end {
                        break;
                    }
                    if run_range.end <= input_ix {
                        continue;
                    }

                    if run_range.start > input_ix {
                        let len = run_range.start - input_ix;
                        output_ix += len;
                        input_ix += len;
                    }

                    let len = range.end.min(run_range.end) - input_ix;
                    runs.push((output_ix..output_ix + len, *id));
                    output_ix += len;
                    input_ix += len;
                }

                text.push_str(code_span);
            }
            wit::CodeLabelSpan::Literal(span) => {
                let highlight_id = language
                    .grammar()
                    .zip(span.highlight_name.as_ref())
                    .and_then(|(grammar, highlight_name)| {
                        grammar.highlight_id_for_name(highlight_name)
                    })
                    .unwrap_or_default();
                let ix = text.len();
                runs.push((ix..ix + span.text.len(), highlight_id));
                text.push_str(&span.text);
            }
        }
    }

    let filter_range = Range::from(label.filter_range);
    text.get(filter_range.clone())?;
    Some(CodeLabel {
        text,
        runs,
        filter_range,
    })
}

impl From<wit::Range> for Range<usize> {
    fn from(range: wit::Range) -> Self {
        let start = range.start as usize;
        let end = range.end as usize;
        start..end
    }
}

impl From<lsp::CompletionItem> for wit::Completion {
    fn from(value: lsp::CompletionItem) -> Self {
        Self {
            label: value.label,
            detail: value.detail,
            kind: value.kind.map(Into::into),
            insert_text_format: value.insert_text_format.map(Into::into),
        }
    }
}

impl From<lsp::CompletionItemKind> for wit::CompletionKind {
    fn from(value: lsp::CompletionItemKind) -> Self {
        match value {
            lsp::CompletionItemKind::TEXT => Self::Text,
            lsp::CompletionItemKind::METHOD => Self::Method,
            lsp::CompletionItemKind::FUNCTION => Self::Function,
            lsp::CompletionItemKind::CONSTRUCTOR => Self::Constructor,
            lsp::CompletionItemKind::FIELD => Self::Field,
            lsp::CompletionItemKind::VARIABLE => Self::Variable,
            lsp::CompletionItemKind::CLASS => Self::Class,
            lsp::CompletionItemKind::INTERFACE => Self::Interface,
            lsp::CompletionItemKind::MODULE => Self::Module,
            lsp::CompletionItemKind::PROPERTY => Self::Property,
            lsp::CompletionItemKind::UNIT => Self::Unit,
            lsp::CompletionItemKind::VALUE => Self::Value,
            lsp::CompletionItemKind::ENUM => Self::Enum,
            lsp::CompletionItemKind::KEYWORD => Self::Keyword,
            lsp::CompletionItemKind::SNIPPET => Self::Snippet,
            lsp::CompletionItemKind::COLOR => Self::Color,
            lsp::CompletionItemKind::FILE => Self::File,
            lsp::CompletionItemKind::REFERENCE => Self::Reference,
            lsp::CompletionItemKind::FOLDER => Self::Folder,
            lsp::CompletionItemKind::ENUM_MEMBER => Self::EnumMember,
            lsp::CompletionItemKind::CONSTANT => Self::Constant,
            lsp::CompletionItemKind::STRUCT => Self::Struct,
            lsp::CompletionItemKind::EVENT => Self::Event,
            lsp::CompletionItemKind::OPERATOR => Self::Operator,
            lsp::CompletionItemKind::TYPE_PARAMETER => Self::TypeParameter,
            _ => Self::Other(extract_int(value)),
        }
    }
}

impl From<lsp::InsertTextFormat> for wit::InsertTextFormat {
    fn from(value: lsp::InsertTextFormat) -> Self {
        match value {
            lsp::InsertTextFormat::PLAIN_TEXT => Self::PlainText,
            lsp::InsertTextFormat::SNIPPET => Self::Snippet,
            _ => Self::Other(extract_int(value)),
        }
    }
}

impl From<lsp::SymbolKind> for wit::SymbolKind {
    fn from(value: lsp::SymbolKind) -> Self {
        match value {
            lsp::SymbolKind::FILE => Self::File,
            lsp::SymbolKind::MODULE => Self::Module,
            lsp::SymbolKind::NAMESPACE => Self::Namespace,
            lsp::SymbolKind::PACKAGE => Self::Package,
            lsp::SymbolKind::CLASS => Self::Class,
            lsp::SymbolKind::METHOD => Self::Method,
            lsp::SymbolKind::PROPERTY => Self::Property,
            lsp::SymbolKind::FIELD => Self::Field,
            lsp::SymbolKind::CONSTRUCTOR => Self::Constructor,
            lsp::SymbolKind::ENUM => Self::Enum,
            lsp::SymbolKind::INTERFACE => Self::Interface,
            lsp::SymbolKind::FUNCTION => Self::Function,
            lsp::SymbolKind::VARIABLE => Self::Variable,
            lsp::SymbolKind::CONSTANT => Self::Constant,
            lsp::SymbolKind::STRING => Self::String,
            lsp::SymbolKind::NUMBER => Self::Number,
            lsp::SymbolKind::BOOLEAN => Self::Boolean,
            lsp::SymbolKind::ARRAY => Self::Array,
            lsp::SymbolKind::OBJECT => Self::Object,
            lsp::SymbolKind::KEY => Self::Key,
            lsp::SymbolKind::NULL => Self::Null,
            lsp::SymbolKind::ENUM_MEMBER => Self::EnumMember,
            lsp::SymbolKind::STRUCT => Self::Struct,
            lsp::SymbolKind::EVENT => Self::Event,
            lsp::SymbolKind::OPERATOR => Self::Operator,
            lsp::SymbolKind::TYPE_PARAMETER => Self::TypeParameter,
            _ => Self::Other(extract_int(value)),
        }
    }
}

fn extract_int<T: Serialize>(value: T) -> i32 {
    maybe!({
        let kind = serde_json::to_value(&value)?;
        serde_json::from_value(kind)
    })
    .log_err()
    .unwrap_or(-1)
}

#[test]
fn test_build_code_label() {
    use util::test::marked_text_ranges;

    let (code, code_ranges) = marked_text_ranges(
        "«const» «a»: «fn»(«Bcd»(«Efgh»)) -> «Ijklm» = pqrs.tuv",
        false,
    );
    let code_runs = code_ranges
        .into_iter()
        .map(|range| (range, HighlightId(0)))
        .collect::<Vec<_>>();

    let label = build_code_label(
        &wit::CodeLabel {
            spans: vec![
                wit::CodeLabelSpan::CodeRange(wit::Range {
                    start: code.find("pqrs").unwrap() as u32,
                    end: code.len() as u32,
                }),
                wit::CodeLabelSpan::CodeRange(wit::Range {
                    start: code.find(": fn").unwrap() as u32,
                    end: code.find(" = ").unwrap() as u32,
                }),
            ],
            filter_range: wit::Range {
                start: 0,
                end: "pqrs.tuv".len() as u32,
            },
            code,
        },
        &code_runs,
        &language::PLAIN_TEXT,
    )
    .unwrap();

    let (label_text, label_ranges) =
        marked_text_ranges("pqrs.tuv: «fn»(«Bcd»(«Efgh»)) -> «Ijklm»", false);
    let label_runs = label_ranges
        .into_iter()
        .map(|range| (range, HighlightId(0)))
        .collect::<Vec<_>>();

    assert_eq!(
        label,
        CodeLabel {
            text: label_text,
            runs: label_runs,
            filter_range: label.filter_range.clone()
        }
    )
}

#[test]
fn test_build_code_label_with_invalid_ranges() {
    use util::test::marked_text_ranges;

    let (code, code_ranges) = marked_text_ranges("const «a»: «B» = '🏀'", false);
    let code_runs = code_ranges
        .into_iter()
        .map(|range| (range, HighlightId(0)))
        .collect::<Vec<_>>();

    // A span uses a code range that is invalid because it starts inside of
    // a multi-byte character.
    let label = build_code_label(
        &wit::CodeLabel {
            spans: vec![
                wit::CodeLabelSpan::CodeRange(wit::Range {
                    start: code.find('B').unwrap() as u32,
                    end: code.find(" = ").unwrap() as u32,
                }),
                wit::CodeLabelSpan::CodeRange(wit::Range {
                    start: code.find('🏀').unwrap() as u32 + 1,
                    end: code.len() as u32,
                }),
            ],
            filter_range: wit::Range {
                start: 0,
                end: "B".len() as u32,
            },
            code,
        },
        &code_runs,
        &language::PLAIN_TEXT,
    );
    assert!(label.is_none());

    // Filter range extends beyond actual text
    let label = build_code_label(
        &wit::CodeLabel {
            spans: vec![wit::CodeLabelSpan::Literal(wit::CodeLabelSpanLiteral {
                text: "abc".into(),
                highlight_name: Some("type".into()),
            })],
            filter_range: wit::Range { start: 0, end: 5 },
            code: String::new(),
        },
        &code_runs,
        &language::PLAIN_TEXT,
    );
    assert!(label.is_none());
}
