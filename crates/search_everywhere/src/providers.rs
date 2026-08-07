use gpui::{Action, Entity, SharedString};
use language::{Anchor, Buffer};
use project::{ProjectPath, Symbol};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchResultCategory {
    File,
    Symbol,
    Action,
}

pub struct SearchResult {
    pub label: SharedString,
    pub detail: Option<SharedString>,
    pub category: SearchResultCategory,
    pub path: Option<ProjectPath>,
    pub action: Option<Box<dyn Action>>,
    pub symbol: Option<Symbol>,
    pub document_symbol: Option<DocumentSymbolResult>,
}

#[derive(Clone)]
pub struct DocumentSymbolResult {
    pub buffer: Entity<Buffer>,
    pub range: std::ops::Range<Anchor>,
}
