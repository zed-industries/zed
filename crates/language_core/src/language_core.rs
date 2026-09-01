// language_core: tree-sitter grammar infrastructure, LSP adapter traits,
// language configuration, and highlight mapping.

pub mod grammar;
pub mod highlight_map;
pub mod language_config;

pub use grammar::{
    BracketsConfig, BracketsPatternConfig, ColorCapture, ColorComponent, ColorPatternScales, ColorScale, ColorsConfig,
    DebugVariablesConfig, DebuggerTextObject, Grammar, GrammarId, HighlightsConfig, IndentConfig,
    InjectionConfig, InjectionPatternConfig, NEXT_GRAMMAR_ID, OutlineConfig, OverrideConfig,
    OverrideEntry, ParseableLanguage, RedactionConfig, RunnableCapture, RunnableConfig, TextObject,
    TextObjectConfig,
};
pub use highlight_map::{HighlightId, HighlightMap};
pub use language_config::{
    BlockCommentConfig, BracketPair, BracketPairConfig, BracketPairContent, DecreaseIndentConfig,
    JsxTagAutoCloseConfig, LanguageConfig, LanguageConfigOverride, LanguageMatcher,
    OrderedListConfig, Override, SoftWrap, TaskListConfig, WrapCharactersConfig, default_true,
    deserialize_regex, deserialize_regex_vec, regex_json_schema, regex_vec_json_schema,
    serialize_regex,
};

pub mod code_label;
pub mod language_name;
pub mod lsp_adapter;
pub mod manifest;
pub mod queries;
pub mod toolchain;

pub use code_label::{CodeLabel, CodeLabelBuilder, Symbol, SymbolKind};
pub use language_name::{LanguageId, LanguageName};
pub use lsp_adapter::{BinaryStatus, LanguageServerStatusUpdate, ServerHealth};
pub use manifest::ManifestName;
pub use queries::{LanguageQueries, QueryFile, QueryFileContents, QueryFiles};
pub use toolchain::{Toolchain, ToolchainList, ToolchainMetadata, ToolchainScope};
