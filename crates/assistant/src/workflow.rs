use crate::{AssistantPanel, InlineAssistId, InlineAssistant};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use editor::Editor;
use gpui::AsyncAppContext;
use gpui::{Model, Task, UpdateGlobal as _, View, WeakView, WindowContext};
use language::{Anchor, Buffer, BufferSnapshot, Outline, OutlineItem, ParseStatus, SymbolPath};
use project::{Project, ProjectPath};
use rope::Point;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{ops::Range, path::Path, sync::Arc};
use workspace::Workspace;

const IMPORTS_SYMBOL: &str = "#imports";

#[derive(Debug)]
pub(crate) struct WorkflowStep {
    pub range: Range<language::Anchor>,
    pub leading_tags_end: text::Anchor,
    pub trailing_tag_start: Option<text::Anchor>,
    pub edits: Arc<[Result<WorkflowStepEdit>]>,
    pub resolution_task: Option<Task<()>>,
    pub resolution: Option<Arc<Result<WorkflowStepResolution>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorkflowStepEdit {
    pub path: String,
    pub kind: WorkflowStepEditKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorkflowStepResolution {
    pub title: String,
    pub suggestion_groups: HashMap<Model<Buffer>, Vec<WorkflowSuggestionGroup>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowSuggestionGroup {
    pub context_range: Range<language::Anchor>,
    pub suggestions: Vec<WorkflowSuggestion>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowSuggestion {
    Update {
        symbol_path: SymbolPath,
        range: Range<language::Anchor>,
        description: String,
    },
    CreateFile {
        description: String,
    },
    InsertSiblingBefore {
        symbol_path: SymbolPath,
        position: language::Anchor,
        description: String,
    },
    InsertSiblingAfter {
        symbol_path: SymbolPath,
        position: language::Anchor,
        description: String,
    },
    PrependChild {
        symbol_path: Option<SymbolPath>,
        position: language::Anchor,
        description: String,
    },
    AppendChild {
        symbol_path: Option<SymbolPath>,
        position: language::Anchor,
        description: String,
    },
    Delete {
        symbol_path: SymbolPath,
        range: Range<language::Anchor>,
    },
}

impl WorkflowSuggestion {
    pub fn range(&self) -> Range<language::Anchor> {
        match self {
            Self::Update { range, .. } => range.clone(),
            Self::CreateFile { .. } => language::Anchor::MIN..language::Anchor::MAX,
            Self::InsertSiblingBefore { position, .. }
            | Self::InsertSiblingAfter { position, .. }
            | Self::PrependChild { position, .. }
            | Self::AppendChild { position, .. } => *position..*position,
            Self::Delete { range, .. } => range.clone(),
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Update { description, .. }
            | Self::CreateFile { description }
            | Self::InsertSiblingBefore { description, .. }
            | Self::InsertSiblingAfter { description, .. }
            | Self::PrependChild { description, .. }
            | Self::AppendChild { description, .. } => Some(description),
            Self::Delete { .. } => None,
        }
    }

    fn description_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Update { description, .. }
            | Self::CreateFile { description }
            | Self::InsertSiblingBefore { description, .. }
            | Self::InsertSiblingAfter { description, .. }
            | Self::PrependChild { description, .. }
            | Self::AppendChild { description, .. } => Some(description),
            Self::Delete { .. } => None,
        }
    }

    pub fn try_merge(&mut self, other: &Self, buffer: &BufferSnapshot) -> bool {
        let range = self.range();
        let other_range = other.range();

        // Don't merge if we don't contain the other suggestion.
        if range.start.cmp(&other_range.start, buffer).is_gt()
            || range.end.cmp(&other_range.end, buffer).is_lt()
        {
            return false;
        }

        if let Some(description) = self.description_mut() {
            if let Some(other_description) = other.description() {
                description.push('\n');
                description.push_str(other_description);
            }
        }
        true
    }

    pub fn show(
        &self,
        editor: &View<Editor>,
        excerpt_id: editor::ExcerptId,
        workspace: &WeakView<Workspace>,
        assistant_panel: &View<AssistantPanel>,
        cx: &mut WindowContext,
    ) -> Option<InlineAssistId> {
        let mut initial_transaction_id = None;
        let initial_prompt;
        let suggestion_range;
        let buffer = editor.read(cx).buffer().clone();
        let snapshot = buffer.read(cx).snapshot(cx);

        match self {
            Self::Update {
                range, description, ..
            } => {
                initial_prompt = description.clone();
                suggestion_range = snapshot.anchor_in_excerpt(excerpt_id, range.start)?
                    ..snapshot.anchor_in_excerpt(excerpt_id, range.end)?;
            }
            Self::CreateFile { description } => {
                initial_prompt = description.clone();
                suggestion_range = editor::Anchor::min()..editor::Anchor::min();
            }
            Self::InsertSiblingBefore {
                position,
                description,
                ..
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            Self::InsertSiblingAfter {
                position,
                description,
                ..
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            Self::PrependChild {
                position,
                description,
                ..
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, false, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            Self::AppendChild {
                position,
                description,
                ..
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, false, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            Self::Delete { range, .. } => {
                initial_prompt = "Delete".to_string();
                suggestion_range = snapshot.anchor_in_excerpt(excerpt_id, range.start)?
                    ..snapshot.anchor_in_excerpt(excerpt_id, range.end)?;
            }
        }

        InlineAssistant::update_global(cx, |inline_assistant, cx| {
            Some(inline_assistant.suggest_assist(
                editor,
                suggestion_range,
                initial_prompt,
                initial_transaction_id,
                Some(workspace.clone()),
                Some(assistant_panel),
                cx,
            ))
        })
    }
}

impl WorkflowStepEdit {
    pub fn new(
        path: Option<String>,
        operation: Option<String>,
        symbol: Option<String>,
        description: Option<String>,
    ) -> Result<Self> {
        let path = path.ok_or_else(|| anyhow!("missing path"))?;
        let operation = operation.ok_or_else(|| anyhow!("missing operation"))?;

        let kind = match operation.as_str() {
            "update" => WorkflowStepEditKind::Update {
                symbol: symbol.ok_or_else(|| anyhow!("missing symbol"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "insert_sibling_before" => WorkflowStepEditKind::InsertSiblingBefore {
                symbol: symbol.ok_or_else(|| anyhow!("missing symbol"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "insert_sibling_after" => WorkflowStepEditKind::InsertSiblingAfter {
                symbol: symbol.ok_or_else(|| anyhow!("missing symbol"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "prepend_child" => WorkflowStepEditKind::PrependChild {
                symbol,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "append_child" => WorkflowStepEditKind::AppendChild {
                symbol,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "delete" => WorkflowStepEditKind::Delete {
                symbol: symbol.ok_or_else(|| anyhow!("missing symbol"))?,
            },
            "create" => WorkflowStepEditKind::Create {
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            _ => Err(anyhow!("unknown operation {operation:?}"))?,
        };

        Ok(Self { path, kind })
    }

    pub async fn resolve(
        &self,
        project: Model<Project>,
        mut cx: AsyncAppContext,
    ) -> Result<(Model<Buffer>, super::WorkflowSuggestion)> {
        let path = self.path.clone();
        let kind = self.kind.clone();
        let buffer = project
            .update(&mut cx, |project, cx| {
                let project_path = project
                    .find_project_path(Path::new(&path), cx)
                    .or_else(|| {
                        // If we couldn't find a project path for it, put it in the active worktree
                        // so that when we create the buffer, it can be saved.
                        let worktree = project
                            .active_entry()
                            .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
                            .or_else(|| project.worktrees(cx).next())?;
                        let worktree = worktree.read(cx);

                        Some(ProjectPath {
                            worktree_id: worktree.id(),
                            path: Arc::from(Path::new(&path)),
                        })
                    })
                    .with_context(|| format!("worktree not found for {:?}", path))?;
                anyhow::Ok(project.open_buffer(project_path, cx))
            })??
            .await?;

        let mut parse_status = buffer.read_with(&cx, |buffer, _cx| buffer.parse_status())?;
        while *parse_status.borrow() != ParseStatus::Idle {
            parse_status.changed().await?;
        }

        let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
        let outline = snapshot.outline(None).context("no outline for buffer")?;

        let suggestion = match kind {
            WorkflowStepEditKind::Update {
                symbol,
                description,
            } => {
                let (symbol_path, symbol) = Self::resolve_symbol(&snapshot, &outline, &symbol)?;
                let start = symbol
                    .annotation_range
                    .map_or(symbol.range.start, |range| range.start);
                let start = Point::new(start.row, 0);
                let end = Point::new(
                    symbol.range.end.row,
                    snapshot.line_len(symbol.range.end.row),
                );
                let range = snapshot.anchor_before(start)..snapshot.anchor_after(end);
                WorkflowSuggestion::Update {
                    range,
                    description,
                    symbol_path,
                }
            }
            WorkflowStepEditKind::Create { description } => {
                WorkflowSuggestion::CreateFile { description }
            }
            WorkflowStepEditKind::InsertSiblingBefore {
                symbol,
                description,
            } => {
                let (symbol_path, symbol) = Self::resolve_symbol(&snapshot, &outline, &symbol)?;
                let position = snapshot.anchor_before(
                    symbol
                        .annotation_range
                        .map_or(symbol.range.start, |annotation_range| {
                            annotation_range.start
                        }),
                );
                WorkflowSuggestion::InsertSiblingBefore {
                    position,
                    description,
                    symbol_path,
                }
            }
            WorkflowStepEditKind::InsertSiblingAfter {
                symbol,
                description,
            } => {
                let (symbol_path, symbol) = Self::resolve_symbol(&snapshot, &outline, &symbol)?;
                let position = snapshot.anchor_after(symbol.range.end);
                WorkflowSuggestion::InsertSiblingAfter {
                    position,
                    description,
                    symbol_path,
                }
            }
            WorkflowStepEditKind::PrependChild {
                symbol,
                description,
            } => {
                if let Some(symbol) = symbol {
                    let (symbol_path, symbol) = Self::resolve_symbol(&snapshot, &outline, &symbol)?;

                    let position = snapshot.anchor_after(
                        symbol
                            .body_range
                            .map_or(symbol.range.start, |body_range| body_range.start),
                    );
                    WorkflowSuggestion::PrependChild {
                        position,
                        description,
                        symbol_path: Some(symbol_path),
                    }
                } else {
                    WorkflowSuggestion::PrependChild {
                        position: language::Anchor::MIN,
                        description,
                        symbol_path: None,
                    }
                }
            }
            WorkflowStepEditKind::AppendChild {
                symbol,
                description,
            } => {
                if let Some(symbol) = symbol {
                    let (symbol_path, symbol) = Self::resolve_symbol(&snapshot, &outline, &symbol)?;

                    let position = snapshot.anchor_before(
                        symbol
                            .body_range
                            .map_or(symbol.range.end, |body_range| body_range.end),
                    );
                    WorkflowSuggestion::AppendChild {
                        position,
                        description,
                        symbol_path: Some(symbol_path),
                    }
                } else {
                    WorkflowSuggestion::PrependChild {
                        position: language::Anchor::MAX,
                        description,
                        symbol_path: None,
                    }
                }
            }
            WorkflowStepEditKind::Delete { symbol } => {
                let (symbol_path, symbol) = Self::resolve_symbol(&snapshot, &outline, &symbol)?;
                let start = symbol
                    .annotation_range
                    .map_or(symbol.range.start, |range| range.start);
                let start = Point::new(start.row, 0);
                let end = Point::new(
                    symbol.range.end.row,
                    snapshot.line_len(symbol.range.end.row),
                );
                let range = snapshot.anchor_before(start)..snapshot.anchor_after(end);
                WorkflowSuggestion::Delete { range, symbol_path }
            }
        };

        Ok((buffer, suggestion))
    }

    fn resolve_symbol(
        snapshot: &BufferSnapshot,
        outline: &Outline<Anchor>,
        symbol: &str,
    ) -> Result<(SymbolPath, OutlineItem<Point>)> {
        if symbol == IMPORTS_SYMBOL {
            let target_row = find_first_non_comment_line(snapshot);
            Ok((
                SymbolPath(IMPORTS_SYMBOL.to_string()),
                OutlineItem {
                    range: Point::new(target_row, 0)..Point::new(target_row + 1, 0),
                    ..Default::default()
                },
            ))
        } else {
            let (symbol_path, symbol) = outline
                .find_most_similar(symbol)
                .with_context(|| format!("symbol not found: {symbol}"))?;
            Ok((symbol_path, symbol.to_point(snapshot)))
        }
    }
}

fn find_first_non_comment_line(snapshot: &BufferSnapshot) -> u32 {
    let Some(language) = snapshot.language() else {
        return 0;
    };

    let scope = language.default_scope();
    let comment_prefixes = scope.line_comment_prefixes();

    let mut chunks = snapshot.as_rope().chunks();
    let mut target_row = 0;
    loop {
        let starts_with_comment = chunks
            .peek()
            .map(|chunk| {
                comment_prefixes
                    .iter()
                    .any(|s| chunk.starts_with(s.as_ref().trim_end()))
            })
            .unwrap_or(false);

        if !starts_with_comment {
            break;
        }

        target_row += 1;
        if !chunks.next_line() {
            break;
        }
    }
    target_row
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation")]
pub enum WorkflowStepEditKind {
    /// Rewrites the specified symbol entirely based on the given description.
    /// This operation completely replaces the existing symbol with new content.
    Update {
        /// A fully-qualified reference to the symbol, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
        /// The path should uniquely identify the symbol within the containing file.
        symbol: String,
        /// A brief description of the transformation to apply to the symbol.
        description: String,
    },
    /// Creates a new file with the given path based on the provided description.
    /// This operation adds a new file to the codebase.
    Create {
        /// A brief description of the file to be created.
        description: String,
    },
    /// Inserts a new symbol based on the given description before the specified symbol.
    /// This operation adds new content immediately preceding an existing symbol.
    InsertSiblingBefore {
        /// A fully-qualified reference to the symbol, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
        /// The new content will be inserted immediately before this symbol.
        symbol: String,
        /// A brief description of the new symbol to be inserted.
        description: String,
    },
    /// Inserts a new symbol based on the given description after the specified symbol.
    /// This operation adds new content immediately following an existing symbol.
    InsertSiblingAfter {
        /// A fully-qualified reference to the symbol, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
        /// The new content will be inserted immediately after this symbol.
        symbol: String,
        /// A brief description of the new symbol to be inserted.
        description: String,
    },
    /// Inserts a new symbol as a child of the specified symbol at the start.
    /// This operation adds new content as the first child of an existing symbol (or file if no symbol is provided).
    PrependChild {
        /// An optional fully-qualified reference to the symbol after the code you want to insert, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
        /// If provided, the new content will be inserted as the first child of this symbol.
        /// If not provided, the new content will be inserted at the top of the file.
        symbol: Option<String>,
        /// A brief description of the new symbol to be inserted.
        description: String,
    },
    /// Inserts a new symbol as a child of the specified symbol at the end.
    /// This operation adds new content as the last child of an existing symbol (or file if no symbol is provided).
    AppendChild {
        /// An optional fully-qualified reference to the symbol before the code you want to insert, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
        /// If provided, the new content will be inserted as the last child of this symbol.
        /// If not provided, the new content will be applied at the bottom of the file.
        symbol: Option<String>,
        /// A brief description of the new symbol to be inserted.
        description: String,
    },
    /// Deletes the specified symbol from the containing file.
    Delete {
        /// An fully-qualified reference to the symbol to be deleted, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
        symbol: String,
    },
}
