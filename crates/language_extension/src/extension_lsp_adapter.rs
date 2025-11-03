use std::ops::Range;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::{HashMap, HashSet};
use extension::{Extension, ExtensionLanguageServerProxy, WorktreeDelegate};
use futures::{Future, FutureExt, future::join_all};
use gpui::{App, AppContext, AsyncApp, Task};
use language::{
    BinaryStatus, CodeLabel, DynLspInstaller, HighlightId, Language, LanguageName, LspAdapter,
    LspAdapterDelegate, Rope, Toolchain,
};
use lsp::{
    CodeActionKind, LanguageServerBinary, LanguageServerBinaryOptions, LanguageServerName,
    LanguageServerSelector,
};
use serde::Serialize;
use serde_json::Value;
use util::{ResultExt, fs::make_file_executable, maybe, rel_path::RelPath};

use crate::{LanguageServerRegistryProxy, LspAccess};

/// An adapter that allows an [`LspAdapterDelegate`] to be used as a [`WorktreeDelegate`].
struct WorktreeDelegateAdapter(pub Arc<dyn LspAdapterDelegate>);

#[async_trait]
impl WorktreeDelegate for WorktreeDelegateAdapter {
    fn id(&self) -> u64 {
        self.0.worktree_id().to_proto()
    }

    fn root_path(&self) -> String {
        self.0.worktree_root_path().to_string_lossy().into_owned()
    }

    async fn read_text_file(&self, path: &RelPath) -> Result<String> {
        self.0.read_text_file(path).await
    }

    async fn which(&self, binary_name: String) -> Option<String> {
        self.0
            .which(binary_name.as_ref())
            .await
            .map(|path| path.to_string_lossy().into_owned())
    }

    async fn shell_env(&self) -> Vec<(String, String)> {
        self.0.shell_env().await.into_iter().collect()
    }
}

impl ExtensionLanguageServerProxy for LanguageServerRegistryProxy {
    fn register_language_server(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    ) {
        self.language_registry.register_lsp_adapter(
            language.clone(),
            Arc::new(ExtensionLspAdapter::new(
                extension,
                language_server_id,
                language,
            )),
        );
    }

    fn remove_language_server(
        &self,
        language: &LanguageName,
        language_server_name: &LanguageServerName,
        cx: &mut App,
    ) -> Task<Result<()>> {
        self.language_registry
            .remove_lsp_adapter(language, language_server_name);

        let mut tasks = Vec::new();
        match &self.lsp_access {
            LspAccess::ViaLspStore(lsp_store) => lsp_store.update(cx, |lsp_store, cx| {
                let stop_task = lsp_store.stop_language_servers_for_buffers(
                    Vec::new(),
                    HashSet::from_iter([LanguageServerSelector::Name(
                        language_server_name.clone(),
                    )]),
                    cx,
                );
                tasks.push(stop_task);
            }),
            LspAccess::ViaWorkspaces(lsp_store_provider) => {
                if let Ok(lsp_stores) = lsp_store_provider(cx) {
                    for lsp_store in lsp_stores {
                        lsp_store.update(cx, |lsp_store, cx| {
                            let stop_task = lsp_store.stop_language_servers_for_buffers(
                                Vec::new(),
                                HashSet::from_iter([LanguageServerSelector::Name(
                                    language_server_name.clone(),
                                )]),
                                cx,
                            );
                            tasks.push(stop_task);
                        });
                    }
                }
            }
            LspAccess::Noop => {}
        }

        cx.background_spawn(async move {
            let results = join_all(tasks).await;
            for result in results {
                result?;
            }
            Ok(())
        })
    }

    fn update_language_server_status(
        &self,
        language_server_id: LanguageServerName,
        status: BinaryStatus,
    ) {
        log::debug!(
            "updating binary status for {} to {:?}",
            language_server_id,
            status
        );
        self.language_registry
            .update_lsp_binary_status(language_server_id, status);
    }
}

struct ExtensionLspAdapter {
    extension: Arc<dyn Extension>,
    language_server_id: LanguageServerName,
    language_name: LanguageName,
}

impl ExtensionLspAdapter {
    fn new(
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language_name: LanguageName,
    ) -> Self {
        Self {
            extension,
            language_server_id,
            language_name,
        }
    }
}

#[async_trait(?Send)]
impl DynLspInstaller for ExtensionLspAdapter {
    fn get_language_server_command<'a>(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: LanguageServerBinaryOptions,
        _: &'a mut Option<(bool, LanguageServerBinary)>,
        _: &'a mut AsyncApp,
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
                make_file_executable(&path)
                    .await
                    .context("failed to set file permissions")?;
            }

            Ok(LanguageServerBinary {
                path,
                arguments: command.args.into_iter().map(|arg| arg.into()).collect(),
                env: Some(command.env.into_iter().collect()),
            })
        }
        .boxed_local()
    }

    async fn try_fetch_server_binary(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: PathBuf,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<LanguageServerBinary> {
        unreachable!("get_language_server_command is overridden")
    }
}

#[async_trait(?Send)]
impl LspAdapter for ExtensionLspAdapter {
    fn name(&self) -> LanguageServerName {
        self.language_server_id.clone()
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

    fn language_ids(&self) -> HashMap<LanguageName, String> {
        // TODO: The language IDs can be provided via the language server options
        // in `extension.toml now but we're leaving these existing usages in place temporarily
        // to avoid any compatibility issues between Zed and the extension versions.
        //
        // We can remove once the following extension versions no longer see any use:
        // - php@0.0.1
        if self.extension.manifest().id.as_ref() == "php" {
            return HashMap::from_iter([(LanguageName::new("PHP"), "php".into())]);
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
        _: Option<Toolchain>,
        _cx: &mut AsyncApp,
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

    async fn additional_initialization_options(
        self: Arc<Self>,
        target_language_server_id: LanguageServerName,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let delegate = Arc::new(WorktreeDelegateAdapter(delegate.clone())) as _;
        let json_options: Option<String> = self
            .extension
            .language_server_additional_initialization_options(
                self.language_server_id.clone(),
                target_language_server_id.clone(),
                delegate,
            )
            .await?;
        Ok(if let Some(json_options) = json_options {
            serde_json::from_str(&json_options).with_context(|| {
                format!(
                    "failed to parse additional_initialization_options from extension: {json_options}"
                )
            })?
        } else {
            None
        })
    }

    async fn additional_workspace_configuration(
        self: Arc<Self>,
        target_language_server_id: LanguageServerName,

        delegate: &Arc<dyn LspAdapterDelegate>,

        _cx: &mut AsyncApp,
    ) -> Result<Option<serde_json::Value>> {
        let delegate = Arc::new(WorktreeDelegateAdapter(delegate.clone())) as _;
        let json_options: Option<String> = self
            .extension
            .language_server_additional_workspace_configuration(
                self.language_server_id.clone(),
                target_language_server_id.clone(),
                delegate,
            )
            .await?;
        Ok(if let Some(json_options) = json_options {
            serde_json::from_str(&json_options).with_context(|| {
                format!("failed to parse additional_workspace_configuration from extension: {json_options}")
            })?
        } else {
            None
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

        Ok(labels_from_extension(labels, language))
    }

    fn is_extension(&self) -> bool {
        true
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
                language.highlight_text(
                    &Rope::from_str_small(label.code.as_str()),
                    0..label.code.len(),
                )
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
    Some(CodeLabel::new(text, filter_range, runs))
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
        CodeLabel::new(label_text, label.filter_range.clone(), label_runs)
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
