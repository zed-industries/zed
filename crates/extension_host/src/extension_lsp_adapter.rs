use anyhow::{Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use extension::{Extension, WorktreeDelegate};
use futures::{Future, FutureExt};
use gpui::AsyncAppContext;
use language::{
    CodeLabel, HighlightId, Language, LanguageName, LanguageToolchainStore, LspAdapter,
    LspAdapterDelegate,
};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerBinaryOptions, LanguageServerName};
use serde::Serialize;
use serde_json::Value;
use std::ops::Range;
use std::{any::Any, path::PathBuf, pin::Pin, sync::Arc};
use util::{maybe, ResultExt};

/// An adapter that allows an [`LspAdapterDelegate`] to be used as a [`WorktreeDelegate`].
pub struct WorktreeDelegateAdapter(pub Arc<dyn LspAdapterDelegate>);

#[async_trait]
impl WorktreeDelegate for WorktreeDelegateAdapter {
    fn id(&self) -> u64 {
        self.0.worktree_id().to_proto()
    }

    fn root_path(&self) -> String {
        self.0.worktree_root_path().to_string_lossy().to_string()
    }

    async fn read_text_file(&self, path: PathBuf) -> Result<String> {
        self.0.read_text_file(path).await
    }

    async fn which(&self, binary_name: String) -> Option<String> {
        self.0
            .which(binary_name.as_ref())
            .await
            .map(|path| path.to_string_lossy().to_string())
    }

    async fn shell_env(&self) -> Vec<(String, String)> {
        self.0.shell_env().await.into_iter().collect()
    }
}

pub struct ExtensionLspAdapter {
    pub(crate) extension: Arc<dyn Extension>,
    pub(crate) language_server_id: LanguageServerName,
    pub(crate) language_name: LanguageName,
}

#[async_trait(?Send)]
impl LspAdapter for ExtensionLspAdapter {
    fn name(&self) -> LanguageServerName {
        self.language_server_id.clone()
    }

    fn get_language_server_command<'a>(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        _: LanguageServerBinaryOptions,
        _: futures::lock::MutexGuard<'a, Option<LanguageServerBinary>>,
        _: &'a mut AsyncAppContext,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<LanguageServerBinary>>>> {
        async move {
            let delegate = Arc::new(WorktreeDelegateAdapter(delegate.clone())) as _;
            let command = self
                .extension
                .language_server_command(
                    self.language_server_id.clone(),
                    self.language_name.clone(),
                    delegate,
                )
                .await?;

            let path = self.extension.path_from_extension(command.command.as_ref());

            // TODO: This should now be done via the `zed::make_file_executable` function in
            // Zed extension API, but we're leaving these existing usages in place temporarily
            // to avoid any compatibility issues between Zed and the extension versions.
            //
            // We can remove once the following extension versions no longer see any use:
            // - toml@0.0.2
            // - zig@0.0.1
            if ["toml", "zig"].contains(&self.extension.manifest().id.as_ref())
                && path.starts_with(&self.extension.work_dir())
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
            .manifest()
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
        if self.extension.manifest().id.as_ref() == "php" {
            return HashMap::from_iter([("PHP".into(), "php".into())]);
        }

        self.extension
            .manifest()
            .language_servers
            .get(&self.language_server_id)
            .map(|server| server.language_ids.clone())
            .unwrap_or_default()
    }

    async fn initialization_options(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let delegate = Arc::new(WorktreeDelegateAdapter(delegate.clone())) as _;
        let json_options = self
            .extension
            .language_server_initialization_options(
                self.language_server_id.clone(),
                self.language_name.clone(),
                delegate,
            )
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
        _: Arc<dyn LanguageToolchainStore>,
        _cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let delegate = Arc::new(WorktreeDelegateAdapter(delegate.clone())) as _;
        let json_options: Option<String> = self
            .extension
            .language_server_workspace_configuration(self.language_server_id.clone(), delegate)
            .await?;
        Ok(if let Some(json_options) = json_options {
            serde_json::from_str(&json_options).with_context(|| {
                format!("failed to parse workspace_configuration from extension: {json_options}")
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
            .cloned()
            .map(lsp_completion_to_extension)
            .collect::<Vec<_>>();

        let labels = self
            .extension
            .labels_for_completions(self.language_server_id.clone(), completions)
            .await?;

        Ok(labels_from_extension(labels, language))
    }

    async fn labels_for_symbols(
        self: Arc<Self>,
        symbols: &[(String, lsp::SymbolKind)],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        let symbols = symbols
            .iter()
            .cloned()
            .map(|(name, kind)| extension::Symbol {
                name,
                kind: lsp_symbol_kind_to_extension(kind),
            })
            .collect::<Vec<_>>();

        let labels = self
            .extension
            .labels_for_symbols(self.language_server_id.clone(), symbols)
            .await?;

        Ok(labels_from_extension(
            labels
                .into_iter()
                .map(|label| label.map(Into::into))
                .collect(),
            language,
        ))
    }
}

fn labels_from_extension(
    labels: Vec<Option<extension::CodeLabel>>,
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
    label: &extension::CodeLabel,
    parsed_runs: &[(Range<usize>, HighlightId)],
    language: &Arc<Language>,
) -> Option<CodeLabel> {
    let mut text = String::new();
    let mut runs = vec![];

    for span in &label.spans {
        match span {
            extension::CodeLabelSpan::CodeRange(range) => {
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
            extension::CodeLabelSpan::Literal(span) => {
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

    let filter_range = label.filter_range.clone();
    text.get(filter_range.clone())?;
    Some(CodeLabel {
        text,
        runs,
        filter_range,
    })
}

fn lsp_completion_to_extension(value: lsp::CompletionItem) -> extension::Completion {
    extension::Completion {
        label: value.label,
        label_details: value
            .label_details
            .map(lsp_completion_item_label_details_to_extension),
        detail: value.detail,
        kind: value.kind.map(lsp_completion_item_kind_to_extension),
        insert_text_format: value
            .insert_text_format
            .map(lsp_insert_text_format_to_extension),
    }
}

fn lsp_completion_item_label_details_to_extension(
    value: lsp::CompletionItemLabelDetails,
) -> extension::CompletionLabelDetails {
    extension::CompletionLabelDetails {
        detail: value.detail,
        description: value.description,
    }
}

fn lsp_completion_item_kind_to_extension(
    value: lsp::CompletionItemKind,
) -> extension::CompletionKind {
    match value {
        lsp::CompletionItemKind::TEXT => extension::CompletionKind::Text,
        lsp::CompletionItemKind::METHOD => extension::CompletionKind::Method,
        lsp::CompletionItemKind::FUNCTION => extension::CompletionKind::Function,
        lsp::CompletionItemKind::CONSTRUCTOR => extension::CompletionKind::Constructor,
        lsp::CompletionItemKind::FIELD => extension::CompletionKind::Field,
        lsp::CompletionItemKind::VARIABLE => extension::CompletionKind::Variable,
        lsp::CompletionItemKind::CLASS => extension::CompletionKind::Class,
        lsp::CompletionItemKind::INTERFACE => extension::CompletionKind::Interface,
        lsp::CompletionItemKind::MODULE => extension::CompletionKind::Module,
        lsp::CompletionItemKind::PROPERTY => extension::CompletionKind::Property,
        lsp::CompletionItemKind::UNIT => extension::CompletionKind::Unit,
        lsp::CompletionItemKind::VALUE => extension::CompletionKind::Value,
        lsp::CompletionItemKind::ENUM => extension::CompletionKind::Enum,
        lsp::CompletionItemKind::KEYWORD => extension::CompletionKind::Keyword,
        lsp::CompletionItemKind::SNIPPET => extension::CompletionKind::Snippet,
        lsp::CompletionItemKind::COLOR => extension::CompletionKind::Color,
        lsp::CompletionItemKind::FILE => extension::CompletionKind::File,
        lsp::CompletionItemKind::REFERENCE => extension::CompletionKind::Reference,
        lsp::CompletionItemKind::FOLDER => extension::CompletionKind::Folder,
        lsp::CompletionItemKind::ENUM_MEMBER => extension::CompletionKind::EnumMember,
        lsp::CompletionItemKind::CONSTANT => extension::CompletionKind::Constant,
        lsp::CompletionItemKind::STRUCT => extension::CompletionKind::Struct,
        lsp::CompletionItemKind::EVENT => extension::CompletionKind::Event,
        lsp::CompletionItemKind::OPERATOR => extension::CompletionKind::Operator,
        lsp::CompletionItemKind::TYPE_PARAMETER => extension::CompletionKind::TypeParameter,
        _ => extension::CompletionKind::Other(extract_int(value)),
    }
}

fn lsp_insert_text_format_to_extension(
    value: lsp::InsertTextFormat,
) -> extension::InsertTextFormat {
    match value {
        lsp::InsertTextFormat::PLAIN_TEXT => extension::InsertTextFormat::PlainText,
        lsp::InsertTextFormat::SNIPPET => extension::InsertTextFormat::Snippet,
        _ => extension::InsertTextFormat::Other(extract_int(value)),
    }
}

fn lsp_symbol_kind_to_extension(value: lsp::SymbolKind) -> extension::SymbolKind {
    match value {
        lsp::SymbolKind::FILE => extension::SymbolKind::File,
        lsp::SymbolKind::MODULE => extension::SymbolKind::Module,
        lsp::SymbolKind::NAMESPACE => extension::SymbolKind::Namespace,
        lsp::SymbolKind::PACKAGE => extension::SymbolKind::Package,
        lsp::SymbolKind::CLASS => extension::SymbolKind::Class,
        lsp::SymbolKind::METHOD => extension::SymbolKind::Method,
        lsp::SymbolKind::PROPERTY => extension::SymbolKind::Property,
        lsp::SymbolKind::FIELD => extension::SymbolKind::Field,
        lsp::SymbolKind::CONSTRUCTOR => extension::SymbolKind::Constructor,
        lsp::SymbolKind::ENUM => extension::SymbolKind::Enum,
        lsp::SymbolKind::INTERFACE => extension::SymbolKind::Interface,
        lsp::SymbolKind::FUNCTION => extension::SymbolKind::Function,
        lsp::SymbolKind::VARIABLE => extension::SymbolKind::Variable,
        lsp::SymbolKind::CONSTANT => extension::SymbolKind::Constant,
        lsp::SymbolKind::STRING => extension::SymbolKind::String,
        lsp::SymbolKind::NUMBER => extension::SymbolKind::Number,
        lsp::SymbolKind::BOOLEAN => extension::SymbolKind::Boolean,
        lsp::SymbolKind::ARRAY => extension::SymbolKind::Array,
        lsp::SymbolKind::OBJECT => extension::SymbolKind::Object,
        lsp::SymbolKind::KEY => extension::SymbolKind::Key,
        lsp::SymbolKind::NULL => extension::SymbolKind::Null,
        lsp::SymbolKind::ENUM_MEMBER => extension::SymbolKind::EnumMember,
        lsp::SymbolKind::STRUCT => extension::SymbolKind::Struct,
        lsp::SymbolKind::EVENT => extension::SymbolKind::Event,
        lsp::SymbolKind::OPERATOR => extension::SymbolKind::Operator,
        lsp::SymbolKind::TYPE_PARAMETER => extension::SymbolKind::TypeParameter,
        _ => extension::SymbolKind::Other(extract_int(value)),
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
        &extension::CodeLabel {
            spans: vec![
                extension::CodeLabelSpan::CodeRange(code.find("pqrs").unwrap()..code.len()),
                extension::CodeLabelSpan::CodeRange(
                    code.find(": fn").unwrap()..code.find(" = ").unwrap(),
                ),
            ],
            filter_range: 0.."pqrs.tuv".len(),
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
        &extension::CodeLabel {
            spans: vec![
                extension::CodeLabelSpan::CodeRange(
                    code.find('B').unwrap()..code.find(" = ").unwrap(),
                ),
                extension::CodeLabelSpan::CodeRange((code.find('🏀').unwrap() + 1)..code.len()),
            ],
            filter_range: 0.."B".len(),
            code,
        },
        &code_runs,
        &language::PLAIN_TEXT,
    );
    assert!(label.is_none());

    // Filter range extends beyond actual text
    let label = build_code_label(
        &extension::CodeLabel {
            spans: vec![extension::CodeLabelSpan::Literal(
                extension::CodeLabelSpanLiteral {
                    text: "abc".into(),
                    highlight_name: Some("type".into()),
                },
            )],
            filter_range: 0..5,
            code: String::new(),
        },
        &code_runs,
        &language::PLAIN_TEXT,
    );
    assert!(label.is_none());
}
