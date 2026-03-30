// language_core: tree-sitter grammar infrastructure, LSP adapter traits,
// language configuration, and highlight mapping.

pub mod diagnostic;
pub mod grammar;
pub mod highlight_map;
pub mod language_config;

pub use diagnostic::{Diagnostic, DiagnosticSourceKind};
pub use grammar::{
    BracketsConfig, BracketsPatternConfig, DebugVariablesConfig, DebuggerTextObject, Grammar,
    GrammarId, HighlightsConfig, ImportsConfig, IndentConfig, InjectionConfig,
    InjectionPatternConfig, NEXT_GRAMMAR_ID, OutlineConfig, OverrideConfig, OverrideEntry,
    RedactionConfig, RunnableCapture, RunnableConfig, TextObject, TextObjectConfig,
};
pub use highlight_map::{HighlightId, HighlightMap};
pub use language_config::{
    BlockCommentConfig, BracketPair, BracketPairConfig, BracketPairContent, DecreaseIndentConfig,
    JsxTagAutoCloseConfig, LanguageConfig, LanguageConfigOverride, LanguageMatcher,
    OrderedListConfig, Override, SoftWrap, TaskListConfig, WrapCharactersConfig,
    auto_indent_using_last_non_empty_line_default, deserialize_regex, deserialize_regex_vec,
    regex_json_schema, regex_vec_json_schema, serialize_regex,
};

pub mod code_label;
pub mod language_name;
pub mod lsp_adapter;
pub mod manifest;
pub mod queries;
pub mod toolchain;

pub use code_label::{CodeLabel, CodeLabelBuilder, Symbol};
pub use language_name::{LanguageId, LanguageName};
pub use lsp_adapter::{
    BinaryStatus, LanguageServerStatusUpdate, PromptResponseContext, ServerHealth, ToLspPosition,
};
pub use manifest::ManifestName;
pub use queries::{LanguageQueries, QUERY_FILENAME_PREFIXES};
pub use toolchain::{Toolchain, ToolchainList, ToolchainMetadata, ToolchainScope};
