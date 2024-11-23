use std::option::Option;

/// An LSP completion.
#[derive(Debug, Clone)]
pub struct Completion {
    pub label: String,
    pub label_details: Option<CompletionLabelDetails>,
    pub detail: Option<String>,
    pub kind: Option<CompletionKind>,
    pub insert_text_format: Option<InsertTextFormat>,
}

/// The kind of an LSP completion.
#[derive(Debug, Clone, Copy)]
pub enum CompletionKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
    Other(i32),
}

/// Label details for an LSP completion.
#[derive(Debug, Clone)]
pub struct CompletionLabelDetails {
    pub detail: Option<String>,
    pub description: Option<String>,
}

/// Defines how to interpret the insert text in a completion item.
#[derive(Debug, Clone, Copy)]
pub enum InsertTextFormat {
    PlainText,
    Snippet,
    Other(i32),
}

/// An LSP symbol.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub name: String,
}

/// The kind of an LSP symbol.
#[derive(Debug, Clone, Copy)]
pub enum SymbolKind {
    File,
    Module,
    Namespace,
    Package,
    Class,
    Method,
    Property,
    Field,
    Constructor,
    Enum,
    Interface,
    Function,
    Variable,
    Constant,
    String,
    Number,
    Boolean,
    Array,
    Object,
    Key,
    Null,
    EnumMember,
    Struct,
    Event,
    Operator,
    TypeParameter,
    Other(i32),
}
