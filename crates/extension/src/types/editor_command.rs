use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorSelection {
    pub start: usize,
    pub end: usize,
    pub reversed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorCommandContext {
    pub text: String,
    pub selections: Vec<EditorSelection>,
    pub language: Option<String>,
    pub path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorEdit {
    pub range: Range<usize>,
    pub new_text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorCommandResult {
    pub edits: Vec<EditorEdit>,
    pub selections: Option<Vec<EditorSelection>>,
}
