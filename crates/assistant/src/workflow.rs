use crate::{AssistantPanel, InlineAssistId, InlineAssistant};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use editor::Editor;
use gpui::AsyncAppContext;
use gpui::{Model, SharedString, UpdateGlobal as _, View, WeakView, WindowContext};
use language::{Buffer, BufferSnapshot};
use project::{Project, ProjectPath};
use std::{ops::Range, path::Path, sync::Arc};
use text::Bias;
use workspace::Workspace;

#[derive(Clone, Debug)]
pub(crate) struct AssistantPatch {
    pub range: Range<language::Anchor>,
    pub title: SharedString,
    pub edits: Arc<[Result<AssistantEdit>]>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AssistantEdit {
    pub path: String,
    pub kind: AssistantEditKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AssistantPatchResolution {
    pub suggestion_groups: HashMap<Model<Buffer>, Vec<WorkflowSuggestionGroup>>,
    pub errors: Vec<AssistantPatchResolutionError>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AssistantPatchResolutionError {
    pub edit_ix: usize,
    pub message: String,
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
        new_text: String,
        description: String,
    },
    CreateFile {
        description: String,
        new_text: String,
    },
    InsertBefore {
        position: language::Anchor,
        new_text: String,
        description: String,
    },
    InsertAfter {
        position: language::Anchor,
        new_text: String,
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

    pub fn new_text(&self) -> String {
        match self {
            Self::Update { new_text, .. }
            | Self::CreateFile { new_text, .. }
            | Self::InsertBefore { new_text, .. }
            | Self::InsertAfter { new_text, .. } => new_text.clone(),
            Self::Delete { .. } => String::new(),
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Update { description, .. }
            | Self::CreateFile { description, .. }
            | Self::InsertBefore { description, .. }
            | Self::InsertAfter { description, .. } => Some(description),
            Self::Delete { .. } => None,
        }
    }

    fn description_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Update { description, .. }
            | Self::CreateFile { description, .. }
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
            Self::CreateFile { description, .. } => {
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
                false,
                Some(workspace.clone()),
                Some(assistant_panel),
                cx,
            ))
        })
    }
}

impl AssistantPatch {
    pub fn path_count(&self) -> usize {
        self.paths().count()
    }

    pub fn paths(&self) -> impl '_ + Iterator<Item = &str> {
        let mut prev_path = None;
        self.edits.iter().filter_map(move |edit| {
            if let Ok(edit) = edit {
                let path = Some(edit.path.as_str());
                if path != prev_path {
                    prev_path = path;
                    return path;
                }
            }
            None
        })
    }
}

impl AssistantEdit {
    pub fn new(
        path: Option<String>,
        operation: Option<String>,
        old_text: Option<String>,
        new_text: Option<String>,
        description: Option<String>,
    ) -> Result<Self> {
        let path = path.ok_or_else(|| anyhow!("missing path"))?;
        let operation = operation.ok_or_else(|| anyhow!("missing operation"))?;

        let kind = match operation.as_str() {
            "update" => AssistantEditKind::Update {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "insert_before" => AssistantEditKind::InsertBefore {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "insert_after" => AssistantEditKind::InsertAfter {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description: description.ok_or_else(|| anyhow!("missing description"))?,
            },
            "delete" => AssistantEditKind::Delete {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
            },
            "create" => AssistantEditKind::Create {
                description: description.ok_or_else(|| anyhow!("missing description"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
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
                    AssistantEditKind::Update {
                        old_text,
                        new_text,
                        description,
                    } => {
                        let range = Self::resolve_location(&snapshot, &old_text);
                        WorkflowSuggestion::Update {
                            range,
                            description,
                            new_text,
                        }
                    }
                    AssistantEditKind::Create {
                        new_text,
                        description,
                    } => WorkflowSuggestion::CreateFile {
                        description,
                        new_text,
                    },
                    AssistantEditKind::InsertBefore {
                        old_text,
                        mut new_text,
                        description,
                    } => {
                        new_text.push('\n');
                        let range = Self::resolve_location(&snapshot, &old_text);
                        WorkflowSuggestion::InsertBefore {
                            position: range.start,
                            description,
                            new_text,
                        }
                    }
                    AssistantEditKind::InsertAfter {
                        old_text,
                        mut new_text,
                        description,
                    } => {
                        new_text.insert(0, '\n');
                        let range = Self::resolve_location(&snapshot, &old_text);
                        WorkflowSuggestion::InsertAfter {
                            position: range.end,
                            description,
                            new_text,
                        }
                    }
                    AssistantEditKind::Delete { old_text } => {
                        let range = Self::resolve_location(&snapshot, &old_text);
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AssistantEditKind {
    Update {
        old_text: String,
        new_text: String,
        description: String,
    },
    Create {
        new_text: String,
        description: String,
    },
    InsertBefore {
        old_text: String,
        new_text: String,
        description: String,
    },
    InsertAfter {
        old_text: String,
        new_text: String,
        description: String,
    },
    Delete {
        old_text: String,
    },
}

impl PartialEq for AssistantPatch {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
            && self.title == other.title
            && Arc::ptr_eq(&self.edits, &other.edits)
    }
}

impl Eq for AssistantPatch {}

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
                AssistantEdit::resolve_location(&snapshot, "ipsum\ndolor").to_point(&snapshot),
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
                AssistantEdit::resolve_location(&snapshot, "fn foo1(b: usize) {\n42\n}")
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
                AssistantEdit::resolve_location(&snapshot, "Foo.bar.baz.qux()").to_point(&snapshot),
                Point::new(1, 0)..Point::new(4, 14)
            );
        }
    }
}
