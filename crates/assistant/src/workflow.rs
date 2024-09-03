use crate::{AssistantPanel, InlineAssistId, InlineAssistant};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use editor::Editor;
use gpui::AsyncAppContext;
use gpui::{Model, Task, UpdateGlobal as _, View, WeakView, WindowContext};
use language::{Buffer, BufferSnapshot};
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{ops::Range, path::Path, sync::Arc};
use text::Bias;
use workspace::Workspace;

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
        range: Range<language::Anchor>,
        description: String,
    },
    CreateFile {
        description: String,
    },
    InsertBefore {
        position: language::Anchor,
        description: String,
    },
    InsertAfter {
        position: language::Anchor,
        description: String,
    },
    Delete {
        range: Range<language::Anchor>,
    },
}

impl WorkflowSuggestion {
    pub fn range(&self) -> Range<language::Anchor> {
        match self {
            Self::Update { range, .. } => range.clone(),
            Self::CreateFile { .. } => language::Anchor::MIN..language::Anchor::MAX,
            Self::InsertBefore { position, .. } | Self::InsertAfter { position, .. } => {
                *position..*position
            }
            Self::Delete { range, .. } => range.clone(),
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Update { description, .. }
            | Self::CreateFile { description }
            | Self::InsertBefore { description, .. }
            | Self::InsertAfter { description, .. } => Some(description),
            Self::Delete { .. } => None,
        }
    }

    fn description_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Update { description, .. }
            | Self::CreateFile { description }
            | Self::InsertBefore { description, .. }
            | Self::InsertAfter { description, .. } => Some(description),
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
            Self::InsertBefore {
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
            Self::InsertAfter {
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
        search: Option<String>,
        description: Option<String>,
    ) -> Result<Self> {
        let path = path.ok_or_else(|| anyhow!("missing path"))?;
        let operation = operation.ok_or_else(|| anyhow!("missing operation"))?;

        let kind = match operation.as_str() {
            "update" => WorkflowStepEditKind::Update {
                search: search.ok_or_else(|| anyhow!("missing search"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "insert_before" => WorkflowStepEditKind::InsertBefore {
                search: search.ok_or_else(|| anyhow!("missing search"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "insert_after" => WorkflowStepEditKind::InsertAfter {
                search: search.ok_or_else(|| anyhow!("missing search"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "delete" => WorkflowStepEditKind::Delete {
                search: search.ok_or_else(|| anyhow!("missing search"))?,
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

        let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
        let suggestion = cx
            .background_executor()
            .spawn(async move {
                match kind {
                    WorkflowStepEditKind::Update {
                        search,
                        description,
                    } => {
                        let range = Self::resolve_location(&snapshot, &search);
                        WorkflowSuggestion::Update { range, description }
                    }
                    WorkflowStepEditKind::Create { description } => {
                        WorkflowSuggestion::CreateFile { description }
                    }
                    WorkflowStepEditKind::InsertBefore {
                        search,
                        description,
                    } => {
                        let range = Self::resolve_location(&snapshot, &search);
                        WorkflowSuggestion::InsertBefore {
                            position: range.start,
                            description,
                        }
                    }
                    WorkflowStepEditKind::InsertAfter {
                        search,
                        description,
                    } => {
                        let range = Self::resolve_location(&snapshot, &search);
                        WorkflowSuggestion::InsertAfter {
                            position: range.end,
                            description,
                        }
                    }
                    WorkflowStepEditKind::Delete { search } => {
                        let range = Self::resolve_location(&snapshot, &search);
                        WorkflowSuggestion::Delete { range }
                    }
                }
            })
            .await;

        Ok((buffer, suggestion))
    }

    fn resolve_location(buffer: &text::BufferSnapshot, search_query: &str) -> Range<text::Anchor> {
        const INSERTION_SCORE: f64 = -1.0;
        const DELETION_SCORE: f64 = -1.0;
        const REPLACEMENT_SCORE: f64 = -1.0;
        const EQUALITY_SCORE: f64 = 5.0;

        struct Matrix {
            cols: usize,
            data: Vec<f64>,
        }

        impl Matrix {
            fn new(rows: usize, cols: usize) -> Self {
                Matrix {
                    cols,
                    data: vec![0.0; rows * cols],
                }
            }

            fn get(&self, row: usize, col: usize) -> f64 {
                self.data[row * self.cols + col]
            }

            fn set(&mut self, row: usize, col: usize, value: f64) {
                self.data[row * self.cols + col] = value;
            }
        }

        let buffer_len = buffer.len();
        let query_len = search_query.len();
        let mut matrix = Matrix::new(query_len + 1, buffer_len + 1);

        for (i, query_byte) in search_query.bytes().enumerate() {
            for (j, buffer_byte) in buffer.bytes_in_range(0..buffer.len()).flatten().enumerate() {
                let match_score = if query_byte == *buffer_byte {
                    EQUALITY_SCORE
                } else {
                    REPLACEMENT_SCORE
                };
                let up = matrix.get(i + 1, j) + DELETION_SCORE;
                let left = matrix.get(i, j + 1) + INSERTION_SCORE;
                let diagonal = matrix.get(i, j) + match_score;
                let score = up.max(left.max(diagonal)).max(0.);
                matrix.set(i + 1, j + 1, score);
            }
        }

        // Traceback to find the best match
        let mut best_buffer_end = buffer_len;
        let mut best_score = 0.0;
        for col in 1..=buffer_len {
            let score = matrix.get(query_len, col);
            if score > best_score {
                best_score = score;
                best_buffer_end = col;
            }
        }

        let mut query_ix = query_len;
        let mut buffer_ix = best_buffer_end;
        while query_ix > 0 && buffer_ix > 0 {
            let current = matrix.get(query_ix, buffer_ix);
            let up = matrix.get(query_ix - 1, buffer_ix);
            let left = matrix.get(query_ix, buffer_ix - 1);
            if current == left + INSERTION_SCORE {
                buffer_ix -= 1;
            } else if current == up + DELETION_SCORE {
                query_ix -= 1;
            } else {
                query_ix -= 1;
                buffer_ix -= 1;
            }
        }

        let mut start = buffer.offset_to_point(buffer.clip_offset(buffer_ix, Bias::Left));
        start.column = 0;
        let mut end = buffer.offset_to_point(buffer.clip_offset(best_buffer_end, Bias::Right));
        end.column = buffer.line_len(end.row);

        buffer.anchor_after(start)..buffer.anchor_before(end)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation")]
pub enum WorkflowStepEditKind {
    /// Rewrites the specified text entirely based on the given description.
    /// This operation completely replaces the given text.
    Update {
        /// A string in the source text to apply the update to.
        search: String,
        /// A brief description of the transformation to apply to the symbol.
        description: String,
    },
    /// Creates a new file with the given path based on the provided description.
    /// This operation adds a new file to the codebase.
    Create {
        /// A brief description of the file to be created.
        description: String,
    },
    /// Inserts text before the specified text in the source file.
    InsertBefore {
        /// A string in the source text to insert text before.
        search: String,
        /// A brief description of how the new text should be generated.
        description: String,
    },
    /// Inserts text after the specified text in the source file.
    InsertAfter {
        /// A string in the source text to insert text after.
        search: String,
        /// A brief description of how the new text should be generated.
        description: String,
    },
    /// Deletes the specified symbol from the containing file.
    Delete {
        /// A string in the source text to delete.
        search: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, Context};
    use text::{OffsetRangeExt, Point};

    #[gpui::test]
    fn test_resolve_location(cx: &mut AppContext) {
        {
            let buffer = cx.new_model(|cx| {
                Buffer::local(
                    concat!(
                        "    Lorem\n",
                        "    ipsum\n",
                        "    dolor sit amet\n",
                        "    consecteur",
                    ),
                    cx,
                )
            });
            let snapshot = buffer.read(cx).snapshot();
            assert_eq!(
                WorkflowStepEdit::resolve_location(&snapshot, "ipsum\ndolor").to_point(&snapshot),
                Point::new(1, 0)..Point::new(2, 18)
            );
        }

        {
            let buffer = cx.new_model(|cx| {
                Buffer::local(
                    concat!(
                        "fn foo1(a: usize) -> usize {\n",
                        "    42\n",
                        "}\n",
                        "\n",
                        "fn foo2(b: usize) -> usize {\n",
                        "    42\n",
                        "}\n",
                    ),
                    cx,
                )
            });
            let snapshot = buffer.read(cx).snapshot();
            assert_eq!(
                WorkflowStepEdit::resolve_location(&snapshot, "fn foo1(b: usize) {\n42\n}")
                    .to_point(&snapshot),
                Point::new(0, 0)..Point::new(2, 1)
            );
        }

        {
            let buffer = cx.new_model(|cx| {
                Buffer::local(
                    concat!(
                        "fn main() {\n",
                        "    Foo\n",
                        "        .bar()\n",
                        "        .baz()\n",
                        "        .qux()\n",
                        "}\n",
                        "\n",
                        "fn foo2(b: usize) -> usize {\n",
                        "    42\n",
                        "}\n",
                    ),
                    cx,
                )
            });
            let snapshot = buffer.read(cx).snapshot();
            assert_eq!(
                WorkflowStepEdit::resolve_location(&snapshot, "Foo.bar.baz.qux()")
                    .to_point(&snapshot),
                Point::new(1, 0)..Point::new(4, 14)
            );
        }
    }
}
